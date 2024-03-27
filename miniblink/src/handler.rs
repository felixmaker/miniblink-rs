use std::ffi::CStr;

use crate::{
    webview::{NavigationType, WebView},
    wstr::WkeStr,
};

pub(crate) unsafe extern "C" fn navigation_handler<F>(
    webview: miniblink_sys::wkeWebView,
    param: *mut ::std::os::raw::c_void,
    navigation_type: miniblink_sys::wkeNavigationType,
    url: miniblink_sys::wkeString,
) -> bool
where
    F: FnMut(&mut WebView, NavigationType, String) -> bool,
{
    let mut webview = WebView { webview };
    let navigation_type: NavigationType = navigation_type.into();
    let url = WkeStr::from_ptr(url).to_string();
    let callback: *mut F = param as _;
    let f = &mut *callback;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        f(&mut webview, navigation_type, url)
    }));
    result.unwrap_or(false)
}

pub(crate) unsafe extern "C" fn wkestring_handler<F>(
    webview: miniblink_sys::wkeWebView,
    param: *mut ::std::os::raw::c_void,
    title: miniblink_sys::wkeString,
) where
    F: FnMut(&mut WebView, String),
{
    let mut webview = WebView { webview };
    let title = WkeStr::from_ptr(title).to_string();
    let callback: *mut F = param as _;
    let f = &mut *callback;

    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut webview, title)));
}

pub(crate) unsafe extern "C" fn void_to_bool_handler<F>(
    webview: miniblink_sys::wkeWebView,
    param: *mut ::std::os::raw::c_void,
) -> bool
where
    F: FnMut(&mut WebView) -> bool + 'static,
{
    let mut webview = WebView { webview };
    let callback: *mut F = param as _;
    let f = &mut *callback;

    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut webview)));
    r.unwrap_or(false)
}

pub(crate) unsafe extern "C" fn void_handler<F>(
    webview: miniblink_sys::wkeWebView,
    param: *mut ::std::os::raw::c_void,
) where
    F: FnMut(&mut WebView),
{
    let mut webview = WebView { webview };
    let callback: *mut F = param as _;
    let f = &mut *callback;

    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut webview)));
}

pub(crate) unsafe extern "C" fn cstr_to_bool_handler<F>(
    webview: miniblink_sys::wkeWebView,
    param: *mut ::std::os::raw::c_void,
    url: *const ::std::os::raw::c_char,
) -> bool
where
    F: FnMut(&mut WebView, String) -> bool,
{
    let mut webview = WebView { webview };
    let url = CStr::from_ptr(url).to_string_lossy().to_string();
    let callback: *mut F = param as _;
    let f = &mut *callback;

    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut webview, url)));
    r.unwrap_or(false)
}
