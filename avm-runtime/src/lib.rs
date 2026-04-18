//! Runtime implementation for AVM

mod runtime_impl;

pub use runtime_impl::AvmRuntime;

use avm_core::{Config, Executor, RuntimeHandle};
use std::sync::Arc;

pub struct RuntimeState {
    pub config: Config,
    pub executors: Vec<Arc<dyn Executor>>,
    pub handle: RuntimeHandle,
}

impl RuntimeState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            executors: Vec::new(),
            handle: RuntimeHandle::new(),
        }
    }
}
