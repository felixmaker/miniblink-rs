use std::ffi::{c_char, CStr, CString};

use miniblink_sys::{wchar_t, wkeString};

use crate::{call_api_or_panic, util::SafeCString};

/// A wrapper to wkeString. See `wkeString`.
pub(crate) struct WkeStr {
    inner: wkeString,
}

impl WkeStr {
    pub(crate) fn from_ptr(ptr: wkeString) -> Self {
        Self { inner: ptr }
    }

    pub(crate) fn to_string(&self) -> String {
        let cstr = unsafe {
            let ptr = call_api_or_panic().wkeGetString(self.inner);
            CStr::from_ptr(ptr)
        };
        cstr.to_string_lossy().to_string()
    }

    pub(crate) fn as_wcstr_ptr(&self) -> *const wchar_t {
        unsafe { call_api_or_panic().wkeGetStringW(self.inner) }
    }

    pub(crate) fn as_cstr_ptr(&self) -> *const c_char {
        unsafe { call_api_or_panic().wkeGetString(self.inner) }
    }
}

/// Owned wkeString
pub(crate) struct WkeString {
    inner: wkeString,
}

impl Drop for WkeString {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe { call_api_or_panic().wkeDeleteString(self.inner) }
        }
    }
}

impl WkeString {
    pub(crate) fn new(s: &str) -> Self {
        let cstring = CString::safe_new(s);
        let inner = unsafe {
            call_api_or_panic().wkeCreateString(cstring.as_ptr(), cstring.as_bytes().len())
        };
        Self { inner }
    }

    pub(crate) fn as_wkestr(&self) -> WkeStr {
        WkeStr::from_ptr(self.inner)
    }

    pub(crate) fn as_wcstr_ptr(&self) -> *const wchar_t {
        self.as_wkestr().as_wcstr_ptr()
    }

    pub(crate) fn as_cstr_ptr(&self) -> *const c_char {
        self.as_wkestr().as_cstr_ptr()
    }
}
