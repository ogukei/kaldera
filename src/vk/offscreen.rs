
use crate::ffi::vk::*;
use super::error::Result;
use super::device::{Device, CommandPool, CommandBufferBuilder, ShaderModule, ShaderModuleSource};
use super::staging::VertexStagingBuffer;
use super::geometry::{Vec3};
use super::image::{ColorImage, DepthImage};

use std::ptr;
use std::mem::MaybeUninit;
use libc::{c_float};
use std::sync::Arc;
use std::ffi::CString;

pub struct OffscreenRenderPass {
    device: Arc<Device>,
    handle: VkRenderPass,
}

impl OffscreenRenderPass {
    unsafe fn new(device: &Arc<Device>, color_format: VkFormat, depth_format: VkFormat) -> Result<Arc<Self>> {
        let attachments = vec![
            VkAttachmentDescription {
                flags: 0,
                format: color_format,
                samples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                loadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR,
                storeOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE,
                stencilLoadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                stencilStoreOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE,
                initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
                finalLayout: VkImageLayout::VK_IMAGE_LAYOUT_GENERAL,
            },
            VkAttachmentDescription {
                flags: 0,
                format: depth_format,
                samples: VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
                loadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR,
                storeOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE,
                stencilLoadOp: VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                stencilStoreOp: VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE,
                initialLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
                finalLayout: VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            },
        ];
        let color_reference = VkAttachmentReference {
            attachment: 0,
            layout: VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        };
        let depth_reference = VkAttachmentReference {
            attachment: 1,
            layout: VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };
        let subpass_desc = VkSubpassDescription {
            flags: 0,
            pipelineBindPoint: VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
            inputAttachmentCount: 0,
            pInputAttachments: ptr::null(),
            colorAttachmentCount: 1,
            pColorAttachments: &color_reference,
            pResolveAttachments: ptr::null(),
            pDepthStencilAttachment: &depth_reference,
            preserveAttachmentCount: 0,
            pPreserveAttachments: ptr::null(),
        };
        let dependencies = vec![
            VkSubpassDependency {
                srcSubpass: VK_SUBPASS_EXTERNAL,
                dstSubpass: 0,
                srcStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT as VkPipelineStageFlags,
                dstStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as VkPipelineStageFlags,
                srcAccessMask: VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as VkAccessFlags,
                dstAccessMask: VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as VkAccessFlags,
                dependencyFlags: VkDependencyFlagBits::VK_DEPENDENCY_BY_REGION_BIT as VkDependencyFlags,
            },
            VkSubpassDependency {
                srcSubpass: 0,
                dstSubpass: VK_SUBPASS_EXTERNAL,
                srcStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT as VkPipelineStageFlags,
                dstStageMask: VkPipelineStageFlagBits::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT as VkPipelineStageFlags,
                srcAccessMask: VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as VkAccessFlags,
                dstAccessMask: VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as VkAccessFlags,
                dependencyFlags: VkDependencyFlagBits::VK_DEPENDENCY_BY_REGION_BIT as VkDependencyFlags,
            },
        ];
        let create_info = VkRenderPassCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            subpassCount: 1,
            pSubpasses: &subpass_desc,
            dependencyCount: dependencies.len() as u32,
            pDependencies: dependencies.as_ptr(),
        };
        let mut handle = MaybeUninit::<VkRenderPass>::zeroed();
        vkCreateRenderPass(device.handle(), &create_info, ptr::null(), handle.as_mut_ptr())
            .into_result()
            .unwrap();
        let handle = handle.assume_init();
        let render_pass = OffscreenRenderPass {
            device: Arc::clone(device),
            handle,
        };
        Ok(Arc::new(render_pass))
    }

    #[inline]
    pub fn handle(&self) -> VkRenderPass {
        self.handle
    }

    #[inline]
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl Drop for OffscreenRenderPass {
    fn drop(&mut self) {
        log_debug!("Drop OffscreenRenderPass");
        unsafe {
            let device = &self.device;
            vkDestroyRenderPass(device.handle(), self.handle, std::ptr::null());
        }
    }
}

