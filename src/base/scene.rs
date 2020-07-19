
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
        let (document, buffers, images) = gltf::import("submodules/kaldera-asset/models/Sponza/glTF/Sponza.gltf").unwrap();
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
        log_debug!("iterating materials");
        let materials: Vec<_> = asset.document().materials()
            .into_iter()
            .map(|v| Material::new(v, asset.images()))
            .collect();
        log_debug!("iterating materials complete");
        Scene::new(&table, &nodes, &materials, command_pool)
    }
}

enum MaterialImagePixels<'a> {
    Ref(&'a Vec<u8>),
    Vec(Vec<u8>),
}

impl<'a> MaterialImagePixels<'a> {
    fn new(image: &'a gltf::image::Data) -> Option<Self> {
        use gltf::image::Format;
        match image.format {
            Format::R8G8B8 => {
                let bytes = image.width as usize * image.height as usize * 4usize;
                let mut pixels: Vec<u8> = Vec::with_capacity(bytes);
                for rgb in image.pixels.chunks(3) {
                    pixels.extend_from_slice(rgb);
                    pixels.push(0xffu8);
                }
                Some(Self::Vec(pixels))
            },
            Format::R8G8B8A8 => {
                Some(Self::Ref(&image.pixels))
            },
            _ => None,
        }
    }

    fn pixels(&self) -> &Vec<u8> {
        match &self {
            &Self::Ref(v) => v,
            &Self::Vec(v) => v,
        }
    }
}

struct Material<'a> {
    image: &'a gltf::image::Data,
    material: gltf::material::Material<'a>,
    pixels: MaterialImagePixels<'a>,
}

impl<'a> Material<'a> {
    fn new(material: gltf::material::Material<'a>, images: &'a Vec<gltf::image::Data>) -> Self {
        let model = material.pbr_metallic_roughness();
        let color = model.base_color_texture().unwrap();
        let image_index = color.texture().source().index();
        let image = images.get(image_index).unwrap();
        let pixels = MaterialImagePixels::new(image).unwrap();
        Self {
            image,
            material,
            pixels,
        }
    }
}

struct AABB {
    min: glm::Vec3,
    max: glm::Vec3,
}

impl AABB {
    fn sphere(center: &glm::Vec3, radius: f32) -> Self {
        let radius = glm::vec3(radius, radius, radius);
        let min = center - radius;
        let max = center + radius;
        Self {
            min,
            max,
        }
    }
}

struct Xorshift64 {
    x: u64,
}

impl Xorshift64 {
    fn new() -> Self {
        Self { x: 88172645463325252, }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.x;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.x = x;
        x
    }

    // @see http://prng.di.unimi.it/
    // xoshiro / xoroshiro generators and the PRNG shootout, 
    // section "Generating uniform doubles in the unit interval"
    fn next_uniform(&mut self) -> f64 {
        let v = self.next();
        let v = (v >> 12) | 0x3ff0000000000000u64;
        f64::from_bits(v) - 1.0
    }

    fn next_uniform_f32(&mut self) -> f32 {
        self.next_uniform() as f32
    }
}

struct SphereMaterial {
    albedo: glm::Vec4,
    tp: glm::UVec4,
}

impl SphereMaterial {
    fn lambertian(x: f32, y: f32, z: f32) -> Self {
        let albedo = glm::vec4(x, y, z, 0.0);
        let tp = glm::vec4(0u32, 0, 0, 0);
        Self {
            albedo,
            tp,
        }
    }

    fn metal(x: f32, y: f32, z: f32, w: f32) -> Self {
        let albedo = glm::vec4(x, y, z, w);
        let tp = glm::vec4(1u32, 0, 0, 0);
        Self {
            albedo,
            tp,
        }
    }

    fn dielectric() -> Self {
        let albedo = glm::vec4(0.0, 0.0, 0.0, 1.5);
        let tp = glm::vec4(2u32, 0, 0, 0);
        Self {
            albedo,
            tp,
        }
    }

