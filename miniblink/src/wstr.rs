use std::ffi::CStr;

pub struct WkeStr {
    inner: miniblink_sys::wkeString,
}

impl WkeStr {
    pub fn from_ptr(ptr: miniblink_sys::wkeString) -> Self {
        Self { inner: ptr }
    }

    pub fn to_string(&self) -> String {
        let cstr = unsafe {
            let ptr = crate::API.wkeGetString(self.inner);
            CStr::from_ptr(ptr)
        };
        cstr.to_string_lossy().to_string()
    }
}
