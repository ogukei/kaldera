
#![allow(dead_code)]
#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_void, c_char, size_t, c_float};

// @see https://www.khronos.org/registry/vulkan/specs/1.1/html/vkspec.html

pub type VkDeviceSize = u64;
pub type VkSampleMask = u32;

pub type VkBool32 = u32;
pub type VkFlags = u32;
pub type VkSampleCountFlags = VkFlags;
pub type VkQueueFlags = VkFlags;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkCommandPoolCreateFlags = VkFlags;
pub type VkBufferCreateFlags = VkFlags;
pub type VkBufferUsageFlags = VkFlags;
pub type VkMemoryPropertyFlags = VkFlags;
pub type VkMemoryHeapFlags = VkFlags;
pub type VkMemoryMapFlags = VkFlags;
pub type VkCommandBufferUsageFlags = VkFlags;
pub type VkQueryControlFlags = VkFlags;
pub type VkQueryPipelineStatisticFlags = VkFlags;
pub type VkFenceCreateFlags = VkFlags;
pub type VkPipelineStageFlags = VkFlags;
pub type VkDescriptorPoolCreateFlags = VkFlags;
pub type VkDescriptorSetLayoutCreateFlags = VkFlags;
pub type VkShaderStageFlags = VkFlags;
pub type VkPipelineLayoutCreateFlags = VkFlags;
pub type VkPipelineCacheCreateFlags = VkFlags;
pub type VkPipelineCreateFlags = VkFlags;
pub type VkPipelineShaderStageCreateFlags = VkFlags;
pub type VkShaderModuleCreateFlags = VkFlags;
pub type VkAccessFlags = VkFlags;
pub type VkDependencyFlags = VkFlags;
pub type VkImageUsageFlags = VkFlags;
pub type VkImageViewCreateFlags = VkFlags;
pub type VkImageAspectFlags = VkFlags;
pub type VkImageCreateFlags = VkFlags;
pub type VkAttachmentDescriptionFlags = VkFlags;
pub type VkSubpassDescriptionFlags = VkFlags;
pub type VkRenderPassCreateFlags = VkFlags;
pub type VkFramebufferCreateFlags = VkFlags;
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
pub type VkPipelineViewportStateCreateFlags = VkFlags;
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;
pub type VkCullModeFlags = VkFlags;
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;
pub type VkPipelineColorBlendStateCreateFlags = VkFlags;
pub type VkColorComponentFlags = VkFlags;
pub type VkPipelineDynamicStateCreateFlags = VkFlags;
pub type VkSemaphoreCreateFlags = VkFlags;
pub type VkSamplerCreateFlags = VkFlags;
pub type VkMemoryAllocateFlags = VkFlags;
pub type VkQueryPoolCreateFlags = VkFlags;
pub type VkQueryResultFlags = VkFlags;
pub type VkDescriptorBindingFlags = VkFlags;

