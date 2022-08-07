
use gltf;
use gltf::{Gltf};
use gltf::image;

use base64;
use image_crate;
use image_crate::ImageFormat::{Jpeg, Png};

use crate::vk::Result;
use crate::vk::*;

use crate::base::scene::image as scene_image;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;


pub struct SceneAsset {
    document: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    base: PathBuf,
}

impl SceneAsset {
    pub fn new<P>(path: P) -> Result<Arc<Self>> where P: AsRef<Path> {
        log_debug!("loading scene asset");
        let path = path.as_ref();
        let base = path.parent().unwrap_or_else(|| Path::new("./"));
        let file = File::open(path)
            .map_err(|_| ErrorCode::Io)?;
        let reader = BufReader::new(file);
        let Gltf { document, blob } = Gltf::from_reader(reader)
            .map_err(|_| ErrorCode::Io)?;
        let buffers = import_buffer_data(&document, Some(base), blob)?;
        log_debug!("loading scene asset complete");
        let asset = Self {
            document,
            buffers,
            base: base.to_owned(),
        };
        Ok(Arc::new(asset))
    }

    #[inline]
    pub fn document(&self) -> &gltf::Document {
        &self.document
    }

    #[inline]
    pub fn buffers(&self) -> &Vec<gltf::buffer::Data> {
        &self.buffers
    }

    pub fn import_image_data(&self, image_index: usize,) -> Result<scene_image::Data> {
        import_image_data(&self.document, Some(self.base.as_path()), &self.buffers, image_index)
    }
}

// ported from the gltf-rs because their library does not support glTF imports excluding images but including documents and buffers
// @see https://docs.rs/gltf/latest/src/gltf/import.rs.html#234-239
fn import_buffer_data(
    document: &gltf::Document,
    base: Option<&Path>,
    mut blob: Option<Vec<u8>>,
) -> Result<Vec<gltf::buffer::Data>> {
    let mut buffers = Vec::new();
    for buffer in document.buffers() {
        let mut data = match buffer.source() {
            gltf::buffer::Source::Uri(uri) => Scheme::read(base, uri),
            gltf::buffer::Source::Bin => blob.take().ok_or(ErrorCode::Io.into()),
        }?;
        if data.len() < buffer.length() {
            return Err(ErrorCode::Io.into());
        }
        while data.len() % 4 != 0 {
            data.push(0);
        }
        buffers.push(gltf::buffer::Data(data));
    }
    Ok(buffers)
}

/// Represents the set of URI schemes the importer supports.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Scheme<'a> {
    /// `data:[<media type>];base64,<data>`.
    Data(Option<&'a str>, &'a str),

    /// `file:[//]<absolute file path>`.
    ///
    /// Note: The file scheme does not implement authority.
    File(&'a str),

    /// `../foo`, etc.
    Relative,

    /// Placeholder for an unsupported URI scheme identifier.
    Unsupported,
}

impl<'a> Scheme<'a> {
    fn parse(uri: &str) -> Scheme<'_> {
        if uri.contains(':') {
            if let Some(rest) = uri.strip_prefix("data:") {
                let mut it = rest.split(";base64,");

                match (it.next(), it.next()) {
                    (match0_opt, Some(match1)) => Scheme::Data(match0_opt, match1),
                    (Some(match0), _) => Scheme::Data(None, match0),
                    _ => Scheme::Unsupported,
                }
            } else if let Some(rest) = uri.strip_prefix("file://") {
                Scheme::File(rest)
            } else if let Some(rest) = uri.strip_prefix("file:") {
                Scheme::File(rest)
            } else {
                Scheme::Unsupported
            }
        } else {
            Scheme::Relative
        }
    }

    fn read(base: Option<&Path>, uri: &str) -> Result<Vec<u8>> {
        match Scheme::parse(uri) {
            // The path may be unused in the Scheme::Data case
            // Example: "uri" : "data:application/octet-stream;base64,wsVHPgA...."
            Scheme::Data(_, base64) => base64::decode(&base64).map_err(|_| ErrorCode::Io.into()),
            Scheme::File(path) if base.is_some() => read_to_end(path),
            Scheme::Relative if base.is_some() => read_to_end(base.unwrap().join(uri)),
            Scheme::Unsupported => Err(ErrorCode::Io.into()),
            _ => Err(ErrorCode::Io.into()),
        }
    }
}

