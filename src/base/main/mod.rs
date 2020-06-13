
use super::Base;
use crate::dispatch::Dispatch;

use std::sync::{Arc, RwLock};

use crate::vk::{
    Instance,
    Result,
};

enum Request {}
enum Response {}

pub struct MainSystem {
    dispatch: Dispatch<Request, Response>,
    state: Arc<MainSystemState>,
}

impl MainSystem {
    pub fn new(base: &Arc<Base>) -> Result<Arc<MainSystem>> {
        let base = Arc::clone(base);
        let state = MainSystemState::new()?;
        let dispatch = Dispatch::new(move |v| {
            match v {
                
            }
        });
        let main_system = MainSystem {
            dispatch,
            state,
        };
        let main_system = Arc::new(main_system);
        Ok(main_system)
    }

    #[inline]
    pub fn instance(&self) -> &Arc<Instance> {
        self.state.instance()
    }
}

struct MainSystemState {
    instance: Arc<Instance>,
}

impl MainSystemState {
    fn new() -> Result<Arc<Self>> {
        let state = MainSystemState {
            instance: Instance::new()?,
        };
        let state = Arc::new(state);
        Ok(state)
    }

    fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }
}
