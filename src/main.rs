
extern crate karst;

use karst::ffi::*;

fn main() {
    unsafe { unsafe_main() }
}


unsafe fn unsafe_main() {
    let connection = XcbConnection::new();
    let window = XcbWindow::new(&connection);
    window.flush();
    std::thread::sleep_ms(1000);
}
