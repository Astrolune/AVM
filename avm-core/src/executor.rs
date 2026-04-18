use crate::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub task_id: String,
    pub timeout: Duration,
    pub memory_limit: u64,
    pub env: HashMap<String, String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: serde_json::Value,
    pub duration: Duration,
    pub memory_used: u64,
    pub error: Option<String>,
}

#[async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, ctx: ExecutionContext, input: serde_json::Value) -> Result<ExecutionResult>;

    async fn cancel(&self, task_id: &str) -> Result<()>;

    fn name(&self) -> &str;
}
