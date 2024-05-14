use std::sync::Arc;
use crate::{cache, domain::{persistence::PersistRecord, processable::Processable, scraper_result::ScraperResult}, log};
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

impl Processable for Core {
    fn process(&self, res: ScraperResult) -> ScraperResult {
        log::logger::debug("Core processing: process");

        let record = PersistRecord::new();
        let _result = self.cache.store(record);
        return res;
    }
}
