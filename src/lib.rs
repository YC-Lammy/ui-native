mod app;
pub mod custom;
pub mod image;
mod native_tree;
mod shadow_tree;
pub mod style;
pub mod util;
pub mod widget;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
use linux as imp;

#[cfg(target_family = "wasm")]
pub mod web;

#[cfg(target_family = "wasm")]
use web as imp;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "android")]
use android as imp;

pub use app::{App, AppBuilder, Application};

pub type ElementLike = Box<dyn private::ElementLike>;

pub trait Element {
    /// called when element is being rendered
    fn render(&self) -> ElementLike;
}

mod private {
    use crate::shadow_tree::component::CoreComponent;

    use super::Element;

    /// native element can not be implemented by client therefore kept private.
    /// `on_render` must not be called outside of the main thread, it will cause a panic.
    pub trait NativeElement {
        fn render(&mut self);
        fn core_component(&mut self) -> CoreComponent;
    }

    pub trait ElementLike: 'static {
        fn as_native(&mut self) -> Option<&mut dyn NativeElement>;
        fn as_element(&mut self) -> Option<&mut dyn Element>;
        fn render(&mut self) -> Result<CoreComponent, Box<dyn ElementLike>> {
            if let Some(e) = self.as_native() {
                e.render();
                return Ok(e.core_component());
            }
            if let Some(e) = self.as_element() {
                return Err(e.render());
            }
            unreachable!()
        }
    }

    impl<T> ElementLike for T
    where
        T: Element + 'static,
    {
        fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
            return None;
        }
        fn as_element(&mut self) -> Option<&mut dyn Element> {
            Some(self)
        }
    }

    impl<T: ElementLike> From<T> for Box<dyn ElementLike> {
        fn from(value: T) -> Self {
            Box::new(value)
        }
    }
}
