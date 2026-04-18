use crate::RuntimeState;
use avm_core::{Config, Error, Executor, Result, Runtime, RuntimeHandle};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct AvmRuntime {
    state: Arc<RwLock<RuntimeState>>,
}

impl AvmRuntime {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(RuntimeState::new(Config::default()))),
        }
    }
}

#[async_trait]
impl Runtime for AvmRuntime {
    async fn start(&mut self, config: Config) -> Result<()> {
        info!("Starting AVM runtime");
        let mut state = self.state.write().await;
        state.config = config;
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down AVM runtime");
        let state = self.state.read().await;
        let timeout = state.config.runtime.shutdown_timeout;

        tokio::time::timeout(timeout, async {
            for executor in &state.executors {
                info!("Shutting down executor: {}", executor.name());
            }
        })
        .await
        .map_err(|_| Error::Timeout("Runtime shutdown timeout".to_string()))?;

        Ok(())
    }

    async fn register_executor(&mut self, executor: Arc<dyn Executor>) -> Result<()> {
        let mut state = self.state.write().await;
        info!("Registering executor: {}", executor.name());
        state.executors.push(executor);
        Ok(())
    }

    fn handle(&self) -> RuntimeHandle {
        RuntimeHandle::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use avm_core::{Config, Executor, ExecutionContext, ExecutionResult};
    use std::sync::Arc;
    use std::time::Duration;

    struct MockExecutor;

    #[async_trait::async_trait]
    impl Executor for MockExecutor {
        async fn execute(
            &self,
            _ctx: ExecutionContext,
            _input: serde_json::Value,
        ) -> avm_core::Result<ExecutionResult> {
            Ok(ExecutionResult {
                success: true,
                output: serde_json::json!({"result": "ok"}),
                duration: Duration::from_millis(10),
                memory_used: 1024,
                error: None,
            })
        }

        async fn cancel(&self, _task_id: &str) -> avm_core::Result<()> {
            Ok(())
        }

        fn name(&self) -> &str {
            "mock"
        }
    }

    #[tokio::test]
    async fn test_runtime_lifecycle() {
        let mut runtime = AvmRuntime::new();
        let config = Config::default();

        runtime.start(config).await.unwrap();
        runtime.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_register_executor() {
        let mut runtime = AvmRuntime::new();
        let config = Config::default();

        runtime.start(config).await.unwrap();
        runtime
            .register_executor(Arc::new(MockExecutor))
            .await
            .unwrap();
        runtime.shutdown().await.unwrap();
    }
}
