
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, ShaderModule, ShaderModuleSource, CommandBufferRecording};
use super::memory::{StorageBuffer, UniformBuffer, DedicatedBufferMemory, DedicatedStagingBuffer};
use super::image::{ColorImage, Texture};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void, size_t};
use std::sync::Arc;
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
use VkGeometryInstanceFlagBitsKHR::*;

pub struct BottomLevelAccelerationStructuresBuilder<'a> {
    command_pool: &'a Arc<CommandPool>,
    geometries: &'a [Arc<BottomLevelAccelerationStructureGeometry>],
}

impl<'a> BottomLevelAccelerationStructuresBuilder<'a> {
    pub fn new(command_pool: &'a Arc<CommandPool>, geometries: &'a [Arc<BottomLevelAccelerationStructureGeometry>]) -> Self {
        Self {
            command_pool,
            geometries,
        }
    }

    pub fn build(self) -> Result<Vec<Arc<BottomLevelAccelerationStructure>>> {
        let command_pool = self.command_pool;
        let device = command_pool.queue().device();
        let recording = CommandBufferRecording::new_onetime_submit(command_pool)?;
        let builds = self.geometries.iter()
            .map(|v| BottomLevelAccelerationStructureBuild::new(&recording, std::slice::from_ref(v)))
            .collect::<Result<Vec<_>>>()?;
        let scratch_size = builds.iter()
            .map(|v| v.scratch_size())
            .max()
            .unwrap();
        let scratch_buffer_memory = DedicatedBufferMemory::new(
            device, 
            VK_BUFFER_USAGE_RAY_TRACING_BIT_KHR as VkBufferUsageFlags
                | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            scratch_size,
        )
            .unwrap();
        let objects: Vec<_> = builds.into_iter()
            .map(|v| v.build(&scratch_buffer_memory))
            .collect();
        let command_buffer = recording.complete();
        unsafe {
            let command_buffer_handle = command_buffer.handle();
            command_pool.queue()
                .submit_then_wait(std::slice::from_ref(&command_buffer_handle))
                .unwrap();
        }
        objects.into_iter()
            .map(BottomLevelAccelerationStructure::new)
            .collect()
    }
}

struct BottomLevelAccelerationStructureBuild<'a, 'b: 'a> {
    recording: &'a CommandBufferRecording<'b>,
    scratch_size: VkDeviceSize,
    geometry_builds: Vec<BottomLevelAccelerationStructureGeometryBuild>,
}

impl<'a, 'b: 'a> BottomLevelAccelerationStructureBuild<'a, 'b> {
    fn new(
        recording: &'b CommandBufferRecording<'a>, 
        geometries: &[Arc<BottomLevelAccelerationStructureGeometry>],
    ) -> Result<Self> {
        unsafe {
            let command_pool = recording.command_pool();
            let device = command_pool.queue().device();
            let geometry_builds = geometries.iter()
                .map(|v| BottomLevelAccelerationStructureGeometryBuild::new(device, v))
                .collect::<Result<Vec<_>>>()?;
            let scratch_size = geometry_builds.iter()
                .map(|v| v.structure())
                .map(|v| v.scratch_memory_requirements())
                .map(|v| v.memoryRequirements.size)
                .max()
                .unwrap();
            let builder = Self {
                recording,
                geometry_builds,
                scratch_size,
            };
            Ok(builder)
        }
    }

    #[inline]
    fn scratch_size(&self) -> VkDeviceSize {
        self.scratch_size
    }

    fn build(self, scratch_buffer: &Arc<DedicatedBufferMemory>) -> BottomLevelAccelerationStructureGeometryObject {
        assert_eq!(self.geometry_builds.len(), 1, "multiple geometries not supported yet");
        let build = self.geometry_builds.into_iter().nth(0).unwrap();
        unsafe {
            build.build(self.recording, scratch_buffer)
        }
    }
}

pub struct BottomLevelAccelerationStructure {
    object: BottomLevelAccelerationStructureGeometryObject,
}

impl BottomLevelAccelerationStructure {
    fn new(object: BottomLevelAccelerationStructureGeometryObject) -> Result<Arc<Self>> {
        let structure = Self {
            object,
        };
        Ok(Arc::new(structure))
    }

    fn structure_device_address(&self) -> VkDeviceAddress {
        self.object.structure().device_address()
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
        vertex_offset_index: u32,
        vertex_buffer_memory: &Arc<DedicatedBufferMemory>,
        num_indices: u32,
        index_offset_index: u32,
        index_buffer_memory: &Arc<DedicatedBufferMemory>,
    ) -> Result<Arc<Self>> {
        unsafe { 
            Self::init(num_vertices, 
                vertex_stride, 
                vertex_offset_index, 
                vertex_buffer_memory, 
                num_indices, 
                index_offset_index, 
                index_buffer_memory) 
        }
    }

