
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

use super::mesh::*;
use super::material::*;


pub struct SceneStagingBuffers {
    vertex_buffer: Arc<DedicatedStagingBuffer>,
    index_buffer: Arc<DedicatedStagingBuffer>,
    normals_buffer: Arc<DedicatedStagingBuffer>,
    description_buffer: Arc<DedicatedStagingBuffer>,
    texcoord_buffer: Arc<DedicatedStagingBuffer>,
    material_description_buffer: Arc<DedicatedStagingBuffer>,
}

impl SceneStagingBuffers {
    pub fn new(command_pool: &Arc<CommandPool>, 
        primitives: &Vec<MeshPrimitive>, 
        material_descriptions: &Vec<SceneMaterialDescription>,
    ) -> Arc<Self> {
        let num_indices: usize = primitives.iter()
            .map(|v| v.primitive().indices().count())
            .sum();
        let num_vertices: usize = primitives.iter()
            .map(|v| v.primitive().positions().count())
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
            .map(|v| SceneMeshPrimitiveDescription::new(v.offset().clone(), v.material_index() as u32))
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
        let material_description_buffer_size = std::mem::size_of::<SceneMaterialDescription>() * material_descriptions.len();
        let material_description_buffer = DedicatedStagingBuffer::new(
            command_pool, 
            VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            material_description_buffer_size as VkDeviceSize,
        ).unwrap();
        // TODO(ogukei): concurrent uploads
        unsafe {
            index_buffer.update(index_buffer_size as VkDeviceSize, |data| {
                let data = data as *mut u8;
                for primitive in primitives.iter() {
                    let byte_size = primitive.primitive().indices().count() * std::mem::size_of::<u32>();
                    let byte_offset = primitive.offset().index_offset * std::mem::size_of::<u32>();
                    let dst = data.offset(byte_offset as isize);
                    let src = primitive.primitive().indices().data();
                    std::ptr::copy_nonoverlapping(src, dst, byte_size);
                }
            });
            vertex_buffer.update(vertex_buffer_size as VkDeviceSize, |data| {
                let data = data as *mut u8;
                for primitive in primitives.iter() {
                    let byte_size = primitive.primitive().positions().count() * std::mem::size_of::<[f32; 3]>();
                    let byte_offset = primitive.offset().vertex_offset * std::mem::size_of::<[f32; 3]>();
                    let dst = data.offset(byte_offset as isize);
                    let src = primitive.primitive().positions().data();
                    std::ptr::copy_nonoverlapping(src, dst, byte_size);
                }
            });
            normals_buffer.update(normals_buffer_size as VkDeviceSize, |data| {
                let data = data as *mut u8;
                for primitive in primitives.iter() {
                    let byte_size = primitive.primitive().normals().count() * std::mem::size_of::<[f32; 3]>();
                    let byte_offset = primitive.offset().vertex_offset * std::mem::size_of::<[f32; 3]>();
                    let dst = data.offset(byte_offset as isize);
                    let src = primitive.primitive().normals().data();
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
                    let byte_size = primitive.primitive().texcoords().count() * std::mem::size_of::<[f32; 2]>();
                    let byte_offset = primitive.offset().vertex_offset * std::mem::size_of::<[f32; 2]>();
                    let dst = data.offset(byte_offset as isize);
                    let src = primitive.primitive().texcoords().data();
                    std::ptr::copy_nonoverlapping(src, dst, byte_size);
                }
            });
            material_description_buffer.update(material_description_buffer_size as VkDeviceSize, |data| {
                let dst = data as *mut u8;
                let src = material_descriptions.as_ptr() as *const u8;
                std::ptr::copy_nonoverlapping(src, dst, material_description_buffer_size);
            });
        }
        let buffer = Self {
            vertex_buffer,
            index_buffer,
            normals_buffer,
            description_buffer,
            texcoord_buffer,
            material_description_buffer,
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

    #[inline]
    pub fn material_description_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.material_description_buffer
    }
}
