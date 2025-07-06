use crate::{Agent, Context, Handler, protocol::*, tool};
use std::sync::Arc;

#[derive(Clone)]
pub struct Session {
    agent: Option<Arc<Box<dyn Agent>>>,
    handler: Option<Arc<Box<dyn Handler>>>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            agent: None,
            handler: None,
        }
    }

    pub fn agent(&mut self, agent: Box<dyn Agent>) -> &mut Self {
        self.agent = Some(Arc::new(agent));
        self
    }

    pub fn handler(&mut self, handler: Box<dyn Handler>) -> &mut Self {
        self.handler = Some(Arc::new(handler));
        self
    }

    pub async fn start(&mut self) {
        let handler = match self.handler.clone() {
            Some(handler) => handler,
            None => return,
        };

        let agent = match self.agent.clone() {
            Some(agent) => agent,
            None => return,
        };

        let mut context = Context::new();

        loop {
            match handler.prompt().await {
                Some(prompt) => {
                    context.push(Message {
                        role: "user".into(),
                        content: prompt.into(),
                    });
                }
                None => break,
            }
            // TODO: Loop until done?
            match agent.chat(&context).await {
                Ok(response) => {
                    context.push(Message {
                        role: "assistant".into(),
                        content: response.message.content.clone(),
                    });

                    handler.response(&response.message.content).await;

                    for tool_call in &response.message.tool_calls {
                        if !handler.allow_tool(&tool_call).await {
                            break;
                        }

                        match tool::call(&tool_call).await {
                            Ok(output) => {
                                context.push(Message {
                                    role: "assistant".into(),
                                    content: format!("Tool result: {}", output),
                                });
                            }
                            Err(err) => {
                                println!("Error while executing tool: {}", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Error while chatting: {}", err);
                }
            }
        }
    }
}
