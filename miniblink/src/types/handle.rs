use std::ffi::c_void;

/// The window handle.
#[repr(transparent)]
pub struct WindowHandle {
    pub(crate) inner: *mut c_void,
}

unsafe impl Send for WindowHandle {}
