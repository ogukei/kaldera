

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
