use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

use super::*;
use miniblink_sys::*;

use crate::{util::SafeCString, webview::WebView};

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

impl PrepareFFI<WkeString> for &str {
    fn prepare(&self) -> WkeString {
        WkeString::new(&self)
    }
}

impl PrepareFFI<CProxy> for Proxy {
    fn prepare(&self) -> CProxy {
        CProxy::new(&self)
    }
}

impl FromFFI<*const c_char> for String {
    fn from(value: *const c_char) -> Self {
        let cstr = unsafe { CStr::from_ptr(value) };
        cstr.to_string_lossy().to_string()
    }
}

impl FromFFI<c_int> for i32 {
    fn from(value: c_int) -> Self {
        value
    }
}

impl FromFFI<wkeWebView> for WebView {
    fn from(value: wkeWebView) -> Self {
        assert!(!value.is_null());
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

impl ToFFI<c_int> for i32 {
    fn to(&self) -> c_int {
        *self
    }
}

impl ToFFI<*const c_char> for CString {
    fn to(&self) -> *const c_char {
        self.as_ptr()
    }
}

impl ToFFI<*const c_char> for WkeString {
    fn to(&self) -> *const c_char {
        self.as_cstr_ptr()
    }
}

impl ToFFI<*const wchar_t> for WkeString {
    fn to(&self) -> *const wchar_t {
        self.as_wcstr_ptr()
    }
}

impl ToFFI<*const c_char> for &str {
    /// Cause memory leak.
    fn to(&self) -> *const c_char {
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

impl ToFFI<wkeMenuItemId> for MenuItemId {
    fn to(&self) -> wkeMenuItemId {
        match self {
            MenuItemId::MenuSelectedAllId => wkeMenuItemId::kWkeMenuSelectedAllId,
            MenuItemId::MenuSelectedTextId => wkeMenuItemId::kWkeMenuSelectedTextId,
            MenuItemId::MenuUndoId => wkeMenuItemId::kWkeMenuUndoId,
            MenuItemId::MenuCopyImageId => wkeMenuItemId::kWkeMenuCopyImageId,
            MenuItemId::MenuInspectElementAtId => wkeMenuItemId::kWkeMenuInspectElementAtId,
            MenuItemId::MenuCutId => wkeMenuItemId::kWkeMenuCutId,
            MenuItemId::MenuPasteId => wkeMenuItemId::kWkeMenuPasteId,
            MenuItemId::MenuPrintId => wkeMenuItemId::kWkeMenuPrintId,
            MenuItemId::MenuGoForwardId => wkeMenuItemId::kWkeMenuGoForwardId,
            MenuItemId::MenuGoBackId => wkeMenuItemId::kWkeMenuGoBackId,
            MenuItemId::MenuReloadId => wkeMenuItemId::kWkeMenuReloadId,
            MenuItemId::MenuSaveImageId => wkeMenuItemId::kWkeMenuSaveImageId,
        }
    }
}

impl PrepareFFI<wkeViewSettings> for ViewSettings {
    fn prepare(&self) -> wkeViewSettings {
        wkeViewSettings {
            size: self.size,
            bgColor: self.backgroud_color,
        }
    }
}

impl ToFFI<*const wkeViewSettings> for wkeViewSettings {
    fn to(&self) -> *const wkeViewSettings {
        &*self
    }
}
