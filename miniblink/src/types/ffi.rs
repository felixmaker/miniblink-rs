use std::ffi::{CStr, CString};

use miniblink_sys::{wkeNavigationType, wkeProxy, wkeString, wkeWebView};

use crate::{util::SafeCString, webview::WebView};

use super::{NavigationType, ProxyConfig, WkeStr};

pub(crate) type CCStr = *const ::std::os::raw::c_char;

pub trait FromFFI<T> {
    fn from(value: T) -> Self;
}

pub trait ToFFI<T> {
    fn to(&self) -> T;
}

impl FromFFI<CCStr> for String {
    fn from(value: CCStr) -> Self {
        let cstr = unsafe { CStr::from_ptr(value) };
        cstr.to_string_lossy().to_string()
    }
}

impl FromFFI<::std::os::raw::c_int> for i32 {
    fn from(value: ::std::os::raw::c_int) -> Self {
        From::from(value)
    }
}

impl ToFFI<::std::os::raw::c_int> for i32 {
    fn to(&self) -> ::std::os::raw::c_int {
        *self
    }
}

impl FromFFI<wkeNavigationType> for NavigationType {
    fn from(value: wkeNavigationType) -> Self {
        From::from(value)
    }
}

impl FromFFI<wkeString> for String {
    fn from(value: wkeString) -> Self {
        let wke_str = WkeStr::from_ptr(value);
        wke_str.to_string()
    }
}

impl ToFFI<CCStr> for &str {
    fn to(&self) -> CCStr {
        let cstring = CString::safe_new(&self);
        cstring.into_raw()
    }
}

impl ToFFI<bool> for bool {
    fn to(&self) -> bool {
        *self
    }
}

impl FromFFI<f32> for f32 {
    fn from(value: f32) -> Self {
        value
    }
}

impl ToFFI<f32> for f32 {
    fn to(&self) -> Self {
        *self
    }
}

impl FromFFI<wkeWebView> for WebView {
    fn from(value: wkeWebView) -> Self {
        WebView { webview: value }
    }
}

impl ToFFI<wkeWebView> for WebView {
    fn to(&self) -> wkeWebView {
        self.webview
    }
}

impl ToFFI<*mut wkeProxy> for &ProxyConfig {
    fn to(&self) -> *mut wkeProxy {
        &mut self.to_wke_proxy()
    }
}
