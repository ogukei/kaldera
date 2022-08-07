
use crate::ffi::vk::*;

use super::error::Result;
use super::error::ErrorCode;
use super::device::{Device};
use super::surface::{Surface};
use super::device_queues::{DeviceQueues};
use super::image::DepthImage;

use std::ptr;
use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};

pub struct Swapchain {
    handle: VkSwapchainKHR,
    device_queues: Arc<DeviceQueues>,
    images: Vec<SwapchainImage>,
    image_format: VkFormat,
    image_extent: VkExtent2D,
    image_index: Mutex<Option<usize>>,
}

impl Swapchain {
    pub fn new(device_queues: &Arc<DeviceQueues>, surface: &Arc<Surface>, extent: VkExtent2D) -> Result<Arc<Self>> {
        let device = device_queues.device();
        let physical_device = device.physical_device();
        assert_eq!(surface.instance().handle(), device_queues.instance().handle());
        assert!(surface.is_supported(device_queues.present_queue().family(), physical_device).unwrap_or(false));
        // default configuration
        let image_usage_flags = VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as VkImageUsageFlags;
        unsafe {
            let surface_capabilities = surface.capabilities(physical_device)?;
            let surface_formats = surface.formats(physical_device)?;
            
            let surface_format = surface_formats.iter()
                .find(|v| v.format == VkFormat::VK_FORMAT_B8G8R8A8_SRGB)
                .or_else(|| surface_formats.iter().find(|v| v.format == VkFormat::VK_FORMAT_B8G8R8A8_UNORM))
                .ok_or_else(|| ErrorCode::SwapchainSurfaceFormatNotSupported)?;
            let image_count = surface_capabilities.minImageCount + 1;
            let image_format = surface_format.format;
            let present_mode = VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR;
            let (sharing_mode, family_indices) = if device_queues.is_sharing_exclusive() { 
                (VkSharingMode::VK_SHARING_MODE_EXCLUSIVE, vec![])
            } else {
                let family_indices = vec![
                    device_queues.graphics_queue().family().index(),
                    device_queues.present_queue().family().index(),
                ];
                (VkSharingMode::VK_SHARING_MODE_CONCURRENT, family_indices)
            };
            let indices_ptr = if family_indices.is_empty() { 
                std::ptr::null() 
            } else { 
                family_indices.as_ptr() 
            };
            let create_info = VkSwapchainCreateInfoKHR {
                sType: VkStructureTypeExt::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
                pNext: std::ptr::null(),
                flags: 0,
                surface: surface.handle(),
                minImageCount: image_count,
                imageFormat: image_format,
                imageColorSpace: surface_format.colorSpace,
                imageExtent: extent,
                imageArrayLayers: 1,
                imageUsage: image_usage_flags,
                imageSharingMode: sharing_mode,
                queueFamilyIndexCount: family_indices.len() as u32,
                pQueueFamilyIndices: indices_ptr,
                preTransform: surface_capabilities.currentTransform,
                compositeAlpha: VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
                presentMode: present_mode,
                clipped: VK_TRUE,
                oldSwapchain: std::ptr::null_mut(),
            };
            let mut handle = MaybeUninit::<VkSwapchainKHR>::zeroed();
            vkCreateSwapchainKHR(device.handle(), &create_info, std::ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            let images: Result<Vec<SwapchainImage>>;
            {
                let mut count = MaybeUninit::<u32>::zeroed();
                vkGetSwapchainImagesKHR(device.handle(), handle, count.as_mut_ptr(), std::ptr::null_mut())
                    .into_result()
                    .unwrap();
                let size = count.assume_init() as usize;
                let mut image_handles: Vec<VkImage> = Vec::with_capacity(size);
                image_handles.resize(size, std::mem::zeroed());
                vkGetSwapchainImagesKHR(device.handle(), handle, count.as_mut_ptr(), image_handles.as_mut_ptr())
                    .into_result()
                    .unwrap();
                images = image_handles.into_iter()
                    .enumerate()
                    .map(|(i, v)| SwapchainImage::new(i, v, device, image_format))
                    .collect();
            }
            let swapchain = Swapchain {
                handle,
                device_queues: Arc::clone(device_queues),
                images: images?,
                image_format,
                image_extent: extent,
                image_index: Mutex::new(None),
            };
            Ok(Arc::new(swapchain))
        }
    }

    pub fn acquire_next_image(&self, semaphore: VkSemaphore) -> Result<&SwapchainImage> {
        unsafe {
            let mut image_index = MaybeUninit::<u32>::zeroed();
            vkAcquireNextImageKHR(self.device_queues.device().handle(), self.handle, crate::vk::DEFAULT_TIMEOUT, semaphore, ptr::null_mut(), image_index.as_mut_ptr())
                .into_result()?;
            let image_index = image_index.assume_init() as usize;
            let image = self.images().get(image_index)
                .ok_or_else(|| ErrorCode::SwapchainImageNotFound)?;
            {
                let mut guard = self.image_index.lock().unwrap();
                *guard = Some(image_index);
            }
            Ok(image)
        }
    }

    pub fn current_image(&self) -> Option<&SwapchainImage> {
        let image_index: Option<usize>;
        {
            let guard = self.image_index.lock().unwrap();
            image_index = *guard;
        }
        self.images().get(image_index?)
    }

    pub fn queue_present<'a>(&'a self, image: &'a SwapchainImage, semaphore: VkSemaphore) -> Result<()> {
        let handle = self.handle;
        let image_index = image.index() as u32;
        let present_info = VkPresentInfoKHR {
            sType: VkStructureTypeExt::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
            pNext: ptr::null(),
            waitSemaphoreCount: 1,
            pWaitSemaphores: &semaphore,
            swapchainCount: 1,
            pSwapchains: &handle,
            pImageIndices: &image_index,
            pResults: ptr::null_mut(),
        };
        unsafe {
            let result = vkQueuePresentKHR(self.device_queues.present_queue().handle(), &present_info);
            let is_optimal = result == VkResult::VK_SUCCESS || result == VkResult::VK_SUBOPTIMAL_KHR;
            if is_optimal {
                Ok(())
            } else {
                Err(ErrorCode::VkResult(result).into())
            }
        }
    }

