use std::ffi::CStr;

use crate::{
    webview::{NavigationType, WebView},
    wstr::WkeStr,
};

pub(crate) type NavigationHandler = Box<dyn FnMut(&mut WebView, NavigationType, String) -> bool>;
pub(crate) type WkeStringHandler = Box<dyn FnMut(&mut WebView, String) -> bool>;
pub(crate) type VoidToBoolHandler = Box<dyn FnMut(&mut WebView) -> bool>;
pub(crate) type VoidHandler = Box<dyn FnMut(&mut WebView)>;
pub(crate) type CStrToBoolHandler = Box<dyn FnMut(&mut WebView, String) -> bool>;

pub(crate) unsafe extern "C" fn navigation_handler(
    webview: miniblink_sys::wkeWebView,
    param: *mut ::std::os::raw::c_void,
    navigation_type: miniblink_sys::wkeNavigationType,
    url: miniblink_sys::wkeString,
) -> bool {
    let mut webview = WebView { webview };
    let navigation_type: NavigationType = navigation_type.into();
    let url = WkeStr::from_ptr(url).to_string();
    let callback: *mut Box<NavigationHandler> = param as _;
    let f = &mut **callback;

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        f(&mut webview, navigation_type, url)
    }));
    result.unwrap_or(false)
}

pub(crate) unsafe extern "C" fn wkestring_handler(
    webview: miniblink_sys::wkeWebView,
    param: *mut ::std::os::raw::c_void,
    title: miniblink_sys::wkeString,
) {
    let mut webview = WebView { webview };
    let title = WkeStr::from_ptr(title).to_string();
    let callback: *mut WkeStringHandler = param as _;
    let f = &mut **callback;

    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut webview, title)));
}

pub(crate) unsafe extern "C" fn void_to_bool_handler(
    webview: miniblink_sys::wkeWebView,
    param: *mut ::std::os::raw::c_void,
) -> bool {
    let mut webview = WebView { webview };
    let callback: *mut VoidToBoolHandler = param as _;
    let f = &mut **callback;

    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut webview)));
    r.unwrap_or(false)
}

pub(crate) unsafe extern "C" fn void_handler(
    webview: miniblink_sys::wkeWebView,
    param: *mut ::std::os::raw::c_void,
) {
    let mut webview = WebView { webview };
    let callback: *mut VoidHandler = param as _;
    let f = &mut **callback;

    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut webview)));
}

pub(crate) unsafe extern "C" fn cstr_to_bool_handler(
    webview: miniblink_sys::wkeWebView,
    param: *mut ::std::os::raw::c_void,
    url: *const ::std::os::raw::c_char,
) -> bool {
    let mut webview = WebView { webview };
    let url = CStr::from_ptr(url).to_string_lossy().to_string();
    let callback: *mut CStrToBoolHandler = param as _;
    let f = &mut **callback;

    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut webview, url)));
    r.unwrap_or(false)
}
