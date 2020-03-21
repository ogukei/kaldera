
extern crate karst;

use karst::ffi::xcb::*;
use karst::vk::*;

fn main() {
    let instance = Instance::new().unwrap();
    let device = DeviceBuilder::new(&instance)
        .build()
        .unwrap();
    let command_pool = CommandPool::new(&device).unwrap();
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

    println!("{:?}", device.handle());
    unsafe { unsafe_main() }
}

unsafe fn unsafe_main() {
    let connection = XcbConnection::new();
    let window = XcbWindow::new(&connection);
    window.flush();
    std::thread::sleep_ms(1000);
}
