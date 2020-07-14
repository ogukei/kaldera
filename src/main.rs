
extern crate kaldera;

use kaldera::ffi::vk::*;
use kaldera::ffi::xcb::*;
use kaldera::vk::*;
use kaldera::base::*;
use std::sync::{Arc, Mutex};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

struct Context {
    camera: Arc<Mutex<OrbitalCamera>>,
    graphics_render: Arc<GraphicsRender>,
    uniform_buffer: Arc<UniformBuffer>,
    device_queues: Arc<DeviceQueues>,
    scene: Option<Scene>,
}

fn main() {
    let instance = InstanceBuilder::new()
        .with_debug()
        .build()
        .unwrap();
    // surface
    let connection = XcbConnection::new();
    let window = XcbWindow::new(&connection, WIDTH as u16, HEIGHT as u16);
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
        }
        camera.update(0.001);
        let model = RayTracingUniformBufferModel {
            view_inverse: camera.view_inverse(),
            proj_inverse: camera.projection_inverse(),
        };
        context.uniform_buffer.update(&vec![model]);
        if let None = context.graphics_render.draw().ok() {
            break
        }
        std::thread::sleep(std::time::Duration::from_millis(15));
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
    let command_pool = CommandPool::new(device_queues.graphics_queue()).unwrap();
    let scene_asset = SceneAsset::new().unwrap();
    let scene = SceneBuilder::new(&scene_asset)
        .build(&command_pool);
    let camera = OrbitalCamera::new(WIDTH as f32, HEIGHT as f32);
    let uniform_buffer_model = RayTracingUniformBufferModel {
        view_inverse: camera.view_inverse(),
        proj_inverse: camera.projection_inverse(),
    };
    let uniform_buffer = UniformBuffer::new(&command_pool, &vec![uniform_buffer_model])
        .unwrap();
    let extent = VkExtent2D {
        width: WIDTH as u32,
        height: HEIGHT as u32,
    };
    let framebuffer = OffscreenFramebuffer::new(device_queues.device(), extent)
        .unwrap();
    let raytracing_pipeline = RayTracingGraphicsPipeline::new(device_queues.device(), scene.textures().len())
        .unwrap();
    let descriptor_sets = RayTracingDescriptorSets::new(
        &raytracing_pipeline, 
        scene.top_level_acceleration_structure(), 
        framebuffer.color_image(),
        &uniform_buffer,
        scene.vertex_staging_buffer(),
        scene.index_staging_buffer(),
        scene.normal_staging_buffer(),
        scene.description_staging_buffer(),
        scene.texcoord_staging_buffer(),
        scene.textures(),
    )
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
        scene: Some(scene),
    }
}

fn rasterization_render(surface: &Arc<Surface>) -> Context {
    let device_queues = DeviceQueuesBuilder::new(&surface)
        .build()
        .unwrap();
    let command_pool = CommandPool::new(device_queues.graphics_queue()).unwrap();
    let camera = OrbitalCamera::new(WIDTH as f32, HEIGHT as f32);
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
        width: WIDTH as u32,
        height: HEIGHT as u32,
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
        scene: None,
    }
}
