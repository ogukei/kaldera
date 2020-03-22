

use crate::ffi::vk::*;
use crate::ffi::xcb::*;

use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;

pub enum Surface {
    Xcb(XcbSurface),
}

impl Surface {
    pub fn handle(&self) -> VkSurfaceKHR {
        match self {
            &Self::Xcb(ref surface) => surface.handle(),
        }
    }

    pub fn instance(&self) -> &Arc<Instance> {
        match self {
            &Self::Xcb(ref surface) => surface.instance(),
        }
    }

    pub fn is_supported(&self, queue_family: &QueueFamily, physical_device: &Arc<PhysicalDevice>) -> Result<bool> {
        unsafe {
            let mut is_supported = MaybeUninit::<VkBool32>::zeroed();
            vkGetPhysicalDeviceSurfaceSupportKHR(physical_device.handle(), queue_family.index() as u32, self.handle(), is_supported.as_mut_ptr())
                .into_result()?;
            let is_supported = is_supported.assume_init();
            Ok(is_supported != VK_FALSE)
        }
    }

    pub fn capabilities(&self, physical_device: &Arc<PhysicalDevice>) -> Result<VkSurfaceCapabilitiesKHR> {
        unsafe {
            let mut capabilities = MaybeUninit::<VkSurfaceCapabilitiesKHR>::zeroed();
            vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device.handle(), self.handle(), capabilities.as_mut_ptr())
                .into_result()
                .unwrap();
            let capabilities = capabilities.assume_init();
            if capabilities.maxImageCount == 0 {
                return Err(ErrorCode::VkResult(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT).into())
            }
            Ok(capabilities)
        }
    }

    pub fn formats(&self, physical_device: &Arc<PhysicalDevice>) -> Result<Vec<VkSurfaceFormatKHR>> {
        unsafe {
            let mut uninit_count = MaybeUninit::<u32>::zeroed();
            vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device.handle(), self.handle(), uninit_count.as_mut_ptr(), std::ptr::null_mut())
                .into_result()?;
            let count = uninit_count.assume_init() as usize;
            if count == 0 {
                return Err(ErrorCode::VkResult(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT).into())
            }
            let mut formats: Vec<VkSurfaceFormatKHR> = Vec::with_capacity(count);
            formats.resize(count, std::mem::zeroed());
            vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device.handle(), self.handle(), uninit_count.as_mut_ptr(), formats.as_mut_ptr())
                .into_result()?;
            Ok(formats)
        }
    }

    pub fn presentation_modes(&self, physical_device: &Arc<PhysicalDevice>) -> Result<Vec<VkPresentModeKHR>> {
        unsafe {
            let mut uninit_count = MaybeUninit::<u32>::zeroed();
            vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device.handle(), self.handle(), uninit_count.as_mut_ptr(), std::ptr::null_mut())
                .into_result()?;
            let count = uninit_count.assume_init() as usize;
            if count == 0 {
                return Err(ErrorCode::VkResult(VkResult::VK_ERROR_EXTENSION_NOT_PRESENT).into())
            }
            let mut modes: Vec<VkPresentModeKHR> = Vec::with_capacity(count);
            modes.resize(count, std::mem::zeroed());
            vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device.handle(), self.handle(), uninit_count.as_mut_ptr(), modes.as_mut_ptr())
                .into_result()?;
            Ok(modes)
        }
    }
}

pub struct XcbSurface {
    instance: Arc<Instance>,
    window: Arc<XcbWindow>,
    handle: VkSurfaceKHR,
}

impl XcbSurface {
    pub fn new(instance: &Arc<Instance>, window: &Arc<XcbWindow>) -> Result<Arc<Surface>> {
        unsafe {
            let create_info = VkXcbSurfaceCreateInfoKHR::new(window.connection().handle(), window.handle());
            let mut handle = MaybeUninit::<VkSurfaceKHR>::zeroed();
            vkCreateXcbSurfaceKHR(instance.handle(), &create_info, std::ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            let surface = XcbSurface {
                instance: Arc::clone(instance),
                window: Arc::clone(window),
                handle,
            };
            Ok(Arc::new(Surface::Xcb(surface)))
        }
    }

    pub fn handle(&self) -> VkSurfaceKHR {
        self.handle
    }

    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }
}

impl Drop for XcbSurface {
    fn drop(&mut self) {
        log_debug!("Drop XcbSurface");
        unsafe {
            let instance = &self.instance;
            vkDestroySurfaceKHR(instance.handle(), self.handle, std::ptr::null());
        }
    }
}
