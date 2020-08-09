
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

pub struct Material<'a> {
    image: &'a gltf::image::Data,
    material: gltf::material::Material<'a>,
    pixels: MaterialImagePixels<'a>,
}

impl<'a> Material<'a> {
    pub fn new(material: gltf::material::Material<'a>, images: &'a Vec<gltf::image::Data>) -> Self {
        let model = material.pbr_metallic_roughness();
        let color = model.base_color_texture().unwrap();
        let image_index = color.texture().source().index();
        let image = images.get(image_index).unwrap();
        let pixels = MaterialImagePixels::new(image).unwrap();
        Self {
            image,
            material,
            pixels,
        }
    }

    pub fn image(&self) -> &'a gltf::image::Data {
        self.image
    }

    pub fn pixels(&self) -> &MaterialImagePixels<'a> {
        &self.pixels
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
