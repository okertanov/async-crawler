pub struct Cron {
    _period_ms: u64
}

impl Cron {
    pub fn new(period_ms: u64) -> Self {
        Self {
            _period_ms: period_ms
        }
    }
}
pub trait Schedulable {
    fn schedule(&self);
}

impl Schedulable for Cron {
    fn schedule(&self) {
    }
}