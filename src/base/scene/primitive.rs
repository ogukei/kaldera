
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

pub struct Primitive<'a> {
    indices: Indices<'a>,
    positions: Positions<'a>,
    normals: Normals<'a>,
    texcoords: Texcoords<'a>,
    material_index: Option<usize>,
    is_opaque: bool,
}

impl<'a> Primitive<'a> {
    pub fn new(primitive: gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let material_index = primitive.material().index();
        let is_opaque = primitive.material().alpha_mode() == gltf::material::AlphaMode::Opaque;
        Self {
            indices: Indices::new(&primitive, buffers),
            positions: Positions::new(&primitive, buffers),
            normals: Normals::new(&primitive, buffers),
            // TODO(ogukei): support default TEXCOORD_0
            texcoords: Texcoords::new(&primitive, buffers).unwrap(),
            material_index,
            is_opaque,
        }
    }

    #[inline]
    pub fn is_opaque(&self) -> bool {
        self.is_opaque
    }

    #[inline]
    pub fn material_index(&self) -> Option<usize> {
        self.material_index
    }

    #[inline]
    pub fn positions(&self) -> &Positions<'a> {
        &self.positions
    }

    #[inline]
    pub fn indices(&self) -> &Indices<'a> {
        &self.indices
    }

    #[inline]
    pub fn normals(&self) -> &Normals<'a> {
        &self.normals
    }

    #[inline]
    pub fn texcoords(&self) -> &Texcoords<'a> {
        &self.texcoords
    }
}

pub enum Indices<'a> {
    Accessor(AccessorIndicesU32<'a>),
    Vector(Vec<u32>),
}

impl<'a> Indices<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let indices = primitive.indices().unwrap();
        let view = indices.view().unwrap();
        let use_reference = view.stride() == None
            && indices.data_type() == DataType::U32
            && indices.dimensions() == Dimensions::Scalar;
        if use_reference {
            Self::Accessor(AccessorIndicesU32::new(primitive, buffers))
        } else {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let indices = reader.read_indices().unwrap()
                .into_u32()
                .collect();
            Self::Vector(indices)
        }
    }

    #[inline]
    pub fn count(&self) -> usize {
        match &self {
            &Self::Accessor(indices) => indices.count,
            &Self::Vector(v) => v.len(),
        }
    }

    #[inline]
    pub fn data(&self) -> *const u8 {
        match &self {
            &Self::Accessor(indices) => indices.slice.as_ptr(),
            &Self::Vector(v) => v.as_ptr() as *const _ as *const u8,
        }
    }
}

pub struct AccessorIndicesU32<'a> {
    slice: &'a [u8],
    count: usize,
}

impl<'a> AccessorIndicesU32<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let indices = primitive.indices().unwrap();
        let view = indices.view().unwrap();
        let buffer = buffers.get(view.buffer().index()).unwrap();
        let offset = view.offset() + indices.offset();
        let slice = &buffer[offset..offset + view.length()];
        Self {
            slice,
            count: indices.count(),
        }
    }
}

pub enum Positions<'a> {
    Accessor(AccessorPositions<'a>),
    Vector(Vec<[f32; 3]>)
}

impl<'a> Positions<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let positions = primitive.attributes()
            .find_map(|(semantic, accessor)| 
                match semantic { 
                    Semantic::Positions => Some(accessor),
                    _ => None,
                }
            )
            .unwrap();
        let view = positions.view().unwrap();
        let use_reference = view.stride() == None
            && positions.data_type() == DataType::F32 
            && positions.dimensions() == Dimensions::Vec3;
        if use_reference {
            Self::Accessor(AccessorPositions::new(&positions, buffers))
        } else {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions = reader.read_positions().unwrap();
            Self::Vector(positions.collect())
        }
    }

    #[inline]
    pub fn count(&self) -> usize {
        match &self {
            &Self::Accessor(positions) => positions.count,
            &Self::Vector(v) => v.len(),
        }
    }

    #[inline]
    pub fn data(&self) -> *const u8 {
        match &self {
            &Self::Accessor(positions) => positions.slice.as_ptr(),
            &Self::Vector(v) => v.as_ptr() as *const _ as *const u8,
        }
    }
}

