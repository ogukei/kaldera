
use crate::ffi::vk::*;

use std::ptr;
use std::mem::MaybeUninit;
use std::ffi::CStr;
use libc::{c_void};

use VkStructureTypeExt::*;
use VkDebugUtilsMessageSeverityFlagBitsEXT::*;
use VkDebugUtilsMessageTypeFlagBitsEXT::*;

pub struct DebugUtilsMessenger {
    instance: VkInstance,
    handle: VkDebugUtilsMessengerEXT, 
}

impl DebugUtilsMessenger {
    pub fn new(instance: VkInstance) -> Self {
        unsafe {
            extern fn callback(
                _: VkDebugUtilsMessageSeverityFlagBitsEXT, 
                _: VkDebugUtilsMessageTypeFlagsEXT,
                data: *const VkDebugUtilsMessengerCallbackDataEXT,
                _: *mut c_void,
            ) -> VkBool32 {
                unsafe {
                    let data = data.as_ref().unwrap();
                    let message = CStr::from_ptr(data.pMessage);
                    let message = message.to_str().unwrap();
                    println!("{}", message);
                }
                VK_FALSE
            }
            let create_info = VkDebugUtilsMessengerCreateInfoEXT {
                sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
                pNext: ptr::null(),
                flags: 0,
                messageSeverity: 0 as VkFlags
                    //| VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT as VkFlags
                    | VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT as VkFlags
                    | VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT as VkFlags 
                    | VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT as VkFlags
                ,
                messageType: 0 as VkFlags
                    | VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT as VkFlags
                    | VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT as VkFlags
                    //| VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT as VkFlags
                ,
                pfnUserCallback: callback,
                pUserData: ptr::null_mut(),
            };
            let mut handle = MaybeUninit::<VkDebugUtilsMessengerEXT>::zeroed();
            vkCreateDebugUtilsMessengerEXT(instance, &create_info, ptr::null(), handle.as_mut_ptr())
                .into_result()
                .unwrap();
            let handle = handle.assume_init();
            Self {
                instance,
                handle,
            }
        }
    }
}

impl Drop for DebugUtilsMessenger {
    fn drop(&mut self) {
        unsafe {
            vkDestroyDebugUtilsMessengerEXT(self.instance, self.handle, ptr::null());
        }
    }
}
