
use super::geometry::{Vec3, Vec4};

#[repr(C)]
pub struct Vertex {
    pub coordinate: Vec3,
    pub color: Vec3,
}
