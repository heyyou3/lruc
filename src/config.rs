use anyhow::{Context as _, Result};
use serde_derive::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct APIInfo {
    pub name: String,
    pub method: String,
    pub desc: String,
    pub path: String,
    pub headers: toml::Value,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_info: Vec<APIInfo>,
    pub base_url: String,
}

pub fn read_config(path: &str) -> Result<Config> {
    let file_str = std::fs::read_to_string(path)
        .with_context(|| format!("Failed File load. path = {}", path))?;
    let config: Config =
        toml::from_str(&file_str).with_context(|| format!("Parse Error {}", path))?;
    Ok(config)
}
