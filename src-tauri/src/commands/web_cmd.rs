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

#[tauri::command]
pub async fn get_tenor_search_results(
    api_key: &str,
    query: &str,
    limit: u32,
    pos: u32,
) -> Result<String, String> {
    let params = format!("&q={query}&limit={limit}&pos={pos}");

    get_tenor_results(api_key, "search", params).await
}

#[tauri::command]
pub async fn get_tenor_trending_results(api_key: &str) -> Result<String, String> {
    get_tenor_results(api_key, "trending", String::new()).await
}

async fn get_tenor_results(api_key: &str, api: &str, params: String) -> Result<String, String> {
    let url = format!("https://api.tenor.com/v1/{api}?key={api_key}{params}");

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("{e:?}"))?
        .text()
        .await
        .map_err(|e| format!("{e:?}"))?;

    Ok(response)
}
