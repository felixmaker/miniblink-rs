#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Wraps to global functions.
pub mod app;
/// Defines the content.
pub mod content;
/// Defines the miniblink error types.
pub mod error;
/// Defines the net.
pub mod net_job;
/// Defines the params.
pub mod params;
/// Defines the types.
pub mod types;
/// Wraps to mbWebView.
pub mod webview;
/// Wraps to mbWebView.
pub mod webwindow;
/// Defines the command.
pub mod command;



pub(crate) mod mbstring;

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
