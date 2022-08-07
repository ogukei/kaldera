
use super::SceneAsset;
use super::image as scene_image;

pub struct ImageProvider<'a> {
    asset: &'a SceneAsset,
}

impl<'a> ImageProvider<'a> {
    pub fn new(asset: &'a SceneAsset) -> Self { Self { asset } }

    pub fn image(&self, index: usize) -> Option<scene_image::Data> {
        let image = self.asset.import_image_data(index).unwrap();
        Some(image)
    }
}
