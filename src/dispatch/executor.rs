

use std::sync::{Arc};
use std::future::Future;

use futures::{
    executor::{
        ThreadPool,
    },
};

pub struct DispatchExecutor {
    thread_pool: Arc<ThreadPool>,
}

impl DispatchExecutor {
    pub fn new() -> Self {
        let thread_pool = ThreadPool::new().unwrap();
        let executor = DispatchExecutor {
            thread_pool: Arc::new(thread_pool)
        };
        executor
    }

    pub fn execute<F: Future>(&self, f: F) 
    where
        F: Future<Output = ()> + Send + 'static 
    {
        self.thread_pool.spawn_ok(f);
    }
}
