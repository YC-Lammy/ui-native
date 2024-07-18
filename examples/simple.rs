use ui_native::widget::{Text, View};
use ui_native::AppBuilder;

fn main() {
    let app = AppBuilder::new()
        .with_title("hello")
        .with_height(800)
        .with_width(800)
        .build()
        .expect("");

    app.launch(|_ctx| {
        let mut view = View::new();

        let t1 = Text::new("hello world");
        let t2 = Text::new("hello world");

        view.add_child(t1);
        view.add_child(t2);

        //view.set_align_items(AlignItems::Centre);
        //view.set_direction(FlexDirection::Row);

        return view
    });
}
