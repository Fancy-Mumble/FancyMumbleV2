use reqwest::redirect::Policy;
use scraper::{Html, Selector};
use serde::Serialize;

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
            .redirect(Policy::limited(10))
            .user_agent("FancyMumbleClient/0.1.0")
            //.cookie_store(true)
            .build()
            .unwrap();
        OpenGraphCrawler { client }
    }

    pub async fn crawl(&self, url: &str) -> Option<OpenGraphMetadata> {
        let response = self.client.get(url).send().await.ok()?;
        let body = response.text().await.ok()?;
        let document = Html::parse_document(&body);

        let mut title = self.extract_metadata(&document, "og:title");
        let description = self.extract_metadata(&document, "og:description");
        let image = self.extract_metadata(&document, "og:image");

        if title.is_none() {
            title = self.extract_property(&document, "title");
        }

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

    fn extract_property(&self, document: &Html, property: &str) -> Option<String> {
        let selector = Selector::parse(property).unwrap();
        let element = document.select(&selector).next()?;
        Some(element.children().next()?.value().as_text()?.to_string())
    }
}
