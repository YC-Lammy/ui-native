use gtk4::prelude::*;

use crate::native_tree::NativeViewImp;

use super::NativeElement;

pub struct NativeView {
    fixed: gtk4::Fixed,
}

impl NativeViewImp for NativeView {
    fn new() -> Self {
        Self {
            fixed: gtk4::Fixed::new(),
        }
    }

    fn set_width(&self, width: f32) {
        self.fixed.set_width_request(width as _);
    }

    fn set_height(&self, height: f32) {
        self.fixed.set_height_request(height as _);
    }

    fn set_visible(&self, visible: bool) {
        self.fixed.set_visible(visible)
    }

    fn insert_child(&self, index: usize, child: &dyn NativeElement) {
        let children = self.fixed.observe_children();
        if let Some(sibling) = children.item(index as u32) {
            let sibling = sibling.dynamic_cast_ref::<gtk4::Widget>().unwrap();

            child
                .as_gtk4_widget()
                .insert_before(&self.fixed, Some(sibling));
            self.fixed.remove(sibling);
        } else {
            self.fixed.put(child.as_gtk4_widget(), 0.0, 0.0);
        }
    }

    fn set_child_position(&self, child: &dyn NativeElement, x: f32, y: f32) {
        self.fixed.move_(child.as_gtk4_widget(), x as _, y as _);
    }

    fn remove_child(&self, child: &dyn NativeElement) {
        self.fixed.remove(child.as_gtk4_widget());
    }
}

impl NativeElement for NativeView {
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.fixed.as_ref()
    }
}
