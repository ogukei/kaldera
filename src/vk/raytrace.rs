
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, ShaderModule, ShaderModuleSource};
use super::memory::{StagingBuffer, StagingBufferUsage, DedicatedBufferMemory, DedicatedStagingBuffer};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;
use std::ffi::CString;

use VkStructureType::*;
use VkStructureTypeExtRay::*;
use VkBuildAccelerationStructureFlagBitsKHR::*;
use VkAccelerationStructureTypeKHR::*;
use VkAccelerationStructureMemoryRequirementsTypeKHR::*;
use VkAccelerationStructureBuildTypeKHR::*;
use VkMemoryPropertyFlagBits::*;
use VkMemoryAllocateFlagBits::*;
use VkBufferUsageFlagBits::*;
use VkGeometryFlagBitsKHR::*;
use VkPipelineStageFlagBits::*;

pub struct BottomLevelAccelerationStructure {
    objects: Vec<Arc<BottomLevelAccelerationStructureGeometryObject>>,
}

impl BottomLevelAccelerationStructure {
    pub fn new(
        command_pool: &Arc<CommandPool>,
        geometries: Vec<Arc<BottomLevelAccelerationStructureGeometry>>,
    ) -> Result<Arc<Self>> {
        unsafe { Self::init(command_pool, geometries) }
    }

    unsafe fn init(
        command_pool: &Arc<CommandPool>,
        geometries: Vec<Arc<BottomLevelAccelerationStructureGeometry>>,
    ) -> Result<Arc<Self>> {
        let device = command_pool.queue().device();
        let builders = geometries.into_iter()
            .map(|v| BottomLevelAccelerationStructureGeometryBuilder::new(device, v))
            .collect::<Result<Vec<_>>>()?;
        let scratch_size = builders.iter()
            .map(|v| v.structure())
            .map(|v| v.scratch_memory_requirements())
            .map(|v| v.memoryRequirements.size)
            .max()
            .unwrap();
        let scratch_buffer_memory = DedicatedBufferMemory::new(
            device, 
            VK_BUFFER_USAGE_RAY_TRACING_BIT_KHR as VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            scratch_size)
            .unwrap();
        let objects: Vec<_> = builders.into_iter()
            .map(|v| v.build(command_pool, &scratch_buffer_memory))
            .collect();
        // build
        {
            let command_buffers: Vec<_> = objects.iter()
                .map(|v| v.command_buffer().handle())
                .collect();
            command_pool.queue()
                .submit_then_wait(command_buffers.as_slice())
                .unwrap();
        }
        let objects: Vec<_> = objects.into_iter()
            .map(|v| v.finalize())
            .collect();
        let structure = BottomLevelAccelerationStructure {
            objects,
        };
        Ok(Arc::new(structure))
    }
}

pub struct BottomLevelAccelerationStructureGeometry {
    type_info_vec: Vec<VkAccelerationStructureCreateGeometryTypeInfoKHR>,
    info_vec: Vec<VkAccelerationStructureGeometryKHR>,
    offset_vec: Vec<VkAccelerationStructureBuildOffsetInfoKHR>,
    vertex_buffer_memory: Arc<DedicatedBufferMemory>,
    index_buffer_memory: Arc<DedicatedBufferMemory>,
}

impl BottomLevelAccelerationStructureGeometry {
    pub fn new(
        num_vertices: u32,
        vertex_stride: VkDeviceSize,
        vertex_buffer_memory: &Arc<DedicatedBufferMemory>,
        num_indices: u32,
        index_buffer_memory: &Arc<DedicatedBufferMemory>,
    ) -> Result<Arc<Self>> {
        unsafe { Self::init(num_vertices, vertex_stride, vertex_buffer_memory, num_indices, index_buffer_memory) }
    }

