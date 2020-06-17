

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
mod surface;
mod device;
mod device_queues;
mod memory;
mod geometry;
mod model;

mod staging;
mod image;
mod swapchain;
mod pipeline;
mod render;
mod offscreen;

pub use error::*;
pub use instance::*;
pub use surface::*;
pub use device::*;
pub use device_queues::*;
pub use memory::*;
pub use geometry::*;
pub use model::*;

pub use staging::*;
pub use image::*;
pub use swapchain::*;
pub use pipeline::*;
pub use render::*;
pub use offscreen::*;
