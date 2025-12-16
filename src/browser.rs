use anyhow::{Result, Context};
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;

pub async fn fetch_page_headless(url: &str) -> Result<String> {
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build browser config: {}", e))?
    )
    .await
    .context("Failed to launch browser")?;

    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let page = browser.new_page(url).await.context("Failed to create new page")?;

    // Wait for the page to load.
    // We can add more sophisticated waiting strategies later (e.g. wait for selector).
    // For now, simple load wait.
    page.wait_for_navigation().await.ok();

    // Get the HTML
    let content = page.content().await.context("Failed to get page content")?;

    browser.close().await.context("Failed to close browser")?;
    handle.await.context("Browser handler task failed")?;

    Ok(content)
}
