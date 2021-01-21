
// Provided by VK_KHR_ray_tracing

#![allow(dead_code)]
#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_void, c_char, size_t, c_float};

use super::types::*;
use super::types_ext::*;

pub type VkAccelerationStructureCreateFlagsKHR = VkFlags;
pub type VkBuildAccelerationStructureFlagsKHR = VkFlags;

#[repr(C)]
pub struct VkAccelerationStructureKHROpaque { _private: [u8; 0] }
pub type VkAccelerationStructureKHR = *mut VkAccelerationStructureKHROpaque;

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

    // DEPRECATED
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_GEOMETRY_TYPE_INFO_KHR = 1000150001,
    VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_KHR = 1000150008,
    VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_KHR = 1000165006,
    VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR = 1000290000,
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
}

pub use dispatch::*;
