use gtk4::prelude::*;

use crate::native_tree::NativeImageViewImp;

use super::NativeElement;


pub struct NativeImageView{
    view: gtk4::Picture
}

impl NativeElement for NativeImageView{
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.view.as_ref()
    }
}

impl NativeImageViewImp for NativeImageView{
    fn new() -> Self {
        Self { 
            view: gtk4::Picture::new()
        }
    }
    fn set_width(&self, width: f32) {
        self.view.set_width_request(width as _);
    }
    fn set_height(&self, height: f32) {
        self.view.set_height_request(height as i32);
    }
    fn get_width(&self) -> f32 {
        self.view.allocated_width() as f32
    }
    fn get_height(&self) -> f32 {
        self.view.allocated_height() as f32
    }
    fn set_visible(&self, visible: bool) {
        self.view.set_visible(visible);
    }
    fn set_source(&self) {

    }
}