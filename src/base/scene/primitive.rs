
use gltf;

use gltf::accessor::DataType;
use gltf::accessor::Dimensions;
use gltf::Semantic;

pub struct Primitive<'a> {
    indices: Indices<'a>,
    positions: Positions<'a>,
    normals: Normals<'a>,
    texcoords: Texcoords<'a>,
    tangents: Option<Tangents<'a>>,
    colors: Option<Colors<'a>>,
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
            tangents: Tangents::new(&primitive, buffers),
            colors: Colors::new(&primitive, buffers),
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

    #[inline]
    pub fn tangents(&self) -> Option<&Tangents<'a>> {
        self.tangents.as_ref()
    }

    #[inline]
    pub fn colors(&self) -> Option<&Colors> {
        self.colors.as_ref()
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

pub enum Tangents<'a> {
    Accessor(AccessorTangents<'a>),
    Vector(Vec<[f32; 4]>)
}

impl<'a> Tangents<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Option<Self> {
        let tangents = primitive.attributes()
            .find_map(|(semantic, accessor)| 
                match semantic { 
                    Semantic::Tangents => Some(accessor),
                    _ => None,
                }
            )?;
        let view = tangents.view().unwrap();
        let use_reference = view.stride() == None
            && tangents.data_type() == DataType::F32 
            && tangents.dimensions() == Dimensions::Vec4;
        let tangents = if use_reference {
            Self::Accessor(AccessorTangents::new(&tangents, buffers))
        } else {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let tangents = reader.read_tangents().unwrap();
            Self::Vector(tangents.collect())
        };
        Some(tangents)
    }

    #[inline]
    pub fn count(&self) -> usize {
        match &self {
            &Self::Accessor(tangents) => tangents.count,
            &Self::Vector(v) => v.len(),
        }
    }

    #[inline]
    pub fn data(&self) -> *const u8 {
        match &self {
            &Self::Accessor(tangents) => tangents.slice.as_ptr(),
            &Self::Vector(v) => v.as_ptr() as *const _ as *const u8,
        }
    }
}

pub struct AccessorTangents<'a> {
    slice: &'a [u8],
    count: usize,
}

impl<'a> AccessorTangents<'a> {
    fn new(tangents: &gltf::Accessor<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let view = tangents.view().unwrap();
        let buffer = buffers.get(view.buffer().index()).unwrap();
        let offset = view.offset() + tangents.offset();
        let slice = &buffer[offset..offset + view.length()];
        Self {
            slice,
            count: tangents.count(),
        }
    }
}

pub enum Colors<'a> {
    Accessor(AccessorColors<'a>),
    Vector(Vec<[f32; 4]>)
}

impl<'a> Colors<'a> {
    fn new(primitive: &gltf::Primitive<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Option<Self> {
        let colors= primitive.attributes()
            .find_map(|(semantic, accessor)| 
                match semantic { 
                    Semantic::Colors(set) => if set == 0 { Some(accessor) } else { None },
                    _ => None,
                }
            )?;
        let view = colors.view().unwrap();
        let use_reference = view.stride() == None
            && colors.data_type() == DataType::F32
            && colors.dimensions() == Dimensions::Vec4;
        let colors = if use_reference {
            Self::Accessor(AccessorColors::new(&colors, buffers))
        } else {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let colors = reader.read_colors(0).unwrap();
            Self::Vector(colors.into_rgba_f32().collect())
        };
        Some(colors)
    }

    #[inline]
    pub fn count(&self) -> usize {
        match &self {
            &Self::Accessor(colors) => colors.count,
            &Self::Vector(v) => v.len(),
        }
    }

    #[inline]
    pub fn data(&self) -> *const u8 {
        match &self {
            &Self::Accessor(colors) => colors.slice.as_ptr(),
            &Self::Vector(v) => v.as_ptr() as *const _ as *const u8,
        }
    }
}

pub struct AccessorColors<'a> {
    slice: &'a [u8],
    count: usize,
}

impl<'a> AccessorColors<'a> {
    fn new(colors: &gltf::Accessor<'a>, buffers: &'a Vec<gltf::buffer::Data>) -> Self {
        let view = colors.view().unwrap();
        let buffer = buffers.get(view.buffer().index()).unwrap();
        let offset = view.offset() + colors.offset();
        let slice = &buffer[offset..offset + view.length()];
        Self {
            slice,
            count: colors.count(),
        }
    }
}
