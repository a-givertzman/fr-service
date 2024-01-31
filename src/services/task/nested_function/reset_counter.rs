use std::sync::atomic::{AtomicUsize, Ordering};

pub trait AtomicReset {
    fn reset(&self) {}
}

impl AtomicReset for AtomicUsize {
    fn reset(&self) {
        self.store(0, Ordering::SeqCst)
    }
}