fn read_to_end<P>(path: P) -> Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    use std::io::Read;
    let file = File::open(path.as_ref()).map_err(|_| ErrorCode::Io)?;
    // Allocate one extra byte so the buffer doesn't need to grow before the
    // final `read` call at the end of the file.  Don't worry about `usize`
    // overflow because reading will fail regardless in that case.
    let length = file.metadata().map(|x| x.len() + 1).unwrap_or(0);
    let mut reader = BufReader::new(file);
    let mut data = Vec::with_capacity(length as usize);
    reader.read_to_end(&mut data).map_err(|_| ErrorCode::Io)?;
    Ok(data)
}

fn convert_image(image: image_crate::DynamicImage) -> image_crate::DynamicImage {
    image
}

/// Import the image data referenced by a glTF document.
pub fn import_image_data(
    document: &gltf::Document,
    base: Option<&Path>,
    buffer_data: &[gltf::buffer::Data],
    image_index: usize,
) -> Result<scene_image::Data> {
    let guess_format = |encoded_image: &[u8]| match image_crate::guess_format(encoded_image) {
        Ok(image_crate::ImageFormat::Png) => Some(Png),
        Ok(image_crate::ImageFormat::Jpeg) => Some(Jpeg),
        _ => None,
    };
    let result_image: scene_image::Data;
    let document_image = document.images().nth(image_index)
        .ok_or_else(|| ErrorCode::Io)?;
    match document_image.source() {
        image::Source::Uri { uri, mime_type } if base.is_some() => {
            match Scheme::parse(uri) {
                Scheme::Data(Some(annoying_case), base64) => {
                    let encoded_image = base64::decode(&base64).map_err(|_| ErrorCode::Io)?;
                    let encoded_format = match annoying_case {
                        "image/png" => Png,
                        "image/jpeg" => Jpeg,
                        _ => match guess_format(&encoded_image) {
                            Some(format) => format,
                            None => return Err(ErrorCode::Io.into()),
                        },
                    };
                    let decoded_image = image_crate::load_from_memory_with_format(
                        &encoded_image,
                        encoded_format,
                    ).map_err(|_| ErrorCode::Io)?;
                    let image = convert_image(decoded_image);
                    let image = scene_image::Data::new(image)
                        .ok_or_else(|| ErrorCode::Io)?;
                    return Ok(image);
                }
                Scheme::Unsupported => return Err(ErrorCode::Io.into()),
                _ => {}
            }
            let encoded_image = Scheme::read(base, uri)?;
            let encoded_format = match mime_type {
                Some("image/png") => Png,
                Some("image/jpeg") => Jpeg,
                Some(_) => match guess_format(&encoded_image) {
                    Some(format) => format,
                    None => return Err(ErrorCode::Io.into()),
                },
                None => match uri.rsplit('.').next() {
                    Some("png") => Png,
                    Some("jpg") | Some("jpeg") => Jpeg,
                    _ => match guess_format(&encoded_image) {
                        Some(format) => format,
                        None => return Err(ErrorCode::Io.into()),
                    },
                },
            };
            let decoded_image =
                image_crate::load_from_memory_with_format(&encoded_image, encoded_format)
                    .map_err(|_| ErrorCode::Io)?;
            let image = convert_image(decoded_image);
            let image = scene_image::Data::new(image)
                .ok_or_else(|| ErrorCode::Io)?;
            result_image = image;
        }
        image::Source::View { view, mime_type } => {
            let parent_buffer_data = &buffer_data[view.buffer().index()].0;
            let begin = view.offset();
            let end = begin + view.length();
            let encoded_image = &parent_buffer_data[begin..end];
            let encoded_format = match mime_type {
                "image/png" => Png,
                "image/jpeg" => Jpeg,
                _ => match guess_format(encoded_image) {
                    Some(format) => format,
                    None => return Err(ErrorCode::Io.into()),
                },
            };
            let decoded_image =
                image_crate::load_from_memory_with_format(encoded_image, encoded_format)
                    .map_err(|_| ErrorCode::Io)?;
            let image = convert_image(decoded_image);
            let image = scene_image::Data::new(image)
            .ok_or_else(|| ErrorCode::Io)?;
            result_image = image;
        }
        _ => return Err(ErrorCode::Io.into()),
    }
    Ok(result_image)
}
