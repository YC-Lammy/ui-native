use std::sync::Arc;

use gtk4::prelude::*;
use parking_lot::RwLock;

use crate::native_tree::context::Context;
use crate::native_tree::AvalableSpace;
use crate::native_tree::{
    traits::NativeButtonImp, MeasuredSize, NativeStyledElement, NativeTextImp,
};
use crate::style::{
    BorderStyle, Colour, FontStyle, FontWeight, PointEvents, TextAlign, TextDecorationLine,
    TextTransform,
};

use super::{NativeElement, NativeText};

pub struct NativeButton {
    label: NativeText,
    button: gtk4::Button,
    callback: Arc<RwLock<Option<Arc<dyn Fn() + Send + Sync>>>>,
}

impl NativeButton {}

impl NativeButtonImp for NativeButton {
    fn new(_ctx: &mut Context) -> Self {
        let label = NativeText::new(_ctx, "");

        let button_callback: Arc<RwLock<Option<Arc<dyn Fn() + Send + Sync>>>> =
            Arc::new(RwLock::new(None));
        let button_callback_cloned = button_callback.clone();

        let button = gtk4::Button::new();

        button.connect_clicked(move |_b| {
            let r = button_callback_cloned.read();

            if let Some(cb) = r.as_ref() {
                cb();
            }
        });

        button.set_child(Some(label.as_gtk4_widget()));

        Self {
            label,
            button: button,
            callback: button_callback,
        }
    }

    fn set_disabled(&self, _ctx: &mut Context, disabled: bool) {
        self.button.set_can_target(!disabled)
    }

    fn set_label(&self, _ctx: &mut Context, text: String) {
        self.label.set_text(_ctx, &text)
    }

    fn set_on_click(&self, _ctx: &mut Context, on_click: Option<Arc<dyn Fn() + Send + Sync>>) {
        *self.callback.write() = on_click;
    }
}

impl super::NativeElement for NativeButton {
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.button.as_ref()
    }
}

impl NativeStyledElement for NativeButton {
    fn measure(
        &self,
        _ctx: &mut Context,
        known_width: AvalableSpace,
        known_height: AvalableSpace,
    ) -> anyhow::Result<MeasuredSize> {
        // measure width
        let (min_width, natural_width, _, _) = self.button.measure(
            gtk4::Orientation::Horizontal,
            match known_height {
                AvalableSpace::AtMost(f) => f as i32,
                AvalableSpace::Exact(f) => f as i32,
                AvalableSpace::Unknown => -1,
            },
        );
        // measure height
        let (min_height, natural_height, _, _) = self.button.measure(
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
        self.button.set_visible(visible)
    }
    fn set_backface_visible(&self, _visible: bool) {}

    fn set_colour(&self, colour: Colour) {
        self.label.set_colour(colour)
    }
    fn set_background_colour(&self, colour: Colour) {
        self.label.set_background_colour(colour)
    }

    fn set_border_top_width(&self, width: f32) {
        self.button.style_context().border().set_top(width as i16);
    }
    fn set_border_bottom_width(&self, width: f32) {
        self.button
            .style_context()
            .border()
            .set_bottom(width as i16);
    }
    fn set_border_left_width(&self, width: f32) {
        self.button.style_context().border().set_left(width as i16);
    }
    fn set_border_right_width(&self, width: f32) {
        self.button.style_context().border().set_right(width as i16);
    }

    fn set_border_top_left_radius(&self, _radius: f32) {}
    fn set_border_top_right_radius(&self, _radius: f32) {}
    fn set_border_bottom_left_radius(&self, _radius: f32) {}
    fn set_border_bottom_right_radius(&self, _radius: f32) {}

    fn set_border_top_colour(&self, _colour: Colour) {}
    fn set_border_bottom_colour(&self, _colour: Colour) {}
    fn set_border_left_colour(&self, _colour: Colour) {}
    fn set_border_right_colour(&self, _colour: Colour) {}

    fn set_border_style(&self, _style: BorderStyle) {}

    fn set_opacity(&self, opacity: f32) {
        self.button.set_opacity(opacity as f64);
    }
    fn set_points_event(&self, _event: PointEvents) {}

    fn set_font_size(&self, size: f32) {
        self.label.set_font_size(size)
    }
    fn set_font_style(&self, style: FontStyle) {
        self.label.set_font_style(style)
    }
    fn set_font_weight(&self, weight: FontWeight) {
        self.label.set_font_weight(weight)
    }

    fn set_letter_spacing(&self, spacing: f32) {
        self.label.set_letter_spacing(spacing)
    }
    fn set_line_height(&self, height: f32) {
        self.label.set_line_height(height)
    }

    fn set_text_align(&self, align: TextAlign) {
        self.label.set_text_align(align)
    }
    fn set_text_decloration_line(&self, line: TextDecorationLine) {
        self.label.set_text_decloration_line(line)
    }
    fn set_text_decloration_colour(&self, colour: Colour) {
        self.label.set_text_decloration_colour(colour)
    }
    fn set_text_shadow_colour(&self, colour: Colour) {
        self.label.set_text_shadow_colour(colour)
    }
    fn set_text_shadow_radius(&self, radius: f32) {
        self.label.set_text_shadow_radius(radius)
    }
    fn set_text_transform(&self, transform: TextTransform) {
        self.label.set_text_transform(transform)
    }
}