    #[inline]
    pub fn images(&self) -> &Vec<SwapchainImage> {
        &self.images
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        self.device_queues.device()
    }

    #[inline]
    pub fn image_format(&self) -> VkFormat {
        self.image_format
    }

    #[inline]
    pub fn image_extent(&self) -> VkExtent2D {
        self.image_extent
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        log_debug!("Drop Swapchain");
        unsafe {
            let device = self.device_queues.device();
            vkDestroySwapchainKHR(device.handle(), self.handle, std::ptr::null());
        }
    }
}

pub struct SwapchainImage {
    index: usize,
    _handle: VkImage,
    view: VkImageView,
    device: Arc<Device>,
}

impl SwapchainImage {
    unsafe fn new(index: usize, handle: VkImage, device: &Arc<Device>, format: VkFormat) -> Result<Self> {
        let create_info = VkImageViewCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: 0,
            image: handle,
            viewType: VkImageViewType::VK_IMAGE_VIEW_TYPE_2D,
            format: format,
            components: VkComponentMapping::rgba(),
            subresourceRange: VkImageSubresourceRange {
                aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            },
        };
        let mut view_handle = MaybeUninit::<VkImageView>::zeroed();
        vkCreateImageView(device.handle(), &create_info, std::ptr::null(), view_handle.as_mut_ptr())
            .into_result()?;
        let view_handle = view_handle.assume_init();
        let image = SwapchainImage {
            index,
            _handle: handle,
            view: view_handle,
            device: Arc::clone(device),
        };
        Ok(image)
    }

    #[inline]
    pub fn view(&self) -> VkImageView {
        self.view
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }
}

impl Drop for SwapchainImage {
    fn drop(&mut self) {
        log_debug!("Drop SwapchainImage");
        unsafe {
            let device = &self.device;
            vkDestroyImageView(device.handle(), self.view, std::ptr::null());
        }
    }
}

pub struct SceneRenderPass {
    device: Arc<Device>,
    handle: VkRenderPass,
}

