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
        // The max length of a Redis list is 2^32 - 1 (4,294,967,295),
        // so 65535 is used here.
        let cached = self.cache.get(usize::MIN, u16::MAX.into());
        match cached {
            Ok(records) => {
                log::logger::debug(format!("Found {:?} records", records.len()).as_str());
                let _stored = self.db.store_many(records);
            },
            Err(error) => {
                log::logger::error(format!("{error}").as_str());
            }
        }
    }
}