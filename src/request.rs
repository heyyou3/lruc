use anyhow::{Context as _, Result};
use http::HeaderMap;
use reqwest;

#[tokio::main]
pub async fn get(url: String, headers: HeaderMap) -> Result<reqwest::Response> {
    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .with_context(|| format!("[GET] {}", url))?;
    Ok(res)
}
