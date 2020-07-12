
use gltf;
use nalgebra_glm as glm;

use std::collections::HashMap;
use std::sync::Arc;

use crate::vk::Result;
use crate::vk::*;
use crate::ffi::vk::*;

use libc::c_void;

use gltf::accessor::DataType;
use gltf::accessor::Dimensions;
use gltf::Semantic;

use VkMemoryPropertyFlagBits::*;
use VkBufferUsageFlagBits::*;

pub struct SceneAsset {
    document: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    images: Vec<gltf::image::Data>,
}

impl SceneAsset {
    pub fn new() -> Result<Self> {
        log_debug!("loading scene asset");
        let (document, buffers, images) = gltf::import("data/models/box.glb").unwrap();
        log_debug!("loading scene asset complete");
        let asset = Self {
            document,
            buffers,
            images,
        };
        Ok(asset)
    }

    #[inline]
    fn document(&self) -> &gltf::Document {
        &self.document
    }

    #[inline]
    fn buffers(&self) -> &Vec<gltf::buffer::Data> {
        &self.buffers
    }

    #[inline]
    fn images(&self) -> &Vec<gltf::image::Data> {
        &self.images
    }
}

pub struct SceneBuilder<'a> {
    asset: &'a SceneAsset,
}

impl<'a> SceneBuilder<'a> {
    pub fn new(asset: &'a SceneAsset) -> Self {
        Self {
            asset,
        }
    }

    pub fn build(self, command_pool: &Arc<CommandPool>) -> Scene {
        let asset = self.asset;
        log_debug!("start scene builder");
        let table = MeshTable::new(asset);
        log_debug!("iterating nodes");
        let scene = asset.document()
            .scenes()
            .nth(0)
            .unwrap();
        let nodes: Vec<_> = scene.nodes()
            .map(|v| FlattenNode::flatten(v))
            .flatten()
            .map(|v| MeshNode::new(v, &table))
            .filter_map(|v| v)
            .flatten()
            .collect();
        log_debug!("iterating nodes complete");
        Scene::new(&table, &nodes, command_pool)
    }
}

pub struct Scene {
    primitives: Vec<Arc<SceneMeshPrimitive>>,
    staging_buffers: Arc<SceneStagingBuffers>,
    top_level_acceleration_structure: Arc<TopLevelAccelerationStructure>,
}

impl Scene {
    fn new(table: &MeshTable, nodes: &[MeshNode], command_pool: &Arc<CommandPool>) -> Self {
        let primitives = table.mesh_primitives();
        log_debug!("creating staging buffers");
        let staging_buffers = SceneStagingBuffers::new(command_pool, primitives);
        log_debug!("creating staging complete");
        log_debug!("building blas");
        let scene_mesh_primitive_geometries: Vec<_> = table.mesh_primitives().iter()
            .map(|v| SceneMeshPrimitiveGeometry::new(v, &staging_buffers, command_pool))
            .collect();
        let geometries: Vec<_> = scene_mesh_primitive_geometries.iter()
            .map(|v| v.structure_geometry())
            .map(Arc::clone)
            .collect();
        let builder = BottomLevelAccelerationStructuresBuilder::new(command_pool, &geometries);
        let structures = builder.build().unwrap();
        let scene_mesh_primitives: Vec<_> = structures.into_iter()
            .zip(scene_mesh_primitive_geometries.into_iter())
            .map(|(structure, geometry)| SceneMeshPrimitive::new(geometry, structure))
            .collect();
        log_debug!("building blas complete");
        log_debug!("building tlas");
        let instances = nodes.into_iter()
            .map(|node| {
                let index = node.primitive().index();
                let mesh_primitive = scene_mesh_primitives.get(index).unwrap();
                let transform = node.transform();
                let transform = VkTransformMatrixKHR {
                    matrix: [
                        [transform.m11, transform.m12, transform.m13, transform.m14],
                        [transform.m21, transform.m22, transform.m23, transform.m24],
                        [transform.m31, transform.m32, transform.m33, transform.m34],
                    ]
                };
                let instance = TopLevelAccelerationStructureInstance::new(
                    index as u32,
                    transform,
                    mesh_primitive.bottom_level_acceleration_structure(),
                ).unwrap();
                instance
            })
            .collect();
        let top_level_acceleration_structure = TopLevelAccelerationStructure::new(command_pool, instances)
            .unwrap();
        log_debug!("building tlas complete");
        log_debug!("scene building complete");
        Self {
            primitives: scene_mesh_primitives,
            staging_buffers,
            top_level_acceleration_structure,
        }
    }

