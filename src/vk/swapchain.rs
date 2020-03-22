
use crate::ffi::vk::*;

use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, Queue};
use super::surface::{Surface};
use super::device_queues::{DeviceQueues};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;

pub struct Swapchain {
    handle: VkSwapchainKHR,
    device_queues: Arc<DeviceQueues>,
    images: Vec<Arc<SwapchainImage>>,
}

impl Swapchain {
    pub fn new(device_queues: &Arc<DeviceQueues>, extent: VkExtent2D) -> Result<Arc<Self>> {
        let device = device_queues.device();
        let physical_device = device.physical_device();
        let surface = device_queues.surface();
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
                imageUsage: VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as VkImageUsageFlags,
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
            let images: Result<Vec<Arc<SwapchainImage>>>;
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
                    .map(|v| SwapchainImage::new(v, device, image_format))
                    .collect();
            }
            let swapchain = Swapchain {
                handle,
                device_queues: Arc::clone(device_queues),
                images: images?,
            };
            Ok(Arc::new(swapchain))
        }
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

struct SwapchainImage {
    handle: VkImage,
    view: VkImageView,
    device: Arc<Device>,
}

impl SwapchainImage {
    fn new(handle: VkImage, device: &Arc<Device>, format: VkFormat) -> Result<Arc<Self>> {
        unsafe {
            let create_info = VkImageViewCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                pNext: std::ptr::null(),
                flags: 0,
                image: handle,
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
            let mut view_handle = MaybeUninit::<VkImageView>::zeroed();
            vkCreateImageView(device.handle(), &create_info, std::ptr::null(), view_handle.as_mut_ptr())
                .into_result()?;
            let view_handle = view_handle.assume_init();
            let image = SwapchainImage {
                handle,
                view: view_handle,
                device: Arc::clone(device),
            };
            Ok(Arc::new(image))
        }
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
