
#![allow(dead_code)]
#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_void, c_char, size_t, c_float};

use super::types::*;
use super::types_ext::*;

// Provided by VK_EXT_debug_utils

pub type VkDebugUtilsMessengerCreateFlagsEXT = VkFlags;
pub type VkDebugUtilsMessageSeverityFlagsEXT = VkFlags;
pub type VkDebugUtilsMessageTypeFlagsEXT = VkFlags;
pub type VkDebugUtilsMessengerCallbackDataFlagsEXT = VkFlags;

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html
pub type PFN_vkDebugUtilsMessengerCallbackEXT = extern fn (
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
    pUserData: *mut c_void,
) -> VkBool32;

#[repr(C)]
pub struct VkDebugUtilsMessengerEXTOpaque { _private: [u8; 0] }
pub type VkDebugUtilsMessengerEXT = *mut VkDebugUtilsMessengerEXTOpaque;

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html
#[repr(C)]
pub enum VkDebugUtilsMessageTypeFlagBitsEXT {
    VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT = 0x00000001,
    VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT = 0x00000002,
    VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT = 0x00000004,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html
#[repr(C)]
pub enum VkDebugUtilsMessageSeverityFlagBitsEXT {
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT = 0x00000001,
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT = 0x00000010,
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT = 0x00000100,
    VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT = 0x00001000,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsLabelEXT.html
#[repr(C)]
pub struct VkDebugUtilsLabelEXT {
    pub sType: VkStructureTypeExt,
    pub pNext: *const c_void,
    pub pLabelName: *const c_char,
    pub color: [c_float; 4],
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkObjectType.html
#[repr(C)]
pub enum VkObjectType {
    VK_OBJECT_TYPE_UNKNOWN = 0,
    VK_OBJECT_TYPE_INSTANCE = 1,
    VK_OBJECT_TYPE_PHYSICAL_DEVICE = 2,
    VK_OBJECT_TYPE_DEVICE = 3,
    VK_OBJECT_TYPE_QUEUE = 4,
    VK_OBJECT_TYPE_SEMAPHORE = 5,
    VK_OBJECT_TYPE_COMMAND_BUFFER = 6,
    VK_OBJECT_TYPE_FENCE = 7,
    VK_OBJECT_TYPE_DEVICE_MEMORY = 8,
    VK_OBJECT_TYPE_BUFFER = 9,
    VK_OBJECT_TYPE_IMAGE = 10,
    VK_OBJECT_TYPE_EVENT = 11,
    VK_OBJECT_TYPE_QUERY_POOL = 12,
    VK_OBJECT_TYPE_BUFFER_VIEW = 13,
    VK_OBJECT_TYPE_IMAGE_VIEW = 14,
    VK_OBJECT_TYPE_SHADER_MODULE = 15,
    VK_OBJECT_TYPE_PIPELINE_CACHE = 16,
    VK_OBJECT_TYPE_PIPELINE_LAYOUT = 17,
    VK_OBJECT_TYPE_RENDER_PASS = 18,
    VK_OBJECT_TYPE_PIPELINE = 19,
    VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT = 20,
    VK_OBJECT_TYPE_SAMPLER = 21,
    VK_OBJECT_TYPE_DESCRIPTOR_POOL = 22,
    VK_OBJECT_TYPE_DESCRIPTOR_SET = 23,
    VK_OBJECT_TYPE_FRAMEBUFFER = 24,
    VK_OBJECT_TYPE_COMMAND_POOL = 25,
    VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION = 1000156000,
    VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE = 1000085000,
    VK_OBJECT_TYPE_SURFACE_KHR = 1000000000,
    VK_OBJECT_TYPE_SWAPCHAIN_KHR = 1000001000,
    VK_OBJECT_TYPE_DISPLAY_KHR = 1000002000,
    VK_OBJECT_TYPE_DISPLAY_MODE_KHR = 1000002001,
    VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT = 1000011000,
    VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT = 1000128000,
    VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR = 1000165000,
    VK_OBJECT_TYPE_VALIDATION_CACHE_EXT = 1000160000,
    VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL = 1000210000,
    VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR = 1000268000,
    VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV = 1000277000,
    VK_OBJECT_TYPE_PRIVATE_DATA_SLOT_EXT = 1000295000,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html
#[repr(C)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub sType: VkStructureTypeExt,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub pObjectName: *const c_char,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html
#[repr(C)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    pub sType: VkStructureTypeExt,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
    pub pMessageIdName: *const c_char,
    pub messageIdNumber: i32,
    pub pMessage: *const c_char,
    pub queueLabelCount: u32,
    pub pQueueLabels: *const VkDebugUtilsLabelEXT,
    pub cmdBufLabelCount: u32,
    pub pCmdBufLabels: *const VkDebugUtilsLabelEXT,
    pub objectCount: u32,
    pub pObjects: *const VkDebugUtilsObjectNameInfoEXT,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html
#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType: VkStructureTypeExt,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
    pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
    pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut c_void,
}

mod dispatch {

    use super::*;

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html
    pub fn vkCreateDebugUtilsMessengerEXT(
        instance: VkInstance,
        pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
        pMessenger: *mut VkDebugUtilsMessengerEXT,
    ) -> VkResult {
        const NAME: &str = "vkCreateDebugUtilsMessengerEXT\0";
        unsafe {
            let addr = vkGetInstanceProcAddr(instance, NAME.as_ptr() as *const c_char);
            let func: extern fn (
                instance: VkInstance,
                pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
                pAllocator: *const VkAllocationCallbacks,
                pMessenger: *mut VkDebugUtilsMessengerEXT,
            ) -> VkResult;
            func = std::mem::transmute(addr);
            (func)(instance, pCreateInfo, pAllocator, pMessenger)
        }
    }
}

pub use dispatch::*;
