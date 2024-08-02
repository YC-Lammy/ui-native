use std::sync::Arc;

use gtk4::prelude::*;
use parking_lot::RwLock;

use crate::native_tree::{MeasuredSize, NativeStyledElement, NativeTextInputImp};
use crate::style::{
    BorderStyle, Colour, FontStyle, FontWeight, PointEvents, TextAlign, TextDecorationLine,
    TextTransform,
};

use super::NativeElement;

pub struct NativeTextInput {
    input: gtk4::Entry,
    text_changed_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>>,
    enter_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>>,
}

impl NativeTextInput {
    fn get_attr(&self) -> gtk4::pango::AttrList {
        match self.input.attributes() {
            Some(attr) => attr,
            None => {
                let attr = gtk4::pango::AttrList::new();
                self.input.set_attributes(&attr);
                return attr;
            }
        }
    }
}

impl NativeElement for NativeTextInput {
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.input.as_ref()
    }
}

impl NativeTextInputImp for NativeTextInput {
    fn new() -> Self {
        let entry = gtk4::Entry::new();
        let text_changed_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>> =
            Arc::new(RwLock::new(None));
        let cb = text_changed_callback.clone();

        entry.connect_changed(move |entry| {
            let lock = cb.read();

            if let Some(cb) = lock.as_ref() {
                (cb)(entry.text().as_str())
            }
        });

        let enter_callback: Arc<RwLock<Option<Arc<dyn Fn(&str) + Sync + Send>>>> =
            Arc::new(RwLock::new(None));
        let cb = enter_callback.clone();

        entry.connect_activate(move |entry| {
            let lock = cb.read();

            if let Some(cb) = lock.as_ref() {
                (cb)(entry.text().as_str())
            }
        });

        Self {
            input: entry,
            text_changed_callback,
            enter_callback,
        }
    }

    fn set_background_text(&self, text: &str) {
        self.input.set_placeholder_text(Some(text))
    }
    fn set_on_text_changed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>) {
        *self.text_changed_callback.write() = Some(callback)
    }
    fn set_on_enter_pressed(&self, callback: Arc<dyn Fn(&str) + Sync + Send>) {
        *self.enter_callback.write() = Some(callback)
    }
}

