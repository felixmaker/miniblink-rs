use miniblink_sys::{wkeNavigationType, wkeWebFrameHandle, wkeWindowType, HWND};

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

impl From<wkeNavigationType> for NavigationType {
    fn from(value: wkeNavigationType) -> Self {
        match value {
            wkeNavigationType::WKE_NAVIGATION_TYPE_LINKCLICK => Self::LinkClick,
            wkeNavigationType::WKE_NAVIGATION_TYPE_FORMRESUBMITT => Self::FormSubmitte,
            wkeNavigationType::WKE_NAVIGATION_TYPE_BACKFORWARD => Self::BackForward,
            wkeNavigationType::WKE_NAVIGATION_TYPE_RELOAD => Self::Reload,
            wkeNavigationType::WKE_NAVIGATION_TYPE_FORMSUBMITTE => Self::FormResubmit,
            _ => Self::Other,
        }
    }
}

/// Navigation Type. See `wkeWindowType`.
#[allow(missing_docs)]
pub enum WindowType {
    Control,
    Popup,
    Transparent,
}

impl From<WindowType> for wkeWindowType {
    fn from(value: WindowType) -> Self {
        match value {
            WindowType::Control => wkeWindowType::WKE_WINDOW_TYPE_CONTROL,
            WindowType::Popup => wkeWindowType::WKE_WINDOW_TYPE_POPUP,
            WindowType::Transparent => wkeWindowType::WKE_WINDOW_TYPE_TRANSPARENT,
        }
    }
}

#[repr(transparent)]
/// Represent a Windows HWND
pub struct Handle(pub isize);

impl Handle {
    /// Handle NULL
    pub fn null() -> Self {
        Handle(0)
    }
}

impl From<HWND> for Handle {
    fn from(value: HWND) -> Self {
        Self(value as _)
    }
}

impl From<Handle> for HWND {
    fn from(value: Handle) -> Self {
        value.0 as _
    }
}

/// See `wkeMenuItemId`
#[allow(missing_docs)]
pub enum MenuItemId {
    MenuSelectedAllId,
    MenuSelectedTextId,
    MenuUndoId,
    MenuCopyImageId,
    MenuInspectElementAtId,
    MenuCutId,
    MenuPasteId,
    MenuPrintId,
    MenuGoForwardId,
    MenuGoBackId,
    MenuReloadId,
    MenuSaveImageId,
}

/// see `wkeViewSettings`
#[allow(missing_docs)]
pub struct ViewSettings {
    pub size: i32,
    pub backgroud_color: u32,
}

/// see `POINT`
#[allow(missing_docs)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(transparent)]
/// see `wkeWebFrameHandle`
pub struct WebFrameHandle {
    frame: wkeWebFrameHandle,
}

impl WebFrameHandle {
    /// Create from wkeWebFrameHandle to WebFrameHandle
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_ptr(ptr: wkeWebFrameHandle) -> Self {
        assert!(!ptr.is_null());
        Self { frame: ptr }
    }

    /// Get the inner wkeWebFrameHandle ptr.
    pub fn as_ptr(&self) -> wkeWebFrameHandle {
        self.frame
    }
}
