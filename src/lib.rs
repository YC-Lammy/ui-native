use std::any::TypeId;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

mod app;
mod context;
pub mod custom;
mod native_tree;
mod shadow_tree;
pub mod style;
pub mod widget;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
use linux as imp;

#[cfg(target_family = "wasm")]
pub mod web;
use parking_lot::Mutex;
#[cfg(target_family = "wasm")]
use web as imp;

pub use app::{App, AppBuilder, Application};
pub use context::Context;

pub type ElementLike = Box<dyn private::ElementLike>;

pub trait Element {
    /// called when element is being rendered
    fn render(&self) -> ElementLike;
}

mod private {
    use crate::shadow_tree::component::CoreComponent;

    use super::Context;
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

pub fn clousure_once<F>(f: F) -> Arc<dyn Fn() + Send + Sync>
where
    F: Fn() + Send + Sync + 'static,
{
    lazy_static::lazy_static! {
        static ref TYPE_MAP: Mutex<HashMap<(TypeId, u64), usize>> = Mutex::new(HashMap::new());
    }

    let mut type_map = TYPE_MAP.lock();

    let type_id = TypeId::of::<F>();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    unsafe {
        let data =
            core::slice::from_raw_parts(&f as *const F as *const u8, core::mem::size_of::<F>());
        data.hash(&mut hasher);
    };
    let hash = hasher.finish();

    match type_map.get(&(type_id, hash)) {
        Some(c) => unsafe {
            let ptr = *c as *const F;
            Arc::increment_strong_count(ptr);

            let a = Arc::from_raw(ptr);

            return a as _;
        },
        None => {
            let f = Arc::new(f);
            let b = f.clone();

            type_map.insert((type_id, hash), Arc::into_raw(b) as usize);

            return f as _;
        }
    };
}

#[test]
fn test_closure_once() {
    for _ in 0..10 {
        let c = clousure_once(move || {});

        println!("{}", c.as_ref() as *const _ as *const u8 as usize);
    }
}