    pub fn top_level_acceleration_structure(&self) -> &Arc<TopLevelAccelerationStructure> {
        &self.top_level_acceleration_structure
    }

    pub fn index_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.index_buffer()
    }

    pub fn vertex_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.vertex_buffer()
    }

    pub fn normal_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.normal_buffer()
    }

    pub fn description_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.description_buffer()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceneMeshPrimitiveDescription {
    vertex_offset: u32,
    index_offset: u32,
}

impl From<MeshPrimitiveOffset> for SceneMeshPrimitiveDescription {
    fn from(offset: MeshPrimitiveOffset) -> Self {
        Self {
            vertex_offset: offset.vertex_offset as u32,
            index_offset: offset.index_offset as u32,
        }
    }
}

pub struct SceneStagingBuffers {
    vertex_buffer: Arc<DedicatedStagingBuffer>,
    index_buffer: Arc<DedicatedStagingBuffer>,
    normals_buffer: Arc<DedicatedStagingBuffer>,
    description_buffer: Arc<DedicatedStagingBuffer>,
}

impl SceneStagingBuffers {
    fn new(command_pool: &Arc<CommandPool>, primitives: &Vec<MeshPrimitive>) -> Arc<Self> {
        let num_indices: usize = primitives.iter()
            .map(|v| v.primitive.indices.count())
            .sum();
        let num_vertices: usize = primitives.iter()
            .map(|v| v.primitive.positions.count())
            .sum();
        let vertex_buffer_size = std::mem::size_of::<[f32; 3]>() * num_vertices;
        let vertex_buffer = DedicatedStagingBuffer::new(
            command_pool, 
            VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as VkBufferUsageFlags 
                | VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            vertex_buffer_size as VkDeviceSize,
        ).unwrap();
        let index_buffer_size = std::mem::size_of::<u32>() * num_indices;
        let index_buffer = DedicatedStagingBuffer::new(
            command_pool, 
            VK_BUFFER_USAGE_INDEX_BUFFER_BIT as VkBufferUsageFlags 
                | VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            index_buffer_size as VkDeviceSize,
        ).unwrap();
        let normals_buffer_size = std::mem::size_of::<[f32; 3]>() * num_vertices;
        let normals_buffer = DedicatedStagingBuffer::new(
            command_pool, 
            VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            normals_buffer_size as VkDeviceSize,
        ).unwrap();
        let descriptions: Vec<SceneMeshPrimitiveDescription> = primitives.iter()
            .map(|v| v.offset.clone().into())
            .collect();
        let description_buffer_size = std::mem::size_of::<SceneMeshPrimitiveDescription>() * descriptions.len();
        let description_buffer = DedicatedStagingBuffer::new(
            command_pool, 
            VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            description_buffer_size as VkDeviceSize,
        ).unwrap();
        // TODO(ogukei): concurrent uploads
        unsafe {
            index_buffer.update(index_buffer_size as VkDeviceSize, |data| {
                let data = data as *mut u8;
                for primitive in primitives.iter() {
                    let byte_size = primitive.primitive.indices.count() * std::mem::size_of::<u32>();
                    let byte_offset = primitive.offset.index_offset * std::mem::size_of::<u32>();
                    let dst = data.offset(byte_offset as isize);
                    let src = primitive.primitive.indices.data();
                    std::ptr::copy_nonoverlapping(src, dst, byte_size);
                }
            });
            vertex_buffer.update(vertex_buffer_size as VkDeviceSize, |data| {
                let data = data as *mut u8;
                for primitive in primitives.iter() {
                    let byte_size = primitive.primitive.positions.count() * std::mem::size_of::<[f32; 3]>();
                    let byte_offset = primitive.offset.vertex_offset * std::mem::size_of::<[f32; 3]>();
                    let dst = data.offset(byte_offset as isize);
                    let src = primitive.primitive.positions.data();
                    std::ptr::copy_nonoverlapping(src, dst, byte_size);
                }
            });
            normals_buffer.update(normals_buffer_size as VkDeviceSize, |data| {
                let data = data as *mut u8;
                for primitive in primitives.iter() {
                    let byte_size = primitive.primitive.normals.count() * std::mem::size_of::<[f32; 3]>();
                    let byte_offset = primitive.offset.vertex_offset * std::mem::size_of::<[f32; 3]>();
                    let dst = data.offset(byte_offset as isize);
                    let src = primitive.primitive.normals.data();
                    std::ptr::copy_nonoverlapping(src, dst, byte_size);
                }
            });
            description_buffer.update(description_buffer_size as VkDeviceSize, |data| {
                let dst = data as *mut u8;
                let src = descriptions.as_ptr() as *const u8;
                std::ptr::copy_nonoverlapping(src, dst, description_buffer_size);
            });
        }
        let buffer = Self {
            vertex_buffer,
            index_buffer,
            normals_buffer,
            description_buffer,
        };
        Arc::new(buffer)
    }

