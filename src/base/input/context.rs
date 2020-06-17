

use super::Base;
use crate::dispatch::Dispatch;
use crate::vk::Result;

use super::window::*;
use super::xcb::*;

use std::sync::{Arc, Mutex};

pub enum InputContext {
    Xcb(Arc<XcbInputContext>)
}

impl InputContext {
    pub async fn create_window(&self, width: usize, height: usize) -> Result<Arc<WindowObject>> {
        let configuration = WindowConfiguration {
            width, 
            height,
        };
        match &self {
            &Self::Xcb(context) => {
                let window = context.create_window(configuration).await?;
                Ok(Arc::new(window.into()))
            }
        }
    }

    #[inline]
    pub fn base(&self) -> &Arc<Base> {
        match &self {
            &Self::Xcb(context) => context.base(),
        }
    }
}

impl From<Arc<XcbInputContext>> for InputContext {
    fn from(xcb: Arc<XcbInputContext>) -> Self {
        Self::Xcb(xcb)
    }
}
