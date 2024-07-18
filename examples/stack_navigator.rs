use ui_native::widget::{Button, StackNavigator, Text, View};
use ui_native::{AppBuilder, Element, ElementLike};

#[derive(Default)]
pub struct MyLayout {
    nav: StackNavigator,
}

impl Element for MyLayout {
    fn render(&self) -> ElementLike {
        let nav = self.nav.clone();
        let nav1 = self.nav.clone();

        Box::new(
            View::new()
                .with_child(Button::new().with_label("<-").with_on_click(move || {
                    nav.goback();
                }))
                .with_child(
                    self.nav
                        .navigator()
                        .with_screen(
                            View::new()
                                .with_child(Text::new("this is page1"))
                                .with_child(Button::new().with_label("goto page2").with_on_click(
                                    move || {
                                        nav1.goto_index(1);
                                    },
                                )),
                            Default::default(),
                        )
                        .with_screen(Text::new("this is page2"), Default::default()),
                ),
        )
    }
}

fn main() {
    let app = AppBuilder::new()
        .with_title("myapp")
        .build()
        .expect("failed to build app");

    app.launch(|_| MyLayout::default());
}
