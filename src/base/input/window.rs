

use std::sync::{Arc, Mutex};
use super::XcbWindowObject;
use crate::vk::Surface;

#[derive(Clone, Copy)]
pub struct WindowConfiguration {
    pub width: usize,
    pub height: usize,
}

pub enum WindowObject {
    Xcb(Arc<XcbWindowObject>)
}

impl WindowObject {
    pub fn surface(&self) -> &Arc<Surface> {
        match &self {
            &Self::Xcb(object) => object.surface(),
        }
    }
}

impl From<Arc<XcbWindowObject>> for WindowObject {
    fn from(xcb: Arc<XcbWindowObject>) -> Self {
        Self::Xcb(xcb)
    }
}
