use std::sync::Arc;

use crossbeam_channel::Receiver;

use crate::image::ImageSource;
use crate::shadow_tree::component::NavigatorCommand;
use crate::shadow_tree::NodeID;
use crate::style::{
    BorderStyle, Colour, FontStyle, FontWeight, PointEvents, TextAlign, TextDecorationLine,
    TextTransform,
};

use crate::imp::NativeElement;
use crate::util::Comparable;
use crate::widget::flatlist::{ListViewDataSourceWrapper, ListViewWidgetFactoryWrapper};

use super::context::Context;

pub trait NativeButtonImp: NativeElement + NativeStyledElement {
    fn new(ctx: &mut Context) -> Self;
    fn set_disabled(&self, ctx: &mut Context, disabled: bool);
    fn set_label(&self, ctx: &mut Context, text: String);
    fn set_on_click(&self, ctx: &mut Context, on_click: Option<Arc<dyn Fn() + Send + Sync>>);
}

pub trait NativeViewImp: NativeElement + NativeStyledElement {
    fn new(ctx: &mut Context) -> Self;
    fn insert_child(&self, ctx: &mut Context, index: usize, elem: &dyn NativeElement);
    fn remove_child(&self, ctx: &mut Context, elem: &dyn NativeElement);
    fn layout_child(
        &self,
        ctx: &mut Context,
        child: &dyn NativeElement,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    );
}

pub trait NativeImageViewImp: NativeElement + NativeStyledElement {
    fn new(ctx: &mut Context, src: Arc<Comparable<dyn ImageSource>>) -> Self;
    fn set_source(&self, ctx: &mut Context, src: Arc<Comparable<dyn ImageSource>>);
    fn check_update(&self);
}

pub trait NativeScrollViewImp: NativeElement + NativeStyledElement {
    fn new(ctx: &mut Context) -> Self;
    fn set_child(&self, ctx: &mut Context, child: &dyn NativeElement);
    fn remove_child(&self, ctx: &mut Context);
    fn layout_child(
        &self,
        ctx: &mut Context,
        child: &dyn NativeElement,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    );
    fn scroll_to_horizontal(&self, ctx: &mut Context, location: f64);
    fn scroll_to_vertical(&self, ctx: &mut Context, location: f64);
    fn set_horizontal_scrollable(&self, ctx: &mut Context, scrollable: bool);
    fn set_vertical_scrollable(&self, ctx: &mut Context, scrollable: bool);
    fn horizontal_scroll_location(&self, ctx: &mut Context) -> f64;
    fn vertical_scroll_location(&self, ctx: &mut Context) -> f64;
}

pub trait NativeTextImp: NativeElement + NativeStyledElement {
    fn new(ctx: &mut Context, text: &str) -> Self;
    fn set_text(&self, ctx: &mut Context, text: &str);
}

pub trait NativeTextInputImp: NativeElement + NativeStyledElement {
    fn new(ctx: &mut Context) -> Self;
    fn set_background_text(&self, ctx: &mut Context, text: &str);
    fn set_on_text_changed(&self, ctx: &mut Context, callback: Arc<dyn Fn(&str) + Sync + Send>);
    fn set_on_enter_pressed(&self, ctx: &mut Context, callback: Arc<dyn Fn(&str) + Sync + Send>);
}

pub trait NativeTextEditImp: NativeElement + NativeStyledElement {
    fn new(ctx: &mut Context) -> Self;
    fn set_on_text_changed(&self, ctx: &mut Context, callback: Arc<dyn Fn(&str) + Sync + Send>);
}

pub trait NativeListViewImp: NativeElement + NativeStyledElement {
    fn new(
        ctx: &mut Context,
        data: Arc<ListViewDataSourceWrapper>,
        render: Arc<ListViewWidgetFactoryWrapper>,
    ) -> Self;
}

pub trait NativeStackNavigatorImp: NativeElement + NativeStyledElement {
    fn new(ctx: &mut Context, command_recv: Receiver<NavigatorCommand>) -> Self;
    fn add_child(&self, ctx: &mut Context, child: &dyn NativeElement, name: &str, id: NodeID);
    fn remove_child(&self, ctx: &mut Context, name: &str);
    fn visible_child(&self, ctx: &mut Context) -> Option<NodeID>;
    fn layout_child(
        &self,
        ctx: &mut Context,
        child: &dyn NativeElement,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    );
    fn should_retain(&self, ctx: &mut Context) -> bool;
}

pub trait NativeNavigatorImp: NativeElement + NativeStyledElement {
    fn new(ctx: &mut Context, command_recv: Receiver<NavigatorCommand>) -> Self;
    fn add_child(&self, ctx: &mut Context, child: &dyn NativeElement, name: &str, id: NodeID);
    fn remove_child(&self, ctx: &mut Context, name: &str);
    fn visible_child(&self, ctx: &mut Context) -> Option<NodeID>;
    fn layout_child(
        &self,
        ctx: &mut Context,
        child: &dyn NativeElement,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    );
    fn should_retain(&self, ctx: &mut Context) -> bool;
}

pub enum AvalableSpace {
    Exact(f32),
    AtMost(f32),
    Unknown,
}

pub struct MeasuredSize {
    pub min_width: f32,
    pub natural_width: f32,
    pub min_height: f32,
    pub natural_height: f32,
}

pub trait NativeStyledElement {
    /// measures the natural and minimum width
    fn measure(
        &self,
        ctx: &mut Context,
        known_width: AvalableSpace,
        known_height: AvalableSpace,
    ) -> anyhow::Result<MeasuredSize>;

    fn set_visible(&self, ctx: &mut Context, visible: bool);
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
