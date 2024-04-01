use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use crate::types::*;
use miniblink_sys::*;

use crate::webview::WebView;

pub trait FromFFI<T> {
    fn from(value: T) -> Self;
}

impl FromFFI<*const c_char> for String {
    fn from(value: *const c_char) -> Self {
        let cstr = unsafe { CStr::from_ptr(value) };
        cstr.to_string_lossy().to_string()
    }
}

macro_rules! from_ffi_based_on_from {
    ($(
        $ctype: ty => $rstype: ty
    );*$(;)?) => {
        $(
            impl FromFFI<$ctype> for $rstype {
                fn from(value: $ctype) -> Self {
                    From::from(value)
                }
            }
        )*
    };
}

from_ffi_based_on_from! {
    c_int => i32;
    f32 => f32;
    f64 => f64;
    HWND => Handle;
    wkeNavigationType => NavigationType;
    jsType => JsType
}

impl FromFFI<wkeString> for String {
    fn from(value: wkeString) -> Self {
        assert!(!value.is_null());
        let wke_str = unsafe { WkeStr::from_ptr(&value) };
        wke_str.to_string()
    }
}

impl FromFFI<jsExecState> for JsExecState {
    fn from(value: jsExecState) -> Self {
        assert!(!value.is_null());
        unsafe { Self::from_ptr(value) }
    }
}

impl FromFFI<wkeWebView> for WebView {
    fn from(value: wkeWebView) -> Self {
        assert!(!value.is_null());
        WebView { webview: value }
    }
}

impl FromFFI<jsValue> for JsValue {
    fn from(value: jsValue) -> Self {
        assert!(value != 0);
        unsafe { JsValue::from_ptr(value) }
    }
}

impl FromFFI<*mut jsKeys> for JsKeys {
    fn from(value: *mut jsKeys) -> Self {
        assert!(!value.is_null());
        unsafe { JsKeys::from_ptr(value) }
    }
}

impl FromFFI<c_int> for bool {
    fn from(value: c_int) -> Self {
        value != 0
    }
}

impl FromFFI<wkeWebFrameHandle> for WebFrameHandle {
    fn from(value: wkeWebFrameHandle) -> Self {
        assert!(!value.is_null());
        unsafe { WebFrameHandle::from_ptr(value) }
    }
}
