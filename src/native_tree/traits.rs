use std::sync::Arc;

use crossbeam_channel::Receiver;

use crate::shadow_tree::component::NavigatorCommand;
use crate::shadow_tree::NodeID;
use crate::style::{
    BorderStyle, Colour, FontStyle, FontWeight, PointEvents, TextAlign, TextDecorationLine,
    TextTransform,
};

use crate::imp::NativeElement;
use crate::widget::flatlist::{ListViewDataSourceWrapper, ListViewWidgetFactoryWrapper};

pub trait NativeButtonImp: NativeElement + NativeStyledElement {
    fn new() -> Self;
    fn set_disabled(&self, disabled: bool);
    fn set_label(&self, text: String);
    fn set_on_click(&self, on_click: Option<Arc<dyn Fn() + Send + Sync>>);
}

pub trait NativeViewImp: NativeElement + NativeStyledElement {
    fn new() -> Self;
    fn insert_child(&self, index: usize, elem: &dyn NativeElement);
    fn remove_child(&self, elem: &dyn NativeElement);
    fn layout_child(&self, child: &dyn NativeElement, x: f32, y: f32, width: f32, height: f32);
}

pub trait NativeImageViewImp: NativeElement + NativeStyledElement {
    fn new() -> Self;
    fn set_source(&self);
}

pub trait NativeScrollViewImp: NativeElement + NativeStyledElement {
    fn new() -> Self;
    fn set_child(&self, child: &dyn NativeElement);
    fn remove_child(&self);
    fn layout_child(&self, child: &dyn NativeElement, x: f32, y: f32, width: f32, height: f32);
    fn scroll_to_horizontal(&self, location: f64);
    fn scroll_to_vertical(&self, location: f64);
    fn set_horizontal_scrollable(&self, scrollable: bool);
    fn set_vertical_scrollable(&self, scrollable: bool);
    fn horizontal_scroll_location(&self) -> f64;
    fn vertical_scroll_location(&self) -> f64;
}

pub trait NativeTextImp: NativeElement + NativeStyledElement {
    fn new(text: &str) -> Self;
    fn set_text(&self, text: &str);
}

pub trait NativeTextInputImp: NativeElement + NativeStyledElement {
    fn new() -> Self;
    fn set_background_text(&self, text: &str);
    fn set_on_text_changed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>);
    fn set_on_enter_pressed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>);
}

pub trait NativeTextEditImp: NativeElement + NativeStyledElement {
    fn new() -> Self;
    fn set_on_text_changed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>);
}

pub trait NativeListViewImp: NativeElement + NativeStyledElement {
    fn new(data: Arc<ListViewDataSourceWrapper>, render: Arc<ListViewWidgetFactoryWrapper>)
        -> Self;
}

pub trait NativeStackNavigatorImp: NativeElement + NativeStyledElement {
    fn new(command_recv: Receiver<NavigatorCommand>) -> Self;
    fn add_child(&self, child: &dyn NativeElement, name: &str, id: NodeID);
    fn remove_child(&self, name: &str);
    fn visible_child(&self) -> Option<NodeID>;
    fn layout_child(&self, child: &dyn NativeElement, x: f32, y: f32, width: f32, height: f32);
    fn should_retain(&self) -> bool;
}

pub trait NativeNavigatorImp: NativeElement + NativeStyledElement{
    fn new(command_recv: Receiver<NavigatorCommand>) -> Self;
    fn add_child(&self, child: &dyn NativeElement, name: &str, id: NodeID);
    fn remove_child(&self, name: &str);
    fn visible_child(&self) -> Option<NodeID>;
    fn layout_child(&self, child: &dyn NativeElement, x: f32, y: f32, width: f32, height: f32);
    fn should_retain(&self) -> bool;
}

pub struct MeasuredSize {
    pub min_width: f32,
    pub natural_width: f32,
    pub min_height: f32,
    pub natural_height: f32,
}

pub trait NativeStyledElement {
    /// measures the natural and minimum width
    fn measure(&self, known_width: Option<f32>, known_height: Option<f32>) -> MeasuredSize;

    fn set_visible(&self, visible: bool);
    fn set_backface_visible(&self, visible: bool);

    fn set_colour(&self, colour: Colour);
    fn set_background_colour(&self, colour: Colour);

    fn set_border_top_width(&self, width: f32);
    fn set_border_bottom_width(&self, width: f32);
    fn set_border_left_width(&self, width: f32);
    fn set_border_right_width(&self, width: f32);

    fn set_border_top_left_radius(&self, radius: f32);
    fn set_border_top_right_radius(&self, radius: f32);
    fn set_border_bottom_left_radius(&self, radius: f32);
    fn set_border_bottom_right_radius(&self, radius: f32);

    fn set_border_top_colour(&self, colour: Colour);
    fn set_border_bottom_colour(&self, colour: Colour);
    fn set_border_left_colour(&self, colour: Colour);
    fn set_border_right_colour(&self, colour: Colour);

    fn set_border_style(&self, style: BorderStyle);

    fn set_opacity(&self, opacity: f32);
    fn set_points_event(&self, event: PointEvents);

    fn set_font_size(&self, size: f32);
    fn set_font_style(&self, style: FontStyle);
    fn set_font_weight(&self, weight: FontWeight);

    fn set_letter_spacing(&self, spacing: f32);
    fn set_line_height(&self, height: f32);

    fn set_text_align(&self, align: TextAlign);
    fn set_text_decloration_line(&self, line: TextDecorationLine);
    fn set_text_decloration_colour(&self, colour: Colour);
    fn set_text_shadow_colour(&self, colour: Colour);
    fn set_text_shadow_radius(&self, radius: f32);
    fn set_text_transform(&self, transform: TextTransform);
}
