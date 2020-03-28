
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder};
use super::memory::{StagingBuffer, StagingBufferUsage};
use super::swapchain::{SwapchainFramebuffers};

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

struct PipelineLayout {
    device: Arc<Device>,
    handle: VkPipelineLayout,
}

impl PipelineLayout {
    unsafe fn new(device: &Arc<Device>) -> Result<Arc<Self>> {
        let mut handle = MaybeUninit::<VkPipelineLayout>::zeroed();
        {
            let create_info = VkPipelineLayoutCreateInfo::new(0, ptr::null());
            vkCreatePipelineLayout(device.handle(), &create_info, ptr::null(), handle.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let handle = handle.assume_init();
        let layout = PipelineLayout {
            device: Arc::clone(device),
            handle,
        };
        Ok(Arc::new(layout))
    }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        log_debug!("Drop PipelineLayout");
        unsafe {
            vkDestroyPipelineLayout(self.device.handle(), self.handle, ptr::null());
        }
    }
}

struct GraphicsPipeline {

}

impl GraphicsPipeline {
    fn new(framebuffers: &Arc<SwapchainFramebuffers>) -> Result<Arc<Self>> {

    }
}
