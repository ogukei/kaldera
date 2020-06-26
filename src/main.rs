
extern crate kaldera;

use kaldera::ffi::vk::*;
use kaldera::ffi::xcb::*;
use kaldera::vk::*;
use std::sync::Arc;

fn raytracing_render(device_queues: &Arc<DeviceQueues>, surface: &Arc<Surface>) -> Arc<GraphicsRender> {
    let command_pool = CommandPool::new(device_queues.graphics_queue()).unwrap();
    let vertices = vec![
        Vec3 { x: 1.0, y: 1.0, z: 0.0 },
        Vec3 { x: -1.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
    ];
    let indices = vec![
        0, 1, 2,
    ];
    let staging_buffer = AccelerationVertexStagingBuffer::new(&command_pool, vertices, indices);
    let geometry = BottomLevelAccelerationStructureGeometry::new(
        3, 
        std::mem::size_of::<Vec3>() as VkDeviceSize, 
        staging_buffer.vertex_buffer().device_buffer_memory(),
        3,
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
    let camera = Camera::new();
    let model = RayTracingUniformBuffer {
        view_inverse: camera.inv_view(),
        proj_inverse: camera.inv_proj(),
    };
    let uniform_buffer = UniformBuffer::new(&command_pool, model)
        .unwrap();
    let extent = VkExtent2D {
        width: 400,
        height: 400,
    };
    let framebuffer = OffscreenFramebuffer::new(device_queues.device(), extent)
        .unwrap();
    let descriptor_sets = RayTracingDescriptorSets::new(&raytracing_pipeline, &top_level_structure, framebuffer.color_image(), &uniform_buffer)
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
    let graphics_frame_renderer = GraphicsFrameRenderer::new(&raytracing_render, &scene_render).unwrap();
    let graphics_render = GraphicsRender::new(&command_pool, &swapchain_framebuffers, &graphics_frame_renderer, extent)
        .unwrap();
    graphics_render
}

fn main() {
    let instance = Instance::new().unwrap();
    // surface
    let connection = XcbConnection::new();
    let window = XcbWindow::new(&connection);
    let surface = XcbSurface::new(&instance, &window).unwrap();
    // device
    let device_queues = DeviceQueuesBuilder::new(&surface)
        .build()
        .unwrap();
    println!("{:?}", device_queues.device().physical_device().properties_ray_tracing());
    let render = raytracing_render(&device_queues, &surface);
    for i in 0..100 {
        println!("frame {}", i);
        render.draw().unwrap();
        let events = window.events();
        if let Some(events) = events {
            let event_types: Vec<&XcbEventType> = events.iter()
                .filter_map(|v| v.event_type())
                .collect();
            for event_type in event_types {
                match event_type {
                    XcbEventType::KeyPress(event) => {
                        println!("KeyPress {}", event.detail);
                    },
                    XcbEventType::KeyRelease(event) => {
                        println!("KeyRelease {}", event.detail);
                    },
                    XcbEventType::ButtonPress(event) => {
                        println!("ButtonPress {} {}", event.event_x, event.event_y);
                    },
                    XcbEventType::ButtonRelease(event) => {
                        println!("ButtonRelease {} {}", event.event_x, event.event_y);
                    },
                    XcbEventType::MotionNotify(event) => {
                        println!("MotionNotify {} {}", event.event_x, event.event_y);
                    },
                    XcbEventType::ConfigureNotify(event) => {
                        println!("ConfigureNotify {} {}", event.width, event.height);
                    }
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

}
