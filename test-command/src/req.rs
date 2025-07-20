use anyhow::{Context, Result};
use reqwest::Client;

pub async fn get(url: &str) -> Result<()> {
    println!("正在请求: {}", url);

    // 发送GET请求
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("无法发送请求到 {}", url))?;

    // 检查响应状态
    if !response.status().is_success() {
        anyhow::bail!("请求失败，状态码: {}", response.status());
    }

    // 获取响应内容（字符串形式）
    let body = response.text().await.context("无法获取响应内容")?;

    // 打印响应内容
    println!("\n响应内容:\n{}", body);

    Ok(())
}
