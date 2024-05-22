use std::{io::{Error, ErrorKind}, sync::Arc};
use redis::Commands;

use crate::{domain::persistence::{PersistRecord, PersistRecordCollection, Persistence}, log::{self, logger}};

#[derive(Clone)]
pub struct InMem {
    client: Arc<redis::Client>
}

impl InMem {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Self::connect())
        }
    }

    fn connect() -> redis::Client {
        let client = redis::Client::open("redis://cache/");
        return client.unwrap();
    }
}

impl Persistence for InMem {
    fn store(&self, record: PersistRecord) -> Result<PersistRecord, std::io::Error> {
        let serialized = serde_json::to_string(&record).unwrap();
        let connection = self.client.get_connection();
        let _: () = connection.unwrap().rpush("InMemList", serialized).unwrap();
        return Ok(record);
    }

    fn store_many(&self, records: PersistRecordCollection) -> Result<PersistRecordCollection, std::io::Error> {
        return Ok(records);
    }

    fn get(&self, _offset: u64, _limit: u64) -> Result<PersistRecordCollection, std::io::Error> {
        Err(Error::new(ErrorKind::Other, "Not implemented"))
    }
}