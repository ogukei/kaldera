
#![allow(dead_code)]
#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_void, c_char, size_t, c_float};

use super::types::*;

pub type VkSurfaceTransformFlagsKHR = VkFlags;
pub type VkCompositeAlphaFlagsKHR = VkFlags;

#[repr(C)]
pub struct VkSurfaceKHROpaque { _private: [u8; 0] }
pub type VkSurfaceKHR = *mut VkSurfaceKHROpaque;

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html
#[repr(C)]
pub enum VkSurfaceTransformFlagBitsKHR {
    VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 0x00000001,
    VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 0x00000002,
    VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 0x00000004,
    VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 0x00000008,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 0x00000010,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040,
    VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080,
    VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR = 0x00000100,
    VK_SURFACE_TRANSFORM_FLAG_BITS_MAX_ENUM_KHR = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSurfaceCapabilitiesKHR.html
#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
}

#[repr(C)]
pub enum VkStructureTypeExt {
    VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR = 1000005000,
}

#[link(name = "vulkan")]
extern "C" {

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html
    pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySurfaceKHR.html
    pub fn vkDestroySurfaceKHR(
        instance: VkInstance,
        surface: VkSurfaceKHR,
        pAllocator: *const VkAllocationCallbacks,
    );
}

mod xcb {
    use super::*;
    use crate::ffi::xcb::*;

    pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;

    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html
    #[repr(C)]
    pub struct VkXcbSurfaceCreateInfoKHR {
        pub sType: VkStructureTypeExt,
        pub pNext: *const c_void,
        pub flags: VkXcbSurfaceCreateFlagsKHR,
        pub connection: *const xcb_connection_t,
        pub window: xcb_window_t,
    }

    impl VkXcbSurfaceCreateInfoKHR {
        pub fn new(connection: *const xcb_connection_t, window: xcb_window_t) -> Self {
            Self {
                sType: VkStructureTypeExt::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
                pNext: std::ptr::null(),
                flags: VK_FLAGS_NONE,
                connection,
                window,
            }
        }
    }
  
    #[link(name = "vulkan")]
    extern "C" {
        // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateXcbSurfaceKHR.html
        pub fn vkCreateXcbSurfaceKHR(
            instance: VkInstance,
            pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
            pAllocator: *const VkAllocationCallbacks,
            pSurface: *mut VkSurfaceKHR,
        ) -> VkResult;
    }
}

pub use xcb::*;
