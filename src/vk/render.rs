
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, ShaderModule, ShaderModuleSource};
use super::memory::{StagingBuffer, StagingBufferUsage};
use super::swapchain::{SwapchainFramebuffers, SwapchainFramebuffer, SceneRenderPass};
use super::pipeline::{SceneGraphicsPipeline};
use super::staging::VertexStagingBuffer;
use super::offscreen::{OffscreenFramebuffer, OffscreenGraphicsPipeline};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;
use std::ffi::CString;

pub struct GraphicsRender {
    frames: Vec<GraphicsFrameRender>,
    swapchain_framebuffers: Arc<SwapchainFramebuffers>,
    offscreen_framebuffer: Arc<OffscreenFramebuffer>,
    offscreen_pipeline: Arc<OffscreenGraphicsPipeline>,
    scene_pipeline: Arc<SceneGraphicsPipeline>,
    staging_buffer: Arc<VertexStagingBuffer>,
    command_pool: Arc<CommandPool>,
    present_semaphore: Arc<Semaphore>,
    render_semaphore: Arc<Semaphore>,
}

impl GraphicsRender {
    pub fn new(
        command_pool: &Arc<CommandPool>, 
        swapchain_framebuffers: &Arc<SwapchainFramebuffers>, 
        offscreen_framebuffer: &Arc<OffscreenFramebuffer>,
        offscreen_pipeline: &Arc<OffscreenGraphicsPipeline>,
        scene_pipeline: &Arc<SceneGraphicsPipeline>,
        staging_buffer: &Arc<VertexStagingBuffer>,
        extent: VkExtent2D,
    ) -> Result<Arc<Self>> {
        unsafe {
            Self::init(command_pool, swapchain_framebuffers, offscreen_framebuffer, offscreen_pipeline, scene_pipeline, staging_buffer, extent)
        }
    }

    pub unsafe fn init(
        command_pool: &Arc<CommandPool>, 
        swapchain_framebuffers: &Arc<SwapchainFramebuffers>, 
        offscreen_framebuffer: &Arc<OffscreenFramebuffer>,
        offscreen_pipeline: &Arc<OffscreenGraphicsPipeline>,
        scene_pipeline: &Arc<SceneGraphicsPipeline>,
        staging_buffer: &Arc<VertexStagingBuffer>,
        extent: VkExtent2D
    ) -> Result<Arc<Self>> {
        let device = swapchain_framebuffers.device();
        let area = VkRect2D {
            offset: VkOffset2D {
                x: 0,
                y: 0,
            },
            extent: extent,
        };
        let scene_render_pass = swapchain_framebuffers.render_pass();
        let frame_renders: Result<Vec<GraphicsFrameRender>>;
        frame_renders = swapchain_framebuffers.framebuffers()
            .iter()
            .map(|framebuffer| GraphicsFrameRender::new(
                framebuffer,
                command_pool,
                offscreen_framebuffer, 
                offscreen_pipeline,
                scene_render_pass,
                scene_pipeline,
                staging_buffer, 
                area))
            .collect();
        let render = GraphicsRender {
            frames: frame_renders?,
            swapchain_framebuffers: Arc::clone(swapchain_framebuffers),
            offscreen_framebuffer: Arc::clone(offscreen_framebuffer),
            offscreen_pipeline: Arc::clone(offscreen_pipeline),
            scene_pipeline: Arc::clone(scene_pipeline),
            staging_buffer: Arc::clone(staging_buffer),
            command_pool: Arc::clone(command_pool),
            present_semaphore: Semaphore::new(device)?,
            render_semaphore: Semaphore::new(device)?,
        };
        Ok(Arc::new(render))
    }

    pub fn draw(&self) -> Result<()> {
        let swapchain = self.swapchain_framebuffers.swapchain();
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
        swapchain_framebuffer: &SwapchainFramebuffer, 
        command_pool: &Arc<CommandPool>,
        offscreen_framebuffer: &Arc<OffscreenFramebuffer>,
        offscreen_pipeline: &Arc<OffscreenGraphicsPipeline>,
        scene_render_pass: &Arc<SceneRenderPass>,
        scene_pipeline: &Arc<SceneGraphicsPipeline>,
        staging_buffer: &Arc<VertexStagingBuffer>,
        area: VkRect2D
    ) -> Result<Self> {
        let command_buffer = CommandBufferBuilder::new(command_pool).build(|command_buffer| {
            {
                let render_pass = offscreen_framebuffer.render_pass();
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
                    framebuffer: offscreen_framebuffer.handle(),
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
                vkCmdBindPipeline(command_buffer, VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, offscreen_pipeline.handle());
                let offset: VkDeviceSize = 0;
                let vertex_buffer: VkBuffer = staging_buffer.vertex_buffer().device_buffer_memory().buffer();
                vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer, &offset);
                let index_buffer: VkBuffer = staging_buffer.index_buffer().device_buffer_memory().buffer();
                vkCmdBindIndexBuffer(command_buffer, index_buffer, 0, VkIndexType::VK_INDEX_TYPE_UINT32);
                vkCmdDrawIndexed(command_buffer, staging_buffer.index_count() as u32, 1, 0, 0, 0);
                vkCmdEndRenderPass(command_buffer);
            }
            {
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
                    renderPass: scene_render_pass.handle(),
                    framebuffer: swapchain_framebuffer.handle(),
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
                vkCmdBindDescriptorSets(command_buffer, VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, 
                    scene_pipeline.layout().handle(), 0, 1, &scene_pipeline.layout().descriptor_set(), 0, ptr::null());
                vkCmdBindPipeline(command_buffer, VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, scene_pipeline.handle());
                vkCmdDraw(command_buffer, 3, 1, 0, 0);
                vkCmdEndRenderPass(command_buffer);
            }
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
