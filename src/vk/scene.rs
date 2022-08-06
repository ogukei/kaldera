
use crate::ffi::vk::*;
use super::error::Result;
use super::error::ErrorCode;
use super::instance::{Instance, QueueFamily, PhysicalDevice, PhysicalDevicesBuilder};
use super::device::{Device, CommandPool, CommandBuffer, CommandBufferBuilder, ShaderModule, ShaderModuleSource};
use super::memory::{StagingBuffer, StagingBufferUsage};
use super::swapchain::{SwapchainFramebuffers, SwapchainFramebuffer, SceneRenderPass};
use super::image::ColorImage;
use super::offscreen::{OffscreenFramebuffer, OffscreenGraphicsPipeline};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use libc::{c_float, c_void};
use std::sync::Arc;
use std::io::Read;
use std::ffi::CString;

pub struct SceneGraphicsPipelineLayout {
    device: Arc<Device>,
    handle: VkPipelineLayout,
    descriptor_pool: VkDescriptorPool,
    descriptor_set_layout: VkDescriptorSetLayout,
    descriptor_set: VkDescriptorSet,
    color_image: Arc<ColorImage>,
}

impl SceneGraphicsPipelineLayout {
    pub fn new(
        device: &Arc<Device>, 
        color_image: &Arc<ColorImage>,
    ) -> Result<Arc<Self>> {
        unsafe {
            Self::init(device, color_image)
        }
    }

