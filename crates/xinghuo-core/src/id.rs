use std::{
    num::NonZeroU64,
    sync::atomic::{AtomicU64, Ordering},
};

pub struct Counter(AtomicU64);

impl Counter {
    pub const fn new() -> Self {
        Self(AtomicU64::new(0))
    }
    pub fn next_nonzero(&self) -> NonZeroU64 {
        unsafe { NonZeroU64::new_unchecked(self.0.fetch_add(1, Ordering::Relaxed)) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Id(NonZeroU64);

impl Id {
    pub fn next() -> Self {
        static ID_COUNTER: Counter = Counter::new();
        Self(ID_COUNTER.next_nonzero())
    }
}

pub type IdPath = Vec<Id>;
