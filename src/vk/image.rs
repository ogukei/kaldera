

use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, ShaderModule, ShaderModuleSource};
use super::memory::{StagingBuffer, StagingBufferUsage, ImageMemory};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;
use std::ffi::CString;


use VkStructureType::*;
use VkImageUsageFlagBits::*;
use VkSampleCountFlagBits::*;
use VkPipelineStageFlagBits::*;

pub struct ColorImage {
    device: Arc<Device>,
    image: VkImage,
    view: VkImageView,
    sampler: VkSampler,
    memory: Arc<ImageMemory>,
    image_format: VkFormat,
}

impl ColorImage {
    // NOTE: has VK_IMAGE_USAGE_STORAGE_BIT
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

    pub unsafe fn command_barrier_initial_layout(&self, command_buffer: VkCommandBuffer) {
        let subresource_range = VkImageSubresourceRange {
            aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
            baseMipLevel: 0,
            levelCount: 1,
            baseArrayLayer: 0,
            layerCount: 1,
        };
        let image_memory_barrier = VkImageMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: 0 as VkAccessFlags,
            dstAccessMask: 0 as VkAccessFlags,
            oldLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
            newLayout: VkImageLayout::VK_IMAGE_LAYOUT_GENERAL,
            srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            image: self.image,
            subresourceRange: subresource_range,
        };
        vkCmdPipelineBarrier(command_buffer,
            VK_PIPELINE_STAGE_ALL_COMMANDS_BIT as VkPipelineStageFlags, 
            VK_PIPELINE_STAGE_ALL_COMMANDS_BIT as VkPipelineStageFlags, 
            0 as VkDependencyFlags, 
            0, ptr::null(), 
            0, ptr::null(), 
            1, &image_memory_barrier);
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

pub struct DepthStencilImage {
    device: Arc<Device>,
    image: VkImage,
    view: VkImageView,
    memory: Arc<ImageMemory>,
    image_format: VkFormat,
}

impl DepthStencilImage {
    pub unsafe fn new(device: &Arc<Device>, extent: VkExtent3D) -> Result<Arc<Self>> {
        let format = VkFormat::VK_FORMAT_D32_SFLOAT;
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
                usage: VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT as VkImageUsageFlags,
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
                    aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as VkImageAspectFlags 
                        | VkImageAspectFlagBits::VK_IMAGE_ASPECT_STENCIL_BIT as VkImageAspectFlags,
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
        let image = DepthStencilImage {
            device: Arc::clone(device),
            image: image_handle,
            view: view_handle,
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

    pub unsafe fn command_barrier_initial_layout(&self, command_buffer: VkCommandBuffer) {
        let subresource_range = VkImageSubresourceRange {
            aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as VkImageAspectFlags 
                | VkImageAspectFlagBits::VK_IMAGE_ASPECT_STENCIL_BIT as VkImageAspectFlags,
            baseMipLevel: 0,
            levelCount: 1,
            baseArrayLayer: 0,
            layerCount: 1,
        };
        let image_memory_barrier = VkImageMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: 0 as VkAccessFlags,
            dstAccessMask: 0 as VkAccessFlags,
            oldLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
            newLayout: VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL_KHR,
            srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            image: self.image,
            subresourceRange: subresource_range,
        };
        vkCmdPipelineBarrier(command_buffer,
            VK_PIPELINE_STAGE_ALL_COMMANDS_BIT as VkPipelineStageFlags, 
            VK_PIPELINE_STAGE_ALL_COMMANDS_BIT as VkPipelineStageFlags, 
            0 as VkDependencyFlags, 
            0, ptr::null(), 
            0, ptr::null(), 
            1, &image_memory_barrier);
    }
}

impl Drop for DepthStencilImage {
    fn drop(&mut self) {
        log_debug!("Drop DepthStencilImage");
        unsafe {
            let device = &self.device;
            vkDestroyImageView(device.handle(), self.view, std::ptr::null());
            // TODO(?): ImageMemory timing
            vkDestroyImage(device.handle(), self.image, std::ptr::null());
        }
    }
}
