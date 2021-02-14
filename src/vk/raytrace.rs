
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

use VkStructureTypeExtRay::*;
use VkStructureType::*;

pub enum BottomLevelAccelerationStructureGeometry {
    Triangles(BottomLevelAccelerationStructureTriangles),
}

impl BottomLevelAccelerationStructureGeometry {
    pub fn triangles(
        num_vertices: u32,
        vertex_stride: VkDeviceSize,
        vertex_offset_index: u32,
        vertex_buffer_memory: &Arc<DedicatedBufferMemory>,
        num_indices: u32,
        index_offset_index: u32,
        index_buffer_memory: &Arc<DedicatedBufferMemory>,
        is_opaque: bool,
    ) -> Arc<Self> {
        let triangles = BottomLevelAccelerationStructureTriangles::new(
            num_vertices, 
            vertex_stride, 
            vertex_offset_index, 
            vertex_buffer_memory,
            num_indices, 
            index_offset_index, 
            index_buffer_memory, 
            is_opaque
        );
        Arc::new(Self::Triangles(triangles))
    }

    fn geometry(&self) -> VkAccelerationStructureGeometryKHR {
        match &self {
            &Self::Triangles(triangles) => triangles.geometry(),
        }
    }

    fn range_info(&self) -> VkAccelerationStructureBuildRangeInfoKHR {
        match &self {
            &Self::Triangles(triangles) => triangles.range_info(),
        }
    }

    fn max_primitive_count(&self) -> u32 {
        match &self {
            &Self::Triangles(triangles) => triangles.max_primitive_count(),
        }
    }
}

pub struct BottomLevelAccelerationStructureTriangles {
    num_vertices: u32,
    vertex_stride: VkDeviceSize,
    vertex_offset_index: u32,
    num_indices: u32,
    index_offset_index: u32,
    is_opaque: bool,
    vertex_buffer_memory: Arc<DedicatedBufferMemory>,
    index_buffer_memory: Arc<DedicatedBufferMemory>,
}

impl BottomLevelAccelerationStructureTriangles {
    fn new(
        num_vertices: u32,
        vertex_stride: VkDeviceSize,
        vertex_offset_index: u32,
        vertex_buffer_memory: &Arc<DedicatedBufferMemory>,
        num_indices: u32,
        index_offset_index: u32,
        index_buffer_memory: &Arc<DedicatedBufferMemory>,
        is_opaque: bool,
    ) -> Self {
        let triangles = Self {
            vertex_buffer_memory: Arc::clone(vertex_buffer_memory),
            index_buffer_memory: Arc::clone(index_buffer_memory),
            num_vertices,
            vertex_stride,
            vertex_offset_index,
            num_indices,
            index_offset_index,
            is_opaque,
        };
        triangles
    }

    fn geometry(&self) -> VkAccelerationStructureGeometryKHR {
        Self::make_geometry(
            self.num_vertices, 
            self.vertex_stride, 
            &self.vertex_buffer_memory, 
            self.num_indices, 
            &self.index_buffer_memory, 
            self.is_opaque
        )
    }

    fn range_info(&self) -> VkAccelerationStructureBuildRangeInfoKHR {
        let primitive_count = self.num_indices / 3;
        let primitive_offset = self.index_offset_index * std::mem::size_of::<u32>() as u32;
        let range_info = VkAccelerationStructureBuildRangeInfoKHR {
            primitiveCount: primitive_count,
            primitiveOffset: primitive_offset,
            firstVertex: self.vertex_offset_index,
            transformOffset: 0,
        };
        range_info
    }

    fn max_primitive_count(&self) -> u32 {
        self.num_indices / 3
    }

    fn make_geometry(
        num_vertices: u32,
        vertex_stride: VkDeviceSize,
        vertex_buffer_memory: &Arc<DedicatedBufferMemory>,
        num_indices: u32,
        index_buffer_memory: &Arc<DedicatedBufferMemory>,
        is_opaque: bool
    ) -> VkAccelerationStructureGeometryKHR {
        use VkGeometryFlagBitsKHR::*;
        let triangles = VkAccelerationStructureGeometryTrianglesDataKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR,
            pNext: ptr::null(),
            vertexFormat: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
            vertexData: VkDeviceOrHostAddressConstKHR {
                deviceAddress: vertex_buffer_memory.buffer_device_address(),
            },
            vertexStride: vertex_stride,
            // maxVertex is the highest index of a vertex that will be addressed 
            // by a build command using this structure.
            maxVertex: num_vertices,
            indexType: VkIndexType::VK_INDEX_TYPE_UINT32,
            indexData: VkDeviceOrHostAddressConstKHR {
                deviceAddress: index_buffer_memory.buffer_device_address(),
            },
            transformData: VkDeviceOrHostAddressConstKHR {
                deviceAddress: 0,
            },
        };
        let geometry_flags: VkGeometryFlagBitsKHR = if is_opaque { 
            VK_GEOMETRY_OPAQUE_BIT_KHR 
        } else { 
            VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR 
        };
        VkAccelerationStructureGeometryKHR {
            sType: VkStructureTypeExtRay::VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR,
            pNext: ptr::null(),
            geometryType: VkGeometryTypeKHR::VK_GEOMETRY_TYPE_TRIANGLES_KHR,
            geometry: VkAccelerationStructureGeometryDataKHR {
                triangles: triangles,
            },
            flags: geometry_flags as VkGeometryFlagsKHR,
        }
    }
}

