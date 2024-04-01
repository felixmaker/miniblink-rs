use miniblink_sys::{wkeNavigationType, wkeWebFrameHandle, HWND};

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
    /// from wkeWebFrameHandle to WebFrameHandle
    pub unsafe fn from_ptr(ptr: wkeWebFrameHandle) -> Self {
        Self { frame: ptr }
    }
}
