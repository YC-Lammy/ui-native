use gtk4::prelude::*;

use crate::native_tree::context::Context;
use crate::native_tree::{AvalableSpace, MeasuredSize, NativeScrollViewImp, NativeStyledElement};
use crate::style::{
    BorderStyle, Colour, FontStyle, FontWeight, PointEvents, TextAlign, TextDecorationLine,
    TextTransform,
};

use super::NativeElement;

pub struct NativeScrollView {
    view: gtk4::ScrolledWindow,
}

impl NativeElement for NativeScrollView {
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.view.as_ref()
    }
}

impl NativeScrollViewImp for NativeScrollView {
    fn new(_ctx: &mut Context) -> Self {
        let scroll = gtk4::ScrolledWindow::new();
        Self { view: scroll }
    }
    fn set_child(&self, _ctx: &mut Context, child: &dyn NativeElement) {
        self.view.set_child(Some(child.as_gtk4_widget()))
    }
    fn remove_child(&self, _ctx: &mut Context) {
        self.view.set_child(None::<&gtk4::Widget>);
    }
    fn layout_child(
        &self,
        _ctx: &mut Context,
        child: &dyn NativeElement,
        _x: f32,
        _y: f32,
        width: f32,
        height: f32,
    ) {
        let w = child.as_gtk4_widget();
        w.set_width_request(width as i32);
        w.set_height_request(height as i32);
    }
    fn scroll_to_horizontal(&self, _ctx: &mut Context, location: f64) {
        let adj = self.view.hadjustment();
        adj.set_upper(1.0);
        adj.set_lower(0.0);
        adj.set_value(location);
    }
    fn scroll_to_vertical(&self, _ctx: &mut Context, location: f64) {
        let adj = self.view.vadjustment();
        adj.set_upper(1.0);
        adj.set_lower(0.0);
        adj.set_value(location);
    }
    fn horizontal_scroll_location(&self, _ctx: &mut Context) -> f64 {
        self.view.hadjustment().value()
    }
    fn vertical_scroll_location(&self, _ctx: &mut Context) -> f64 {
        self.view.vadjustment().value()
    }
    fn set_horizontal_scrollable(&self, _ctx: &mut Context, scrollable: bool) {
        if !scrollable {
            self.view.set_hscrollbar_policy(gtk4::PolicyType::Never)
        } else {
            self.view.set_vscrollbar_policy(gtk4::PolicyType::Automatic)
        }
    }
    fn set_vertical_scrollable(&self, _ctx: &mut Context, scrollable: bool) {
        if !scrollable {
            self.view.set_vscrollbar_policy(gtk4::PolicyType::Never)
        } else {
            self.view.set_vscrollbar_policy(gtk4::PolicyType::Automatic)
        }
    }
}

impl NativeStyledElement for NativeScrollView {
    fn measure(
        &self,
        _ctx: &mut Context,
        known_width: AvalableSpace,
        known_height: AvalableSpace,
    ) -> anyhow::Result<MeasuredSize> {
        // measure width
        let (min_width, natural_width, _, _) = self.view.measure(
            gtk4::Orientation::Horizontal,
            match known_height {
                AvalableSpace::AtMost(f) => f as i32,
                AvalableSpace::Exact(f) => f as i32,
                AvalableSpace::Unknown => -1,
            },
        );
        // measure height
        let (min_height, natural_height, _, _) = self.view.measure(
            gtk4::Orientation::Vertical,
            match known_width {
                AvalableSpace::AtMost(f) => f as i32,
                AvalableSpace::Exact(f) => f as i32,
                AvalableSpace::Unknown => -1,
            },
        );

        return Ok(MeasuredSize {
            min_width: min_width as f32,
            natural_width: natural_width as f32,
            min_height: min_height as f32,
            natural_height: natural_height as f32,
        });
    }
    fn set_visible(&self, _ctx: &mut Context, visible: bool) {
        self.view.set_visible(visible)
    }
    fn set_backface_visible(&self, _visible: bool) {}

    fn set_colour(&self, _colour: Colour) {}
    fn set_background_colour(&self, _colour: Colour) {}

    fn set_border_top_width(&self, _width: f32) {}
    fn set_border_bottom_width(&self, _width: f32) {}
    fn set_border_left_width(&self, _width: f32) {}
    fn set_border_right_width(&self, _width: f32) {}

    fn set_border_top_left_radius(&self, _radius: f32) {}
    fn set_border_top_right_radius(&self, _radius: f32) {}
    fn set_border_bottom_left_radius(&self, _radius: f32) {}
    fn set_border_bottom_right_radius(&self, _radius: f32) {}

    fn set_border_top_colour(&self, _colour: Colour) {}
    fn set_border_bottom_colour(&self, _colour: Colour) {}
    fn set_border_left_colour(&self, _colour: Colour) {}
    fn set_border_right_colour(&self, _colour: Colour) {}

    fn set_border_style(&self, _style: BorderStyle) {}

    fn set_opacity(&self, _opacity: f32) {}
    fn set_points_event(&self, _event: PointEvents) {}

    fn set_font_size(&self, _size: f32) {}
    fn set_font_style(&self, _style: FontStyle) {}
    fn set_font_weight(&self, _weight: FontWeight) {}

    fn set_letter_spacing(&self, _spacing: f32) {}
    fn set_line_height(&self, _height: f32) {}

    fn set_text_align(&self, _align: TextAlign) {}
    fn set_text_decloration_line(&self, _line: TextDecorationLine) {}
    fn set_text_decloration_colour(&self, _colour: Colour) {}
    fn set_text_shadow_colour(&self, _colour: Colour) {}
    fn set_text_shadow_radius(&self, _radius: f32) {}
    fn set_text_transform(&self, _transform: TextTransform) {}
}
