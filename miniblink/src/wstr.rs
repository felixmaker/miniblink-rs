use std::ffi::CStr;

use miniblink_sys::wkeString;

use crate::call_api;

/// A wrapper to wkeString. See wkeString.
pub struct WkeStr {
    inner: wkeString,
}

impl WkeStr {
    pub fn from_ptr(ptr: wkeString) -> Self {
        Self { inner: ptr }
    }

    pub fn to_string(&self) -> String {
        let cstr = unsafe {
            let ptr = call_api().wkeGetString(self.inner);
            CStr::from_ptr(ptr)
        };
        cstr.to_string_lossy().to_string()
    }
}
