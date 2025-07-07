use crate::protocol::*;
use crate::{Agent, Context};
use anyhow::Result;
use reqwest;

pub struct Ollama {
    server_url: String,
    model: String,
}

impl Ollama {
    pub fn new(server_url: &str, model: &str) -> Self {
        Self {
            server_url: server_url.into(),
            model: model.into(),
        }
    }
}

#[async_trait::async_trait]
impl Agent for Ollama {
    async fn chat(&self, context: &Context) -> Result<Response> {
        let client = reqwest::Client::new();

        let request = Request {
            model: self.model.clone(),
            messages: context.messages().to_vec(),
            tools: context.tools(),
            stream: false,
        };

        let response = client
            .post(&format!("{}/api/chat", self.server_url))
            .json(&request)
            .send()
            .await
            .unwrap();

        let response_text = response.text().await.unwrap();
        let chat_response: Response = serde_json::from_str(&response_text).unwrap();
        Ok(chat_response)
    }
}
