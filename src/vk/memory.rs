
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder};

use std::ptr;
use std::mem::MaybeUninit;
use libc::{c_void};
use std::sync::Arc;

use VkBufferUsageFlagBits::*;
use VkMemoryPropertyFlagBits::*;
use VkPipelineStageFlagBits::*;

#[allow(dead_code)]
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
            // requirements
            let mut requirements = MaybeUninit::<VkMemoryRequirements>::zeroed();
            vkGetBufferMemoryRequirements(device.handle(), buffer, requirements.as_mut_ptr());
            let requirements = requirements.assume_init();
            let memory_type_index = device.physical_device()
                .memory_type_index(&requirements, memory_property_flags)
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
            let memory_type_index = device.physical_device()
                .memory_type_index(&requirements, VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags)
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

pub struct DedicatedBufferMemory {
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    device: Arc<Device>,
    size: VkDeviceSize,
}

impl DedicatedBufferMemory {
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
            // find memory requirements
            let mut dedicated = MaybeUninit::<VkMemoryDedicatedRequirements>::zeroed();
            {
                let dedicated = dedicated.as_mut_ptr().as_mut().unwrap();
                dedicated.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS;
            }
            let mut requirements = MaybeUninit::<VkMemoryRequirements2>::zeroed();
            {
                let requirements = requirements.as_mut_ptr().as_mut().unwrap();
                requirements.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2;
                requirements.pNext = dedicated.as_mut_ptr() as *mut _;
            }
            let requirements_info = VkBufferMemoryRequirementsInfo2 {
                sType: VkStructureType::VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2,
                pNext: ptr::null(),
                buffer: buffer,
            };
            vkGetBufferMemoryRequirements2(device.handle(), &requirements_info, requirements.as_mut_ptr());
            let requirements = requirements.assume_init();
            // physical memory properties
            let memory_type_index = device.physical_device()
                .memory_type_index(&requirements.memoryRequirements, memory_property_flags)
                .ok_or_else(|| ErrorCode::SuitableBufferMemoryTypeNotFound)
                .unwrap();
            // allocation
            let mut memory = MaybeUninit::<VkDeviceMemory>::zeroed();
            {
                let use_device_address = (usage & VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags) 
                    == VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags;
                let allocate_flags = if use_device_address {
                    VkMemoryAllocateFlagBits::VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT as VkMemoryAllocateFlags
                } else {
                    0 as VkMemoryAllocateFlags
                };
                let allocate_flags_info = VkMemoryAllocateFlagsInfo {
                    sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO,
                    pNext: ptr::null(),
                    flags: allocate_flags,
                    deviceMask: 0,
                };
                let allocate_info = VkMemoryAllocateInfo {
                    sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
                    pNext: &allocate_flags_info as *const _ as *const c_void,
                    allocationSize: requirements.memoryRequirements.size,
                    memoryTypeIndex: memory_type_index,
                };
                vkAllocateMemory(device.handle(), &allocate_info, ptr::null(), memory.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let memory = memory.assume_init();
            // binding
            vkBindBufferMemory(device.handle(), buffer, memory, 0)
                .into_result()
                .unwrap();
            let buffer_memory = DedicatedBufferMemory { 
                buffer: buffer,
                memory: memory,
                device: Arc::clone(device),
                size: size,
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

    #[inline]
    pub fn size(&self) -> VkDeviceSize {
        self.size
    }

    pub fn buffer_device_address(&self) -> VkDeviceAddress {
        unsafe {
            let address_info = VkBufferDeviceAddressInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO,
                pNext: ptr::null(),
                buffer: self.buffer(),
            };
            vkGetBufferDeviceAddress(self.device.handle(), &address_info)
        }
    }
}

impl Drop for DedicatedBufferMemory {
    fn drop(&mut self) {
        unsafe {
            log_debug!("Drop DedicatedBufferMemory");
            vkDestroyBuffer(self.device.handle(), self.buffer, ptr::null());
            vkFreeMemory(self.device.handle(), self.memory, ptr::null());
        }
    }
}

pub struct DedicatedStagingBuffer {
    command_pool: Arc<CommandPool>,
    host_buffer_memory: Arc<DedicatedBufferMemory>,
    device_buffer_memory: Arc<DedicatedBufferMemory>,
    copying_command_buffer: Arc<CommandBuffer>,
    size: VkDeviceSize,
}

impl DedicatedStagingBuffer {
    pub fn new(command_pool: &Arc<CommandPool>, 
    device_buffer_usage: VkBufferUsageFlags, 
    device_memory_properties: VkMemoryPropertyFlags,
    size: VkDeviceSize,
) -> Result<Arc<Self>> {
        unsafe {
            let device = command_pool.queue().device();
            let host_buffer_memory = DedicatedBufferMemory::new(
                device,
                VK_BUFFER_USAGE_TRANSFER_SRC_BIT as VkBufferUsageFlags,
                VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as VkMemoryPropertyFlags
                    | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as VkMemoryPropertyFlags,
                size,
            )?;
            let device_buffer_memory = DedicatedBufferMemory::new(
                device,
                device_buffer_usage
                    | VK_BUFFER_USAGE_TRANSFER_DST_BIT as VkBufferUsageFlags,
                device_memory_properties,
                size,
            )?;
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
            let staging_buffer = DedicatedStagingBuffer {
                command_pool: Arc::clone(command_pool),
                host_buffer_memory,
                device_buffer_memory,
                copying_command_buffer: command_buffer,
                size,
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
            self.copying_command_buffer
                .submit_then_wait(VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags);
        }
    }

    pub unsafe fn update(&self, size: VkDeviceSize, func: impl FnOnce(*mut c_void)) {
        assert_eq!(size, self.size);
        let device = self.command_pool.queue().device();
        let host_buffer_memory = &self.host_buffer_memory;
        let mut mapped = MaybeUninit::<*mut c_void>::zeroed();
        vkMapMemory(device.handle(), host_buffer_memory.memory(), 0, size, 0, mapped.as_mut_ptr())
            .into_result()
            .unwrap();
        let mapped = mapped.assume_init();
        func(mapped);
        vkUnmapMemory(device.handle(), host_buffer_memory.memory());
        self.copying_command_buffer
            .submit_then_wait(VK_PIPELINE_STAGE_TRANSFER_BIT as VkPipelineStageFlags);
    }

    #[inline]
    pub fn host_buffer_memory(&self) -> &Arc<DedicatedBufferMemory> {
        &self.host_buffer_memory
    }

    #[inline]
    pub fn device_buffer_memory(&self) -> &Arc<DedicatedBufferMemory> {
        &self.device_buffer_memory
    }
}

pub struct UniformBuffer {
    staging_buffer: Arc<DedicatedStagingBuffer>,
}

impl UniformBuffer {
    pub fn new<Model>(command_pool: &Arc<CommandPool>, model: &Vec<Model>) -> Result<Arc<Self>> {
        unsafe {
            Self::init(command_pool, model)
        }
    }

    unsafe fn init<Model>(command_pool: &Arc<CommandPool>, model: &Vec<Model>) -> Result<Arc<Self>> {
        let size = std::mem::size_of::<Model>() * model.len();
        let staging_buffer = DedicatedStagingBuffer::new(command_pool, 
            VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            size as VkDeviceSize,
        )
            .unwrap();
        staging_buffer.write(model.as_ptr() as *const _ as *const c_void, size);
        let uniform_buffer = UniformBuffer {
            staging_buffer,
        };
        Ok(Arc::new(uniform_buffer))
    }

    #[inline]
    pub fn device_buffer_memory(&self) -> &Arc<DedicatedBufferMemory> {
        self.staging_buffer.device_buffer_memory()
    }

    pub fn update<Model>(&self, model: &Vec<Model>) {
        let size = std::mem::size_of::<Model>() * model.len();
        assert_eq!(size as VkDeviceSize, self.staging_buffer.host_buffer_memory().size());
        self.staging_buffer.write(model.as_ptr() as *const _ as *const c_void, size);
    }
}

pub struct StorageBuffer {
    staging_buffer: Arc<DedicatedStagingBuffer>,
}

impl StorageBuffer {
    pub fn new<Model>(
        command_pool: &Arc<CommandPool>, 
        model: &Vec<Model>,
    ) -> Result<Arc<Self>> {
        unsafe {
            Self::init(command_pool, model)
        }
    }

    unsafe fn init<Model>(
        command_pool: &Arc<CommandPool>, 
        model: &Vec<Model>,
    ) -> Result<Arc<Self>> {
        let size = std::mem::size_of::<Model>() * model.len();
        let staging_buffer = DedicatedStagingBuffer::new(command_pool, 
            VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            size as VkDeviceSize,
        )
            .unwrap();
        staging_buffer.write(model.as_ptr() as *const _ as *const c_void, size);
        let storage_buffer = StorageBuffer {
            staging_buffer,
        };
        Ok(Arc::new(storage_buffer))
    }

    #[inline]
    pub fn device_buffer_memory(&self) -> &Arc<DedicatedBufferMemory> {
        self.staging_buffer.device_buffer_memory()
    }

    pub fn update<Model>(&self, model: &Vec<Model>) {
        let size = std::mem::size_of::<Model>() * model.len();
        assert_eq!(size as VkDeviceSize, self.staging_buffer.host_buffer_memory().size());
        self.staging_buffer.write(model.as_ptr() as *const _ as *const c_void, size);
    }
}
