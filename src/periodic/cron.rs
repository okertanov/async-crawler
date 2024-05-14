use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::time::Duration;
use tokio::time;
use tokio_stream::{wrappers::IntervalStream, StreamExt};
use crate::{log, processing};

#[derive(Clone)]
pub struct Cron {
    period_ms: u64,
    // TODO: To use Processible Trait here
    store: Arc<processing::store::Store>,
    started: Arc<AtomicBool>
}

impl Cron {
    pub fn new(period_ms: u64, store: Arc<processing::store::Store>) -> Self {
        Self {
            period_ms,
            store,
            started: Arc::new(AtomicBool::new(false))
        }
    }
}
pub trait Schedulable {
    async fn schedule(&self);
    fn terminate(&self);
}

impl Schedulable for Cron {
    async fn schedule(&self) {
        log::logger::debug("Cron: scheduling...");

        self.started.store(true, Ordering::Relaxed);

        let mut stream = IntervalStream::new(
            time::interval(Duration::from_millis(self.period_ms))
        );

        while let Some(_ts) = stream.next().await {
            log::logger::debug("Cron: activated.");

            if !self.started.load(Ordering::Relaxed) {
                break;
            }

            // Do actual work
            self.store.run();
        }

        log::logger::debug("Cron: terminated.");
    }

    fn terminate(&self) {
        log::logger::debug("Cron: terminating...");
        self.started.store(false, Ordering::Relaxed);
    }
}