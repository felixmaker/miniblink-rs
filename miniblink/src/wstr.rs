use std::ffi::CStr;

use miniblink_sys::wkeString;

use crate::call_api_or_panic;

/// A wrapper to wkeString. See wkeString.
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
}