    unsafe fn init(
        num_vertices: u32,
        vertex_stride: VkDeviceSize,
        vertex_offset_index: u32,
        vertex_buffer_memory: &Arc<DedicatedBufferMemory>,
        num_indices: u32,
        index_offset_index: u32,
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
        let geometry_data = VkAccelerationStructureGeometryDataKHR {
            triangles: triangles_data,
        };
        let info = VkAccelerationStructureGeometryKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR,
            pNext: ptr::null(),
            geometryType: VkGeometryTypeKHR::VK_GEOMETRY_TYPE_TRIANGLES_KHR,
            geometry: geometry_data,
            flags: VK_GEOMETRY_OPAQUE_BIT_KHR as VkGeometryFlagsKHR,
        };
        let offset = VkAccelerationStructureBuildOffsetInfoKHR {
            primitiveCount: primitive_count,
            primitiveOffset: index_offset_index * std::mem::size_of::<u32>() as u32,
            firstVertex: vertex_offset_index,
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

pub struct BottomLevelAccelerationStructureGeometryBuild {
    geometry: Arc<BottomLevelAccelerationStructureGeometry>,
    structure: Arc<AccelerationStructure>,
}

impl BottomLevelAccelerationStructureGeometryBuild {
    unsafe fn new(device: &Arc<Device>, geometry: &Arc<BottomLevelAccelerationStructureGeometry>) -> Result<Self> {
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
            geometry: Arc::clone(geometry),
            structure,
        };
        Ok(object)
    }

    fn structure(&self) -> &Arc<AccelerationStructure> {
        &self.structure
    }

    unsafe fn build(self, 
        recording: &CommandBufferRecording, 
        scratch_buffer_memory: &Arc<DedicatedBufferMemory>,
    ) -> BottomLevelAccelerationStructureGeometryObject {
        let structure = self.structure;
        let geometry = self.geometry;
        let command_pool = recording.command_pool();
        let device = command_pool.queue().device();
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
            recording.command_buffer(), 1, &build_info, offset_ptr_vec.as_ptr());
        // memory barrier allowing reuse of scratch across builds
        let memory_barrier = VkMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: VkAccessFlagBits::VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR as VkAccessFlags,
            dstAccessMask: VkAccessFlagBits::VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR as VkAccessFlags
        };
        vkCmdPipelineBarrier(recording.command_buffer(), 
            VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR as VkPipelineStageFlags,
            VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR as VkPipelineStageFlags, 
            0, 
            1, &memory_barrier,
            0, ptr::null(),
            0, ptr::null(),
        );
        BottomLevelAccelerationStructureGeometryObject {
            geometry,
            structure,
        }
    }
}

struct BottomLevelAccelerationStructureGeometryObject {
    geometry: Arc<BottomLevelAccelerationStructureGeometry>,
    structure: Arc<AccelerationStructure>,
}

