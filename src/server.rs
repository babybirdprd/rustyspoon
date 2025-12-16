use axum::{
    routing::post,
    Router,
    Json,
};
use std::net::SocketAddr;
use crate::model::{ScrapeConfig, SpoonOutput};
use crate::strategies::get_strategy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScrapeRequest {
    pub url: String,
    pub config: Option<ScrapeConfig>,
}

async fn scrape_handler(Json(payload): Json<ScrapeRequest>) -> Json<SpoonOutput> {
    let config = payload.config.unwrap_or_default();
    let strategy = get_strategy(&payload.url);

    // In a real server, we would handle errors properly and return 500s.
    // For now, we unwrap or panic/return default error to fit the simple phase requirement.
    // Ideally we should change return type to Result<Json<SpoonOutput>, StatusCode>

    match strategy.execute(&payload.url, &config).await {
        Ok(output) => Json(output),
        Err(e) => {
             // Return an error output or panic. To be safe let's return a dummy error output
             // or just panic since we haven't defined error responses in the PRD.
             // Let's print error and return empty? No, let's try to do better.
             // We can return a specific error structure if we wanted, but let's stick to SpoonOutput with error metadata?
             Json(SpoonOutput {
                 url: payload.url,
                 title: Some("Error".to_string()),
                 markdown_content: format!("Error executing strategy: {}", e),
                 code_snippets: vec![],
                 metadata: serde_json::json!({"error": e.to_string()}),
             })
        }
    }
}

pub async fn run_server(addr: SocketAddr) {
    let app = Router::new()
        .route("/v1/scrape", post(scrape_handler));

    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