struct AccelerationStructureBufferMemory {
    device: Arc<Device>,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
}

impl AccelerationStructureBufferMemory {
    fn new(device: &Arc<Device>, structure_size: VkDeviceSize) -> Arc<Self> {
        unsafe {
            // creates buffer
            let mut buffer = MaybeUninit::<VkBuffer>::zeroed();
            {
                use VkBufferUsageFlagBits::*;
                let buffer_create_info = VkBufferCreateInfo::new(
                    structure_size, 
                    VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR as VkBufferUsageFlags
                        | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags,
                    VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
                );
                vkCreateBuffer(device.handle(), &buffer_create_info, ptr::null(), buffer.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let buffer = buffer.assume_init();
            // creates memory
            let mut memory = MaybeUninit::<VkDeviceMemory>::zeroed();
            {
                use VkMemoryAllocateFlagBits::*;
                let mut requirements = MaybeUninit::<VkMemoryRequirements>::zeroed();
                vkGetBufferMemoryRequirements(device.handle(), buffer, requirements.as_mut_ptr());
                let requirements = requirements.assume_init();
                let flags_info = VkMemoryAllocateFlagsInfo {
                    sType: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO,
                    pNext: ptr::null(),
                    flags: VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT as VkMemoryAllocateFlags,
                    deviceMask: 0,
                };
                // physical memory properties
                use VkMemoryPropertyFlagBits::*;
                let memory_type_index = device.physical_device()
                    .memory_type_index(&requirements, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags)
                    .unwrap();
                let allocate_info = VkMemoryAllocateInfo {
                    sType: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
                    pNext: &flags_info as *const _ as *const c_void,
                    allocationSize: requirements.size,
                    memoryTypeIndex: memory_type_index,
                };
                vkAllocateMemory(device.handle(), &allocate_info, ptr::null(), memory.as_mut_ptr())
                    .into_result()
                    .unwrap();
            }
            let memory = memory.assume_init();
            vkBindBufferMemory(device.handle(), buffer, memory, 0);
            let buffer_memory = Self {
                device: Arc::clone(device),
                buffer,
                memory,
            };
            Arc::new(buffer_memory)
        }
    }

    fn buffer(&self) -> VkBuffer {
        self.buffer
    }
}

impl Drop for AccelerationStructureBufferMemory {
    fn drop(&mut self) {
        unsafe {
            vkFreeMemory(self.device.handle(), self.memory, ptr::null());
            vkDestroyBuffer(self.device.handle(), self.buffer, ptr::null());
        }
    }
}

// represents an internal acceleration structure both in and after build
struct AccelerationStructure {
    handle: VkAccelerationStructureKHR,
    buffer_memory: Arc<AccelerationStructureBufferMemory>,
    device: Arc<Device>,
}

impl AccelerationStructure {
    fn new(
        device: &Arc<Device>,
        structure_size: VkDeviceSize, 
        structure_type: VkAccelerationStructureTypeKHR,
    ) -> Arc<Self> {
        unsafe {
            let buffer_memory = AccelerationStructureBufferMemory::new(device, structure_size);
            let create_info = VkAccelerationStructureCreateInfoKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                createFlags: 0,
                buffer: buffer_memory.buffer(),
                offset: 0,
                size: structure_size,
                r#type: structure_type,
                deviceAddress: 0,
            };
            let mut handle = MaybeUninit::<VkAccelerationStructureKHR>::zeroed();
            vkCreateAccelerationStructureKHR(device.handle(), &create_info, ptr::null(), handle.as_mut_ptr())
                .into_result()
                .unwrap();
            let handle = handle.assume_init();
            let structure = Self {
                handle,
                buffer_memory,
                device: Arc::clone(device),
            };
            Arc::new(structure)
        }
    }

    fn handle(&self) -> VkAccelerationStructureKHR {
        self.handle
    }

    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl Drop for AccelerationStructure {
    fn drop(&mut self) {
        vkDestroyAccelerationStructureKHR(self.device.handle(), self.handle, ptr::null());
    }
}

// represents the method to build a structure
pub struct BottomLevelAccelerationStructureBuildQuery {
    geometries: Vec<Arc<BottomLevelAccelerationStructureGeometry>>,
}

impl BottomLevelAccelerationStructureBuildQuery {
    pub fn new(
        geometries: Vec<Arc<BottomLevelAccelerationStructureGeometry>>,
    ) -> Self {
        Self {
            geometries,
        }
    }

    fn build(self, device: &Arc<Device>) -> BottomLevelAccelerationStructureBuild {
        BottomLevelAccelerationStructureBuild::new(device, self.geometries)
    }
}

struct BottomLevelAccelerationStructureBuild {
    device: Arc<Device>,
    geometries: Vec<Arc<BottomLevelAccelerationStructureGeometry>>,
    sizes_info: VkAccelerationStructureBuildSizesInfoKHR,
    geometries_vec: Vec<VkAccelerationStructureGeometryKHR>,
    max_primitive_count_vec: Vec<u32>,
}

impl BottomLevelAccelerationStructureBuild {
    fn new(
        device: &Arc<Device>,
        geometries: Vec<Arc<BottomLevelAccelerationStructureGeometry>>,
    ) -> Self {
        unsafe {
            let geometries_vec: Vec<VkAccelerationStructureGeometryKHR> = geometries.iter()
                .map(|v| v.geometry())
                .collect();
            // pMaxPrimitiveCounts is a pointer to an array of pBuildInfo->geometryCount uint32_t values 
            // defining the number of primitives built into each geometry.
            let max_primitive_count_vec: Vec<u32> = geometries.iter()
                .map(|v| v.max_primitive_count())
                .collect();
            let sizes_info = Self::make_sizes_info(device, &geometries_vec, &max_primitive_count_vec);
            Self {
                device: Arc::clone(device),
                geometries,
                sizes_info,
                geometries_vec,
                max_primitive_count_vec,
            }
        }
    }

    fn scratch_size(&self) -> VkDeviceSize {
        self.sizes_info.buildScratchSize
    }

    unsafe fn begin(self,
        recording: &CommandBufferRecording,
        scratch_buffer_memory: &Arc<DedicatedBufferMemory>,
    ) -> BottomLevelAccelerationStructureBuildProcess {
        use VkAccelerationStructureTypeKHR::*;
        use VkBuildAccelerationStructureFlagBitsKHR::*;
        use VkBuildAccelerationStructureModeKHR::*;
        use VkAccessFlagBits::*;
        use VkPipelineStageFlagBits::*;
        let geometries = self.geometries;
        let geometries_vec = self.geometries_vec;
        let sizes_info = self.sizes_info;
        let device = recording.command_pool().queue().device();
        let structure = AccelerationStructure::new(
            device, 
            sizes_info.accelerationStructureSize, 
            VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR,
        );
        let build_info = VkAccelerationStructureBuildGeometryInfoKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
            pNext: ptr::null(),
            r#type: VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR,
            flags: VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR as VkBuildAccelerationStructureFlagsKHR,
            mode: VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR,
            srcAccelerationStructure: ptr::null_mut(),
            dstAccelerationStructure: structure.handle(),
            geometryCount: geometries_vec.len() as u32,
            pGeometries: geometries_vec.as_ptr(),
            ppGeometries: ptr::null(),
            scratchData: VkDeviceOrHostAddressKHR {
                deviceAddress: scratch_buffer_memory.buffer_device_address(),
            },
        };
        let range_info_vec: Vec<VkAccelerationStructureBuildRangeInfoKHR> = geometries.iter()
            .map(|v| v.range_info())
            .collect();
        let range_info_vec_vec: Vec<*const VkAccelerationStructureBuildRangeInfoKHR> = vec![range_info_vec.as_ptr()];
        dispatch_vkCmdBuildAccelerationStructuresKHR(
            device.handle(), 
            recording.command_buffer(),
            1,
            &build_info,
            range_info_vec_vec.as_ptr(),
        );
        // memory barrier allowing reuse of scratch across builds
        let memory_barrier = VkMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR as VkAccessFlags,
            dstAccessMask: VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR as VkAccessFlags,
        };
        vkCmdPipelineBarrier(
            recording.command_buffer(), 
            VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR as VkPipelineStageFlags,
            VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR as VkPipelineStageFlags, 
            0, 
            1, &memory_barrier,
            0, ptr::null(),
            0, ptr::null(),
        );
        BottomLevelAccelerationStructureBuildProcess::new(
            geometries, 
            structure, 
            scratch_buffer_memory
        )
    }

