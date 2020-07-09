
// Provided by VK_KHR_ray_tracing

#![allow(dead_code)]
#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_void, c_char, size_t, c_float};

use super::types::*;
use super::types_ext::*;

pub type VkGeometryFlagsKHR = VkFlags;
pub type VkBuildAccelerationStructureFlagsKHR = VkFlags;
pub type VkGeometryInstanceFlagsKHR = VkFlags;

#[repr(C)]
pub struct VkAccelerationStructureKHROpaque { _private: [u8; 0] }
pub type VkAccelerationStructureKHR = *mut VkAccelerationStructureKHROpaque;

pub const VK_SHADER_UNUSED_KHR: u32 = !0u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum VkStructureTypeExtRay {
    VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_KHR = 1000165006,
    VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR = 1000165007,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR = 1000150000,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_GEOMETRY_TYPE_INFO_KHR = 1000150001,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR = 1000150002,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR = 1000150003,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR = 1000150004,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR = 1000150005,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR = 1000150006,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_KHR = 1000150008,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_KHR = 1000150009,
    VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR = 1000150010,
    VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR = 1000150011,
    VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR = 1000150012,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_FEATURES_KHR = 1000150013,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_KHR = 1000150014,
    VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR = 1000150015,
    VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR = 1000150016,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR = 1000150017,
    VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR = 1000150018,
    VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV = 1000152000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV = 1000154000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV = 1000154001,
    VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT = 1000158000,
    VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT = 1000158001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT = 1000158002,
    VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT = 1000158003,
    VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT = 1000158004,
    VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT = 1000158005,
    VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT = 1000160000,
    VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT = 1000160001,
    VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR = 1000290000,
}

