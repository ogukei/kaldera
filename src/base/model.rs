
use gltf;
use crate::vk::Vec3;
use crate::vk::Result;

use std::sync::Arc;

pub struct BoxModel {
    vertices: Vec<Vec3>,
    indices: Vec<u32>,
}

impl BoxModel {
    pub fn new() -> Result<Arc<Self>> {
        let (file, buffers, _) = gltf::import("data/models/box.glb").unwrap();
        let mesh = file.meshes()
            .nth(0)
            .unwrap();
        let vertices = mesh.primitives()
            .filter_map(|v| v.reader(|buffer| Some(&buffers[buffer.index()])).read_positions())
            .flat_map(|v| v)
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect::<Vec<Vec3>>();
        let indices = mesh.primitives()
            .filter_map(|v| v.reader(|buffer| Some(&buffers[buffer.index()])).read_indices())
            .map(|v| v.into_u32())
            .flat_map(|v| v)
            .collect::<Vec<u32>>();
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
