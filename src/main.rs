
extern crate karst;

use karst::ffi::xcb::*;
use karst::vk::*;

fn main() {
    let instance = Instance::new().unwrap();
    let device = DeviceBuilder::new(&instance)
        .build()
        .unwrap();
    println!("{:?}", device.handle());
    unsafe { unsafe_main() }
}

unsafe fn unsafe_main() {
    let connection = XcbConnection::new();
    let window = XcbWindow::new(&connection);
    window.flush();
    std::thread::sleep_ms(1000);
}
