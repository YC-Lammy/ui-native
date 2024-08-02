use gtk4::prelude::*;

use crate::native_tree::{MeasuredSize, NativeImageViewImp, NativeStyledElement};
use crate::style::{
    BorderStyle, Colour, FontStyle, FontWeight, PointEvents, TextAlign, TextDecorationLine,
    TextTransform,
};

use super::NativeElement;

pub struct NativeImageView {
    view: gtk4::Picture,
}

impl NativeElement for NativeImageView {
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.view.as_ref()
    }
}

impl NativeImageViewImp for NativeImageView {
    fn new() -> Self {
        Self {
            view: gtk4::Picture::new(),
        }
    }
    fn set_source(&self) {}
}

impl NativeStyledElement for NativeImageView {
    fn measure(&self, known_width: Option<f32>, known_height: Option<f32>) -> MeasuredSize {
        // measure width
        let (min_width, natural_width, _, _) = self.view.measure(
            gtk4::Orientation::Horizontal,
            known_height.map(|i| i as i32).unwrap_or(-1),
        );
        // measure height
        let (min_height, natural_height, _, _) = self.view.measure(
            gtk4::Orientation::Vertical,
            known_width.map(|i| i as i32).unwrap_or(-1),
        );

        return MeasuredSize {
            min_width: min_width as f32,
            natural_width: natural_width as f32,
            min_height: min_height as f32,
            natural_height: natural_height as f32,
        };
    }
    fn set_visible(&self, visible: bool) {
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
