//! IPC integration for AVM

use serde::{Deserialize, Serialize};

#[cfg(unix)]
use avm_core::{Error, Result};
#[cfg(unix)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub id: String,
    pub payload: serde_json::Value,
}

#[allow(dead_code)]
pub struct IpcServer {
    socket_path: String,
}

impl IpcServer {
    pub fn new(socket_path: String) -> Self {
        Self { socket_path }
    }

    #[cfg(unix)]
    pub async fn start<F>(&self, handler: F) -> Result<()>
    where
        F: Fn(IpcMessage) -> Result<IpcMessage> + Send + Sync + 'static,
    {
        let listener = UnixListener::bind(&self.socket_path)
            .map_err(|e| Error::Ipc(format!("Failed to bind socket: {}", e)))?;

        loop {
            let (stream, _) = listener
                .accept()
                .await
                .map_err(|e| Error::Ipc(format!("Accept failed: {}", e)))?;

            let handler = &handler;
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, handler).await {
                    tracing::error!("Connection error: {}", e);
                }
            });
        }
    }

    #[cfg(unix)]
    async fn handle_connection<F>(mut stream: UnixStream, handler: &F) -> Result<()>
    where
        F: Fn(IpcMessage) -> Result<IpcMessage>,
    {
        let mut buffer = vec![0u8; 4096];
        let n = stream
            .read(&mut buffer)
            .await
            .map_err(|e| Error::Ipc(format!("Read failed: {}", e)))?;

        let msg: IpcMessage = serde_json::from_slice(&buffer[..n])?;
        let response = handler(msg)?;

        let response_bytes = serde_json::to_vec(&response)?;
        stream
            .write_all(&response_bytes)
            .await
            .map_err(|e| Error::Ipc(format!("Write failed: {}", e)))?;

        Ok(())
    }
}

#[allow(dead_code)]
pub struct IpcClient {
    socket_path: String,
}

impl IpcClient {
    pub fn new(socket_path: String) -> Self {
        Self { socket_path }
    }

    #[cfg(unix)]
    pub async fn send(&self, msg: IpcMessage) -> Result<IpcMessage> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| Error::Ipc(format!("Connection failed: {}", e)))?;

        let msg_bytes = serde_json::to_vec(&msg)?;
        stream
            .write_all(&msg_bytes)
            .await
            .map_err(|e| Error::Ipc(format!("Write failed: {}", e)))?;

        let mut buffer = vec![0u8; 4096];
        let n = stream
            .read(&mut buffer)
            .await
            .map_err(|e| Error::Ipc(format!("Read failed: {}", e)))?;

        let response: IpcMessage = serde_json::from_slice(&buffer[..n])?;
        Ok(response)
    }
}
