
extern crate karst;

use karst::ffi::vk::*;
use karst::ffi::xcb::*;
use karst::vk::*;

fn main() {
    let instance = Instance::new().unwrap();

    let connection = XcbConnection::new();
    let window = XcbWindow::new(&connection);
    let surface = XcbSurface::new(&instance, &window).unwrap();

    let device_queues = DeviceQueuesBuilder::new(&surface)
        .build()
        .unwrap();
    let swapchain = Swapchain::new(&device_queues, VkExtent2D { width: 400, height: 400 }).unwrap();
    let command_pool = CommandPool::new(device_queues.graphics_queue()).unwrap();
    let vertices = vec![
        Vertex {
            coordinate: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
        },
        Vertex {
            coordinate: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
        },
        Vertex {
            coordinate: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
            color: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
        },
    ];
    let indices = vec![
        1, 2, 3,
    ];
    let staging_buffer = RenderStagingBuffer::new(&command_pool, vertices, indices);

    println!("{:?}", device_queues.device().handle());
    unsafe {
        window.flush();
        std::thread::sleep_ms(1000);
    }
}
