use ui_native::style::{Dimension, Style};
use ui_native::widget::flatlist::{ListViewDataSource, ListViewWidgetFactory};
use ui_native::{widget::*, AppBuilder};

struct Range;

impl ListViewDataSource for Range {
    type Item = usize;
    fn get(&self, index: usize) -> Option<Self::Item> {
        if index >= 10000 {
            return None;
        }
        return Some(index);
    }
    fn len(&self) -> usize {
        10000
    }
}

struct Factory;

impl ListViewWidgetFactory for Factory {
    type Item = usize;
    fn render_item(&self, index: usize, data: Self::Item) -> ui_native::ElementLike {
        return Box::new(View::new().with_child(Text::new(format!(
            "this is the {} item in list, with data '{}'",
            index, data
        ))));
    }
}

static STYLE: Style = Style {
    height: Dimension::Percent(1.0),
    ..Style::DEFAULT
};

pub fn main() {
    let app = AppBuilder::new()
        .with_title("list view example")
        .with_width(800)
        .with_height(800)
        .build()
        .expect("failed to initialise app");

    app.launch(|| {
        View::new()
            .with_style(&STYLE)
            .with_child(ListView::new(Range, Factory).with_style(&STYLE))
    })
}