    fn random(rng: &mut Xorshift64) -> Self {
        let choose = rng.next_uniform();
        if choose < 0.8 {
            Self::lambertian(
                rng.next_uniform_f32() * rng.next_uniform_f32(), 
                rng.next_uniform_f32() * rng.next_uniform_f32(), 
                rng.next_uniform_f32() * rng.next_uniform_f32())
        } else if choose < 0.98 {
            let x = rng.next_uniform_f32();
            let y = rng.next_uniform_f32();
            let z = rng.next_uniform_f32();
            let w = 0.5 * rng.next_uniform_f32();
            Self::metal(x, y, z, w)
        } else {
            Self::dielectric()
        }
    }
}

struct SphereGenerator {
    rng: Xorshift64,
}

impl SphereGenerator {
    fn new() -> Self {
        Self { rng: Xorshift64::new() }
    }

    fn gen_vec4(&mut self, x: isize, z: isize) -> glm::Vec4 {
        let x = x as f32 + 0.9 * self.rng.next_uniform_f32();
        let y = 0.2;
        let z = z as f32 + 0.9 * self.rng.next_uniform_f32();
        let v = glm::vec4(x, y, z, 0.2);
        v
    }

    fn generate(&mut self, spheres: &mut Vec<glm::Vec4>, sphere_materials: &mut Vec<SphereMaterial>) {
        let mut rng = Xorshift64::new();
        for x in -7..7isize {
            for z in -7..7isize {
                let mut v: glm::Vec4;
                loop {
                    v = self.gen_vec4(x, z);
                    let intersects = spheres.iter()
                        .any(|s| glm::distance(&s.xyz(), &v.xyz()) < (s.w + v.w));
                    if !intersects {
                        break;
                    }
                }
                spheres.push(v);
                sphere_materials.push(SphereMaterial::random(&mut rng));
            }
        }
    }
}

struct SceneProceduralGeometry {
    aabb_buffer: Arc<DedicatedStagingBuffer>,
    sphere_buffer: Arc<DedicatedStagingBuffer>,
    structures: Vec<Arc<BottomLevelAccelerationStructure>>,
    sphere_material_buffer: Arc<DedicatedStagingBuffer>,
}

impl SceneProceduralGeometry {
    fn new(aabbs: &Vec<AABB>, spheres: &Vec<glm::Vec4>, materials: &Vec<SphereMaterial>, command_pool: &Arc<CommandPool>) -> Result<Arc<Self>> {
        let device = command_pool.queue().device();
        let num_aabb = aabbs.len();
        let aabb_buffer_size = num_aabb * std::mem::size_of::<AABB>();
        let aabb_buffer = DedicatedStagingBuffer::new(command_pool, 
                VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            aabb_buffer_size as VkDeviceSize,
        )?;
        let sphere_buffer_size = spheres.len() * std::mem::size_of::<[f32; 4]>();
        let sphere_buffer = DedicatedStagingBuffer::new(command_pool, 
                VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            sphere_buffer_size as VkDeviceSize,
        )?;
        let sphere_material_buffer_size = spheres.len() * std::mem::size_of::<SphereMaterial>();
        let sphere_material_buffer = DedicatedStagingBuffer::new(command_pool, 
                VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            sphere_material_buffer_size as VkDeviceSize,
        )?;
        unsafe {
            aabb_buffer.update(aabb_buffer_size as VkDeviceSize, |data| {
                let dst = data as *mut u8;
                std::ptr::copy_nonoverlapping(aabbs.as_ptr() as *const u8, dst, aabb_buffer_size);
            });
            sphere_buffer.update(sphere_buffer_size as VkDeviceSize, |data| {
                let dst = data as *mut u8;
                std::ptr::copy_nonoverlapping(spheres.as_ptr() as *const u8, dst, sphere_buffer_size);
            });
            sphere_material_buffer.update(sphere_material_buffer_size as VkDeviceSize, |data| {
                let dst = data as *mut u8;
                std::ptr::copy_nonoverlapping(materials.as_ptr() as *const u8, dst, sphere_material_buffer_size);
            });
        }
        assert_eq!(std::mem::size_of::<AABB>(), std::mem::size_of::<[f32; 6]>());
        let structure_geometry = BottomLevelAccelerationStructureAABBsGeometry::new(
            num_aabb as u32, 
            aabb_buffer.device_buffer_memory())?;
        let builder = BottomLevelAccelerationStructuresBuilder::new(command_pool, std::slice::from_ref(&structure_geometry));
        let structures = builder.build()?;
        let geometry = Self {
            aabb_buffer,
            structures,
            sphere_buffer,
            sphere_material_buffer,
        };
        Ok(Arc::new(geometry))
    }

