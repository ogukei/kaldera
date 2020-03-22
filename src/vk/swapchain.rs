
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

}

impl Swapchain {
    pub fn new(device_queues: &Arc<DeviceQueues>, extent: VkExtent2D) -> Result<Arc<Self>> {
        let device = device_queues.device();
        let physical_device = device.physical_device();
        let surface = device_queues.surface();
        unsafe {
            let surface_capabilities = Self::surface_capabilities(physical_device, surface)?;
            let surface_formats = Self::surface_formats(physical_device, surface)?;
            
            let surface_format = surface_formats.iter()
                .find(|v| v.format == VkFormat::VK_FORMAT_B8G8R8A8_SRGB)
                .or_else(|| surface_formats.iter().find(|v| v.format == VkFormat::VK_FORMAT_B8G8R8A8_UNORM))
                .ok_or_else(|| ErrorCode::SwapchainSurfaceFormatNotSupported)?;
            let image_count = surface_capabilities.minImageCount + 1;
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
                imageFormat: surface_format.format,
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
            Ok(Arc::new(Swapchain {}))
        }
    }

    fn surface_capabilities(physical_device: &Arc<PhysicalDevice>, surface: &Arc<Surface>) -> Result<VkSurfaceCapabilitiesKHR> {
        unsafe {
            let mut capabilities = MaybeUninit::<VkSurfaceCapabilitiesKHR>::zeroed();
            vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device.handle(), surface.handle(), capabilities.as_mut_ptr())
                .into_result()
                .unwrap();
            let capabilities = capabilities.assume_init();
            if capabilities.maxImageCount == 0 {
                return Err(ErrorCode::VkResult(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT).into())
            }
            Ok(capabilities)
        }
    }

    fn surface_formats(physical_device: &Arc<PhysicalDevice>, surface: &Arc<Surface>) -> Result<Vec<VkSurfaceFormatKHR>> {
        unsafe {
            let mut uninit_count = MaybeUninit::<u32>::zeroed();
            vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device.handle(), surface.handle(), uninit_count.as_mut_ptr(), std::ptr::null_mut())
                .into_result()?;
            let count = uninit_count.assume_init() as usize;
            if count == 0 {
                return Err(ErrorCode::VkResult(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT).into())
            }
            let mut formats: Vec<VkSurfaceFormatKHR> = Vec::with_capacity(count);
            formats.resize(count, std::mem::zeroed());
            vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device.handle(), surface.handle(), uninit_count.as_mut_ptr(), formats.as_mut_ptr())
                .into_result()?;
            Ok(formats)
        }
    }

    fn surface_presentation_modes(physical_device: &Arc<PhysicalDevice>, surface: &Arc<Surface>) -> Result<Vec<VkPresentModeKHR>> {
        unsafe {
            let mut uninit_count = MaybeUninit::<u32>::zeroed();
            vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device.handle(), surface.handle(), uninit_count.as_mut_ptr(), std::ptr::null_mut())
                .into_result()?;
            let count = uninit_count.assume_init() as usize;
            if count == 0 {
                return Err(ErrorCode::VkResult(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT).into())
            }
            let mut modes: Vec<VkPresentModeKHR> = Vec::with_capacity(count);
            modes.resize(count, std::mem::zeroed());
            vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device.handle(), surface.handle(), uninit_count.as_mut_ptr(), modes.as_mut_ptr())
                .into_result()?;
            Ok(modes)
        }
    }
}
