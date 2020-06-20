
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, ShaderModule, ShaderModuleSource};
use super::memory::{StagingBuffer, StagingBufferUsage, DedicatedStagingBuffer};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;
use std::ffi::CString;

pub struct VertexStagingBuffer {
    vertex_buffer: Arc<StagingBuffer>,
    index_buffer: Arc<StagingBuffer>,
    index_count: usize,
}

impl VertexStagingBuffer {
    pub fn new<Vertex>(command_pool: &Arc<CommandPool>, vertices: Vec<Vertex>, indices: Vec<u32>) -> Arc<Self> {
        let vertex_buffer_size = std::mem::size_of::<Vertex>() * vertices.len();
        let vertex_buffer = StagingBuffer::new(
            command_pool, 
            StagingBufferUsage::Vertex, 
            vertex_buffer_size as VkDeviceSize)
            .unwrap();
        let index_buffer_size = std::mem::size_of::<u32>() * indices.len();
        let index_buffer = StagingBuffer::new(
            command_pool, 
            StagingBufferUsage::Index, 
            index_buffer_size as VkDeviceSize)
            .unwrap();
        // transfer
        vertex_buffer.write(vertices.as_ptr() as *const c_void, vertex_buffer_size);
        index_buffer.write(indices.as_ptr() as *const c_void, index_buffer_size);
        let buffer = VertexStagingBuffer {
            vertex_buffer,
            index_buffer,
            index_count: indices.len(),
        };
        Arc::new(buffer)
    }

    #[inline]
    pub fn vertex_buffer(&self) -> &Arc<StagingBuffer> {
        &self.vertex_buffer
    }

    #[inline]
    pub fn index_buffer(&self) -> &Arc<StagingBuffer> {
        &self.index_buffer
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.index_count
    }
}