pub struct AccessorPositions<'a> {
    slice: &'a [u8],
    count: usize,
}

impl<'a> AccessorPositions<'a> {
    fn new(positions: &gltf::Accessor<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let view = positions.view().unwrap();
        let buffer = buffers.get(view.buffer().index()).unwrap();
        let offset = view.offset() + positions.offset();
        let slice = &buffer[offset..offset + view.length()];
        Self {
            slice,
            count: positions.count(),
        }
    }
}

pub enum Normals<'a> {
    Accessor(AccessorNormals<'a>),
    Vector(Vec<[f32; 3]>)
}

impl<'a> Normals<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let normals = primitive.attributes()
            .find_map(|(semantic, accessor)| 
                match semantic { 
                    Semantic::Normals => Some(accessor),
                    _ => None,
                }
            )
            .unwrap();
        let view = normals.view().unwrap();
        let use_reference = view.stride() == None
            && normals.data_type() == DataType::F32 
            && normals.dimensions() == Dimensions::Vec3;
        if use_reference {
            Self::Accessor(AccessorNormals::new(&normals, buffers))
        } else {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let normals = reader.read_normals().unwrap();
            Self::Vector(normals.collect())
        }
    }

    #[inline]
    pub fn count(&self) -> usize {
        match &self {
            &Self::Accessor(normals) => normals.count,
            &Self::Vector(v) => v.len(),
        }
    }

    #[inline]
    pub fn data(&self) -> *const u8 {
        match &self {
            &Self::Accessor(normals) => normals.slice.as_ptr(),
            &Self::Vector(v) => v.as_ptr() as *const _ as *const u8,
        }
    }
}

pub struct AccessorNormals<'a> {
    slice: &'a [u8],
    count: usize,
}

impl<'a> AccessorNormals<'a> {
    fn new(normals: &gltf::Accessor<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let view = normals.view().unwrap();
        let buffer = buffers.get(view.buffer().index()).unwrap();
        let offset = view.offset() + normals.offset();
        let slice = &buffer[offset..offset + view.length()];
        Self {
            slice,
            count: normals.count(),
        }
    }
}

pub enum Texcoords<'a> {
    Accessor(AccessorTexcoords<'a>),
    Vector(Vec<[f32; 2]>)
}

impl<'a> Texcoords<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Option<Self> {
        let texcoords = primitive.attributes()
            .find_map(|(semantic, accessor)| 
                match semantic { 
                    Semantic::TexCoords(0) => Some(accessor),
                    _ => None,
                }
            )?;
        let view = texcoords.view().unwrap();
        let use_reference = view.stride() == None
            && texcoords.data_type() == DataType::F32 
            && texcoords.dimensions() == Dimensions::Vec2;
        if use_reference {
            Some(Self::Accessor(AccessorTexcoords::new(&texcoords, buffers)))
        } else {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let texcoords = reader.read_tex_coords(0).unwrap().into_f32();
            Some(Self::Vector(texcoords.collect()))
        }
    }

    #[inline]
    pub fn count(&self) -> usize {
        match &self {
            &Self::Accessor(texcoords) => texcoords.count,
            &Self::Vector(v) => v.len(),
        }
    }

    #[inline]
    pub fn data(&self) -> *const u8 {
        match &self {
            &Self::Accessor(texcoords) => texcoords.slice.as_ptr(),
            &Self::Vector(v) => v.as_ptr() as *const _ as *const u8,
        }
    }
}

pub struct AccessorTexcoords<'a> {
    slice: &'a [u8],
    count: usize,
}

impl<'a> AccessorTexcoords<'a> {
    fn new(texcoords: &gltf::Accessor<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let view = texcoords.view().unwrap();
        let buffer = buffers.get(view.buffer().index()).unwrap();
        let offset = view.offset() + texcoords.offset();
        let slice = &buffer[offset..offset + view.length()];
        Self {
            slice,
            count: texcoords.count(),
        }
    }
}