pub struct OffscreenFramebuffer {
    handle: VkFramebuffer,
    device: Arc<Device>,
    color_image: Arc<ColorImage>,
    depth_image: Arc<DepthImage>,
    render_pass: Arc<OffscreenRenderPass>,
}

impl OffscreenFramebuffer {
    pub fn new(device: &Arc<Device>, extent: VkExtent2D) -> Result<Arc<Self>> {
        unsafe {
            Self::init(device, extent)
        }
    }

    unsafe fn init(device: &Arc<Device>, extent: VkExtent2D) -> Result<Arc<Self>> {
        let width = extent.width;
        let height = extent.height;
        let extent = VkExtent3D {
            width,
            height,
            depth: 1,
        };
        let color_image = ColorImage::new(device, extent)?;
        let depth_image = DepthImage::new(device, extent)?;
        let render_pass = OffscreenRenderPass::new(device, color_image.image_format(), depth_image.image_format())?;
        let attachments = vec![
            color_image.view(),
            depth_image.view(),
        ];
        let create_info = VkFramebufferCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            renderPass: render_pass.handle(),
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            width: width,
            height: height,
            layers: 1,
        };
        let mut handle = MaybeUninit::<VkFramebuffer>::zeroed();
        vkCreateFramebuffer(device.handle(), &create_info, ptr::null(), handle.as_mut_ptr())
            .into_result()
            .unwrap();
        let handle = handle.assume_init();
        let framebuffer = Self {
            handle,
            device: Arc::clone(device),
            color_image,
            depth_image,
            render_pass,
        };
        Ok(Arc::new(framebuffer))
    }

    #[inline]
    pub fn render_pass(&self) -> &Arc<OffscreenRenderPass> {
        &self.render_pass
    }

    #[inline]
    pub fn handle(&self) -> VkFramebuffer {
        self.handle
    }

    #[inline]
    pub fn color_image(&self) -> &Arc<ColorImage> {
        &self.color_image
    }

    #[inline]
    pub fn depth_image(&self) -> &Arc<DepthImage> {
        &self.depth_image
    }

    pub fn barrier_initial_layout(&self, command_pool: &Arc<CommandPool>) {
        unsafe {
            let command_buffer = CommandBufferBuilder::new(command_pool).build(|command_buffer| {
                self.color_image().command_barrier_initial_layout(command_buffer);
                self.depth_image().command_barrier_initial_layout(command_buffer);
            });
            let command_buffers = vec![command_buffer.handle()];
            command_pool.queue()
                .submit_then_wait(&command_buffers)
                .unwrap();
        }
    }
}

impl Drop for OffscreenFramebuffer {
    fn drop(&mut self) {
        log_debug!("Drop OffscreenFramebuffer");
        unsafe {
            let device = &self.device;
            vkDestroyFramebuffer(device.handle(), self.handle, std::ptr::null());
        }
    }
}

pub struct OffscreenGraphicsPipelineLayout {
    device: Arc<Device>,
    handle: VkPipelineLayout,
}

