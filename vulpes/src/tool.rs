use crate::protocol::*;
use anyhow::Result;

pub async fn call(tool_call: &ToolCall) -> Result<String> {
    let _tool_name = &tool_call.function.name;
    Ok("".into())
}
