use clap::Parser;
use rustyspoon::model::ScrapeConfig;
use rustyspoon::strategies::generic::GenericStrategy;
use rustyspoon::strategy::SpoonStrategy;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL to scrape
    url: String,

    /// Extract code blocks into separate JSON array
    #[arg(long, default_value_t = true)]
    extract_code: bool,

    /// Use headless browser (not implemented in Phase 1)
    #[arg(long, default_value_t = false)]
    render_js: bool,

    /// Output full JSON response
    #[arg(long, default_value_t = false)]
    json: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = ScrapeConfig {
        extract_code_blocks: args.extract_code,
        render_js: args.render_js,
    };

    // Strategy resolution (simple for Phase 1)
    let strategy = GenericStrategy;

    if strategy.can_handle(&args.url) {
        let output = strategy.execute(&args.url, &config).await?;
        if args.json {
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            println!("{}", output.markdown_content);
        }
    } else {
        eprintln!("No strategy found for URL: {}", args.url);
    }

    Ok(())
}
