
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder};
use super::memory::{StagingBuffer, StagingBufferUsage};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;

#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
pub struct Vertex {
    pub coordinate: Vec4,
    pub color: Vec4,
}

pub struct RenderStagingBuffer {
    vertex_buffer: Arc<StagingBuffer>,
    index_buffer: Arc<StagingBuffer>,
}

impl RenderStagingBuffer {
    pub fn new(command_pool: &Arc<CommandPool>, vertices: Vec<Vertex>, indices: Vec<u32>) -> Arc<Self> {
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
        let buffer = RenderStagingBuffer {
            vertex_buffer,
            index_buffer,
        };
        Arc::new(buffer)
    }
}
