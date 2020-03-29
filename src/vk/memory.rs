
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;

pub struct BufferMemory {
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    device: Arc<Device>,
    whole_size: VkDeviceSize,
}

impl BufferMemory {
    pub fn new(device: &Arc<Device>, 
        usage: VkBufferUsageFlags, 
        memory_property_flags: VkMemoryPropertyFlags, 
        size: VkDeviceSize) -> Result<Arc<Self>> {
        unsafe {
            // creates buffer
            let mut buffer = MaybeUninit::<VkBuffer>::zeroed();
            let buffer_create_info = VkBufferCreateInfo::new(size, usage, VkSharingMode::VK_SHARING_MODE_EXCLUSIVE);
            vkCreateBuffer(device.handle(), &buffer_create_info, ptr::null(), buffer.as_mut_ptr())
                .into_result()
                .unwrap();
            let buffer = buffer.assume_init();
            // physical memory properties
            let mut memory_properties = MaybeUninit::<VkPhysicalDeviceMemoryProperties>::zeroed();
            vkGetPhysicalDeviceMemoryProperties(device.physical_device().handle(), memory_properties.as_mut_ptr());
            let memory_properties = memory_properties.assume_init();
            // requirements
            let mut requirements = MaybeUninit::<VkMemoryRequirements>::zeroed();
            vkGetBufferMemoryRequirements(device.handle(), buffer, requirements.as_mut_ptr());
            let requirements = requirements.assume_init();
            // find a memory type index that fits the properties
            let memory_type_bits = requirements.memoryTypeBits;
            let memory_type_index = memory_properties.memoryTypes.iter()
                .enumerate()
                .filter(|(i,_)| ((memory_type_bits >> i) & 1) == 1)
                .filter(|(_,v)| (v.propertyFlags & memory_property_flags) == memory_property_flags)
                .nth(0)
                .map(|(i,_)| i as u32)
                .ok_or_else(|| ErrorCode::SuitableBufferMemoryTypeNotFound)
                .unwrap();
            // allocation
            let mut memory = MaybeUninit::<VkDeviceMemory>::zeroed();
            let allocate_info = VkMemoryAllocateInfo::new(requirements.size, memory_type_index);
            vkAllocateMemory(device.handle(), &allocate_info, ptr::null(), memory.as_mut_ptr())
                .into_result()
                .unwrap();
            let memory = memory.assume_init();
            // binding
            vkBindBufferMemory(device.handle(), buffer, memory, 0)
                .into_result()
                .unwrap();
            let buffer_memory = BufferMemory { 
                buffer: buffer,
                memory: memory,
                device: Arc::clone(device),
                whole_size: size,
            };
            Ok(Arc::new(buffer_memory))
        }
    }

    #[inline]
    pub fn buffer(&self) -> VkBuffer {
        self.buffer
    }

