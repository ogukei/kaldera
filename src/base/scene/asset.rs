
use gltf;
use nalgebra_glm as glm;

use std::collections::HashMap;
use std::sync::Arc;

use crate::vk::Result;
use crate::vk::*;
use crate::ffi::vk::*;

use libc::c_void;

use gltf::accessor::DataType;
use gltf::accessor::Dimensions;
use gltf::Semantic;

use VkMemoryPropertyFlagBits::*;
use VkBufferUsageFlagBits::*;

pub struct SceneAsset {
    document: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    images: Vec<gltf::image::Data>,
}

impl SceneAsset {
    pub fn new() -> Result<Self> {
        log_debug!("loading scene asset");
        let (document, buffers, images) = gltf::import("submodules/kaldera-asset/models/Sponza/glTF/Sponza.gltf").unwrap();
        log_debug!("loading scene asset complete");
        let asset = Self {
            document,
            buffers,
            images,
        };
        Ok(asset)
    }

    #[inline]
    pub fn document(&self) -> &gltf::Document {
        &self.document
    }

    #[inline]
    pub fn buffers(&self) -> &Vec<gltf::buffer::Data> {
        &self.buffers
    }

    #[inline]
    pub fn images(&self) -> &Vec<gltf::image::Data> {
        &self.images
    }
}
