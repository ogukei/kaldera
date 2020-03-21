
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;

pub struct Device {
    handle: VkDevice,
    queue: Queue,
    physical_device: Arc<PhysicalDevice>,
}

impl Device {
    #[inline]
    pub fn handle(&self) -> VkDevice {
        self.handle
    }

    #[inline]
    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    #[inline]
    pub fn physical_device(&self) -> &Arc<PhysicalDevice> {
        &self.physical_device
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        log_debug!("Drop Device");
        unsafe {
            vkDestroyDevice(self.handle, ptr::null());
            self.handle = ptr::null_mut();
        }
    }
}

pub struct CommandPool {
    handle: VkCommandPool,
    device: Arc<Device>,
}

impl CommandPool {
    pub fn new(device: &Arc<Device>) -> Result<Arc<Self>> {
        unsafe {
            let mut handle = MaybeUninit::<VkCommandPool>::zeroed();
            let info = VkCommandPoolCreateInfo::new(device.queue().family().index() as u32);
            vkCreateCommandPool(device.handle, &info, ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            let command_pool = CommandPool {
                handle: handle,
                device: Arc::clone(device),
            };
            Ok(Arc::new(command_pool))
        }
    }

    #[inline]
    pub fn handle(&self) -> VkCommandPool {
        self.handle
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        log_debug!("Drop CommandPool");
        unsafe {
            vkDestroyCommandPool(self.device.handle(), self.handle, ptr::null());
            self.handle = ptr::null_mut();
        }
    }
}

pub struct CommandBufferBuilder<'a> {
    command_pool: &'a Arc<CommandPool>,
}

impl<'a> CommandBufferBuilder<'a> {
    pub fn new(command_pool: &'a Arc<CommandPool>) -> Self {
        CommandBufferBuilder {
            command_pool,
        }
    }

