use crate::protocol::*;
use anyhow::Result;
use serde_json::Value;
mod shell;

pub use shell::*;

#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> String;
    fn definition(&self) -> Value;
    async fn call(&self, arguments: Value) -> Result<String>;
}
