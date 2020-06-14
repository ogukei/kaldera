
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, ShaderModule, ShaderModuleSource};
use super::memory::{StagingBuffer, StagingBufferUsage};
use super::swapchain::{SwapchainFramebuffers, SceneRenderPass};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;
use std::ffi::CString;

#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct Vertex {
    pub coordinate: Vec3,
    pub color: Vec3,
}

pub struct RenderStagingBuffer {
    vertex_buffer: Arc<StagingBuffer>,
    index_buffer: Arc<StagingBuffer>,
    index_count: usize,
}

impl RenderStagingBuffer {
    pub fn new(command_pool: &Arc<CommandPool>, vertices: Vec<Vertex>, indices: Vec<u32>) -> Arc<Self> {
        let vertex_buffer_size = std::mem::size_of::<Vertex>() * vertices.len();
        let vertex_buffer = StagingBuffer::new(
            command_pool, 
            StagingBufferUsage::Vertex, 
            vertex_buffer_size as VkDeviceSize)
            .unwrap();
        let index_buffer_size = std::mem::size_of::<u32>() * indices.len();
        let index_buffer = StagingBuffer::new(
            command_pool, 
            StagingBufferUsage::Index, 
            index_buffer_size as VkDeviceSize)
            .unwrap();
        // transfer
        vertex_buffer.write(vertices.as_ptr() as *const c_void, vertex_buffer_size);
        index_buffer.write(indices.as_ptr() as *const c_void, index_buffer_size);
        let buffer = RenderStagingBuffer {
            vertex_buffer,
            index_buffer,
            index_count: indices.len(),
        };
        Arc::new(buffer)
    }

    #[inline]
    pub fn vertex_buffer(&self) -> &Arc<StagingBuffer> {
        &self.vertex_buffer
    }

    #[inline]
    pub fn index_buffer(&self) -> &Arc<StagingBuffer> {
        &self.index_buffer
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.index_count
    }
}

pub struct SceneGraphicsPipelineLayout {
    device: Arc<Device>,
    handle: VkPipelineLayout,
}

impl SceneGraphicsPipelineLayout {
    pub fn new(device: &Arc<Device>) -> Result<Arc<Self>> {
        unsafe { Self::init(device) }
    }

