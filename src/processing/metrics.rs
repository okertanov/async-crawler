use crate::domain::processable::Processable;

pub struct Metrics {
    _update_interval_ms: u64
}

impl Metrics {
    pub fn new(update_interval_ms: u64) -> Self {
        Self {
            _update_interval_ms: update_interval_ms
        }
    }
}

impl Processable for Metrics {
}