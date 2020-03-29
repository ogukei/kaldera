
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, ShaderModule, ShaderModuleSource};
use super::memory::{StagingBuffer, StagingBufferUsage};
use super::swapchain::{SwapchainFramebuffers, Framebuffer, RenderPass};
use super::pipeline::{GraphicsPipeline, RenderStagingBuffer};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;
use std::ffi::CString;

pub struct GraphicsRender {
    frames: Vec<GraphicsFrameRender>,
    framebuffers: Arc<SwapchainFramebuffers>,
    pipeline: Arc<GraphicsPipeline>,
    staging_buffer: Arc<RenderStagingBuffer>,
    command_pool: Arc<CommandPool>,
    present_semaphore: Arc<Semaphore>,
    render_semaphore: Arc<Semaphore>,
}

impl GraphicsRender {
    pub fn new(
        framebuffers: &Arc<SwapchainFramebuffers>, 
        pipeline: &Arc<GraphicsPipeline>,
        staging_buffer: &Arc<RenderStagingBuffer>,
        command_pool: &Arc<CommandPool>,
    ) -> Result<Arc<Self>> {
        unsafe { Self::init(framebuffers, pipeline, staging_buffer, command_pool) }
    }

    unsafe fn init(
        framebuffers: &Arc<SwapchainFramebuffers>, 
        pipeline: &Arc<GraphicsPipeline>,
        staging_buffer: &Arc<RenderStagingBuffer>,
        command_pool: &Arc<CommandPool>,
    ) -> Result<Arc<Self>> {
        let device = framebuffers.device();
        let render_pass = framebuffers.render_pass();
        let area = VkRect2D {
            offset: VkOffset2D {
                x: 0,
                y: 0,
            },
            extent: VkExtent2D {
                width: 400,
                height: 400,
            },
        };
        let frame_renders: Result<Vec<GraphicsFrameRender>>;
        frame_renders = framebuffers.framebuffers()
            .iter()
            .map(|framebuffer| GraphicsFrameRender::new(framebuffer, render_pass, command_pool, pipeline, staging_buffer, area))
            .collect();
        let render = GraphicsRender {
            frames: frame_renders?,
            framebuffers: Arc::clone(framebuffers),
            pipeline: Arc::clone(pipeline),
            staging_buffer: Arc::clone(staging_buffer),
            command_pool: Arc::clone(command_pool),
            present_semaphore: Semaphore::new(device)?,
            render_semaphore: Semaphore::new(device)?,
        };
        Ok(Arc::new(render))
    }

    pub fn draw(&self) -> Result<()> {
        let swapchain = self.framebuffers.swapchain();
        let image = swapchain.acquire_next_image(self.present_semaphore.handle())?;
        let frame = self.frames.get(image.index())
            .ok_or_else(|| ErrorCode::RenderFrameNotFound)?;
        let command_buffer = frame.command_buffer();
        command_buffer.wait_and_reset();
        command_buffer.submit(
            VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as VkPipelineStageFlags,
            self.present_semaphore.handle(),
            self.render_semaphore.handle(),
        )?;
        swapchain.queue_present(image, self.render_semaphore.handle())?;
        Ok(())
    }
}

struct GraphicsFrameRender {
    command_buffer: Arc<CommandBuffer>,
}

impl GraphicsFrameRender {
    unsafe fn new(
        framebuffer: &Framebuffer, 
        render_pass: &Arc<RenderPass>,
        command_pool: &Arc<CommandPool>, 
        pipeline: &Arc<GraphicsPipeline>,
        staging_buffer: &Arc<RenderStagingBuffer>,
        area: VkRect2D
    ) -> Result<Self> {
        let command_buffer = CommandBufferBuilder::new(command_pool).build(|command_buffer| {
            let clear_values = vec![
                VkClearValue {
                    values: [0.0, 0.0, 0.2, 1.0],
                },
                VkClearValue {
                    values: [1.0, 0.0, 0.0, 0.0],
                },
            ];
            let render_pass_begin_info = VkRenderPassBeginInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
                pNext: ptr::null(),
                renderPass: render_pass.handle(),
                framebuffer: framebuffer.handle(),
                renderArea: area,
                clearValueCount: clear_values.len() as u32,
                pClearValues: clear_values.as_ptr(),
            };
            vkCmdBeginRenderPass(command_buffer, &render_pass_begin_info, VkSubpassContents::VK_SUBPASS_CONTENTS_INLINE);
            let viewport = VkViewport {
                x: 0.0,
                y: 0.0,
                width: area.extent.width as c_float,
                height: area.extent.height as c_float,
                minDepth: 0.0,
                maxDepth: 1.0,
            };
            vkCmdSetViewport(command_buffer, 0, 1, &viewport);
            let scissor = area;
            vkCmdSetScissor(command_buffer, 0, 1, &scissor);
            vkCmdBindPipeline(command_buffer, VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline.handle());
            let offset: VkDeviceSize = 0;
            let vertex_buffer: VkBuffer = staging_buffer.vertex_buffer().device_buffer_memory().buffer();
            vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer, &offset);
            let index_buffer: VkBuffer = staging_buffer.index_buffer().device_buffer_memory().buffer();
            vkCmdBindIndexBuffer(command_buffer, index_buffer, 0, VkIndexType::VK_INDEX_TYPE_UINT32);
            vkCmdDrawIndexed(command_buffer, staging_buffer.index_count() as u32, 1, 0, 0, 0);
            vkCmdEndRenderPass(command_buffer);
        });
        let render = GraphicsFrameRender {
            command_buffer,
        };
        Ok(render)
    }

    #[inline]
    fn command_buffer(&self) -> &Arc<CommandBuffer> {
        &self.command_buffer
    }
}

struct Semaphore {
    device: Arc<Device>,
    handle: VkSemaphore,
}

impl Semaphore {
    pub fn new(device: &Arc<Device>) -> Result<Arc<Self>> {
        unsafe {
            let create_info = VkSemaphoreCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
            };
            let mut handle = MaybeUninit::<VkSemaphore>::zeroed();
            vkCreateSemaphore(device.handle(), &create_info, ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            let semaphore = Semaphore {
                device: Arc::clone(device),
                handle,
            };
            Ok(Arc::new(semaphore))
        }
    }

    #[inline]
    pub fn handle(&self) -> VkSemaphore {
        self.handle
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        log_debug!("Drop Semaphore");
        unsafe {
            vkDestroySemaphore(self.device.handle(), self.handle, ptr::null());
        }
    }
}
