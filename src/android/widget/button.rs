use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use jni::JNIEnv;
use jni_bind::jboolean;
use parking_lot::RwLock;

use crate::android::binding::android::view::{MeasureSpec, View};
use crate::android::binding::ui_native::ButtonOnClickListener;
use crate::android::binding::{android, java};

use crate::custom::MeasuredSize;
use crate::native_tree::context::Context;
use crate::native_tree::{AvalableSpace, NativeButtonImp, NativeStyledElement};
use crate::style::*;

use super::NativeElement;

static CALLBACKS: RwLock<Vec<Option<Arc<dyn Fn() + Sync + Send>>>> = RwLock::new(Vec::new());

pub struct NativeButton {
    button: android::widget::Button,
    listener: ButtonOnClickListener,
    callback_id: AtomicU32,
}

#[no_mangle]
pub extern "system" fn Java_com_uinative_ButtonOnClickListener_onClick(
    mut env: JNIEnv,
    listener: ButtonOnClickListener,
    _view: View,
) {
    let id = listener.get_id(&mut env).expect("failed to get id");
    let callbacks = CALLBACKS.read();

    if let Some(cb) = callbacks.get(id as usize) {
        if let Some(cb) = cb {
            (cb)();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_uinative_ButtonOnClickListener_onDrop(
    mut env: JNIEnv,
    listener: ButtonOnClickListener,
) {
    let id = listener.get_id(&mut env).expect("failed to get id");
    let mut callbacks = CALLBACKS.write();

    if let Some(cb) = callbacks.get_mut(id as usize) {
        *cb = None;
    }
}

impl NativeElement for NativeButton {
    fn as_android_view(&self) -> &crate::android::binding::android::view::View {
        todo!()
    }
}

impl NativeButtonImp for NativeButton {
    fn new(ctx: &mut Context) -> Self {
        // create a button
        let button = android::widget::Button::new(&mut ctx.jni_env, ctx.android_activity.as_ref())
            .expect("");

        // create a listener
        let listener = ButtonOnClickListener::new(&mut ctx.jni_env, u32::MAX as i32).expect("");

        // set the listener
        button
            .set_on_click_listener(&mut ctx.jni_env, &listener)
            .expect("");

        Self {
            button,
            listener,
            callback_id: AtomicU32::new(u32::MAX),
        }
    }

    fn set_disabled(&self, ctx: &mut Context, disabled: bool) {
        let _ = self
            .button
            .set_clickable(&mut ctx.jni_env, disabled as jboolean);
    }

    fn set_label(&self, ctx: &mut Context, text: String) {
        let s = java::lang::String::from_str(&mut ctx.jni_env, &text);

        self.button.set_text(&mut ctx.jni_env, s);
    }

    fn set_on_click(&self, ctx: &mut Context, on_click: Option<Arc<dyn Fn() + Send + Sync>>) {
        let old_id = self.callback_id.load(Ordering::Relaxed);
        let mut id = old_id;

        let mut callbacks = CALLBACKS.write();

        // u32::MAX is not a valid id
        if id == u32::MAX && on_click.is_some() {
            // loop through the callbacks and find an empty slot
            for (i, cb) in callbacks.iter_mut().enumerate() {
                // found an empty slot
                if cb.is_none() {
                    id = i as u32;
                    break;
                }
            }
        }

        // try to get the old callback from vec
        if let Some(cb) = callbacks.get_mut(id as usize) {
            // if callback is removed, drop the slot
            if on_click.is_none() {
                id = u32::MAX
            }
            // set the new callback
            *cb = on_click;
        } else if on_click.is_some() {
            // push the callback to the back
            callbacks.push(on_click);
        }

        if id != old_id {
            // store the new callback id
            self.callback_id.store(id, Ordering::Relaxed);
            let _ = self.listener.set_id(&mut ctx.jni_env, id as i32);
        }
    }
}

impl NativeStyledElement for NativeButton {
    fn measure(
        &self,
        ctx: &mut Context,
        known_width: AvalableSpace,
        known_height: AvalableSpace,
    ) -> anyhow::Result<MeasuredSize> {
        use crate::android::binding::android::view::View;

        let view: &View = self.button.as_ref();

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

        let view: &View = self.button.as_ref();

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
