use clap::Parser;
use rustyspoon::model::ScrapeConfig;
use rustyspoon::strategies::get_strategy;
use rustyspoon::server::run_server;
use anyhow::Result;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL to scrape. If not provided, starts server (or use --server flag).
    /// Made optional to allow running as server without URL.
    #[arg(required_unless_present = "server")]
    url: Option<String>,

    /// Extract code blocks into separate JSON array
    #[arg(long, default_value_t = true)]
    extract_code: bool,

    /// Use headless browser
    #[arg(long, default_value_t = false)]
    render_js: bool,

    /// Output full JSON response
    #[arg(long, default_value_t = false)]
    json: bool,

    /// Run as API server
    #[arg(long)]
    server: bool,

    /// Bind address for server
    #[arg(long, default_value = "0.0.0.0:4000")]
    bind: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.server {
        let addr: SocketAddr = args.bind.parse().expect("Invalid bind address");
        run_server(addr).await;
        return Ok(());
    }

    if let Some(url) = args.url {
        let config = ScrapeConfig {
            extract_code_blocks: args.extract_code,
            render_js: args.render_js,
        };

        let strategy = get_strategy(&url);

        if strategy.can_handle(&url) {
            let output = strategy.execute(&url, &config).await?;
            if args.json {
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("{}", output.markdown_content);
            }
        } else {
            eprintln!("No strategy found for URL: {}", url);
        }
    }

    Ok(())
}
