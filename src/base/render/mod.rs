
use super::Base;
use crate::dispatch::Dispatch;

use std::sync::{Arc, RwLock};

use crate::vk::{
    Instance,
};

mod context_3d;
pub use context_3d::*;

enum Request {

}

enum Response {
    
}

pub struct RenderSystem {
    dispatch: Dispatch<Request, Response>,
    state: Arc<RenderSystemState>,
}

impl RenderSystem {
    pub fn new(base: &Arc<Base>) -> Arc<RenderSystem> {
        let base = Arc::clone(base);
        let state = RenderSystemState::new();
        let dispatch = Dispatch::new(move |v| {
            match v {
                
            }
        });
        let render_system = RenderSystem {
            dispatch,
            state,
        };
        Arc::new(render_system)
    }
}

struct RenderSystemState {
    
}

impl RenderSystemState {
    fn new() -> Arc<Self> {
        let state = RenderSystemState {
        };
        Arc::new(state)
    }
}
