use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::time::Duration;
use tokio::{sync::Mutex, time};
use tokio_stream::{wrappers::IntervalStream, StreamExt};
use crate::{domain::processable::Processable, log};

#[derive(Clone)]
pub struct Cron {
    period_ms: u64,
    processable: Arc<Mutex<dyn Processable + Send>>,
    started: Arc<AtomicBool>
}

impl Cron {
    pub fn new(period_ms: u64, processable: Arc<Mutex<dyn Processable + Send>>) -> Self {
        Self {
            period_ms,
            processable,
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
            if !self.started.load(Ordering::Relaxed) {
                break;
            }

            // Do actual work
            let processable_locked = self.processable.clone().lock_owned().await;
            processable_locked.run().await;
            std::mem::drop(processable_locked);
        }

        log::logger::debug("Cron: terminated.");
    }

    fn terminate(&self) {
        log::logger::debug("Cron: terminating...");
        self.started.store(false, Ordering::Relaxed);
    }
}