impl SceneRenderPass {
    unsafe fn new(device: &Arc<Device>, color_format: VkFormat, depth_format: VkFormat) -> Result<Arc<Self>> {
        let attachments = vec![
            VkAttachmentDescription {
                flags: 0,
                format: color_format,
                samples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                loadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR,
                storeOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE,
                stencilLoadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                stencilStoreOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE,
                initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
                finalLayout: VkImageLayout::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
            },
            VkAttachmentDescription {
                flags: 0,
                format: depth_format,
                samples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                loadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR,
                storeOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE,
                stencilLoadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                stencilStoreOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE,
                initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
                finalLayout: VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            },
        ];
        let color_reference = VkAttachmentReference {
            attachment: 0,
            layout: VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        };
        let depth_reference = VkAttachmentReference {
            attachment: 1,
            layout: VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };
        let subpass_desc = VkSubpassDescription {
            flags: 0,
            pipelineBindPoint: VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
            inputAttachmentCount: 0,
            pInputAttachments: ptr::null(),
            colorAttachmentCount: 1,
            pColorAttachments: &color_reference,
            pResolveAttachments: ptr::null(),
            pDepthStencilAttachment: &depth_reference,
            preserveAttachmentCount: 0,
            pPreserveAttachments: ptr::null(),
        };
        let dependencies = vec![
            VkSubpassDependency {
                srcSubpass: VK_SUBPASS_EXTERNAL,
                dstSubpass: 0,
                srcStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT as VkPipelineStageFlags,
                dstStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as VkPipelineStageFlags,
                srcAccessMask: VkAccessFlagBits::VK_ACCESS_MEMORY_READ_BIT as VkAccessFlags,
                dstAccessMask: VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT as VkAccessFlags
                    | VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as VkAccessFlags,
                dependencyFlags: VkDependencyFlagBits::VK_DEPENDENCY_BY_REGION_BIT as VkDependencyFlags,
            },
            VkSubpassDependency {
                srcSubpass: 0,
                dstSubpass: VK_SUBPASS_EXTERNAL,
                srcStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as VkPipelineStageFlags,
                dstStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT as VkPipelineStageFlags,
                srcAccessMask: VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT as VkAccessFlags
                    | VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as VkAccessFlags,
                dstAccessMask: VkAccessFlagBits::VK_ACCESS_MEMORY_READ_BIT as VkAccessFlags,
                dependencyFlags: VkDependencyFlagBits::VK_DEPENDENCY_BY_REGION_BIT as VkDependencyFlags,
            },
        ];
        let create_info = VkRenderPassCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            subpassCount: 1,
            pSubpasses: &subpass_desc,
            dependencyCount: dependencies.len() as u32,
            pDependencies: dependencies.as_ptr(),
        };
        let mut handle = MaybeUninit::<VkRenderPass>::zeroed();
        vkCreateRenderPass(device.handle(), &create_info, ptr::null(), handle.as_mut_ptr())
            .into_result()
            .unwrap();
        let handle = handle.assume_init();
        let render_pass = SceneRenderPass {
            device: Arc::clone(device),
            handle,
        };
        Ok(Arc::new(render_pass))
    }

    #[inline]
    pub fn handle(&self) -> VkRenderPass {
        self.handle
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl Drop for SceneRenderPass {
    fn drop(&mut self) {
        log_debug!("Drop SceneRenderPass");
        unsafe {
            let device = &self.device;
            vkDestroyRenderPass(device.handle(), self.handle, std::ptr::null());
        }
    }
}

#[allow(dead_code)]
pub struct SwapchainFramebuffers {
    swapchain: Arc<Swapchain>,
    depth_image: Arc<DepthImage>,
    render_pass: Arc<SceneRenderPass>,
    framebuffers: Vec<SwapchainFramebuffer>,
}

impl SwapchainFramebuffers {
    pub fn new(swapchain: &Arc<Swapchain>) -> Result<Arc<Self>> {
        unsafe { Self::init(swapchain) }
    }

    unsafe fn init(swapchain: &Arc<Swapchain>) -> Result<Arc<Self>> {
        let device = swapchain.device();
        let extent = swapchain.image_extent();
        let width = extent.width;
        let height = extent.height;
        let depth_image = DepthImage::new(device, VkExtent3D { width, height, depth: 1 })?;
        let render_pass = SceneRenderPass::new(device, swapchain.image_format(), depth_image.image_format())?;
        let framebuffers: Result<Vec<SwapchainFramebuffer>>;
        framebuffers = swapchain.images()
            .iter()
            .map(|image| SwapchainFramebuffer::new(device, &render_pass, image, &depth_image, width, height))
            .collect();
        let swapchain_framebuffers = SwapchainFramebuffers {
            swapchain: Arc::clone(swapchain),
            depth_image,
            render_pass,
            framebuffers: framebuffers?,
        };
        Ok(Arc::new(swapchain_framebuffers))
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        self.swapchain.device()
    }

    #[inline]
    pub fn render_pass(&self) -> &Arc<SceneRenderPass> {
        &self.render_pass
    }

    #[inline]
    pub fn framebuffers(&self) -> &Vec<SwapchainFramebuffer> {
        &self.framebuffers
    }

    #[inline]
    pub fn swapchain(&self) -> &Arc<Swapchain> {
        &self.swapchain
    }
}

pub struct SwapchainFramebuffer {
    device: Arc<Device>,
    handle: VkFramebuffer,
}

impl SwapchainFramebuffer {
    unsafe fn new(
        device: &Arc<Device>,
        render_pass: &Arc<SceneRenderPass>,
        swapchain_image: &SwapchainImage, 
        depth_image: &DepthImage,
        width: u32,
        height: u32) -> Result<Self> {
        let attachments = vec![
            swapchain_image.view(),
            depth_image.view(),
        ];
        let create_info = VkFramebufferCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            renderPass: render_pass.handle(),
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            width: width,
            height: height,
            layers: 1,
        };
        let mut handle = MaybeUninit::<VkFramebuffer>::zeroed();
        vkCreateFramebuffer(device.handle(), &create_info, ptr::null(), handle.as_mut_ptr())
            .into_result()
            .unwrap();
        let handle = handle.assume_init();
        let framebuffer = SwapchainFramebuffer {
            device: Arc::clone(device),
            handle,
        };
        Ok(framebuffer)
    }

    #[inline]
    pub fn handle(&self) -> VkFramebuffer {
        self.handle
    }
}

impl Drop for SwapchainFramebuffer {
    fn drop(&mut self) {
        log_debug!("Drop SwapchainFramebuffer");
        unsafe {
            let device = &self.device;
            vkDestroyFramebuffer(device.handle(), self.handle, std::ptr::null());
        }
    }
}
