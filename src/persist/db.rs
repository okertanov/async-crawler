use std::io::{Error, ErrorKind};

pub struct Db {
}

pub struct Record {
}

type RecordCollection = Box<[Record]>;

#[allow(dead_code)]
pub trait Persistence {
    fn store(&self, record: Record);
    fn get(&self, offset: u64, limit: u64) -> Result<RecordCollection, std::io::Error>;
}

impl Persistence for Db {
    fn store(&self, _record: Record) {
    }

    fn get(&self, _offset: u64, _limit: u64) -> Result<RecordCollection, std::io::Error> {
        Error::new(ErrorKind::Other, "Not implemented");
        Ok(Box::new([]))
    }
}