
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
    color_image: Option<MaterialImage<'a>>,
    normal_image: Option<MaterialImage<'a>>,
}

impl<'a> Material<'a> {
    pub fn new(material: gltf::material::Material<'a>, images: &'a Vec<gltf::image::Data>) -> Self {
        let color_image = Self::color_info(&material, images);
        let normal_image = Self::normal_info(&material, images);
        Self {
            material,
            color_image,
            normal_image,
        }
    }

    fn color_info(
        material: &gltf::material::Material<'a>, 
        images: &'a Vec<gltf::image::Data>
    ) -> Option<MaterialImage<'a>> 
    {
        let model = material.pbr_metallic_roughness();
        let color = model.base_color_texture()?;
        let image_index = color.texture().source().index();
        let image = images.get(image_index)?;
        let pixels = MaterialImagePixels::new(image)?;
        let image = MaterialImage::new(pixels, image.width, image.height);
        Some(image)
    }

    fn normal_info(
        material: &gltf::material::Material<'a>, 
        images: &'a Vec<gltf::image::Data>
    ) -> Option<MaterialImage<'a>> 
    {
        let normal = material.normal_texture()?;
        let image_index = normal.texture().source().index();
        let image = images.get(image_index)?;
        let pixels = MaterialImagePixels::new(image)?;
        let image = MaterialImage::new(pixels, image.width, image.height);
        Some(image)
    }

    pub fn color_image<'s>(&self) -> Option<&MaterialImage<'a>> {
        self.color_image.as_ref()
    }

    pub fn normal_image(&self) -> Option<&MaterialImage<'a>> {
        self.normal_image.as_ref()
    }
}

pub struct MaterialImage<'a> {
    pixels: MaterialImagePixels<'a>,
    width: u32,
    height: u32,
}

impl<'a> MaterialImage<'a> {
    fn new(
        pixels: MaterialImagePixels<'a>,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceneMaterialDescription {
    color_texture_index: i32,
    normal_texture_index: i32,
}

pub struct MaterialDescriptionsTextures {
    pub descriptions: Vec<SceneMaterialDescription>,
    pub textures: Vec<Arc<Texture>>,
    pub materials: Vec<Arc<SceneMeshMaterial>>,
}

impl MaterialDescriptionsTextures {
    pub fn new(materials: &[Material], command_pool: &Arc<CommandPool>) -> Self {
        let materials: Vec<_> = materials.iter()
            .map(|v| SceneMeshMaterial::new(v, command_pool))
            .collect();
        let mut descriptions: Vec<SceneMaterialDescription> = vec![];
        let mut textures: Vec<Arc<Texture>> = vec![];
        for material in materials.iter() {
            // color
            let color_texture_index: i32;
            if let Some(color_texture) = material.color_texture() {
                color_texture_index = textures.len() as i32;
                textures.push(Arc::clone(color_texture));
            } else {
                color_texture_index = -1;
            }
            // normal
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
            materials,
        }
    }
}
