use gtk4::prelude::*;

use crate::native_tree::{MeasuredSize, NativeStyledElement, NativeViewImp};
use crate::style::{
    BorderStyle, Colour, FontStyle, FontWeight, PointEvents, TextAlign, TextDecorationLine,
    TextTransform,
};

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

    fn remove_child(&self, child: &dyn NativeElement) {
        self.fixed.remove(child.as_gtk4_widget());
    }

    fn layout_child(&self, child: &dyn NativeElement, x: f32, y: f32, width: f32, height: f32) {
        let w = child.as_gtk4_widget();

        self.fixed.move_(w, x as f64, y as f64);
        w.set_width_request(width as i32);
        w.set_height_request(height as i32);
    }
}

impl NativeElement for NativeView {
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.fixed.as_ref()
    }
}

impl NativeStyledElement for NativeView {
    fn measure(&self, known_width: Option<f32>, known_height: Option<f32>) -> MeasuredSize {
        // measure width
        let (min_width, natural_width, _, _) = self.fixed.measure(
            gtk4::Orientation::Horizontal,
            known_height.map(|i| i as i32).unwrap_or(-1),
        );
        // measure height
        let (min_height, natural_height, _, _) = self.fixed.measure(
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
        self.fixed.set_visible(visible)
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
