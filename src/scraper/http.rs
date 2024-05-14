
use std::sync::Arc;
use crate::{domain::{processable::Processable, scraper_result::ScraperResult}, log};

pub struct HttpScraper {
    url: String,
    processables: Vec<Arc<dyn Processable>>
}

impl HttpScraper {
    pub fn new(url: String, processables: Vec<Arc<dyn Processable>>) -> Self {
        Self {
            url,
            processables
        }
    }
}

pub trait Scraper {
    async fn run(&self);
}

impl Scraper for HttpScraper {
    async fn run(&self) {
        let url = self.url.clone();
        log::logger::info(format!("Http Scraper: running for {url}").as_str());

        let txt = self.rest_api_http_get_impl(self.url.clone()).await;
        log::logger::info(format!("Http Scraper: result: {txt:?}").as_str());

        let result = ScraperResult::new();

        for processable in self.processables.iter() {
            processable.process(result);
        }
    }
}

impl HttpScraper {
    async fn rest_api_http_get_impl(&self, url: String) -> Box<String> {
        let response = reqwest::get(url).await.unwrap().text().await;

        match response {
            Ok(text) => {
                return Box::new(text.to_string());
            }
            Err(error) => {
               log::logger::error(format!("Http Scraper: {error}").as_str());
               return Box::new("".to_string());
            }
        };
    }
}