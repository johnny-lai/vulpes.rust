use crate::protocol::*;
use anyhow::Result;
use serde_json::Value;
mod shell;

pub use shell::*;

pub async fn call(tool_call: &ToolCall) -> Result<String> {
    let _tool_name = &tool_call.function.name;
    Ok("".into())
}

#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn definition(&self) -> Value;
    async fn call(&self, arguments: Value) -> Result<String>;
}
