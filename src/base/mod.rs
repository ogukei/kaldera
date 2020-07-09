
macro_rules! log_debug {
    () => { println!() };
    ($($arg:tt)*) => { 
        let s: &'static str = file!();
        let filename = s.split('/').last().unwrap_or("");
        let label = format!("{}:{}", filename, line!());
        println!("[{}] {}", label, format!($($arg)*)) 
    };
}

mod input;
pub use input::*;

#[cfg(feature = "with-nalgebra")]
mod geometry;
#[cfg(feature = "with-nalgebra")]
pub use geometry::*;

#[cfg(feature = "with-nalgebra")]
mod camera;
#[cfg(feature = "with-nalgebra")]
pub use camera::*;

#[cfg(feature = "with-nalgebra")]
#[cfg(feature = "with-gltf")]
mod scene;
#[cfg(feature = "with-nalgebra")]
#[cfg(feature = "with-gltf")]
pub use scene::*;

