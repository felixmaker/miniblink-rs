use std::{
    ffi::{c_char, CStr, CString},
    mem::ManuallyDrop,
};

use miniblink_sys::{wchar_t, wkeString};

use crate::{call_api_or_panic, util::SafeCString};

#[derive(Debug)]
/// Raw wraps to wkeString. See `wkeString`.
#[repr(transparent)]
pub struct WkeStr {
    inner: wkeString,
}

impl WkeStr {
    /// Wraps a wkeString from pointer.
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_ptr(ptr: wkeString) -> Self {
        assert!(!ptr.is_null());
        Self { inner: ptr }
    }

    pub(crate) fn to_string(&self) -> String {
        let cstr = unsafe {
            let ptr = call_api_or_panic().wkeGetString(self.inner);
            CStr::from_ptr(ptr)
        };
        cstr.to_string_lossy().to_string()
    }

    /// See `wkeGetStringW`.
    pub fn as_wcstr_ptr(&self) -> *const wchar_t {
        unsafe { call_api_or_panic().wkeGetStringW(self.inner) }
    }

    /// See `wkeGetString`.
    pub fn as_cstr_ptr(&self) -> *const c_char {
        unsafe { call_api_or_panic().wkeGetString(self.inner) }
    }
}

/// Wraps to wkeString. Auto drop the inner wkeString. 
pub struct WkeString {
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
    /// Create a wkeString. See `wkeCreateString`.
    pub fn new(s: &str) -> Self {
        let cstring = CString::safe_new(s);
        let inner = unsafe {
            call_api_or_panic().wkeCreateString(cstring.as_ptr(), cstring.as_bytes().len())
        };
        Self { inner }
    }

    /// Consumes the WkeString and transfers ownership to a C caller.
    pub fn into_raw(self) -> wkeString {
        let ptr = self.inner;
        let _ = ManuallyDrop::new(self);
        ptr
    }

    /// Retakes ownership of a WkeString that was transferred to C via WkeString::into_raw.
    pub unsafe fn from_raw(ptr: wkeString) -> Self {
        Self { inner: ptr }
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
        app::initialize("node.dll").unwrap();
        let wke_string = WkeString::new("Hello");
        let wke_str = wke_string.to_string();
        assert_eq!(wke_str, "Hello");
    }
}
