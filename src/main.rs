
extern crate kaldera;

use kaldera::ffi::vk::*;
use kaldera::ffi::xcb::*;
use kaldera::vk::*;
use kaldera::base::Environment;
use kaldera::base::*;

use futures::executor::block_on;
use std::sync::Arc;

fn _main() {
    let env = Environment::new();
    block_on(async {
        let input_context = env.input().acquire_xcb().await;
        let window = input_context.create_window(400, 400).await
            .unwrap();

        let base = env.base();
        let surface_context = Render3DSurfaceContext::new(base, window.surface());
        
    });

    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn renderer(device_queues: &Arc<DeviceQueues>, surface: Arc<Surface>) -> Arc<GraphicsRender> {
    let swapchain = Swapchain::new(&device_queues, &surface, VkExtent2D { width: 400, height: 400 }).unwrap();
    let framebuffers = SwapchainFramebuffers::new(&swapchain).unwrap();
    let layout = SceneGraphicsPipelineLayout::new(device_queues.device()).unwrap();
    let pipeline = SceneGraphicsPipeline::new(framebuffers.render_pass(), &layout).unwrap();
    let command_pool = CommandPool::new(device_queues.graphics_queue()).unwrap();
    let vertices = vec![
        Vertex {
            coordinate: Vec3 { x: 1.0, y: 1.0, z: 0.0 },
            color: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        },
        Vertex {
            coordinate: Vec3 { x: -1.0, y: 1.0, z: 0.0 },
            color: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        },
        Vertex {
            coordinate: Vec3 { x: 0.0, y: -1.0, z: 0.0 },
            color: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        },
    ];
    let indices = vec![
        0, 1, 2,
    ];
    let staging_buffer = RenderStagingBuffer::new(&command_pool, vertices, indices);
    let render = GraphicsRender::new(&framebuffers, &pipeline, &staging_buffer, &command_pool).unwrap();
    render
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
    let renderer = renderer(&device_queues, surface);
    for i in 0..100 {
        println!("frame {}", i);
        renderer.draw().unwrap();
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