    #[inline]
    pub fn memory(&self) -> VkDeviceMemory {
        self.memory
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl Drop for BufferMemory {
    fn drop(&mut self) {
        unsafe {
            log_debug!("Drop BufferMemory");
            vkDestroyBuffer(self.device.handle(), self.buffer, ptr::null());
            self.buffer = ptr::null_mut();
            vkFreeMemory(self.device.handle(), self.memory, ptr::null());
            self.memory = ptr::null_mut();
        }
    }
}

pub enum StagingBufferUsage {
    Vertex,
    Index,
}

impl StagingBufferUsage {
    fn host_buffer_usage_flags(&self) -> VkBufferUsageFlags {
        VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as VkBufferUsageFlags
    }

    fn device_buffer_usage_flags(&self) -> VkBufferUsageFlags {
        let usage = match self {
            &Self::Vertex => VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT,
            &Self::Index => VkBufferUsageFlagBits::VK_BUFFER_USAGE_INDEX_BUFFER_BIT,
        };
        VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as VkBufferUsageFlags
            | usage as VkBufferUsageFlags
    }

    fn host_memory_property_flags(&self) -> VkMemoryPropertyFlags {
        VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as VkMemoryPropertyFlags
            | VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as VkMemoryPropertyFlags
    }

    fn device_memory_property_flags(&self) -> VkMemoryPropertyFlags {
        VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags
    }
}

pub struct StagingBuffer {
    command_pool: Arc<CommandPool>,
    host_buffer_memory: Arc<BufferMemory>,
    device_buffer_memory: Arc<BufferMemory>,
    copying_command_buffer: Arc<CommandBuffer>,
}

impl StagingBuffer {
    pub fn new(command_pool: &Arc<CommandPool>, usage: StagingBufferUsage, size: VkDeviceSize) -> Result<Arc<Self>> {
        unsafe {
            let device = command_pool.queue().device();
            let host_buffer_memory = BufferMemory::new(
                device,
                usage.host_buffer_usage_flags(),
                usage.host_memory_property_flags(),
                size)?;
            let device_buffer_memory = BufferMemory::new(
                device,
                usage.device_buffer_usage_flags(),
                usage.device_memory_property_flags(),
                size)?;
            let command_buffer = CommandBufferBuilder::new(command_pool).build(|command_buffer| {
                let copy_region = VkBufferCopy::new(0, size);
                vkCmdCopyBuffer(
                    command_buffer,
                    host_buffer_memory.buffer(),
                    device_buffer_memory.buffer(),
                    1,
                    &copy_region,
                );
            });
            let staging_buffer = StagingBuffer {
                command_pool: Arc::clone(command_pool),
                host_buffer_memory,
                device_buffer_memory,
                copying_command_buffer: command_buffer,
            };
            Ok(Arc::new(staging_buffer))
        }
    }

    pub fn write(&self, src: *const c_void, size: usize) {
        unsafe {
            let device = self.command_pool.queue().device();
            let host_buffer_memory = &self.host_buffer_memory;
            let mut mapped = MaybeUninit::<*mut c_void>::zeroed();
            vkMapMemory(device.handle(), host_buffer_memory.memory(), 0, size as VkDeviceSize, 0, mapped.as_mut_ptr())
                .into_result()
                .unwrap();
            let mapped = mapped.assume_init();
            std::ptr::copy_nonoverlapping(src, mapped, size);
            vkUnmapMemory(device.handle(), host_buffer_memory.memory());
            self.copying_command_buffer.submit_then_wait(
                VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags);
        }
    }

    #[inline]
    pub fn host_buffer_memory(&self) -> &Arc<BufferMemory> {
        &self.host_buffer_memory
    }

    #[inline]
    pub fn device_buffer_memory(&self) -> &Arc<BufferMemory> {
        &self.device_buffer_memory
    }
}

pub struct ImageMemory {
    device: Arc<Device>,
    handle: VkDeviceMemory,
}

impl ImageMemory {
    pub fn new(device: &Arc<Device>, image: VkImage) -> Result<Arc<Self>> {
        unsafe {
            // image memory requirements
            let mut requirements = MaybeUninit::<VkMemoryRequirements>::zeroed();
            vkGetImageMemoryRequirements(device.handle(), image, requirements.as_mut_ptr());
            let requirements = requirements.assume_init();
            // physical memory properties
            let mut memory_properties = MaybeUninit::<VkPhysicalDeviceMemoryProperties>::zeroed();
            vkGetPhysicalDeviceMemoryProperties(device.physical_device().handle(), memory_properties.as_mut_ptr());
            let memory_properties = memory_properties.assume_init();
            // find a memory type index that fits the properties
            let memory_property_flags = VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags;
            let memory_type_bits = requirements.memoryTypeBits;
            let memory_type_index = memory_properties.memoryTypes.iter()
                .enumerate()
                .filter(|(i,_)| ((memory_type_bits >> i) & 1) == 1)
                .filter(|(_,v)| (v.propertyFlags & memory_property_flags) == memory_property_flags)
                .nth(0)
                .map(|(i,_)| i as u32)
                .ok_or_else(|| ErrorCode::SuitableImageMemoryTypeNotFound)
                .unwrap();
            // allocation
            let alloc_info = VkMemoryAllocateInfo::new(requirements.size, memory_type_index);
            let mut memory = MaybeUninit::<VkDeviceMemory>::zeroed();
            vkAllocateMemory(device.handle(), &alloc_info, ptr::null(), memory.as_mut_ptr())
                .into_result()
                .unwrap();
            let memory = memory.assume_init();
            vkBindImageMemory(device.handle(), image, memory, 0)
                .into_result()
                .unwrap();
            let image_memory = ImageMemory {
                device: Arc::clone(device),
                handle: memory,
            };
            Ok(Arc::new(image_memory))
        }
    }

    #[inline]
    pub fn handle(&self) -> VkDeviceMemory {
        self.handle
    }
}

impl Drop for ImageMemory {
    fn drop(&mut self) {
        unsafe {
            log_debug!("Drop ImageMemory");
            vkFreeMemory(self.device.handle(), self.handle, ptr::null());
            self.handle = ptr::null_mut();
        }
    }
}
