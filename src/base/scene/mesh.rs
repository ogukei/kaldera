
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

use super::asset::*;
use super::material::*;
use super::buffer::*;
use super::primitive::*;

pub struct SceneMeshMaterial {
    color_texture: Option<Arc<Texture>>,
    normal_texture: Option<Arc<Texture>>,
}

impl SceneMeshMaterial {
    pub fn new(material: &Material, command_pool: &Arc<CommandPool>) -> Arc<Self> {
        let color_texture = material.color_image()
            .map(|image| {
                let pixels = image.pixels().pixels();
                let data = pixels.as_ptr() as *const c_void;
                let data_size = pixels.len();
                let extent = VkExtent3D {
                    width: image.width(),
                    height: image.height(),
                    depth: 1,
                };
                let device = command_pool.queue().device();
                let mipmaps = true;
                let texture_image = TextureImage::new(device, extent, VkFormat::VK_FORMAT_R8G8B8A8_SRGB, mipmaps).unwrap();
                let texture = Texture::new(command_pool, &texture_image, data, data_size).unwrap();
                texture
            });
        let normal_texture = material.normal_image()
            .map(|image| {
                let pixels = image.pixels().pixels();
                let data = pixels.as_ptr() as *const c_void;
                let data_size = pixels.len();
                let extent = VkExtent3D {
                    width: image.width(),
                    height: image.height(),
                    depth: 1,
                };
                let device = command_pool.queue().device();
                let mipmaps = true;
                let texture_image = TextureImage::new(device, extent, VkFormat::VK_FORMAT_R8G8B8A8_UNORM, mipmaps).unwrap();
                let texture = Texture::new(command_pool, &texture_image, data, data_size).unwrap();
                texture
            });
        let mesh_material = Self {
            color_texture,
            normal_texture,
        };
        Arc::new(mesh_material)
    }

    pub fn new_placeholder(command_pool: &Arc<CommandPool>) -> Arc<Self> {
        let extent = VkExtent3D {
            width: 1,
            height: 1,
            depth: 1,
        };
        let mut data: Vec<u8> = vec![];
        data.resize(4, 255u8);
        let device = command_pool.queue().device();
        let mipmaps = false;
        let texture_image = TextureImage::new(device, extent, VkFormat::VK_FORMAT_R8G8B8A8_SRGB, mipmaps).unwrap();
        let texture = Texture::new(command_pool, &texture_image, data.as_ptr() as *const c_void, data.len()).unwrap();
        let mesh_material = Self {
            color_texture: Some(texture),
            normal_texture: None,
        };
        Arc::new(mesh_material)
    }

    pub fn color_texture(&self) -> Option<&Arc<Texture>> {
        self.color_texture.as_ref()
    }

    pub fn normal_texture(&self) -> Option<&Arc<Texture>> {
        self.normal_texture.as_ref()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceneMeshPrimitiveDescription {
    vertex_offset: u32,
    index_offset: u32,
    material_index: u32,
    flags: u32,
}

impl SceneMeshPrimitiveDescription {
    pub fn new(offset: MeshPrimitiveOffset, material_index: u32, use_color_multipliers: bool) -> Self {
        let flags = if use_color_multipliers {
            SceneMeshPrimitiveDescriptionFlags::UseColorMultipliers as u32
        } else {
            0u32
        };
        Self {
            vertex_offset: offset.vertex_offset as u32,
            index_offset: offset.index_offset as u32,
            material_index: material_index,
            flags,
        }
    }
}

#[repr(C)]
pub enum SceneMeshPrimitiveDescriptionFlags {
    UseColorMultipliers = 1,
}

pub struct MeshTable<'a> {
    primitives: Vec<MeshPrimitive<'a>>,
    mesh_table: HashMap<usize, Vec<usize>>,
}

impl<'a> MeshTable<'a> {
    pub fn new(asset: &'a SceneAsset) -> Self {
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
                state.index_offset += primitive.indices().count();
                state.vertex_offset += primitive.positions().count();
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

    pub fn mesh_primitives(&self) -> &Vec<MeshPrimitive<'a>> {
        &self.primitives
    }
}

pub struct MeshNode<'a, 'b: 'a> {
    primitive: &'b MeshPrimitive<'a>,
    transform: glm::Mat4,
}

