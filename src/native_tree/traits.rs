use std::sync::Arc;

use crate::style::Colour;

use crate::imp::NativeElement;

pub trait NativeButtonImp: NativeElement{
    fn new() -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, height: f32);
    fn set_visible(&self, visible: bool);
    fn set_disabled(&self, disabled: bool);
    fn set_label(&self, text: String);
    fn set_on_click(&self, on_click: Option<Arc<dyn Fn() + Send + Sync>>);
}

pub trait NativeViewImp: NativeElement{
    fn new() -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, height: f32);
    fn set_visible(&self, visible: bool);
    fn insert_child(&self, index: usize, elem: &dyn NativeElement);
    fn set_child_position(&self, child: &dyn NativeElement, x: f32, y: f32);
    fn remove_child(&self, elem: &dyn NativeElement);
}

pub trait NativeImageViewImp: NativeElement{
    fn new() -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, height: f32);
    fn set_visible(&self, visible: bool);
    fn set_source(&self);
}

pub trait NativeTextImp: NativeElement{
    fn new(text: &str) -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, height: f32);
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn set_text(&self, text: &str);
    fn set_colour(&self, colour: Colour);
    fn set_background_colour(&self, colour: Colour);
    fn set_font(&self, font: &str);
    fn set_underline(&self, underline: bool);
    fn set_underline_colour(&self, colour: Colour);
    fn set_overline(&self, overline: bool);
    fn set_overline_colour(&self, colour: Colour);
}