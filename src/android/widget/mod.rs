mod button;
mod image_view;
mod view;

pub use button::NativeButton;
pub use image_view::NativeImageView;
pub use view::NativeView;

use super::binding::android::view::View;

pub trait NativeElement {
    fn as_android_view(&self) -> &View;
}