impl<'a, 'b: 'a> MeshNode<'a, 'b> {
    pub fn new(node: FlattenNode<'a>, mesh_table: &'b MeshTable<'a>) -> Option<Vec<Self>> {
        let mesh = node.node().mesh()?;
        let transform = node.transform();
        let primitives = mesh_table.get(mesh.index());
        let nodes = primitives.into_iter()
            .map(|primitive| MeshNode { primitive, transform: transform.clone() })
            .collect();
        Some(nodes)
    }

    pub fn primitive(&self) -> &'b MeshPrimitive<'a> {
        self.primitive
    }

    pub fn transform(&self) -> &glm::Mat4 {
        &self.transform
    }
}

pub struct SceneMeshPrimitive {
    geometry: Arc<SceneMeshPrimitiveGeometry>,
    structure: Arc<BottomLevelAccelerationStructure>,
}

impl SceneMeshPrimitive {
    pub fn new(geometry: Arc<SceneMeshPrimitiveGeometry>, structure: Arc<BottomLevelAccelerationStructure>) -> Arc<Self> {
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
    material_index: Option<usize>,
}

impl SceneMeshPrimitiveGeometry {
    pub fn new(mesh_primitive: &MeshPrimitive, staging_buffers: &Arc<SceneStagingBuffers>, command_pool: &Arc<CommandPool>) -> Arc<Self> {
        let vertex_stride = std::mem::size_of::<[f32; 3]>();
        let num_vertices = mesh_primitive.primitive().positions().count();
        let num_indices = mesh_primitive.primitive().indices().count();
        assert_eq!(mesh_primitive.primitive().positions().count(), mesh_primitive.primitive.normals().count());
        let structure_geometry = BottomLevelAccelerationStructureGeometry::triangles(
            num_vertices as u32, 
            vertex_stride as VkDeviceSize,
            mesh_primitive.offset.vertex_offset as u32,
            staging_buffers.vertex_buffer().device_buffer_memory(), 
            num_indices as u32, 
            mesh_primitive.offset.index_offset as u32,
            staging_buffers.index_buffer().device_buffer_memory(),
            mesh_primitive.primitive().is_opaque()
        );
        let v = Self {
            index: mesh_primitive.index(),
            offset: mesh_primitive.offset.clone(),
            staging_buffers: Arc::clone(staging_buffers),
            structure_geometry,
            material_index: mesh_primitive.primitive.material_index(),
        };
        Arc::new(v)
    }
    
    fn staging_buffers(&self) -> &Arc<SceneStagingBuffers> {
        &self.staging_buffers
    }

    fn index(&self) -> usize {
        self.index
    }

    pub fn structure_geometry(&self) -> &Arc<BottomLevelAccelerationStructureGeometry> {
        &self.structure_geometry
    }
}

#[derive(Default, Clone)]
pub struct MeshPrimitiveOffset {
    pub vertex_offset: usize,
    pub index_offset: usize,
}

pub struct MeshPrimitive<'a> {
    mesh_primitive_index: usize,
    mesh_index: usize,
    material_index: usize,
    use_color_multipliers: bool,
    offset: MeshPrimitiveOffset,
    primitive: Primitive<'a>,
}

impl<'a> MeshPrimitive<'a> {
    fn new(mesh_primitive_index: usize, mesh_index: usize, offset: MeshPrimitiveOffset, primitive: Primitive<'a>) -> Self {
        Self {
            mesh_index,
            mesh_primitive_index,
            use_color_multipliers: primitive.colors().is_some(),
            material_index: primitive.material_index().unwrap_or(0),
            offset,
            primitive,
        }
    }

    #[inline]
    pub fn primitive(&self) -> &Primitive<'a> {
        &self.primitive
    }

    #[inline]
    pub fn offset(&self) -> &MeshPrimitiveOffset {
        &self.offset
    }

    #[inline]
    pub fn mesh_index(&self) -> usize {
        self.mesh_index
    }

    #[inline]
    pub fn material_index(&self) -> usize {
        self.material_index
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.mesh_primitive_index
    }

    #[inline]
    pub fn use_color_multipliers(&self) -> bool {
        self.use_color_multipliers
    }
}

struct Mesh<'a> {
    index: usize,
    primitives: Vec<Primitive<'a>>,
}

impl<'a> Mesh<'a> {
    fn new(mesh: gltf::Mesh<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
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

pub struct FlattenNode<'a> {
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

    pub fn flatten(root: gltf::Node<'a>) -> Vec<Self> {
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
