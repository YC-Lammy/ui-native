use jni_bind::jint;

use crate::android::binding::android::view::{view_group, MeasureSpec, ViewGroup};
use crate::android::binding::android::widget::{relative_layout, RelativeLayout};
use crate::custom::MeasuredSize;
use crate::native_tree::context::Context;
use crate::native_tree::{AvalableSpace, NativeStyledElement, NativeViewImp};
use crate::style::*;

use super::NativeElement;

pub struct NativeView {
    layout: RelativeLayout,
}

impl NativeElement for NativeView {
    fn as_android_view(&self) -> &super::View {
        self.layout.as_ref()
    }
}

impl NativeViewImp for NativeView {
    fn new(ctx: &mut Context) -> Self {
        let layout = RelativeLayout::new(&mut ctx.jni_env, ctx.android_activity.as_ref())
            .expect("failed to create relative layout");

        Self { layout: layout }
    }
    fn insert_child(&self, ctx: &mut Context, index: usize, elem: &dyn NativeElement) {
        let view = elem.as_android_view();

        let params = relative_layout::LayoutParams::new(&mut ctx.jni_env, 10, 10)
            .expect("failed to create layout params");

        view.set_layout_params(&mut ctx.jni_env, params.as_ref());

        let v: &ViewGroup = self.layout.as_ref();

        let re = v.add_view(&mut ctx.jni_env, view, index as jint, params.as_ref());

        if let Err(e) = re {
            log::warn!(target: "main ui thread", "encountered error: {}", e)
        }
    }
    fn remove_child(&self, ctx: &mut Context, elem: &dyn NativeElement) {
        let view = elem.as_android_view();

        let v: &ViewGroup = self.layout.as_ref();

        let re = v.remove_view(&mut ctx.jni_env, view);

        if let Err(e) = re {
            log::warn!(target: "main ui thread", "encountered error: {}", e)
        }
    }
    fn layout_child(
        &self,
        ctx: &mut Context,
        child: &dyn NativeElement,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        // get the child view
        let view = child.as_android_view();

        // create new layout params
        let params =
            relative_layout::LayoutParams::new(&mut ctx.jni_env, width as i32, height as i32)
                .expect("");
        // cast layout type
        let params: &view_group::MarginLayoutParams = params.as_ref();

        params.set_left_margin(&mut ctx.jni_env, x as i32);
        params.set_top_margin(&mut ctx.jni_env, y as i32);

        // set the layout params
        view.set_layout_params(&mut ctx.jni_env, params);
    }
}

impl NativeStyledElement for NativeView {
    fn measure(
        &self,
        ctx: &mut Context,
        known_width: AvalableSpace,
        known_height: AvalableSpace,
    ) -> anyhow::Result<MeasuredSize> {
        use crate::android::binding::android::view::View;

        let view: &View = self.layout.as_ref();

        let min_width = view.get_suggested_minimum_width(&mut ctx.jni_env)?;

        let min_height = view.get_suggested_minimum_height(&mut ctx.jni_env)?;

        let width_spec = match known_width {
            AvalableSpace::Unknown => 0,
            AvalableSpace::Exact(w) => {
                MeasureSpec::make_measure_spec(&mut ctx.jni_env, w as i32, MeasureSpec::EXACTLY)?
            }
            AvalableSpace::AtMost(w) => {
                MeasureSpec::make_measure_spec(&mut ctx.jni_env, w as i32, MeasureSpec::AT_MOST)?
            }
        };

        let height_spec = match known_height {
            AvalableSpace::Unknown => 0,
            AvalableSpace::Exact(w) => {
                MeasureSpec::make_measure_spec(&mut ctx.jni_env, w as i32, MeasureSpec::EXACTLY)?
            }
            AvalableSpace::AtMost(w) => {
                MeasureSpec::make_measure_spec(&mut ctx.jni_env, w as i32, MeasureSpec::AT_MOST)?
            }
        };

        view.measure(&mut ctx.jni_env, width_spec, height_spec)?;

        let w = view.get_measured_width(&mut ctx.jni_env)?;
        let h = view.get_measured_height(&mut ctx.jni_env)?;

        return Ok(MeasuredSize {
            min_width: min_width as f32,
            min_height: min_height as f32,
            natural_width: w as f32,
            natural_height: h as f32,
        });
    }

    fn set_visible(&self, ctx: &mut Context, visible: bool) {
        use crate::android::binding::android::view::View;

        let view: &View = self.layout.as_ref();

        if visible {
            let _ = view.set_visibility(&mut ctx.jni_env, View::VISIBLE);
        } else {
            let _ = view.set_visibility(&mut ctx.jni_env, View::GONE);
        }
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
