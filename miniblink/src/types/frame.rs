use std::ffi::c_void;

/// The web frame handle.
pub struct WebFrameHandle {
    pub(crate) inner: *mut c_void,
}

unsafe impl Send for WebFrameHandle {}

impl WebFrameHandle {
    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.inner
    }
}