    unsafe fn init(
        num_vertices: u32,
        vertex_stride: VkDeviceSize,
        vertex_buffer_memory: &Arc<DedicatedBufferMemory>,
        num_indices: u32,
        index_buffer_memory: &Arc<DedicatedBufferMemory>,
    ) -> Result<Arc<Self>> {
        // assumes single type info
        let primitive_count = num_indices / 3;
        let type_info = VkAccelerationStructureCreateGeometryTypeInfoKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_GEOMETRY_TYPE_INFO_KHR,
            pNext: ptr::null(),
            geometryType: VkGeometryTypeKHR::VK_GEOMETRY_TYPE_TRIANGLES_KHR,
            maxPrimitiveCount: primitive_count,
            indexType: VkIndexType::VK_INDEX_TYPE_UINT32,
            maxVertexCount: num_vertices,
            vertexFormat: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
            allowsTransforms: VK_FALSE,
        };
        let triangles_data = VkAccelerationStructureGeometryTrianglesDataKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR,
            pNext: ptr::null(),
            vertexFormat: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
            vertexData: vertex_buffer_memory.buffer_device_address(),
            vertexStride: vertex_stride,
            indexType: VkIndexType::VK_INDEX_TYPE_UINT32,
            indexData: index_buffer_memory.buffer_device_address(),
            transformData: 0,
        };
        let info = VkAccelerationStructureGeometryKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR,
            pNext: ptr::null(),
            geometryType: VkGeometryTypeKHR::VK_GEOMETRY_TYPE_TRIANGLES_KHR,
            geometry: triangles_data,
            flags: VK_GEOMETRY_OPAQUE_BIT_KHR as VkGeometryFlagsKHR,
        };
        let offset = VkAccelerationStructureBuildOffsetInfoKHR {
            primitiveCount: primitive_count,
            primitiveOffset: 0,
            firstVertex: 0,
            transformOffset: 0,
        };
        let geometry = Self {
            type_info_vec: vec![type_info],
            info_vec: vec![info],
            offset_vec: vec![offset],
            vertex_buffer_memory: Arc::clone(vertex_buffer_memory),
            index_buffer_memory: Arc::clone(index_buffer_memory),
        };
        Ok(Arc::new(geometry))
    }

    fn type_info_vec(&self) -> &Vec<VkAccelerationStructureCreateGeometryTypeInfoKHR> {
        &self.type_info_vec
    }

    fn info_vec(&self) -> &Vec<VkAccelerationStructureGeometryKHR> {
        &self.info_vec
    }

    fn offset_vec(&self) -> &Vec<VkAccelerationStructureBuildOffsetInfoKHR> {
        &self.offset_vec
    }
}

pub struct BottomLevelAccelerationStructureGeometryBuilder {
    geometry: Arc<BottomLevelAccelerationStructureGeometry>,
    structure: Arc<AccelerationStructure>,
}

impl BottomLevelAccelerationStructureGeometryBuilder {
    unsafe fn new(device: &Arc<Device>, geometry: Arc<BottomLevelAccelerationStructureGeometry>) -> Result<Self> {
        let create_info = VkAccelerationStructureCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            compactedSize: 0,
            r#type: VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR,
            flags: VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR as VkFlags,
            maxGeometryCount: geometry.type_info_vec().len() as u32,
            pGeometryInfos: geometry.type_info_vec().as_ptr(),
            deviceAddress: 0,
        };
        let structure = AccelerationStructure::new(device, &create_info)
            .unwrap();
        let object = Self {
            geometry: geometry,
            structure,
        };
        Ok(object)
    }

    fn structure(&self) -> &Arc<AccelerationStructure> {
        &self.structure
    }

    unsafe fn build(self, 
        command_pool: &Arc<CommandPool>, 
        scratch_buffer_memory: &Arc<DedicatedBufferMemory>,
    ) -> BottomLevelAccelerationStructureGeometryBuild {
        let structure = self.structure;
        let geometry = self.geometry;
        let device = command_pool.queue().device();
        let command_buffer = CommandBufferBuilder::new(command_pool).build(|command_buffer| {
            let build_info = VkAccelerationStructureBuildGeometryInfoKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
                pNext: ptr::null(),
                r#type: VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR,
                flags: VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR as VkFlags,
                update: VK_FALSE,
                srcAccelerationStructure: ptr::null_mut(),
                dstAccelerationStructure: structure.handle(),
                geometryArrayOfPointers: VK_FALSE,
                geometryCount: geometry.info_vec().len() as u32,
                ppGeometries: &geometry.info_vec().as_ptr(),
                scratchData: scratch_buffer_memory.buffer_device_address(),
            };
            // converts to an array of pointers to arrays of VkAccelerationStructureBuildOffsetInfoKHR
            let offset_ptr_vec = geometry.offset_vec().iter()
                .map(|v| v as *const VkAccelerationStructureBuildOffsetInfoKHR)
                .collect::<Vec<_>>();
            dispatch_vkCmdBuildAccelerationStructureKHR(device.handle(), 
                command_buffer, 1, &build_info, offset_ptr_vec.as_ptr());
            // memory barrier allowing reuse of scratch across builds
            let memory_barrier = VkMemoryBarrier {
                sType: VK_STRUCTURE_TYPE_MEMORY_BARRIER,
                pNext: ptr::null(),
                srcAccessMask: VkAccessFlagBits::VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR as VkAccessFlags,
                dstAccessMask: VkAccessFlagBits::VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR as VkAccessFlags
            };
            vkCmdPipelineBarrier(command_buffer, 
                VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR as VkPipelineStageFlags,
                VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR as VkPipelineStageFlags, 
                0, 
                1, &memory_barrier,
                0, ptr::null(),
                0, ptr::null(),
            );
        });
        BottomLevelAccelerationStructureGeometryBuild::new(
            geometry,
            structure,
            command_buffer,
            scratch_buffer_memory,
        )
    }
}

