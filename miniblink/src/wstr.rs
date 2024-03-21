use std::ffi::CStr;

use crate::call_api;

pub struct WkeStr {
    inner: miniblink_sys::wkeString,
}

impl WkeStr {
    pub fn from_ptr(ptr: miniblink_sys::wkeString) -> Self {
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
