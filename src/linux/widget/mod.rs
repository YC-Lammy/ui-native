
pub mod text;
pub mod view;
pub mod button;

pub mod list_model;
pub mod flat_list;


use std::any::Any;

pub use view::NativeView;
pub use button::NativeButton;
pub use text::NativeText;
pub use flat_list::NativeFlatList;

pub trait NativeElement: Any {
    fn as_gtk4_widget(&self) -> &gtk4::Widget;
}
