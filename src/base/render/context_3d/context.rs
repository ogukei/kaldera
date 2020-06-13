


use crate::dispatch::{Dispatch, DispatchExecutor};

use crate::base::Base;

use std::sync::{Arc, RwLock};
use crate::vk::*;

enum Request {

}

enum Response {

}

// represents context that renders 3D viewports onto surfaces directly
pub struct Render3DSurfaceContext {
    base: Arc<Base>,
    dispatch: Dispatch<Request, Response>,
    executor: DispatchExecutor,
}

impl Render3DSurfaceContext {
    pub fn new(base: &Arc<Base>, surface: &Arc<Surface>) -> Arc<Render3DSurfaceContext> {
        let state = Render3DSurfaceState::new(surface);
        let input_state = Arc::clone(&state);
        let dispatch = Dispatch::new(move |v| {
            match v {
                
            }
        });
        let executor = DispatchExecutor::new();
        let context = Render3DSurfaceContext {
            base: Arc::clone(base),
            dispatch,
            executor,
        };
        let context = Arc::new(context);
        input_state.initialize(&context);
        context
    }

    fn executor(&self) -> &DispatchExecutor {
        &self.executor
    }
}

struct Render3DSurfaceState {
    surface: Arc<Surface>,
    context: RwLock<Option<Arc<Render3DSurfaceContext>>>,
}

unsafe impl Send for Render3DSurfaceState {}
unsafe impl Sync for Render3DSurfaceState {}

impl Render3DSurfaceState {
    fn new(surface: &Arc<Surface>) -> Arc<Self> {
        let device_queues = DeviceQueuesBuilder::new(surface)
            .build()
            .unwrap();
        
        let state = Render3DSurfaceState {
            surface: Arc::clone(surface),
            context: RwLock::new(None),
        };
        Arc::new(state)
    }

    fn initialize(&self, context: &Arc<Render3DSurfaceContext>) {
        let context = Arc::clone(context);
        let mut guard = self.context.write().unwrap();
        guard.replace(context);
    }

    fn context(&self) -> Arc<Render3DSurfaceContext> {
        let guard = self.context.read().unwrap();
        guard.as_ref()
            .map(|v| Arc::clone(v))
            .unwrap()
    }
}
