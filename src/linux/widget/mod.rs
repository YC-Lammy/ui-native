pub mod button;
pub mod image_view;
pub mod input;
pub mod navigator;
pub mod scroll_view;
pub mod stack_navigator;
pub mod text;
pub mod text_edit;
pub mod view;

pub mod list_item_widget;
pub mod list_model;
pub mod list_view;
pub mod paintable;

use std::any::Any;

pub use button::NativeButton;
pub use image_view::NativeImageView;
pub use input::NativeTextInput;
pub use list_view::NativeListView;
pub use scroll_view::NativeScrollView;
pub use stack_navigator::NativeStackNavigator;
pub use text::NativeText;
pub use text_edit::NativeTextEdit;
pub use view::NativeView;

pub trait NativeElement: Any {
    fn as_gtk4_widget(&self) -> &gtk4::Widget;
}
