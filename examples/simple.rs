use std::ops::Deref;

use ui_native::style::{
    AlignContent, AlignItems, Dimension, FlexDirection, JustifyContent, MarginDimension, Style,
};
use ui_native::widget::{Button, ImageView, Text, TextEdit, TextInput, View};
use ui_native::AppBuilder;

static MY_VIEW_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Centre,
    align_content: AlignContent::Centre,
    justify_items: AlignItems::Centre,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

static MY_INPUT_STYLE: Style = Style {
    margin_top: MarginDimension::Points(20.0),
    width: Dimension::Points(400.0),
    ..Style::DEFAULT
};

fn main() {
    let app = AppBuilder::new()
        .with_title("hello")
        .with_height(800)
        .with_width(800)
        .build()
        .expect("");

    app.launch(|| {
        let mut view = View::new();

        view.set_style(&MY_VIEW_STYLE);

        let t1 = Text::new("hello world");
        let t2 = Text::new("hello world");

        view.add_child(t1);
        view.add_child(t2);
        view.add_child(
            Button::new()
                .with_label("press me")
                .with_on_click(|| println!("button clicked")),
        );
        view.add_child(
            Button::new()
                .with_label("press me")
                .with_on_click(|| println!("button clicked")),
        );
        view.add_child(
            TextInput::new()
                .with_background_text("hello world")
                .with_style(&MY_INPUT_STYLE),
        );
        view.add_child(TextEdit::new());

        lazy_static::lazy_static! {
            static ref IMAGE: image::RgbaImage = {
                image::load_from_memory(include_bytes!("./ferris.png"))
                .unwrap()
                .into_rgba8()
            };
        }

        view.add_child(ImageView::new(IMAGE.deref()));

        return view;
    });
}
