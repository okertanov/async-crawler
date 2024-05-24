pub struct Configuration {
    scraper_api_url: String,
    scraper_sleep_interval_ms: u64,
    cron_periodic_interval_ms: u64,
    metrics_update_interval_ms: u64,
    svc_redis_endpoint: String,
    svc_db_endpoint: String,
}

//
// TODO: In the real-world case it supposed to support ENV + config files,
// but for now it's hardcoded.
//
impl Configuration {
    pub fn new() -> Self {
        Self {
            scraper_api_url: "https://uselessfacts.jsph.pl/api/v2/facts/random".to_string(),
            scraper_sleep_interval_ms: 100,     // 100 msec
            cron_periodic_interval_ms: 5_000,   // 5 sec
            metrics_update_interval_ms: 10_000, // 10 sec
            svc_redis_endpoint: "redis://cache/".to_string(),
            svc_db_endpoint: "database.sqlite3t".to_string(),
        }
    }

    pub fn get_scraper_api_url(&self) -> String {
       self.scraper_api_url.clone()
    }

    pub fn get_scraper_sleep_interval_ms(&self) -> u64 {
        self.scraper_sleep_interval_ms
    }

    pub fn get_cron_periodic_interval_ms(&self) -> u64 {
        self.cron_periodic_interval_ms
    }

    pub fn get_metrics_update_interval_ms(&self) -> u64 {
        self.metrics_update_interval_ms
    }

    pub fn get_svc_redis_endpoint(&self) -> String {
        self.svc_redis_endpoint.clone()
    }

    pub fn get_svc_db_endpoint(&self) -> String {
        self.svc_db_endpoint.clone()
    }
}