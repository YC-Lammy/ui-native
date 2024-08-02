/// trait bounds to standardise methods over implementations
pub trait DrawingAreaShadow {
    fn new() -> Self;
}

/// trait bounds to standardise methods over implementations
pub trait DrawingAreaNative {}

/// implement drawing area for linux
#[cfg(target_os = "linux")]
mod linux {
    use std::cell::RefCell;
    use std::rc::Rc;

    use ui_native::custom::{
        MeasuredSize, NativeElementNativeNodeImpl, NativeElementShadowNodeImpl,
    };
    use ui_native::linux::{gtk4, NativeElement};

    use gtk4::cairo;
    use gtk4::gdk;
    use gtk4::prelude::*;
    use ui_native::style::StyleRef;

    /// we define the shadow node first
    #[derive(Debug, Clone)]
    pub struct DrawingArea {
        style: StyleRef,
    }

    impl DrawingArea {
        pub fn new() -> Self {
            Self {
                style: StyleRef::DEFAULT,
            }
        }

        pub fn with_style<S: Into<StyleRef>>(mut self, style: S) -> Self {
            self.set_style(style);
            return self;
        }

        pub fn set_style<S: Into<StyleRef>>(&mut self, style: S) {
            self.style = style.into();
        }
    }

    impl NativeElementShadowNodeImpl for DrawingArea {
        type NativeNode = NativeDrawingArea;
        type Changes = ();
        fn style(&self) -> Option<StyleRef> {
            Some(self.style.clone())
        }
        fn children(&self) -> impl Iterator<Item = &mut ui_native::ElementLike> {
            [].iter_mut()
        }
        fn compare(&self, _old: &Self) -> Vec<Self::Changes> {
            Vec::new()
        }
        fn build_native(&self) -> Self::NativeNode {
            NativeDrawingArea::new()
        }
    }

    pub struct NativeDrawingArea {
        frame: gtk4::Frame,
        area: gtk4::DrawingArea,
    }

    struct NativeDrawingAreaInner {
        start_x: f64,
        start_y: f64,

        last_x: f64,
        last_y: f64,

        area: gtk4::DrawingArea,
        surface: cairo::ImageSurface,
    }

    impl NativeDrawingArea {
        pub fn new() -> Self {
            let frame = gtk4::Frame::new(None);

            let area = gtk4::DrawingArea::new();
            let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 1, 1)
                .expect("failed to create surface");

            let inner = Rc::new(RefCell::new(NativeDrawingAreaInner {
                start_x: 0.0,
                start_y: 0.0,

                last_x: f64::NAN,
                last_y: f64::NAN,

                area: area.clone(),
                surface,
            }));
            let inner1 = inner.clone();
            let inner2 = inner.clone();
            let inner3 = inner.clone();
            let inner4 = inner.clone();

            area.connect_resize(move |_area, width, height| {
                let mut inner = inner.borrow_mut();

                // create a new surface
                inner.surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width, height)
                    .expect("failed to create surface");
            });

            area.set_draw_func(move |_area, context, _width, _height| {
                let inner = inner1.borrow_mut();

                let _ = context.set_source_surface(&inner.surface, 0.0, 0.0);
                let _ = context.paint();
            });

            fn draw_rect(inner: &mut NativeDrawingAreaInner, x: f64, y: f64) {
                let context =
                    cairo::Context::new(&inner.surface).expect("failed to create cairo context");

                context.rectangle(x - 3.0, y - 3.0, 6.0, 6.0);

                if !inner.last_x.is_nan() {
                    let x_dif = x - inner.last_x;
                    let y_dif = y - inner.last_y;

                    // check if the two rectangle overlapes
                    if x_dif.abs() > 3.0 || y_dif.abs() > 3.0 {
                        let max_dif = x_dif.abs().max(y_dif.abs());

                        let rects = (max_dif / 3.0).ceil() as usize;
                        let r = rects as f64;

                        let mut x = inner.last_x;
                        let mut y = inner.last_y;
                        for _ in 0..rects {
                            x += x_dif / r;
                            y += y_dif / r;

                            context.rectangle(x - 3.0, y - 3.0, 6.0, 6.0);
                        }
                    }
                }

                let _ = context.fill();

                inner.last_x = x;
                inner.last_y = y;
            }

            let drag = gtk4::GestureDrag::new();
            drag.set_button(gdk::BUTTON_PRIMARY);

            drag.connect_drag_begin(move |_drag, x, y| {
                let mut inner = inner2.borrow_mut();
                inner.start_x = x;
                inner.start_y = y;
            });
            drag.connect_drag_update(move |_drag, x, y| {
                let mut inner = inner3.borrow_mut();

                let x = inner.start_x + x;
                let y = inner.start_y + y;

                draw_rect(&mut inner, x, y);

                inner.area.queue_draw();
            });
            drag.connect_drag_end(move |_drag, x, y| {
                let mut inner = inner4.borrow_mut();

                let x = inner.start_x + x;
                let y = inner.start_y + y;

                draw_rect(&mut inner, x, y);

                inner.last_x = f64::NAN;
                inner.last_y = f64::NAN;

                inner.area.queue_draw();
            });

            area.add_controller(drag);

            frame.set_child(Some(&area));

            Self { frame, area }
        }
    }

    impl NativeElementNativeNodeImpl for NativeDrawingArea {
        type Changes = ();
        fn commit_changes(&self, _changes: &[Self::Changes]) {
            // nothing to do
        }
        fn measure(&self, known_width: Option<f32>, known_height: Option<f32>) -> MeasuredSize {
            let (min_width, natural_width, _, _) = self.area.measure(
                gtk4::Orientation::Horizontal,
                known_height.map(|i| i as i32).unwrap_or(-1),
            );

            let (min_height, natural_height, _, _) = self.area.measure(
                gtk4::Orientation::Vertical,
                known_width.map(|i| i as i32).unwrap_or(-1),
            );

            MeasuredSize {
                min_width: min_width as f32,
                natural_width: natural_width as f32,
                min_height: min_height as f32,
                natural_height: natural_height as f32,
            }
        }
    }

    impl NativeElement for NativeDrawingArea {
        fn as_gtk4_widget(&self) -> &gtk4::Widget {
            self.frame.as_ref()
        }
    }
}

/// implement drawing area for android
#[cfg(target_os = "android")]
mod android {
    // todo
}

/// implement drawing area for ios
#[cfg(target_os = "ios")]
mod ios {
    // todo
}

#[cfg(target_os = "linux")]
use linux::DrawingArea;

use ui_native::style::{AlignItems, Dimension, FlexDirection, Style};
use ui_native::widget::{Text, View};
use ui_native::AppBuilder;

static STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Centre,
    height: Dimension::Percent(1.0),
    ..Style::DEFAULT
};

pub fn main() {
    let app = AppBuilder::new()
        .with_title("drawing area example")
        .with_width(800)
        .with_height(800)
        .build()
        .expect("failed to build app");

    app.launch(|| {
        View::new()
            .with_style(&STYLE)
            .with_child(Text::new("this is our drawing area"))
            .with_child(DrawingArea::new().with_style(&STYLE))
    })
}
