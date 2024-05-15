use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use crate::{domain::{processable::Processable, scraper_result::ScraperResult}, log};

#[derive(Clone)]
pub struct Metrics {
    _update_interval_ms: u64,
    total_size: usize,
    total_time: i64
}

impl Metrics {
    pub fn new(update_interval_ms: u64) -> Self {
        Self {
            _update_interval_ms: update_interval_ms,
            total_size: 1,
            total_time: 1
        }
    }
}

#[async_trait]
impl Processable for Metrics {
    async fn process(&mut self, res: Arc<Mutex<ScraperResult>>) -> Arc<Mutex<ScraperResult>> {
        let res_locked = res.lock().await;
        self.total_size += res_locked.response_len;
        self.total_time += res_locked.timestamp_fetched - res_locked.timestamp_started;
        std::mem::drop(res_locked);

        let total_size = self.total_size;
        let total_time = self.total_time;
        let total_size_formatted = (total_size as f64) / 1024.0 / 1024.0;
        let total_time_formatted = total_time / 1000;
        let total_bps_rate = self.total_size as f64 / self.total_time as f64 * 1000.0 / 1024.0 / 1024.0;
        log::logger::info(format!("Metrics: {total_size}/{total_time} - {total_size_formatted:.3} MB - {total_time_formatted} s - {total_bps_rate:.3} Mbps").as_str());

        return res;
    }
}