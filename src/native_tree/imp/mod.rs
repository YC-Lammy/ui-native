#[cfg(target_os = "android")]
pub mod android;
pub mod linux;

#[cfg(target_os = "android")]
pub use android::*;
#[cfg(target_os = "linux")]
pub use linux::*;