    unsafe fn init(device: &Arc<Device>, color_image: &Arc<ColorImage>) -> Result<Arc<Self>> {
        let image_info = VkDescriptorImageInfo {
            imageLayout: VkImageLayout::VK_IMAGE_LAYOUT_GENERAL,
            imageView: color_image.view(),
            sampler: color_image.sampler(),
        };
        // Descriptor Pool
        let mut descriptor_pool = MaybeUninit::<VkDescriptorPool>::zeroed();
        {
            let size = VkDescriptorPoolSize::new(VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 1);
            let create_info = VkDescriptorPoolCreateInfo::new(1, 1, &size, 0);
            vkCreateDescriptorPool(device.handle(), &create_info, ptr::null(), descriptor_pool.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let descriptor_pool = descriptor_pool.assume_init();
        // Descriptor Set Layout
        let mut descriptor_set_layout = MaybeUninit::<VkDescriptorSetLayout>::zeroed();
        {
            let bindings = vec![
                VkDescriptorSetLayoutBinding::new(
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 
                    VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32,
                    0,
                )
            ];
            let create_info = VkDescriptorSetLayoutCreateInfo::new(bindings.len() as u32, bindings.as_ptr());
            vkCreateDescriptorSetLayout(device.handle(), &create_info, ptr::null(), descriptor_set_layout.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let descriptor_set_layout = descriptor_set_layout.assume_init();
        // Pipeline Layout
        let mut handle = MaybeUninit::<VkPipelineLayout>::zeroed();
        {
            let create_info = VkPipelineLayoutCreateInfo::new(1, &descriptor_set_layout);
            vkCreatePipelineLayout(device.handle(), &create_info, ptr::null(), handle.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let handle = handle.assume_init();
        // Instantiate
        let mut descriptor_set = MaybeUninit::<VkDescriptorSet>::zeroed();
        {
            let alloc_info = VkDescriptorSetAllocateInfo::new(descriptor_pool, 1, &descriptor_set_layout);
            vkAllocateDescriptorSets(device.handle(), &alloc_info, descriptor_set.as_mut_ptr())
                .into_result()
                .unwrap();
        }
        let descriptor_set = descriptor_set.assume_init();
        // Write Descriptor
        {
            let write_sets = vec![
                VkWriteDescriptorSet::from_image(
                    descriptor_set, 
                    VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 
                    0,
                    &image_info,
                )
            ];
            vkUpdateDescriptorSets(device.handle(), write_sets.len() as u32, write_sets.as_ptr(), 0, ptr::null());
        }
        let layout = SceneGraphicsPipelineLayout {
            device: Arc::clone(device),
            handle,
            descriptor_pool,
            descriptor_set_layout,
            descriptor_set,
            color_image: Arc::clone(color_image),
        };
        Ok(Arc::new(layout))
    }

    #[inline]
    pub fn handle(&self) -> VkPipelineLayout {
        self.handle
    }

    #[inline]
    pub fn descriptor_set(&self) -> VkDescriptorSet {
        self.descriptor_set
    }
}

impl Drop for SceneGraphicsPipelineLayout {
    fn drop(&mut self) {
        log_debug!("Drop SceneGraphicsPipelineLayout");
        unsafe {
            let device = &self.device;
            vkDestroyPipelineLayout(device.handle(), self.handle, ptr::null());
            vkDestroyDescriptorSetLayout(device.handle(), self.descriptor_set_layout, ptr::null());
            vkDestroyDescriptorPool(device.handle(), self.descriptor_pool, ptr::null());
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
        let vertex_input_state = VkPipelineVertexInputStateCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            vertexBindingDescriptionCount: 0,
            pVertexBindingDescriptions: ptr::null(),
            vertexAttributeDescriptionCount: 0,
            pVertexAttributeDescriptions: ptr::null(),
        };
        // shaders
        let vertex_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/blit.vert.spv")).unwrap();
        let fragment_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/blit.frag.spv")).unwrap();
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

    #[inline]
    pub fn layout(&self) -> &Arc<SceneGraphicsPipelineLayout> {
        &self.layout
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

pub struct SceneGraphicsRender {
    pipeline: Arc<SceneGraphicsPipeline>,
}

impl SceneGraphicsRender {
    pub fn new(
        pipeline: &Arc<SceneGraphicsPipeline>,
    ) -> Result<Arc<Self>> {
        unsafe {
            Self::init(pipeline)
        }
    }

    unsafe fn init(
        pipeline: &Arc<SceneGraphicsPipeline>,
    ) -> Result<Arc<Self>> {
        let render = Self {
            pipeline: Arc::clone(pipeline),
        };
        Ok(Arc::new(render))
    }

    pub unsafe fn command(&self, 
        command_buffer: VkCommandBuffer, 
        swapchain_framebuffer: &SwapchainFramebuffer,
        area: VkRect2D,
    ) {
        let pipeline = &self.pipeline;
        let descriptor_set = pipeline.layout().descriptor_set();
        let clear_values = vec![
            VkClearValue {
                values: [0.0, 0.0, 0.2, 1.0],
            },
            VkClearValue {
                values: [1.0, 0.0, 0.0, 0.0],
            },
        ];
        let render_pass_begin_info = VkRenderPassBeginInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
            pNext: ptr::null(),
            renderPass: pipeline.render_pass().handle(),
            framebuffer: swapchain_framebuffer.handle(),
            renderArea: area,
            clearValueCount: clear_values.len() as u32,
            pClearValues: clear_values.as_ptr(),
        };
        vkCmdBeginRenderPass(command_buffer, &render_pass_begin_info, VkSubpassContents::VK_SUBPASS_CONTENTS_INLINE);
        let viewport = VkViewport {
            x: 0.0,
            y: area.extent.height as c_float,
            width: area.extent.width as c_float,
            height: -(area.extent.height as c_float),
            minDepth: 0.0,
            maxDepth: 1.0,
        };
        vkCmdSetViewport(command_buffer, 0, 1, &viewport);
        let scissor = area;
        vkCmdSetScissor(command_buffer, 0, 1, &scissor);
        vkCmdBindDescriptorSets(command_buffer, VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, 
            pipeline.layout().handle(), 0, 1, &descriptor_set, 0, ptr::null());
        vkCmdBindPipeline(command_buffer, VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline.handle());
        vkCmdDraw(command_buffer, 3, 1, 0, 0);
        vkCmdEndRenderPass(command_buffer);
    }
}
