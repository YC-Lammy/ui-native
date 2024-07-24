use std::sync::Arc;

use gtk4::prelude::*;
use parking_lot::RwLock;

use crate::native_tree::NativeTextEditImp;

use super::NativeElement;


pub struct NativeTextEdit{
    edit: gtk4::TextView,
    scroll: gtk4::ScrolledWindow,
    text_changed_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>>,
}

impl NativeElement for NativeTextEdit{
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.scroll.as_ref()
    }
}

impl NativeTextEditImp for NativeTextEdit{
    fn new() -> Self {
        let buffer = gtk4::TextBuffer::new(None);
        let view = gtk4::TextView::with_buffer(&buffer);
        let text_changed_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>> = Arc::new(RwLock::new(None));
        let cb = text_changed_callback.clone();

        view.connect_buffer_notify(move |view|{
            let lock = cb.read();

            if let Some(cb) = lock.as_ref(){
                let buffer = view.buffer();
                let start = buffer.start_iter();
                let end = buffer.end_iter();
                (cb)(buffer.slice(&start, &end, false).as_str())
            }
        });

        view.set_height_request(100);
        view.set_width_request(100);
        view.set_hexpand(true);
        view.set_vexpand(true);

        let scroll = gtk4::ScrolledWindow::new();
        scroll.set_child(Some(&view));

        Self { 
            edit: gtk4::TextView::new(),
            scroll: scroll,
            text_changed_callback
        }
    }
    fn set_width(&self, width: f32) {
        self.scroll.set_width_request(width as i32);
    }
    fn set_height(&self, height: f32) {
        self.scroll.set_height_request(height as i32);
    }
    fn get_width(&self) -> f32 {
        self.scroll.allocated_width() as f32
    }
    fn get_height(&self) -> f32 {
        self.scroll.allocated_height() as f32
    }
    fn set_on_text_changed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>) {
        *self.text_changed_callback.write() = Some(callback)
    }
}