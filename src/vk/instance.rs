

use crate::ffi::vk::*;

use super::error::Result;
use super::error::ErrorCode;
use super::debug::{DebugUtilsMessenger};

use std::ptr;
use std::ffi::{CString};
use std::mem::MaybeUninit;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct Instance {
    handle: VkInstance,
    messenger: Option<DebugUtilsMessenger>,
}

impl Instance {
    pub fn new() -> Result<Arc<Self>> {
        InstanceBuilder::new().build()
    }

    #[inline]
    pub fn handle(&self) -> VkInstance {
        self.handle
    }

    pub fn extension_properties() -> Result<Vec<VkExtensionProperties>> {
        unsafe {
            let mut count = MaybeUninit::<u32>::zeroed();
            vkEnumerateInstanceExtensionProperties(std::ptr::null(), count.as_mut_ptr(), std::ptr::null_mut())
                .into_result()?;
            let size = count.assume_init() as usize;
            let mut extensions: Vec<VkExtensionProperties> = Vec::with_capacity(size);
            extensions.resize(size, std::mem::zeroed());
            vkEnumerateInstanceExtensionProperties(std::ptr::null(), count.as_mut_ptr(), extensions.as_mut_ptr())
                .into_result()?;
            Ok(extensions)
        }
    }

    pub fn layer_properties() -> Result<Vec<VkLayerProperties>> {
        unsafe {
            let mut count = MaybeUninit::<u32>::zeroed();
            vkEnumerateInstanceLayerProperties(count.as_mut_ptr(), ptr::null_mut())
                .into_result()?;
            let size = count.assume_init() as usize;
            let mut layers: Vec<VkLayerProperties> = Vec::with_capacity(size);
            layers.resize(size, std::mem::zeroed());
            vkEnumerateInstanceLayerProperties(count.as_mut_ptr(), layers.as_mut_ptr())
                .into_result()?;
            Ok(layers)
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        log_debug!("Drop Instance");
        unsafe {
            drop(self.messenger.take());
            vkDestroyInstance(self.handle, ptr::null());
            self.handle = ptr::null_mut();
        }
    }
}

#[derive(Default)]
pub struct InstanceBuilder<IsDebug = ()> {
    _is_debug: IsDebug,
}

// default instance creation
impl InstanceBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Result<Arc<Instance>> {
        unsafe {
            let application_name = CString::new("kaldera")?;
            let engine_name = CString::new("Kaldera Engine")?;
            let app_info = VkApplicationInfo::new(application_name.as_ptr(), 0, engine_name.as_ptr(), 0);
            let extension_names = Self::extension_names();
            let extension_name_ptrs: Vec<_> = extension_names.iter()
                .map(|v| v.as_ptr())
                .collect();
            {
                let create_info = VkInstanceCreateInfo { 
                    sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                    pNext: ptr::null(),
                    flags: 0,
                    pApplicationInfo: &app_info,
                    enabledLayerCount: 0,
                    ppEnabledLayerNames: ptr::null(),
                    enabledExtensionCount: extension_name_ptrs.len() as u32,
                    ppEnabledExtensionNames: extension_name_ptrs.as_ptr(),
                };
                let mut handle = MaybeUninit::<VkInstance>::zeroed();
                vkCreateInstance(&create_info, ptr::null(), handle.as_mut_ptr())
                    .into_result()?;
                let handle = handle.assume_init();
                let instance = Instance { 
                    handle,
                    messenger: None,
                };
                Ok(Arc::new(instance))
            }
        }
    }

    pub fn with_debug(self) -> InstanceBuilder<PhantomData<()>> {
        Default::default()
    }

    fn extension_names() -> Vec<CString> {
        vec![
            CString::new("VK_KHR_surface").unwrap(),
            CString::new("VK_KHR_xcb_surface").unwrap(),
            CString::new("VK_KHR_get_physical_device_properties2").unwrap(),
        ]
    }
}

