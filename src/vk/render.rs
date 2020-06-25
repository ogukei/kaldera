
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, ShaderModule, ShaderModuleSource};
use super::memory::{StagingBuffer, StagingBufferUsage};
use super::swapchain::{SwapchainFramebuffers, SwapchainFramebuffer, SceneRenderPass};
use super::offscreen::{OffscreenFramebuffer, OffscreenGraphicsPipeline};
use super::scene::{SceneGraphicsRender};
use super::raytrace::{RayTracingGraphicsRender};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;
use std::ffi::CString;

pub struct GraphicsRender {
    frames: Vec<GraphicsFramePrerender>,
    swapchain_framebuffers: Arc<SwapchainFramebuffers>,
    frame_renderer: Arc<GraphicsFrameRenderer>,
    command_pool: Arc<CommandPool>,
    present_semaphore: Arc<Semaphore>,
    render_semaphore: Arc<Semaphore>,
}

impl GraphicsRender {
    pub fn new(
        command_pool: &Arc<CommandPool>,
        swapchain_framebuffers: &Arc<SwapchainFramebuffers>, 
        frame_renderer: &Arc<GraphicsFrameRenderer>,
        extent: VkExtent2D,
    ) -> Result<Arc<Self>> {
        unsafe {
            Self::init(command_pool, swapchain_framebuffers, frame_renderer, extent)
        }
    }

    pub unsafe fn init(
        command_pool: &Arc<CommandPool>,
        swapchain_framebuffers: &Arc<SwapchainFramebuffers>, 
        frame_renderer: &Arc<GraphicsFrameRenderer>,
        extent: VkExtent2D,
    ) -> Result<Arc<Self>> {
        let device = swapchain_framebuffers.device();
        let area = VkRect2D {
            offset: VkOffset2D {
                x: 0,
                y: 0,
            },
            extent: extent,
        };
        let frames: Vec<GraphicsFramePrerender>;
        frames = swapchain_framebuffers.framebuffers()
            .iter()
            .map(|framebuffer| frame_renderer.render(command_pool, framebuffer, area))
            .collect();
        let render = GraphicsRender {
            frames,
            swapchain_framebuffers: Arc::clone(swapchain_framebuffers),
            frame_renderer: Arc::clone(frame_renderer),
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

struct GraphicsFramePrerender {
    command_buffer: Arc<CommandBuffer>,
}

impl GraphicsFramePrerender {
    pub fn new(command_buffer: Arc<CommandBuffer>) -> Self {
        Self { 
            command_buffer,
        }
    }

    #[inline]
    fn command_buffer(&self) -> &Arc<CommandBuffer> {
        &self.command_buffer
    }
}

pub struct GraphicsFrameRenderer {
    raytracing_render: Arc<RayTracingGraphicsRender>,
    scene_render: Arc<SceneGraphicsRender>,
}

impl GraphicsFrameRenderer {
    pub fn new(
        raytracing_render: &Arc<RayTracingGraphicsRender>,
        scene_render: &Arc<SceneGraphicsRender>,
    ) -> Result<Arc<Self>> {
        let renderer = Self {
            raytracing_render: Arc::clone(raytracing_render),
            scene_render: Arc::clone(scene_render),
        };
        Ok(Arc::new(renderer))
    }

    unsafe fn render(&self, 
        command_pool: &Arc<CommandPool>, 
        framebuffer: &SwapchainFramebuffer, 
        area: VkRect2D,
    ) -> GraphicsFramePrerender {
        let command_buffer = CommandBufferBuilder::new(command_pool).build(|command_buffer| {
            self.raytracing_render.command(command_buffer, area);
            self.scene_render.command(command_buffer, framebuffer, area);
        });
        GraphicsFramePrerender::new(command_buffer)
    }
}