impl BottomLevelAccelerationStructureGeometryObject {
    #[inline]
    fn structure(&self) -> &Arc<AccelerationStructure> {
        &self.structure
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

    pub fn device_address(&self) -> VkDeviceAddress {
        unsafe {
            let info = VkAccelerationStructureDeviceAddressInfoKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
                pNext: ptr::null(),
                accelerationStructure: self.handle,
            };
            vkGetAccelerationStructureDeviceAddressKHR(self.device.handle(), &info)
        }
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

pub struct TopLevelAccelerationStructureInstance {
    instance_custom_index: u32,
    transform: VkTransformMatrixKHR,
    bottom_level_acceleration_structure: Arc<BottomLevelAccelerationStructure>,
}

impl TopLevelAccelerationStructureInstance {
    pub fn new(
        instance_custom_index: u32, 
        transform: VkTransformMatrixKHR,
        bottom_level_acceleration_structure: &Arc<BottomLevelAccelerationStructure>,
    ) -> Result<Arc<Self>> {
        let instance = Self {
            instance_custom_index,
            transform,
            bottom_level_acceleration_structure: Arc::clone(bottom_level_acceleration_structure),
        };
        Ok(Arc::new(instance))
    }

    #[inline]
    fn instance_custom_index(&self) -> u32 {
        self.instance_custom_index
    }

    #[inline]
    fn bottom_level_acceleration_structure(&self) -> &Arc<BottomLevelAccelerationStructure> {
        &self.bottom_level_acceleration_structure
    }

    #[inline]
    fn transform(&self) -> VkTransformMatrixKHR {
        self.transform.clone()
    }
}

pub struct TopLevelAccelerationStructure {
    instances_buffer: Arc<DedicatedStagingBuffer>,
    structure: Arc<AccelerationStructure>,
    instances: Vec<Arc<TopLevelAccelerationStructureInstance>>,
}

impl TopLevelAccelerationStructure {
    pub fn new(
        command_pool: &Arc<CommandPool>, 
        instances: Vec<Arc<TopLevelAccelerationStructureInstance>>,
    ) -> Result<Arc<Self>> {
        unsafe {
            Self::init(command_pool, instances)
        }
    }

    unsafe fn init(
        command_pool: &Arc<CommandPool>, 
        instances: Vec<Arc<TopLevelAccelerationStructureInstance>>,
    ) -> Result<Arc<Self>> {
        let device = command_pool.queue().device();
        let type_info = VkAccelerationStructureCreateGeometryTypeInfoKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_GEOMETRY_TYPE_INFO_KHR,
            pNext: ptr::null(),
            geometryType: VkGeometryTypeKHR::VK_GEOMETRY_TYPE_INSTANCES_KHR,
            maxPrimitiveCount: instances.len() as u32,
            indexType: VkIndexType::VK_INDEX_TYPE_UINT16,
            maxVertexCount: 0,
            vertexFormat: VkFormat::VK_FORMAT_UNDEFINED,
            allowsTransforms: VK_FALSE,
        };
        let create_info = VkAccelerationStructureCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            compactedSize: 0,
            r#type: VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR,
            flags: VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR as VkFlags,
            maxGeometryCount: 1,
            pGeometryInfos: &type_info,
            deviceAddress: 0,
        };
        let structure = AccelerationStructure::new(device, &create_info)
            .unwrap();
        let instance_structs: Vec<_> = instances.iter()
            .map(|instance| {
                let custom_index = instance.instance_custom_index();
                let structure = instance.bottom_level_acceleration_structure();
                let transform = instance.transform();
                let instance = VkAccelerationStructureInstanceKHR {
                    transform: transform,
                    instanceCustomIndexAndMask: (0xff << 24) | (custom_index & ((1u32 << 25) - 1)),
                    instanceShaderBindingTableRecordOffsetAndFlags: 
                        (VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR as VkFlags) << 24,
                    accelerationStructureReference: structure.structure_device_address(),
                };
                instance
            })
            .collect();
        let instances_size = instance_structs.len() * std::mem::size_of::<VkAccelerationStructureInstanceKHR>();
        let instances_buffer = DedicatedStagingBuffer::new(command_pool, 
            VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags, 
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            instances_size as VkDeviceSize)
            .unwrap();
        instances_buffer.write(instance_structs.as_ptr() as *const c_void, instances_size);
        // build
        {
            let geometry_instances = VkAccelerationStructureGeometryInstancesDataKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR,
                pNext: ptr::null(),
                arrayOfPointers: VK_FALSE,
                data: instances_buffer.device_buffer_memory().buffer_device_address(),
            };
            let geometry_data = VkAccelerationStructureGeometryDataKHR {
                instances: geometry_instances,
            };
            let geometry = VkAccelerationStructureGeometryKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR,
                pNext: ptr::null(),
                flags: VK_GEOMETRY_OPAQUE_BIT_KHR as VkGeometryFlagsKHR,
                geometryType: VkGeometryTypeKHR::VK_GEOMETRY_TYPE_INSTANCES_KHR,
                geometry: geometry_data,
            };
            let geometry_vec = vec![geometry];
            let scratch_size = structure.scratch_memory_requirements()
                .memoryRequirements.size;
            let scratch_buffer_memory = DedicatedBufferMemory::new(
                device, 
                VK_BUFFER_USAGE_RAY_TRACING_BIT_KHR as VkBufferUsageFlags
                    | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
                VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
                scratch_size)
                .unwrap();
            let build_info = VkAccelerationStructureBuildGeometryInfoKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
                pNext: ptr::null(),
                r#type: VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR,
                flags: VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR as VkFlags,
                update: VK_FALSE,
                srcAccelerationStructure: ptr::null_mut(),
                dstAccelerationStructure: structure.handle(),
                geometryArrayOfPointers: VK_FALSE,
                geometryCount: 1,
                ppGeometries: &geometry_vec.as_ptr() ,
                scratchData: scratch_buffer_memory.buffer_device_address(),
            };
            let offset = VkAccelerationStructureBuildOffsetInfoKHR {
                primitiveCount: instances.len() as u32,
                primitiveOffset: 0,
                firstVertex: 0,
                transformOffset: 0,
            };
            let offset_ptr_vec = vec![&offset as *const VkAccelerationStructureBuildOffsetInfoKHR];
            let command_buffer = CommandBufferBuilder::new(command_pool).build(|command_buffer| {
                dispatch_vkCmdBuildAccelerationStructureKHR(device.handle(), 
                    command_buffer, 1, &build_info, offset_ptr_vec.as_ptr());
            });
            let command_buffers = vec![command_buffer.handle()];
            command_pool.queue()
                .submit_then_wait(command_buffers.as_slice())
                .unwrap();
        }
        let top_level_structure = TopLevelAccelerationStructure {
            instances_buffer,
            structure,
            instances,
        };
        Ok(Arc::new(top_level_structure))
    }

    #[inline]
    pub fn handle(&self) -> VkAccelerationStructureKHR {
        self.structure.handle()
    }
}


