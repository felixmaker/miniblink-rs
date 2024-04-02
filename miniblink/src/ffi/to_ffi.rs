use std::ffi::CString;
use std::os::raw::{c_char, c_int};

use crate::types::*;
use miniblink_sys::*;

use crate::{util::SafeCString, webview::RawWebView};

pub trait ToFFI<T> {
    fn to(&self) -> T;
}

macro_rules! to_ffi_based_on_from {
    ($(
        $rstype: ty => $ctype: ty
    );*$(;)?) => {
        $(
            impl ToFFI<$ctype> for $rstype {
                fn to(&self) -> $ctype {
                    From::from(*self)
                }
            }
        )*
    };
}

to_ffi_based_on_from! {
    i32 => c_int;
    f64 => f64
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

impl ToFFI<wkeWebView> for RawWebView {
    fn to(&self) -> wkeWebView {
        self.as_ptr()
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

impl ToFFI<HWND> for Handle {
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

impl ToFFI<*const wkeViewSettings> for wkeViewSettings {
    fn to(&self) -> *const wkeViewSettings {
        &*self
    }
}

impl ToFFI<*const POINT> for POINT {
    fn to(&self) -> *const POINT {
        &*self
    }
}

impl ToFFI<jsExecState> for JsExecState {
    fn to(&self) -> jsExecState {
        self.as_ptr()
    }
}

impl ToFFI<jsValue> for JsValue {
    fn to(&self) -> jsValue {
        self.as_ptr()
    }
}

impl ToFFI<wkeWebFrameHandle> for WebFrameHandle {
    fn to(&self) -> wkeWebFrameHandle {
        self.as_ptr()
    }
}