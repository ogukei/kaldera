

use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, Queue};
use super::surface::{Surface};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;

pub struct DeviceQueuesBuilder {
    surface: Arc<Surface>,
}

impl DeviceQueuesBuilder {
    pub fn new(surface: &Arc<Surface>) -> Self {
        DeviceQueuesBuilder { 
            surface: Arc::clone(surface),
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
        let priority: c_float = 0.0;
        if (graphics_family as *const QueueFamily) == (present_family as *const QueueFamily) {
            // unique family
            let family = graphics_family;
            let family_index = family.index();
            let queue_create_info = VkDeviceQueueCreateInfo::new(family_index, 1, &priority);
            let device_create_info = VkDeviceCreateInfo::new(1, &queue_create_info);
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
                let device_queues = DeviceQueues::new(surface, device, &queue, &queue);
                Ok(device_queues)
            }
        } else {
            let queue_create_infos = vec![
                VkDeviceQueueCreateInfo::new(graphics_family.index(), 1, &priority),
                VkDeviceQueueCreateInfo::new(present_family.index(), 1, &priority),
            ];
            let device_create_info = VkDeviceCreateInfo::new(queue_create_infos.len() as u32, queue_create_infos.as_ptr());
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
                let device_queues = DeviceQueues::new(surface, device, &graphics_queue, &present_queue);
                Ok(device_queues)
            }
        }
    }
}

pub struct DeviceQueues {
    surface: Arc<Surface>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    present_queue: Arc<Queue>,
}

impl DeviceQueues {
    fn new(surface: &Arc<Surface>, device: Arc<Device>, graphics_queue: &Arc<Queue>, present_queue: &Arc<Queue>) -> Arc<Self> {
        let device_queues = DeviceQueues {
            surface: Arc::clone(surface),
            device,
            graphics_queue: Arc::clone(graphics_queue),
            present_queue: Arc::clone(present_queue),
        };
        Arc::new(device_queues)
    }

    #[inline]
    pub fn surface(&self) -> &Arc<Surface> {
        &self.surface
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