    unsafe fn make_sizes_info(
        device: &Arc<Device>,
        geometries_vec: &Vec<VkAccelerationStructureGeometryKHR>, 
        max_primitive_count_vec: &Vec<u32>
    ) -> VkAccelerationStructureBuildSizesInfoKHR {
        use VkAccelerationStructureTypeKHR::*;
        use VkBuildAccelerationStructureFlagBitsKHR::*;
        use VkBuildAccelerationStructureModeKHR::*;
        use VkAccelerationStructureBuildTypeKHR::*;
        let build_info = VkAccelerationStructureBuildGeometryInfoKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
            pNext: ptr::null(),
            r#type: VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR,
            flags: VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR as VkBuildAccelerationStructureFlagsKHR,
            mode: VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR,
            srcAccelerationStructure: ptr::null_mut(),
            dstAccelerationStructure: ptr::null_mut(),
            geometryCount: geometries_vec.len() as u32,
            pGeometries: geometries_vec.as_ptr(),
            ppGeometries: ptr::null(),
            scratchData: VkDeviceOrHostAddressKHR {
                deviceAddress: 0,
            },
        };
        let mut sizes_info = MaybeUninit::<VkAccelerationStructureBuildSizesInfoKHR>::zeroed();
        {
            let sizes_info = &mut *sizes_info.as_mut_ptr();
            sizes_info.sType = VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR;
        }
        vkGetAccelerationStructureBuildSizesKHR(
            device.handle(), 
            VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR,
            &build_info,
            max_primitive_count_vec.as_ptr(),
            sizes_info.as_mut_ptr(),
        );
        let sizes_info = sizes_info.assume_init();
        sizes_info
    }
}

