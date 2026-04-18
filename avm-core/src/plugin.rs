use crate::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub capabilities: Vec<String>,
}

#[async_trait]
pub trait Plugin: Send + Sync {
    async fn initialize(&mut self) -> Result<()>;

    async fn shutdown(&mut self) -> Result<()>;

    fn metadata(&self) -> &PluginMetadata;
}
