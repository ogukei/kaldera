
macro_rules! log_debug {
    () => { println!() };
    ($($arg:tt)*) => { 
        let s: &'static str = file!();
        let filename = s.split('/').last().unwrap_or("");
        let label = format!("{}:{}", filename, line!());
        println!("[{}] {}", label, format!($($arg)*)) 
    };
}

const DEFAULT_TIMEOUT: u64 = 10000000000; // 10sec

mod error;
mod instance;
mod debug;
mod surface;
mod device;
mod device_queues;
mod memory;
mod geometry;
mod model;
mod queue_submit;

mod staging;
mod image;
mod swapchain;
mod render;
mod offscreen;
mod raytrace;
mod scene;

pub use error::*;
pub use instance::*;
pub use debug::*;
pub use surface::*;
pub use device::*;
pub use device_queues::*;
pub use memory::*;
pub use geometry::*;
pub use model::*;
pub use queue_submit::*;

pub use staging::*;
pub use crate::vk::image::*;
pub use swapchain::*;
pub use render::*;
pub use offscreen::*;
pub use raytrace::*;
pub use scene::*;
