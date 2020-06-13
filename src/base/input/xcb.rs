


use super::Base;
use crate::dispatch::{Dispatch, DispatchExecutor};

use super::window::*;
use super::context::*;

use crate::ffi::xcb::*;
use crate::vk::{Surface, XcbSurface, Result};

use std::sync::{Arc, RwLock};

enum Request {
    CreateWindow(WindowConfiguration),
    DestroyWindow(Arc<XcbWindowInner>),
}

enum Response {
    CreateWindow(Result<Arc<XcbWindowObject>>),
    DestroyWindow,
}

pub struct XcbInputContext {
    base: Arc<Base>,
    dispatch: Dispatch<Request, Response>,
    executor: DispatchExecutor,
}

impl XcbInputContext {
    pub fn new(base: &Arc<Base>) -> Arc<XcbInputContext> {
        let state = XcbInputState::new();
        let input_state = Arc::clone(&state);
        let dispatch = Dispatch::new(move |v| {
            match v {
                Request::CreateWindow(configuration) => {
                    let window = state.create_window(configuration);
                    Response::CreateWindow(window)
                },
                Request::DestroyWindow(inner) => {
                    state.destroy_window(inner);
                    Response::DestroyWindow
                },
            }
        });
        let executor = DispatchExecutor::new();
        let context = XcbInputContext {
            base: Arc::clone(base),
            dispatch,
            executor,
        };
        let context = Arc::new(context);
        input_state.initialize(&context);
        context
    }

    pub async fn create_window(&self, configuration: WindowConfiguration) -> Result<Arc<XcbWindowObject>> {
        let message = self.dispatch.invoke(Request::CreateWindow(configuration)).await;
        match message {
            Response::CreateWindow(window) => window,
            _ => unreachable!(),
        }
    }

    async fn destroy_window(&self, object: Arc<XcbWindowInner>) {
        let message = self.dispatch.invoke(Request::DestroyWindow(object)).await;
        match message {
            Response::DestroyWindow => (),
            _ => unreachable!(),
        }
    }

    fn executor(&self) -> &DispatchExecutor {
        &self.executor
    }

    #[inline]
    pub fn base(&self) -> &Arc<Base> {
        &self.base
    }
}

struct XcbInputState {
    connection: Arc<XcbConnection>,
    context: RwLock<Option<Arc<XcbInputContext>>>,
}

unsafe impl Send for XcbInputState {}
unsafe impl Sync for XcbInputState {}

impl XcbInputState {
    fn new() -> Arc<Self> {
        let connection = XcbConnection::new();
        let state = XcbInputState {
            connection,
            context: RwLock::new(None),
        };
        Arc::new(state)
    }

    fn initialize(&self, context: &Arc<XcbInputContext>) {
        let context = Arc::clone(context);
        let mut guard = self.context.write().unwrap();
        guard.replace(context);
    }

    fn create_window(&self, configuration: WindowConfiguration) -> Result<Arc<XcbWindowObject>> {
        let context = self.context();
        let object = XcbWindowObject::new(context, &self.connection);
        object
    }

    fn destroy_window(&self, window: Arc<XcbWindowInner>) {
        drop(window);
    }

    fn context(&self) -> Arc<XcbInputContext> {
        let guard = self.context.read().unwrap();
        guard.as_ref()
            .map(|v| Arc::clone(v))
            .unwrap()
    }
}

struct XcbWindowInner {
    window: Arc<XcbWindow>,
    surface: Arc<Surface>,
    thread_id: std::thread::ThreadId,
}

unsafe impl Send for XcbWindowInner {}
unsafe impl Sync for XcbWindowInner {}

impl XcbWindowInner {
    pub fn new(window: Arc<XcbWindow>, surface: Arc<Surface>) -> Arc<Self> {
        let inner = XcbWindowInner {
            window,
            surface,
            thread_id: std::thread::current().id(),
        };
        Arc::new(inner)
    }

    pub fn surface(&self) -> &Arc<Surface> {
        &self.surface
    }
}

impl Drop for XcbWindowInner {
    fn drop(&mut self) {
        assert_eq!(std::thread::current().id(), self.thread_id);
    }
}

pub struct XcbWindowObject {
    context: Arc<XcbInputContext>,
    inner: Arc<XcbWindowInner>,
}

unsafe impl Send for XcbWindowObject {}
unsafe impl Sync for XcbWindowObject {}

impl XcbWindowObject {
    pub fn new(context: Arc<XcbInputContext>, connection: &Arc<XcbConnection>) -> Result<Arc<Self>> {
        let enviroment = context.base().environment();
        let instance = enviroment.main().instance();
        let window = XcbWindow::new(connection);
        let surface = XcbSurface::new(instance, &window)?;
        let object = XcbWindowObject {
            context,
            inner: XcbWindowInner::new(window, surface),
        };
        let object = Arc::new(object);
        Ok(object)
    }

    pub fn surface(&self) -> &Arc<Surface> {
        self.inner.surface()
    }
}

impl Drop for XcbWindowObject {
    fn drop(&mut self) {
        let executor = self.context.executor();
        let context = Arc::clone(&self.context);
        let inner = Arc::clone(&self.inner);
        executor.execute(async move {
            context.destroy_window(inner).await;
        });
    }
}
