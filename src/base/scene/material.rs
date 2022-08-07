
use gltf;

use std::sync::{Arc};

use crate::vk::Result;
use crate::vk::*;

use super::image as scene_image;

use super::mesh::*;
use super::image_provider::ImageProvider;


pub struct Material<'a> {
    material: gltf::material::Material<'a>,
}

impl<'a> Material<'a> {
    pub fn new(material: gltf::material::Material<'a>) -> Self {
        Self {
            material,
        }
    }

    pub fn name(&self) -> Option<&'a str> {
        self.material.name()
    }

    fn material(&self) -> &gltf::material::Material<'a> {
        &self.material
    }
}

pub struct MaterialImageSources {
    pub color_image_index: Option<usize>,
    pub normal_image_index: Option<usize>,
}

impl MaterialImageSources {
    fn new(material: &gltf::material::Material) -> Self { 
        let color_image_index = Self::color_image_index(material);
        let normal_image_index = Self::normal_image_index(material);
        Self { color_image_index, normal_image_index }
    }

    fn color_image_index(
        material: &gltf::material::Material,
    ) -> Option<usize>
    {
        let model = material.pbr_metallic_roughness();
        let color = model.base_color_texture()?;
        let image_index = color.texture().source().index();
        Some(image_index)
    }

    fn normal_image_index(
        material: &gltf::material::Material, 
    ) -> Option<usize>
    {
        let normal = material.normal_texture()?;
        let image_index = normal.texture().source().index();
        Some(image_index)
    }
}

pub struct MaterialImageData {
    color_image: Option<scene_image::Data>,
    normal_image: Option<scene_image::Data>,
}

impl MaterialImageData {
    pub fn new(material: &Material, image_provider: &ImageProvider) -> Result<Self> { 
        let sources = MaterialImageSources::new(material.material());
        let color_image = sources.color_image_index
            .map(|index| image_provider.image(index)
                .ok_or_else(|| ErrorCode::ImageNotFound))
            .transpose()?;
        let normal_image = sources.normal_image_index
            .map(|index| image_provider.image(index)
                .ok_or_else(|| ErrorCode::ImageNotFound))
            .transpose()?;
        let this = Self { color_image, normal_image };
        Ok(this)
    }

    pub fn color_image_data(&self) -> Option<&scene_image::Data> {
        self.color_image.as_ref()
    }

    pub fn normal_image_data(&self) -> Option<&scene_image::Data> {
        self.normal_image.as_ref()
    }
}

pub struct MaterialImages<'a> {
    color_image: Option<MaterialImage<'a>>,
    normal_image: Option<MaterialImage<'a>>,
}

impl<'a> MaterialImages<'a> {
    pub fn new(data: &'a MaterialImageData) -> Result<Self> { 
        let color_image = data.color_image_data()
            .map(|data| MaterialImagePixels::new(data)
                .ok_or_else(|| ErrorCode::ImageFormatInvalid)
                .map(|v| MaterialImage::new(v, data.width, data.height)))
            .transpose()?;
        let normal_image = data.normal_image_data()
            .map(|data| MaterialImagePixels::new(data)
                .ok_or_else(|| ErrorCode::ImageFormatInvalid)
                .map(|v| MaterialImage::new(v, data.width, data.height)))
            .transpose()?;
        let this = Self { color_image, normal_image };
        Ok(this)
    }

    pub fn color_image(&self) -> Option<&MaterialImage<'a>> {
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
    fn new(image: &'a scene_image::Data) -> Option<Self> {
        use scene_image::Format;
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
    pub fn new(materials: &[Material], image_provider: &ImageProvider, command_pool: &Arc<CommandPool>) -> Self {
        let materials: Vec<_> = materials.iter()
            .map(|v| SceneMeshMaterial::new(v, image_provider, command_pool))
            //.map(|v| SceneMeshMaterial::new_placeholder(command_pool))
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

    pub fn replace_material(&mut self, command_pool: &Arc<CommandPool>, image_provider: &ImageProvider, material: &Material, material_index: usize) {
        let mesh_material = SceneMeshMaterial::new(material, image_provider, command_pool);
        // color
        let color_texture_index: i32;
        if let Some(color_texture) = mesh_material.color_texture() {
            color_texture_index = self.textures.len() as i32;
            self.textures.push(Arc::clone(color_texture));
        } else {
            color_texture_index = -1;
        }
        // normal
        let normal_texture_index: i32;
        if let Some(normal_texture) = mesh_material.normal_texture() {
            normal_texture_index = self.textures.len() as i32;
            self.textures.push(Arc::clone(normal_texture));
        } else {
            normal_texture_index = -1;
        }
        let desc = SceneMaterialDescription {
            color_texture_index,
            normal_texture_index,
        };
        self.descriptions[material_index] = desc;
    }
}
