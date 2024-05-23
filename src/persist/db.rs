use std::io::{Error, ErrorKind};
use crate::domain::persistence::{Persistence, PersistRecord, PersistRecordCollection};

#[derive(Clone)]
pub struct Db {
}

impl Db {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Persistence for Db {
    fn store(&self, _record: PersistRecord) -> Result<PersistRecord, std::io::Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented"))
    }

    fn store_many(&self, _records: PersistRecordCollection) -> Result<PersistRecordCollection, std::io::Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented"))
    }

    fn get(&self, _offset: usize, _limit: usize) -> Result<PersistRecordCollection, std::io::Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented"))
    }
}