struct BottomLevelAccelerationStructureBuildProcess {
    geometries: Vec<Arc<BottomLevelAccelerationStructureGeometry>>,
    structure: Arc<AccelerationStructure>,
    scratch: Arc<DedicatedBufferMemory>,
}

impl BottomLevelAccelerationStructureBuildProcess {
    fn new(
        geometries: Vec<Arc<BottomLevelAccelerationStructureGeometry>>,
        structure: Arc<AccelerationStructure>,
        scratch: &Arc<DedicatedBufferMemory>,
    ) -> Self {
        let building = Self {
            geometries,
            structure,
            scratch: Arc::clone(scratch),
        };
        building
    }

    fn complete(self) -> Arc<BottomLevelAccelerationStructure> {
        BottomLevelAccelerationStructure::new(self.geometries, self.structure)
    }
}

// represents a complete built structure
pub struct BottomLevelAccelerationStructure {
    geometries: Vec<Arc<BottomLevelAccelerationStructureGeometry>>,
    structure: Arc<AccelerationStructure>,
    device_address: VkDeviceAddress,
}

impl BottomLevelAccelerationStructure {
    fn new(
        geometries: Vec<Arc<BottomLevelAccelerationStructureGeometry>>,
        structure: Arc<AccelerationStructure>,
    ) -> Arc<Self> {
        let device = structure.device();
        // device address is unknown until its build process completes
        let device_address = {
            let info = VkAccelerationStructureDeviceAddressInfoKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
                pNext: ptr::null(),
                accelerationStructure: structure.handle,
            };
            vkGetAccelerationStructureDeviceAddressKHR(device.handle(), &info)
        };
        let structure = Self {
            geometries,
            structure,
            device_address,
        };
        Arc::new(structure)
    }

    #[inline]
    pub fn handle(&self) -> VkAccelerationStructureKHR {
        self.structure.handle()
    }

    pub fn device_address(&self) -> VkDeviceAddress {
        self.device_address
    }
}

pub struct BottomLevelAccelerationStructuresBuilder<'a> {
    command_pool: &'a Arc<CommandPool>, 
    queries: Vec<BottomLevelAccelerationStructureBuildQuery>,
}

impl<'a> BottomLevelAccelerationStructuresBuilder<'a> {
    // creates bulk of structures
    pub fn new(
        command_pool: &'a Arc<CommandPool>, 
        queries: Vec<BottomLevelAccelerationStructureBuildQuery>,
    ) -> Self {
        Self {
            command_pool,
            queries,
        }
    }

    pub fn build(self) -> Vec<Arc<BottomLevelAccelerationStructure>> {
        use VkBufferUsageFlagBits::*;
        use VkMemoryPropertyFlagBits::*;
        let command_pool = self.command_pool;
        let queries = self.queries;
        let device = command_pool.queue().device();
        unsafe {
            let builds: Vec<_> = queries.into_iter()
                .map(|v| v.build(device))
                .collect();
            // estimate required scratch size
            let build_scratch_size = builds.iter()
                .map(|v| v.scratch_size())
                .max()
                .unwrap();
            // creates shared scratch memory
            let scratch_buffer_memory = DedicatedBufferMemory::new(
                device, 
                VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as VkBufferUsageFlags
                    | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags, 
                VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
                build_scratch_size,
            )
                .unwrap();
            // begin builds
            let recording = CommandBufferRecording::new_onetime_submit(command_pool)
                .unwrap();
            let processes: Vec<_> = builds.into_iter()
                .map(|v| v.begin(&recording, &scratch_buffer_memory))
                .collect();
            let command_buffer = recording.complete();
            // wait until all commands complete
            command_pool.queue()
                .submit_then_wait(&[command_buffer.handle()])
                .unwrap();
            // discards scratch buffers once it completes
            let blas_vec: Vec<_> = processes.into_iter()
                .map(|v| v.complete())
                .collect();
            blas_vec
        }
    }
}



pub struct TopLevelAccelerationStructureInstance {
    instance_custom_index: u32,
    transform: VkTransformMatrixKHR,
    hit_group: u32,
    bottom_level_acceleration_structure: Arc<BottomLevelAccelerationStructure>,
}

