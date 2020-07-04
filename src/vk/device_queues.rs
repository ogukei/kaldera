

use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, Queue};
use super::surface::{Surface};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void, c_char};
use std::sync::Arc;
use std::ffi::CString;

enum DeviceQueuesCapabilities {
    Default,
    RayTracing,
}

impl DeviceQueuesCapabilities {
    fn extension_names(&self) -> Vec<CString> {
        match &self {
            &Self::Default => vec![
                CString::new("VK_KHR_swapchain").unwrap(),
            ],
            &Self::RayTracing => vec![
                CString::new("VK_KHR_swapchain").unwrap(),
                CString::new("VK_KHR_ray_tracing").unwrap(),
                CString::new("VK_KHR_dedicated_allocation").unwrap(),
                CString::new("VK_KHR_get_memory_requirements2").unwrap(),
                CString::new("VK_KHR_buffer_device_address").unwrap(),
                CString::new("VK_KHR_deferred_host_operations").unwrap(),
                CString::new("VK_EXT_descriptor_indexing").unwrap(),
                CString::new("VK_KHR_pipeline_library").unwrap(),
            ],
        }
    }
}

pub struct DeviceQueuesBuilder {
    surface: Arc<Surface>,
    capabilities: DeviceQueuesCapabilities,
}

impl DeviceQueuesBuilder {
    pub fn new(surface: &Arc<Surface>) -> Self {
        DeviceQueuesBuilder { 
            surface: Arc::clone(surface),
            capabilities: DeviceQueuesCapabilities::Default,
        }
    }

    pub fn with_raytracing(self) -> Self {
        DeviceQueuesBuilder { 
            surface: self.surface,
            capabilities: DeviceQueuesCapabilities::RayTracing,
        }
    }

    pub fn build(self) -> Result<Arc<DeviceQueues>> {
        let surface = &self.surface;
        let instance = surface.instance();
        let physical_devices = PhysicalDevicesBuilder::new(instance).build()?;
        let physical_device = physical_devices.into_iter()
            .nth(0)
            .ok_or_else(|| ErrorCode::SuitablePhysicalDeviceNotFound)?;
        // choose families
        let families = physical_device.queue_families()?;
        let graphics_family = families.iter()
            .find(|v| v.is_graphics());
        let present_family = families.iter()
            .find(|v| surface.is_supported(v, &physical_device).unwrap_or(false));
        let (graphics_family, present_family) = graphics_family
            .and_then(|a| present_family.map(|b| (a, b)))
            .ok_or_else(|| ErrorCode::SuitableQueueFamilyNotFound)?;
        // create infos
        let extension_names = self.capabilities.extension_names();
        let extension_name_ptrs: Vec<*const c_char> = extension_names.iter()
            .map(|v| v.as_ptr())
            .collect();
        let features = physical_device.features();
        let priority: c_float = 0.0;
        if (graphics_family as *const QueueFamily) == (present_family as *const QueueFamily) {
            // unique family
            log_debug!("Unique family");
            let family = graphics_family;
            let family_index = family.index();
            let queue_create_info = VkDeviceQueueCreateInfo::new(family_index, 1, &priority);
            let device_create_info = unsafe { VkDeviceCreateInfo::new(1, &queue_create_info, &extension_name_ptrs, features.features()) };
            unsafe {
                let mut handle = MaybeUninit::<VkDevice>::zeroed();
                vkCreateDevice(physical_device.handle(), &device_create_info, std::ptr::null(), handle.as_mut_ptr())
                    .into_result()?;
                let handle = handle.assume_init();
                let device = Device::new(handle, &physical_device);
                // queue
                let mut queue = MaybeUninit::<VkQueue>::zeroed();
                vkGetDeviceQueue(device.handle(), family_index, 0, queue.as_mut_ptr());
                let queue = queue.assume_init();
                let queue = Queue::new(queue, family.clone(), &device);
                // device queues
                let device_queues = DeviceQueues::new(instance, device, &queue, &queue);
                Ok(device_queues)
            }
        } else {
            let queue_create_infos = vec![
                VkDeviceQueueCreateInfo::new(graphics_family.index(), 1, &priority),
                VkDeviceQueueCreateInfo::new(present_family.index(), 1, &priority),
            ];
            let device_create_info = unsafe { 
                VkDeviceCreateInfo::new(
                    queue_create_infos.len() as u32, 
                    queue_create_infos.as_ptr(),
                    &extension_name_ptrs,
                    features.features())
            };
            unsafe {
                let mut handle = MaybeUninit::<VkDevice>::zeroed();
                vkCreateDevice(physical_device.handle(), &device_create_info, std::ptr::null(), handle.as_mut_ptr())
                    .into_result()?;
                let handle = handle.assume_init();
                let device = Device::new(handle, &physical_device);
                // graphics queue
                let mut graphics_queue = MaybeUninit::<VkQueue>::zeroed();
                vkGetDeviceQueue(device.handle(), graphics_family.index(), 0, graphics_queue.as_mut_ptr());
                let graphics_queue = graphics_queue.assume_init();
                let graphics_queue = Queue::new(graphics_queue, graphics_family.clone(), &device);
                // present queue
                let mut present_queue = MaybeUninit::<VkQueue>::zeroed();
                vkGetDeviceQueue(device.handle(), present_family.index(), 1, present_queue.as_mut_ptr());
                let present_queue = present_queue.assume_init();
                let present_queue = Queue::new(present_queue, present_family.clone(), &device);
                // device queues
                let device_queues = DeviceQueues::new(instance, device, &graphics_queue, &present_queue);
                Ok(device_queues)
            }
        }
    }
}

pub struct DeviceQueues {
    instance: Arc<Instance>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    present_queue: Arc<Queue>,
}

impl DeviceQueues {
    fn new(instance: &Arc<Instance>, device: Arc<Device>, graphics_queue: &Arc<Queue>, present_queue: &Arc<Queue>) -> Arc<Self> {
        let device_queues = DeviceQueues {
            instance: Arc::clone(instance),
            device,
            graphics_queue: Arc::clone(graphics_queue),
            present_queue: Arc::clone(present_queue),
        };
        Arc::new(device_queues)
    }

    #[inline]
    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    #[inline]
    pub fn graphics_queue(&self) -> &Arc<Queue> {
        &self.graphics_queue
    }

    #[inline]
    pub fn present_queue(&self) -> &Arc<Queue> {
        &self.present_queue
    }

    pub fn is_sharing_exclusive(&self) -> bool {
        std::ptr::eq(
            self.graphics_queue.as_ref() as *const Queue,
            self.present_queue.as_ref() as *const Queue)
    }
}