    #[inline]
    pub fn vertex_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.vertex_buffer
    }

    #[inline]
    pub fn index_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.index_buffer
    }

    #[inline]
    pub fn normal_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.normals_buffer
    }

    #[inline]
    pub fn description_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.description_buffer
    }
}

struct FlattenNode<'a> {
    node: gltf::Node<'a>,
    transform: glm::Mat4,
}

impl<'a> FlattenNode<'a> {
    fn new(node: gltf::Node<'a>, transform: &glm::Mat4) -> Self {
        let local_transform: Vec<f32> = node.transform().matrix()
            .iter()
            .flat_map(|v| v.into_iter())
            .copied()
            .collect();
        let local_transform = glm::make_mat4(&local_transform);
        Self {
            node,
            transform: transform * local_transform,
        }
    }

    fn flatten(root: gltf::Node<'a>) -> Vec<Self> {
        let node = Self::new(root, &glm::identity());
        Self::flatten_nodes(node)
    }

    fn flatten_nodes(node: Self) -> Vec<Self> {
        let children: Vec<_> = node.node().children()
            .map(|v| Self::new(v, node.transform()))
            .flat_map(|v| Self::flatten_nodes(v))
            .collect();
        std::iter::once(node)
            .chain(children)
            .collect()
    }

    fn node(&self) -> &gltf::Node<'a> {
        &self.node
    }

    fn transform(&self) -> &glm::Mat4 {
        &self.transform
    }
}

struct MeshTable<'a> {
    primitives: Vec<MeshPrimitive<'a>>,
    mesh_table: HashMap<usize, Vec<usize>>,
}

impl<'a> MeshTable<'a> {
    fn new(asset: &'a SceneAsset) -> Self {
        log_debug!("iterating meshes");
        let meshes = asset.document().meshes()
            .map(|v| Mesh::new(v, asset.buffers()));
        log_debug!("constructing mesh primitives");
        let primitives: Vec<MeshPrimitive> = meshes
            .flat_map(|mesh| {
                let mesh_index = mesh.index();
                mesh.into_primitives()
                    .into_iter()
                    .map(move |mesh| (mesh_index, mesh))
            })
            .scan(MeshPrimitiveOffset::default(), |state, item| {
                let (mesh_index, primitive) = item;
                let offset = state.clone();
                state.index_offset += primitive.indices.count();
                state.vertex_offset += primitive.positions.count();
                Some((mesh_index, offset, primitive))
            })
            .enumerate()
            .map(|(index, (mesh_index, offset, primitive))| MeshPrimitive::new(index, mesh_index, offset, primitive))
            .collect();
        log_debug!("constructing mesh table");
        let mut mesh_table: HashMap<usize, Vec<usize>> = HashMap::new();
        for (index, primitive) in primitives.iter().enumerate() {
            let entry = mesh_table.entry(primitive.mesh_index())
                .or_insert_with(|| vec![]);
            entry.push(index);
        }
        log_debug!("constructing mesh table complete");
        Self {
            primitives,
            mesh_table,
        }
    }

    fn get(&self, mesh_index: usize) -> Vec<&MeshPrimitive<'a>> {
        self.mesh_table.get(&mesh_index).unwrap()
            .into_iter()
            .filter_map(|&v| self.primitives.get(v))
            .collect()
    }

    fn mesh_primitives(&self) -> &Vec<MeshPrimitive<'a>> {
        &self.primitives
    }
}

struct MeshNode<'a, 'b: 'a> {
    primitive: &'b MeshPrimitive<'a>,
    transform: glm::Mat4,
}

impl<'a, 'b: 'a> MeshNode<'a, 'b> {
    fn new(node: FlattenNode<'a>, mesh_table: &'b MeshTable<'a>) -> Option<Vec<Self>> {
        let mesh = node.node().mesh()?;
        let transform = node.transform();
        let primitives = mesh_table.get(mesh.index());
        let nodes = primitives.into_iter()
            .map(|primitive| MeshNode { primitive, transform: transform.clone() })
            .collect();
        Some(nodes)
    }

    fn primitive(&self) -> &'b MeshPrimitive<'a> {
        self.primitive
    }

    fn transform(&self) -> &glm::Mat4 {
        &self.transform
    }
}

