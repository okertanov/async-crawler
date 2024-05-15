use super::rest_fact_dto::RestFactDto;


#[derive(Clone, Debug)]
pub struct ScraperResult {
    pub dto: RestFactDto,
    pub timestamp: i64
}

impl ScraperResult {
    pub fn new(dto: RestFactDto) -> Self {
        Self {
            dto,
            timestamp: chrono::Utc::now().timestamp()
        }
    }
}
