
use std::sync::{Arc, Weak};
use std::sync::RwLock;

pub struct Base {
    environment: RwLock<Weak<Environment>>,
}

unsafe impl Sync for Base {}
unsafe impl Send for Base {}

impl Base {
    pub fn environment(&self) -> Arc<Environment> {
        let guard = self.environment.read().unwrap();
        guard.upgrade().unwrap()
    }

    pub fn initialize(&self, env: &Arc<Environment>) {
        let mut write = self.environment.write().unwrap();
        *write = Arc::downgrade(&env);
    }
}

impl Default for Base {
    fn default() -> Self {
        Base {
            environment: RwLock::new(Weak::new()),
        }
    }
}

mod main;
mod input;
mod render;

pub use main::*;
pub use input::*;
pub use render::*;

pub struct Environment {
    base: Arc<Base>,
    main: Arc<MainSystem>,
    input: Arc<InputSystem>,
    render: Arc<RenderSystem>,
}

impl Environment {
    pub fn new() -> Arc<Environment> {
        let base = Arc::new(Base::default());
        let main = MainSystem::new(&base).unwrap();
        let input = InputSystem::new(&base);
        let render = RenderSystem::new(&base);
        let env = Environment {
            base,
            main,
            input,
            render,
        };
        let env = Arc::new(env);
        env.base.initialize(&env);
        env
    }

    #[inline]
    pub fn base(&self) -> &Arc<Base> {
        &self.base
    }

    #[inline]
    pub fn main(&self) -> &Arc<MainSystem> {
        &self.main
    }

    #[inline]
    pub fn input(&self) -> &Arc<InputSystem> {
        &self.input
    }

    #[inline]
    pub fn render(&self) -> &Arc<RenderSystem> {
        &self.render
    }
}
