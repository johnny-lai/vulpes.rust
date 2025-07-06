pub mod agent;
pub mod context;
pub mod protocol;
pub mod session;

pub use agent::Agent;
pub use context::Context;
pub use session::Session;

pub trait Handler {
    /// Return None to terminate
    fn prompt(&self) -> Option<String>;
}
