use crate::scraper::http::Scraper;
use crate::{config, log, processing, scraper};
use crate::periodic::cron::Schedulable;
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc};
use std::{thread, time};

pub struct App {
    config: config::configuration::Configuration
}

impl App {
    pub fn new(config: config::configuration::Configuration) -> Self {
        Self {
            config
        }
    }
}

pub trait Runnable {
    fn run(&self) -> u32;
}

impl Runnable for App {
    fn run(&self) -> u32 {
        log::logger::debug("Running...");

        // 1. Schedule periodic cron for the persistence
        let persistence_cron = crate::periodic::cron::Cron::new(
            self.config.get_cron_periodic_interval_ms()
        );
        persistence_cron.schedule();

        // 2. Instantiate all of these data scraping pipeline parts: 
        //   - HTTP Scraper
        //   - Results Core processor
        //   - Metrics dumper
        // TODO: to use D/I here
        let metrics_processor = processing::metrics::Metrics::new(
            self.config.get_metrics_update_interval_ms()
        );
        let core_processor = processing::core::Core::new();
        let http_scraper = scraper::http::HttpScraper::new(
            self.config.get_scraper_api_url(),
            vec![
                Arc::new(metrics_processor),
                Arc::new(core_processor),
            ]
        );
        
        // 3. Set-up SIGINT handler aka Ctrl-C to break the loop and gracefully shutdown
        let should_stop = Arc::new(AtomicBool::new(false));
        setup_sigint_handler(should_stop.clone());
        
        // 4. Run 'forever' scraper query cycle until terminated via SIGINT
        loop {     
            // 5. Scrap current iteration & pipe it for the processor non-blocking/asyncronously
            let _scraper_result = http_scraper.get(); 

            // 6. Sleep/trottle before next iteration
            sleep_trottle_next_with_progress(
                self.config.get_scraper_sleep_interval_ms()
            );

            // 7. Break the loop and stop if requested to do so
            if should_stop.load(Ordering::Relaxed) {
                break;
            }
        }

        // 8. Done.
        log::logger::debug("Done.");

        return 0;
    }
}

fn setup_sigint_handler(should_stop: Arc<AtomicBool>) {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        log::logger::debug("Terminating...");
        should_stop.store(true, Ordering::Relaxed);
    });
}

fn sleep_trottle_next_with_progress(millis: u64) {
    let duration_ms = time::Duration::from_millis(millis);
    thread::sleep(duration_ms);
    log::logger::progress(".");
}