use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RestFactDto {
    pub id: String,
    pub text: String,
    pub source: String,
    pub source_url: String,
    pub language: String,
    pub permalink: String,
}

#[allow(dead_code)]
impl RestFactDto {
    pub fn new(
        id: String,
        text: String,
        source: String,
        source_url: String,
        language: String,
        permalink: String,
    ) -> Self {
        Self {
            id,
            text,
            source,
            source_url,
            language,
            permalink,
        }
    }
}
