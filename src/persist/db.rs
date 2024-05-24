use std::io::{Error, ErrorKind};
use diesel::{Connection, SqliteConnection};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use crate::domain::persistence::{Persistence, PersistRecord, PersistRecordCollection};

diesel::table! {
    entity (id) {
        id -> Integer,
        body -> Text,
        date_created -> BigInt
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::persist::db::entity)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PersistRecordEntity {
    pub id: i32,
    pub body: String,
    pub date_created: i64,
}

#[derive(Clone)]
pub struct Db {
    db_endpoint: String
}

impl Db {
    pub fn new(db_endpoint: String) -> Self {
        Self {
            db_endpoint
        }
    }

    pub fn establish_connection(&self) -> SqliteConnection {
        dotenv().ok();
        return SqliteConnection::establish(&self.db_endpoint).unwrap();
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