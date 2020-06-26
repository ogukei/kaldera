
use super::geometry::{Vec3, Vec4, Mat4};

#[repr(C)]
pub struct Vertex {
    pub coordinate: Vec3,
    pub color: Vec3,
}

#[repr(C)]
pub struct RayTracingUniformBuffer {
    pub view_inverse: Mat4,
    pub proj_inverse: Mat4,
}
