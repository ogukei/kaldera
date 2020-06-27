
use super::geometry::{Vec3, Vec4, Mat4};
use super::Result;

use std::sync::Arc;

#[repr(C)]
pub struct Vertex {
    pub coordinate: Vec3,
    pub color: Vec3,
}

#[repr(C)]
pub struct RayTracingUniformBufferModel {
    pub view_inverse: Mat4,
    pub proj_inverse: Mat4,
}

pub struct TriangleModel {
    vertices: Vec<Vec3>,
    indices: Vec<u32>,
}

impl TriangleModel {
    pub fn new() -> Result<Arc<Self>> {
        let vertices = vec![
            Vec3 { x: 1.0, y: 1.0, z: 0.0 },
            Vec3 { x: -1.0, y: 1.0, z: 0.0 },
            Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        ];
        let indices = vec![
            0, 1, 2,
        ];
        let model = Self {
            vertices,
            indices,
        };
        Ok(Arc::new(model))
    }

    pub fn vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }
}
