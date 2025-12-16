use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapeConfig {
    pub extract_code_blocks: bool, // Specialized parsing for code
    pub render_js: bool,
}

impl Default for ScrapeConfig {
    fn default() -> Self {
        Self {
            extract_code_blocks: true,
            render_js: false,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub content: String,
    pub line_count: usize,
}

#[derive(Debug, Serialize)]
pub struct SpoonOutput {
    pub url: String,
    pub title: Option<String>,
    pub markdown_content: String, // Main context for LLM
    pub code_snippets: Vec<CodeBlock>, // Separated code for analysis
    pub metadata: serde_json::Value,
}
