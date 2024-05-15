
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{domain::{processable::Processable, rest_fact_dto::RestFactDto, scraper_result::ScraperResult}, log};

pub struct HttpScraper {
    url: String,
    processables: Vec<Arc<Mutex<dyn Processable>>>
}

impl HttpScraper {
    pub fn new(url: String, processables: Vec<Arc<Mutex<dyn Processable>>>) -> Self {
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
        // let url = self.url.clone();
        // log::logger::info(format!("Http Scraper: running for {url}").as_str());

        // Call actual REST endpoint to get the results
        let timestamp_started = chrono::Utc::now().timestamp_millis();
        let response_txt = self.rest_api_http_get_impl(self.url.clone()).await;
        // log::logger::info(format!("Http Scraper: result: {response_txt:?}").as_str());

        // Not interested if empty response or error, just being logged,
        // and http errors are being logged at the Impl level.
        if response_txt.is_empty() {
            log::logger::warn(format!("Http Scraper: empty response").as_str());
            return;
        }

        // Deserialize response via serde 
        let rest_fact_dto: RestFactDto = match serde_json::from_str(&response_txt) {
            Ok(rest_fact_dto) => rest_fact_dto,
            Err(error) => {
                log::logger::error(format!("Http Scraper: {error}").as_str());
                return;
            }
        };

        let timestamp_fetched = chrono::Utc::now().timestamp_millis();
        let result = Arc::new(
            Mutex::new(
                ScraperResult::new(
                    rest_fact_dto,
                    response_txt.len(),
                    timestamp_started,
                    timestamp_fetched
        )));

        // println!("🐞 {:?}", result);

        // Run all processibles over the result
        for processable in self.processables.iter() {
            let mut processable_locked = processable.lock().await;
            processable_locked.process(result.clone()).await;
        }
    }
}

impl HttpScraper {
    async fn rest_api_http_get_impl(&self, url: String) -> Box<String> {
        let response = match reqwest::get(url).await {
            Ok(response) => response,
            Err(error) => {
                log::logger::error(format!("Http Scraper: {error}").as_str());
                return Box::new("".to_string());
            }
        };

        let text = match response.text().await {
            Ok(text) => text,
            Err(error) => {
                log::logger::error(format!("Http Scraper: {error}").as_str());
                return Box::new("".to_string());
            }
        };

        return Box::new(text.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::rest_fact_dto::RestFactDto;

    #[test]
    fn serialize_for_dto() {
        let rest_fact_dto = RestFactDto::new(
            "1".to_string(),
            "text".to_string(),
            "test".to_string(),
            "https://tld.de/".to_string(),
            "de".to_string(),
            "https://tld.de/1".to_string()
        );
        let serialized = serde_json::to_string(&rest_fact_dto).unwrap();
        assert!(serialized.len() > 0);
        assert!(serialized.contains(rest_fact_dto.permalink.as_str()));
        assert!(serialized.contains(rest_fact_dto.source_url.as_str()));
    }

    #[test]
    fn deserialize_for_dto() {
        let raw_json_str = "{\"id\":\"8c8f92c9c74bdab48d73bbdce50c06d5\",\"text\":\"A horse can look forward with one eye and back with the other.\",\"source\":\"djtech.net\",\"source_url\":\"http://www.djtech.net/humor/useless_facts.htm\",\"language\":\"en\",\"permalink\":\"https://uselessfacts.jsph.pl/api/v2/facts/8c8f92c9c74bdab48d73bbdce50c06d5\"}\n";
        let deserialized: RestFactDto = serde_json::from_str(&raw_json_str).unwrap();
        assert!(deserialized.id.len() > 0);
        assert!(deserialized.id == "8c8f92c9c74bdab48d73bbdce50c06d5".to_string());
        assert!(deserialized.language.len() > 0);
        assert!(deserialized.language == "en".to_string());
        assert!(deserialized.text.len() > 0);
    }
}