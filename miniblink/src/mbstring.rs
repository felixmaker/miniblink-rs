use std::ffi::{CString, NulError};

use crate::call_api_or_panic;

pub struct MbString {
    inner: *mut miniblink_sys::mbString,
}

impl MbString {
    pub fn new<T>(t: T) -> Result<Self, NulError>
    where
        T: Into<Vec<u8>>,
    {
        let c_string = CString::new(t.into())?;
        let inner = unsafe {
            call_api_or_panic().mbCreateString(c_string.as_ptr(), c_string.as_bytes().len() as _)
        };
        Ok(Self { inner })
    }

    pub unsafe fn from_vec_unchecked(vec: Vec<u8>) -> Self {
        let inner =
            unsafe { call_api_or_panic().mbCreateString(vec.as_ptr() as _, vec.len() as _) };
        Self { inner }
    }
}

impl MbString {
    pub fn as_ptr(&self) -> *mut miniblink_sys::mbString {
        self.inner
    }
}

impl Drop for MbString {
    fn drop(&mut self) {
        unsafe {
            call_api_or_panic().mbDeleteString(self.inner);
        }
    }
}
