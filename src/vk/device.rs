
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{QueueFamily, PhysicalDevice};

use std::ptr;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::io::Read;

pub struct Device {
    handle: VkDevice,
    physical_device: Arc<PhysicalDevice>,
}

impl Device {
    pub(crate) fn new(handle: VkDevice, physical_device: &Arc<PhysicalDevice>) -> Arc<Device> {
        let device = Device {
            handle,
            physical_device: Arc::clone(physical_device),
        };
        Arc::new(device)
    }

    #[inline]
    pub fn handle(&self) -> VkDevice {
        self.handle
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
    queue: Arc<Queue>,
}

impl CommandPool {
    pub fn new(queue: &Arc<Queue>) -> Result<Arc<Self>> {
        unsafe {
            let device = queue.device();
            let mut handle = MaybeUninit::<VkCommandPool>::zeroed();
            let info = VkCommandPoolCreateInfo::new(queue.family().index() as u32);
            vkCreateCommandPool(device.handle(), &info, ptr::null(), handle.as_mut_ptr())
                .into_result()?;
            let handle = handle.assume_init();
            let command_pool = CommandPool {
                handle: handle,
                queue: Arc::clone(queue),
            };
            Ok(Arc::new(command_pool))
        }
    }

    #[inline]
    pub fn handle(&self) -> VkCommandPool {
        self.handle
    }

    #[inline]
    pub fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        log_debug!("Drop CommandPool");
        unsafe {
            vkDestroyCommandPool(self.queue().device().handle(), self.handle, ptr::null());
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
            let device = command_pool.queue().device();
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
            let queue = self.command_pool.queue();
            let device = queue.device();
            let fence = self.fence;
            let command_buffer = self.handle;
            vkResetFences(device.handle(), 1, &fence)
                .into_result()
                .unwrap();
            let submit_info = VkSubmitInfo::with_command_buffer_wait(1, &command_buffer, &wait_mask);
            vkQueueSubmit(queue.handle(), 1, &submit_info, fence);
            vkWaitForFences(device.handle(), 1, &fence, VK_TRUE, crate::vk::DEFAULT_TIMEOUT)
                .into_result()
                .unwrap();
        }
    }

    pub fn wait_and_reset(&self) {
        unsafe {
            let queue = self.command_pool.queue();
            let device = queue.device();
            let fence = self.fence;
            vkWaitForFences(device.handle(), 1, &fence, VK_TRUE, crate::vk::DEFAULT_TIMEOUT)
                .into_result()
                .unwrap();
            vkResetFences(device.handle(), 1, &fence)
                .into_result()
                .unwrap();
        }
    }

    pub fn submit(&self, wait_mask: VkPipelineStageFlags, wait_semaphore: VkSemaphore, signal_semaphore: VkSemaphore) -> Result<()> {
        unsafe {
            let queue = self.command_pool.queue();
            let fence = self.fence;
            let command_buffer = self.handle;
            let submit_info = VkSubmitInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
                pNext: ptr::null(),
                waitSemaphoreCount: 1,
                pWaitSemaphores: &wait_semaphore,
                pWaitDstStageMask: &wait_mask,
                commandBufferCount: 1,
                pCommandBuffers: &command_buffer,
                signalSemaphoreCount: 1,
                pSignalSemaphores: &signal_semaphore,
            };
            vkQueueSubmit(queue.handle(), 1, &submit_info, fence)
                .into_result()?;
            Ok(())
        }
    }

    #[inline]
    pub fn handle(&self) -> VkCommandBuffer {
        self.handle
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        log_debug!("Drop CommandBuffer");
        unsafe {
            let command_pool = &self.command_pool;
            let device = command_pool.queue().device();
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
    device: Arc<Device>,
}

impl Queue {
    pub fn new(handle: VkQueue, family: QueueFamily, device: &Arc<Device>) -> Arc<Self> {
        let queue = Queue { 
            handle: handle, 
            family: family,
            device: Arc::clone(device),
        };
        Arc::new(queue)
    }

    #[inline]
    pub fn handle(&self) -> VkQueue {
        self.handle
    }

    #[inline]
    pub fn family(&self) -> &QueueFamily {
        &self.family
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub unsafe fn submit_then_wait(&self, command_buffers: &[VkCommandBuffer]) -> Result<()> {
        let submit_info = VkSubmitInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
            pNext: ptr::null(),
            waitSemaphoreCount: 0,
            pWaitSemaphores: ptr::null(),
            pWaitDstStageMask: ptr::null(),
            commandBufferCount: command_buffers.len() as u32,
            pCommandBuffers: command_buffers.as_ptr(),
            signalSemaphoreCount: 0,
            pSignalSemaphores: ptr::null(),
        };
        vkQueueSubmit(self.handle(), 1, &submit_info, ptr::null_mut());
        vkQueueWaitIdle(self.handle())
            .into_result()
    }

    pub fn wait_idle(&self) -> Result<()> {
        unsafe {
            vkQueueWaitIdle(self.handle())
                .into_result()
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

pub struct CommandBufferRecording<'a> {
    command_pool: &'a Arc<CommandPool>,
    command_buffer: Option<VkCommandBuffer>,
}

impl<'a> CommandBufferRecording<'a> {
    pub fn new_onetime_submit(command_pool: &'a Arc<CommandPool>) -> Result<Self> {
        unsafe {
            let device = command_pool.queue().device();
            let mut command_buffer = MaybeUninit::<VkCommandBuffer>::zeroed();
            {
                let alloc_info = VkCommandBufferAllocateInfo::new(command_pool.handle(), VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY, 1);
                vkAllocateCommandBuffers(device.handle(), &alloc_info, command_buffer.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let command_buffer = command_buffer.assume_init();
            let begin_info = VkCommandBufferBeginInfo::new_onetime_submit();
            vkBeginCommandBuffer(command_buffer, &begin_info)
                .into_result()
                .unwrap();
            let recording = Self {
                command_pool,
                command_buffer: Some(command_buffer),
            };
            Ok(recording)
        }
    }

    #[inline]
    pub fn command_buffer(&self) -> VkCommandBuffer {
        self.command_buffer.unwrap()
    }

    #[inline]
    pub fn command_pool(&self) -> &'a Arc<CommandPool> {
        self.command_pool
    }

    pub fn complete(mut self) -> Arc<CommandBuffer> {
        unsafe {
            let command_buffer = self.command_buffer.take().unwrap();
            let command_pool = &self.command_pool;
            let device = command_pool.queue().device();
            let mut fence = MaybeUninit::<VkFence>::zeroed();
            {
                let create_info = VkFenceCreateInfo::new(VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT as VkFlags);
                vkCreateFence(device.handle(), &create_info, ptr::null(), fence.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let fence = fence.assume_init();
            vkEndCommandBuffer(command_buffer);
            CommandBuffer::new(command_pool, command_buffer, fence)
        }
    }
}

impl<'a> Drop for CommandBufferRecording<'a> {
    fn drop(&mut self) {
        assert!(self.command_buffer.is_none(), "incomplete command buffer recording");
    }
}