impl NativeStyledElement for NativeTextInput {
    fn measure(&self, known_width: Option<f32>, known_height: Option<f32>) -> MeasuredSize {
        // measure width
        let (min_width, natural_width, _, _) = self.input.measure(
            gtk4::Orientation::Horizontal,
            known_height.map(|i| i as i32).unwrap_or(-1),
        );
        // measure height
        let (min_height, natural_height, _, _) = self.input.measure(
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
        self.input.set_visible(visible)
    }
    fn set_backface_visible(&self, _visible: bool) {}

    fn set_colour(&self, colour: Colour) {
        let attr = self.get_attr();
        attr.change(pango::AttrColor::new_foreground(
            colour.r as u16,
            colour.g as u16,
            colour.b as u16,
        ));
        attr.change(pango::AttrInt::new_foreground_alpha(colour.a as u16))
    }
    fn set_background_colour(&self, colour: Colour) {
        let attr = self.get_attr();
        attr.change(pango::AttrColor::new_background(
            colour.r as u16,
            colour.g as u16,
            colour.b as u16,
        ));
        attr.change(pango::AttrInt::new_background_alpha(colour.a as u16))
    }

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

    fn set_opacity(&self, opacity: f32) {
        self.input.set_opacity(opacity as f64)
    }
    fn set_points_event(&self, _event: PointEvents) {}

    fn set_font_size(&self, size: f32) {
        let attr = self.get_attr();
        attr.change(pango::AttrSize::new(size as i32));
    }
    fn set_font_style(&self, style: FontStyle) {
        let attr = self.get_attr();
        match style {
            FontStyle::Normal => attr.change(pango::AttrInt::new_style(pango::Style::Normal)),
            FontStyle::Italic => attr.change(pango::AttrInt::new_style(pango::Style::Italic)),
        }
    }
    fn set_font_weight(&self, weight: FontWeight) {
        let attr = self.get_attr();

        let w = match weight {
            FontWeight::Normal => pango::Weight::Normal,
            FontWeight::Bold => pango::Weight::Bold,
            FontWeight::Number(n) => {
                let n = n as i32;
                match n {
                    // 100
                    i32::MIN..150 => pango::Weight::Thin,
                    // 200
                    150..250 => pango::Weight::Ultralight,
                    // 300
                    250..325 => pango::Weight::Light,
                    // 350
                    325..365 => pango::Weight::Semilight,
                    // 380
                    365..390 => pango::Weight::Book,
                    // 400
                    390..450 => pango::Weight::Normal,
                    // 500
                    450..550 => pango::Weight::Medium,
                    // 600
                    550..650 => pango::Weight::Semibold,
                    // 700
                    650..750 => pango::Weight::Bold,
                    // 800
                    750..850 => pango::Weight::Ultrabold,
                    // 900
                    850..950 => pango::Weight::Heavy,
                    950.. => pango::Weight::Ultraheavy,
                }
            }
        };

        attr.change(pango::AttrInt::new_weight(w));
    }

    fn set_letter_spacing(&self, spacing: f32) {
        let attr = self.get_attr();
        attr.change(pango::AttrInt::new_letter_spacing(spacing as i32));
    }
    fn set_line_height(&self, height: f32) {
        let attr = self.get_attr();
        attr.change(pango::AttrInt::new_line_height_absolute(height as i32))
    }
    fn set_text_align(&self, align: TextAlign) {
        let attr = self.get_attr();
        attr.change(pango::AttrInt::new_gravity(match align {
            TextAlign::Auto => pango::Gravity::Auto,
            TextAlign::Centre => pango::Gravity::Auto,
            TextAlign::Justified => pango::Gravity::Auto,
            TextAlign::Left => pango::Gravity::West,
            TextAlign::Right => pango::Gravity::East,
        }))
    }
    fn set_text_decloration_line(&self, line: TextDecorationLine) {
        let attr = self.get_attr();
        match line {
            TextDecorationLine::None => {
                attr.change(pango::AttrInt::new_overline(pango::Overline::None));
                attr.change(pango::AttrInt::new_underline(pango::Underline::None));
                attr.change(pango::AttrInt::new_strikethrough(false));
            }
            TextDecorationLine::LineThrough => {
                attr.change(pango::AttrInt::new_overline(pango::Overline::None));
                attr.change(pango::AttrInt::new_underline(pango::Underline::None));
                attr.change(pango::AttrInt::new_strikethrough(true));
            }
            TextDecorationLine::Underline => {
                attr.change(pango::AttrInt::new_overline(pango::Overline::None));
                attr.change(pango::AttrInt::new_underline(pango::Underline::Single));
                attr.change(pango::AttrInt::new_strikethrough(false));
            }
            TextDecorationLine::UnderlineLineThrough => {
                attr.change(pango::AttrInt::new_overline(pango::Overline::None));
                attr.change(pango::AttrInt::new_underline(pango::Underline::Single));
                attr.change(pango::AttrInt::new_strikethrough(true));
            }
            TextDecorationLine::Overline => {
                attr.change(pango::AttrInt::new_overline(pango::Overline::Single));
                attr.change(pango::AttrInt::new_underline(pango::Underline::None));
                attr.change(pango::AttrInt::new_strikethrough(false));
            }
        }
    }
    fn set_text_decloration_colour(&self, colour: Colour) {
        let attr = self.get_attr();
        attr.change(pango::AttrColor::new_overline_color(
            colour.r as u16,
            colour.g as u16,
            colour.b as u16,
        ));
        attr.change(pango::AttrColor::new_underline_color(
            colour.r as u16,
            colour.g as u16,
            colour.b as u16,
        ));
        attr.change(pango::AttrColor::new_strikethrough_color(
            colour.r as u16,
            colour.g as u16,
            colour.b as u16,
        ));
    }
    fn set_text_shadow_colour(&self, _colour: Colour) {}
    fn set_text_shadow_radius(&self, _radius: f32) {}
    fn set_text_transform(&self, transform: TextTransform) {
        let attr = self.get_attr();
        attr.change(pango::AttrInt::new_text_transform(match transform {
            TextTransform::Capitalise => pango::TextTransform::Capitalize,
            TextTransform::Lowercase => pango::TextTransform::Lowercase,
            TextTransform::Uppercase => pango::TextTransform::Uppercase,
            TextTransform::None => pango::TextTransform::None,
        }));
    }
}
