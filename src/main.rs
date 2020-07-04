
extern crate kaldera;

use kaldera::ffi::vk::*;
use kaldera::ffi::xcb::*;
use kaldera::vk::*;
use kaldera::base::*;
use std::sync::{Arc, Mutex};

struct Context {
    camera: Arc<Mutex<OrbitalCamera>>,
    graphics_render: Arc<GraphicsRender>,
    uniform_buffer: Arc<UniformBuffer>,
    device_queues: Arc<DeviceQueues>,
}

fn main() {
    let instance = InstanceBuilder::new()
        .with_debug()
        .build()
        .unwrap();
    // surface
    let connection = XcbConnection::new();
    let window = XcbWindow::new(&connection, 800, 800);
    let surface = XcbSurface::new(&instance, &window).unwrap();
    // choose render strategy
    let capabilities = PhysicalDeviceCapabilities::new(&instance).unwrap();
    let context = if capabilities.has_raytracing() { 
        raytracing_render(&surface) 
    } else {
        rasterization_render(&surface)
    };
    // render loop
    let interpreter = XcbInputInterpreter::new(&window);
    let mut camera = context.camera.lock().unwrap();
    loop {
        if let Some(event) = interpreter.next() {
            camera.apply(event);
            let model = RayTracingUniformBufferModel {
                view_inverse: camera.view_inverse(),
                proj_inverse: camera.projection_inverse(),
            };
            context.uniform_buffer.update(&vec![model]);
        }
        if let None = context.graphics_render.draw().ok() {
            break
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    context.device_queues.graphics_queue()
        .wait_idle()
        .unwrap();
}

fn raytracing_render(surface: &Arc<Surface>) -> Context {
    let device_queues = DeviceQueuesBuilder::new(&surface)
        .with_raytracing()
        .build()
        .unwrap();
    let model = BoxModel::new().unwrap();
    let vertices = model.vertices();
    let indices = model.indices();
    let vertex_normals = model.vertex_normals();
    let num_vertices = vertices.len() as u32;
    let num_indices = indices.len() as u32;
    let command_pool = CommandPool::new(device_queues.graphics_queue()).unwrap();
    let staging_buffer = AccelerationVertexStagingBuffer::new(&command_pool, &vertices, &indices);
    let geometry = BottomLevelAccelerationStructureGeometry::new(
        num_vertices, 
        std::mem::size_of::<Vec3>() as VkDeviceSize, 
        staging_buffer.vertex_buffer().device_buffer_memory(),
        num_indices,
        staging_buffer.index_buffer().device_buffer_memory(),
    )
        .unwrap();
    let geometries = vec![geometry];
    let bottom_level_structure = BottomLevelAccelerationStructure::new(&command_pool, geometries)
        .unwrap();
    let top_level_structure = TopLevelAccelerationStructure::new(&command_pool, &bottom_level_structure)
        .unwrap();
    let raytracing_pipeline = RayTracingGraphicsPipeline::new(device_queues.device())
        .unwrap();
    let camera = OrbitalCamera::new();
    let uniform_buffer_model = RayTracingUniformBufferModel {
        view_inverse: camera.view_inverse(),
        proj_inverse: camera.projection_inverse(),
    };
    let uniform_buffer = UniformBuffer::new(&command_pool, &vec![uniform_buffer_model])
        .unwrap();
    let extent = VkExtent2D {
        width: 800,
        height: 800,
    };
    let framebuffer = OffscreenFramebuffer::new(device_queues.device(), extent)
        .unwrap();
    let vertex_storage_buffer = StorageBuffer::new(&command_pool, &vertex_normals).unwrap();
    let index_storage_buffer = StorageBuffer::new(&command_pool, &indices).unwrap();
    let descriptor_sets = RayTracingDescriptorSets::new(
        &raytracing_pipeline, 
        &top_level_structure, 
        framebuffer.color_image(), 
        &uniform_buffer, 
        &vertex_storage_buffer,
        &index_storage_buffer)
        .unwrap();
    let raytracing_render = RayTracingGraphicsRender::new(&command_pool, &raytracing_pipeline, &descriptor_sets)
        .unwrap();
    framebuffer.barrier_initial_layout(&command_pool);
    // scene
    let swapchain = Swapchain::new(&device_queues, &surface, extent).unwrap();
    let swapchain_framebuffers = SwapchainFramebuffers::new(&swapchain).unwrap();
    let scene_pipeline_layout = SceneGraphicsPipelineLayout::new(device_queues.device(), framebuffer.color_image()).unwrap();
    let scene_pipeline = SceneGraphicsPipeline::new(swapchain_framebuffers.render_pass(), &scene_pipeline_layout).unwrap();
    let scene_render = SceneGraphicsRender::new(&scene_pipeline).unwrap();
    // graphics
    let graphics_frame_renderer = GraphicsFrameRayTracingRenderer::new(&raytracing_render, &scene_render).unwrap();
    let graphics_frame_renderer = GraphicsFrameRenderer::raytracing(&graphics_frame_renderer);
    let graphics_render = GraphicsRender::new(&command_pool, &swapchain_framebuffers, &graphics_frame_renderer, extent)
        .unwrap();
    Context {
        camera: Arc::new(Mutex::new(camera)),
        graphics_render,
        uniform_buffer,
        device_queues,
    }
}

fn rasterization_render(surface: &Arc<Surface>) -> Context {
    let device_queues = DeviceQueuesBuilder::new(&surface)
        .build()
        .unwrap();
    let command_pool = CommandPool::new(device_queues.graphics_queue()).unwrap();
    let camera = OrbitalCamera::new();
    let uniform_buffer_model = RayTracingUniformBufferModel {
        view_inverse: camera.view_inverse(),
        proj_inverse: camera.projection_inverse(),
    };
    let model = TriangleModel::new().unwrap();
    let vertices = model.vertices();
    let indices = model.indices();
    let vertex_staging_buffer = VertexStagingBuffer::new(&command_pool, &vertices, &indices);
    let uniform_buffer = UniformBuffer::new(&command_pool, &vec![uniform_buffer_model])
        .unwrap();
    let extent = VkExtent2D {
        width: 800,
        height: 800,
    };
    let swapchain = Swapchain::new(&device_queues, &surface, extent).unwrap();
    let swapchain_framebuffers = SwapchainFramebuffers::new(&swapchain).unwrap();
    let offscreen_framebuffer = OffscreenFramebuffer::new(device_queues.device(), extent)
        .unwrap();
    offscreen_framebuffer.barrier_initial_layout(&command_pool);
    let offscreen_pipeline_layout = OffscreenGraphicsPipelineLayout::new(device_queues.device()).unwrap();
    let offscreen_pipeline = OffscreenGraphicsPipeline::new(offscreen_framebuffer.render_pass(), &offscreen_pipeline_layout).unwrap();
    let offscreen_render = OffscreenGraphicsRender::new(&command_pool, &offscreen_pipeline, &offscreen_framebuffer, &vertex_staging_buffer)
        .unwrap();
    // scene
    let scene_pipeline_layout = SceneGraphicsPipelineLayout::new(device_queues.device(), offscreen_framebuffer.color_image()).unwrap();
    let scene_pipeline = SceneGraphicsPipeline::new(swapchain_framebuffers.render_pass(), &scene_pipeline_layout).unwrap();
    let scene_render = SceneGraphicsRender::new(&scene_pipeline).unwrap();
    // graphics
    let graphics_frame_renderer = GraphicsFrameRasterizationRenderer::new(&offscreen_render, &scene_render).unwrap();
    let graphics_frame_renderer = GraphicsFrameRenderer::rasterization(&graphics_frame_renderer);
    let graphics_render = GraphicsRender::new(&command_pool, &swapchain_framebuffers, &graphics_frame_renderer, extent)
        .unwrap();
    Context {
        camera: Arc::new(Mutex::new(camera)),
        graphics_render,
        uniform_buffer,
        device_queues,
    }
}
