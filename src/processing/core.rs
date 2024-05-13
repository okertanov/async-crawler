use std::sync::Arc;
use crate::{cache, domain::processable::Processable};

pub struct Core {
    _cache: Arc<cache::inmem::InMem>
}

impl Core {
    pub fn new() -> Self {
        Self {
            _cache: Arc::new(cache::inmem::InMem::new())
        }
    }
}

impl Processable for Core {
}