pub struct SceneMeshPrimitive {
    geometry: Arc<SceneMeshPrimitiveGeometry>,
    structure: Arc<BottomLevelAccelerationStructure>,
}

impl SceneMeshPrimitive {
    fn new(geometry: Arc<SceneMeshPrimitiveGeometry>, structure: Arc<BottomLevelAccelerationStructure>) -> Arc<Self> {
        let primitive = Self {
            geometry,
            structure,
        };
        Arc::new(primitive)
    }

    pub fn staging_buffers(&self) -> &Arc<SceneStagingBuffers> {
        &self.geometry.staging_buffers()
    }

    pub fn index(&self) -> usize {
        self.geometry.index()
    }

    pub fn bottom_level_acceleration_structure(&self) -> &Arc<BottomLevelAccelerationStructure> {
        &self.structure
    }
}

pub struct SceneMeshPrimitiveGeometry {
    index: usize,
    offset: MeshPrimitiveOffset,
    structure_geometry: Arc<BottomLevelAccelerationStructureGeometry>,
    staging_buffers: Arc<SceneStagingBuffers>,
}

impl SceneMeshPrimitiveGeometry {
    fn new(mesh_primitive: &MeshPrimitive, staging_buffers: &Arc<SceneStagingBuffers>, command_pool: &Arc<CommandPool>) -> Arc<Self> {
        let vertex_stride = std::mem::size_of::<[f32; 3]>();
        let num_vertices = mesh_primitive.primitive.positions.count();
        let num_indices = mesh_primitive.primitive.indices.count();
        assert_eq!(mesh_primitive.primitive.positions.count(), mesh_primitive.primitive.normals.count());
        let structure_geometry = BottomLevelAccelerationStructureGeometry::new(
                num_vertices as u32, 
                vertex_stride as VkDeviceSize,
                mesh_primitive.offset.vertex_offset as u32,
                staging_buffers.vertex_buffer().device_buffer_memory(), 
                num_indices as u32, 
                mesh_primitive.offset.index_offset as u32,
                staging_buffers.index_buffer().device_buffer_memory())
            .unwrap();
        let v = Self {
            index: mesh_primitive.index(),
            offset: mesh_primitive.offset.clone(),
            staging_buffers: Arc::clone(staging_buffers),
            structure_geometry,
        };
        Arc::new(v)
    }
    
    fn staging_buffers(&self) -> &Arc<SceneStagingBuffers> {
        &self.staging_buffers
    }

    fn index(&self) -> usize {
        self.index
    }

    fn structure_geometry(&self) -> &Arc<BottomLevelAccelerationStructureGeometry> {
        &self.structure_geometry
    }
}

#[derive(Default, Clone)]
pub struct MeshPrimitiveOffset {
    pub vertex_offset: usize,
    pub index_offset: usize,
}

struct MeshPrimitive<'a> {
    mesh_primitive_index: usize,
    mesh_index: usize,
    offset: MeshPrimitiveOffset,
    primitive: Primitive<'a>,
}

impl<'a> MeshPrimitive<'a> {
    fn new(mesh_primitive_index: usize, mesh_index: usize, offset: MeshPrimitiveOffset, primitive: Primitive<'a>) -> Self {
        Self {
            mesh_index,
            mesh_primitive_index,
            offset,
            primitive,
        }
    }

    fn mesh_index(&self) -> usize {
        self.mesh_index
    }

    fn index(&self) -> usize {
        self.mesh_primitive_index
    }
}

struct Mesh<'a> {
    index: usize,
    primitives: Vec<Primitive<'a>>,
}

impl<'a> Mesh<'a> {
    fn new(mesh: gltf::Mesh<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        println!("Mesh {:?}", mesh.name());
        let index = mesh.index();
        let primitives = mesh.primitives()
            .map(|v| Primitive::new(v, buffers))
            .collect();
        Self {
            index,
            primitives,
        }
    }

    fn into_primitives(self) -> Vec<Primitive<'a>> {
        self.primitives
    }

    fn index(&self) -> usize {
        self.index
    }
}

struct Primitive<'a> {
    indices: Indices<'a>,
    positions: Positions<'a>,
    normals: Normals<'a>,
}

impl<'a> Primitive<'a> {
    fn new(primitive: gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        Self {
            indices: Indices::new(&primitive, buffers),
            positions: Positions::new(&primitive, buffers),
            normals: Normals::new(&primitive, buffers),
        }
    }
}

enum Indices<'a> {
    Accessor(AccessorIndicesU32<'a>),
    Vector(Vec<u32>),
}