    unsafe fn init(device: &Arc<Device>) -> Result<Arc<Self>> {
        let mut handle = MaybeUninit::<VkPipelineLayout>::zeroed();
        {
            let create_info = VkPipelineLayoutCreateInfo::new(0, ptr::null());
            vkCreatePipelineLayout(device.handle(), &create_info, ptr::null(), handle.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let handle = handle.assume_init();
        let layout = SceneGraphicsPipelineLayout {
            device: Arc::clone(device),
            handle,
        };
        Ok(Arc::new(layout))
    }

    #[inline]
    pub fn handle(&self) -> VkPipelineLayout {
        self.handle
    }
}

impl Drop for SceneGraphicsPipelineLayout {
    fn drop(&mut self) {
        log_debug!("Drop SceneGraphicsPipelineLayout");
        unsafe {
            vkDestroyPipelineLayout(self.device.handle(), self.handle, ptr::null());
        }
    }
}

pub struct SceneGraphicsPipeline {
    render_pass: Arc<SceneRenderPass>,
    layout: Arc<SceneGraphicsPipelineLayout>,
    cache: VkPipelineCache,
    handle: VkPipeline,
}

impl SceneGraphicsPipeline {
    pub fn new(render_pass: &Arc<SceneRenderPass>, layout: &Arc<SceneGraphicsPipelineLayout>) -> Result<Arc<Self>> {
        unsafe { Self::init(render_pass, layout) }
    }

    unsafe fn init(render_pass: &Arc<SceneRenderPass>, layout: &Arc<SceneGraphicsPipelineLayout>) -> Result<Arc<Self>> {
        let device = render_pass.device();
        // input assembly
        let input_assembly_state = VkPipelineInputAssemblyStateCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            topology: VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            primitiveRestartEnable: VK_FALSE,
        };
        // rasterization
        let rasterization_state = VkPipelineRasterizationStateCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            depthClampEnable: VK_FALSE,
            rasterizerDiscardEnable: VK_FALSE,
            polygonMode: VkPolygonMode::VK_POLYGON_MODE_FILL,
            cullMode: VkCullModeFlagBits::VK_CULL_MODE_NONE as VkCullModeFlags,
            frontFace: VkFrontFace::VK_FRONT_FACE_COUNTER_CLOCKWISE,
            depthBiasEnable: VK_FALSE,
            depthBiasConstantFactor: 0.0,
            depthBiasClamp: 0.0,
            depthBiasSlopeFactor: 0.0,
            lineWidth: 1.0,
        };
        // color blend
        let color_blend_attachment_state = VkPipelineColorBlendAttachmentState {
            blendEnable: VK_FALSE,
            srcColorBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ZERO,
            dstColorBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ZERO,
            colorBlendOp: VkBlendOp::VK_BLEND_OP_ADD,
            srcAlphaBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ZERO,
            dstAlphaBlendFactor: VkBlendFactor::VK_BLEND_FACTOR_ZERO,
            alphaBlendOp: VkBlendOp::VK_BLEND_OP_ADD,
            colorWriteMask: VkColorComponentFlagBits::rgba(),
        };
        let color_blend_state = VkPipelineColorBlendStateCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            logicOpEnable: VK_FALSE,
            logicOp: VkLogicOp::VK_LOGIC_OP_CLEAR,
            attachmentCount: 1,
            pAttachments: &color_blend_attachment_state,
            blendConstants: [0.0; 4],
        };
        // viewport (dynamic state)
        let viewport_state = VkPipelineViewportStateCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            viewportCount: 1,
            pViewports: ptr::null(),
            scissorCount: 1,
            pScissors: ptr::null(),
        };
        let dynamic_state_enables = vec![
            VkDynamicState::VK_DYNAMIC_STATE_VIEWPORT,
            VkDynamicState::VK_DYNAMIC_STATE_SCISSOR,
        ];
        let dynamic_state = VkPipelineDynamicStateCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            dynamicStateCount: dynamic_state_enables.len() as u32,
            pDynamicStates: dynamic_state_enables.as_ptr(),
        };
        // depth stencil
        let depth_stencil_state = VkPipelineDepthStencilStateCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            depthTestEnable: VK_TRUE,
            depthWriteEnable: VK_TRUE,
            depthCompareOp: VkCompareOp::VK_COMPARE_OP_LESS_OR_EQUAL,
            depthBoundsTestEnable: VK_FALSE,
            stencilTestEnable: VK_FALSE,
            front: VkStencilOpState::default(),
            back: VkStencilOpState::default(),
            minDepthBounds: 0.0,
            maxDepthBounds: 0.0,
        };
        // multisampling
        let multisampling_state = VkPipelineMultisampleStateCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            rasterizationSamples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
            sampleShadingEnable: VK_FALSE,
            minSampleShading: 0.0,
            pSampleMask: ptr::null(),
            alphaToCoverageEnable: VK_FALSE,
            alphaToOneEnable: VK_FALSE,
        };
        // vertex input
        let vertex_input_binding = VkVertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as u32,
            inputRate: VkVertexInputRate::VK_VERTEX_INPUT_RATE_VERTEX,
        };
        let vertex_input_attributes = vec![
            VkVertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
                offset: 0,
            },
            VkVertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
                offset: std::mem::size_of::<Vec3>() as u32,
            },
        ];
        let vertex_input_state = VkPipelineVertexInputStateCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            vertexBindingDescriptionCount: 1,
            pVertexBindingDescriptions: &vertex_input_binding,
            vertexAttributeDescriptionCount: vertex_input_attributes.len() as u32,
            pVertexAttributeDescriptions: vertex_input_attributes.as_ptr(),
        };
        // shaders
        let vertex_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/triangle.vert.spv")).unwrap();
        let fragment_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/triangle.frag.spv")).unwrap();
        let shader_entry_point = CString::new("main").unwrap();
        let shader_stages = vec![
            VkPipelineShaderStageCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                stage: VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT,
                module: vertex_shader_module.handle(),
                pName: shader_entry_point.as_ptr(),
                pSpecializationInfo: ptr::null(),
            },
            VkPipelineShaderStageCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: 0,
                stage: VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT,
                module: fragment_shader_module.handle(),
                pName: shader_entry_point.as_ptr(),
                pSpecializationInfo: ptr::null(),
            },
        ];
        // pipeline
        let create_info = VkGraphicsPipelineCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            stageCount: shader_stages.len() as u32,
            pStages: shader_stages.as_ptr(),
            pVertexInputState: &vertex_input_state,
            pInputAssemblyState: &input_assembly_state,
            pTessellationState: ptr::null(),
            pViewportState: &viewport_state,
            pRasterizationState: &rasterization_state,
            pMultisampleState: &multisampling_state,
            pDepthStencilState: &depth_stencil_state,
            pColorBlendState: &color_blend_state,
            pDynamicState: &dynamic_state,
            layout: layout.handle(),
            renderPass: render_pass.handle(),
            subpass: 0,
            basePipelineHandle: ptr::null_mut(),
            basePipelineIndex: 0,
        };
        let mut cache = MaybeUninit::<VkPipelineCache>::zeroed();
        let cache_create_info = VkPipelineCacheCreateInfo::new();
        vkCreatePipelineCache(device.handle(), &cache_create_info, ptr::null(), cache.as_mut_ptr())
            .into_result()
            .unwrap();
        let cache = cache.assume_init();
        let mut handle = MaybeUninit::<VkPipeline>::zeroed();
        vkCreateGraphicsPipelines(device.handle(), cache, 1, &create_info, ptr::null(), handle.as_mut_ptr())
            .into_result()
            .unwrap();
        let handle = handle.assume_init();
        let pipeline = SceneGraphicsPipeline {
            render_pass: Arc::clone(render_pass),
            layout: Arc::clone(layout),
            cache: cache,
            handle: handle,
        };
        Ok(Arc::new(pipeline))
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        self.render_pass.device()
    }

    #[inline]
    pub fn render_pass(&self) -> &Arc<SceneRenderPass> {
        &self.render_pass
    }

    #[inline]
    pub fn handle(&self) -> VkPipeline {
        self.handle
    }
}

impl Drop for SceneGraphicsPipeline {
    fn drop(&mut self) {
        log_debug!("Drop SceneGraphicsPipeline");
        unsafe {
            let device = self.render_pass.device();
            vkDestroyPipelineCache(device.handle(), self.cache, ptr::null());
            vkDestroyPipeline(device.handle(), self.handle, ptr::null());
        }
    }
}
