use reqwest::header::{HeaderMap, COOKIE};
use scraper::{Html, Selector};
use serde::Serialize;
use tauri::regex::Regex;
use tracing_subscriber::prelude::*;

#[derive(Debug, Serialize)]
pub struct OpenGraphMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

pub struct OpenGraphCrawler {
    client: reqwest::Client,
}

impl OpenGraphCrawler {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            //.cookie_store(true)
            .build()
            .unwrap();
        OpenGraphCrawler { client }
    }

    pub async fn crawl(&self, url: &str) -> Option<OpenGraphMetadata> {
        let response = self.client.get(url).send().await.ok()?;
        let body = response.text().await.ok()?;
        let document = Html::parse_document(&body);

        let title = self.extract_metadata(&document, "og:title");
        let description = self.extract_metadata(&document, "og:description");
        let image = self.extract_metadata(&document, "og:image");

        Some(OpenGraphMetadata {
            title,
            description,
            image,
        })
    }

    fn extract_metadata(&self, document: &Html, property: &str) -> Option<String> {
        let selector = Selector::parse(&format!("meta[property='{}']", property)).unwrap();
        let element = document.select(&selector).next()?;
        element.value().attr("content").map(String::from)
    }
}

pub(crate) fn extract_og_property(body: &str, pattern: &str) -> Result<String, String> {
    let re = Regex::new(pattern).map_err(|e| format!("{e:?}"))?;
    let property = re
        .captures(body)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str())
        .map(String::from)
        .ok_or("regex not found")?;

    Ok(property)
}
