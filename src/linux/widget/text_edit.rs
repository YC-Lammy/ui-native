use std::sync::Arc;

use gtk4::prelude::*;
use parking_lot::RwLock;

use crate::native_tree::context::Context;
use crate::native_tree::{AvalableSpace, MeasuredSize, NativeStyledElement, NativeTextEditImp};
use crate::style::{
    BorderStyle, Colour, FontStyle, FontWeight, PointEvents, TextAlign, TextDecorationLine,
    TextTransform,
};

use super::NativeElement;

pub struct NativeTextEdit {
    edit: gtk4::TextView,
    tag: gtk4::TextTag,
    scroll: gtk4::ScrolledWindow,
    text_changed_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>>,
}

impl NativeElement for NativeTextEdit {
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.scroll.as_ref()
    }
}

impl NativeTextEditImp for NativeTextEdit {
    fn new(_ctx: &mut Context) -> Self {
        let buffer = gtk4::TextBuffer::new(None);
        let tag = buffer.create_tag(Some("default_style"), &[]).unwrap();
        buffer.apply_tag(&tag, &buffer.start_iter(), &buffer.end_iter());

        let view = gtk4::TextView::with_buffer(&buffer);
        let text_changed_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>> =
            Arc::new(RwLock::new(None));
        let cb = text_changed_callback.clone();

        view.connect_buffer_notify(move |view| {
            let lock = cb.read();

            if let Some(cb) = lock.as_ref() {
                let buffer = view.buffer();
                let start = buffer.start_iter();
                let end = buffer.end_iter();
                (cb)(buffer.slice(&start, &end, false).as_str())
            }
        });

        view.set_height_request(100);
        view.set_width_request(100);
        view.set_hexpand(true);
        view.set_vexpand(true);

        let scroll = gtk4::ScrolledWindow::new();
        scroll.set_child(Some(&view));

        Self {
            edit: view,
            tag: tag,
            scroll: scroll,
            text_changed_callback,
        }
    }
    fn set_on_text_changed(&self, _ctx: &mut Context, callback: Arc<dyn Fn(&str) + Sync + Send>) {
        *self.text_changed_callback.write() = Some(callback)
    }
}

impl NativeStyledElement for NativeTextEdit {
    fn measure(
        &self,
        _ctx: &mut Context,
        known_width: AvalableSpace,
        known_height: AvalableSpace,
    ) -> anyhow::Result<MeasuredSize> {
        // measure width
        let (min_width, natural_width, _, _) = self.edit.measure(
            gtk4::Orientation::Horizontal,
            match known_height {
                AvalableSpace::AtMost(f) => f as i32,
                AvalableSpace::Exact(f) => f as i32,
                AvalableSpace::Unknown => -1,
            },
        );
        // measure height
        let (min_height, natural_height, _, _) = self.edit.measure(
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
        self.edit.set_visible(visible)
    }
    fn set_backface_visible(&self, _visible: bool) {}

    fn set_colour(&self, colour: Colour) {
        self.tag.set_foreground_rgba(Some(&gtk4::gdk::RGBA::new(
            (colour.r as f32) / 255.0,
            (colour.g as f32) / 255.0,
            (colour.b as f32) / 255.0,
            (colour.a as f32) / 255.0,
        )));
    }
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
