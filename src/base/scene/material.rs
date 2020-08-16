
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
    color_image_pixels: Option<(&'a gltf::image::Data, MaterialImagePixels<'a>)>,
    normal_image_pixels: Option<(&'a gltf::image::Data, MaterialImagePixels<'a>)>,
}

impl<'a> Material<'a> {
    pub fn new(material: gltf::material::Material<'a>, images: &'a Vec<gltf::image::Data>) -> Self {
        let color_image_pixels = Self::color_info(&material, images);
        let normal_image_pixels = Self::normal_info(&material, images);
        Self {
            material,
            color_image_pixels,
            normal_image_pixels,
        }
    }

    fn color_info(
        material: &gltf::material::Material<'a>, 
        images: &'a Vec<gltf::image::Data>
    ) -> Option<(&'a gltf::image::Data, MaterialImagePixels<'a>)> 
    {
        let model = material.pbr_metallic_roughness();
        let color = model.base_color_texture()?;
        let image_index = color.texture().source().index();
        let image = images.get(image_index)?;
        let pixels = MaterialImagePixels::new(image)?;
        Some((image, pixels))
    }

    fn normal_info(
        material: &gltf::material::Material<'a>, 
        images: &'a Vec<gltf::image::Data>
    ) -> Option<(&'a gltf::image::Data, MaterialImagePixels<'a>)> 
    {
        let normal = material.normal_texture()?;
        let image_index = normal.texture().source().index();
        let image = images.get(image_index)?;
        let pixels = MaterialImagePixels::new(image)?;
        Some((image, pixels))
    }

    pub fn color_image(&self) -> Option<&'a gltf::image::Data> {
        self.color_image_pixels.as_ref()
            .map(|v| v.0)
    }

    pub fn color_pixels(&self) -> Option<&MaterialImagePixels<'a>> {
        self.color_image_pixels.as_ref()
            .map(|v| &v.1)
    }

    pub fn normal_image(&self) -> Option<&'a gltf::image::Data> {
        self.normal_image_pixels.as_ref()
            .map(|v| v.0)
    }

    pub fn normal_pixels(&self) -> Option<&MaterialImagePixels<'a>> {
        self.normal_image_pixels.as_ref()
            .map(|v| &v.1)
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