// validation layer instance creation
impl InstanceBuilder<PhantomData<()>> {
    pub fn build(self) -> Result<Arc<Instance>> {
        unsafe {
            let application_name = CString::new("kaldera")?;
            let engine_name = CString::new("Kaldera Engine")?;
            let app_info = VkApplicationInfo::new(application_name.as_ptr(), 0, engine_name.as_ptr(), 0);
            let layer_names = Self::layer_names();
            let layer_name_ptrs: Vec<_> = layer_names.iter()
                .map(|v| v.as_ptr())
                .collect();
            let extension_names = Self::extension_names();
            let extension_name_ptrs: Vec<_> = extension_names.iter()
                .map(|v| v.as_ptr())
                .collect();
            {
                let create_info = VkInstanceCreateInfo { 
                    sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                    pNext: ptr::null(),
                    flags: 0,
                    pApplicationInfo: &app_info,
                    enabledLayerCount: layer_name_ptrs.len() as u32,
                    ppEnabledLayerNames: layer_name_ptrs.as_ptr(),
                    enabledExtensionCount: extension_name_ptrs.len() as u32,
                    ppEnabledExtensionNames: extension_name_ptrs.as_ptr(),
                };
                let mut handle = MaybeUninit::<VkInstance>::zeroed();
                vkCreateInstance(&create_info, ptr::null(), handle.as_mut_ptr())
                    .into_result()?;
                let handle = handle.assume_init();
                let messenger = Some(DebugUtilsMessenger::new(handle));
                let instance = Instance { 
                    handle,
                    messenger,
                };
                Ok(Arc::new(instance))
            }
        }
    }

    fn layer_names() -> Vec<CString> {
        vec![
            CString::new("VK_LAYER_KHRONOS_validation").unwrap(),
        ]
    }

    fn extension_names() -> Vec<CString> {
        vec![
            CString::new("VK_KHR_surface").unwrap(),
            CString::new("VK_KHR_xcb_surface").unwrap(),
            CString::new("VK_KHR_get_physical_device_properties2").unwrap(),
            CString::new("VK_EXT_debug_utils").unwrap(),
        ]
    }
}

pub struct PhysicalDevicesBuilder<'a> {
    instance: &'a Arc<Instance>,
}

impl<'a> PhysicalDevicesBuilder<'a> {
    pub fn new(instance: &'a Arc<Instance>) -> Self {
        PhysicalDevicesBuilder { instance: instance }
    }

    pub fn build(self) -> Result<Vec<Arc<PhysicalDevice>>> {
        let instance = self.instance;
        unsafe {
            let mut count = MaybeUninit::<u32>::zeroed();
            // obtain count
            vkEnumeratePhysicalDevices(instance.handle, count.as_mut_ptr(), ptr::null_mut())
                .into_result()?;
            // obtain items
            let size: usize = count.assume_init() as usize;
            let mut devices: Vec<VkPhysicalDevice> = Vec::with_capacity(size);
            devices.resize(size, ptr::null_mut());
            vkEnumeratePhysicalDevices(instance.handle, count.as_mut_ptr(), devices.as_mut_ptr())
                .into_result()?;
            let devices: Vec<Arc<PhysicalDevice>> = devices.into_iter()
                .map(|v| PhysicalDevice::new(v, instance))
                .collect();
            Ok(devices)
        }
    }
}

pub struct PhysicalDevice {
    handle: VkPhysicalDevice,
    instance: Arc<Instance>,
}

impl PhysicalDevice {
    pub fn new(device: VkPhysicalDevice, instance: &Arc<Instance>) -> Arc<Self> {
        let device = PhysicalDevice { handle: device, instance: Arc::clone(instance) };
        Arc::new(device)
    }

    #[inline]
    pub fn handle(&self) -> VkPhysicalDevice {
        self.handle
    }

    #[inline]
    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    pub fn properties(&self) -> VkPhysicalDeviceProperties {
        unsafe {
            let mut properties = MaybeUninit::<VkPhysicalDeviceProperties>::zeroed();
            vkGetPhysicalDeviceProperties(self.handle, properties.as_mut_ptr());
            properties.assume_init()
        }
    }

