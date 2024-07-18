use core::sync::atomic::{AtomicBool, Ordering};

pub struct Context {
    need_redraw: AtomicBool,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            need_redraw: AtomicBool::new(false),
        }
    }
    pub fn request_redraw(&self) {
        self.need_redraw.store(true, Ordering::Relaxed)
    }
}
