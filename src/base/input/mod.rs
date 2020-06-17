
use super::Base;
use crate::dispatch::Dispatch;

use std::sync::{Arc, RwLock};

mod window;
mod xcb;
mod context;

pub use window::*;
pub use xcb::*;
pub use context::*;

pub struct InputSystem {
    dispatch: Dispatch<Request, Response>,
}

impl InputSystem {
    pub fn new(base: &Arc<Base>) -> Arc<InputSystem> {
        let base = Arc::clone(base);
        let state = InputSystemState::new();
        let dispatch = Dispatch::new(move |v| {
            match v {
                Request::AcquireXcbContext => {
                    let context = state.acquire_xcb_context(&base);
                    Response::AcquireXcbContext(context)
                },
                Request::CurrentContext => {
                    let context = state.current_context();
                    Response::CurrentContext(context)
                },
            }
        });
        let input_system = InputSystem {
            dispatch,
        };
        Arc::new(input_system)
    }

    pub async fn acquire_xcb(&self) -> Arc<InputContext> {
        let message = self.dispatch.invoke(Request::AcquireXcbContext).await;
        match message {
            Response::AcquireXcbContext(context) => context,
            _ => unreachable!(),
        }
    }

    pub async fn current_context(&self) -> Option<Arc<InputContext>> {
        let message = self.dispatch.invoke(Request::CurrentContext).await;
        match message {
            Response::CurrentContext(context) => context,
            _ => unreachable!(),
        }
    }
}

struct InputSystemState {
    pub context: RwLock<Option<Arc<InputContext>>>,
}

impl InputSystemState {
    fn new() -> Arc<Self> {
        let state = InputSystemState {
            context: RwLock::new(None),
        };
        Arc::new(state)
    }

    fn acquire_xcb_context(&self, base: &Arc<Base>) -> Arc<InputContext> {
        let mut guard = self.context.write().unwrap();
        let context = guard.get_or_insert_with(
            || Arc::new(InputContext::Xcb(XcbInputContext::new(base))));
        Arc::clone(context)
    }

    fn current_context(&self) -> Option<Arc<InputContext>> {
        let guard = self.context.read().unwrap();
        guard.as_ref()
            .map(|v| Arc::clone(v))
    }
}

enum Request {
    AcquireXcbContext,
    CurrentContext,
}

enum Response {
    AcquireXcbContext(Arc<InputContext>),
    CurrentContext(Option<Arc<InputContext>>),
}
