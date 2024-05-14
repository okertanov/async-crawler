use std::io::{Error, ErrorKind};
use crate::domain::persistence::{Persistence, PersistRecord, PersistRecordCollection};

#[derive(Clone)]
pub struct InMem {
}

impl InMem {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Persistence for InMem {
    fn store(&self, _record: PersistRecord) -> Result<PersistRecord, std::io::Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented"))
    }

    fn store_many(&self, _records: PersistRecordCollection) -> Result<PersistRecordCollection, std::io::Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented"))
    }

    fn get(&self, _offset: u64, _limit: u64) -> Result<PersistRecordCollection, std::io::Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented"))
    }
}