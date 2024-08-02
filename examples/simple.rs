use std::sync::Arc;

use ui_native::style::{
    AlignContent, AlignItems, Dimension, FlexDirection, MarginDimension, StyleSheet,
};
use ui_native::widget::{Button, Text, TextEdit, TextInput, View};
use ui_native::AppBuilder;

lazy_static::lazy_static! {
    static ref MY_VIEW_STYLE: Arc<StyleSheet> = {
        let style_sheet = Arc::new(StyleSheet::new());


        style_sheet.set_flex_direction(FlexDirection::Column);
        style_sheet.set_align_items(AlignItems::Centre);
        style_sheet.set_align_content(AlignContent::Centre);
        style_sheet.set_justify_items(AlignItems::Centre);

        style_sheet.set_height(Dimension::Percent(1.0));
        style_sheet.set_width(Dimension::Percent(1.0));

        return style_sheet
    };

    static ref MY_INPUT_STYLE: Arc<StyleSheet> = {
        let style_sheet = Arc::new(StyleSheet::new());

        style_sheet.set_margin_top(MarginDimension::Points(20.0));
        style_sheet.set_width(Dimension::Points(400.0));

        return style_sheet
    };
}

fn main() {
    let app = AppBuilder::new()
        .with_title("hello")
        .with_height(800)
        .with_width(800)
        .build()
        .expect("");

    app.launch(|| {
        let mut view = View::new();

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
                .with_style(MY_INPUT_STYLE.clone()),
        );
        view.add_child(TextEdit::new());

        view.set_style(MY_VIEW_STYLE.clone());

        //view.set_align_items(AlignItems::Centre);
        //view.set_direction(FlexDirection::Row);

        return view;
    });
}
