pub mod button;
pub mod text;
pub mod view;
pub mod image_view;
pub mod scroll_view;
pub mod input;
pub mod text_edit;

pub mod flat_list;
pub mod list_model;

use std::any::Any;

pub use button::NativeButton;
pub use flat_list::NativeFlatList;
pub use text::NativeText;
pub use view::NativeView;
pub use image_view::NativeImageView;
pub use scroll_view::NativeScrollView;
pub use input::NativeTextInput;
pub use text_edit::NativeTextEdit;

pub trait NativeElement: Any {
    fn as_gtk4_widget(&self) -> &gtk4::Widget;
}
