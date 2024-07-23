extern crate alloc;

mod app;
mod context;
pub mod custom;
mod native_tree;
mod shadow_tree;
pub mod state;
pub mod style;
pub mod widget;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
use linux as imp;

#[cfg(target_family = "wasm")]
pub mod web;
#[cfg(target_family = "wasm")]
use web as imp;

use alloc::boxed::Box;
pub use app::{App, AppBuilder};
pub use context::Context;
pub use state::State;

pub type ElementLike = Box<dyn private::ElementLike>;

pub trait Element {
    /// elements are responsible for checking
    /// whether state change affects itself.
    /// It is also responsible for calling `on_state_change` on any child element.
    ///
    /// element should call `context.request_redraw()` if redraw is needed
    fn on_state_change(&self, _ctx: &Context) {}
    /// called when element is being rendered
    fn render(&self) -> ElementLike;
}

mod private {
    use alloc::boxed::Box;

    use crate::shadow_tree::component::CoreComponent;

    use super::Context;
    use super::Element;

    /// native element can not be implemented by client therefore kept private.
    /// `on_render` must not be called outside of the main thread, it will cause a panic.
    pub trait NativeElement {
        fn on_state_change(&mut self, ctx: &Context);
        fn render(&mut self);
        fn core_component(&mut self) -> CoreComponent;
    }

    pub trait ElementLike: 'static {
        fn as_native(&mut self) -> Option<&mut dyn NativeElement>;
        fn as_element(&mut self) -> Option<&mut dyn Element>;
        fn on_state_change(&mut self, ctx: &Context) {
            if let Some(e) = self.as_native() {
                e.on_state_change(ctx);
            }
            if let Some(e) = self.as_element() {
                e.on_state_change(ctx);
            }
        }
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
}
