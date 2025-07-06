use crate::protocol::*;
use crate::{Agent, Context, Handler};

pub struct Session {
    agent: Option<Box<dyn Agent>>,
    handler: Option<Box<dyn Handler>>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            agent: None,
            handler: None,
        }
    }

    pub fn agent(&mut self, agent: Box<dyn Agent>) -> &mut Self {
        self.agent = Some(agent);
        self
    }

    pub fn handler(&mut self, handler: Box<dyn Handler>) -> &mut Self {
        self.handler = Some(handler);
        self
    }

    pub fn start(&mut self) {
        let handler = match self.handler.as_mut() {
            Some(handler) => handler,
            None => return,
        };

        let mut context = Context::new();

        loop {
            match handler.prompt() {
                Some(prompt) => {
                    context.push(Message {
                        role: "user".into(),
                        content: prompt.into(),
                    });
                }
                None => break,
            }
        }
    }
}
