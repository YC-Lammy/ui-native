use gtk4::pango;
use gtk4::prelude::*;

use crate::native_tree::NativeTextImp;
use crate::style::Colour;

use super::NativeElement;

#[derive(Default)]
pub struct NativeText {
    label: gtk4::Label,
}

impl NativeElement for NativeText {
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.label.as_ref()
    }
}

impl NativeText {
    fn get_attr(&self) -> gtk4::pango::AttrList {
        match self.label.attributes() {
            Some(attr) => attr,
            None => {
                let attr = gtk4::pango::AttrList::new();
                self.label.set_attributes(Some(&attr));
                return attr;
            }
        }
    }
}

impl NativeTextImp for NativeText {
    fn new(s: &str) -> Self {
        Self {
            label: gtk4::Label::new(Some(&s)),
            ..Default::default()
        }
    }

    fn set_text(&self, text: &str) {
        self.label.set_text(text)
    }

    fn set_width(&self, width: f32) {
        self.label.set_width_request(width as _);
    }

    fn set_height(&self, height: f32) {
        self.label.set_height_request(height as _)
    }

    fn get_width(&self) -> f32 {
        self.label.width() as f32
    }

    fn get_height(&self) -> f32 {
        self.label.height() as f32
    }

    fn set_visible(&self, visible: bool) {
        self.label.set_visible(visible)
    }

    fn set_font(&self, font: &str) {
        let attr = self.get_attr();
        attr.change(pango::AttrFontDesc::new(
            &pango::FontDescription::from_string(font),
        ))
    }

    fn set_colour(&self, colour: Colour) {
        let attr = self.get_attr();
        attr.change(pango::AttrColor::new_foreground(
            colour.r as u16,
            colour.g as u16,
            colour.b as u16,
        ));
        // change foreground alpha
        attr.change(pango::AttrInt::new_foreground_alpha(colour.a as u16));
    }

    fn set_background_colour(&self, colour: Colour) {
        let attr = self.get_attr();
        // change background colour
        attr.change(pango::AttrColor::new_background(
            colour.r as u16,
            colour.g as u16,
            colour.b as u16,
        ));
        // change background alpha
        attr.change(pango::AttrInt::new_background_alpha(colour.a as u16));
    }

    fn set_underline(&self, underline: bool) {
        let attr = self.get_attr();

        let underline = if underline {
            pango::Underline::Single
        } else {
            pango::Underline::None
        };

        attr.change(pango::AttrInt::new_underline(underline))
    }

    fn set_underline_colour(&self, colour: Colour) {
        let attr = self.get_attr();
        attr.change(pango::AttrColor::new_underline_color(
            colour.r as u16,
            colour.g as u16,
            colour.b as u16,
        ))
    }

    fn set_overline(&self, overline: bool) {
        let attr = self.get_attr();
        // convert overline
        let overline = if overline {
            pango::Overline::Single
        } else {
            pango::Overline::None
        };
        // set overline
        attr.change(pango::AttrInt::new_overline(overline));
    }

    fn set_overline_colour(&self, colour: Colour) {
        let attr = self.get_attr();
        attr.change(pango::AttrColor::new_overline_color(
            colour.r as u16,
            colour.g as u16,
            colour.b as u16,
        ));
    }
}