pub struct RayTracingGraphicsPipeline {
    device: Arc<Device>,
    layout: VkPipelineLayout,
    handle: VkPipeline,
    descriptor_set_layout: VkDescriptorSetLayout,
}

impl RayTracingGraphicsPipeline {
    pub fn new(
        device: &Arc<Device>,
        textures_count: usize,
    ) -> Result<Arc<Self>> {
        unsafe {
            Self::init(device, textures_count)
        }
    }

    unsafe fn init(device: &Arc<Device>, textures_count: usize) -> Result<Arc<Self>> {
        // Descriptor Set Layout
        let mut descriptor_set_layout = MaybeUninit::<VkDescriptorSetLayout>::zeroed();
        {
            let bindings = vec![
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_RAYGEN_BIT_KHR as u32,
                    0,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_RAYGEN_BIT_KHR as u32,
                    1,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_RAYGEN_BIT_KHR as u32,
                    2,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32,
                    3,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32,
                    4,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32,
                    5,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32,
                    6,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32,
                    7,
                ),
                VkDescriptorSetLayoutBinding::new_array(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32,
                    8,
                    textures_count,
                ),
            ];
            let create_info = VkDescriptorSetLayoutCreateInfo::new(bindings.len() as u32, bindings.as_ptr());
            vkCreateDescriptorSetLayout(device.handle(), &create_info, ptr::null(), descriptor_set_layout.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let descriptor_set_layout = descriptor_set_layout.assume_init();
        // Pipeline Layout
        let mut pipeline_layout = MaybeUninit::<VkPipelineLayout>::zeroed();
        {
            let create_info = VkPipelineLayoutCreateInfo::new(1, &descriptor_set_layout);
            vkCreatePipelineLayout(device.handle(), &create_info, ptr::null(), pipeline_layout.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let pipeline_layout = pipeline_layout.assume_init();
        // Shader Stages
        let raygen_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/ray.raygen.rgen.spv")).unwrap();
        let rmiss_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/ray.miss.rmiss.spv")).unwrap();
        let rchit_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/ray.closesthit.rchit.spv")).unwrap();
        let shader_entry_point = CString::new("main").unwrap();
        const INDEX_RAYGEN: u32 = 0;
        const INDEX_MISS: u32 = 1;
        const INDEX_CLOSEST_HIT: u32 = 2;
        let shader_stages = vec![
            VkPipelineShaderStageCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                stage: VkShaderStageFlagBits::VK_SHADER_STAGE_RAYGEN_BIT_KHR,
                module: raygen_shader_module.handle(),
                pName: shader_entry_point.as_ptr(),
                pSpecializationInfo: ptr::null(),
            },
            VkPipelineShaderStageCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                stage: VkShaderStageFlagBits::VK_SHADER_STAGE_MISS_BIT_KHR,
                module: rmiss_shader_module.handle(),
                pName: shader_entry_point.as_ptr(),
                pSpecializationInfo: ptr::null(),
            },
            VkPipelineShaderStageCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                stage: VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR,
                module: rchit_shader_module.handle(),
                pName: shader_entry_point.as_ptr(),
                pSpecializationInfo: ptr::null(),
            },
        ];
        let shader_groups = vec![
            VkRayTracingShaderGroupCreateInfoKHR {
                sType: VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
                pNext: ptr::null(),
                r#type: VkRayTracingShaderGroupTypeKHR::VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR,
                generalShader: INDEX_RAYGEN,
                closestHitShader: VK_SHADER_UNUSED_KHR,
                anyHitShader: VK_SHADER_UNUSED_KHR,
                intersectionShader: VK_SHADER_UNUSED_KHR,
                pShaderGroupCaptureReplayHandle: ptr::null(),
            },
            VkRayTracingShaderGroupCreateInfoKHR {
                sType: VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
                pNext: ptr::null(),
                r#type: VkRayTracingShaderGroupTypeKHR::VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR,
                generalShader: INDEX_MISS,
                closestHitShader: VK_SHADER_UNUSED_KHR,
                anyHitShader: VK_SHADER_UNUSED_KHR,
                intersectionShader: VK_SHADER_UNUSED_KHR,
                pShaderGroupCaptureReplayHandle: ptr::null(),
            },
            VkRayTracingShaderGroupCreateInfoKHR {
                sType: VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
                pNext: ptr::null(),
                r#type: VkRayTracingShaderGroupTypeKHR::VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR,
                generalShader: VK_SHADER_UNUSED_KHR,
                closestHitShader: INDEX_CLOSEST_HIT,
                anyHitShader: VK_SHADER_UNUSED_KHR,
                intersectionShader: VK_SHADER_UNUSED_KHR,
                pShaderGroupCaptureReplayHandle: ptr::null(),
            },
        ];
        let libraries = VkPipelineLibraryCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR,
            pNext: ptr::null(),
            libraryCount: 0,
            pLibraries: ptr::null(),
        };
        let create_info = VkRayTracingPipelineCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: 0,
            stageCount: shader_stages.len() as u32,
            pStages: shader_stages.as_ptr(),
            groupCount: shader_groups.len() as u32,
            pGroups: shader_groups.as_ptr(),
            maxRecursionDepth: 1,
            libraries: libraries,
            pLibraryInterface: ptr::null(),
            layout: pipeline_layout,
            basePipelineHandle: ptr::null_mut(),
            basePipelineIndex: 0,
        };
        let mut handle = MaybeUninit::<VkPipeline>::zeroed();
        vkCreateRayTracingPipelinesKHR(device.handle(), ptr::null_mut(), 1, &create_info, ptr::null(), handle.as_mut_ptr())
            .into_result()
            .unwrap();
        let handle = handle.assume_init();
        let layout = RayTracingGraphicsPipeline {
            device: Arc::clone(device),
            layout: pipeline_layout,
            handle,
            descriptor_set_layout,
        };
        Ok(Arc::new(layout))
    }

    #[inline]
    pub fn layout(&self) -> VkPipelineLayout {
        self.layout
    }

    #[inline]
    pub fn handle(&self) -> VkPipeline {
        self.handle
    }

    #[inline]
    pub fn descriptor_set_layout(&self) -> VkDescriptorSetLayout {
        self.descriptor_set_layout
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl Drop for RayTracingGraphicsPipeline {
    fn drop(&mut self) {
        log_debug!("Drop RayTracingGraphicsPipeline");
        unsafe {
            let device = &self.device;
            vkDestroyPipelineLayout(device.handle(), self.layout, ptr::null());
            vkDestroyDescriptorSetLayout(device.handle(), self.descriptor_set_layout, ptr::null());
            vkDestroyPipeline(device.handle(), self.handle, ptr::null());
        }
    }
}

pub struct RayTracingDescriptorSets {
    device: Arc<Device>,
    pipeline: Arc<RayTracingGraphicsPipeline>,
    acceleration_structure: Arc<TopLevelAccelerationStructure>,
    storage_image: Arc<ColorImage>,
    scene_uniform_buffer: Arc<UniformBuffer>,
    vertex_storage_buffer: Arc<DedicatedStagingBuffer>,
    index_storage_buffer: Arc<DedicatedStagingBuffer>,
    normal_storage_buffer: Arc<DedicatedStagingBuffer>,
    description_storage_buffer: Arc<DedicatedStagingBuffer>,
    texcoord_storage_buffer: Arc<DedicatedStagingBuffer>,
    textures: Vec<Arc<Texture>>,
    descriptor_pool: VkDescriptorPool,
    descriptor_set: VkDescriptorSet,
}

impl RayTracingDescriptorSets {
    pub fn new(
        pipeline: &Arc<RayTracingGraphicsPipeline>, 
        acceleration_structure: &Arc<TopLevelAccelerationStructure>,
        storage_image: &Arc<ColorImage>,
        scene_uniform_buffer: &Arc<UniformBuffer>,
        vertex_storage_buffer: &Arc<DedicatedStagingBuffer>,
        index_storage_buffer: &Arc<DedicatedStagingBuffer>,
        normal_storage_buffer: &Arc<DedicatedStagingBuffer>,
        description_storage_buffer: &Arc<DedicatedStagingBuffer>,
        texcoord_storage_buffer: &Arc<DedicatedStagingBuffer>,
        textures: &[Arc<Texture>],
    ) -> Result<Arc<Self>> {
        unsafe {
            Self::init(pipeline, 
                acceleration_structure, 
                storage_image, 
                scene_uniform_buffer, 
                vertex_storage_buffer, 
                index_storage_buffer, 
                normal_storage_buffer, 
                description_storage_buffer,
                texcoord_storage_buffer,
                textures,
            )
        }
    }

    unsafe fn init(
        pipeline: &Arc<RayTracingGraphicsPipeline>, 
        acceleration_structure: &Arc<TopLevelAccelerationStructure>,
        storage_image: &Arc<ColorImage>,
        scene_uniform_buffer: &Arc<UniformBuffer>,
        vertex_storage_buffer: &Arc<DedicatedStagingBuffer>,
        index_storage_buffer: &Arc<DedicatedStagingBuffer>,
        normal_storage_buffer: &Arc<DedicatedStagingBuffer>,
        description_storage_buffer: &Arc<DedicatedStagingBuffer>,
        texcoord_storage_buffer: &Arc<DedicatedStagingBuffer>,
        textures: &[Arc<Texture>],
    ) -> Result<Arc<Self>> {
        let device = pipeline.device();
        // Descriptor Pool
        let mut descriptor_pool = MaybeUninit::<VkDescriptorPool>::zeroed();
        {
            let sizes = vec![
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, textures.len() as u32),
            ];
            let create_info = VkDescriptorPoolCreateInfo::new(1, sizes.len() as u32, sizes.as_ptr());
            vkCreateDescriptorPool(device.handle(), &create_info, ptr::null(), descriptor_pool.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let descriptor_pool = descriptor_pool.assume_init();
        // Allocate Descriptor Set
        let mut descriptor_set = MaybeUninit::<VkDescriptorSet>::zeroed();
        {
            let descriptor_set_layout = pipeline.descriptor_set_layout();
            let alloc_info = VkDescriptorSetAllocateInfo::new(descriptor_pool, 1, &descriptor_set_layout);
            vkAllocateDescriptorSets(device.handle(), &alloc_info, descriptor_set.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let descriptor_set = descriptor_set.assume_init();
        let acceleration_structure_handle = acceleration_structure.handle();
        let write_acceleration_structure_info = VkWriteDescriptorSetAccelerationStructureKHR {
            sType: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR,
            pNext: ptr::null(),
            accelerationStructureCount: 1,
            pAccelerationStructures: &acceleration_structure_handle,
        };
        let write_acceleration_structure = VkWriteDescriptorSet {
            sType: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            pNext: &write_acceleration_structure_info as *const _ as *const c_void,
            dstSet: descriptor_set,
            dstBinding: 0,
            dstArrayElement: 0,
            descriptorCount: 1,
            descriptorType: VkDescriptorType::VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR,
            pImageInfo: ptr::null(),
            pBufferInfo: ptr::null(),
            pTexelBufferView: ptr::null(),
        };
        let write_image_info = VkDescriptorImageInfo {
            sampler: ptr::null_mut(),
            imageView: storage_image.view(),
            imageLayout: VkImageLayout::VK_IMAGE_LAYOUT_GENERAL,
        };
        let write_image = VkWriteDescriptorSet::from_image(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
            1,
            &write_image_info);
        let write_uniform_buffer_info = VkDescriptorBufferInfo {
            buffer: scene_uniform_buffer.device_buffer_memory().buffer(),
            offset: 0,
            range: scene_uniform_buffer.device_buffer_memory().size(),
        };
        let write_uniform_buffer = VkWriteDescriptorSet::from_buffer(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            2,
            &write_uniform_buffer_info);
        let write_vertex_buffer_info = VkDescriptorBufferInfo {
            buffer: vertex_storage_buffer.device_buffer_memory().buffer(),
            offset: 0,
            range: vertex_storage_buffer.device_buffer_memory().size(),
        };
        let write_vertex_buffer = VkWriteDescriptorSet::from_buffer(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            3,
            &write_vertex_buffer_info);
        let write_index_buffer_info = VkDescriptorBufferInfo {
            buffer: index_storage_buffer.device_buffer_memory().buffer(),
            offset: 0,
            range: index_storage_buffer.device_buffer_memory().size(),
        };
        let write_index_buffer = VkWriteDescriptorSet::from_buffer(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            4,
            &write_index_buffer_info);
        let write_normal_buffer_info = VkDescriptorBufferInfo {
            buffer: normal_storage_buffer.device_buffer_memory().buffer(),
            offset: 0,
            range: normal_storage_buffer.device_buffer_memory().size(),
        };
        let write_normal_buffer = VkWriteDescriptorSet::from_buffer(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            5,
            &write_normal_buffer_info);
        let write_description_buffer_info = VkDescriptorBufferInfo {
            buffer: description_storage_buffer.device_buffer_memory().buffer(),
            offset: 0,
            range: description_storage_buffer.device_buffer_memory().size(),
        };
        let write_description_buffer = VkWriteDescriptorSet::from_buffer(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            6,
            &write_description_buffer_info);
        let write_texcoord_buffer_info = VkDescriptorBufferInfo {
            buffer: texcoord_storage_buffer.device_buffer_memory().buffer(),
            offset: 0,
            range: texcoord_storage_buffer.device_buffer_memory().size(),
        };
        let write_texcoord_buffer = VkWriteDescriptorSet::from_buffer(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            7,
            &write_texcoord_buffer_info);
        let textures: Vec<_> = textures.iter().map(Arc::clone).collect();
        let texture_descriptors: Vec<VkDescriptorImageInfo> = textures.iter()
            .map(|v| v.descriptor())
            .collect();
        let write_texture_images = VkWriteDescriptorSet::from_image_array(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            8,
            texture_descriptors.len(),
            texture_descriptors.as_ptr());
        let write_descriptor_sets = vec![
            write_acceleration_structure,
            write_image,
            write_uniform_buffer,
            write_vertex_buffer,
            write_index_buffer,
            write_normal_buffer,
            write_description_buffer,
            write_texcoord_buffer,
            write_texture_images,
        ];
        vkUpdateDescriptorSets(device.handle(), 
            write_descriptor_sets.len() as u32, 
            write_descriptor_sets.as_ptr(), 
            0, 
            ptr::null());
        let descriptors = Self {
            device: Arc::clone(device),
            pipeline: Arc::clone(pipeline),
            acceleration_structure: Arc::clone(acceleration_structure),
            storage_image: Arc::clone(storage_image),
            scene_uniform_buffer: Arc::clone(scene_uniform_buffer),
            vertex_storage_buffer: Arc::clone(vertex_storage_buffer),
            index_storage_buffer: Arc::clone(index_storage_buffer),
            normal_storage_buffer: Arc::clone(normal_storage_buffer),
            description_storage_buffer: Arc::clone(description_storage_buffer),
            texcoord_storage_buffer: Arc::clone(texcoord_storage_buffer),
            textures,
            descriptor_pool,
            descriptor_set,
        };
        Ok(Arc::new(descriptors))
    }

    #[inline]
    pub fn handle(&self) -> VkDescriptorSet {
        self.descriptor_set
    }
}

impl Drop for RayTracingDescriptorSets {
    fn drop(&mut self) {
        unsafe {
            let device = &self.device;
            vkDestroyDescriptorPool(device.handle(), self.descriptor_pool, ptr::null());
        }
    }
}

pub struct ShaderBindingTable {
    staging_buffer: Arc<DedicatedStagingBuffer>,
    pipeline: Arc<RayTracingGraphicsPipeline>,
    raygen_entry: VkStridedBufferRegionKHR,
    miss_entry: VkStridedBufferRegionKHR,
    hit_entry: VkStridedBufferRegionKHR,
    callable_entry: VkStridedBufferRegionKHR,
}

impl ShaderBindingTable {
    pub fn new(command_pool: &Arc<CommandPool>, pipeline: &Arc<RayTracingGraphicsPipeline>) -> Result<Arc<Self>> {
        unsafe {
            Self::init(command_pool, pipeline)
        }
    }

    unsafe fn init(command_pool: &Arc<CommandPool>, pipeline: &Arc<RayTracingGraphicsPipeline>) -> Result<Arc<Self>> {
        let device = command_pool.queue().device();
        let properties = device.physical_device().properties_ray_tracing();
        let table_size = (properties.shaderGroupBaseAlignment * 3) as VkDeviceSize;
        let staging_buffer = DedicatedStagingBuffer::new(command_pool, 
            VK_BUFFER_USAGE_TRANSFER_SRC_BIT as VkBufferUsageFlags 
                | VK_BUFFER_USAGE_RAY_TRACING_BIT_KHR as VkBufferUsageFlags,
            VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
            table_size)
            .unwrap();
        staging_buffer.update(table_size, |buffer_data| {
            let buffer_data = buffer_data as *mut u8;
            let data_size = properties.shaderGroupHandleSize as usize * 3;
            let mut data: Vec<u8> = vec![];
            data.resize(data_size, 0);
            vkGetRayTracingShaderGroupHandlesKHR(device.handle(), pipeline.handle(), 0, 3, data_size as size_t, data.as_mut_ptr() as *mut c_void)
                .into_result()
                .unwrap();
            for i in 0..3isize {
                let src_offset = i * properties.shaderGroupHandleSize as isize;
                let dst_offset = i * properties.shaderGroupBaseAlignment as isize;
                let src = data.as_mut_ptr().offset(src_offset);
                let dst = buffer_data.offset(dst_offset);
                std::ptr::copy_nonoverlapping(src, dst, properties.shaderGroupHandleSize as usize);
            }
        });
        // buffer region calculation
        let alignment = properties.shaderGroupBaseAlignment as VkDeviceSize;
        let handle_size = properties.shaderGroupHandleSize as VkDeviceSize;
        let raygen_entry = VkStridedBufferRegionKHR {
            buffer: staging_buffer.device_buffer_memory().buffer(),
            offset: alignment * 0,
            stride: alignment,
            size: handle_size,
        };
        let miss_entry = VkStridedBufferRegionKHR {
            buffer: staging_buffer.device_buffer_memory().buffer(),
            offset: alignment * 1,
            stride: alignment,
            size: handle_size,
        };
        let hit_entry = VkStridedBufferRegionKHR {
            buffer: staging_buffer.device_buffer_memory().buffer(),
            offset: alignment * 2,
            stride: alignment,
            size: handle_size,
        };
        let callable_entry = VkStridedBufferRegionKHR::default();
        let table = Self {
            staging_buffer,
            pipeline: Arc::clone(pipeline),
            raygen_entry,
            miss_entry,
            hit_entry,
            callable_entry,
        };
        Ok(Arc::new(table))
    }

    #[inline]
    fn raygen_entry(&self) -> &VkStridedBufferRegionKHR {
        &self.raygen_entry
    }

    #[inline]
    fn miss_entry(&self) -> &VkStridedBufferRegionKHR {
        &self.miss_entry
    }

    #[inline]
    fn hit_entry(&self) -> &VkStridedBufferRegionKHR {
        &self.hit_entry
    }

    #[inline]
    fn callable_entry(&self) -> &VkStridedBufferRegionKHR {
        &self.callable_entry
    }
}

pub struct RayTracingGraphicsRender {
    command_pool: Arc<CommandPool>,
    pipeline: Arc<RayTracingGraphicsPipeline>,
    descriptor_sets: Arc<RayTracingDescriptorSets>,
    shader_binding_table: Arc<ShaderBindingTable>,
    properties: Arc<VkPhysicalDeviceRayTracingPropertiesKHR>,
}

impl RayTracingGraphicsRender {
    pub fn new(
        command_pool: &Arc<CommandPool>, 
        pipeline: &Arc<RayTracingGraphicsPipeline>,
        descriptor_sets: &Arc<RayTracingDescriptorSets>,
    ) -> Result<Arc<Self>> {
        unsafe {
            Self::init(command_pool, pipeline, descriptor_sets)
        }
    }

    unsafe fn init(
        command_pool: &Arc<CommandPool>, 
        pipeline: &Arc<RayTracingGraphicsPipeline>,
        descriptor_sets: &Arc<RayTracingDescriptorSets>,
    ) -> Result<Arc<Self>> {
        let shader_binding_table = ShaderBindingTable::new(command_pool, pipeline)
            .unwrap();
        let device = command_pool.queue().device();
        let properties = device.physical_device().properties_ray_tracing();
        let render = Self {
            command_pool: Arc::clone(command_pool),
            pipeline: Arc::clone(pipeline),
            descriptor_sets: Arc::clone(descriptor_sets),
            shader_binding_table,
            properties: Arc::new(properties),
        };
        Ok(Arc::new(render))
    }

    pub unsafe fn command(&self, command_buffer: VkCommandBuffer, area: VkRect2D) {
        let device = self.command_pool.queue().device();
        let shader_binding_table = &self.shader_binding_table;
        let descriptor_set = self.descriptor_sets.handle();
        vkCmdBindPipeline(command_buffer, VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR, self.pipeline.handle());
        vkCmdBindDescriptorSets(command_buffer, VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR,
            self.pipeline.layout(), 0, 1, &descriptor_set, 0, ptr::null());
        dispatch_vkCmdTraceRaysKHR(device.handle(), 
            command_buffer, 
            shader_binding_table.raygen_entry(), 
            shader_binding_table.miss_entry(), 
            shader_binding_table.hit_entry(), 
            shader_binding_table.callable_entry(), 
            area.extent.width, 
            area.extent.height, 
            1);
    }
}
