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

        // System prompt
        let prompt = r#""#;
        context.push(Message {
            role: "user".into(),
            content: prompt.into(),
        });

        let mut prompt_user = true;
        loop {
            // Check if we should prompt the user
            if prompt_user {
                match handler.prompt().await {
                    Some(prompt) => {
                        context.push(Message {
                            role: "user".into(),
                            content: prompt.into(),
                        });
                    }
                    None => break,
                }
            }

            // Assume we want to prompt the user for next steps
            prompt_user = true;
            match agent.chat(&context).await {
                Ok(response) => {
                    context.push(Message {
                        role: "assistant".into(),
                        content: response.message.content.clone(),
                    });

                    handler.response(&response.message.content).await;

                    for tool_call in response.message.tool_calls {
                        if !handler.allow_tool(&tool_call).await {
                            break;
                        }

                        match context.call_tool(tool_call).await {
                            Ok(output) => {
                                context.push(Message {
                                    role: "assistant".into(),
                                    content: format!("Tool result: {}", output),
                                });
                            }
                            Err(err) => {
                                context.push(Message {
                                    role: "assistant".into(),
                                    content: format!(
                                        "Tool result: Error while executing tool: {}",
                                        err
                                    ),
                                });
                                println!("Error while executing tool: {}", err);
                            }
                        }

                        // If there is tool usage, we want to check with the
                        // agent for next steps.
                        prompt_user = false;
                    }
                }
                Err(err) => {
                    println!("Error while chatting: {}", err);
                }
            }
        }
    }
}
