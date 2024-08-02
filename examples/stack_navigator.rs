use ui_native::style::{Dimension, FlexDirection, Style};
use ui_native::widget::{Button, StackNavigator, View};
use ui_native::{AppBuilder, Application, ElementLike};

static STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    width: Dimension::Percent(1.0),
    height: Dimension::Percent(1.0),
    ..Style::DEFAULT
};

#[derive(Default)]
pub struct MyApp;

lazy_static::lazy_static! {
    static ref STACK: StackNavigator = StackNavigator::new();
}

impl Application for MyApp {
    fn render(&mut self) -> ElementLike {
        let mut navigator = STACK.navigator();

        navigator.add_page("page1", |navigator| {
            View::new().with_style(&STYLE).with_child(
                Button::new()
                    .with_label("go to page2")
                    .with_on_click(move || {
                        // push
                        navigator.push("page2")
                    }),
            )
        });

        navigator.add_page("page2", |navigator| {
            View::new().with_style(&STYLE).with_child(
                Button::new()
                    .with_label("go back to page1")
                    .with_on_click(move || {
                        // go back
                        navigator.goback()
                    }),
            )
        });

        Box::new(navigator)
    }
}

fn main() {
    let app = AppBuilder::new()
        .with_title("myapp")
        .build()
        .expect("failed to build app");

    app.launch(MyApp);
}
