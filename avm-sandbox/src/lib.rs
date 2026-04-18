//! Sandbox implementation for AVM

use avm_core::{Error, Result, SandboxConfig};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct Sandbox {
    config: SandboxConfig,
    memory_limiter: Arc<Semaphore>,
}

impl Sandbox {
    pub fn new(config: SandboxConfig) -> Self {
        let permits = (config.max_memory_bytes / 1024) as usize;
        Self {
            config,
            memory_limiter: Arc::new(Semaphore::new(permits)),
        }
    }

    pub async fn allocate_memory(&self, bytes: u64) -> Result<MemoryGuard> {
        let permits = (bytes / 1024) as usize;
        let permit = self
            .memory_limiter
            .clone()
            .acquire_many_owned(permits as u32)
            .await
            .map_err(|_| Error::ResourceLimit("Memory allocation failed".to_string()))?;

        Ok(MemoryGuard { _permit: permit })
    }

    pub fn check_filesystem_access(&self, path: &str) -> Result<()> {
        if !self.config.allow_filesystem {
            return Err(Error::SandboxViolation(
                "Filesystem access not allowed".to_string(),
            ));
        }

        if let Some(root) = &self.config.filesystem_root {
            if !path.starts_with(root) {
                return Err(Error::SandboxViolation(format!(
                    "Path {} outside allowed root {}",
                    path, root
                )));
            }
        }

        Ok(())
    }

    pub fn check_network_access(&self) -> Result<()> {
        if !self.config.allow_network {
            return Err(Error::SandboxViolation(
                "Network access not allowed".to_string(),
            ));
        }
        Ok(())
    }
}

pub struct MemoryGuard {
    _permit: tokio::sync::OwnedSemaphorePermit,
}

#[cfg(test)]
mod tests {
    use super::*;
    use avm_core::SandboxConfig;
    use std::time::Duration;

    #[tokio::test]
    async fn test_memory_allocation() {
        let config = SandboxConfig {
            max_memory_bytes: 1024 * 1024,
            max_cpu_time: Duration::from_secs(10),
            max_wall_time: Duration::from_secs(20),
            allow_network: false,
            allow_filesystem: false,
            filesystem_root: None,
        };

        let sandbox = Sandbox::new(config);
        let _guard = sandbox.allocate_memory(1024).await.unwrap();
    }

    #[tokio::test]
    async fn test_filesystem_access_denied() {
        let config = SandboxConfig {
            max_memory_bytes: 1024 * 1024,
            max_cpu_time: Duration::from_secs(10),
            max_wall_time: Duration::from_secs(20),
            allow_network: false,
            allow_filesystem: false,
            filesystem_root: None,
        };

        let sandbox = Sandbox::new(config);
        let result = sandbox.check_filesystem_access("/tmp/test");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_network_access_denied() {
        let config = SandboxConfig {
            max_memory_bytes: 1024 * 1024,
            max_cpu_time: Duration::from_secs(10),
            max_wall_time: Duration::from_secs(20),
            allow_network: false,
            allow_filesystem: false,
            filesystem_root: None,
        };

        let sandbox = Sandbox::new(config);
        let result = sandbox.check_network_access();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_filesystem_root_restriction() {
        let config = SandboxConfig {
            max_memory_bytes: 1024 * 1024,
            max_cpu_time: Duration::from_secs(10),
            max_wall_time: Duration::from_secs(20),
            allow_network: false,
            allow_filesystem: true,
            filesystem_root: Some("/allowed".to_string()),
        };

        let sandbox = Sandbox::new(config);
        assert!(sandbox.check_filesystem_access("/allowed/file").is_ok());
        assert!(sandbox.check_filesystem_access("/forbidden/file").is_err());
    }
}
