#![doc = include_str!("../README.md")]

mod handler;
mod util;
mod wstr;

pub mod app;
pub mod error;
pub mod proxy;
pub mod serde;
pub mod value;
pub mod webview;

use std::sync::OnceLock;

use error::{MBError, MBResult};
use miniblink_sys::Library;

pub(crate) static LIB: OnceLock<Library> = OnceLock::new();

pub(crate) fn call_api() -> MBResult<&'static Library> {
    LIB.get().ok_or_else(|| MBError::NotInitialized)
}

pub(crate) fn call_api_or_panic() -> &'static Library {
    call_api().unwrap()
}
