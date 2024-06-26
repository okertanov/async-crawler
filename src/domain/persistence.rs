use serde::{Serialize, Deserialize};
use super::scraper_result::ScraperResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersistRecord {
    data: ScraperResult
}

#[allow(dead_code)]
impl PersistRecord {
    pub fn new(data: ScraperResult) -> Self {
        Self {
            data
        }
    }
}

pub type PersistRecordCollection = Box<Vec<PersistRecord>>;

#[allow(dead_code)]
pub trait Persistence {
    fn store(&self, record: PersistRecord) -> Result<PersistRecord, std::io::Error>;
    fn store_many(&self, records: PersistRecordCollection) -> Result<PersistRecordCollection, std::io::Error>;
    fn get(&self, offset: usize, limit: usize) -> Result<PersistRecordCollection, std::io::Error>;
}