pub struct BottomLevelAccelerationStructureGeometryBuild {
    geometry: Arc<BottomLevelAccelerationStructureGeometry>,
    structure: Arc<AccelerationStructure>,
    command_buffer: Arc<CommandBuffer>,
    scratch_buffer_memory: Arc<DedicatedBufferMemory>,
}

impl BottomLevelAccelerationStructureGeometryBuild {
    fn new(
        geometry: Arc<BottomLevelAccelerationStructureGeometry>,
        structure: Arc<AccelerationStructure>,
        command_buffer: Arc<CommandBuffer>,
        scratch_buffer_memory: &Arc<DedicatedBufferMemory>,
    ) -> Self {
        let object = Self {
            geometry,
            structure,
            command_buffer,
            scratch_buffer_memory: Arc::clone(scratch_buffer_memory),
        };
        object
    }

    fn command_buffer(&self) -> &Arc<CommandBuffer> {
        &self.command_buffer
    }

    fn finalize(self) -> Arc<BottomLevelAccelerationStructureGeometryObject> {
        BottomLevelAccelerationStructureGeometryObject::new(self.geometry, self.structure)
    }
}

pub struct BottomLevelAccelerationStructureGeometryObject {
    geometry: Arc<BottomLevelAccelerationStructureGeometry>,
    structure: Arc<AccelerationStructure>,
}

impl BottomLevelAccelerationStructureGeometryObject {
    fn new(
        geometry: Arc<BottomLevelAccelerationStructureGeometry>,
        structure: Arc<AccelerationStructure>,
    ) -> Arc<Self> {
        let object = Self {
            geometry,
            structure,
        };
        Arc::new(object)
    }
}

pub struct AccelerationStructure {
    device: Arc<Device>,
    handle: VkAccelerationStructureKHR,
    memory: VkDeviceMemory,
    memory_requirements: VkMemoryRequirements2,
}

impl AccelerationStructure {
    pub fn new(
        device: &Arc<Device>, 
        create_info: &VkAccelerationStructureCreateInfoKHR,
    ) -> Result<Arc<Self>> {
        unsafe { Self::init(device, create_info) }
    }

