pub mod generic;
pub mod github;
pub mod docs_rs;

use crate::strategy::SpoonStrategy;
use generic::GenericStrategy;
use github::GithubStrategy;
use docs_rs::DocsRsStrategy;

pub fn get_strategy(url: &str) -> Box<dyn SpoonStrategy> {
    if url.contains("github.com") {
        Box::new(GithubStrategy)
    } else if url.contains("docs.rs") {
        Box::new(DocsRsStrategy)
    } else {
        Box::new(GenericStrategy)
    }
}
