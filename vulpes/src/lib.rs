use anyhow::Result;

pub mod agent;
pub mod context;
pub mod protocol;
pub mod session;
pub mod tool;

pub use context::Context;
pub use protocol::Response;
pub use protocol::ToolCall;
pub use session::Session;

#[async_trait::async_trait]
pub trait Agent {
    async fn chat(&self, context: &Context) -> Result<Response>;
}

#[async_trait::async_trait]
pub trait Handler {
    /// Return None to terminate
    async fn prompt(&self) -> Option<String>;

    /// Reponse from assistant
    async fn response(&self, content: &str);

    /// Whether tool invocation should be allowed
    async fn allow_tool(&self, tool_call: &ToolCall) -> bool;
}
