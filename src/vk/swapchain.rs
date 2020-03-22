
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

pub struct Swapchain {

}

impl Swapchain {
    pub fn new(device: &Arc<Device>, surface: &Arc<Surface>) -> Result<Arc<Self>> {
        let physical_device = device.physical_device();
        unsafe {
            let mut capabilies = MaybeUninit::<VkSurfaceCapabilitiesKHR>::zeroed();
            vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device.handle(), surface.handle(), capabilies.as_mut_ptr())
                .into_result()
                .unwrap();
            let capabilies = capabilies.assume_init();
            println!("minImageCount {}, minImageExtent {:?}", capabilies.minImageCount, capabilies.minImageExtent);
            Ok(Arc::new(Swapchain {}))
        }
        
    }
}

pub enum Surface {
    Xcb(XcbSurface),
}

impl Surface {
    pub fn handle(&self) -> VkSurfaceKHR {
        match self {
            &Self::Xcb(ref surface) => surface.handle(),
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
}

impl Drop for XcbSurface {
    fn drop(&mut self) {
        unsafe {
            let instance = &self.instance;
            vkDestroySurfaceKHR(instance.handle(), self.handle, std::ptr::null());
        }
    }
}