impl OffscreenGraphicsPipelineLayout {
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
        let layout = OffscreenGraphicsPipelineLayout {
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

impl Drop for OffscreenGraphicsPipelineLayout {
    fn drop(&mut self) {
        log_debug!("Drop OffscreenGraphicsPipelineLayout");
        unsafe {
            vkDestroyPipelineLayout(self.device.handle(), self.handle, ptr::null());
        }
    }
}

#[allow(dead_code)]
pub struct OffscreenGraphicsPipeline {
    render_pass: Arc<OffscreenRenderPass>,
    layout: Arc<OffscreenGraphicsPipelineLayout>,
    cache: VkPipelineCache,
    handle: VkPipeline,
}

impl OffscreenGraphicsPipeline {
    pub fn new(
        render_pass: &Arc<OffscreenRenderPass>, 
        layout: &Arc<OffscreenGraphicsPipelineLayout>
    ) -> Result<Arc<Self>> {
        unsafe { Self::init(render_pass, layout) }
    }

    unsafe fn init(
        render_pass: &Arc<OffscreenRenderPass>, 
        layout: &Arc<OffscreenGraphicsPipelineLayout>
    ) -> Result<Arc<Self>> {
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
            stride: std::mem::size_of::<Vec3>() as u32,
            inputRate: VkVertexInputRate::VK_VERTEX_INPUT_RATE_VERTEX,
        };
        let vertex_input_attributes = vec![
            VkVertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: VkFormat::VK_FORMAT_R32G32B32_SFLOAT,
                offset: 0,
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
        let vertex_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/raster.triangle.vert.spv")).unwrap();
        let fragment_shader_module = ShaderModule::new(device, ShaderModuleSource::from_file("data/shaders/raster.triangle.frag.spv")).unwrap();
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
        let pipeline = OffscreenGraphicsPipeline {
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
    pub fn render_pass(&self) -> &Arc<OffscreenRenderPass> {
        &self.render_pass
    }

    #[inline]
    pub fn handle(&self) -> VkPipeline {
        self.handle
    }
}

impl Drop for OffscreenGraphicsPipeline {
    fn drop(&mut self) {
        log_debug!("Drop OffscreenGraphicsPipeline");
        unsafe {
            let device = self.render_pass.device();
            vkDestroyPipelineCache(device.handle(), self.cache, ptr::null());
            vkDestroyPipeline(device.handle(), self.handle, ptr::null());
        }
    }
}

#[allow(dead_code)]
pub struct OffscreenGraphicsRender {
    command_pool: Arc<CommandPool>,
    pipeline: Arc<OffscreenGraphicsPipeline>,
    vertex_staging_buffer: Arc<VertexStagingBuffer>,
    framebuffer: Arc<OffscreenFramebuffer>,
}

impl OffscreenGraphicsRender {
    pub fn new(
        command_pool: &Arc<CommandPool>, 
        pipeline: &Arc<OffscreenGraphicsPipeline>,
        offscreen_framebuffer: &Arc<OffscreenFramebuffer>,
        vertex_staging_buffer: &Arc<VertexStagingBuffer>,
    ) -> Result<Arc<Self>> {
        unsafe {
            Self::init(command_pool, pipeline, offscreen_framebuffer, vertex_staging_buffer)
        }
    }

    unsafe fn init(
        command_pool: &Arc<CommandPool>, 
        pipeline: &Arc<OffscreenGraphicsPipeline>,
        offscreen_framebuffer: &Arc<OffscreenFramebuffer>,
        vertex_staging_buffer: &Arc<VertexStagingBuffer>,
    ) -> Result<Arc<Self>> {
        let render = Self {
            command_pool: Arc::clone(command_pool),
            pipeline: Arc::clone(pipeline),
            vertex_staging_buffer: Arc::clone(vertex_staging_buffer),
            framebuffer: Arc::clone(offscreen_framebuffer),
        };
        Ok(Arc::new(render))
    }

    pub unsafe fn command(&self, command_buffer: VkCommandBuffer, area: VkRect2D) {
        let render_pass = self.framebuffer.render_pass();
        let staging_buffer = &self.vertex_staging_buffer;
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
            renderPass: render_pass.handle(),
            framebuffer: self.framebuffer.handle(),
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
        vkCmdBindPipeline(command_buffer, VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS, self.pipeline.handle());
        let offset: VkDeviceSize = 0;
        let vertex_buffer: VkBuffer = staging_buffer.vertex_buffer().device_buffer_memory().buffer();
        vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer, &offset);
        let index_buffer: VkBuffer = staging_buffer.index_buffer().device_buffer_memory().buffer();
        vkCmdBindIndexBuffer(command_buffer, index_buffer, 0, VkIndexType::VK_INDEX_TYPE_UINT32);
        vkCmdDrawIndexed(command_buffer, staging_buffer.index_count() as u32, 1, 0, 0, 0);
        vkCmdEndRenderPass(command_buffer);
    }
}
