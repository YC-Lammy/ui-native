
use gtk4::prelude::*;

use crate::native_tree::NativeScrollViewImp;

use super::NativeElement;

pub struct NativeScrollView{
    view: gtk4::ScrolledWindow
}

impl NativeElement for NativeScrollView{
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.view.as_ref()
    }
}

impl NativeScrollViewImp for NativeScrollView{
    fn new() -> Self {
        let scroll = gtk4::ScrolledWindow::new();
        Self { 
            view: scroll
        }
    }
    fn set_width(&self, width: f32) {
        self.view.set_width_request(width as i32);
    }
    fn set_height(&self, height: f32) {
        self.view.set_height_request(height as i32);
    }
    fn set_child(&self, child: &dyn NativeElement) {
        self.view.set_child(Some(child.as_gtk4_widget()))
    }
    fn remove_child(&self) {
        self.view.set_child(None::<&gtk4::Widget>);
    }
    fn scroll_to_horizontal(&self, location: f64) {
        let adj = self.view.hadjustment();
        adj.set_upper(1.0);
        adj.set_lower(0.0);
        adj.set_value(location);
    }
    fn scroll_to_vertical(&self, location: f64) {
        let adj = self.view.vadjustment();
        adj.set_upper(1.0);
        adj.set_lower(0.0);
        adj.set_value(location);
    }
    fn horizontal_scroll_location(&self) -> f64 {
        self.view.hadjustment().value()
    }
    fn vertical_scroll_location(&self) -> f64 {
        self.view.vadjustment().value()
    }
    fn set_horizontal_scrollable(&self, scrollable: bool) {
        if !scrollable{
            self.view.set_hscrollbar_policy(gtk4::PolicyType::Never)
        } else{
            self.view.set_vscrollbar_policy(gtk4::PolicyType::Automatic)
        }
    }
    fn set_vertical_scrollable(&self, scrollable: bool) {
        if !scrollable{
            self.view.set_vscrollbar_policy(gtk4::PolicyType::Never)
        } else{
            self.view.set_vscrollbar_policy(gtk4::PolicyType::Automatic)
        }
    }
}