#[repr(C)]
pub struct VkInstanceOpaque { _private: [u8; 0] }
pub type VkInstance = *mut VkInstanceOpaque;
#[repr(C)]
pub struct VkPhysicalDeviceOpaque { _private: [u8; 0] }
pub type VkPhysicalDevice = *mut VkPhysicalDeviceOpaque;
#[repr(C)]
pub struct VkDeviceOpaque { _private: [u8; 0] }
pub type VkDevice = *mut VkDeviceOpaque;
#[repr(C)]
pub struct VkQueueOpaque { _private: [u8; 0] }
pub type VkQueue = *mut VkQueueOpaque;
#[repr(C)]
pub struct VkCommandPoolOpaque { _private: [u8; 0] }
pub type VkCommandPool = *mut VkCommandPoolOpaque;
#[repr(C)]
pub struct VkBufferOpaque { _private: [u8; 0] }
pub type VkBuffer = *mut VkBufferOpaque;
#[repr(C)]
pub struct VkDeviceMemoryOpaque { _private: [u8; 0] }
pub type VkDeviceMemory = *mut VkDeviceMemoryOpaque;
#[repr(C)]
pub struct VkCommandBufferOpaque { _private: [u8; 0] }
pub type VkCommandBuffer = *mut VkCommandBufferOpaque;
#[repr(C)]
pub struct VkRenderPassOpaque { _private: [u8; 0] }
pub type VkRenderPass = *mut VkRenderPassOpaque;
#[repr(C)]
pub struct VkFramebufferOpaque { _private: [u8; 0] }
pub type VkFramebuffer = *mut VkFramebufferOpaque;
#[repr(C)]
pub struct VkFenceOpaque { _private: [u8; 0] }
pub type VkFence = *mut VkFenceOpaque;
#[repr(C)]
pub struct VkSemaphoreOpaque { _private: [u8; 0] }
pub type VkSemaphore = *mut VkSemaphoreOpaque;
#[repr(C)]
pub struct VkDescriptorPoolOpaque { _private: [u8; 0] }
pub type VkDescriptorPool = *mut VkDescriptorPoolOpaque;
#[repr(C)]
pub struct VkDescriptorSetLayoutOpaque { _private: [u8; 0] }
pub type VkDescriptorSetLayout = *mut VkDescriptorSetLayoutOpaque;
#[repr(C)]
pub struct VkSamplerOpaque { _private: [u8; 0] }
pub type VkSampler = *mut VkSamplerOpaque;
#[repr(C)]
pub struct VkPipelineLayoutOpaque { _private: [u8; 0] }
pub type VkPipelineLayout = *mut VkPipelineLayoutOpaque;
#[repr(C)]
pub struct VkDescriptorSetOpaque { _private: [u8; 0] }
pub type VkDescriptorSet = *mut VkDescriptorSetOpaque;
#[repr(C)]
pub struct VkBufferViewOpaque { _private: [u8; 0] }
pub type VkBufferView = *mut VkBufferViewOpaque;
#[repr(C)]
pub struct VkImageViewOpaque { _private: [u8; 0] }
pub type VkImageView = *mut VkImageViewOpaque;
#[repr(C)]
pub struct VkPipelineCacheOpaque { _private: [u8; 0] }
pub type VkPipelineCache = *mut VkPipelineCacheOpaque;
#[repr(C)]
pub struct VkPipelineOpaque { _private: [u8; 0] }
pub type VkPipeline = *mut VkPipelineOpaque;
#[repr(C)]
pub struct VkShaderModuleOpaque { _private: [u8; 0] }
pub type VkShaderModule = *mut VkShaderModuleOpaque;
#[repr(C)]
pub struct VkImageOpaque { _private: [u8; 0] }
pub type VkImage = *mut VkImageOpaque;
#[repr(C)]
pub struct VkQueryPoolOpaque { _private: [u8; 0] }
pub type VkQueryPool = *mut VkQueryPoolOpaque;

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: size_t = 256;
pub const VK_UUID_SIZE: size_t = 16;
pub const VK_MAX_MEMORY_TYPES: size_t = 32;
pub const VK_MAX_MEMORY_HEAPS: size_t = 16;
pub const VK_WHOLE_SIZE: u64 = u64::max_value();
pub const VK_FLAGS_NONE: VkFlags = 0;
pub const VK_TRUE: VkBool32 = 1;
pub const VK_FALSE: VkBool32 = 0;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = u32::max_value();
pub const VK_MAX_EXTENSION_NAME_SIZE: size_t = 256;
pub const VK_MAX_DESCRIPTION_SIZE: size_t = 256;
pub const VK_SUBPASS_EXTERNAL: u32 = !0u32;
pub const VK_REMAINING_MIP_LEVELS: u32 = !0u32;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0u32;

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkResult.html
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum VkResult {
    VK_SUCCESS = 0,
    VK_NOT_READY = 1,
    VK_TIMEOUT = 2,
    VK_EVENT_SET = 3,
    VK_EVENT_RESET = 4,
    VK_INCOMPLETE = 5,
    VK_ERROR_OUT_OF_HOST_MEMORY = -1,
    VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,
    VK_ERROR_INITIALIZATION_FAILED = -3,
    VK_ERROR_DEVICE_LOST = -4,
    VK_ERROR_MEMORY_MAP_FAILED = -5,
    VK_ERROR_LAYER_NOT_PRESENT = -6,
    VK_ERROR_EXTENSION_NOT_PRESENT = -7,
    VK_ERROR_FEATURE_NOT_PRESENT = -8,
    VK_ERROR_INCOMPATIBLE_DRIVER = -9,
    VK_ERROR_TOO_MANY_OBJECTS = -10,
    VK_ERROR_FORMAT_NOT_SUPPORTED = -11,
    VK_ERROR_FRAGMENTED_POOL = -12,
    VK_ERROR_OUT_OF_POOL_MEMORY = -1000069000,
    VK_ERROR_INVALID_EXTERNAL_HANDLE = -1000072003,
    VK_ERROR_SURFACE_LOST_KHR = -1000000000,
    VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,
    VK_SUBOPTIMAL_KHR = 1000001003,
    VK_ERROR_OUT_OF_DATE_KHR = -1000001004,
    VK_ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001,
    VK_ERROR_VALIDATION_FAILED_EXT = -1000011001,
    VK_ERROR_INVALID_SHADER_NV = -1000012000,
    VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT = -1000158000,
    VK_ERROR_FRAGMENTATION_EXT = -1000161000,
    VK_ERROR_NOT_PERMITTED_EXT = -1000174001,
    VK_ERROR_INVALID_DEVICE_ADDRESS_EXT = -1000244000,
    VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT = -1000255000,
    VK_RESULT_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent2D.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkExtent3D.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSharingMode.html
#[repr(C)]
pub enum VkSharingMode {
    VK_SHARING_MODE_EXCLUSIVE = 0,
    VK_SHARING_MODE_CONCURRENT = 1,
    VK_SHARING_MODE_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStructureType.html
#[repr(C)]
#[derive(Clone, Copy)]
pub enum VkStructureType {
    VK_STRUCTURE_TYPE_APPLICATION_INFO = 0,
    VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO = 2,
    VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO = 3,
    VK_STRUCTURE_TYPE_SUBMIT_INFO = 4,
    VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO = 5,
    VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE = 6,
    VK_STRUCTURE_TYPE_BIND_SPARSE_INFO = 7,
    VK_STRUCTURE_TYPE_FENCE_CREATE_INFO = 8,
    VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO = 9,
    VK_STRUCTURE_TYPE_EVENT_CREATE_INFO = 10,
    VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO = 11,
    VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO = 12,
    VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO = 13,
    VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO = 14,
    VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO = 15,
    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO = 16,
    VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO = 17,
    VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO = 18,
    VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,
    VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,
    VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,
    VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,
    VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,
    VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,
    VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,
    VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,
    VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,
    VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO = 28,
    VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO = 29,
    VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO = 30,
    VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO = 31,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
    VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO = 33,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO = 34,
    VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET = 35,
    VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET = 36,
    VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO = 37,
    VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO = 38,
    VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO = 39,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO = 40,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO = 41,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO = 42,
    VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO = 43,
    VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER = 44,
    VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER = 45,
    VK_STRUCTURE_TYPE_MEMORY_BARRIER = 46,
    VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO = 47,
    VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO = 48,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES = 1000094000,
    VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO = 1000157000,
    VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO = 1000157001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES = 1000083000,
    VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS = 1000127000,
    VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO = 1000127001,
    VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO = 1000060000,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO = 1000060003,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO = 1000060004,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO = 1000060005,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO = 1000060006,
    VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO = 1000060013,
    VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO = 1000060014,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES = 1000070000,
    VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO = 1000070001,
    VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2 = 1000146000,
    VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2 = 1000146001,
    VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2 = 1000146002,
    VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2 = 1000146003,
    VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2 = 1000146004,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2 = 1000059000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2 = 1000059001,
    VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2 = 1000059002,
    VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2 = 1000059003,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2 = 1000059004,
    VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2 = 1000059005,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2 = 1000059006,
    VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2 = 1000059007,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2 = 1000059008,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES = 1000117000,
    VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO = 1000117001,
    VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO = 1000117002,
    VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO = 1000117003,
    VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO = 1000053000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES = 1000053001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES = 1000053002,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES = 1000120000,
    VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO = 1000145000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES = 1000145001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES = 1000145002,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2 = 1000145003,
    VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO = 1000156000,
    VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO = 1000156001,
    VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO = 1000156002,
    VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO = 1000156003,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES = 1000156004,
    VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES = 1000156005,
    VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO = 1000085000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO = 1000071000,
    VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES = 1000071001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO = 1000071002,
    VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES = 1000071003,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES = 1000071004,
    VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO = 1000072000,
    VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO = 1000072001,
    VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO = 1000072002,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO = 1000112000,
    VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES = 1000112001,
    VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO = 1000113000,
    VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO = 1000077000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO = 1000076000,
    VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES = 1000076001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES = 1000168000,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT = 1000168001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES = 1000063000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES = 49,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES = 50,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES = 51,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES = 52,
    VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO = 1000147000,
    VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2 = 1000109000,
    VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2 = 1000109001,
    VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2 = 1000109002,
    VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2 = 1000109003,
    VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2 = 1000109004,
    VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO = 1000109005,
    VK_STRUCTURE_TYPE_SUBPASS_END_INFO = 1000109006,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES = 1000177000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES = 1000196000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES = 1000180000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES = 1000082000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES = 1000197000,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO = 1000161000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES = 1000161001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES = 1000161002,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO = 1000161003,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT = 1000161004,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES = 1000199000,
    VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE = 1000199001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES = 1000221000,
    VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO = 1000246000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES = 1000130000,
    VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO = 1000130001,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES = 1000211000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES = 1000108000,
    VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO = 1000108001,
    VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO = 1000108002,
    VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO = 1000108003,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES = 1000253000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES = 1000175000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES = 1000241000,
    VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT = 1000241001,
    VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT = 1000241002,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES = 1000261000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES = 1000207000,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES = 1000207001,
    VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO = 1000207002,
    VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO = 1000207003,
    VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO = 1000207004,
    VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO = 1000207005,
    VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES = 1000257000,
    VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO = 1000244001,
    VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO = 1000257002,
    VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO = 1000257003,
    VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO = 1000257004,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkApplicationInfo.html
#[repr(C)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkInstanceCreateInfo.html
#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkAllocationCallbacks.html
pub enum VkAllocationCallbacks {}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLayerProperties.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkLayerProperties {
    pub layerName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: u32,
    pub implementationVersion: u32,
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE],
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceType.html
#[repr(C)]
pub enum VkPhysicalDeviceType {
    VK_PHYSICAL_DEVICE_TYPE_OTHER = 0,
    VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
    VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
    VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
    VK_PHYSICAL_DEVICE_TYPE_CPU = 4,
    VK_PHYSICAL_DEVICE_TYPE_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceLimits.html
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: VkDeviceSize,
    pub sparseAddressSpaceSize: VkDeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: c_float,
    pub maxSamplerAnisotropy: c_float,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [c_float; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: size_t,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: c_float,
    pub maxInterpolationOffset: c_float,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: VkSampleCountFlags,
    pub framebufferDepthSampleCounts: VkSampleCountFlags,
    pub framebufferStencilSampleCounts: VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
    pub sampledImageDepthSampleCounts: VkSampleCountFlags,
    pub sampledImageStencilSampleCounts: VkSampleCountFlags,
    pub storageImageSampleCounts: VkSampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: VkBool32,
    pub timestampPeriod: c_float,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [c_float; 2],
    pub lineWidthRange: [c_float; 2],
    pub pointSizeGranularity: c_float,
    pub lineWidthGranularity: c_float,
    pub strictLines: VkBool32,
    pub standardSampleLocations: VkBool32,
    pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    pub nonCoherentAtomSize: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceSparseProperties.html
#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceProperties.html
#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties2.html
#[repr(C)]
pub struct VkPhysicalDeviceProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub properties: VkPhysicalDeviceProperties,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDeviceQueueCreateInfo.html
#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const c_float,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueueFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkQueueFlagBits {
    VK_QUEUE_GRAPHICS_BIT = 0x00000001,
    VK_QUEUE_COMPUTE_BIT = 0x00000002,
    VK_QUEUE_TRANSFER_BIT = 0x00000004,
    VK_QUEUE_SPARSE_BINDING_BIT = 0x00000008,
    VK_QUEUE_PROTECTED_BIT = 0x00000010,
    VK_QUEUE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandPoolCreateFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkCommandPoolCreateFlagBits {
    VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = 0x00000001,
    VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 0x00000002,
    VK_COMMAND_POOL_CREATE_PROTECTED_BIT = 0x00000004,
    VK_COMMAND_POOL_CREATE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueueFamilyProperties.html
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceFeatures.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFeatures {
    pub robustBufferAccess: VkBool32,
    pub fullDrawIndexUint32: VkBool32,
    pub imageCubeArray: VkBool32,
    pub independentBlend: VkBool32,
    pub geometryShader: VkBool32,
    pub tessellationShader: VkBool32,
    pub sampleRateShading: VkBool32,
    pub dualSrcBlend: VkBool32,
    pub logicOp: VkBool32,
    pub multiDrawIndirect: VkBool32,
    pub drawIndirectFirstInstance: VkBool32,
    pub depthClamp: VkBool32,
    pub depthBiasClamp: VkBool32,
    pub fillModeNonSolid: VkBool32,
    pub depthBounds: VkBool32,
    pub wideLines: VkBool32,
    pub largePoints: VkBool32,
    pub alphaToOne: VkBool32,
    pub multiViewport: VkBool32,
    pub samplerAnisotropy: VkBool32,
    pub textureCompressionETC2: VkBool32,
    pub textureCompressionASTC_LDR: VkBool32,
    pub textureCompressionBC: VkBool32,
    pub occlusionQueryPrecise: VkBool32,
    pub pipelineStatisticsQuery: VkBool32,
    pub vertexPipelineStoresAndAtomics: VkBool32,
    pub fragmentStoresAndAtomics: VkBool32,
    pub shaderTessellationAndGeometryPointSize: VkBool32,
    pub shaderImageGatherExtended: VkBool32,
    pub shaderStorageImageExtendedFormats: VkBool32,
    pub shaderStorageImageMultisample: VkBool32,
    pub shaderStorageImageReadWithoutFormat: VkBool32,
    pub shaderStorageImageWriteWithoutFormat: VkBool32,
    pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
    pub shaderSampledImageArrayDynamicIndexing: VkBool32,
    pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageImageArrayDynamicIndexing: VkBool32,
    pub shaderClipDistance: VkBool32,
    pub shaderCullDistance: VkBool32,
    pub shaderFloat64: VkBool32,
    pub shaderInt64: VkBool32,
    pub shaderInt16: VkBool32,
    pub shaderResourceResidency: VkBool32,
    pub shaderResourceMinLod: VkBool32,
    pub sparseBinding: VkBool32,
    pub sparseResidencyBuffer: VkBool32,
    pub sparseResidencyImage2D: VkBool32,
    pub sparseResidencyImage3D: VkBool32,
    pub sparseResidency2Samples: VkBool32,
    pub sparseResidency4Samples: VkBool32,
    pub sparseResidency8Samples: VkBool32,
    pub sparseResidency16Samples: VkBool32,
    pub sparseResidencyAliased: VkBool32,
    pub variableMultisampleRate: VkBool32,
    pub inheritedQueries: VkBool32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFeatures2.html
#[repr(C)]
pub struct VkPhysicalDeviceFeatures2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub features: VkPhysicalDeviceFeatures,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDeviceCreateInfo.html
#[repr(C)]
pub struct VkDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeatures.html
#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub bufferDeviceAddress: VkBool32,
    pub bufferDeviceAddressCaptureReplay: VkBool32,
    pub bufferDeviceAddressMultiDevice: VkBool32,
}

// @see https://khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeatures.html
#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
    pub descriptorBindingPartiallyBound: VkBool32,
    pub descriptorBindingVariableDescriptorCount: VkBool32,
    pub runtimeDescriptorArray: VkBool32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandPoolCreateInfo.html
#[repr(C)]
pub struct VkCommandPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferCreateInfo.html
#[repr(C)]
pub struct VkBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryPropertyFlagBits.html
#[repr(C)]
pub enum VkMemoryPropertyFlagBits {
    VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001,
    VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002,
    VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004,
    VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008,
    VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010,
    VK_MEMORY_PROPERTY_PROTECTED_BIT = 0x00000020,
    VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD = 0x00000040,
    VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD = 0x00000080,
    VK_MEMORY_PROPERTY_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryHeapFlagBits.html
#[repr(C)]
pub enum VkMemoryHeapFlagBits {
    VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x00000001,
    VK_MEMORY_HEAP_MULTI_INSTANCE_BIT = 0x00000002,
    VK_MEMORY_HEAP_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryType.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryHeap.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPhysicalDeviceMemoryProperties.html
#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryRequirements.html
#[repr(C)]
#[derive(Debug)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMemoryAllocateInfo.html
#[repr(C)]
pub struct VkMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeIndex: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferUsageFlagBits.html
#[repr(C)]
pub enum VkBufferUsageFlagBits {
    VK_BUFFER_USAGE_TRANSFER_SRC_BIT = 0x00000001,
    VK_BUFFER_USAGE_TRANSFER_DST_BIT = 0x00000002,
    VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000004,
    VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 0x00000008,
    VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = 0x00000010,
    VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = 0x00000020,
    VK_BUFFER_USAGE_INDEX_BUFFER_BIT = 0x00000040,
    VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = 0x00000080,
    VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = 0x00000100,
  // Provided by VK_VERSION_1_2
    VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT = 0x00020000,
  // Provided by VK_EXT_transform_feedback
    VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT = 0x00000800,
  // Provided by VK_EXT_transform_feedback
    VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT = 0x00001000,
  // Provided by VK_EXT_conditional_rendering
    VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT = 0x00000200,
  // Provided by VK_KHR_acceleration_structure
    VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR = 0x00080000,
  // Provided by VK_KHR_acceleration_structure
    VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR = 0x00100000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR = 0x00000400,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkMappedMemoryRange.html
#[repr(C)]
pub struct VkMappedMemoryRange {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferLevel.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkCommandBufferLevel {
    VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
    VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1,
    VK_COMMAND_BUFFER_LEVEL_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferAllocateInfo.html
#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferInheritanceInfo.html
#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusionQueryEnable: VkBool32,
    pub queryFlags: VkQueryControlFlags,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkQueryPipelineStatisticFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkQueryPipelineStatisticFlagBits {
    VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = 0x00000001,
    VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = 0x00000002,
    VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = 0x00000004,
    VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = 0x00000008,
    VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = 0x00000010,
    VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = 0x00000020,
    VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = 0x00000040,
    VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = 0x00000080,
    VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 0x00000100,
    VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 0x00000200,
    VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = 0x00000400,
    VK_QUERY_PIPELINE_STATISTIC_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCommandBufferBeginInfo.html
#[repr(C)]
pub struct VkCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferUsageFlagBits.html
#[repr(C)]
pub enum VkCommandBufferUsageFlagBits {
    VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 0x00000001,
    VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 0x00000002,
    VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 0x00000004,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferCopy.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferCopy {
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkFenceCreateInfo.html
#[repr(C)]
pub struct VkFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFenceCreateFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSubmitInfo.html
#[repr(C)]
pub struct VkSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub pWaitDstStageMask: *const VkPipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const VkCommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorType.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkDescriptorType {
    VK_DESCRIPTOR_TYPE_SAMPLER = 0,
    VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = 1,
    VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE = 2,
    VK_DESCRIPTOR_TYPE_STORAGE_IMAGE = 3,
    VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER = 4,
    VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER = 5,
    VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER = 6,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER = 7,
    VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC = 8,
    VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC = 9,
    VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT = 10,
  // Provided by VK_EXT_inline_uniform_block
    VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT = 1000138000,
  // Provided by VK_KHR_acceleration_structure
    VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR = 1000150000,
  // Provided by VK_NV_ray_tracing
    VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV = 1000165000,
  // Provided by VK_VALVE_mutable_descriptor_type
    VK_DESCRIPTOR_TYPE_MUTABLE_VALVE = 1000351000,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorPoolSize.html
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolSize {
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorPoolCreateInfo.html
#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const VkDescriptorPoolSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorSetLayoutBinding.html
#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: VkShaderStageFlags,
    pub pImmutableSamplers: *const VkSampler,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html
#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineLayoutCreateInfo.html
#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPushConstantRange.html
#[repr(C)]
pub struct VkPushConstantRange {
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorSetAllocateInfo.html
#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkWriteDescriptorSet.html
#[repr(C)]
pub struct VkWriteDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub pImageInfo: *const VkDescriptorImageInfo,
    pub pBufferInfo: *const VkDescriptorBufferInfo,
    pub pTexelBufferView: *const VkBufferView,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorImageInfo.html
#[repr(C)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorBufferInfo.html
#[repr(C)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkCopyDescriptorSet.html
#[repr(C)]
pub struct VkCopyDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcSet: VkDescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkImageLayout.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkImageLayout {
    VK_IMAGE_LAYOUT_UNDEFINED = 0,
    VK_IMAGE_LAYOUT_GENERAL = 1,
    VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
    VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
    VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,
    VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6,
    VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7,
    VK_IMAGE_LAYOUT_PREINITIALIZED = 8,
    VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL = 1000117000,
    VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL = 1000117001,
    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR = 1000001002,
    VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR = 1000111000,
    VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV = 1000164003,
    VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT = 1000218000,
    VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL_KHR = 1000241000,
    VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR = 1000241001,
    VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL_KHR = 1000241002,
    VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL_KHR = 1000241003,
    VK_IMAGE_LAYOUT_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderStageFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkShaderStageFlagBits {
    VK_SHADER_STAGE_VERTEX_BIT = 0x00000001,
    VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = 0x00000002,
    VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 0x00000004,
    VK_SHADER_STAGE_GEOMETRY_BIT = 0x00000008,
    VK_SHADER_STAGE_FRAGMENT_BIT = 0x00000010,
    VK_SHADER_STAGE_COMPUTE_BIT = 0x00000020,
    VK_SHADER_STAGE_ALL_GRAPHICS = 0x0000001F,
    VK_SHADER_STAGE_ALL = 0x7FFFFFFF,
    VK_SHADER_STAGE_RAYGEN_BIT_KHR = 0x00000100,
    VK_SHADER_STAGE_ANY_HIT_BIT_KHR = 0x00000200,
    VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR = 0x00000400,
    VK_SHADER_STAGE_MISS_BIT_KHR = 0x00000800,
    VK_SHADER_STAGE_INTERSECTION_BIT_KHR = 0x00001000,
    VK_SHADER_STAGE_CALLABLE_BIT_KHR = 0x00002000,
    VK_SHADER_STAGE_TASK_BIT_NV = 0x00000040,
    VK_SHADER_STAGE_MESH_BIT_NV = 0x00000080,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkComputePipelineCreateInfo.html
#[repr(C)]
pub struct VkComputePipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineShaderStageCreateInfo.html
#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlagBits,
    pub module: VkShaderModule,
    pub pName: *const c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPipelineCacheCreateInfo.html
#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initialDataSize: size_t,
    pub pInitialData: *const c_void,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSpecializationInfo.html
#[repr(C)]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: size_t,
    pub pData: *const c_void,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkSpecializationMapEntry.html
#[repr(C)]
pub struct VkSpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: size_t,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkShaderModuleCreateInfo.html
#[repr(C)]
pub struct VkShaderModuleCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub codeSize: size_t,
    pub pCode: *const u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkFenceCreateFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkFenceCreateFlagBits {
    VK_FENCE_CREATE_SIGNALED_BIT = 0x00000001,
    VK_FENCE_CREATE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccessFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkAccessFlagBits {
    VK_ACCESS_INDIRECT_COMMAND_READ_BIT = 0x00000001,
    VK_ACCESS_INDEX_READ_BIT = 0x00000002,
    VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 0x00000004,
    VK_ACCESS_UNIFORM_READ_BIT = 0x00000008,
    VK_ACCESS_INPUT_ATTACHMENT_READ_BIT = 0x00000010,
    VK_ACCESS_SHADER_READ_BIT = 0x00000020,
    VK_ACCESS_SHADER_WRITE_BIT = 0x00000040,
    VK_ACCESS_COLOR_ATTACHMENT_READ_BIT = 0x00000080,
    VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 0x00000100,
    VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200,
    VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400,
    VK_ACCESS_TRANSFER_READ_BIT = 0x00000800,
    VK_ACCESS_TRANSFER_WRITE_BIT = 0x00001000,
    VK_ACCESS_HOST_READ_BIT = 0x00002000,
    VK_ACCESS_HOST_WRITE_BIT = 0x00004000,
    VK_ACCESS_MEMORY_READ_BIT = 0x00008000,
    VK_ACCESS_MEMORY_WRITE_BIT = 0x00010000,
  // Provided by VK_EXT_transform_feedback
    VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 0x02000000,
  // Provided by VK_EXT_transform_feedback
    VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 0x04000000,
  // Provided by VK_EXT_transform_feedback
    VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 0x08000000,
  // Provided by VK_EXT_conditional_rendering
    VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT = 0x00100000,
  // Provided by VK_EXT_blend_operation_advanced
    VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 0x00080000,
  // Provided by VK_KHR_acceleration_structure
    VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR = 0x00200000,
  // Provided by VK_KHR_acceleration_structure
    VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 0x00400000,
  // Provided by VK_EXT_fragment_density_map
    VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 0x01000000,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineStageFlagBits.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkPipelineStageFlagBits {
    VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT = 0x00000001,
    VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT = 0x00000002,
    VK_PIPELINE_STAGE_VERTEX_INPUT_BIT = 0x00000004,
    VK_PIPELINE_STAGE_VERTEX_SHADER_BIT = 0x00000008,
    VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 0x00000010,
    VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020,
    VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 0x00000040,
    VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 0x00000080,
    VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 0x00000100,
    VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 0x00000200,
    VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400,
    VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT = 0x00000800,
    VK_PIPELINE_STAGE_TRANSFER_BIT = 0x00001000,
    VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 0x00002000,
    VK_PIPELINE_STAGE_HOST_BIT = 0x00004000,
    VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT = 0x00008000,
    VK_PIPELINE_STAGE_ALL_COMMANDS_BIT = 0x00010000,
  // Provided by VK_EXT_transform_feedback
    VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT = 0x01000000,
  // Provided by VK_EXT_conditional_rendering
    VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT = 0x00040000,
  // Provided by VK_KHR_acceleration_structure
    VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR = 0x02000000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR = 0x00200000,
  // Provided by VK_EXT_fragment_density_map
    VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT = 0x00800000,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkBufferMemoryBarrier.html
#[repr(C)]
pub struct VkBufferMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

// @see http://khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineBindPoint.html
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VkPipelineBindPoint {
    VK_PIPELINE_BIND_POINT_GRAPHICS = 0,
    VK_PIPELINE_BIND_POINT_COMPUTE = 1,
    VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR = 1000165000,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtensionProperties.html
#[repr(C)]
#[derive(Clone)]
pub struct VkExtensionProperties {
    pub extensionName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormat.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkFormat {
    VK_FORMAT_UNDEFINED = 0,
    VK_FORMAT_R4G4_UNORM_PACK8 = 1,
    VK_FORMAT_R4G4B4A4_UNORM_PACK16 = 2,
    VK_FORMAT_B4G4R4A4_UNORM_PACK16 = 3,
    VK_FORMAT_R5G6B5_UNORM_PACK16 = 4,
    VK_FORMAT_B5G6R5_UNORM_PACK16 = 5,
    VK_FORMAT_R5G5B5A1_UNORM_PACK16 = 6,
    VK_FORMAT_B5G5R5A1_UNORM_PACK16 = 7,
    VK_FORMAT_A1R5G5B5_UNORM_PACK16 = 8,
    VK_FORMAT_R8_UNORM = 9,
    VK_FORMAT_R8_SNORM = 10,
    VK_FORMAT_R8_USCALED = 11,
    VK_FORMAT_R8_SSCALED = 12,
    VK_FORMAT_R8_UINT = 13,
    VK_FORMAT_R8_SINT = 14,
    VK_FORMAT_R8_SRGB = 15,
    VK_FORMAT_R8G8_UNORM = 16,
    VK_FORMAT_R8G8_SNORM = 17,
    VK_FORMAT_R8G8_USCALED = 18,
    VK_FORMAT_R8G8_SSCALED = 19,
    VK_FORMAT_R8G8_UINT = 20,
    VK_FORMAT_R8G8_SINT = 21,
    VK_FORMAT_R8G8_SRGB = 22,
    VK_FORMAT_R8G8B8_UNORM = 23,
    VK_FORMAT_R8G8B8_SNORM = 24,
    VK_FORMAT_R8G8B8_USCALED = 25,
    VK_FORMAT_R8G8B8_SSCALED = 26,
    VK_FORMAT_R8G8B8_UINT = 27,
    VK_FORMAT_R8G8B8_SINT = 28,
    VK_FORMAT_R8G8B8_SRGB = 29,
    VK_FORMAT_B8G8R8_UNORM = 30,
    VK_FORMAT_B8G8R8_SNORM = 31,
    VK_FORMAT_B8G8R8_USCALED = 32,
    VK_FORMAT_B8G8R8_SSCALED = 33,
    VK_FORMAT_B8G8R8_UINT = 34,
    VK_FORMAT_B8G8R8_SINT = 35,
    VK_FORMAT_B8G8R8_SRGB = 36,
    VK_FORMAT_R8G8B8A8_UNORM = 37,
    VK_FORMAT_R8G8B8A8_SNORM = 38,
    VK_FORMAT_R8G8B8A8_USCALED = 39,
    VK_FORMAT_R8G8B8A8_SSCALED = 40,
    VK_FORMAT_R8G8B8A8_UINT = 41,
    VK_FORMAT_R8G8B8A8_SINT = 42,
    VK_FORMAT_R8G8B8A8_SRGB = 43,
    VK_FORMAT_B8G8R8A8_UNORM = 44,
    VK_FORMAT_B8G8R8A8_SNORM = 45,
    VK_FORMAT_B8G8R8A8_USCALED = 46,
    VK_FORMAT_B8G8R8A8_SSCALED = 47,
    VK_FORMAT_B8G8R8A8_UINT = 48,
    VK_FORMAT_B8G8R8A8_SINT = 49,
    VK_FORMAT_B8G8R8A8_SRGB = 50,
    VK_FORMAT_A8B8G8R8_UNORM_PACK32 = 51,
    VK_FORMAT_A8B8G8R8_SNORM_PACK32 = 52,
    VK_FORMAT_A8B8G8R8_USCALED_PACK32 = 53,
    VK_FORMAT_A8B8G8R8_SSCALED_PACK32 = 54,
    VK_FORMAT_A8B8G8R8_UINT_PACK32 = 55,
    VK_FORMAT_A8B8G8R8_SINT_PACK32 = 56,
    VK_FORMAT_A8B8G8R8_SRGB_PACK32 = 57,
    VK_FORMAT_A2R10G10B10_UNORM_PACK32 = 58,
    VK_FORMAT_A2R10G10B10_SNORM_PACK32 = 59,
    VK_FORMAT_A2R10G10B10_USCALED_PACK32 = 60,
    VK_FORMAT_A2R10G10B10_SSCALED_PACK32 = 61,
    VK_FORMAT_A2R10G10B10_UINT_PACK32 = 62,
    VK_FORMAT_A2R10G10B10_SINT_PACK32 = 63,
    VK_FORMAT_A2B10G10R10_UNORM_PACK32 = 64,
    VK_FORMAT_A2B10G10R10_SNORM_PACK32 = 65,
    VK_FORMAT_A2B10G10R10_USCALED_PACK32 = 66,
    VK_FORMAT_A2B10G10R10_SSCALED_PACK32 = 67,
    VK_FORMAT_A2B10G10R10_UINT_PACK32 = 68,
    VK_FORMAT_A2B10G10R10_SINT_PACK32 = 69,
    VK_FORMAT_R16_UNORM = 70,
    VK_FORMAT_R16_SNORM = 71,
    VK_FORMAT_R16_USCALED = 72,
    VK_FORMAT_R16_SSCALED = 73,
    VK_FORMAT_R16_UINT = 74,
    VK_FORMAT_R16_SINT = 75,
    VK_FORMAT_R16_SFLOAT = 76,
    VK_FORMAT_R16G16_UNORM = 77,
    VK_FORMAT_R16G16_SNORM = 78,
    VK_FORMAT_R16G16_USCALED = 79,
    VK_FORMAT_R16G16_SSCALED = 80,
    VK_FORMAT_R16G16_UINT = 81,
    VK_FORMAT_R16G16_SINT = 82,
    VK_FORMAT_R16G16_SFLOAT = 83,
    VK_FORMAT_R16G16B16_UNORM = 84,
    VK_FORMAT_R16G16B16_SNORM = 85,
    VK_FORMAT_R16G16B16_USCALED = 86,
    VK_FORMAT_R16G16B16_SSCALED = 87,
    VK_FORMAT_R16G16B16_UINT = 88,
    VK_FORMAT_R16G16B16_SINT = 89,
    VK_FORMAT_R16G16B16_SFLOAT = 90,
    VK_FORMAT_R16G16B16A16_UNORM = 91,
    VK_FORMAT_R16G16B16A16_SNORM = 92,
    VK_FORMAT_R16G16B16A16_USCALED = 93,
    VK_FORMAT_R16G16B16A16_SSCALED = 94,
    VK_FORMAT_R16G16B16A16_UINT = 95,
    VK_FORMAT_R16G16B16A16_SINT = 96,
    VK_FORMAT_R16G16B16A16_SFLOAT = 97,
    VK_FORMAT_R32_UINT = 98,
    VK_FORMAT_R32_SINT = 99,
    VK_FORMAT_R32_SFLOAT = 100,
    VK_FORMAT_R32G32_UINT = 101,
    VK_FORMAT_R32G32_SINT = 102,
    VK_FORMAT_R32G32_SFLOAT = 103,
    VK_FORMAT_R32G32B32_UINT = 104,
    VK_FORMAT_R32G32B32_SINT = 105,
    VK_FORMAT_R32G32B32_SFLOAT = 106,
    VK_FORMAT_R32G32B32A32_UINT = 107,
    VK_FORMAT_R32G32B32A32_SINT = 108,
    VK_FORMAT_R32G32B32A32_SFLOAT = 109,
    VK_FORMAT_R64_UINT = 110,
    VK_FORMAT_R64_SINT = 111,
    VK_FORMAT_R64_SFLOAT = 112,
    VK_FORMAT_R64G64_UINT = 113,
    VK_FORMAT_R64G64_SINT = 114,
    VK_FORMAT_R64G64_SFLOAT = 115,
    VK_FORMAT_R64G64B64_UINT = 116,
    VK_FORMAT_R64G64B64_SINT = 117,
    VK_FORMAT_R64G64B64_SFLOAT = 118,
    VK_FORMAT_R64G64B64A64_UINT = 119,
    VK_FORMAT_R64G64B64A64_SINT = 120,
    VK_FORMAT_R64G64B64A64_SFLOAT = 121,
    VK_FORMAT_B10G11R11_UFLOAT_PACK32 = 122,
    VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 = 123,
    VK_FORMAT_D16_UNORM = 124,
    VK_FORMAT_X8_D24_UNORM_PACK32 = 125,
    VK_FORMAT_D32_SFLOAT = 126,
    VK_FORMAT_S8_UINT = 127,
    VK_FORMAT_D16_UNORM_S8_UINT = 128,
    VK_FORMAT_D24_UNORM_S8_UINT = 129,
    VK_FORMAT_D32_SFLOAT_S8_UINT = 130,
    VK_FORMAT_BC1_RGB_UNORM_BLOCK = 131,
    VK_FORMAT_BC1_RGB_SRGB_BLOCK = 132,
    VK_FORMAT_BC1_RGBA_UNORM_BLOCK = 133,
    VK_FORMAT_BC1_RGBA_SRGB_BLOCK = 134,
    VK_FORMAT_BC2_UNORM_BLOCK = 135,
    VK_FORMAT_BC2_SRGB_BLOCK = 136,
    VK_FORMAT_BC3_UNORM_BLOCK = 137,
    VK_FORMAT_BC3_SRGB_BLOCK = 138,
    VK_FORMAT_BC4_UNORM_BLOCK = 139,
    VK_FORMAT_BC4_SNORM_BLOCK = 140,
    VK_FORMAT_BC5_UNORM_BLOCK = 141,
    VK_FORMAT_BC5_SNORM_BLOCK = 142,
    VK_FORMAT_BC6H_UFLOAT_BLOCK = 143,
    VK_FORMAT_BC6H_SFLOAT_BLOCK = 144,
    VK_FORMAT_BC7_UNORM_BLOCK = 145,
    VK_FORMAT_BC7_SRGB_BLOCK = 146,
    VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK = 147,
    VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK = 148,
    VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK = 149,
    VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK = 150,
    VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK = 151,
    VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK = 152,
    VK_FORMAT_EAC_R11_UNORM_BLOCK = 153,
    VK_FORMAT_EAC_R11_SNORM_BLOCK = 154,
    VK_FORMAT_EAC_R11G11_UNORM_BLOCK = 155,
    VK_FORMAT_EAC_R11G11_SNORM_BLOCK = 156,
    VK_FORMAT_ASTC_4x4_UNORM_BLOCK = 157,
    VK_FORMAT_ASTC_4x4_SRGB_BLOCK = 158,
    VK_FORMAT_ASTC_5x4_UNORM_BLOCK = 159,
    VK_FORMAT_ASTC_5x4_SRGB_BLOCK = 160,
    VK_FORMAT_ASTC_5x5_UNORM_BLOCK = 161,
    VK_FORMAT_ASTC_5x5_SRGB_BLOCK = 162,
    VK_FORMAT_ASTC_6x5_UNORM_BLOCK = 163,
    VK_FORMAT_ASTC_6x5_SRGB_BLOCK = 164,
    VK_FORMAT_ASTC_6x6_UNORM_BLOCK = 165,
    VK_FORMAT_ASTC_6x6_SRGB_BLOCK = 166,
    VK_FORMAT_ASTC_8x5_UNORM_BLOCK = 167,
    VK_FORMAT_ASTC_8x5_SRGB_BLOCK = 168,
    VK_FORMAT_ASTC_8x6_UNORM_BLOCK = 169,
    VK_FORMAT_ASTC_8x6_SRGB_BLOCK = 170,
    VK_FORMAT_ASTC_8x8_UNORM_BLOCK = 171,
    VK_FORMAT_ASTC_8x8_SRGB_BLOCK = 172,
    VK_FORMAT_ASTC_10x5_UNORM_BLOCK = 173,
    VK_FORMAT_ASTC_10x5_SRGB_BLOCK = 174,
    VK_FORMAT_ASTC_10x6_UNORM_BLOCK = 175,
    VK_FORMAT_ASTC_10x6_SRGB_BLOCK = 176,
    VK_FORMAT_ASTC_10x8_UNORM_BLOCK = 177,
    VK_FORMAT_ASTC_10x8_SRGB_BLOCK = 178,
    VK_FORMAT_ASTC_10x10_UNORM_BLOCK = 179,
    VK_FORMAT_ASTC_10x10_SRGB_BLOCK = 180,
    VK_FORMAT_ASTC_12x10_UNORM_BLOCK = 181,
    VK_FORMAT_ASTC_12x10_SRGB_BLOCK = 182,
    VK_FORMAT_ASTC_12x12_UNORM_BLOCK = 183,
    VK_FORMAT_ASTC_12x12_SRGB_BLOCK = 184,
    VK_FORMAT_MAX_ENUM = 0x7FFFFFFF,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageUsageFlagBits.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkImageUsageFlagBits {
    VK_IMAGE_USAGE_TRANSFER_SRC_BIT = 0x00000001,
    VK_IMAGE_USAGE_TRANSFER_DST_BIT = 0x00000002,
    VK_IMAGE_USAGE_SAMPLED_BIT = 0x00000004,
    VK_IMAGE_USAGE_STORAGE_BIT = 0x00000008,
    VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 0x00000010,
    VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020,
    VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 0x00000040,
    VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 0x00000080,
    VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV = 0x00000100,
    VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT = 0x00000200,
    VK_IMAGE_USAGE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageAspectFlagBits.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkImageAspectFlagBits {
    VK_IMAGE_ASPECT_COLOR_BIT = 0x00000001,
    VK_IMAGE_ASPECT_DEPTH_BIT = 0x00000002,
    VK_IMAGE_ASPECT_STENCIL_BIT = 0x00000004,
    VK_IMAGE_ASPECT_METADATA_BIT = 0x00000008,
    VK_IMAGE_ASPECT_PLANE_0_BIT = 0x00000010,
    VK_IMAGE_ASPECT_PLANE_1_BIT = 0x00000020,
    VK_IMAGE_ASPECT_PLANE_2_BIT = 0x00000040,
    VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT = 0x00000080,
    VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT = 0x00000100,
    VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT = 0x00000200,
    VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT = 0x00000400,
    VK_IMAGE_ASPECT_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewType.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkImageViewType {
    VK_IMAGE_VIEW_TYPE_1D = 0,
    VK_IMAGE_VIEW_TYPE_2D = 1,
    VK_IMAGE_VIEW_TYPE_3D = 2,
    VK_IMAGE_VIEW_TYPE_CUBE = 3,
    VK_IMAGE_VIEW_TYPE_1D_ARRAY = 4,
    VK_IMAGE_VIEW_TYPE_2D_ARRAY = 5,
    VK_IMAGE_VIEW_TYPE_CUBE_ARRAY = 6,
    VK_IMAGE_VIEW_TYPE_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentSwizzle.html
#[repr(C)]
pub enum VkComponentSwizzle {
    VK_COMPONENT_SWIZZLE_IDENTITY = 0,
    VK_COMPONENT_SWIZZLE_ZERO = 1,
    VK_COMPONENT_SWIZZLE_ONE = 2,
    VK_COMPONENT_SWIZZLE_R = 3,
    VK_COMPONENT_SWIZZLE_G = 4,
    VK_COMPONENT_SWIZZLE_B = 5,
    VK_COMPONENT_SWIZZLE_A = 6,
    VK_COMPONENT_SWIZZLE_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentMapping.html
#[repr(C)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewCreateInfo.html
#[repr(C)]
pub struct VkImageViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageViewCreateFlags,
    pub image: VkImage,
    pub viewType: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresourceRange: VkImageSubresourceRange,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresourceRange.html
#[repr(C)]
#[derive(Clone)]
pub struct VkImageSubresourceRange {
    pub aspectMask: VkImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageType.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkImageType {
    VK_IMAGE_TYPE_1D = 0,
    VK_IMAGE_TYPE_2D = 1,
    VK_IMAGE_TYPE_3D = 2,
    VK_IMAGE_TYPE_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleCountFlagBits.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkSampleCountFlagBits {
    VK_SAMPLE_COUNT_1_BIT = 0x00000001,
    VK_SAMPLE_COUNT_2_BIT = 0x00000002,
    VK_SAMPLE_COUNT_4_BIT = 0x00000004,
    VK_SAMPLE_COUNT_8_BIT = 0x00000008,
    VK_SAMPLE_COUNT_16_BIT = 0x00000010,
    VK_SAMPLE_COUNT_32_BIT = 0x00000020,
    VK_SAMPLE_COUNT_64_BIT = 0x00000040,
    VK_SAMPLE_COUNT_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageTiling.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkImageTiling {
    VK_IMAGE_TILING_OPTIMAL = 0,
    VK_IMAGE_TILING_LINEAR = 1,
    VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT = 1000158000,
    VK_IMAGE_TILING_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCreateInfo.html
#[repr(C)]
pub struct VkImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageCreateFlags,
    pub imageType: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: VkSampleCountFlagBits,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: VkImageLayout,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescription.html
#[repr(C)]
pub struct VkAttachmentDescription {
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub stencilLoadOp: VkAttachmentLoadOp,
    pub stencilStoreOp: VkAttachmentStoreOp,
    pub initialLayout: VkImageLayout,
    pub finalLayout: VkImageLayout,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentLoadOp.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkAttachmentLoadOp {
    VK_ATTACHMENT_LOAD_OP_LOAD = 0,
    VK_ATTACHMENT_LOAD_OP_CLEAR = 1,
    VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2,
    VK_ATTACHMENT_LOAD_OP_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentStoreOp.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkAttachmentStoreOp {
    VK_ATTACHMENT_STORE_OP_STORE = 0,
    VK_ATTACHMENT_STORE_OP_DONT_CARE = 1,
    VK_ATTACHMENT_STORE_OP_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReference.html
#[repr(C)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescription.html
#[repr(C)]
pub struct VkSubpassDescription {
    pub flags: VkSubpassDescriptionFlags,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference,
    pub pResolveAttachments: *const VkAttachmentReference,
    pub pDepthStencilAttachment: *const VkAttachmentReference,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDependency.html
#[repr(C)]
pub struct VkSubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateInfo.html
#[repr(C)]
pub struct VkRenderPassCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDependencyFlagBits.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkDependencyFlagBits {
    VK_DEPENDENCY_BY_REGION_BIT = 0x00000001,
    VK_DEPENDENCY_DEVICE_GROUP_BIT = 0x00000004,
    VK_DEPENDENCY_VIEW_LOCAL_BIT = 0x00000002,
    VK_DEPENDENCY_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferCreateInfo.html
#[repr(C)]
pub struct VkFramebufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFramebufferCreateFlags,
    pub renderPass: VkRenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsPipelineCreateInfo.html
#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
    pub pViewportState: *const VkPipelineViewportStateCreateInfo,
    pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html
#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDescription.html
#[repr(C)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VkVertexInputRate,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputRate.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkVertexInputRate {
    VK_VERTEX_INPUT_RATE_VERTEX = 0,
    VK_VERTEX_INPUT_RATE_INSTANCE = 1,
    VK_VERTEX_INPUT_RATE_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputAttributeDescription.html
#[repr(C)]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html
#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlags,
    pub topology: VkPrimitiveTopology,
    pub primitiveRestartEnable: VkBool32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrimitiveTopology.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkPrimitiveTopology {
    VK_PRIMITIVE_TOPOLOGY_POINT_LIST = 0,
    VK_PRIMITIVE_TOPOLOGY_LINE_LIST = 1,
    VK_PRIMITIVE_TOPOLOGY_LINE_STRIP = 2,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST = 3,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP = 4,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN = 5,
    VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY = 6,
    VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY = 7,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY = 8,
    VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY = 9,
    VK_PRIMITIVE_TOPOLOGY_PATCH_LIST = 10,
    VK_PRIMITIVE_TOPOLOGY_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationStateCreateInfo.html
#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportStateCreateInfo.html
#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineViewportStateCreateFlags,
    pub viewportCount: u32,
    pub pViewports: *const VkViewport,
    pub scissorCount: u32,
    pub pScissors: *const VkRect2D,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewport.html
#[repr(C)]
pub struct VkViewport {
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
    pub minDepth: c_float,
    pub maxDepth: c_float,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRect2D.html
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset2D.html
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html
#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineRasterizationStateCreateFlags,
    pub depthClampEnable: VkBool32,
    pub rasterizerDiscardEnable: VkBool32,
    pub polygonMode: VkPolygonMode,
    pub cullMode: VkCullModeFlags,
    pub frontFace: VkFrontFace,
    pub depthBiasEnable: VkBool32,
    pub depthBiasConstantFactor: c_float,
    pub depthBiasClamp: c_float,
    pub depthBiasSlopeFactor: c_float,
    pub lineWidth: c_float,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPolygonMode.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkPolygonMode {
    VK_POLYGON_MODE_FILL = 0,
    VK_POLYGON_MODE_LINE = 1,
    VK_POLYGON_MODE_POINT = 2,
    VK_POLYGON_MODE_FILL_RECTANGLE_NV = 1000153000,
    VK_POLYGON_MODE_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFrontFace.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkFrontFace {
    VK_FRONT_FACE_COUNTER_CLOCKWISE = 0,
    VK_FRONT_FACE_CLOCKWISE = 1,
    VK_FRONT_FACE_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html
#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: VkSampleCountFlagBits,
    pub sampleShadingEnable: VkBool32,
    pub minSampleShading: c_float,
    pub pSampleMask: *const VkSampleMask,
    pub alphaToCoverageEnable: VkBool32,
    pub alphaToOneEnable: VkBool32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html
#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDepthStencilStateCreateFlags,
    pub depthTestEnable: VkBool32,
    pub depthWriteEnable: VkBool32,
    pub depthCompareOp: VkCompareOp,
    pub depthBoundsTestEnable: VkBool32,
    pub stencilTestEnable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub minDepthBounds: c_float,
    pub maxDepthBounds: c_float,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompareOp.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkCompareOp {
    VK_COMPARE_OP_NEVER = 0,
    VK_COMPARE_OP_LESS = 1,
    VK_COMPARE_OP_EQUAL = 2,
    VK_COMPARE_OP_LESS_OR_EQUAL = 3,
    VK_COMPARE_OP_GREATER = 4,
    VK_COMPARE_OP_NOT_EQUAL = 5,
    VK_COMPARE_OP_GREATER_OR_EQUAL = 6,
    VK_COMPARE_OP_ALWAYS = 7,
    VK_COMPARE_OP_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilOpState.html
#[repr(C)]
pub struct VkStencilOpState {
    pub failOp: VkStencilOp,
    pub passOp: VkStencilOp,
    pub depthFailOp: VkStencilOp,
    pub compareOp: VkCompareOp,
    pub compareMask: u32,
    pub writeMask: u32,
    pub reference: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilOp.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkStencilOp {
    VK_STENCIL_OP_KEEP = 0,
    VK_STENCIL_OP_ZERO = 1,
    VK_STENCIL_OP_REPLACE = 2,
    VK_STENCIL_OP_INCREMENT_AND_CLAMP = 3,
    VK_STENCIL_OP_DECREMENT_AND_CLAMP = 4,
    VK_STENCIL_OP_INVERT = 5,
    VK_STENCIL_OP_INCREMENT_AND_WRAP = 6,
    VK_STENCIL_OP_DECREMENT_AND_WRAP = 7,
    VK_STENCIL_OP_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html
#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineColorBlendStateCreateFlags,
    pub logicOpEnable: VkBool32,
    pub logicOp: VkLogicOp,
    pub attachmentCount: u32,
    pub pAttachments: *const VkPipelineColorBlendAttachmentState,
    pub blendConstants: [c_float; 4],
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLogicOp.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkLogicOp {
    VK_LOGIC_OP_CLEAR = 0,
    VK_LOGIC_OP_AND = 1,
    VK_LOGIC_OP_AND_REVERSE = 2,
    VK_LOGIC_OP_COPY = 3,
    VK_LOGIC_OP_AND_INVERTED = 4,
    VK_LOGIC_OP_NO_OP = 5,
    VK_LOGIC_OP_XOR = 6,
    VK_LOGIC_OP_OR = 7,
    VK_LOGIC_OP_NOR = 8,
    VK_LOGIC_OP_EQUIVALENT = 9,
    VK_LOGIC_OP_INVERT = 10,
    VK_LOGIC_OP_OR_REVERSE = 11,
    VK_LOGIC_OP_COPY_INVERTED = 12,
    VK_LOGIC_OP_OR_INVERTED = 13,
    VK_LOGIC_OP_NAND = 14,
    VK_LOGIC_OP_SET = 15,
    VK_LOGIC_OP_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendAttachmentState.html
#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blendEnable: VkBool32,
    pub srcColorBlendFactor: VkBlendFactor,
    pub dstColorBlendFactor: VkBlendFactor,
    pub colorBlendOp: VkBlendOp,
    pub srcAlphaBlendFactor: VkBlendFactor,
    pub dstAlphaBlendFactor: VkBlendFactor,
    pub alphaBlendOp: VkBlendOp,
    pub colorWriteMask: VkColorComponentFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendFactor.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkBlendFactor {
    VK_BLEND_FACTOR_ZERO = 0,
    VK_BLEND_FACTOR_ONE = 1,
    VK_BLEND_FACTOR_SRC_COLOR = 2,
    VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR = 3,
    VK_BLEND_FACTOR_DST_COLOR = 4,
    VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR = 5,
    VK_BLEND_FACTOR_SRC_ALPHA = 6,
    VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA = 7,
    VK_BLEND_FACTOR_DST_ALPHA = 8,
    VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA = 9,
    VK_BLEND_FACTOR_CONSTANT_COLOR = 10,
    VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR = 11,
    VK_BLEND_FACTOR_CONSTANT_ALPHA = 12,
    VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA = 13,
    VK_BLEND_FACTOR_SRC_ALPHA_SATURATE = 14,
    VK_BLEND_FACTOR_SRC1_COLOR = 15,
    VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR = 16,
    VK_BLEND_FACTOR_SRC1_ALPHA = 17,
    VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA = 18,
    VK_BLEND_FACTOR_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendOp.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkBlendOp {
    VK_BLEND_OP_ADD = 0,
    VK_BLEND_OP_SUBTRACT = 1,
    VK_BLEND_OP_REVERSE_SUBTRACT = 2,
    VK_BLEND_OP_MIN = 3,
    VK_BLEND_OP_MAX = 4,
    VK_BLEND_OP_ZERO_EXT = 1000148000,
    VK_BLEND_OP_SRC_EXT = 1000148001,
    VK_BLEND_OP_DST_EXT = 1000148002,
    VK_BLEND_OP_SRC_OVER_EXT = 1000148003,
    VK_BLEND_OP_DST_OVER_EXT = 1000148004,
    VK_BLEND_OP_SRC_IN_EXT = 1000148005,
    VK_BLEND_OP_DST_IN_EXT = 1000148006,
    VK_BLEND_OP_SRC_OUT_EXT = 1000148007,
    VK_BLEND_OP_DST_OUT_EXT = 1000148008,
    VK_BLEND_OP_SRC_ATOP_EXT = 1000148009,
    VK_BLEND_OP_DST_ATOP_EXT = 1000148010,
    VK_BLEND_OP_XOR_EXT = 1000148011,
    VK_BLEND_OP_MULTIPLY_EXT = 1000148012,
    VK_BLEND_OP_SCREEN_EXT = 1000148013,
    VK_BLEND_OP_OVERLAY_EXT = 1000148014,
    VK_BLEND_OP_DARKEN_EXT = 1000148015,
    VK_BLEND_OP_LIGHTEN_EXT = 1000148016,
    VK_BLEND_OP_COLORDODGE_EXT = 1000148017,
    VK_BLEND_OP_COLORBURN_EXT = 1000148018,
    VK_BLEND_OP_HARDLIGHT_EXT = 1000148019,
    VK_BLEND_OP_SOFTLIGHT_EXT = 1000148020,
    VK_BLEND_OP_DIFFERENCE_EXT = 1000148021,
    VK_BLEND_OP_EXCLUSION_EXT = 1000148022,
    VK_BLEND_OP_INVERT_EXT = 1000148023,
    VK_BLEND_OP_INVERT_RGB_EXT = 1000148024,
    VK_BLEND_OP_LINEARDODGE_EXT = 1000148025,
    VK_BLEND_OP_LINEARBURN_EXT = 1000148026,
    VK_BLEND_OP_VIVIDLIGHT_EXT = 1000148027,
    VK_BLEND_OP_LINEARLIGHT_EXT = 1000148028,
    VK_BLEND_OP_PINLIGHT_EXT = 1000148029,
    VK_BLEND_OP_HARDMIX_EXT = 1000148030,
    VK_BLEND_OP_HSL_HUE_EXT = 1000148031,
    VK_BLEND_OP_HSL_SATURATION_EXT = 1000148032,
    VK_BLEND_OP_HSL_COLOR_EXT = 1000148033,
    VK_BLEND_OP_HSL_LUMINOSITY_EXT = 1000148034,
    VK_BLEND_OP_PLUS_EXT = 1000148035,
    VK_BLEND_OP_PLUS_CLAMPED_EXT = 1000148036,
    VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT = 1000148037,
    VK_BLEND_OP_PLUS_DARKER_EXT = 1000148038,
    VK_BLEND_OP_MINUS_EXT = 1000148039,
    VK_BLEND_OP_MINUS_CLAMPED_EXT = 1000148040,
    VK_BLEND_OP_CONTRAST_EXT = 1000148041,
    VK_BLEND_OP_INVERT_OVG_EXT = 1000148042,
    VK_BLEND_OP_RED_EXT = 1000148043,
    VK_BLEND_OP_GREEN_EXT = 1000148044,
    VK_BLEND_OP_BLUE_EXT = 1000148045,
    VK_BLEND_OP_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDynamicStateCreateInfo.html
#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const VkDynamicState,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDynamicState.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkDynamicState {
    VK_DYNAMIC_STATE_VIEWPORT = 0,
    VK_DYNAMIC_STATE_SCISSOR = 1,
    VK_DYNAMIC_STATE_LINE_WIDTH = 2,
    VK_DYNAMIC_STATE_DEPTH_BIAS = 3,
    VK_DYNAMIC_STATE_BLEND_CONSTANTS = 4,
    VK_DYNAMIC_STATE_DEPTH_BOUNDS = 5,
    VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK = 6,
    VK_DYNAMIC_STATE_STENCIL_WRITE_MASK = 7,
    VK_DYNAMIC_STATE_STENCIL_REFERENCE = 8,
    VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV = 1000087000,
    VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT = 1000099000,
    VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT = 1000143000,
    VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV = 1000164004,
    VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV = 1000164006,
    VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV = 1000205001,
    VK_DYNAMIC_STATE_LINE_STIPPLE_EXT = 1000259000,
    VK_DYNAMIC_STATE_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCullModeFlagBits.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkCullModeFlagBits {
    VK_CULL_MODE_NONE = 0,
    VK_CULL_MODE_FRONT_BIT = 0x00000001,
    VK_CULL_MODE_BACK_BIT = 0x00000002,
    VK_CULL_MODE_FRONT_AND_BACK = 0x00000003,
    VK_CULL_MODE_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkColorComponentFlagBits.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkColorComponentFlagBits {
    VK_COLOR_COMPONENT_R_BIT = 0x00000001,
    VK_COLOR_COMPONENT_G_BIT = 0x00000002,
    VK_COLOR_COMPONENT_B_BIT = 0x00000004,
    VK_COLOR_COMPONENT_A_BIT = 0x00000008,
    VK_COLOR_COMPONENT_FLAG_BITS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassBeginInfo.html
#[repr(C)]
pub struct VkRenderPassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub framebuffer: VkFramebuffer,
    pub renderArea: VkRect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const VkClearValue,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearValue.html
// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearColorValue.html
#[repr(C)]
pub struct VkClearValue {
    pub values: [c_float; 4],
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassContents.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkSubpassContents {
    VK_SUBPASS_CONTENTS_INLINE = 0,
    VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1,
    VK_SUBPASS_CONTENTS_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndexType.html
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Debug, PartialEq, Eq)]
pub enum VkIndexType {
    VK_INDEX_TYPE_UINT16 = 0,
    VK_INDEX_TYPE_UINT32 = 1,
    VK_INDEX_TYPE_NONE_KHR = 1000165000,
    VK_INDEX_TYPE_UINT8_EXT = 1000265000,
    VK_INDEX_TYPE_MAX_ENUM = 0x7FFFFFFF
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreCreateInfo.html
#[repr(C)]
pub struct VkSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSemaphoreCreateFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFilter.html
#[repr(C)]
pub enum VkFilter {
    VK_FILTER_NEAREST = 0,
    VK_FILTER_LINEAR = 1,
    VK_FILTER_CUBIC_IMG = 1000015000,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerMipmapMode.html
#[repr(C)]
pub enum VkSamplerMipmapMode {
    VK_SAMPLER_MIPMAP_MODE_NEAREST = 0,
    VK_SAMPLER_MIPMAP_MODE_LINEAR = 1,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerAddressMode.html
#[repr(C)]
pub enum VkSamplerAddressMode {
    VK_SAMPLER_ADDRESS_MODE_REPEAT = 0,
    VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1,
    VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2,
    VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,
    VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE = 4,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBorderColor.html
#[repr(C)]
pub enum VkBorderColor {
    VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0,
    VK_BORDER_COLOR_INT_TRANSPARENT_BLACK = 1,
    VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2,
    VK_BORDER_COLOR_INT_OPAQUE_BLACK = 3,
    VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4,
    VK_BORDER_COLOR_INT_OPAQUE_WHITE = 5,
    VK_BORDER_COLOR_FLOAT_CUSTOM_EXT = 1000287003,
    VK_BORDER_COLOR_INT_CUSTOM_EXT = 1000287004,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCreateInfo.html
#[repr(C)]
pub struct VkSamplerCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSamplerCreateFlags,
    pub magFilter: VkFilter,
    pub minFilter: VkFilter,
    pub mipmapMode: VkSamplerMipmapMode,
    pub addressModeU: VkSamplerAddressMode,
    pub addressModeV: VkSamplerAddressMode,
    pub addressModeW: VkSamplerAddressMode,
    pub mipLodBias: c_float,
    pub anisotropyEnable: VkBool32,
    pub maxAnisotropy: c_float,
    pub compareEnable: VkBool32,
    pub compareOp: VkCompareOp,
    pub minLod: c_float,
    pub maxLod: c_float,
    pub borderColor: VkBorderColor,
    pub unnormalizedCoordinates: VkBool32,
}

// Provided by VK_VERSION_1_2
// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferDeviceAddressInfo.html
#[repr(C)]
pub struct VkBufferDeviceAddressInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements2.html
#[repr(C)]
pub struct VkMemoryRequirements2 {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub memoryRequirements: VkMemoryRequirements,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryRequirementsInfo2.html
#[repr(C)]
pub struct VkBufferMemoryRequirementsInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub buffer: VkBuffer,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryDedicatedRequirements.html
#[repr(C)]
pub struct VkMemoryDedicatedRequirements {
    pub sType: VkStructureType,
    pub pNext: *mut c_void,
    pub prefersDedicatedAllocation: VkBool32,
    pub requiresDedicatedAllocation: VkBool32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagsInfo.html
#[repr(C)]
pub struct VkMemoryAllocateFlagsInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMemoryAllocateFlags,
    pub deviceMask: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateFlagBits.html
#[repr(C)]
pub enum VkMemoryAllocateFlagBits {
    VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT = 0x00000001,
    // Provided by VK_VERSION_1_2
    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT = 0x00000002,
    // Provided by VK_VERSION_1_2
    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 0x00000004,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryBarrier.html
#[repr(C)]
pub struct VkMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryBarrier.html
#[repr(C)]
pub struct VkImageMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresourceLayers.html
#[repr(C)]
pub struct VkImageSubresourceLayers {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset3D.html
#[repr(C)]
pub struct VkOffset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferImageCopy.html
#[repr(C)]
pub struct VkBufferImageCopy {
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageBlit.html
#[repr(C)]
pub struct VkImageBlit {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2],
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolCreateInfo.html
#[repr(C)]
pub struct VkQueryPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkQueryPoolCreateFlags,
    pub queryType: VkQueryType,
    pub queryCount: u32,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryType.html
#[repr(C)]
pub enum VkQueryType {
    VK_QUERY_TYPE_OCCLUSION = 0,
    VK_QUERY_TYPE_PIPELINE_STATISTICS = 1,
    VK_QUERY_TYPE_TIMESTAMP = 2,
  // Provided by VK_EXT_transform_feedback
    VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT = 1000028004,
  // Provided by VK_KHR_performance_query
    VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR = 1000116000,
  // Provided by VK_KHR_acceleration_structure
    VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR = 1000150000,
  // Provided by VK_KHR_acceleration_structure
    VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR = 1000150001,
  // Provided by VK_NV_ray_tracing
    VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV = 1000165000,
  // Provided by VK_INTEL_performance_query
    VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL = 1000210000,
}

// @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryResultFlagBits.html
#[repr(C)]
pub enum VkQueryResultFlagBits {
    VK_QUERY_RESULT_64_BIT = 0x00000001,
    VK_QUERY_RESULT_WAIT_BIT = 0x00000002,
    VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = 0x00000004,
    VK_QUERY_RESULT_PARTIAL_BIT = 0x00000008,
}

// @see https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html
#[repr(C)]
pub enum VkDescriptorPoolCreateFlagBits {
    VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 0x00000001,
  // Provided by VK_VERSION_1_2
    VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT = 0x00000002,
  // Provided by VK_VALVE_mutable_descriptor_type
    VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE = 0x00000004,
}

// @see https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfoEXT.html
#[repr(C)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub bindingCount: u32,
    pub pBindingFlags: *const VkDescriptorBindingFlags,
}

// @see https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html
#[repr(C)]
pub enum VkDescriptorBindingFlagBits {
    VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT = 0x00000001,
    VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT = 0x00000002,
    VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT = 0x00000004,
    VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT = 0x00000008,
}


// @see https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html
#[repr(C)]
pub enum VkDescriptorSetLayoutCreateFlagBits {
// Provided by VK_VERSION_1_2
    VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT = 0x00000002,
// Provided by VK_KHR_push_descriptor
    VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR = 0x00000001,
// Provided by VK_VALVE_mutable_descriptor_type
    VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE = 0x00000004,
}

// @see https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfo.html
#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorSetCount: u32,
    pub pDescriptorCounts: *const u32,
}

#[link(name = "vulkan")]
extern "C" {
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateInstance.html
    pub fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceLayerProperties.html
    pub fn vkEnumerateInstanceLayerProperties(
        pPropertyCount: *mut u32,
        pProperties: *mut VkLayerProperties,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkEnumeratePhysicalDevices.html
    pub fn vkEnumeratePhysicalDevices(
        instance: VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut VkPhysicalDevice,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetPhysicalDeviceProperties.html
    pub fn vkGetPhysicalDeviceProperties(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(
        physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32,
        pQueueFamilyProperties: *mut VkQueueFamilyProperties,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDevice.html
    pub fn vkCreateDevice(
        physicalDevice: VkPhysicalDevice,
        pCreateInfo: *const VkDeviceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDevice: *mut VkDevice,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetDeviceQueue.html
    pub fn vkGetDeviceQueue(
        device: VkDevice,
        queueFamilyIndex: u32,
        queueIndex: u32,
        queue: *mut VkQueue,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateCommandPool.html
    pub fn vkCreateCommandPool(
        device: VkDevice,
        pCreateInfo: *const VkCommandPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        commandPool: *mut VkCommandPool,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateBuffer.html
    pub fn vkCreateBuffer(
        device: VkDevice,
        pCreateInfo: *const VkBufferCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pBuffer: *mut VkBuffer,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html
    pub fn vkGetPhysicalDeviceMemoryProperties(
        physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetBufferMemoryRequirements.html
    pub fn vkGetBufferMemoryRequirements(
        device: VkDevice,
        buffer: VkBuffer,
        pMemoryRequirements: *mut VkMemoryRequirements,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkAllocateMemory.html
    pub fn vkAllocateMemory(
        device: VkDevice,
        pAllocateInfo: *const VkMemoryAllocateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pMemory: *mut VkDeviceMemory,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkMapMemory.html
    pub fn vkMapMemory(
        device: VkDevice,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
        size: VkDeviceSize,
        flags: VkMemoryMapFlags,
        ppData: *mut *mut c_void,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkUnmapMemory.html
    pub fn vkUnmapMemory(
        device: VkDevice,
        memory: VkDeviceMemory,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkBindBufferMemory.html
    pub fn vkBindBufferMemory(
        device: VkDevice,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkFlushMappedMemoryRanges.html
    pub fn vkFlushMappedMemoryRanges(
        device: VkDevice,
        memoryRangeCount: u32,
        pMemoryRange: *const VkMappedMemoryRange,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkAllocateCommandBuffers.html
    pub fn vkAllocateCommandBuffers(
        device: VkDevice,
        pAllocateInfo: *const VkCommandBufferAllocateInfo,
        pCommandBuffer: *mut VkCommandBuffer,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkBeginCommandBuffer.html
    pub fn vkBeginCommandBuffer(
        commandBuffer: VkCommandBuffer,
        pBeginInfo: *const VkCommandBufferBeginInfo,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkEndCommandBuffer.html
    pub fn vkEndCommandBuffer(
        commandBuffer: VkCommandBuffer,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateFence.html
    pub fn vkCreateFence(
        device: VkDevice,
        pCreateInfo: *const VkFenceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFence: *mut VkFence,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueueSubmit.html
    pub fn vkQueueSubmit(
        queue: VkQueue,
        submitCount: u32,
        pSubmits: *const VkSubmitInfo,
        fence: VkFence,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkWaitForFences.html
    pub fn vkWaitForFences(
        device: VkDevice,
        fenceCount: u32,
        pFences: *const VkFence,
        waitAll: VkBool32,
        timeout: u64,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyFence.html
    pub fn vkDestroyFence(
        device: VkDevice,
        fence: VkFence,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkFreeCommandBuffers.html
    pub fn vkFreeCommandBuffers(
        device: VkDevice,
        commandPool: VkCommandPool,
        commandBufferCount: u32,
        pCommandBuffers: *const VkCommandBuffer,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdCopyBuffer.html
    pub fn vkCmdCopyBuffer(
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstBuffer: VkBuffer,
        regionCount: u32,
        pRegions: *const VkBufferCopy,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDescriptorPool.html
    pub fn vkCreateDescriptorPool(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDescriptorPool: *mut VkDescriptorPool,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDescriptorSetLayout.html
    pub fn vkCreateDescriptorSetLayout(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSetLayout: *mut VkDescriptorSetLayout,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreatePipelineLayout.html
    pub fn vkCreatePipelineLayout(
        device: VkDevice,
        pCreateInfo: *const VkPipelineLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineLayout: *mut VkPipelineLayout,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkAllocateDescriptorSets.html
    pub fn vkAllocateDescriptorSets(
        device: VkDevice,
        pAllocateInfo: *const VkDescriptorSetAllocateInfo,
        pDescriptorSets: *mut VkDescriptorSet,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkUpdateDescriptorSets.html
    pub fn vkUpdateDescriptorSets(
        device: VkDevice,
        descriptorWriteCount: u32,
        pDescriptorWrites: *const VkWriteDescriptorSet,
        descriptorCopyCount: u32,
        pDescriptorCopies: *const VkCopyDescriptorSet,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreatePipelineCache.html
    pub fn vkCreatePipelineCache(
        device: VkDevice,
        pCreateInfo: *const VkPipelineCacheCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineCache: *mut VkPipelineCache,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateComputePipelines.html
    pub fn vkCreateComputePipelines(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkComputePipelineCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateShaderModule.html
    pub fn vkCreateShaderModule(
        device: VkDevice,
        pCreateInfo: *const VkShaderModuleCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pShaderModule: *mut VkShaderModule,
    ) -> VkResult;
    // @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdPipelineBarrier.html
    pub fn vkCmdPipelineBarrier(
        commandBuffer: VkCommandBuffer,
        srcStageMask: VkPipelineStageFlags,
        dstStageMask: VkPipelineStageFlags,
        dependencyFlags: VkDependencyFlags,
        memoryBarrierCount: u32,
        pMemoryBarriers: *const VkMemoryBarrier,
        bufferMemoryBarrierCount: u32,
        pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
        imageMemoryBarrierCount: u32,
        pImageMemoryBarriers: *const VkImageMemoryBarrier,
    );
    // @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdBindPipeline.html
    pub fn vkCmdBindPipeline(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        pipeline: VkPipeline,
    );
    // @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdBindDescriptorSets.html
    pub fn vkCmdBindDescriptorSets(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        firstSet: u32,
        descriptorSetCount: u32,
        pDescriptorSets: *const VkDescriptorSet,
        dynamicOffsetCount: u32,
        pDynamicOffsets: *const u32,
    );
    // @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdDispatch.html
    pub fn vkCmdDispatch(
        commandBuffer: VkCommandBuffer,
        groupCountX: u32,
        groupCountY: u32,
        groupCountZ: u32,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCmdPushConstants.html
    pub fn vkCmdPushConstants(
        commandBuffer: VkCommandBuffer,
        layout: VkPipelineLayout,
        stageFlags: VkShaderStageFlags,
        offset: u32,
        size: u32,
        pValues: *const c_void,
    );
    // @see http://khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkResetFences.html
    pub fn vkResetFences(
        device: VkDevice,
        fenceCount: u32,
        pFences: *const VkFence,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkInvalidateMappedMemoryRanges.html
    pub fn vkInvalidateMappedMemoryRanges(
        device: VkDevice,
        memoryRangeCount: u32,
        pMemoryRanges: *const VkMappedMemoryRange,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkQueueWaitIdle.html
    pub fn vkQueueWaitIdle(
        queue: VkQueue,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyBuffer.html
    pub fn vkDestroyBuffer(
        device: VkDevice,
        buffer: VkBuffer,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkFreeMemory.html
    pub fn vkFreeMemory(
        device: VkDevice,
        memory: VkDeviceMemory,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyCommandPool.html
    pub fn vkDestroyCommandPool(
        device: VkDevice,
        commandPool: VkCommandPool,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyShaderModule.html
    pub fn vkDestroyShaderModule(
        device: VkDevice,
        shaderModule: VkShaderModule,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyDevice.html
    pub fn vkDestroyDevice(
        device: VkDevice,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyInstance.html
    pub fn vkDestroyInstance(
        instance: VkInstance,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyPipelineLayout.html
    pub fn vkDestroyPipelineLayout(
        device: VkDevice,
        pipelineLayout: VkPipelineLayout,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyDescriptorSetLayout.html
    pub fn vkDestroyDescriptorSetLayout(
        device: VkDevice,
        descriptorSetLayout: VkDescriptorSetLayout,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyDescriptorPool.html
    pub fn vkDestroyDescriptorPool(
        device: VkDevice,
        descriptorPool: VkDescriptorPool,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyPipeline.html
    pub fn vkDestroyPipeline(
        device: VkDevice,
        pipeline: VkPipeline,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyPipelineCache.html
    pub fn vkDestroyPipelineCache(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceExtensionProperties.html
    pub fn vkEnumerateDeviceExtensionProperties(
        physicalDevice: VkPhysicalDevice,
        pLayerName: *const c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceExtensionProperties.html
    pub fn vkEnumerateInstanceExtensionProperties(
        pLayerName: *const c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImageView.html
    pub fn vkCreateImageView(
        device: VkDevice,
        pCreateInfo: *const VkImageViewCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pView: *mut VkImageView,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyImageView.html
    pub fn vkDestroyImageView(
        device: VkDevice,
        imageView: VkImageView,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImage.html
    pub fn vkCreateImage(
        device: VkDevice,
        pCreateInfo: *const VkImageCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pImage: *mut VkImage,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory.html
    pub fn vkBindImageMemory(
        device: VkDevice,
        image: VkImage,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements.html
    pub fn vkGetImageMemoryRequirements(
        device: VkDevice,
        image: VkImage,
        pMemoryRequirements: *mut VkMemoryRequirements,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyImage.html
    pub fn vkDestroyImage(
        device: VkDevice,
        image: VkImage,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass.html
    pub fn vkCreateRenderPass(
        device: VkDevice,
        pCreateInfo: *const VkRenderPassCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pRenderPass: *mut VkRenderPass,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyRenderPass.html
    pub fn vkDestroyRenderPass(
        device: VkDevice,
        renderPass: VkRenderPass,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateFramebuffer.html
    pub fn vkCreateFramebuffer(
        device: VkDevice,
        pCreateInfo: *const VkFramebufferCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFramebuffer: *mut VkFramebuffer,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyFramebuffer.html
    pub fn vkDestroyFramebuffer(
        device: VkDevice,
        framebuffer: VkFramebuffer,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateGraphicsPipelines.html
    pub fn vkCreateGraphicsPipelines(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkGraphicsPipelineCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass.html
    pub fn vkCmdBeginRenderPass(
        commandBuffer: VkCommandBuffer,
        pRenderPassBegin: *const VkRenderPassBeginInfo,
        contents: VkSubpassContents,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewport.html
    pub fn vkCmdSetViewport(
        commandBuffer: VkCommandBuffer,
        firstViewport: u32,
        viewportCount: u32,
        pViewports: *const VkViewport,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissor.html
    pub fn vkCmdSetScissor(
        commandBuffer: VkCommandBuffer,
        firstScissor: u32,
        scissorCount: u32,
        pScissors: *const VkRect2D,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers.html
    pub fn vkCmdBindVertexBuffers(
        commandBuffer: VkCommandBuffer,
        firstBinding: u32,
        bindingCount: u32,
        pBuffers: *const VkBuffer,
        pOffsets: *const VkDeviceSize,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindIndexBuffer.html
    pub fn vkCmdBindIndexBuffer(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        indexType: VkIndexType,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexed.html
    pub fn vkCmdDrawIndexed(
        commandBuffer: VkCommandBuffer,
        indexCount: u32,
        instanceCount: u32,
        firstIndex: u32,
        vertexOffset: i32,
        firstInstance: u32,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDraw.html
    pub fn vkCmdDraw(
        commandBuffer: VkCommandBuffer,
        vertexCount: u32,
        instanceCount: u32,
        firstVertex: u32,
        firstInstance: u32,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass.html
    pub fn vkCmdEndRenderPass(
        commandBuffer: VkCommandBuffer,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSemaphore.html
    pub fn vkCreateSemaphore(
        device: VkDevice,
        pCreateInfo: *const VkSemaphoreCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSemaphore: *mut VkSemaphore,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySemaphore.html
    pub fn vkDestroySemaphore(
        device: VkDevice,
        semaphore: VkSemaphore,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2.html
    pub fn vkGetPhysicalDeviceProperties2(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties2,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSampler.html
    pub fn vkCreateSampler(
        device: VkDevice,
        pCreateInfo: *const VkSamplerCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSampler: *mut VkSampler,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySampler.html
    pub fn vkDestroySampler(
        device: VkDevice,
        sampler: VkSampler,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2.html
    pub fn vkGetBufferMemoryRequirements2(
        device: VkDevice,
        pInfo: *const VkBufferMemoryRequirementsInfo2,
        pMemoryRequirements: *mut VkMemoryRequirements2,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html
    pub fn vkGetInstanceProcAddr(
        instance: VkInstance,
        pName: *const c_char,
    ) -> *const c_void;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceProcAddr.html
    pub fn vkGetDeviceProcAddr(
        device: VkDevice,
        pName: *const c_char,
    ) -> *const c_void;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2.html
    pub fn vkGetPhysicalDeviceFeatures2(
        physicalDevice: VkPhysicalDevice,
        pFeatures: *mut VkPhysicalDeviceFeatures2,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBufferToImage.html
    pub fn vkCmdCopyBufferToImage(
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkBufferImageCopy,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBlitImage.html
    pub fn vkCmdBlitImage(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageBlit,
        filter: VkFilter,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateQueryPool.html
    pub fn vkCreateQueryPool(
        device: VkDevice,
        pCreateInfo: *const VkQueryPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pQueryPool: *mut VkQueryPool,
    ) -> VkResult;
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyQueryPool.html
    pub fn vkDestroyQueryPool(
        device: VkDevice,
        queryPool: VkQueryPool,
        pAllocator: *const VkAllocationCallbacks,
    );
    // @see https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueryPoolResults.html
    pub fn vkGetQueryPoolResults(
        device: VkDevice,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
        dataSize: size_t,
        pData: *mut c_void,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags,
    ) -> VkResult;
    // @see https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html
    pub fn vkCmdResetQueryPool(
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
    );
}
