#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod handler;
mod util;
mod wstr;

/// wrapper to wke global function, like wkeInitialize.
pub mod app;
/// error in miniblink (this crate). See [`MBResult`] and [`MBError`].
pub mod error;
/// wapper to proxy structs, See [`ProxyConfig`].
pub mod proxy;
/// wapper to jsValue. See [`JsValue`].
pub mod value;
/// wapper to wkeWebView. See [`WebView`].
pub mod webview;

/// support for serde. ensure to enable serde feature.
#[cfg(feature = "serde")]
pub mod serde;

use std::sync::OnceLock;

use error::{MBError, MBResult};
use miniblink_sys::Library;

pub(crate) static LIB: OnceLock<Library> = OnceLock::new();

/// Get the inner [`Library`]. Use it to call unwrappered API.
pub fn call_api() -> MBResult<&'static Library> {
    LIB.get().ok_or_else(|| MBError::NotInitialized)
}

pub(crate) fn call_api_or_panic() -> &'static Library {
    call_api().unwrap()
}
