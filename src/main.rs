
extern crate kaldera;

use kaldera::ffi::vk::*;
use kaldera::ffi::xcb::*;
use kaldera::vk::*;
use kaldera::base::Environment;
use kaldera::base::*;
use futures::executor::block_on;
use std::sync::Arc;

fn rasterize_renderer(device_queues: &Arc<DeviceQueues>, surface: Arc<Surface>) -> Arc<GraphicsRender> {
    let device = device_queues.device();
    let swapchain = Swapchain::new(&device_queues, &surface, VkExtent2D { width: 400, height: 400 }).unwrap();
    let framebuffers = SwapchainFramebuffers::new(&swapchain).unwrap();
    let extent_2d = VkExtent2D {
        width: 400,
        height: 400,
    };
    let extent_3d = VkExtent3D {
        width: 400,
        height: 400,
        depth: 1,
    };
    let offscreen_framebuffer = OffscreenFramebuffer::new(device, extent_3d, extent_2d.width, extent_2d.height).unwrap();
    let offscreen_layout = OffscreenGraphicsPipelineLayout::new(device).unwrap();
    let offscreen_pipeline = OffscreenGraphicsPipeline::new(offscreen_framebuffer.render_pass(), &offscreen_layout).unwrap();
    // scene
    let scene_image_info = VkDescriptorImageInfo {
        imageLayout: VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
        imageView: offscreen_framebuffer.color_image().view(),
        sampler: offscreen_framebuffer.color_image().sampler(),
    };
    let scene_layout = SceneGraphicsPipelineLayout::new(device, scene_image_info).unwrap();
    let scene_pipeline = SceneGraphicsPipeline::new(framebuffers.render_pass(), &scene_layout).unwrap();
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
    let staging_buffer = VertexStagingBuffer::new(&command_pool, vertices, indices);
    let render = GraphicsRender::new(
        &command_pool, 
        &framebuffers, 
        &offscreen_framebuffer, 
        &offscreen_pipeline, 
        &scene_pipeline, 
        &staging_buffer, 
        extent_2d).unwrap();
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
    let pipeline = RayTracingGraphicsPipeline::new(device_queues.device())
        .unwrap();
    let model = RayTracingUniformBuffer {
        proj_inverse: Default::default(),
        view_inverse: Default::default(),
    };
    let uniform_buffer = UniformBuffer::new(&command_pool, model)
        .unwrap();
    let extent = VkExtent2D {
        width: 400,
        height: 400,
    };
    let storage_image = StorageImage::new(&command_pool, extent)
        .unwrap();
    let descriptors = RayTracingDescriptors::new(&pipeline, &top_level_structure, &storage_image, &uniform_buffer)
        .unwrap();
    // let renderer = renderer(&device_queues, surface);
    for i in 0..100 {
        println!("frame {}", i);
        //renderer.draw().unwrap();
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
