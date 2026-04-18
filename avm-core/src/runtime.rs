use crate::{Config, Executor, Result};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Runtime: Send + Sync {
    async fn start(&mut self, config: Config) -> Result<()>;

    async fn shutdown(&mut self) -> Result<()>;

    async fn register_executor(&mut self, executor: Arc<dyn Executor>) -> Result<()>;

    fn handle(&self) -> RuntimeHandle;
}

#[derive(Clone)]
pub struct RuntimeHandle {
    inner: Arc<RuntimeHandleInner>,
}

struct RuntimeHandleInner {
    _private: (),
}

impl RuntimeHandle {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RuntimeHandleInner { _private: () }),
        }
    }
}
