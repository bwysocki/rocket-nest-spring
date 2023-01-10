use std::sync::atomic::{AtomicU64, Ordering};

pub struct VisitorCounter {
    pub visitor: AtomicU64,
}

impl VisitorCounter {
    pub fn increment(&self) {
        self.visitor.fetch_add(1, Ordering::Relaxed);
        println!(
            "The number of visitor is: {}",
            self.visitor.load(Ordering::Relaxed)
        );
    }
}