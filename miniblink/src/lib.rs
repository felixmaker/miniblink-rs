#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod handler;
mod util;
mod wstr;

/// Wrapper to wke global function, like wkeInitialize.
pub mod app;
/// Error in miniblink (this crate). See [`MBResult`] and [`MBError`].
pub mod error;
/// Wapper to proxy structs, See [`proxy::ProxyConfig`].
pub mod proxy;
/// Wapper to jsValue. See [`value::JsValue`].
pub mod value;
/// Wapper to wkeWebView. See [`webview::WebView`].
pub mod webview;

/// Support for serde. Ensure to enable `serde` feature.
#[cfg(feature = "serde")]
pub mod serde;

use std::sync::OnceLock;

use error::{MBError, MBResult};
use miniblink_sys::Library;

pub(crate) static LIB: OnceLock<Library> = OnceLock::new();

/// Call the inner [`Library`]. Use it to call unwrapped API.
pub fn call_api() -> MBResult<&'static Library> {
    LIB.get().ok_or_else(|| MBError::NotInitialized)
}

pub(crate) fn call_api_or_panic() -> &'static Library {
    call_api().unwrap()
}
