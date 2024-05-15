use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::domain::processable::Processable;
use crate::domain::scraper_result::ScraperResult;
use crate::{cache, log, persist};
use crate::domain::persistence::Persistence;

#[derive(Clone)]
pub struct Store {
    cache: Arc<cache::inmem::InMem>,
    db: Arc<persist::db::Db>
}

impl Store {
    pub fn new(
        cache: Arc<cache::inmem::InMem>,
        db: Arc<persist::db::Db>
    ) -> Self {
        Self {
            cache,
            db
        }
    }
}

#[async_trait]
impl Processable for Store {
    async fn process(&mut self, res: Arc<Mutex<ScraperResult>>) -> Arc<Mutex<ScraperResult>> {
        return res;
    }
    
    async fn run(&self) {
        let cached = self.cache.get(0, u64::MAX);
        match cached {
            Ok(record) => {
                let _stored = self.db.store_many(record);
            },
            Err(error) => {
                log::logger::error(format!("{error}").as_str());
            }
        }
    }
}