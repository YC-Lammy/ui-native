use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub(crate) static GLOBAL_STATE_CHANGED: AtomicBool = AtomicBool::new(false);

pub struct State<T> {
    inner: Arc<StateInner<T>>,
}

#[repr(C)]
struct StateInner<T> {
    version: AtomicU64,
    value: RwLock<T>,
}

impl<T> State<T> {
    pub fn new<V: Into<T>>(value: V) -> State<T> {
        State {
            inner: Arc::new(StateInner {
                version: AtomicU64::new(0),
                value: RwLock::new(value.into()),
            }),
        }
    }
    fn trigger_state_change(&self) {
        self.inner.version.fetch_add(1, Ordering::Release);
        GLOBAL_STATE_CHANGED.store(true, Ordering::Release);
    }
    pub fn version(&self) -> u64 {
        self.inner.version.load(Ordering::Relaxed)
    }
    /// set the content of state, triggers a state change
    pub fn set(&self, mut value: T) -> T {
        let mut v = self.inner.value.write();
        core::mem::swap(&mut value, &mut v);
        // trigger state change
        self.trigger_state_change();
        return value;
    }
    pub fn get(&self) -> RwLockReadGuard<T> {
        self.inner.value.read()
    }
    /// calling `get_mut` will trigger a state change
    pub fn get_mut(&self) -> StateWriteGuard<T> {
        // acquire write guard
        let guard = self.inner.value.write();
        StateWriteGuard {
            state: self,
            guard: guard,
        }
    }
    pub fn is_same_state(&self, other: &State<T>) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

pub struct StateWriteGuard<'a, T> {
    state: &'a State<T>,
    guard: RwLockWriteGuard<'a, T>,
}

impl<'a, T> Drop for StateWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.state.trigger_state_change();
    }
}

impl<'a, T> AsRef<T> for StateWriteGuard<'a, T> {
    fn as_ref(&self) -> &T {
        &self.guard
    }
}

impl<'a, T> AsMut<T> for StateWriteGuard<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.guard
    }
}

impl<'a, T> core::borrow::Borrow<T> for StateWriteGuard<'a, T> {
    fn borrow(&self) -> &T {
        &self.guard
    }
}

impl<'a, T> core::borrow::BorrowMut<T> for StateWriteGuard<'a, T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.guard
    }
}

impl<'a, T> core::ops::Deref for StateWriteGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> core::ops::DerefMut for StateWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

pub trait StateLike {
    fn version(&self) -> u64;
    fn is_same_state(&self, other: &State<()>) -> bool;
}

impl<T> StateLike for State<T> {
    fn version(&self) -> u64 {
        self.version()
    }
    fn is_same_state(&self, other: &State<()>) -> bool {
        self.is_same_state(unsafe { core::mem::transmute::<_, &State<T>>(other) })
    }
}
