use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use crate::{cache, domain::{persistence::PersistRecord, processable::Processable, scraper_result::ScraperResult}};
use crate::domain::persistence::Persistence;

pub struct Core {
    cache: Arc<cache::inmem::InMem>
}

impl Core {
    pub fn new(cache: Arc<cache::inmem::InMem>) -> Self {
        Self {
            cache
        }
    }
}

#[async_trait]
impl Processable for Core {
    async fn process(&mut self, res: Arc<Mutex<ScraperResult>>) -> Arc<Mutex<ScraperResult>> {
        let res_cloned = res.clone();
        let result = res.lock().await;
        let record = PersistRecord::new(result.to_owned());
        let _result = self.cache.store(record);
        return res_cloned;
    }

    async fn run(&self) {
    }
}
