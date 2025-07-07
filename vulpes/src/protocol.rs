use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub tools: Vec<Value>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub message: ResponseMessage,
    #[serde(default)]
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub role: String,
    pub content: String,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolCall {
    pub function: FunctionCall,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tool {
    Function {
        name: String,
        title: Option<String>,
        description: String,
        #[serde(rename = "inputSchema")]
        input_schema: Value,
    },
}
