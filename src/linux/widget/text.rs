//use gtk4::pango;
use gtk4::prelude::*;

use crate::native_tree::context::Context;
use crate::native_tree::AvalableSpace;
use crate::native_tree::MeasuredSize;
use crate::native_tree::NativeStyledElement;
use crate::native_tree::NativeTextImp;
use crate::style::{
    BorderStyle, Colour, FontStyle, FontWeight, PointEvents, TextAlign, TextDecorationLine,
    TextTransform,
};

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
    fn new(_ctx: &mut Context, s: &str) -> Self {
        Self {
            label: gtk4::Label::new(Some(&s)),
            ..Default::default()
        }
    }

    fn set_text(&self, _ctx: &mut Context, text: &str) {
        self.label.set_text(text)
    }
}

impl NativeStyledElement for NativeText {
    fn measure(
        &self,
        _ctx: &mut Context,
        known_width: AvalableSpace,
        known_height: AvalableSpace,
    ) -> anyhow::Result<MeasuredSize> {
        // measure width
        let (min_width, natural_width, _, _) = self.label.measure(
            gtk4::Orientation::Horizontal,
            match known_height {
                AvalableSpace::AtMost(f) => f as i32,
                AvalableSpace::Exact(f) => f as i32,
                AvalableSpace::Unknown => -1,
            },
        );
        // measure height
        let (min_height, natural_height, _, _) = self.label.measure(
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
        self.label.set_visible(visible)
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
        self.label.set_opacity(opacity as f64)
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
