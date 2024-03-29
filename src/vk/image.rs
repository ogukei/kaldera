

use crate::ffi::vk::*;
use super::QueueSubmit;
use super::error::Result;
use super::device::{Device, CommandPool, CommandBufferBuilder};
use super::memory::{BufferMemory, ImageMemory};

use std::ptr;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;

use VkStructureType::*;
use VkPipelineStageFlagBits::*;
use VkBufferUsageFlagBits::*;
use VkMemoryPropertyFlagBits::*;
use VkAccessFlagBits::*;

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct DepthImage {
    device: Arc<Device>,
    image: VkImage,
    view: VkImageView,
    memory: Arc<ImageMemory>,
    image_format: VkFormat,
}

impl DepthImage {
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
                    aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as VkImageAspectFlags,
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
        let image = DepthImage {
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
            aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as VkImageAspectFlags,
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

impl Drop for DepthImage {
    fn drop(&mut self) {
        log_debug!("Drop DepthImage");
        unsafe {
            let device = &self.device;
            vkDestroyImageView(device.handle(), self.view, std::ptr::null());
            // TODO(?): ImageMemory timing
            vkDestroyImage(device.handle(), self.image, std::ptr::null());
        }
    }
}

pub struct TextureImage {
    device: Arc<Device>,
    image: VkImage,
    view: VkImageView,
    sampler: VkSampler,
    memory: Arc<ImageMemory>,
    image_format: VkFormat,
    mip_levels: u32,
    extent: VkExtent3D,
}

impl TextureImage {
    pub fn new(device: &Arc<Device>, extent: VkExtent3D, format: VkFormat, mipmap: bool) -> Result<Arc<Self>> {
        unsafe {
            if mipmap {
                Self::init_mipmap(device, extent, format)
            } else {
                Self::init(device, extent, format)
            }
        }
    }

    unsafe fn init(device: &Arc<Device>, extent: VkExtent3D, format: VkFormat) -> Result<Arc<Self>> {
        let mip_levels = 1;
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
                usage: VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_SRC_BIT as VkImageUsageFlags
                    | VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_DST_BIT as VkImageUsageFlags
                    | VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT as VkImageUsageFlags,
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
            let subresource_range = VkImageSubresourceRange {
                aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            };
            let create_info = VkImageViewCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                image: image_handle,
                viewType: VkImageViewType::VK_IMAGE_VIEW_TYPE_2D,
                format: format,
                components: VkComponentMapping::default(),
                subresourceRange: subresource_range,
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
                addressModeU: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT,
                addressModeV: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT,
                addressModeW: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT,
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
        let image = Self {
            device: Arc::clone(device),
            image: image_handle,
            view: view_handle,
            sampler: sampler_handle,
            memory: image_memory,
            image_format: format,
            mip_levels,
            extent,
        };
        Ok(Arc::new(image))
    }

    unsafe fn init_mipmap(device: &Arc<Device>, extent: VkExtent3D, format: VkFormat) -> Result<Arc<Self>> {
        let mip_levels = extent.width
            .max(extent.height)
            .next_power_of_two()
            .trailing_zeros()
            .max(1);
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
                mipLevels: mip_levels,
                arrayLayers: 1,
                samples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                tiling: VkImageTiling::VK_IMAGE_TILING_OPTIMAL,
                usage: VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_SRC_BIT as VkImageUsageFlags
                    | VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_DST_BIT as VkImageUsageFlags
                    | VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT as VkImageUsageFlags,
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
            let subresource_range = VkImageSubresourceRange {
                aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                baseMipLevel: 0,
                levelCount: mip_levels,
                baseArrayLayer: 0,
                layerCount: 1,
            };
            let create_info = VkImageViewCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                image: image_handle,
                viewType: VkImageViewType::VK_IMAGE_VIEW_TYPE_2D,
                format: format,
                components: VkComponentMapping::default(),
                subresourceRange: subresource_range,
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
                addressModeU: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT,
                addressModeV: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT,
                addressModeW: VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT,
                mipLodBias: 0.0,
                anisotropyEnable: VK_FALSE,
                maxAnisotropy: 1.0,
                compareEnable: VK_FALSE,
                compareOp: VkCompareOp::VK_COMPARE_OP_NEVER,
                minLod: 0.0,
                maxLod: mip_levels as c_float,
                borderColor: VkBorderColor::VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
                unnormalizedCoordinates: VK_FALSE,
            };
            vkCreateSampler(device.handle(), &create_info, ptr::null(), sampler_handle.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let sampler_handle = sampler_handle.assume_init();
        let image = Self {
            device: Arc::clone(device),
            image: image_handle,
            view: view_handle,
            sampler: sampler_handle,
            memory: image_memory,
            image_format: format,
            mip_levels,
            extent,
        };
        Ok(Arc::new(image))
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
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

    #[inline]
    pub fn mip_levels(&self) -> u32 {
        self.mip_levels
    }

    #[inline]
    pub fn memory(&self) -> &Arc<ImageMemory> {
        &self.memory
    }

    #[inline]
    pub fn image(&self) -> VkImage {
        self.image
    }

    #[inline]
    pub fn extent(&self) -> VkExtent3D {
        self.extent
    }

    unsafe fn command_transfer_to_device(&self, 
        command_buffer: VkCommandBuffer,
        buffer_memory: &Arc<BufferMemory>,
    ) {
        if self.mip_levels == 1 {
            self.command_blit(command_buffer, buffer_memory)
        } else {
            self.command_mipmap(command_buffer, buffer_memory)
        }
    }

    unsafe fn command_blit(&self, 
        command_buffer: VkCommandBuffer,
        buffer_memory: &Arc<BufferMemory>,
    ) {
        let image = self.image();
        let subresource_range = VkImageSubresourceRange {
            aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
            baseMipLevel: 0,
            levelCount: 1,
            baseArrayLayer: 0,
            layerCount: 1,
        };
        // barrier
        let image_memory_barrier = VkImageMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: 0 as VkAccessFlags,
            dstAccessMask: VK_ACCESS_TRANSFER_WRITE_BIT as VkAccessFlags,
            oldLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
            newLayout: VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            image: image,
            subresourceRange: subresource_range.clone(),
        };
        vkCmdPipelineBarrier(command_buffer,
            VK_PIPELINE_STAGE_HOST_BIT as VkPipelineStageFlags, 
            VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags, 
            0 as VkDependencyFlags, 
            0, ptr::null(), 
            0, ptr::null(), 
            1, &image_memory_barrier);
        // copy
        let subresource_layers = VkImageSubresourceLayers {
            aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
            mipLevel: 0,
            baseArrayLayer: 0,
            layerCount: 1,
        };
        let region = VkBufferImageCopy {
            bufferOffset: 0,
            bufferRowLength: 0,
            bufferImageHeight: 0,
            imageSubresource: subresource_layers,
            imageOffset: VkOffset3D { x: 0, y: 0, z: 0 },
            imageExtent: self.extent(),
        };
        vkCmdCopyBufferToImage(command_buffer, 
            buffer_memory.buffer(), 
            image, 
            VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 
            1, 
            &region);
        // barrier
        let image_memory_barrier = VkImageMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: VK_ACCESS_TRANSFER_WRITE_BIT as VkAccessFlags,
            dstAccessMask: VK_ACCESS_SHADER_READ_BIT as VkAccessFlags,
            oldLayout: VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            newLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
            srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
            image: image,
            subresourceRange: subresource_range,
        };
        vkCmdPipelineBarrier(command_buffer,
            VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags, 
            VK_PIPELINE_STAGE_ALL_COMMANDS_BIT as VkPipelineStageFlags, 
            0 as VkDependencyFlags, 
            0, ptr::null(), 
            0, ptr::null(), 
            1, &image_memory_barrier);
    }

    unsafe fn command_mipmap(&self, 
        command_buffer: VkCommandBuffer,
        buffer_memory: &Arc<BufferMemory>,
    ) {
        let image = self.image();
        // top image
        {
            let subresource_range = VkImageSubresourceRange {
                aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            };
            // barrier
            let image_memory_barrier = VkImageMemoryBarrier {
                sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                pNext: ptr::null(),
                srcAccessMask: 0 as VkAccessFlags,
                dstAccessMask: VK_ACCESS_TRANSFER_WRITE_BIT as VkAccessFlags,
                oldLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
                newLayout: VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                image: image,
                subresourceRange: subresource_range.clone(),
            };
            vkCmdPipelineBarrier(command_buffer,
                VK_PIPELINE_STAGE_HOST_BIT as VkPipelineStageFlags, 
                VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags, 
                0 as VkDependencyFlags, 
                0, ptr::null(), 
                0, ptr::null(), 
                1, &image_memory_barrier);
            // copy
            let subresource_layers = VkImageSubresourceLayers {
                aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                mipLevel: 0,
                baseArrayLayer: 0,
                layerCount: 1,
            };
            let region = VkBufferImageCopy {
                bufferOffset: 0,
                bufferRowLength: 0,
                bufferImageHeight: 0,
                imageSubresource: subresource_layers,
                imageOffset: VkOffset3D { x: 0, y: 0, z: 0 },
                imageExtent: self.extent(),
            };
            vkCmdCopyBufferToImage(command_buffer, 
                buffer_memory.buffer(), 
                image, 
                VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 
                1, 
                &region);
            // barrier to make it readable
            let image_memory_barrier = VkImageMemoryBarrier {
                sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                pNext: ptr::null(),
                srcAccessMask: VK_ACCESS_TRANSFER_WRITE_BIT as VkAccessFlags,
                dstAccessMask: VK_ACCESS_TRANSFER_READ_BIT as VkAccessFlags,
                oldLayout: VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                newLayout: VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                image: image,
                subresourceRange: subresource_range,
            };
            vkCmdPipelineBarrier(command_buffer,
                VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags, 
                VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags, 
                0 as VkDependencyFlags, 
                0, ptr::null(), 
                0, ptr::null(), 
                1, &image_memory_barrier);
        }
        // remaining image
        {
            let subresource_range = VkImageSubresourceRange {
                aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                baseMipLevel: 1,
                levelCount: VK_REMAINING_MIP_LEVELS,
                baseArrayLayer: 0,
                layerCount: 1,
            };
            // barrier to make them writable
            let image_memory_barrier = VkImageMemoryBarrier {
                sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                pNext: ptr::null(),
                srcAccessMask: 0 as VkAccessFlags,
                dstAccessMask: VK_ACCESS_TRANSFER_WRITE_BIT as VkAccessFlags,
                oldLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
                newLayout: VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                image: image,
                subresourceRange: subresource_range,
            };
            vkCmdPipelineBarrier(command_buffer,
                VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags, 
                VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags, 
                0 as VkDependencyFlags, 
                0, ptr::null(), 
                0, ptr::null(), 
                1, &image_memory_barrier);
        }
        // transfer
        {
            let extent = self.extent();
            let mips = (0u32..self.mip_levels - 1)
                .map(|i| (
                    i, 
                    (extent.width >> i).max(1) as i32, 
                    (extent.height >> i).max(1) as i32, 
                    i + 1,
                    (extent.width >> (i + 1)).max(1) as i32,
                    (extent.height >> (i + 1)).max(1) as i32));
            for (i0_mip, w0, h0, i1_mip, w1, h1) in mips {
                let blit = VkImageBlit {
                    srcSubresource: VkImageSubresourceLayers {
                        aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                        mipLevel: i0_mip,
                        baseArrayLayer: 0,
                        layerCount: 1,
                    },
                    srcOffsets: [
                        VkOffset3D { x: 0, y: 0, z: 0 },
                        VkOffset3D { x: w0, y: h0, z: 1 },
                    ],
                    dstSubresource: VkImageSubresourceLayers {
                        aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                        mipLevel: i1_mip,
                        baseArrayLayer: 0,
                        layerCount: 1,
                    },
                    dstOffsets: [
                        VkOffset3D { x: 0, y: 0, z: 0 },
                        VkOffset3D { x: w1, y: h1, z: 1 },
                    ],
                };
                vkCmdBlitImage(command_buffer, 
                    image, 
                    VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL, 
                    image, 
                    VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 
                    1,
                    &blit, 
                    VkFilter::VK_FILTER_LINEAR);
                // barrier into TRANSFER_SRC_OPTIMAL
                let subresource_range = VkImageSubresourceRange {
                    aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                    baseMipLevel: i1_mip,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                };
                let image_memory_barrier = VkImageMemoryBarrier {
                    sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                    pNext: ptr::null(),
                    srcAccessMask: VK_ACCESS_TRANSFER_WRITE_BIT as VkAccessFlags,
                    dstAccessMask: VK_ACCESS_TRANSFER_READ_BIT as VkAccessFlags,
                    oldLayout: VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                    newLayout: VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                    srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                    dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                    image: image,
                    subresourceRange: subresource_range,
                };
                vkCmdPipelineBarrier(command_buffer,
                    VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags, 
                    VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags, 
                    0 as VkDependencyFlags, 
                    0, ptr::null(), 
                    0, ptr::null(), 
                    1, &image_memory_barrier);
            }
        }
        // barrier all mip level images from TRANSFER_SRC_OPTIMAL into SHADER_READ_ONLY_OPTIMAL
        {
            let subresource_range = VkImageSubresourceRange {
                aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as VkImageAspectFlags,
                baseMipLevel: 0,
                levelCount: VK_REMAINING_MIP_LEVELS,
                baseArrayLayer: 0,
                layerCount: 1,
            };
            let image_memory_barrier = VkImageMemoryBarrier {
                sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                pNext: ptr::null(),
                srcAccessMask: VK_ACCESS_TRANSFER_READ_BIT as VkAccessFlags,
                dstAccessMask: VK_ACCESS_SHADER_READ_BIT as VkAccessFlags,
                oldLayout: VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                newLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
                srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                image: image,
                subresourceRange: subresource_range,
            };
            vkCmdPipelineBarrier(command_buffer,
                VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags, 
                VK_PIPELINE_STAGE_ALL_COMMANDS_BIT as VkPipelineStageFlags, 
                0 as VkDependencyFlags, 
                0, ptr::null(), 
                0, ptr::null(), 
                1, &image_memory_barrier);
        }
    }
}

impl Drop for TextureImage {
    fn drop(&mut self) {
        log_debug!("Drop TextureImage");
        unsafe {
            let device = &self.device;
            vkDestroySampler(device.handle(), self.sampler, std::ptr::null());
            vkDestroyImageView(device.handle(), self.view, std::ptr::null());
            vkDestroyImage(device.handle(), self.image, std::ptr::null());
        }
    }
}

#[allow(dead_code)]
pub struct Texture {
    device: Arc<Device>,
    buffer_memory: Arc<BufferMemory>,
    texture_image: Arc<TextureImage>,
}

impl Texture {
    pub fn new(
        command_pool: &Arc<CommandPool>,
        queue_submit: &Arc<QueueSubmit>,
        texture_image: &Arc<TextureImage>, 
        data: *const c_void,
        data_size: usize,
    ) -> Result<Arc<Self>> {
        unsafe {
            let device = texture_image.device();
            let buffer_memory = BufferMemory::new(device, 
                VK_BUFFER_USAGE_TRANSFER_SRC_BIT as VkFlags, 
                VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as VkFlags 
                    | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as VkFlags, 
                data_size as VkDeviceSize).unwrap();
            {
                let mut mapped = MaybeUninit::<*mut c_void>::zeroed();
                vkMapMemory(device.handle(), buffer_memory.memory(), 0, data_size as VkDeviceSize, 0, mapped.as_mut_ptr())
                    .into_result()
                    .unwrap();
                let mapped = mapped.assume_init();
                std::ptr::copy_nonoverlapping(data, mapped, data_size);
                vkUnmapMemory(device.handle(), buffer_memory.memory());
            }
            let command_buffer = CommandBufferBuilder::new(command_pool).build(|command_buffer| {
                texture_image.command_transfer_to_device(command_buffer, &buffer_memory);
            });
            queue_submit.defer_submit(&command_buffer, VK_PIPELINE_STAGE_ALL_COMMANDS_BIT as VkPipelineStageFlags);
            let image_buffer_memory = Self {
                device: Arc::clone(device),
                texture_image: Arc::clone(texture_image),
                buffer_memory,
            };
            Ok(Arc::new(image_buffer_memory))
        }
    }

    pub(crate) fn descriptor(&self) -> VkDescriptorImageInfo {
        VkDescriptorImageInfo {
            imageLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
            imageView: self.texture_image.view(),
            sampler: self.texture_image.sampler(),
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        log_debug!("Drop Texture");
    }
}
