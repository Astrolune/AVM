# Quick Start Guide

## Installation

### As a Library
Add to your `Cargo.toml`:
```toml
[dependencies]
avm-core = { git = "https://github.com/astrolune/avm", branch = "main" }
avm-runtime = { git = "https://github.com/astrolune/avm", branch = "main" }
```

### As a Submodule (Recommended for ACP)
```bash
git submodule add https://github.com/astrolune/avm.git
git submodule update --init --recursive
```

Then in `Cargo.toml`:
```toml
[dependencies]
avm-core = { path = "avm/avm-core" }
avm-runtime = { path = "avm/avm-runtime" }
avm-sandbox = { path = "avm/avm-sandbox" }
```

## Basic Usage

### 1. Create a Runtime

```rust
use avm_core::{Config, Runtime};
use avm_runtime::AvmRuntime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = AvmRuntime::new();
    let config = Config::default();
    
    runtime.start(config).await?;
    
    // Use runtime...
    
    runtime.shutdown().await?;
    Ok(())
}
```

### 2. Implement an Executor

```rust
use avm_core::{Executor, ExecutionContext, ExecutionResult};
use async_trait::async_trait;

struct MyExecutor;

#[async_trait]
impl Executor for MyExecutor {
    async fn execute(
        &self,
        ctx: ExecutionContext,
        input: serde_json::Value,
    ) -> avm_core::Result<ExecutionResult> {
        // Your execution logic here
        Ok(ExecutionResult {
            success: true,
            output: serde_json::json!({"result": "done"}),
            duration: ctx.timeout,
            memory_used: 1024,
            error: None,
        })
    }

    async fn cancel(&self, task_id: &str) -> avm_core::Result<()> {
        // Cancellation logic
        Ok(())
    }

    fn name(&self) -> &str {
        "my_executor"
    }
}
```

### 3. Register and Use Executor

```rust
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;

// Register executor
runtime.register_executor(Arc::new(MyExecutor)).await?;

// Create execution context
let ctx = ExecutionContext {
    task_id: "task-1".to_string(),
    timeout: Duration::from_secs(30),
    memory_limit: 512 * 1024 * 1024,
    env: HashMap::new(),
    metadata: HashMap::new(),
};

// Execute task
let executor = Arc::new(MyExecutor);
let result = executor.execute(ctx, serde_json::json!({"data": "test"})).await?;
println!("Result: {:?}", result);
```

### 4. Configure Sandbox

```rust
use avm_core::{Config, SandboxConfig, RuntimeConfig, TelemetryConfig};
use std::time::Duration;

let config = Config {
    sandbox: SandboxConfig {
        max_memory_bytes: 1024 * 1024 * 1024, // 1GB
        max_cpu_time: Duration::from_secs(60),
        max_wall_time: Duration::from_secs(120),
        allow_network: false,
        allow_filesystem: true,
        filesystem_root: Some("/tmp/avm".to_string()),
    },
    runtime: RuntimeConfig {
        worker_threads: 8,
        max_blocking_threads: 512,
        shutdown_timeout: Duration::from_secs(30),
    },
    telemetry: TelemetryConfig {
        enabled: true,
        log_level: "info".to_string(),
    },
};

runtime.start(config).await?;
```

### 5. Use Sandbox Controls

```rust
use avm_sandbox::Sandbox;

let sandbox = Sandbox::new(config.sandbox);

// Check filesystem access
sandbox.check_filesystem_access("/tmp/avm/data.txt")?;

// Check network access
sandbox.check_network_access()?;

// Allocate memory
let guard = sandbox.allocate_memory(1024 * 1024).await?;
// Memory is released when guard is dropped
```

## Running Examples

```bash
cd avm
cargo run --example basic_usage
```

## Testing

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p avm-sandbox

# Run with output
cargo test --workspace -- --nocapture
```

## Building

```bash
# Debug build
cargo build --workspace

# Release build
cargo build --workspace --release

# Check without building
cargo check --workspace
```

## Common Patterns

### Error Handling

```rust
use avm_core::{Error, Result};

fn my_function() -> Result<()> {
    // Recoverable errors
    if timeout {
        return Err(Error::Timeout("Operation timed out".to_string()));
    }
    
    // Sandbox violations
    if unauthorized {
        return Err(Error::SandboxViolation("Access denied".to_string()));
    }
    
    Ok(())
}
```

### Telemetry

```rust
use avm_core::{TelemetryEvent, TelemetryHook};
use std::time::SystemTime;

struct MyTelemetry;

impl TelemetryHook for MyTelemetry {
    fn on_event(&self, event: TelemetryEvent) {
        match event {
            TelemetryEvent::ExecutionCompleted { task_id, duration_ms, success } => {
                println!("Task {} completed in {}ms: {}", task_id, duration_ms, success);
            }
            _ => {}
        }
    }
}
```

## Troubleshooting

### Memory Limit Errors
- Increase `max_memory_bytes` in config
- Check for memory leaks in executor
- Monitor actual memory usage

### Timeout Errors
- Increase `max_wall_time` in config
- Optimize executor logic
- Check for blocking operations

### Sandbox Violations
- Verify filesystem paths are within `filesystem_root`
- Enable network access if needed
- Check permissions

## Next Steps

- Read [ARCHITECTURE.md](ARCHITECTURE.md) for design details
- Review [SECURITY.md](SECURITY.md) for security best practices
- Check [examples/](avm-examples/examples/) for more examples
- See [CONTRIBUTING.md](CONTRIBUTING.md) to contribute
