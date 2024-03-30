use std::ffi::{CStr, CString};

use miniblink_sys::{
    jsExecState, wkeNavigationType, wkeProxy, wkeString, wkeWebView, wkeWindowType,
};

use crate::{util::SafeCString, webview::WebView};

use super::{CProxy, JsExecState, NavigationType, Proxy, WindowType, WkeStr};

pub trait PrepareFFI<T> {
    fn prepare(&self) -> T;
}

pub trait FromFFI<T> {
    fn from(value: T) -> Self;
}

pub trait ToFFI<T> {
    fn to(&self) -> T;
}

impl PrepareFFI<CString> for &str {
    fn prepare(&self) -> CString {
        CString::safe_new(&self)
    }
}

impl PrepareFFI<CProxy> for Proxy {
    fn prepare(&self) -> CProxy {
        CProxy::new(&self)
    }
}

impl FromFFI<*const ::std::os::raw::c_char> for String {
    fn from(value: *const ::std::os::raw::c_char) -> Self {
        let cstr = unsafe { CStr::from_ptr(value) };
        cstr.to_string_lossy().to_string()
    }
}

impl FromFFI<::std::os::raw::c_int> for i32 {
    fn from(value: ::std::os::raw::c_int) -> Self {
        value
    }
}

impl FromFFI<wkeWebView> for WebView {
    fn from(value: wkeWebView) -> Self {
        WebView { webview: value }
    }
}

impl FromFFI<wkeNavigationType> for NavigationType {
    fn from(value: wkeNavigationType) -> Self {
        match value {
            wkeNavigationType::WKE_NAVIGATION_TYPE_LINKCLICK => Self::LinkClick,
            wkeNavigationType::WKE_NAVIGATION_TYPE_FORMRESUBMITT => Self::FormSubmitte,
            wkeNavigationType::WKE_NAVIGATION_TYPE_BACKFORWARD => Self::BackForward,
            wkeNavigationType::WKE_NAVIGATION_TYPE_RELOAD => Self::Reload,
            wkeNavigationType::WKE_NAVIGATION_TYPE_FORMSUBMITTE => Self::FormResubmit,
            _ => Self::Other,
        }
    }
}

impl FromFFI<wkeString> for String {
    fn from(value: wkeString) -> Self {
        let wke_str = WkeStr::from_ptr(value);
        wke_str.to_string()
    }
}

impl FromFFI<jsExecState> for JsExecState {
    fn from(value: jsExecState) -> Self {
        Self::from_ptr(value)
    }
}

impl FromFFI<f32> for f32 {
    fn from(value: f32) -> Self {
        value
    }
}

impl ToFFI<::std::os::raw::c_int> for i32 {
    fn to(&self) -> ::std::os::raw::c_int {
        *self
    }
}

impl ToFFI<*const ::std::os::raw::c_char> for CString {
    fn to(&self) -> *const ::std::os::raw::c_char {
        self.as_ptr()
    }
}

impl ToFFI<*const ::std::os::raw::c_char> for &str {
    /// Cause memory leak.
    fn to(&self) -> *const ::std::os::raw::c_char {
        let cstring = CString::safe_new(&self);
        cstring.into_raw()
    }
}

impl ToFFI<bool> for bool {
    fn to(&self) -> bool {
        *self
    }
}

impl ToFFI<f32> for f32 {
    fn to(&self) -> Self {
        *self
    }
}

impl ToFFI<wkeWebView> for WebView {
    fn to(&self) -> wkeWebView {
        self.webview
    }
}

impl ToFFI<*const wkeProxy> for CProxy {
    fn to(&self) -> *const wkeProxy {
        self.as_ptr()
    }
}

impl ToFFI<*mut wkeProxy> for CProxy {
    fn to(&self) -> *mut wkeProxy {
        let proxy = self.clone();
        Box::into_raw(Box::new(proxy.into_wke_proxy()))
    }
}

impl ToFFI<miniblink_sys::HWND> for super::HWND {
    fn to(&self) -> miniblink_sys::HWND {
        self.0 as _
    }
}

impl ToFFI<wkeWindowType> for WindowType {
    fn to(&self) -> wkeWindowType {
        match self {
            WindowType::Control => wkeWindowType::WKE_WINDOW_TYPE_CONTROL,
            WindowType::Popup => wkeWindowType::WKE_WINDOW_TYPE_POPUP,
            WindowType::Transparent => wkeWindowType::WKE_WINDOW_TYPE_TRANSPARENT,
        }
    }
}
