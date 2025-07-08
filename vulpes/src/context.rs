use std::collections::HashMap;

use crate::{protocol::*, tool};
use anyhow::{Result, anyhow};
use serde_json::Value;

pub struct Context {
    messages: Vec<Message>,
    tools: HashMap<String, Box<dyn tool::Tool>>,
}

impl Context {
    pub fn new() -> Self {
        let mut ret = Self {
            messages: Vec::new(),
            tools: HashMap::new(),
        };
        ret.register_tool(Box::new(tool::ReadFile::new()));
        ret.register_tool(Box::new(tool::ShellExecute::new()));
        ret
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn tools(&self) -> Vec<Value> {
        let mut ret = Vec::new();
        for (_name, tool) in &self.tools {
            ret.push(tool.definition());
        }
        ret
    }

    pub fn register_tool(&mut self, tool: Box<dyn tool::Tool>) {
        self.tools.insert(tool.name(), tool);
    }

    pub async fn call_tool(&self, tool_call: ToolCall) -> Result<String> {
        let name = &tool_call.function.name;
        let arguments = tool_call.function.arguments;
        match self.tools.get(name) {
            Some(tool) => tool.call(arguments).await,
            None => Err(anyhow!("unknown tool")),
        }
    }
}
