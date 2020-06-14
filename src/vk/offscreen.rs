
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, ShaderModule, ShaderModuleSource};
use super::memory::{StagingBuffer, StagingBufferUsage, ImageMemory};
use super::swapchain::{DepthStencilImage};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;
use std::ffi::CString;

pub struct ColorImage {
    device: Arc<Device>,
    image: VkImage,
    view: VkImageView,
    sampler: VkSampler,
    memory: Arc<ImageMemory>,
    image_format: VkFormat,
}

impl ColorImage {
    pub unsafe fn new(device: &Arc<Device>, extent: VkExtent3D) -> Result<Arc<Self>> {
        let format = VkFormat::VK_FORMAT_R8G8B8A8_UNORM;
        // image
        let mut image_handle = MaybeUninit::<VkImage>::zeroed();
        {
            let create_info = VkImageCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                imageType: VkImageType::VK_IMAGE_TYPE_2D,
                format: format,
                extent: extent,
                mipLevels: 1,
                arrayLayers: 1,
                samples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                tiling: VkImageTiling::VK_IMAGE_TILING_OPTIMAL,
                usage: VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as VkImageUsageFlags
                    | VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT as VkImageUsageFlags
                    | VkImageUsageFlagBits::VK_IMAGE_USAGE_STORAGE_BIT as VkImageUsageFlags,
                sharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
                queueFamilyIndexCount: 0,
                pQueueFamilyIndices: std::ptr::null(),
                initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
            };
            vkCreateImage(device.handle(), &create_info, std::ptr::null(), image_handle.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let image_handle = image_handle.assume_init();
        // memory
        let image_memory = ImageMemory::new(device, image_handle)?;
        // view
        let mut view_handle = MaybeUninit::<VkImageView>::zeroed();
        {
            let create_info = VkImageViewCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                image: image_handle,
                viewType: VkImageViewType::VK_IMAGE_VIEW_TYPE_2D,
                format: format,
                components: VkComponentMapping::default(),
                subresourceRange: VkImageSubresourceRange {
                    aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                    baseMipLevel: 0,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                },
            };
            vkCreateImageView(device.handle(), &create_info, ptr::null(), view_handle.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let view_handle = view_handle.assume_init();
        // sampler
        let mut sampler_handle = MaybeUninit::<VkSampler>::zeroed();
        {
            let create_info = VkSamplerCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                magFilter: VkFilter::VK_FILTER_LINEAR,
                minFilter: VkFilter::VK_FILTER_LINEAR,
                mipmapMode: VkSamplerMipmapMode::VK_SAMPLER_MIPMAP_MODE_LINEAR,
                addressModeU: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
                addressModeV: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
                addressModeW: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
                mipLodBias: 0.0,
                anisotropyEnable: VK_FALSE,
                maxAnisotropy: 1.0,
                compareEnable: VK_FALSE,
                compareOp: VkCompareOp::VK_COMPARE_OP_NEVER,
                minLod: 0.0,
                maxLod: 1.0,
                borderColor: VkBorderColor::VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
                unnormalizedCoordinates: VK_FALSE,
            };
            vkCreateSampler(device.handle(), &create_info, ptr::null(), sampler_handle.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let sampler_handle = sampler_handle.assume_init();
        let image = ColorImage {
            device: Arc::clone(device),
            image: image_handle,
            view: view_handle,
            sampler: sampler_handle,
            memory: image_memory,
            image_format: format,
        };
        Ok(Arc::new(image))
    }

    #[inline]
    pub fn view(&self) -> VkImageView {
        self.view
    }

    #[inline]
    pub fn image_format(&self) -> VkFormat {
        self.image_format
    }

    #[inline]
    pub fn sampler(&self) -> VkSampler {
        self.sampler
    }
}

impl Drop for ColorImage {
    fn drop(&mut self) {
        log_debug!("Drop ColorImage");
        unsafe {
            let device = &self.device;
            vkDestroySampler(device.handle(), self.sampler, std::ptr::null());
            vkDestroyImageView(device.handle(), self.view, std::ptr::null());
            // TODO(?): ImageMemory timing
            vkDestroyImage(device.handle(), self.image, std::ptr::null());
        }
    }
}

pub struct OffscreenRenderPass {
    device: Arc<Device>,
    handle: VkRenderPass,
}

impl OffscreenRenderPass {
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
                finalLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
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
                srcStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT as VkPipelineStageFlags,
                dstStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as VkPipelineStageFlags,
                srcAccessMask: VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as VkAccessFlags,
                dstAccessMask: VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as VkAccessFlags,
                dependencyFlags: VkDependencyFlagBits::VK_DEPENDENCY_BY_REGION_BIT as VkDependencyFlags,
            },
            VkSubpassDependency {
                srcSubpass: 0,
                dstSubpass: VK_SUBPASS_EXTERNAL,
                srcStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as VkPipelineStageFlags,
                dstStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT as VkPipelineStageFlags,
                srcAccessMask: VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as VkAccessFlags,
                dstAccessMask: VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as VkAccessFlags,
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
        let render_pass = OffscreenRenderPass {
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

impl Drop for OffscreenRenderPass {
    fn drop(&mut self) {
        log_debug!("Drop OffscreenRenderPass");
        unsafe {
            let device = &self.device;
            vkDestroyRenderPass(device.handle(), self.handle, std::ptr::null());
        }
    }
}

pub struct OffscreenFramebuffer {
    handle: VkFramebuffer,
    device: Arc<Device>,
    color_image: Arc<ColorImage>,
    depth_image: Arc<DepthStencilImage>,
    render_pass: Arc<OffscreenRenderPass>,
}

impl OffscreenFramebuffer {
    pub unsafe fn new(device: &Arc<Device>, extent: VkExtent3D, width: u32, height: u32) -> Result<Arc<Self>> {
        let color_image = ColorImage::new(device, extent)?;
        let depth_image = DepthStencilImage::new(device, extent)?;
        let render_pass = OffscreenRenderPass::new(device, color_image.image_format(), depth_image.image_format())?;
        let attachments = vec![
            color_image.view(),
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
        let framebuffer = Self {
            handle,
            device: Arc::clone(device),
            color_image,
            depth_image,
            render_pass,
        };
        Ok(Arc::new(framebuffer))
    }
}

impl Drop for OffscreenFramebuffer {
    fn drop(&mut self) {
        log_debug!("Drop OffscreenFramebuffer");
        unsafe {
            let device = &self.device;
            vkDestroyFramebuffer(device.handle(), self.handle, std::ptr::null());
        }
    }
}
