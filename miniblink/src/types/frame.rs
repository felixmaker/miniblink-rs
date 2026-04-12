use std::ffi::c_void;

/// The web frame handle.
pub struct WebFrameHandle {
    pub(crate) inner: *mut c_void,
}

impl WebFrameHandle {
    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.inner
    }
}