impl TopLevelAccelerationStructureInstance {
    pub fn new(
        instance_custom_index: u32, 
        transform: VkTransformMatrixKHR,
        hit_group: u32,
        bottom_level_acceleration_structure: &Arc<BottomLevelAccelerationStructure>,
    ) -> Result<Arc<Self>> {
        let instance = Self {
            instance_custom_index,
            transform,
            hit_group,
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

    #[inline]
    fn hit_group(&self) -> u32 {
        self.hit_group
    }

    fn instance_struct(&self) -> VkAccelerationStructureInstanceKHR {
        use VkGeometryInstanceFlagBitsKHR::*;
        VkAccelerationStructureInstanceKHR {
            transform: self.transform.clone(),
            instanceCustomIndexAndMask: (0xff << 24) | (self.instance_custom_index() & ((1u32 << 25) - 1)),
            instanceShaderBindingTableRecordOffsetAndFlags: 
                ((VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR as VkFlags) << 24)
                    | (self.hit_group() & 0xff),
            accelerationStructureReference: self.bottom_level_acceleration_structure().device_address(),
        }
    }
}

pub struct TopLevelAccelerationStructure {
    instances_buffer: Arc<DedicatedStagingBuffer>,
    structure: Arc<AccelerationStructure>,
    instances: Vec<Arc<TopLevelAccelerationStructureInstance>>,
    device_address: VkDeviceAddress,
}

impl TopLevelAccelerationStructure {
    pub fn new(
        command_pool: &Arc<CommandPool>, 
        instances: Vec<Arc<TopLevelAccelerationStructureInstance>>,
    ) -> Result<Arc<Self>> {
        use VkBufferUsageFlagBits::*;
        use VkMemoryPropertyFlagBits::*;
        use VkGeometryFlagBitsKHR::*;
        use VkAccelerationStructureTypeKHR::*;
        use VkBuildAccelerationStructureFlagBitsKHR::*;
        use VkBuildAccelerationStructureModeKHR::*;
        unsafe {
            // sends instance structs to the GPU
            let instance_structs: Vec<VkAccelerationStructureInstanceKHR> = instances.iter()
                .map(|v| v.instance_struct())
                .collect();
            let instances_size = instance_structs.len() * std::mem::size_of::<VkAccelerationStructureInstanceKHR>();
            let instances_buffer = DedicatedStagingBuffer::new(
                command_pool, 
                VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags
                    | VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR as VkBufferUsageFlags, 
                VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
                instances_size as VkDeviceSize,
            )
                .unwrap();
            // TODO: optimize (write blocks)
            instances_buffer.write(instance_structs.as_ptr() as *const c_void, instances_size);
            // build info construction
            let geometry = VkAccelerationStructureGeometryKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR,
                pNext: ptr::null(),
                geometryType: VkGeometryTypeKHR::VK_GEOMETRY_TYPE_INSTANCES_KHR,
                geometry: VkAccelerationStructureGeometryDataKHR {
                    instances: VkAccelerationStructureGeometryInstancesDataKHR {
                        sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR,
                        pNext: ptr::null(),
                        arrayOfPointers: VK_FALSE,
                        data: VkDeviceOrHostAddressConstKHR {
                            deviceAddress: instances_buffer.device_buffer_memory().buffer_device_address(),
                        },
                    },
                },
                flags: VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR as VkGeometryFlagsKHR,
            };
            let device = command_pool.queue().device();
            let geometries_vec = vec![geometry];
            let max_primitive_count_vec = vec![instance_structs.len() as u32];
            let sizes_info = Self::make_sizes_info(device, &geometries_vec, &max_primitive_count_vec);
            let structure = AccelerationStructure::new(
                device, 
                sizes_info.accelerationStructureSize, 
                VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR,
            );
            let scratch_buffer_memory = DedicatedBufferMemory::new(
                device, 
                VK_BUFFER_USAGE_STORAGE_BUFFER_BIT as VkBufferUsageFlags
                    | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags, 
                VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags,
                sizes_info.buildScratchSize,
            )
                .unwrap();
            let build_info = VkAccelerationStructureBuildGeometryInfoKHR {
                sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
                pNext: ptr::null(),
                r#type: VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR,
                flags: VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR as VkBuildAccelerationStructureFlagsKHR,
                mode: VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR,
                srcAccelerationStructure: ptr::null_mut(),
                dstAccelerationStructure: structure.handle(),
                geometryCount: geometries_vec.len() as u32,
                pGeometries: geometries_vec.as_ptr(),
                ppGeometries: ptr::null(),
                scratchData: VkDeviceOrHostAddressKHR {
                    deviceAddress: scratch_buffer_memory.buffer_device_address(),
                },
            };
            let range_info = VkAccelerationStructureBuildRangeInfoKHR {
                primitiveCount: instance_structs.len() as u32,
                primitiveOffset: 0,
                firstVertex: 0,
                transformOffset: 0,
            };
            let range_info_vec = vec![range_info];
            let range_info_vec_vec: Vec<*const VkAccelerationStructureBuildRangeInfoKHR> = vec![range_info_vec.as_ptr()];
            // command dispatch
            let recording = CommandBufferRecording::new_onetime_submit(command_pool)
                .unwrap();
            dispatch_vkCmdBuildAccelerationStructuresKHR(
                device.handle(), 
                recording.command_buffer(),
                1,
                &build_info,
                range_info_vec_vec.as_ptr(),
            );
            let command_buffer = recording.complete();
            let command_buffer_vec = vec![command_buffer.handle()];
            command_pool.queue()
                .submit_then_wait(&command_buffer_vec)
                .unwrap();
            // device address is unknown until its build process completes
            let device_address = {
                let info = VkAccelerationStructureDeviceAddressInfoKHR {
                    sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
                    pNext: ptr::null(),
                    accelerationStructure: structure.handle,
                };
                vkGetAccelerationStructureDeviceAddressKHR(device.handle(), &info)
            };
            let structure = Self {
                instances_buffer,
                instances,
                structure,
                device_address,
            };
            Ok(Arc::new(structure))
        }
    }

    #[inline]
    pub fn handle(&self) -> VkAccelerationStructureKHR {
        self.structure.handle()
    }

    unsafe fn make_sizes_info(
        device: &Arc<Device>,
        geometries_vec: &Vec<VkAccelerationStructureGeometryKHR>, 
        max_primitive_count_vec: &Vec<u32>,
    ) -> VkAccelerationStructureBuildSizesInfoKHR {
        use VkAccelerationStructureTypeKHR::*;
        use VkBuildAccelerationStructureFlagBitsKHR::*;
        use VkBuildAccelerationStructureModeKHR::*;
        use VkAccelerationStructureBuildTypeKHR::*;
        let build_info = VkAccelerationStructureBuildGeometryInfoKHR {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
            pNext: ptr::null(),
            r#type: VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR,
            flags: VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR as VkBuildAccelerationStructureFlagsKHR,
            mode: VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR,
            srcAccelerationStructure: ptr::null_mut(),
            dstAccelerationStructure: ptr::null_mut(),
            geometryCount: geometries_vec.len() as u32,
            pGeometries: geometries_vec.as_ptr(),
            ppGeometries: ptr::null(),
            scratchData: VkDeviceOrHostAddressKHR {
                deviceAddress: 0,
            },
        };
        let mut sizes_info = MaybeUninit::<VkAccelerationStructureBuildSizesInfoKHR>::zeroed();
        {
            let sizes_info = &mut *sizes_info.as_mut_ptr();
            sizes_info.sType = VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR;
        }
        vkGetAccelerationStructureBuildSizesKHR(
            device.handle(), 
            VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR,
            &build_info,
            max_primitive_count_vec.as_ptr(),
            sizes_info.as_mut_ptr(),
        );
        let sizes_info = sizes_info.assume_init();
        sizes_info
    }
}




pub struct RayTracingGraphicsPipeline {
    device: Arc<Device>,
    layout: VkPipelineLayout,
    handle: VkPipeline,
    descriptor_set_layout: VkDescriptorSetLayout,
    shader_group_count: u32,
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
                    VkShaderStageFlagBits::VK_SHADER_STAGE_RAYGEN_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32,
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
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR as u32,
                    3,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR as u32,
                    4,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR as u32,
                    5,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR as u32,
                    6,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR as u32,
                    7,
                ),
                VkDescriptorSetLayoutBinding::new_array(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR as u32,
                    8,
                    textures_count,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32 
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_INTERSECTION_BIT_KHR as u32,
                    9,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR as u32,
                    10,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR as u32,
                    11,
                ),
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR as u32
                        | VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR as u32,
                    12,
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
        let raygen_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/ray.rgen.spv")).unwrap();
        let rmiss_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/ray.rmiss.spv")).unwrap();
        let shadow_rmiss_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/ray.shadow.rmiss.spv")).unwrap();
        let triangles_rchit_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/ray.triangles.rchit.spv")).unwrap();
        let triangles_rahit_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/ray.triangles.rahit.spv")).unwrap();
        let procedural_rint_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/ray.procedural.rint.spv")).unwrap();
        let procedural_rchit_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/ray.procedural.rchit.spv")).unwrap();
        let shader_entry_point = CString::new("main").unwrap();
        const INDEX_RAYGEN: u32 = 0;
        const INDEX_MISS: u32 = 1;
        const INDEX_SHADOW_MISS: u32 = 2;
        const INDEX_TRIANGLES_CLOSEST_HIT: u32 = 3;
        const INDEX_TRIANGLES_ANY_HIT: u32 = 4;
        const INDEX_PROCEDURAL_INTERSECTION: u32 = 5;
        const INDEX_PROCEDURAL_CLOSEST_HIT: u32 = 6;
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
                stage: VkShaderStageFlagBits::VK_SHADER_STAGE_MISS_BIT_KHR,
                module: shadow_rmiss_shader_module.handle(),
                pName: shader_entry_point.as_ptr(),
                pSpecializationInfo: ptr::null(),
            },
            VkPipelineShaderStageCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                stage: VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR,
                module: triangles_rchit_shader_module.handle(),
                pName: shader_entry_point.as_ptr(),
                pSpecializationInfo: ptr::null(),
            },
            VkPipelineShaderStageCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                stage: VkShaderStageFlagBits::VK_SHADER_STAGE_ANY_HIT_BIT_KHR,
                module: triangles_rahit_shader_module.handle(),
                pName: shader_entry_point.as_ptr(),
                pSpecializationInfo: ptr::null(),
            },
            VkPipelineShaderStageCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                stage: VkShaderStageFlagBits::VK_SHADER_STAGE_INTERSECTION_BIT_KHR,
                module: procedural_rint_shader_module.handle(),
                pName: shader_entry_point.as_ptr(),
                pSpecializationInfo: ptr::null(),
            },
            VkPipelineShaderStageCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                stage: VkShaderStageFlagBits::VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR,
                module: procedural_rchit_shader_module.handle(),
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
                r#type: VkRayTracingShaderGroupTypeKHR::VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR,
                generalShader: INDEX_SHADOW_MISS,
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
                closestHitShader: INDEX_TRIANGLES_CLOSEST_HIT,
                anyHitShader: INDEX_TRIANGLES_ANY_HIT,
                intersectionShader: VK_SHADER_UNUSED_KHR,
                pShaderGroupCaptureReplayHandle: ptr::null(),
            },
            VkRayTracingShaderGroupCreateInfoKHR {
                sType: VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR,
                pNext: ptr::null(),
                r#type: VkRayTracingShaderGroupTypeKHR::VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR,
                generalShader: VK_SHADER_UNUSED_KHR,
                closestHitShader: INDEX_PROCEDURAL_CLOSEST_HIT,
                anyHitShader: VK_SHADER_UNUSED_KHR,
                intersectionShader: INDEX_PROCEDURAL_INTERSECTION,
                pShaderGroupCaptureReplayHandle: ptr::null(),
            },
        ];
        // allows casting a shadow ray from the closest hit shader
        let max_recursion_depth = 2;
        let create_info = VkRayTracingPipelineCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: 0,
            stageCount: shader_stages.len() as u32,
            pStages: shader_stages.as_ptr(),
            groupCount: shader_groups.len() as u32,
            pGroups: shader_groups.as_ptr(),
            maxPipelineRayRecursionDepth: max_recursion_depth,
            pLibraryInfo: ptr::null(),
            pLibraryInterface: ptr::null(),
            pDynamicState: ptr::null(),
            layout: pipeline_layout,
            basePipelineHandle: ptr::null_mut(),
            basePipelineIndex: 0,
        };
        let mut handle = MaybeUninit::<VkPipeline>::zeroed();
        vkCreateRayTracingPipelinesKHR(device.handle(), ptr::null_mut(), ptr::null_mut(), 1, &create_info, ptr::null(), handle.as_mut_ptr())
            .into_result()
            .unwrap();
        let handle = handle.assume_init();
        let layout = RayTracingGraphicsPipeline {
            device: Arc::clone(device),
            layout: pipeline_layout,
            handle,
            descriptor_set_layout,
            shader_group_count: shader_groups.len() as u32,
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

    pub fn shader_group_count(&self) -> u32 {
        self.shader_group_count
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
    sphere_storage_buffer: Arc<DedicatedStagingBuffer>,
    material_storage_buffer: Arc<DedicatedStagingBuffer>,
    material_description_storage_buffer: Arc<DedicatedStagingBuffer>,
    tangent_storage_buffer: Arc<DedicatedStagingBuffer>,
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
        sphere_storage_buffer: &Arc<DedicatedStagingBuffer>,
        material_storage_buffer: &Arc<DedicatedStagingBuffer>,
        material_description_storage_buffer: &Arc<DedicatedStagingBuffer>,
        tangent_storage_buffer: &Arc<DedicatedStagingBuffer>,
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
                sphere_storage_buffer,
                material_storage_buffer,
                material_description_storage_buffer,
                tangent_storage_buffer,
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
        sphere_storage_buffer: &Arc<DedicatedStagingBuffer>,
        material_storage_buffer: &Arc<DedicatedStagingBuffer>,
        material_description_storage_buffer: &Arc<DedicatedStagingBuffer>,
        tangent_storage_buffer: &Arc<DedicatedStagingBuffer>,
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
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1),
                VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1),
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
        let write_sphere_buffer_info = VkDescriptorBufferInfo {
            buffer: sphere_storage_buffer.device_buffer_memory().buffer(),
            offset: 0,
            range: sphere_storage_buffer.device_buffer_memory().size(),
        };
        let write_sphere_buffer = VkWriteDescriptorSet::from_buffer(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            9,
            &write_sphere_buffer_info);
        let write_material_buffer_info = VkDescriptorBufferInfo {
            buffer: material_storage_buffer.device_buffer_memory().buffer(),
            offset: 0,
            range: material_storage_buffer.device_buffer_memory().size(),
        };
        let write_material_buffer = VkWriteDescriptorSet::from_buffer(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            10,
            &write_material_buffer_info);
        let write_material_description_buffer_info = VkDescriptorBufferInfo {
            buffer: material_description_storage_buffer.device_buffer_memory().buffer(),
            offset: 0,
            range: material_description_storage_buffer.device_buffer_memory().size(),
        };
        let write_material_description_buffer = VkWriteDescriptorSet::from_buffer(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            11,
            &write_material_description_buffer_info);
        let write_tangent_buffer_info = VkDescriptorBufferInfo {
            buffer: tangent_storage_buffer.device_buffer_memory().buffer(),
            offset: 0,
            range: tangent_storage_buffer.device_buffer_memory().size(),
        };
        let write_tangent_buffer = VkWriteDescriptorSet::from_buffer(descriptor_set, 
            VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            12,
            &write_tangent_buffer_info);
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
            write_sphere_buffer,
            write_material_buffer,
            write_material_description_buffer,
            write_tangent_buffer,
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
            sphere_storage_buffer: Arc::clone(sphere_storage_buffer),
            material_storage_buffer: Arc::clone(material_storage_buffer),
            material_description_storage_buffer: Arc::clone(material_description_storage_buffer),
            tangent_storage_buffer: Arc::clone(tangent_storage_buffer),
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
    storage_buffer: Arc<DedicatedStagingBuffer>,
    pipeline: Arc<RayTracingGraphicsPipeline>,
    raygen_entry: VkStridedDeviceAddressRegionKHR,
    miss_entry: VkStridedDeviceAddressRegionKHR,
    hit_entry: VkStridedDeviceAddressRegionKHR,
    callable_entry: VkStridedDeviceAddressRegionKHR,
}