    unsafe fn init(device: &Arc<Device>, create_info: &VkAccelerationStructureCreateInfoKHR) -> Result<Arc<Self>> {
        let mut structure_handle = MaybeUninit::<VkAccelerationStructureKHR>::zeroed();
        vkCreateAccelerationStructureKHR(device.handle(), create_info, ptr::null(), structure_handle.as_mut_ptr())
            .into_result()
            .unwrap();
        let structure_handle = structure_handle.assume_init();
        // find memory requirements
        let mut requirements = MaybeUninit::<VkMemoryRequirements2>::zeroed();
        {
            {
                let requirements = requirements.as_mut_ptr().as_mut().unwrap();
                requirements.sType = VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2;
            }
            let memory_info = VkAccelerationStructureMemoryRequirementsInfoKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_KHR,
                pNext: ptr::null(),
                r#type: VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_KHR,
                accelerationStructure: structure_handle,
                buildType: VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR,
            };
            vkGetAccelerationStructureMemoryRequirementsKHR(device.handle(), &memory_info, requirements.as_mut_ptr());
        }
        let requirements = requirements.assume_init();
        // allocate memory
        let mut memory = MaybeUninit::<VkDeviceMemory>::zeroed();
        {
            let memory_type_index = device.physical_device()
                .memory_type_index(&requirements.memoryRequirements, 
                    VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags)
                .ok_or_else(|| ErrorCode::SuitableBufferMemoryTypeNotFound)
                .unwrap();
            let allocate_flags_info = VkMemoryAllocateFlagsInfo {
                sType: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO,
                pNext: ptr::null(),
                flags: VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT as VkMemoryAllocateFlags,
                deviceMask: 0,
            };
            let allocate_info = VkMemoryAllocateInfo {
                sType: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
                pNext: &allocate_flags_info as *const _ as *const c_void,
                allocationSize: requirements.memoryRequirements.size,
                memoryTypeIndex: memory_type_index,
            };
            vkAllocateMemory(device.handle(), &allocate_info, ptr::null(), memory.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let memory = memory.assume_init();
        // bind memory
        {
            let bind_info = VkBindAccelerationStructureMemoryInfoKHR {
                sType: VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_KHR,
                pNext: ptr::null(),
                accelerationStructure: structure_handle,
                memory: memory,
                memoryOffset: 0,
                deviceIndexCount: 0,
                pDeviceIndices: ptr::null(),
            };
            vkBindAccelerationStructureMemoryKHR(device.handle(), 1, &bind_info)
                .into_result()
                .unwrap();
        }
        let structure = AccelerationStructure {
            device: Arc::clone(device),
            handle: structure_handle,
            memory,
            memory_requirements: requirements,
        };
        Ok(Arc::new(structure))
    }

    #[inline]
    pub fn handle(&self) -> VkAccelerationStructureKHR {
        self.handle
    }

    fn memory_requirements(&self) -> &VkMemoryRequirements2 {
        &self.memory_requirements
    }

    fn scratch_memory_requirements(&self) -> VkMemoryRequirements2 {
        let mut requirements = MaybeUninit::<VkMemoryRequirements2>::zeroed();
        unsafe {
            {
                let requirements = requirements.as_mut_ptr().as_mut().unwrap();
                requirements.sType = VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2;
            }
            let memory_info = VkAccelerationStructureMemoryRequirementsInfoKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_KHR,
                pNext: ptr::null(),
                r#type: VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_KHR,
                accelerationStructure: self.handle(),
                buildType: VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR,
            };
            vkGetAccelerationStructureMemoryRequirementsKHR(self.device.handle(), &memory_info, requirements.as_mut_ptr());
            requirements.assume_init()
        }
    }
}

impl Drop for AccelerationStructure {
    fn drop(&mut self) {
        unsafe {
            log_debug!("Drop AccelerationStructure");
            vkDestroyAccelerationStructureKHR(self.device.handle(), self.handle, ptr::null());
            vkFreeMemory(self.device.handle(), self.memory, ptr::null());
        }
    }
}

pub struct AccelerationVertexStagingBuffer {
    vertex_buffer: Arc<DedicatedStagingBuffer>,
    index_buffer: Arc<DedicatedStagingBuffer>,
    index_count: usize,
}

impl AccelerationVertexStagingBuffer {
    pub fn new<Vertex>(command_pool: &Arc<CommandPool>, vertices: Vec<Vertex>, indices: Vec<u32>) -> Arc<Self> {
        let vertex_buffer_size = std::mem::size_of::<Vertex>() * vertices.len();
        let vertex_buffer = DedicatedStagingBuffer::new(
            command_pool, 
            VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as VkBufferUsageFlags 
                | VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            vertex_buffer_size as VkDeviceSize,
        ).unwrap();
        let index_buffer_size = std::mem::size_of::<u32>() * indices.len();
        let index_buffer = DedicatedStagingBuffer::new(
            command_pool, 
            VK_BUFFER_USAGE_INDEX_BUFFER_BIT as VkBufferUsageFlags 
                | VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as  VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            vertex_buffer_size as VkDeviceSize,
        ).unwrap();
        // transfer
        vertex_buffer.write(vertices.as_ptr() as *const c_void, vertex_buffer_size);
        index_buffer.write(indices.as_ptr() as *const c_void, index_buffer_size);
        let buffer = AccelerationVertexStagingBuffer {
            vertex_buffer,
            index_buffer,
            index_count: indices.len(),
        };
        Arc::new(buffer)
    }

    #[inline]
    pub fn vertex_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.vertex_buffer
    }

    #[inline]
    pub fn index_buffer(&self) -> &Arc<DedicatedStagingBuffer> {
        &self.index_buffer
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.index_count
    }
}
