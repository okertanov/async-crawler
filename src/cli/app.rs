use tokio::sync::Mutex;
use crate::scraper::http::Scraper;
use crate::{cache, config, log, persist, processing, scraper};
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
    async fn run(&self) -> u32;
}

impl Runnable for App {
    async fn run(&self) -> u32 {
        log::logger::info("App running...");

        // 1. First of all tp setup SIGINT handler aka Ctrl-C
        // to break the loop and gracefully shutdown
        let should_stop = Arc::new(AtomicBool::new(false));
        setup_sigint_handler(should_stop.clone());

        // 2. Instantiate all of these data scraping pipeline parts: 
        //   - HTTP Scraper
        //   - Results Core processor
        //   - Metrics dumper
        //   - InMemory cache
        //   - Persistence DB adapter
        //   - DB Store processor 
        // TODO: to use D/I here
        let inmem_cache = Arc::new(cache::inmem::InMem::new(
            self.config.get_svc_redis_endpoint(),
        ));
        let persist_db = Arc::new(persist::db::Db::new());
        let store_processor = Arc::new(
            Mutex::new(
                processing::store::Store::new(inmem_cache.clone(), persist_db)
            )
        );
        let core_processor = Arc::new(
            Mutex::new(
                processing::core::Core::new(inmem_cache)
            )
        );
        let metrics_processor = Arc::new(
            Mutex::new(
                processing::metrics::Metrics::new(
                    self.config.get_metrics_update_interval_ms()
                )
            )
        );
        let http_scraper = Arc::new(scraper::http::HttpScraper::new(
            self.config.get_scraper_api_url(),
            vec![
                core_processor,
                metrics_processor,
            ]
        ));

        // 3. Schedule periodic cron for the timed persistence
        let persistence_cron = Arc::new(
            crate::periodic::cron::Cron::new(
                self.config.get_cron_periodic_interval_ms(),
                store_processor
            )
        );
        let persistence_cron_clone = persistence_cron.clone();
        tokio::spawn(async move {
            persistence_cron_clone.schedule().await;
        });
        
        // 4. Run 'forever' scraper query cycle until terminated via SIGINT
        loop {     
            // 5. Schedule 'scrap' current iteration & pipe it for the processor,
            // it should be performed non-blocking/asyncronously via Tokyo tasks pool
            http_scraper.run().await;

            // 6. Sleep/trottle before next iteration
            sleep_trottle_next_with_progress(
                self.config.get_scraper_sleep_interval_ms()
            );

            // 7. Break the loop and stop if requested to do so
            if should_stop.load(Ordering::Relaxed) {
                break;
            }
        }

        // 8. Terminate persistence cron
        persistence_cron.terminate();

        // 9. Done.
        log::logger::info("App done.");

        return 0;
    }
}

fn setup_sigint_handler(should_stop: Arc<AtomicBool>) {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        log::logger::debug("App terminating...");
        should_stop.store(true, Ordering::Relaxed);
    });
}

fn sleep_trottle_next_with_progress(millis: u64) {
    let duration_ms = time::Duration::from_millis(millis);
    thread::sleep(duration_ms);
}