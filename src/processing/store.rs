use std::sync::Arc;
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

    pub fn run(&self) {
        log::logger::debug("Store processing: run");

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
