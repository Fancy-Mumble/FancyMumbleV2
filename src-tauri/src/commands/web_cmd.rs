use serde_json::json;
use tauri::State;
use tokio::sync::Mutex;
use webbrowser::{Browser, BrowserOptions};

use super::helper::OpenGraphCrawler;

#[tauri::command]
pub fn open_browser(url: &str) -> Result<(), String> {
    if let Err(e) = webbrowser::open_browser_with_options(
        Browser::Default,
        url,
        BrowserOptions::new().with_suppress_output(false),
    ) {
        return Err(format!("{e:?}"));
    }

    Ok(())
}

pub struct CrawlerState {
    pub crawler: Mutex<Option<OpenGraphCrawler>>,
}

#[tauri::command]
pub async fn get_open_graph_data_from_website(
    state: State<'_, CrawlerState>,
    url: &str,
) -> Result<String, String> {
    // setup crawler if not already done
    let result = {
        let mut client = state.crawler.lock().await;
        if client.is_none() {
            *client = OpenGraphCrawler::try_new();
        }

        client
            .as_ref()
            .ok_or_else(|| "Failed to read website body".to_string())?
            .crawl(url)
            .await
    };

    let result = json!(result);

    Ok(result.to_string())
}
