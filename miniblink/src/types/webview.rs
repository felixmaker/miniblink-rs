use miniblink_sys::HWND;

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
pub struct Handle(pub isize);

impl Handle {
    /// Handle NULL
    pub fn null() -> Self {
        Handle(0)
    }

    pub fn from_hwnd(hwnd: HWND) -> Self {
        Self(hwnd as isize)
    }
}

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

pub struct ViewSettings {
    pub size: i32,
    pub backgroud_color: u32,
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}
