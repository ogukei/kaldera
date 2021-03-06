
// Provided by VK_KHR_ray_tracing

#![allow(dead_code)]
#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_void, c_char, size_t, c_float};

use super::types::*;
use super::types_ext::*;

pub type VkAccelerationStructureCreateFlagsKHR = VkFlags;
pub type VkBuildAccelerationStructureFlagsKHR = VkFlags;
pub type VkGeometryFlagsKHR = VkFlags;

#[repr(C)]
pub struct VkAccelerationStructureKHROpaque { _private: [u8; 0] }
pub type VkAccelerationStructureKHR = *mut VkAccelerationStructureKHROpaque;

#[repr(C)]
pub struct VkDeferredOperationKHROpaque { _private: [u8; 0] }
pub type VkDeferredOperationKHR = *mut VkDeferredOperationKHROpaque;


pub const VK_SHADER_UNUSED_KHR: u32 = !0u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum VkStructureTypeExtRay {
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR = 1000150007,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR = 1000150000,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR = 1000150002,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR = 1000150003,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR = 1000150004,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR = 1000150005,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR = 1000150006,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR = 1000150009,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR = 1000150010,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR = 1000150011,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR = 1000150012,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR = 1000150013,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR = 1000150014,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR = 1000150017,
    // Provided by VK_KHR_acceleration_structure
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR = 1000150020,
    // Provided by VK_KHR_ray_tracing_pipeline
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR = 1000347000,
    // Provided by VK_KHR_ray_tracing_pipeline
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR = 1000347001,
    // Provided by VK_KHR_ray_tracing_pipeline
    VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR = 1000150015,
    // Provided by VK_KHR_ray_tracing_pipeline
    VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR = 1000150016,
    // Provided by VK_KHR_ray_tracing_pipeline
    VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR = 1000150018,
    // Provided by VK_KHR_ray_query
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR = 1000348013,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html
#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *mut c_void,
    pub shaderGroupHandleSize: u32,
    pub maxRayRecursionDepth: u32,
    pub maxShaderGroupStride: u32,
    pub shaderGroupBaseAlignment: u32,
    pub shaderGroupHandleCaptureReplaySize: u32,
    pub maxRayDispatchInvocationCount: u32,
    pub shaderGroupHandleAlignment: u32,
    pub maxRayHitAttributeSize: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *mut c_void,
    pub rayTracingPipeline: VkBool32,
    pub rayTracingPipelineShaderGroupHandleCaptureReplay: VkBool32,
    pub rayTracingPipelineShaderGroupHandleCaptureReplayMixed: VkBool32,
    pub rayTracingPipelineTraceRaysIndirect: VkBool32,
    pub rayTraversalPrimitiveCulling: VkBool32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html
#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *mut c_void,
    pub accelerationStructure: VkBool32,
    pub accelerationStructureCaptureReplay: VkBool32,
    pub accelerationStructureIndirectBuild: VkBool32,
    pub accelerationStructureHostCommands: VkBool32,
    pub descriptorBindingAccelerationStructureUpdateAfterBind: VkBool32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html
#[repr(C)]
pub struct VkAccelerationStructureCreateInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub createFlags: VkAccelerationStructureCreateFlagsKHR,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub r#type: VkAccelerationStructureTypeKHR,
    pub deviceAddress: VkDeviceAddress,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html
#[repr(C)]
pub struct VkAccelerationStructureBuildGeometryInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub r#type: VkAccelerationStructureTypeKHR,
    pub flags: VkBuildAccelerationStructureFlagsKHR,
    pub mode: VkBuildAccelerationStructureModeKHR,
    pub srcAccelerationStructure: VkAccelerationStructureKHR,
    pub dstAccelerationStructure: VkAccelerationStructureKHR,
    pub geometryCount: u32,
    pub pGeometries: *const VkAccelerationStructureGeometryKHR,
    pub ppGeometries: *const *const VkAccelerationStructureGeometryKHR,
    pub scratchData: VkDeviceOrHostAddressKHR,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureTypeKHR.html
#[repr(C)]
pub enum VkAccelerationStructureTypeKHR {
    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR = 0,
    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR = 1,
    VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR = 2,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryKHR.html
#[repr(C)]
pub struct VkAccelerationStructureGeometryKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub geometryType: VkGeometryTypeKHR,
    pub geometry: VkAccelerationStructureGeometryDataKHR,
    pub flags: VkGeometryFlagsKHR,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureModeKHR.html
#[repr(C)]
pub enum VkBuildAccelerationStructureModeKHR {
    VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR = 0,
    VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR = 1,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryTypeKHR.html
#[repr(C)]
pub enum VkGeometryTypeKHR {
    VK_GEOMETRY_TYPE_TRIANGLES_KHR = 0,
    VK_GEOMETRY_TYPE_AABBS_KHR = 1,
    VK_GEOMETRY_TYPE_INSTANCES_KHR = 2,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html
#[repr(C)]
pub union VkAccelerationStructureGeometryDataKHR {
    pub triangles: VkAccelerationStructureGeometryTrianglesDataKHR,
    //pub aabbs: VkAccelerationStructureGeometryAabbsDataKHR,
    pub instances: VkAccelerationStructureGeometryInstancesDataKHR,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAccelerationStructureGeometryTrianglesDataKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub vertexFormat: VkFormat,
    pub vertexData: VkDeviceOrHostAddressConstKHR,
    pub vertexStride: VkDeviceSize,
    pub maxVertex: u32,
    pub indexType: VkIndexType,
    pub indexData: VkDeviceOrHostAddressConstKHR,
    pub transformData: VkDeviceOrHostAddressConstKHR,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAccelerationStructureGeometryInstancesDataKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub arrayOfPointers: VkBool32,
    pub data: VkDeviceOrHostAddressConstKHR,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceOrHostAddressConstKHR.html
#[repr(C)]
#[derive(Clone, Copy)]
pub union VkDeviceOrHostAddressConstKHR {
    pub deviceAddress: VkDeviceAddress,
    pub hostAddress: *const c_void,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceOrHostAddressKHR.html
#[repr(C)]
#[derive(Clone, Copy)]
pub union VkDeviceOrHostAddressKHR {
    pub deviceAddress: VkDeviceAddress,
    pub hostAddress: *mut c_void,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html
#[repr(C)]
pub struct VkAccelerationStructureBuildRangeInfoKHR {
    pub primitiveCount: u32,
    pub primitiveOffset: u32,
    pub firstVertex: u32,
    pub transformOffset: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html
#[repr(C)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub accelerationStructure: VkAccelerationStructureKHR,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineCreateInfoKHR.html
#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub groupCount: u32,
    pub pGroups: *const VkRayTracingShaderGroupCreateInfoKHR,
    pub maxPipelineRayRecursionDepth: u32,
    pub pLibraryInfo: *const VkPipelineLibraryCreateInfoKHR,
    pub pLibraryInterface: *const VkRayTracingPipelineInterfaceCreateInfoKHR,
    pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStridedDeviceAddressRegionKHR.html
#[repr(C)]
#[derive(Default)]
pub struct VkStridedDeviceAddressRegionKHR {
    pub deviceAddress: VkDeviceAddress,
    pub stride: VkDeviceSize,
    pub size: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html
#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub r#type: VkRayTracingShaderGroupTypeKHR,
    pub generalShader: u32,
    pub closestHitShader: u32,
    pub anyHitShader: u32,
    pub intersectionShader: u32,
    pub pShaderGroupCaptureReplayHandle: *const c_void,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html
#[repr(C)]
pub enum VkRayTracingShaderGroupTypeKHR {
    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR = 0,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR = 1,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR = 2,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html
#[repr(C)]
pub struct VkPipelineLibraryCreateInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub libraryCount: u32,
    pub pLibraries: *const VkPipeline,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html
#[repr(C)]
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxPipelineRayPayloadSize: u32,
    pub maxPipelineRayHitAttributeSize: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagBitsKHR.html#[repr(C)]
#[repr(C)]
pub enum VkGeometryFlagBitsKHR {
    VK_GEOMETRY_OPAQUE_BIT_KHR = 0x00000001,
    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = 0x00000002,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html
#[repr(C)]
pub enum VkAccelerationStructureBuildTypeKHR {
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR = 0,
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR = 1,
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR = 2,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html
#[repr(C)]
pub struct VkAccelerationStructureBuildSizesInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub accelerationStructureSize: VkDeviceSize,
    pub updateScratchSize: VkDeviceSize,
    pub buildScratchSize: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html
#[repr(C)]
pub enum VkBuildAccelerationStructureFlagBitsKHR {
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR = 0x00000001,
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR = 0x00000002,
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR = 0x00000004,
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR = 0x00000008,
    VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR = 0x00000010,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTransformMatrixKHR.html
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkTransformMatrixKHR {
    pub matrix: [[c_float; 4]; 3],
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html
#[repr(C)]
pub enum VkGeometryInstanceFlagBitsKHR {
    VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = 0x00000001,
    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR = 0x00000002,
    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR = 0x00000004,
    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR = 0x00000008,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInstanceKHR.html
#[repr(C)]
pub struct VkAccelerationStructureInstanceKHR {
    pub transform: VkTransformMatrixKHR,
    pub instanceCustomIndexAndMask: u32,
    pub instanceShaderBindingTableRecordOffsetAndFlags: u32,
    pub accelerationStructureReference: u64,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html
#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub accelerationStructureCount: u32,
    pub pAccelerationStructures: *const VkAccelerationStructureKHR,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html
#[repr(C)]
pub struct VkCopyAccelerationStructureInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub src: VkAccelerationStructureKHR,
    pub dst: VkAccelerationStructureKHR,
    pub mode: VkCopyAccelerationStructureModeKHR,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyAccelerationStructureModeKHR.html
#[repr(C)]
pub enum VkCopyAccelerationStructureModeKHR {
    VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR = 0,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR = 1,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR = 2,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR = 3,
}

mod dispatch {
    use super::*;

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateAccelerationStructureKHR.html
    pub fn vkCreateAccelerationStructureKHR(
        device: VkDevice,
        pCreateInfo: *const VkAccelerationStructureCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pAccelerationStructure: *mut VkAccelerationStructureKHR,
    ) -> VkResult {
        const NAME: &str = "vkCreateAccelerationStructureKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkDevice, 
                *const VkAccelerationStructureCreateInfoKHR, 
                *const VkAllocationCallbacks, 
                *mut VkAccelerationStructureKHR,
            ) -> VkResult;
            func = std::mem::transmute(addr);
            (func)(device, pCreateInfo, pAllocator, pAccelerationStructure)
        }
    }

    // @see https://khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyAccelerationStructureKHR.html
    pub fn vkDestroyAccelerationStructureKHR(
        device: VkDevice,
        accelerationStructure: VkAccelerationStructureKHR,
        pAllocator: *const VkAllocationCallbacks,
    ) {
        const NAME: &str = "vkDestroyAccelerationStructureKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkDevice, 
                VkAccelerationStructureKHR, 
                *const VkAllocationCallbacks,
            ) -> ();
            func = std::mem::transmute(addr);
            (func)(device, accelerationStructure, pAllocator)
        }
    }

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html
    pub fn dispatch_vkCmdBuildAccelerationStructuresKHR(
        device: VkDevice,
        commandBuffer: VkCommandBuffer,
        infoCount: u32,
        pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
        ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
    ) {
        const NAME: &str = "vkCmdBuildAccelerationStructuresKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkCommandBuffer,
                u32,
                *const VkAccelerationStructureBuildGeometryInfoKHR,
                *const *const VkAccelerationStructureBuildRangeInfoKHR,
            ) -> ();
            func = std::mem::transmute(addr);
            (func)(commandBuffer, infoCount, pInfos, ppBuildRangeInfos)
        }
    }

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html
    pub fn vkGetAccelerationStructureDeviceAddressKHR(
        device: VkDevice,
        pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR,
    ) -> VkDeviceAddress {
        const NAME: &str = "vkGetAccelerationStructureDeviceAddressKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkDevice,
                *const VkAccelerationStructureDeviceAddressInfoKHR,
            ) -> VkDeviceAddress;
            func = std::mem::transmute(addr);
            (func)(device, pInfo)
        }
    }

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRayTracingPipelinesKHR.html
    pub fn vkCreateRayTracingPipelinesKHR(
        device: VkDevice,
        deferredOperation: VkDeferredOperationKHR,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> VkResult {
        const NAME: &str = "vkCreateRayTracingPipelinesKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkDevice,
                VkDeferredOperationKHR,
                VkPipelineCache,
                u32,
                *const VkRayTracingPipelineCreateInfoKHR,
                *const VkAllocationCallbacks,
                *mut VkPipeline,
            ) -> VkResult;
            func = std::mem::transmute(addr);
            (func)(
                device,
                deferredOperation,
                pipelineCache,
                createInfoCount,
                pCreateInfos,
                pAllocator,
                pPipelines,
            )
        }
    }

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html
    pub fn vkGetRayTracingShaderGroupHandlesKHR(
        device: VkDevice,
        pipeline: VkPipeline,
        firstGroup: u32,
        groupCount: u32,
        dataSize: size_t,
        pData: *mut c_void,
    ) -> VkResult {
        const NAME: &str = "vkGetRayTracingShaderGroupHandlesKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkDevice,
                VkPipeline,
                u32,
                u32,
                size_t,
                *mut c_void,
            ) -> VkResult;
            func = std::mem::transmute(addr);
            (func)(
                device,
                pipeline,
                firstGroup,
                groupCount,
                dataSize,
                pData,
            )
        }
    }

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdTraceRaysKHR.html
    pub fn dispatch_vkCmdTraceRaysKHR(
        device: VkDevice,
        commandBuffer: VkCommandBuffer,
        pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
        pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
        pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
        pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        const NAME: &str = "vkCmdTraceRaysKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkCommandBuffer,
                *const VkStridedDeviceAddressRegionKHR,
                *const VkStridedDeviceAddressRegionKHR,
                *const VkStridedDeviceAddressRegionKHR,
                *const VkStridedDeviceAddressRegionKHR,
                u32,
                u32,
                u32,
            ) -> ();
            func = std::mem::transmute(addr);
            (func)(
                commandBuffer,
                pRaygenShaderBindingTable,
                pMissShaderBindingTable,
                pHitShaderBindingTable,
                pCallableShaderBindingTable,
                width,
                height,
                depth,
            )
        }
    }

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html
    pub fn dispatch_vkCmdWriteAccelerationStructuresPropertiesKHR(
        device: VkDevice,
        commandBuffer: VkCommandBuffer,
        accelerationStructureCount: u32,
        pAccelerationStructures: *const VkAccelerationStructureKHR,
        queryType: VkQueryType,
        queryPool: VkQueryPool,
        firstQuery: u32,
    ) {
        const NAME: &str = "vkCmdWriteAccelerationStructuresPropertiesKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkCommandBuffer,
                u32,
                *const VkAccelerationStructureKHR,
                VkQueryType,
                VkQueryPool,
                u32,
            ) -> ();
            func = std::mem::transmute(addr);
            (func)(
                commandBuffer,
                accelerationStructureCount,
                pAccelerationStructures,
                queryType,
                queryPool,
                firstQuery,
            )
        }
    }

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html
    pub fn dispatch_vkCmdCopyAccelerationStructureKHR(
        device: VkDevice,
        commandBuffer: VkCommandBuffer,
        pInfo: *const VkCopyAccelerationStructureInfoKHR,
    ) {
        const NAME: &str = "vkCmdCopyAccelerationStructureKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkCommandBuffer,
                *const VkCopyAccelerationStructureInfoKHR,
            ) -> ();
            func = std::mem::transmute(addr);
            (func)(
                commandBuffer,
                pInfo,
            )
        }
    }

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html
    pub fn vkGetAccelerationStructureBuildSizesKHR(
        device: VkDevice,
        buildType: VkAccelerationStructureBuildTypeKHR,
        pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR,
        pMaxPrimitiveCounts: *const u32,
        pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
    ) {
        const NAME: &str = "vkGetAccelerationStructureBuildSizesKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkDevice,
                VkAccelerationStructureBuildTypeKHR,
                *const VkAccelerationStructureBuildGeometryInfoKHR,
                *const u32,
                *mut VkAccelerationStructureBuildSizesInfoKHR,
            ) -> ();
            func = std::mem::transmute(addr);
            (func)(
                device,
                buildType,
                pBuildInfo,
                pMaxPrimitiveCounts,
                pSizeInfo,
            )
        }
    }
}

pub use dispatch::*;
