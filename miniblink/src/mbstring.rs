use std::ffi::{CString, NulError};

use crate::call_api_or_panic;

pub(crate) struct MbString {
    inner: *mut miniblink_sys::mbString,
}

#[allow(unused)]
impl MbString {
    pub(crate) fn new<T>(t: T) -> Result<Self, NulError>
    where
        T: Into<Vec<u8>>,
    {
        let c_string = CString::new(t.into())?;
        let inner = unsafe {
            call_api_or_panic().mbCreateString(c_string.as_ptr(), c_string.as_bytes().len() as _)
        };
        Ok(Self { inner })
    }

    pub(crate) unsafe fn from_vec_unchecked(vec: Vec<u8>) -> Self {
        let inner =
            unsafe { call_api_or_panic().mbCreateString(vec.as_ptr() as _, vec.len() as _) };
        Self { inner }
    }

    pub(crate) fn into_raw(self) -> *mut miniblink_sys::mbString {
        let s = std::mem::ManuallyDrop::new(self);
        s.inner
    }

    /// Create a new `MbString` from a raw pointer from `into_raw`.
    pub(crate) unsafe fn from_raw(inner: *mut miniblink_sys::mbString) -> Self {
        assert!(inner.is_null());
        Self { inner }
    }

    pub(crate) fn as_ptr(&self) -> *mut miniblink_sys::mbString {
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
