use async_trait::async_trait;
use anyhow::Result;
use crate::model::{ScrapeConfig, SpoonOutput};

#[async_trait]
pub trait SpoonStrategy: Send + Sync {
    /// Returns true if this strategy owns the domain (e.g., "github.com")
    fn can_handle(&self, url: &str) -> bool;

    /// The execution logic
    async fn execute(&self, url: &str, config: &ScrapeConfig) -> Result<SpoonOutput>;
}
