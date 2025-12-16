use async_trait::async_trait;
use anyhow::Result;
use scraper::{Html, Selector};
use crate::model::{ScrapeConfig, SpoonOutput, CodeBlock};
use crate::strategy::SpoonStrategy;
use crate::browser::fetch_page_headless;

pub struct GenericStrategy;

#[async_trait]
impl SpoonStrategy for GenericStrategy {
    fn can_handle(&self, _url: &str) -> bool {
        true
    }

    async fn execute(&self, url: &str, config: &ScrapeConfig) -> Result<SpoonOutput> {
        // Fast path: reqwest
        let mut html_content = match reqwest::get(url).await {
            Ok(resp) => resp.text().await.unwrap_or_default(),
            Err(_) => String::new(),
        };

        // Slow path: Headless if content is small or explicitly requested
        if config.render_js || html_content.len() < 500 {
            // Log or debug here if possible
            if let Ok(headless_content) = fetch_page_headless(url).await {
                html_content = headless_content;
            } else if html_content.is_empty() {
                 return Err(anyhow::anyhow!("Failed to fetch content via both methods"));
            }
        }

        let document = Html::parse_document(&html_content);
        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>());

        let mut code_snippets = Vec::new();
        if config.extract_code_blocks {
            let pre_selector = Selector::parse("pre").unwrap();
            let code_selector = Selector::parse("code").unwrap();

            for element in document.select(&pre_selector) {
                // Check if there is a child code element
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
        }

        // Clean
        let cleaner = ammonia::Builder::new()
             .add_tags(&["pre", "code"])
             .add_generic_attributes(&["class"])
             .clean(&html_content)
             .to_string();

        // Convert to Markdown
        let markdown = html2md::parse_html(&cleaner);

        Ok(SpoonOutput {
            url: url.to_string(),
            title,
            markdown_content: markdown,
            code_snippets,
            metadata: serde_json::json!({
                "source": "generic_scraper",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
        })
    }
}
