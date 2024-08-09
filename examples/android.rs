#![no_main]
#![crate_type = "cdylib"]
#![cfg(target_os = "android")]

use android_activity::AndroidApp;
use ui_native::style::{Dimension, FlexDirection, Style};
use ui_native::widget::*;
use ui_native::AppBuilder;

static STYLE: Style = Style {
    width: Dimension::Percent(0.7),
    height: Dimension::Percent(0.7),
    flex_direction: FlexDirection::Column,
    ..Style::DEFAULT
};

#[no_mangle]
fn android_main(activity: AndroidApp) {
    let app = AppBuilder::new()
        .build_android(activity)
        .expect("failed to initialise app");

    app.launch(|| {
        View::new()
            .with_style(&STYLE)
            .with_child(Text::new("hello android, type something below"))
            .with_child(TextEdit::new())
    })
}
