use crate::api::models::*;
use anyhow::{Context, Result};
use reqwest::Client;
use std::io::Write;
pub struct OpenAIClient {
    client: Client,
    api_key: String,
    messages: Vec<Message>,
}

impl OpenAIClient {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable not set")?;

        Ok(Self {
            client: Client::new(),
            api_key,
            messages: Vec::new(),
        })
    }

    pub fn clear_context(&mut self) {
        self.messages.clear();
    }

    pub async fn chat(&mut self, prompt: &str) -> Result<String> {
        self.messages.push(Message::user(prompt));
        let request = ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: self.messages.clone(),
            stream: Some(false),
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .json::<ChatResponse>()
            .await?;
        
        let assitant_message = &response.choices[0].message.content;
        self.messages.push(Message::assistant(assitant_message));
        Ok(assitant_message.clone())
    }

    pub async fn chat_stream(&mut self, prompt: &str) -> Result<()> {
        self.messages.push(Message::user(prompt));
        let request = ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: self.messages.clone(),
            stream: Some(true),
        };
        std::io::stdout().flush()?;

        let mut response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let mut assistant_response = String::new();

        while let Some(chunk) = response.chunk().await? {
            let chunk_str = String::from_utf8_lossy(&chunk);

            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];

                    if data == "[DONE]" {
                        println!();
                        self.messages.push(Message::assistant(&assistant_response));
                        return Ok(());
                    }

                    if let Ok(response) = serde_json::from_str::<StreamResponse>(data) {
                        if let Some(content) = &response.choices[0].delta.content {
                            print!("{}", content);
                            std::io::stdout().flush()?;
                            assistant_response.push_str(content);
                        }
                    }
                }
            }
        }

        Ok(())
    }
    // 获取当前对话的消息数量
    pub fn context_length(&self) -> usize {
        self.messages.len() / 2 // 每轮对话包含用户和助手各一条消息
    }
    // 显示对话历史
    pub fn show_context(&self) -> Vec<&Message> {
        self.messages.iter().collect()
    }
}
