use crate::Context;
use crate::protocol::*;

pub trait Agent {
    fn send(&self, context: &Context) -> Response;
}
