use super::config::{ApiConfig, ChatModel};
use crate::api::models::*;
use anyhow::{Context, Result};
use reqwest::Client;
use std::io::Write;
pub struct OpenAIClient {
    client: Client,
    config: ApiConfig,
    messages: Vec<Message>,
}

impl OpenAIClient {
    pub fn new() -> Result<Self> {
        let config = ApiConfig::load()?;
        let _api_key = config.api_key.clone().context("API key not found. Please set OPENAI_API_KEY environment variable or configure it in the config file.")?;

        Ok(Self {
            client: Client::new(),
            config,
            messages: Vec::new(),
        })
    }

    pub fn get_config(&self) -> &ApiConfig {
        &self.config
    }

    pub fn set_model(&mut self, model: ChatModel) -> Result<()> {
        self.config.model = model;
        self.config.save()?;
        Ok(())
    }

    pub fn clear_context(&mut self) {
        self.messages.clear();
    }

    pub async fn chat(&mut self, prompt: &str) -> Result<String> {
        self.messages.push(Message::user(prompt));
        let request = ChatRequest {
            model: self.config.model.as_str().to_string(),
            messages: self.messages.clone(),
            stream: Some(false),
        };
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header(
                "Authorization",
                format!("Bearer {}", self.config.api_key.clone().unwrap()),
            )
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
            model: self.config.model.as_str().to_string(),
            messages: self.messages.clone(),
            stream: Some(true),
        };
        std::io::stdout().flush()?;

        let mut response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header(
                "Authorization",
                format!("Bearer {}", self.config.api_key.clone().unwrap()),
            )
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

    pub async fn generate_shell_command(&mut self, prompt: &str) -> Result<String> {
        // 保存当前消息历史
        let original_messages = self.messages.clone();
        let system_prompt = "You are a command line expert. Generate shell commands based on user descriptions. \
                           Return ONLY the command itself, no markdown formatting, no backticks, no explanation. \
                           The command should be a single line. \
                           Ensure the command is safe and won't cause damage to the system.";
        // 设置命令生成的系统提示
        self.messages.clear();
        self.messages.push(Message::system(system_prompt));
        self.messages.push(Message::user(prompt));

        // 获取响应
        let response = self.chat(prompt).await?;

        // 恢复原始消息历史
        self.messages = original_messages;

        Ok(response
            .trim()
            .replace("```shell", "")
            .replace("```", "")
            .replace("`", "")
            .trim()
            .to_string())
    }
}
