
use std::sync::Arc;
use crate::domain::{processable::Processable, scraper_result::ScraperResult};

pub struct HttpScraper {
    _url: String,
    _processables: Vec<Arc<dyn Processable>>
}

impl HttpScraper {
    pub fn new(url: String, processables: Vec<Arc<dyn Processable>>) -> Self {
        Self {
            _url: url,
            _processables: processables
        }
    }
}

pub trait Scraper {
    fn get(&self) -> ScraperResult;
}

impl Scraper for HttpScraper {
    fn get(&self) -> ScraperResult {
        return ScraperResult::new();
    }
}
