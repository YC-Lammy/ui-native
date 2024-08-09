pub mod button;
pub mod flatlist;
pub mod image_view;
pub mod input;
pub mod scrollview;
pub mod stack_navigator;
pub mod text;
pub mod text_edit;
pub mod view;

pub use button::Button;
pub use flatlist::ListView;
pub use image_view::ImageView;
pub use input::TextInput;
pub use scrollview::ScrollView;
pub use stack_navigator::{StackNavigator, StackNavigatorElement};
pub use text::Text;
pub use text_edit::TextEdit;
pub use view::View;

use crate::private::ElementLike;
pub trait BuildableWidget {
    type Builder: WidgetBuilder<Widget = Self>;
    fn builder() -> Self::Builder;
}

pub trait WidgetBuilder {
    type Widget: ElementLike;
    fn try_build(self) -> anyhow::Result<Self::Widget>;
}
