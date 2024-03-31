use std::{
    ffi::{c_char, CStr, CString},
    mem::ManuallyDrop,
};

use miniblink_sys::{wchar_t, wkeString};

use crate::{call_api_or_panic, util::SafeCString};

/// A wrapper to wkeString. See `wkeString`.
#[repr(transparent)]
pub(crate) struct WkeStr {
    inner: wkeString,
}

impl WkeStr {
    pub(crate) unsafe fn from_ptr<'a>(ptr: wkeString) -> &'a Self {
        unsafe { &*(&ptr as *const wkeString as *const WkeStr) }
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
#[repr(transparent)]
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

    #[allow(unused)]
    pub(crate) fn into_raw(self) -> wkeString {
        let ptr = self.inner;
        let _ = ManuallyDrop::new(self);
        ptr
    }
}

impl std::ops::Deref for WkeString {
    type Target = WkeStr;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.inner as *const wkeString as *const WkeStr) }
    }
}

mod tests {

    #[test]
    fn test_wkestring() {
        use super::WkeString;
        use crate::app;
        use std::ffi::CStr;

        app::initialize("node.dll").unwrap();
        let wke_string = WkeString::new("Hello");
        let wke_str = &wke_string;
        let cstr = unsafe { CStr::from_ptr(wke_str.as_cstr_ptr()) };
        let cstr = cstr.to_string_lossy().to_string();
        assert_eq!(cstr, "Hello");
    }
}