    pub fn queue_families(&self) -> Result<Vec<QueueFamily>> {
        unsafe {
            let mut count = MaybeUninit::<u32>::zeroed();
            // obtain count
            vkGetPhysicalDeviceQueueFamilyProperties(self.handle, count.as_mut_ptr(), ptr::null_mut());
            // obtain items
            let size: usize = count.assume_init() as usize;
            let mut families: Vec<VkQueueFamilyProperties> = Vec::with_capacity(size);
            families.resize(size, std::mem::zeroed());
            vkGetPhysicalDeviceQueueFamilyProperties(self.handle, count.as_mut_ptr(), families.as_mut_ptr());
            let families: Vec<QueueFamily> = families.into_iter()
                .enumerate()
                .map(|(i, v)| QueueFamily::new(i as u32, v))
                .collect();
            Ok(families)
        }
    }

    pub fn extension_properties(&self) -> Result<Vec<VkExtensionProperties>> {
        unsafe {
            let mut count = MaybeUninit::<u32>::zeroed();
            vkEnumerateDeviceExtensionProperties(self.handle, ptr::null(), count.as_mut_ptr(), ptr::null_mut());
            let size = count.assume_init() as usize;
            let mut extensions: Vec<VkExtensionProperties> = Vec::with_capacity(size);
            extensions.resize(size, std::mem::zeroed());
            vkEnumerateDeviceExtensionProperties(self.handle, ptr::null(), count.as_mut_ptr(), extensions.as_mut_ptr());
            Ok(extensions)
        }
    }

    pub fn memory_type_index(&self, 
        memory_requirements: &VkMemoryRequirements, 
        memory_property_flags: VkMemoryPropertyFlags,
    ) -> Option<u32> {
        unsafe {
            let mut memory_properties = MaybeUninit::<VkPhysicalDeviceMemoryProperties>::zeroed();
            vkGetPhysicalDeviceMemoryProperties(self.handle(), memory_properties.as_mut_ptr());
            let memory_properties = memory_properties.assume_init();
            // find a memory type index that fits the properties
            let memory_type_bits = memory_requirements.memoryTypeBits;
            let memory_type_index = memory_properties.memoryTypes.iter()
                .enumerate()
                .find(|(i, v)| ((memory_type_bits >> i) & 1) == 1 
                    && (v.propertyFlags & memory_property_flags) == memory_property_flags)
                .map(|(i, _)| i as u32);
            memory_type_index
        }
    }

    pub fn features(&self) -> PhysicalDeviceFeatures {
        unsafe {
            PhysicalDeviceFeatures::init(self.handle)
        }
    }
}

pub struct PhysicalDeviceFeatures {
    features: Box<MaybeUninit<VkPhysicalDeviceFeatures2>>,
    device_address: Box<MaybeUninit<VkPhysicalDeviceBufferDeviceAddressFeatures>>,
    indexing: Box<MaybeUninit<VkPhysicalDeviceDescriptorIndexingFeatures>>,
    ray_tracing_pipeline: Box<MaybeUninit<VkPhysicalDeviceRayTracingPipelineFeaturesKHR>>,
    ray_tracing_structures: Box<MaybeUninit<VkPhysicalDeviceAccelerationStructureFeaturesKHR>>,
}