impl<'a> Indices<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let indices = primitive.indices().unwrap();
        let view = indices.view().unwrap();
        let use_reference = view.stride() == None
            && indices.data_type() == DataType::U32
            && indices.dimensions() == Dimensions::Scalar;
        if use_reference {
            Self::Accessor(AccessorIndicesU32::new(primitive, buffers))
        } else {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let indices = reader.read_indices().unwrap()
                .into_u32()
                .collect();
            Self::Vector(indices)
        }
    }

    fn count(&self) -> usize {
        match &self {
            &Self::Accessor(indices) => indices.count,
            &Self::Vector(v) => v.len(),
        }
    }

    fn data(&self) -> *const u8 {
        match &self {
            &Self::Accessor(indices) => indices.slice.as_ptr(),
            &Self::Vector(v) => v.as_ptr() as *const _ as *const u8,
        }
    }
}

struct AccessorIndicesU32<'a> {
    slice: &'a [u8],
    count: usize,
}

impl<'a> AccessorIndicesU32<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let indices = primitive.indices().unwrap();
        let view = indices.view().unwrap();
        let buffer = buffers.get(view.buffer().index()).unwrap();
        let offset = view.offset() + indices.offset();
        let slice = &buffer[offset..offset + view.length()];
        Self {
            slice,
            count: indices.count(),
        }
    }
}

enum Positions<'a> {
    Accessor(AccessorPositions<'a>),
    Vector(Vec<[f32; 3]>)
}

impl<'a> Positions<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let positions = primitive.attributes()
            .find_map(|(semantic, accessor)| 
                match semantic { 
                    Semantic::Positions => Some(accessor),
                    _ => None,
                }
            )
            .unwrap();
        let view = positions.view().unwrap();
        let use_reference = view.stride() == None
            && positions.data_type() == DataType::F32 
            && positions.dimensions() == Dimensions::Vec3;
        if use_reference {
            Self::Accessor(AccessorPositions::new(&positions, buffers))
        } else {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions = reader.read_positions().unwrap();
            Self::Vector(positions.collect())
        }
    }

    fn count(&self) -> usize {
        match &self {
            &Self::Accessor(positions) => positions.count,
            &Self::Vector(v) => v.len(),
        }
    }

    fn data(&self) -> *const u8 {
        match &self {
            &Self::Accessor(positions) => positions.slice.as_ptr(),
            &Self::Vector(v) => v.as_ptr() as *const _ as *const u8,
        }
    }
}

struct AccessorPositions<'a> {
    slice: &'a [u8],
    count: usize,
}

impl<'a> AccessorPositions<'a> {
    fn new(positions: &gltf::Accessor<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let view = positions.view().unwrap();
        let buffer = buffers.get(view.buffer().index()).unwrap();
        let offset = view.offset() + positions.offset();
        let slice = &buffer[offset..offset + view.length()];
        Self {
            slice,
            count: positions.count(),
        }
    }
}

enum Normals<'a> {
    Accessor(AccessorNormals<'a>),
    Vector(Vec<[f32; 3]>)
}

impl<'a> Normals<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let normals = primitive.attributes()
            .find_map(|(semantic, accessor)| 
                match semantic { 
                    Semantic::Normals => Some(accessor),
                    _ => None,
                }
            )
            .unwrap();
        let view = normals.view().unwrap();
        let use_reference = view.stride() == None
            && normals.data_type() == DataType::F32 
            && normals.dimensions() == Dimensions::Vec3;
        if use_reference {
            Self::Accessor(AccessorNormals::new(&normals, buffers))
        } else {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let normals = reader.read_normals().unwrap();
            Self::Vector(normals.collect())
        }
    }

    fn count(&self) -> usize {
        match &self {
            &Self::Accessor(normals) => normals.count,
            &Self::Vector(v) => v.len(),
        }
    }

    fn data(&self) -> *const u8 {
        match &self {
            &Self::Accessor(normals) => normals.slice.as_ptr(),
            &Self::Vector(v) => v.as_ptr() as *const _ as *const u8,
        }
    }
}

struct AccessorNormals<'a> {
    slice: &'a [u8],
    count: usize,
}

impl<'a> AccessorNormals<'a> {
    fn new(normals: &gltf::Accessor<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let view = normals.view().unwrap();
        let buffer = buffers.get(view.buffer().index()).unwrap();
        let offset = view.offset() + normals.offset();
        let slice = &buffer[offset..offset + view.length()];
        Self {
            slice,
            count: normals.count(),
        }
    }
}
