use crate::{protocol::*, tool};
use serde_json::Value;

pub struct Context {
    messages: Vec<Message>,
    tools: Vec<Box<dyn tool::Tool>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            tools: vec![
                Box::new(tool::ReadFile::new()),
                Box::new(tool::ShellExecute::new()),
            ],
        }
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn tools(&self) -> Vec<Value> {
        let mut ret = Vec::new();
        for tool in &self.tools {
            ret.push(tool.definition());
        }
        ret
    }
}