    fn structures(&self) -> &Vec<Arc<BottomLevelAccelerationStructure>> {
        &self.structures
    }

    fn sphere_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.sphere_buffer
    }

    fn sphere_material_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.sphere_material_buffer
    }
}

pub struct Scene {
    primitives: Vec<Arc<SceneMeshPrimitive>>,
    staging_buffers: Arc<SceneStagingBuffers>,
    top_level_acceleration_structure: Arc<TopLevelAccelerationStructure>,
    materials: Vec<Arc<SceneMeshMaterial>>,
    textures: Vec<Arc<Texture>>,
    procedural: Arc<SceneProceduralGeometry>,
}

impl Scene {
    fn new(table: &MeshTable, nodes: &[MeshNode], materials: &[Material], command_pool: &Arc<CommandPool>) -> Self {
        let primitives = table.mesh_primitives();
        log_debug!("creating staging buffers");
        let staging_buffers = SceneStagingBuffers::new(command_pool, primitives);
        log_debug!("creating staging buffers complete");
        log_debug!("creating material images");
        let materials: Vec<_> = materials.iter()
            .map(|v| SceneMeshMaterial::new(v, command_pool))
            .collect();
        let textures: Vec<_> = materials.iter()
            .map(|v| v.texture())
            .map(Arc::clone)
            .collect();
        log_debug!("creating material images complete");
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
        log_debug!("building blas procedurals");
        let mut spheres = vec![
            glm::vec4(0.0, -1000.0, 0.0, 1000.0), 
            glm::vec4(0.0, 1.0, 0.0, 1.0), 
            glm::vec4(-4.0, 1.0, 0.0, 1.0),
            glm::vec4(4.0, 1.0, 0.0, 1.0),
        ];
        let mut sphere_materials = vec![
            SphereMaterial::lambertian(0.5, 0.5, 0.5),
            SphereMaterial::dielectric(), 
            SphereMaterial::metal(0.7, 0.6, 0.5, 0.0), 
            SphereMaterial::lambertian(0.7, 0.6, 0.5),
        ];
        SphereGenerator::new().generate(&mut spheres, &mut sphere_materials);

        let aabbs: Vec<_> = spheres.iter()
            .map(|v| AABB::sphere(&v.xyz(), v.w))
            .collect();
        let procedural = SceneProceduralGeometry::new(&aabbs, &spheres, &sphere_materials, command_pool).unwrap();
        log_debug!("building blas procedurals complete");
        log_debug!("building tlas");
        let node_scale: f32 = 5.0;
        let node_scale = glm::scaling(&glm::vec3(node_scale, node_scale, node_scale));
        let node_translate = glm::translation(&glm::vec3(0.0, -20.0, 15.0));
        let node_instances = nodes.into_iter()
            .take(0)
            .map(|node| {
                let index = node.primitive().index();
                let mesh_primitive = scene_mesh_primitives.get(index).unwrap();
                let transform = node_translate * node_scale * node.transform();
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
                    0,
                    mesh_primitive.bottom_level_acceleration_structure(),
                ).unwrap();
                instance
            });
        let procedural_instances = procedural.structures().iter()
            .enumerate()
            .map(|(index, structure)| {
                let transform = VkTransformMatrixKHR {
                    matrix: [
                        [1.0, 0.0, 0.0, 0.0,],
                        [0.0, 1.0, 0.0, 0.0,],
                        [0.0, 0.0, 1.0, 0.0,],
                    ]
                };
                let instance = TopLevelAccelerationStructureInstance::new(
                    index as u32,
                    transform,
                    1,
                    structure,
                ).unwrap();
                instance
            });
        let instances = node_instances
            .chain(procedural_instances)
            .collect();
        let top_level_acceleration_structure = TopLevelAccelerationStructure::new(command_pool, instances)
            .unwrap();
        log_debug!("building tlas complete");
        log_debug!("scene building complete");
        Self {
            primitives: scene_mesh_primitives,
            staging_buffers,
            top_level_acceleration_structure,
            materials,
            textures,
            procedural,
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

    pub fn texcoord_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.staging_buffers.texcoord_buffer()
    }

