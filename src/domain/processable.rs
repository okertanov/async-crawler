use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use super::scraper_result::ScraperResult;

#[async_trait]
pub trait Processable {
    async fn process(&mut self, res: Arc<Mutex<ScraperResult>>) -> Arc<Mutex<ScraperResult>>;
    async fn run(&self);
}