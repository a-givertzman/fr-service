use std::sync::atomic::{AtomicUsize, Ordering};

pub trait AtomicReset<T> {
    fn reset(&self, val: T);
}

impl AtomicReset<usize> for AtomicUsize {
    fn reset(&self, val: usize) {
        self.store(val, Ordering::SeqCst)
    }
}

