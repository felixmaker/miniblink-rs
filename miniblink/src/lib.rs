#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

// pub(crate) mod macros;
mod util;

/// Wrapper to wke global function, like wkeInitialize.
pub mod app;
/// Error in miniblink (this crate). See [`MBResult`] and [`MBError`].
pub mod error;
/// Wapper to minibink types.
pub mod types;
/// Wapper to wkeWebView. See [`webview::WebView`].
pub mod webview;

/// Prelude to use some useful functions and traits.
pub mod prelude {
    pub use super::error::{MBError, MBResult};
    pub use super::types::{JsExecStateExt, MBExecStateValue};
    pub use super::webview::WebViewExt;
}

/// Support for serde. Require `serde` feature.
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