impl PhysicalDeviceFeatures {
    unsafe fn init(handle: VkPhysicalDevice) -> Self {
        let mut features: Box<MaybeUninit<VkPhysicalDeviceFeatures2>> = Box::new(MaybeUninit::zeroed());
        let mut device_address: Box<MaybeUninit<VkPhysicalDeviceBufferDeviceAddressFeatures>> = Box::new(MaybeUninit::zeroed());
        let mut indexing: Box<MaybeUninit<VkPhysicalDeviceDescriptorIndexingFeatures>> = Box::new(MaybeUninit::zeroed());
        let mut ray_tracing_pipeline: Box<MaybeUninit<VkPhysicalDeviceRayTracingPipelineFeaturesKHR>> = Box::new(MaybeUninit::zeroed());
        let mut ray_tracing_structures: Box<MaybeUninit<VkPhysicalDeviceAccelerationStructureFeaturesKHR>> = Box::new(MaybeUninit::zeroed());
        {
            let features = features.as_mut_ptr().as_mut().unwrap();
            features.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2;
            let device_address = device_address.as_mut_ptr().as_mut().unwrap();
            device_address.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
            let indexing = indexing.as_mut_ptr().as_mut().unwrap();
            indexing.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
            let ray_tracing_pipeline = ray_tracing_pipeline.as_mut_ptr().as_mut().unwrap();
            ray_tracing_pipeline.sType = VkStructureTypeExtRay::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR;
            let ray_tracing_structures = ray_tracing_structures.as_mut_ptr().as_mut().unwrap();
            ray_tracing_structures.sType = VkStructureTypeExtRay::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR;
        }
        {
            let features = features.as_mut_ptr().as_mut().unwrap();
            features.pNext = device_address.as_mut_ptr() as *mut _;
        }
        {
            let device_address = device_address.as_mut_ptr().as_mut().unwrap();
            device_address.pNext = indexing.as_mut_ptr() as *mut _;
        }
        {
            let indexing = indexing.as_mut_ptr().as_mut().unwrap();
            indexing.pNext = ray_tracing_pipeline.as_mut_ptr() as *mut _;
        }
        {
            let ray_tracing_pipeline = ray_tracing_pipeline.as_mut_ptr().as_mut().unwrap();
            ray_tracing_pipeline.pNext = ray_tracing_structures.as_mut_ptr() as *mut _;
        }
        vkGetPhysicalDeviceFeatures2(handle, features.as_mut_ptr());
        Self {
            features,
            device_address,
            indexing,
            ray_tracing_pipeline,
            ray_tracing_structures,
        }
    }

    pub fn features(&self) -> &VkPhysicalDeviceFeatures2 {
        unsafe { 
            self.features.as_ref().as_ptr().as_ref().unwrap() 
        }
    }
}

impl PhysicalDevice {
    pub fn properties_ray_tracing(&self) -> VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
        unsafe {
            let mut ray_tracing = MaybeUninit::<VkPhysicalDeviceRayTracingPipelinePropertiesKHR>::zeroed();
            {
                let ray_tracing = ray_tracing.as_mut_ptr().as_mut().unwrap();
                ray_tracing.sType = VkStructureTypeExtRay::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR;
                ray_tracing.pNext = ptr::null_mut();
            }
            let mut properties = MaybeUninit::<VkPhysicalDeviceProperties2>::zeroed();
            {
                let properties = properties.as_mut_ptr().as_mut().unwrap();
                properties.sType = VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;
                properties.pNext = ray_tracing.as_mut_ptr() as *mut _;
            }
            vkGetPhysicalDeviceProperties2(self.handle, properties.as_mut_ptr());
            ray_tracing.assume_init()
        }
    }
}

#[derive(Clone)]
pub struct QueueFamily {
    index: u32,
    property: VkQueueFamilyProperties,
}

impl QueueFamily {
    pub fn new(index: u32, property: VkQueueFamilyProperties) -> Self {
        QueueFamily { index: index, property: property }
    }

    #[inline]
    pub fn index(&self) -> u32 {
        self.index
    }

    #[inline]
    pub fn queue_count(&self) -> u32 {
        self.property.queueCount
    }

    #[inline]
    pub fn is_compute(&self) -> bool {
        self.property.has_compute_queue_bit()
    }

    #[inline]
    pub fn is_graphics(&self) -> bool {
        self.property.has_graphics_queue_bit()
    }
}

pub struct PhysicalDeviceCapabilities {
    devices: Vec<Arc<PhysicalDevice>>,
}

impl PhysicalDeviceCapabilities {
    pub fn new(instance: &Arc<Instance>) -> Result<Arc<Self>> {
        let devices = PhysicalDevicesBuilder::new(instance).build()?;
        let capabilities = Self {
            devices,
        };
        Ok(Arc::new(capabilities))
    }

    pub fn has_raytracing(&self) -> bool {
        let name = CString::new("VK_KHR_ray_tracing_pipeline")
            .unwrap_or_default();
        self.devices.iter()
            .filter_map(|v| v.extension_properties().ok())
            .any(|v| v.iter().any(|v| v.extension_name() == name))
    }
}
