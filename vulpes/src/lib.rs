pub mod agent;
pub mod context;
pub mod protocol;
pub mod session;

pub use context::Context;
pub use session::Session;

#[async_trait::async_trait]
pub trait Agent {
    async fn chat(&self, context: &Context) -> protocol::Response;
}

#[async_trait::async_trait]
pub trait Handler {
    /// Return None to terminate
    async fn prompt(&self) -> Option<String>;
}