    pub fn textures(&self) -> &Vec<Arc<Texture>> {
        &self.textures
    }

    pub fn sphere_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.procedural.sphere_buffer()
    }

    pub fn material_staging_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.procedural.sphere_material_buffer()
    }
}

pub struct SceneMeshMaterial {
    texture: Arc<Texture>,
}

impl SceneMeshMaterial {
    fn new(material: &Material, command_pool: &Arc<CommandPool>) -> Arc<Self> {
        let pixels = material.pixels.pixels();
        let data = pixels.as_ptr() as *const c_void;
        let data_size = pixels.len();
        let extent = VkExtent3D {
            width: material.image.width,
            height: material.image.height,
            depth: 1,
        };
        let device = command_pool.queue().device();
        let texture_image = TextureImage::new(device, extent).unwrap();
        let texture = Texture::new(command_pool, &texture_image, data, data_size).unwrap();
        let mesh_material = Self {
            texture,
        };
        Arc::new(mesh_material)
    }

    pub fn texture(&self) -> &Arc<Texture> {
        &self.texture
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceneMeshPrimitiveDescription {
    vertex_offset: u32,
    index_offset: u32,
    texture_index: u32,
    reserved: u32,
}

impl SceneMeshPrimitiveDescription {
    fn new(offset: MeshPrimitiveOffset, material_index: u32) -> Self {
        Self {
            vertex_offset: offset.vertex_offset as u32,
            index_offset: offset.index_offset as u32,
            texture_index: material_index,
            reserved: 0u32,
        }
    }
}

pub struct SceneStagingBuffers {
    vertex_buffer: Arc<DedicatedStagingBuffer>,
    index_buffer: Arc<DedicatedStagingBuffer>,
    normals_buffer: Arc<DedicatedStagingBuffer>,
    description_buffer: Arc<DedicatedStagingBuffer>,
    texcoord_buffer: Arc<DedicatedStagingBuffer>,
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
            .map(|v| SceneMeshPrimitiveDescription::new(v.offset.clone(), v.material_index() as u32))
            .collect();
        let description_buffer_size = std::mem::size_of::<SceneMeshPrimitiveDescription>() * descriptions.len();
        let description_buffer = DedicatedStagingBuffer::new(
            command_pool, 
            VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            description_buffer_size as VkDeviceSize,
        ).unwrap();
        let texcoord_buffer_size = std::mem::size_of::<[f32; 2]>() * num_vertices;
        let texcoord_buffer = DedicatedStagingBuffer::new(
            command_pool, 
            VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            texcoord_buffer_size as VkDeviceSize,
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
            texcoord_buffer.update(texcoord_buffer_size as VkDeviceSize, |data| {
                let data = data as *mut u8;
                for primitive in primitives.iter() {
                    let byte_size = primitive.primitive.texcoords.count() * std::mem::size_of::<[f32; 2]>();
                    let byte_offset = primitive.offset.vertex_offset * std::mem::size_of::<[f32; 2]>();
                    let dst = data.offset(byte_offset as isize);
                    let src = primitive.primitive.texcoords.data();
                    std::ptr::copy_nonoverlapping(src, dst, byte_size);
                }
            });
        }
        let buffer = Self {
            vertex_buffer,
            index_buffer,
            normals_buffer,
            description_buffer,
            texcoord_buffer,
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

    #[inline]
    pub fn texcoord_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.texcoord_buffer
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
    material_index: Option<usize>,
}

impl SceneMeshPrimitiveGeometry {
    fn new(mesh_primitive: &MeshPrimitive, staging_buffers: &Arc<SceneStagingBuffers>, command_pool: &Arc<CommandPool>) -> Arc<Self> {
        let vertex_stride = std::mem::size_of::<[f32; 3]>();
        let num_vertices = mesh_primitive.primitive.positions.count();
        let num_indices = mesh_primitive.primitive.indices.count();
        assert_eq!(mesh_primitive.primitive.positions.count(), mesh_primitive.primitive.normals.count());
        let structure_geometry = BottomLevelAccelerationStructureTrianglesGeometry::new(
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
            material_index: mesh_primitive.primitive.material_index,
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
    material_index: usize,
    offset: MeshPrimitiveOffset,
    primitive: Primitive<'a>,
}

impl<'a> MeshPrimitive<'a> {
    fn new(mesh_primitive_index: usize, mesh_index: usize, offset: MeshPrimitiveOffset, primitive: Primitive<'a>) -> Self {
        Self {
            mesh_index,
            mesh_primitive_index,
            material_index: primitive.material_index.unwrap_or(0),
            offset,
            primitive,
        }
    }

    fn mesh_index(&self) -> usize {
        self.mesh_index
    }

    fn material_index(&self) -> usize {
        self.material_index
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
    texcoords: Texcoords<'a>,
    material_index: Option<usize>,
}

impl<'a> Primitive<'a> {
    fn new(primitive: gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let material_index = primitive.material().index();
        Self {
            indices: Indices::new(&primitive, buffers),
            positions: Positions::new(&primitive, buffers),
            normals: Normals::new(&primitive, buffers),
            // TODO(ogukei): support default TEXCOORD_0
            texcoords: Texcoords::new(&primitive, buffers).unwrap(),
            material_index,
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

enum Texcoords<'a> {
    Accessor(AccessorTexcoords<'a>),
    Vector(Vec<[f32; 2]>)
}

impl<'a> Texcoords<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Option<Self> {
        let texcoords = primitive.attributes()
            .find_map(|(semantic, accessor)| 
                match semantic { 
                    Semantic::TexCoords(0) => Some(accessor),
                    _ => None,
                }
            )?;
        let view = texcoords.view().unwrap();
        let use_reference = view.stride() == None
            && texcoords.data_type() == DataType::F32 
            && texcoords.dimensions() == Dimensions::Vec2;
        if use_reference {
            Some(Self::Accessor(AccessorTexcoords::new(&texcoords, buffers)))
        } else {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let texcoords = reader.read_tex_coords(0).unwrap().into_f32();
            Some(Self::Vector(texcoords.collect()))
        }
    }

    fn count(&self) -> usize {
        match &self {
            &Self::Accessor(texcoords) => texcoords.count,
            &Self::Vector(v) => v.len(),
        }
    }

    fn data(&self) -> *const u8 {
        match &self {
            &Self::Accessor(texcoords) => texcoords.slice.as_ptr(),
            &Self::Vector(v) => v.as_ptr() as *const _ as *const u8,
        }
    }
}

struct AccessorTexcoords<'a> {
    slice: &'a [u8],
    count: usize,
}

impl<'a> AccessorTexcoords<'a> {
    fn new(texcoords: &gltf::Accessor<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let view = texcoords.view().unwrap();
        let buffer = buffers.get(view.buffer().index()).unwrap();
        let offset = view.offset() + texcoords.offset();
        let slice = &buffer[offset..offset + view.length()];
        Self {
            slice,
            count: texcoords.count(),
        }
    }
}
