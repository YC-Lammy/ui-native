pub mod command;
pub mod commit;
pub mod component;

use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeID(u64);

impl Default for NodeID {
    fn default() -> Self {
        Self::new_unique()
    }
}

impl NodeID {
    pub fn new_unique() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}
