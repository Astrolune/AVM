use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub sandbox: SandboxConfig,
    pub runtime: RuntimeConfig,
    pub telemetry: TelemetryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub max_memory_bytes: u64,
    pub max_cpu_time: Duration,
    pub max_wall_time: Duration,
    pub allow_network: bool,
    pub allow_filesystem: bool,
    pub filesystem_root: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub worker_threads: usize,
    pub max_blocking_threads: usize,
    pub shutdown_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sandbox: SandboxConfig {
                max_memory_bytes: 512 * 1024 * 1024,
                max_cpu_time: Duration::from_secs(30),
                max_wall_time: Duration::from_secs(60),
                allow_network: false,
                allow_filesystem: false,
                filesystem_root: None,
            },
            runtime: RuntimeConfig {
                worker_threads: 4,
                max_blocking_threads: 512,
                shutdown_timeout: Duration::from_secs(10),
            },
            telemetry: TelemetryConfig {
                enabled: true,
                log_level: "info".to_string(),
            },
        }
    }
}
