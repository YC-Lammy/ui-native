

use std::sync::Arc;

use gtk4::prelude::*;
use parking_lot::RwLock;

use crate::native_tree::traits::NativeButtonImp;

pub struct NativeButton{
    button: gtk4::Button,
    callback: Arc<RwLock<Option<Arc<dyn Fn() + Send + Sync>>>>
}

impl NativeButtonImp for NativeButton{
    fn new() -> Self{
        let button_callback: Arc<RwLock<Option<Arc<dyn Fn() + Send + Sync>>>> = Arc::new(RwLock::new(None));
        let button_callback_cloned  = button_callback.clone();

        let button = gtk4::Button::new();

        button.connect_clicked(move |_b|{
            let r = button_callback_cloned.read();

            if let Some(cb) = r.as_ref(){
                cb();
            }
        });

        Self { 
            button: button,
            callback: button_callback
        }
    }

    fn set_width(&self, width: f32) {
        self.button.set_width_request(width as _);
    }

    fn set_height(&self, height: f32) {
        self.button.set_height_request(height as _);
    }

    fn set_visible(&self, visible: bool) {
        self.button.set_visible(visible)
    }

    fn set_disabled(&self, disabled: bool){
        self.button.set_can_target(!disabled)
    }

    fn set_label(&self, text: String){
        self.button.set_label(&text)
    }

    fn set_on_click(&self, on_click: Option<Arc<dyn Fn() + Send + Sync>>){
        *self.callback.write() = on_click;
    }
}

impl super::NativeElement for NativeButton{
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.button.as_ref()
    }
}