pub mod button;
pub mod flatlist;
pub mod stack_navigator;
pub mod text;
pub mod view;
pub mod scrollview;
pub mod input;

pub use button::Button;
pub use stack_navigator::{StackNavigator, StackNavigatorElement};
pub use text::Text;
pub use view::View;
pub use input::TextInput;

use crate::private::ElementLike;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlexDirection {
    Column,
    Row,
}

pub enum AlignContent {
    Centre,
    Start,
    End,
    Stretch,
}

pub enum AlignItems {
    Centre,
    Start,
    End,
    Stretch,
    Baseline,
}

pub trait BuildableWidget {
    type Builder: WidgetBuilder<Widget = Self>;
    fn builder() -> Self::Builder;
}

pub trait WidgetBuilder {
    type Widget: ElementLike;
    fn try_build(self) -> anyhow::Result<Self::Widget>;
}