impl ShaderBindingTable {
    pub fn new(command_pool: &Arc<CommandPool>, pipeline: &Arc<RayTracingGraphicsPipeline>) -> Result<Arc<Self>> {
        unsafe {
            Self::init(command_pool, pipeline)
        }
    }

    unsafe fn init(command_pool: &Arc<CommandPool>, pipeline: &Arc<RayTracingGraphicsPipeline>) -> Result<Arc<Self>> {
        use VkBufferUsageFlagBits::*;
        use VkMemoryPropertyFlagBits::*;
        let device = command_pool.queue().device();
        // | RAYGEN |
        // | MISS |
        // | MISS (SHADOW) |
        // | HIT (TRIANGLE) |
        // | HIT (PROCEDURAL) |
        let group_count = pipeline.shader_group_count();
        let properties = device.physical_device().properties_ray_tracing();
        let group_handle_size = properties.shaderGroupHandleSize;
        let group_size_aligned = Self::aligned_size(properties.shaderGroupHandleSize, properties.shaderGroupBaseAlignment);
        let table_size = (group_size_aligned * group_count) as VkDeviceSize;
        let buffer_flags = VK_BUFFER_USAGE_TRANSFER_SRC_BIT as VkBufferUsageFlags 
            | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT as VkBufferUsageFlags
            | VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR as VkBufferUsageFlags;
        let memory_flags = VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as VkMemoryPropertyFlags;
        let storage_buffer = DedicatedStagingBuffer::new(
            command_pool, 
            buffer_flags,
            memory_flags,
            table_size,
        ).unwrap();
        storage_buffer.update(table_size, |buffer_data| {
            let buffer_data = buffer_data as *mut u8;
            let mut data: Vec<u8> = vec![];
            let data_size = group_handle_size * group_count;
            data.resize(data_size as usize, 0);
            vkGetRayTracingShaderGroupHandlesKHR(
                device.handle(), 
                pipeline.handle(), 
                0,
                group_count,
                data_size as size_t, 
                data.as_mut_ptr() as *mut c_void,
            )
                .into_result()
                .unwrap();
            for i in 0..(group_count as isize) {
                let src_offset = i * group_handle_size as isize;
                let dst_offset = i * group_size_aligned as isize;
                let src = data.as_mut_ptr().offset(src_offset);
                let dst = buffer_data.offset(dst_offset);
                ptr::copy_nonoverlapping(src, dst, group_handle_size as usize);
            }
        });
        // buffer region calculation
        let group_size = group_size_aligned as VkDeviceSize;
        let base_device_address = storage_buffer.device_buffer_memory().buffer_device_address();
        let raygen_entry = VkStridedDeviceAddressRegionKHR {
            deviceAddress: base_device_address + 0 * group_size,
            stride: group_size,
            size: group_size,
        };
        let miss_entry = VkStridedDeviceAddressRegionKHR {
            deviceAddress: base_device_address + 1 * group_size,
            stride: group_size,
            size: group_size * 2,
        };
        let hit_entry = VkStridedDeviceAddressRegionKHR {
            deviceAddress: base_device_address + 3 * group_size,
            stride: group_size,
            size: group_size * 2,
        };
        let callable_entry = VkStridedDeviceAddressRegionKHR::default();
        let table = Self {
            storage_buffer,
            pipeline: Arc::clone(pipeline),
            raygen_entry,
            miss_entry,
            hit_entry,
            callable_entry,
        };
        Ok(Arc::new(table))
    }

