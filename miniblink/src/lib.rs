#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Wraps to global functions.
pub mod app;
/// Defines the miniblink error types.
pub mod error;
/// Wraps to mbWebView.
pub mod webview;

use std::sync::OnceLock;

use error::{MBError, MBResult};

type MbLibrary = miniblink_sys::Library;

pub(crate) static LIB: OnceLock<MbLibrary> = OnceLock::new();

/// Call the inner api. Use it to call unwrapped api.
pub fn call_api() -> MBResult<&'static MbLibrary> {
    LIB.get().ok_or_else(|| MBError::NotInitialized)
}

pub(crate) fn call_api_or_panic() -> &'static MbLibrary {
    call_api().unwrap()
}
