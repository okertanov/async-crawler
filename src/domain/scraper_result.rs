use serde::{Serialize, Deserialize};
use super::rest_fact_dto::RestFactDto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScraperResult {
    pub dto: RestFactDto,
    pub response_len: usize,
    pub timestamp_started: i64,
    pub timestamp_fetched: i64
}

impl ScraperResult {
    pub fn new(
        dto: RestFactDto,
        response_len: usize,
        timestamp_started: i64,
        timestamp_fetched: i64
    ) -> Self {
        Self {
            dto,
            response_len,
            timestamp_started,
            timestamp_fetched
        }
    }
}