#[repr(C)]
pub enum VkGeometryTypeKHR {
    VK_GEOMETRY_TYPE_TRIANGLES_KHR = 0,
    VK_GEOMETRY_TYPE_AABBS_KHR = 1,
    VK_GEOMETRY_TYPE_INSTANCES_KHR = 1000150000,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesKHR.html
#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceRayTracingPropertiesKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *mut c_void,
    pub shaderGroupHandleSize: u32,
    pub maxRecursionDepth: u32,
    pub maxShaderGroupStride: u32,
    pub shaderGroupBaseAlignment: u32,
    pub maxGeometryCount: u64,
    pub maxInstanceCount: u64,
    pub maxPrimitiveCount: u64,
    pub maxDescriptorSetAccelerationStructures: u32,
    pub shaderGroupHandleCaptureReplaySize: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateGeometryTypeInfoKHR.html 
#[repr(C)]
pub struct VkAccelerationStructureCreateGeometryTypeInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub geometryType: VkGeometryTypeKHR,
    pub maxPrimitiveCount: u32,
    pub indexType: VkIndexType,
    pub maxVertexCount: u32,
    pub vertexFormat: VkFormat,
    pub allowsTransforms: VkBool32,
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

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html
#[repr(C)]
#[derive(Clone, Copy)]
pub union VkAccelerationStructureGeometryDataKHR {
    pub triangles: VkAccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: VkAccelerationStructureGeometryAabbsDataKHR,
    pub instances: VkAccelerationStructureGeometryInstancesDataKHR,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAccelerationStructureGeometryInstancesDataKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub arrayOfPointers: VkBool32,
    pub data: VkDeviceAddress,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAccelerationStructureGeometryTrianglesDataKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub vertexFormat: VkFormat,
    pub vertexData: VkDeviceAddress,
    pub vertexStride: VkDeviceSize,
    pub indexType: VkIndexType,
    pub indexData: VkDeviceAddress,
    pub transformData: VkDeviceAddress,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAccelerationStructureGeometryAabbsDataKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub data: VkDeviceAddress,
    pub stride: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureTypeKHR.html
#[repr(C)]
pub enum VkAccelerationStructureTypeKHR {
    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR = 0,
    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR = 1,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html
#[repr(C)]
pub struct VkAccelerationStructureCreateInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub compactedSize: VkDeviceSize,
    pub r#type: VkAccelerationStructureTypeKHR,
    pub flags: VkBuildAccelerationStructureFlagsKHR,
    pub maxGeometryCount: u32,
    pub pGeometryInfos: *const VkAccelerationStructureCreateGeometryTypeInfoKHR,
    pub deviceAddress: VkDeviceAddress,
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

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeKHR.html
#[repr(C)]
pub enum VkAccelerationStructureMemoryRequirementsTypeKHR {
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_KHR = 0,
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_KHR = 1,
    VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_KHR = 2,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html
#[repr(C)]
pub enum VkAccelerationStructureBuildTypeKHR {
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR = 0,
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR = 1,
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR = 2,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureMemoryRequirementsInfoKHR.html
#[repr(C)]
pub struct VkAccelerationStructureMemoryRequirementsInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub r#type: VkAccelerationStructureMemoryRequirementsTypeKHR,
    pub buildType: VkAccelerationStructureBuildTypeKHR,
    pub accelerationStructure: VkAccelerationStructureKHR,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindAccelerationStructureMemoryInfoKHR.html
#[repr(C)]
pub struct VkBindAccelerationStructureMemoryInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub accelerationStructure: VkAccelerationStructureKHR,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryFlagBitsKHR.html
#[repr(C)]
pub enum VkGeometryFlagBitsKHR {
    VK_GEOMETRY_OPAQUE_BIT_KHR = 0x00000001,
    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = 0x00000002,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildOffsetInfoKHR.html
#[repr(C)]
pub struct VkAccelerationStructureBuildOffsetInfoKHR {
    pub primitiveCount: u32,
    pub primitiveOffset: u32,
    pub firstVertex: u32,
    pub transformOffset: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html
#[repr(C)]
pub struct VkAccelerationStructureBuildGeometryInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub r#type: VkAccelerationStructureTypeKHR,
    pub flags: VkBuildAccelerationStructureFlagsKHR,
    pub update: VkBool32,
    pub srcAccelerationStructure: VkAccelerationStructureKHR,
    pub dstAccelerationStructure: VkAccelerationStructureKHR,
    pub geometryArrayOfPointers: VkBool32,
    pub geometryCount: u32,
    pub ppGeometries: *const *const VkAccelerationStructureGeometryKHR,
    pub scratchData: VkDeviceAddress,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkTransformMatrixKHR.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkTransformMatrixKHR {
    pub matrix: [[c_float; 4]; 3],
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureInstanceKHR.html
#[repr(C)]
pub struct VkAccelerationStructureInstanceKHR {
    pub transform: VkTransformMatrixKHR,
    pub instanceCustomIndexAndMask: u32,
    pub instanceShaderBindingTableRecordOffsetAndFlags: u32,
    pub accelerationStructureReference: u64,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html
#[repr(C)]
pub enum VkGeometryInstanceFlagBitsKHR {
    VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = 0x00000001,
    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR = 0x00000002,
    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR = 0x00000004,
    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR = 0x00000008,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html
#[repr(C)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub accelerationStructure: VkAccelerationStructureKHR,
}

// @see https://khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html
#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub accelerationStructureCount: u32,
    pub pAccelerationStructures: *const VkAccelerationStructureKHR,
}

// @see https://khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html
#[repr(C)]
pub enum VkRayTracingShaderGroupTypeKHR {
    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR = 0,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR = 1,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR = 2,
}

// @see https://khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html
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
    pub sType: VkStructureTypeExtRay,
    pub pNext: *const c_void,
    pub maxPayloadSize: u32,
    pub maxAttributeSize: u32,
    pub maxCallableSize: u32,
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
    pub maxRecursionDepth: u32,
    pub libraries: VkPipelineLibraryCreateInfoKHR,
    pub pLibraryInterface: *const VkRayTracingPipelineInterfaceCreateInfoKHR,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStridedBufferRegionKHR.html
#[repr(C)]
pub struct VkStridedBufferRegionKHR {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub stride: VkDeviceSize,
    pub size: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayTracingFeaturesKHR.html
#[repr(C)]
pub struct VkPhysicalDeviceRayTracingFeaturesKHR {
    pub sType: VkStructureTypeExtRay,
    pub pNext: *mut c_void,
    pub rayTracing: VkBool32,
    pub rayTracingShaderGroupHandleCaptureReplay: VkBool32,
    pub rayTracingShaderGroupHandleCaptureReplayMixed: VkBool32,
    pub rayTracingAccelerationStructureCaptureReplay: VkBool32,
    pub rayTracingIndirectTraceRays: VkBool32,
    pub rayTracingIndirectAccelerationStructureBuild: VkBool32,
    pub rayTracingHostAccelerationStructureCommands: VkBool32,
    pub rayQuery: VkBool32,
    pub rayTracingPrimitiveCulling: VkBool32,
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

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsKHR.html
    pub fn vkGetAccelerationStructureMemoryRequirementsKHR(
        device: VkDevice,
        pInfo: *const VkAccelerationStructureMemoryRequirementsInfoKHR,
        pMemoryRequirements: *mut VkMemoryRequirements2,
    ) {
        const NAME: &str = "vkGetAccelerationStructureMemoryRequirementsKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkDevice, 
                *const VkAccelerationStructureMemoryRequirementsInfoKHR, 
                *mut VkMemoryRequirements2,
            ) -> ();
            func = std::mem::transmute(addr);
            (func)(device, pInfo, pMemoryRequirements)
        }
    }

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindAccelerationStructureMemoryKHR.html
    pub fn vkBindAccelerationStructureMemoryKHR(
        device: VkDevice,
        bindInfoCount: u32,
        pBindInfos: *const VkBindAccelerationStructureMemoryInfoKHR,
    ) -> VkResult {
        const NAME: &str = "vkBindAccelerationStructureMemoryKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkDevice, 
                u32, 
                *const VkBindAccelerationStructureMemoryInfoKHR,
            ) -> VkResult;
            func = std::mem::transmute(addr);
            (func)(device, bindInfoCount, pBindInfos)
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

    // @see https://khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBuildAccelerationStructureKHR.html
    pub fn dispatch_vkCmdBuildAccelerationStructureKHR(
        device: VkDevice,
        commandBuffer: VkCommandBuffer,
        infoCount: u32,
        pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
        ppOffsetInfos: *const *const VkAccelerationStructureBuildOffsetInfoKHR,
    ) {
        const NAME: &str = "vkCmdBuildAccelerationStructureKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkCommandBuffer,
                u32,
                *const VkAccelerationStructureBuildGeometryInfoKHR,
                *const *const VkAccelerationStructureBuildOffsetInfoKHR,
            ) -> ();
            func = std::mem::transmute(addr);
            (func)(commandBuffer, infoCount, pInfos, ppOffsetInfos)
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
                VkPipelineCache,
                u32,
                *const VkRayTracingPipelineCreateInfoKHR,
                *const VkAllocationCallbacks,
                *mut VkPipeline,
            ) -> VkResult;
            func = std::mem::transmute(addr);
            (func)(
                device,
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
        pRaygenShaderBindingTable: *const VkStridedBufferRegionKHR,
        pMissShaderBindingTable: *const VkStridedBufferRegionKHR,
        pHitShaderBindingTable: *const VkStridedBufferRegionKHR,
        pCallableShaderBindingTable: *const VkStridedBufferRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        const NAME: &str = "vkCmdTraceRaysKHR\0";
        unsafe {
            let addr = vkGetDeviceProcAddr(device, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                VkCommandBuffer,
                *const VkStridedBufferRegionKHR,
                *const VkStridedBufferRegionKHR,
                *const VkStridedBufferRegionKHR,
                *const VkStridedBufferRegionKHR,
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
}

pub use dispatch::*;
