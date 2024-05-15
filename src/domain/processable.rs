use std::sync::Arc;
use super::scraper_result::ScraperResult;

#[allow(dead_code)]
pub trait Processable {
    fn process(&self, res: Arc<ScraperResult>) -> Arc<ScraperResult>;
}