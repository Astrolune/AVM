use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryEvent {
    RuntimeStarted { timestamp: SystemTime },
    RuntimeStopped { timestamp: SystemTime },
    ExecutionStarted { task_id: String, timestamp: SystemTime },
    ExecutionCompleted { task_id: String, duration_ms: u64, success: bool },
    ResourceLimitReached { resource: String, limit: u64 },
    Error { message: String, timestamp: SystemTime },
}

pub trait TelemetryHook: Send + Sync {
    fn on_event(&self, event: TelemetryEvent);
}
