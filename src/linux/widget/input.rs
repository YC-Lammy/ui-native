use std::sync::Arc;

use gtk4::prelude::*;
use parking_lot::RwLock;

use crate::native_tree::NativeTextInputImp;

use super::NativeElement;


pub struct NativeTextInput{
    input: gtk4::Entry,
    text_changed_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>>,
    enter_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>>
}

impl NativeElement for NativeTextInput{
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.input.as_ref()
    }
}

impl NativeTextInputImp for NativeTextInput{
    fn new() -> Self {
        let entry = gtk4::Entry::new();
        let text_changed_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>> = Arc::new(RwLock::new(None));
        let cb = text_changed_callback.clone();

        entry.connect_changed(move |entry|{
            let lock = cb.read();

            if let Some(cb) = lock.as_ref(){
                (cb)(entry.text().as_str())
            }
        });

        let enter_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>> = Arc::new(RwLock::new(None));
        let cb = enter_callback.clone();

        entry.connect_activate(move |entry|{
            let lock = cb.read();

            if let Some(cb) = lock.as_ref(){
                (cb)(entry.text().as_str())
            }
        });

        Self { 
            input: entry,
            text_changed_callback,
            enter_callback
        }
    }

    fn set_width(&self, width: f32) {
        self.input.set_width_request(width as i32);
    }
    fn set_height(&self, height: f32) {
        self.input.set_height_request(height as i32);
    }
    fn get_width(&self) -> f32 {
        self.input.width() as f32
    }
    fn get_height(&self) -> f32 {
        self.input.height() as f32
    }
    fn set_background_text(&self, text: &str) {
        self.input.set_placeholder_text(Some(text))
    }
    fn set_on_text_changed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>) {
        *self.text_changed_callback.write() = Some(callback)
    }
    fn set_on_enter_pressed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>) {
        *self.enter_callback.write() = Some(callback)
    }
}