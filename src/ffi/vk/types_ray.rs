
// Provided by VK_KHR_ray_tracing

#![allow(dead_code)]
#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_void, c_char, size_t, c_float};

use super::types::*;
use super::types_ext::*;

#[repr(C)]
#[derive(Debug)]
pub enum VkStructureTypeExtRay {
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_KHR = 1000150014,
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
