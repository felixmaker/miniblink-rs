/// The window handle.
#[repr(transparent)]
pub struct WindowHandle {
    pub(crate) inner: miniblink_sys::HWND,
}
