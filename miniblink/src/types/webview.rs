/// Navigation Type. See `wkeNavigationType`.
#[allow(missing_docs)]
pub enum NavigationType {
    LinkClick,
    FormSubmitte,
    BackForward,
    Reload,
    FormResubmit,
    Other,
}

/// Navigation Type. See `wkeWindowType`.
#[allow(missing_docs)]
pub enum WindowType {
    Control,
    Popup,
    Transparent,
}

/// Represent a Windows HWND
pub struct HWND(pub isize);

impl HWND {
    /// Handle NULL
    pub fn null() -> Self {
        HWND(0)
    }
}
