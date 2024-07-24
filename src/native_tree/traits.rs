use std::sync::Arc;

use crate::style::Colour;

use crate::imp::NativeElement;
use crate::widget::flatlist::{ListViewDataSourceWrapper, ListViewWidgetFactoryWrapper};

pub trait NativeButtonImp: NativeElement {
    fn new() -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, height: f32);
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn set_visible(&self, visible: bool);
    fn set_disabled(&self, disabled: bool);
    fn set_label(&self, text: String);
    fn set_on_click(&self, on_click: Option<Arc<dyn Fn() + Send + Sync>>);
}

pub trait NativeViewImp: NativeElement {
    fn new() -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, height: f32);
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn set_visible(&self, visible: bool);
    fn insert_child(&self, index: usize, elem: &dyn NativeElement);
    fn set_child_position(&self, child: &dyn NativeElement, x: f32, y: f32);
    fn remove_child(&self, elem: &dyn NativeElement);
}

pub trait NativeImageViewImp: NativeElement {
    fn new() -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, height: f32);
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn set_visible(&self, visible: bool);
    fn set_source(&self);
}

pub trait NativeScrollViewImp: NativeElement{
    fn new() -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, heigth: f32);
    fn set_child(&self, child: &dyn NativeElement);
    fn remove_child(&self);
    fn scroll_to_horizontal(&self, location: f64);
    fn scroll_to_vertical(&self, location: f64);
    fn set_horizontal_scrollable(&self, scrollable: bool);
    fn set_vertical_scrollable(&self, scrollable: bool);
    fn horizontal_scroll_location(&self) -> f64;
    fn vertical_scroll_location(&self) -> f64;
}

pub trait NativeTextImp: NativeElement {
    fn new(text: &str) -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, height: f32);
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn set_visible(&self, visible: bool);
    fn set_text(&self, text: &str);
    fn set_colour(&self, colour: Colour);
    fn set_background_colour(&self, colour: Colour);
    fn set_font(&self, font: &str);
    fn set_underline(&self, underline: bool);
    fn set_underline_colour(&self, colour: Colour);
    fn set_overline(&self, overline: bool);
    fn set_overline_colour(&self, colour: Colour);
}

pub trait NativeTextInputImp: NativeElement{
    fn new() -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, height: f32);
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn set_background_text(&self, text: &str);
    fn set_on_text_changed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>);
    fn set_on_enter_pressed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>);
}

pub trait NativeTextEditImp: NativeElement{
    fn new() -> Self;
    fn set_width(&self, width: f32);
    fn set_height(&self, height: f32);
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn set_on_text_changed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>);
}

pub trait NativeListViewImp: NativeElement{
    fn new(
        data: ListViewDataSourceWrapper,
        render: ListViewWidgetFactoryWrapper
    ) -> Self;
}