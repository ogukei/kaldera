
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

use super::mesh::*;

pub struct Material<'a> {
    material: gltf::material::Material<'a>,
    color_image: &'a gltf::image::Data,
    color_pixels: MaterialImagePixels<'a>,
    normal_image: Option<&'a gltf::image::Data>,
    normal_pixels: Option<MaterialImagePixels<'a>>,
}

impl<'a> Material<'a> {
    pub fn new(material: gltf::material::Material<'a>, images: &'a Vec<gltf::image::Data>) -> Self {
        let model = material.pbr_metallic_roughness();
        let color = model.base_color_texture().unwrap();
        // color
        let color_image_index = color.texture().source().index();
        let color_image = images.get(color_image_index).unwrap();
        let color_pixels = MaterialImagePixels::new(color_image).unwrap();
        // normal
        let normal = material.normal_texture();
        let normal_image_index = normal
            .map(|v| v.texture().source().index());
        let normal_image = normal_image_index
            .map(|index| images.get(index).unwrap());
        let normal_pixels = normal_image
            .map(|image| MaterialImagePixels::new(image).unwrap());
        Self {
            material,
            color_image,
            color_pixels,
            normal_image,
            normal_pixels,
        }
    }

    pub fn color_image(&self) -> &'a gltf::image::Data {
        self.color_image
    }

    pub fn color_pixels(&self) -> &MaterialImagePixels<'a> {
        &self.color_pixels
    }

    pub fn normal_image(&self) -> Option<&'a gltf::image::Data> {
        self.normal_image
    }

    pub fn normal_pixels(&self) -> Option<&MaterialImagePixels<'a>> {
        self.normal_pixels.as_ref()
    }
}

pub enum MaterialImagePixels<'a> {
    Ref(&'a Vec<u8>),
    Vec(Vec<u8>),
}

impl<'a> MaterialImagePixels<'a> {
    fn new(image: &'a gltf::image::Data) -> Option<Self> {
        use gltf::image::Format;
        match image.format {
            Format::R8G8B8 => {
                let bytes = image.width as usize * image.height as usize * 4usize;
                let mut pixels: Vec<u8> = Vec::with_capacity(bytes);
                for rgb in image.pixels.chunks(3) {
                    pixels.extend_from_slice(rgb);
                    pixels.push(0xffu8);
                }
                Some(Self::Vec(pixels))
            },
            Format::R8G8B8A8 => {
                Some(Self::Ref(&image.pixels))
            },
            _ => None,
        }
    }

    pub fn pixels(&self) -> &Vec<u8> {
        match &self {
            &Self::Ref(v) => v,
            &Self::Vec(v) => v,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceneMaterialDescription {
    color_texture_index: i32,
    normal_texture_index: i32,
}

pub struct MaterialDescriptionsTextures {
    pub descriptions: Vec<SceneMaterialDescription>,
    pub textures: Vec<Arc<Texture>>,
}

impl MaterialDescriptionsTextures {
    pub fn new(materials: &[Arc<SceneMeshMaterial>]) -> Self {
        let mut descriptions: Vec<SceneMaterialDescription> = vec![];
        let mut textures: Vec<Arc<Texture>> = vec![];
        for material in materials {
            let color_texture_index = textures.len() as i32;
            let color_texture = material.color_texture();
            textures.push(Arc::clone(color_texture));
            let normal_texture_index: i32;
            if let Some(normal_texture) = material.normal_texture() {
                normal_texture_index = textures.len() as i32;
                textures.push(Arc::clone(normal_texture));
            } else {
                normal_texture_index = -1;
            }
            let desc = SceneMaterialDescription {
                color_texture_index,
                normal_texture_index,
            };
            descriptions.push(desc);
        }
        Self {
            descriptions,
            textures,
        }
    }
}
