
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
    fn run(&self);
}

impl Scraper for HttpScraper {
    fn run(&self) {
        let url = self.url.clone();
        log::logger::info(format!("Http Scraper: running for {url}").as_str());

        let result = ScraperResult::new();

        for processable in self.processables.iter() {
            processable.process(result);
        }
    }
}
