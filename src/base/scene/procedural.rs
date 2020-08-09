

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

use super::aabb::*;
use super::material::*;

pub struct SceneProceduralGeometry {
    aabb_buffer: Arc<DedicatedStagingBuffer>,
    sphere_buffer: Arc<DedicatedStagingBuffer>,
    structures: Vec<Arc<BottomLevelAccelerationStructure>>,
    sphere_material_buffer: Arc<DedicatedStagingBuffer>,
}

impl SceneProceduralGeometry {
    pub fn new(aabbs: &Vec<AABB>, spheres: &Vec<glm::Vec4>, materials: &Vec<SphereMaterial>, command_pool: &Arc<CommandPool>) -> Result<Arc<Self>> {
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

    pub fn structures(&self) -> &Vec<Arc<BottomLevelAccelerationStructure>> {
        &self.structures
    }

    pub fn sphere_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.sphere_buffer
    }

    pub fn sphere_material_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.sphere_material_buffer
    }
}

pub struct SphereGenerator {
    rng: Xorshift64,
}

impl SphereGenerator {
    pub fn new() -> Self {
        Self { rng: Xorshift64::new() }
    }

    fn gen_vec4(&mut self, x: isize, z: isize) -> glm::Vec4 {
        let x = x as f32 + 0.9 * self.rng.next_uniform_f32();
        let y = 0.2;
        let z = z as f32 + 0.9 * self.rng.next_uniform_f32();
        let v = glm::vec4(x, y, z, 0.2);
        v
    }

    pub fn generate(&mut self, spheres: &mut Vec<glm::Vec4>, sphere_materials: &mut Vec<SphereMaterial>) {
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

pub struct SphereMaterial {
    albedo: glm::Vec4,
    tp: glm::UVec4,
}

impl SphereMaterial {
    pub fn lambertian(x: f32, y: f32, z: f32) -> Self {
        let albedo = glm::vec4(x, y, z, 0.0);
        let tp = glm::vec4(0u32, 0, 0, 0);
        Self {
            albedo,
            tp,
        }
    }

    pub fn metal(x: f32, y: f32, z: f32, w: f32) -> Self {
        let albedo = glm::vec4(x, y, z, w);
        let tp = glm::vec4(1u32, 0, 0, 0);
        Self {
            albedo,
            tp,
        }
    }

    pub fn dielectric() -> Self {
        let albedo = glm::vec4(0.0, 0.0, 0.0, 1.5);
        let tp = glm::vec4(2u32, 0, 0, 0);
        Self {
            albedo,
            tp,
        }
    }

    pub fn random(rng: &mut Xorshift64) -> Self {
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