    pub fn build<F>(self, op: F) -> Arc<CommandBuffer> where F: FnOnce(VkCommandBuffer) -> () {
        unsafe {
            let command_pool = self.command_pool;
            let device = command_pool.device();
            let mut command_buffer = MaybeUninit::<VkCommandBuffer>::zeroed();
            {
                let alloc_info = VkCommandBufferAllocateInfo::new(command_pool.handle(), VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY, 1);
                vkAllocateCommandBuffers(device.handle(), &alloc_info, command_buffer.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let command_buffer = command_buffer.assume_init();
            let mut fence = MaybeUninit::<VkFence>::zeroed();
            {
                let create_info = VkFenceCreateInfo::new(VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT as VkFlags);
                vkCreateFence(device.handle(), &create_info, ptr::null(), fence.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let fence = fence.assume_init();
            let begin_info = VkCommandBufferBeginInfo::new();
            vkBeginCommandBuffer(command_buffer, &begin_info)
                .into_result()
                .unwrap();
            {
                op(command_buffer);
            }
            vkEndCommandBuffer(command_buffer);
            CommandBuffer::new(command_pool, command_buffer, fence)
        }
    }
}

pub struct CommandBuffer {
    command_pool: Arc<CommandPool>,
    handle: VkCommandBuffer,
    fence: VkFence,
}

impl CommandBuffer {
    fn new(command_pool: &Arc<CommandPool>, handle: VkCommandBuffer, fence: VkFence) -> Arc<CommandBuffer> {
        let command_buffer = CommandBuffer {
            command_pool: Arc::clone(command_pool),
            handle,
            fence,
        };
        Arc::new(command_buffer)
    }

    pub fn submit_then_wait(&self, wait_mask: VkPipelineStageFlags) {
        unsafe {
            let device = self.command_pool.device();
            let fence = self.fence;
            let command_buffer = self.handle;
            vkResetFences(device.handle(), 1, &fence)
                .into_result()
                .unwrap();
            let submit_info = VkSubmitInfo::with_command_buffer_wait(1, &command_buffer, &wait_mask);
            vkQueueSubmit(device.queue().handle(), 1, &submit_info, fence);
            vkWaitForFences(device.handle(), 1, &fence, VK_TRUE, u64::max_value())
                .into_result()
                .unwrap();
        }
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe {
            let command_pool = &self.command_pool;
            let device = command_pool.device();
            vkDestroyFence(device.handle(), self.fence, ptr::null());
            self.fence = ptr::null_mut();
            vkFreeCommandBuffers(device.handle(), command_pool.handle(), 1, &self.handle);
            self.handle = ptr::null_mut();
        }
    }
}

pub struct Queue {
    handle: VkQueue,
    family: QueueFamily,
}

impl Queue {
    fn new(handle: VkQueue, family: QueueFamily) -> Self {
        Queue { handle: handle, family: family }
    }

    #[inline]
    pub fn handle(&self) -> VkQueue {
        self.handle
    }

    #[inline]
    pub fn family(&self) -> &QueueFamily {
        &self.family
    }
}

pub struct DeviceBuilder<'a> {
    instance: &'a Arc<Instance>,
}

impl<'a> DeviceBuilder<'a> {
    pub fn new(instance: &'a Arc<Instance>) -> Self {
        DeviceBuilder { instance }
    }

    pub fn build(self) -> Result<Arc<Device>> {
        let devices = PhysicalDevicesBuilder::new(self.instance).build()?;
        let device = devices.into_iter()
            .nth(0)
            .ok_or_else(|| ErrorCode::SuitablePhysicalDeviceNotFound)?;
        let families = device.queue_families()?;
        // iterate through compute family candidates keeping the indices
        let suitable_families: Vec<_> = families.into_iter()
            .filter(|family| family.is_graphics())
            .collect();
        // request single queue
        let family = suitable_families.into_iter()
            .nth(0)
            .ok_or_else(|| ErrorCode::SuitablePhysicalDeviceNotFound)?;
        let family_index = family.index() as u32;
        let priority: c_float = 0.0;
        let queue_create_info = VkDeviceQueueCreateInfo::new(family_index, 1, &priority);
        let device_create_info = VkDeviceCreateInfo::new(1, &queue_create_info);
        unsafe {
            let mut handle = MaybeUninit::<VkDevice>::zeroed();
            vkCreateDevice(device.handle(), &device_create_info, std::ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            // queues
            let mut queue = MaybeUninit::<VkQueue>::zeroed();
            vkGetDeviceQueue(handle, family_index, 0, queue.as_mut_ptr());
            let queue = Queue::new(queue.assume_init(), family);
            let device = Device {
                handle: handle,
                queue: queue,
                physical_device: device,
            };
            Ok(Arc::new(device))
        }
    }
}

pub enum ShaderModuleSource {
    FilePath(String),
    Bytes(Vec<u8>),
}

impl ShaderModuleSource {
    pub fn from_file(filename: impl Into<String>) -> Self {
        ShaderModuleSource::FilePath(filename.into())        
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        ShaderModuleSource::Bytes(bytes)        
    }

    fn load(self) -> Result<(Vec<u8>, usize)> {
        match self {
            ShaderModuleSource::FilePath(filename) => {
                let mut file = std::fs::File::open(filename)
                    .map_err(|v| ErrorCode::ShaderLoadIO(v))?;
                let mut buffer = Vec::<u8>::new();
                let bytes = file.read_to_end(&mut buffer)
                    .map_err(|v| ErrorCode::ShaderLoadIO(v))?;
                if bytes > 0 && (bytes % 4) == 0 {
                    Ok((buffer, bytes))
                } else {
                    Err(ErrorCode::ShaderLoadUnaligned.into())
                }
            },
            ShaderModuleSource::Bytes(vector) => {
                let bytes = vector.len();
                if bytes > 0 && (bytes % 4) == 0 {
                    Ok((vector, bytes))
                } else {
                    Err(ErrorCode::ShaderLoadUnaligned.into())
                }
            },
        }
    }
}

pub struct ShaderModule {
    handle: VkShaderModule,
    device: Arc<Device>,
}

impl ShaderModule {
    pub fn new(device: &Arc<Device>, source: ShaderModuleSource) -> Result<Arc<Self>> {
        unsafe {
            let (buffer, num_bytes) = source.load()?;
            let mut handle = MaybeUninit::<VkShaderModule>::zeroed();
            let create_info = VkShaderModuleCreateInfo::new(num_bytes, buffer.as_ptr() as *const u32);
            vkCreateShaderModule(device.handle, &create_info, ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            let shader_module = ShaderModule {
                handle: handle,
                device: Arc::clone(device),
            };
            Ok(Arc::new(shader_module))
        }
    }

    #[inline]
    pub fn handle(&self) -> VkShaderModule {
        self.handle
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        log_debug!("Drop ShaderModule");
        unsafe {
            vkDestroyShaderModule(self.device.handle(), self.handle, ptr::null());
            self.handle = ptr::null_mut();
        }
    }
}