    #[inline]
    fn raygen_entry(&self) -> &VkStridedDeviceAddressRegionKHR {
        &self.raygen_entry
    }

    #[inline]
    fn miss_entry(&self) -> &VkStridedDeviceAddressRegionKHR {
        &self.miss_entry
    }

    #[inline]
    fn hit_entry(&self) -> &VkStridedDeviceAddressRegionKHR {
        &self.hit_entry
    }

    #[inline]
    fn callable_entry(&self) -> &VkStridedDeviceAddressRegionKHR {
        &self.callable_entry
    }

    // @see https://nvpro-samples.github.io/vk_raytracing_tutorial_KHR/
    // Size and Alignment Gotcha
    // alignedSize=[size+(alignment−1)] & ~(alignment−1)
    fn aligned_size(size: u32, alignment: u32) -> u32 {
        (size + alignment - 1) & !(alignment - 1)
    }
}

pub struct RayTracingGraphicsRender {
    command_pool: Arc<CommandPool>,
    pipeline: Arc<RayTracingGraphicsPipeline>,
    descriptor_sets: Arc<RayTracingDescriptorSets>,
    shader_binding_table: Arc<ShaderBindingTable>,
    properties: Arc<VkPhysicalDeviceRayTracingPipelinePropertiesKHR>,
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
