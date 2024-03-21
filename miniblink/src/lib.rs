mod handler;
mod util;

pub mod app;
pub mod error;
pub mod proxy;
pub mod value;
pub mod webview;
pub mod wstr;

use std::sync::OnceLock;

use miniblink_sys::Library;

pub(crate) static LIB: OnceLock<Library> = OnceLock::new();

pub(crate) fn call_api() -> &'static Library {
    LIB.get().expect("You must init before using miniblink")
}
