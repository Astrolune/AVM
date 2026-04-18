use avm_core::{Config, Executor, ExecutionContext, ExecutionResult, Runtime};
use avm_runtime::AvmRuntime;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

struct SimpleExecutor;

#[async_trait::async_trait]
impl Executor for SimpleExecutor {
    async fn execute(
        &self,
        ctx: ExecutionContext,
        input: serde_json::Value,
    ) -> avm_core::Result<ExecutionResult> {
        println!("Executing task: {}", ctx.task_id);
        println!("Input: {}", input);

        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(ExecutionResult {
            success: true,
            output: serde_json::json!({"result": "Task completed"}),
            duration: Duration::from_millis(100),
            memory_used: 1024,
            error: None,
        })
    }

    async fn cancel(&self, task_id: &str) -> avm_core::Result<()> {
        println!("Cancelling task: {}", task_id);
        Ok(())
    }

    fn name(&self) -> &str {
        "simple"
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut runtime = AvmRuntime::new();
    let config = Config::default();

    runtime.start(config).await?;
    println!("Runtime started");

    let executor = Arc::new(SimpleExecutor);
    runtime.register_executor(executor.clone()).await?;
    println!("Executor registered");

    let ctx = ExecutionContext {
        task_id: "task-1".to_string(),
        timeout: Duration::from_secs(5),
        memory_limit: 1024 * 1024,
        env: HashMap::new(),
        metadata: HashMap::new(),
    };

    let result = executor
        .execute(ctx, serde_json::json!({"data": "test"}))
        .await?;
    println!("Execution result: {:?}", result);

    runtime.shutdown().await?;
    println!("Runtime shutdown");

    Ok(())
}
