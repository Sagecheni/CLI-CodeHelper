use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct StreamResponse {
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
pub struct StreamChoice {
    pub delta: DeltaContent,
}

#[derive(Debug, Deserialize)]
pub struct DeltaContent {
    pub content: Option<String>,
}
