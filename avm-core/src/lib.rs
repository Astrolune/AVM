//! Core traits and types for AVM

pub mod config;
pub mod error;
pub mod executor;
pub mod plugin;
pub mod runtime;
pub mod telemetry;

pub use config::{Config, RuntimeConfig, SandboxConfig, TelemetryConfig};
pub use error::{Error, Result};
pub use executor::{ExecutionContext, ExecutionResult, Executor};
pub use plugin::{Plugin, PluginMetadata};
pub use runtime::{Runtime, RuntimeHandle};
pub use telemetry::{TelemetryEvent, TelemetryHook};
