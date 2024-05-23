use std::sync::Arc;
use redis::Commands;
use crate::domain::persistence::{PersistRecord, PersistRecordCollection, Persistence};


#[derive(Clone)]
pub struct InMem {
    client: Arc<redis::Client>
}

impl InMem {
    const STORAGE_LIST_KEY: &'static str = "InMemList";

    pub fn new(redis_endpoint: String) -> Self {
        Self {
            client: Arc::new(InMem::connect(redis_endpoint))
        }
    }

    fn connect(redis_endpoint: String) -> redis::Client {
        let client = redis::Client::open(redis_endpoint.as_str());
        return client.unwrap();
    }
}

impl Persistence for InMem {
    fn store(&self, record: PersistRecord) -> Result<PersistRecord, std::io::Error> {
        let connection = self.client.get_connection();
        let record_serialized = serde_json::to_string(&record).unwrap();
        let _: () = connection
            .unwrap()
            .rpush(
                InMem::STORAGE_LIST_KEY,
                record_serialized)
            .unwrap();
        return Ok(record);
    }

    fn store_many(&self, records: PersistRecordCollection) ->
        Result<
            PersistRecordCollection,
            std::io::Error
        > {
        let connection = self.client.get_connection();
        let records_serialized: Vec<String>  = records
            .iter()
            .map(
                |record|
                serde_json::to_string(&record).unwrap()
            )
            .collect();
        let _: i64 = connection
            .unwrap()
            .rpush::<String, Vec<String>, i64>(
                InMem::STORAGE_LIST_KEY.to_string(),
                records_serialized
            )
            .unwrap();
        return Ok(records);
    }

    fn get(&self, _offset: usize, limit: usize) ->
        Result<
            PersistRecordCollection,
            std::io::Error
        > {
        let connection = self.client.get_connection();
        let optional_limit = match limit {
            0 => None,
            _ => std::num::NonZeroUsize::new(limit)
        };
        let records_serialized: Vec<String> = connection
            .unwrap()
            .lpop(
                InMem::STORAGE_LIST_KEY.to_string(),
                optional_limit
            )
            .unwrap();
        let records_deserialized: Vec<PersistRecord> = records_serialized
            .iter()
            .map(
                |s|
                serde_json::from_str(s).unwrap()
            )
            .collect();
        return Ok(Box::new(records_deserialized));
    }

}