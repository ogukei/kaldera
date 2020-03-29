

macro_rules! log_debug {
    ($e:expr) => {
        println!("Drop {:?}", $e)
    };
}

mod error;
mod instance;
mod surface;
mod device;
mod device_queues;
mod memory;

mod swapchain;
mod pipeline;
mod render;

pub use error::*;
pub use instance::*;
pub use surface::*;
pub use device::*;
pub use device_queues::*;
pub use memory::*;
pub use swapchain::*;
pub use pipeline::*;
pub use render::*;
