use async_trait::async_trait;
use anyhow::Result;
use crate::model::{ScrapeConfig, SpoonOutput, CodeBlock};
use crate::strategy::SpoonStrategy;

pub struct GithubStrategy;

#[async_trait]
impl SpoonStrategy for GithubStrategy {
    fn can_handle(&self, url: &str) -> bool {
        url.contains("github.com")
    }

    async fn execute(&self, url: &str, _config: &ScrapeConfig) -> Result<SpoonOutput> {
        // Simple logic: if it's a blob url, fetch raw content.
        // e.g. https://github.com/user/repo/blob/main/src/main.rs
        // Raw url: https://raw.githubusercontent.com/user/repo/main/src/main.rs

        let mut raw_url = url.to_string();
        let mut title = None;
        let mut is_code = false;

        if url.contains("/blob/") {
            raw_url = url.replace("github.com", "raw.githubusercontent.com")
                         .replace("/blob/", "/");
            is_code = true;

            // Try to deduce title/filename
             if let Some(segments) = url.split('/').last() {
                title = Some(segments.to_string());
            }
        }

        let content = reqwest::get(&raw_url).await?.text().await?;

        let mut code_snippets = Vec::new();
        let markdown_content;

        if is_code {
            // If it is a code file, wrap it in markdown block
            // Try to detect extension
            let ext = raw_url.split('.').last().unwrap_or("txt");
            markdown_content = format!("```{}\n{}\n```", ext, content);

            code_snippets.push(CodeBlock {
                language: Some(ext.to_string()),
                line_count: content.lines().count(),
                content: content.clone(),
            });
        } else {
            // Fallback for non-blob github urls (e.g. issues, readme) - for now just raw text or markdown if README
            markdown_content = content.clone();
        }

        Ok(SpoonOutput {
            url: url.to_string(),
            title,
            markdown_content,
            code_snippets,
            metadata: serde_json::json!({
                "source": "github_strategy",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
        })
    }
}
