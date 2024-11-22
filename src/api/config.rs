use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub api_key: Option<String>,
    pub model: ChatModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChatModel {
    Gpt35Turbo,
    Gpt4o,
    Gpt4omini,
}

impl ChatModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChatModel::Gpt35Turbo => "gpt-3.5-turbo",
            ChatModel::Gpt4o => "gpt-4o",
            ChatModel::Gpt4omini => "gpt-4o-mini",
        }
    }
    //Todo 详细描述下每个模型
    pub fn list_available_models() -> Vec<(&'static str, &'static str)> {
        vec![
            (
                "gpt-3.5-turbo",
                "GPT-3.5 Turbo - Fastest and most cost-effective",
            ),
            ("gpt-4o", "GPT-4o - More capable but slower"),
            (
                "gpt-4o-mini",
                "GPT-4 Turbo - Latest model with improved capabilities",
            ),
        ]
    }
}

impl Default for ChatModel {
    fn default() -> Self {
        Self::Gpt35Turbo
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            api_key: std::env::var("OPENAI_API_KEY").ok(),
            model: ChatModel::default(),
        }
    }
}

impl ApiConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let config_str = fs::read_to_string(&config_path)?;
            Ok(serde_json::from_str(&config_str)?)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        // 确保配置目录存在
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, config_str)?;
        Ok(())
    }

    fn config_path() -> Result<std::path::PathBuf> {
        let mut path = dirs::home_dir().context("Could not find home directory")?;
        path.push(".config");
        path.push("cli-chat");
        path.push("config.json");
        Ok(path)
    }
}
