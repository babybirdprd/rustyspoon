use async_trait::async_trait;
use anyhow::Result;
use scraper::{Html, Selector};
use crate::model::{ScrapeConfig, SpoonOutput, CodeBlock};
use crate::strategy::SpoonStrategy;

pub struct DocsRsStrategy;

#[async_trait]
impl SpoonStrategy for DocsRsStrategy {
    fn can_handle(&self, url: &str) -> bool {
        url.contains("docs.rs")
    }

    async fn execute(&self, url: &str, _config: &ScrapeConfig) -> Result<SpoonOutput> {
        let resp = reqwest::get(url).await?;
        let html_content = resp.text().await?;
        let document = Html::parse_document(&html_content);

        let markdown_content;
        let mut title = None;

        // Extract title
        let title_selector = Selector::parse("h1").unwrap();
         if let Some(h1) = document.select(&title_selector).next() {
            title = Some(h1.text().collect::<String>());
        }

        // Extract main content from the rustdoc structure
        // Usually id="main-content"
        let main_selector = Selector::parse("#main-content").unwrap();

        if let Some(main) = document.select(&main_selector).next() {
             let html = main.html();
              // Clean
            let cleaner = ammonia::Builder::new()
                .add_tags(&["pre", "code"])
                .clean(&html)
                .to_string();

            markdown_content = html2md::parse_html(&cleaner);
        } else {
             // Fallback
             markdown_content = html2md::parse_html(&html_content);
        }

        let mut code_snippets = Vec::new();
        let pre_selector = Selector::parse("pre").unwrap();
        let code_selector = Selector::parse("code").unwrap();

        // Extract code snippets from within the main content if possible, or document
        let search_root = if let Some(main) = document.select(&main_selector).next() {
            Html::parse_fragment(&main.html())
        } else {
             document.clone()
        };

        for element in search_root.select(&pre_selector) {
            let code_child = element.select(&code_selector).next();

            let (class_attr, content) = if let Some(code_el) = code_child {
                (code_el.value().attr("class"), code_el.text().collect::<String>())
            } else {
                (element.value().attr("class"), element.text().collect::<String>())
            };

            let language = class_attr.and_then(|classes| {
                classes.split_whitespace()
                    .find(|c| c.starts_with("language-") || c.starts_with("lang-"))
                    .map(|c| c.trim_start_matches("language-").trim_start_matches("lang-").to_string())
            });

            code_snippets.push(CodeBlock {
                language,
                line_count: content.lines().count(),
                content,
            });
        }

        Ok(SpoonOutput {
            url: url.to_string(),
            title,
            markdown_content,
            code_snippets,
            metadata: serde_json::json!({
                "source": "docs_rs_strategy",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
        })
    }
}
