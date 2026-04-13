use miniblink_sys::mbWindowFeatures;

/// Window Type.
#[repr(i32)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WindowType {
    /// Popup type
    Popup = 0,
    /// Transparent type. Achieved using layer window.    
    Transparent = 1,
    /// Control type. Create window as child window. Requied parent.
    Control = 2,
}

/// The window features.
#[derive(Debug, Copy, Clone)]
pub struct WindowFeatures {
    /// The x position.
    pub x: i32,
    /// The y position.
    pub y: i32,
    /// The width.
    pub width: i32,
    /// The height.
    pub height: i32,
    /// Whether the menu bar is visible.
    pub menu_bar_visible: bool,
    /// Whether the status bar is visible.
    pub status_bar_visible: bool,
    /// Whether the tool bar is visible.
    pub tool_bar_visible: bool,
    /// Whether the location bar is visible.
    pub location_bar_visible: bool,
    /// Whether the scroll bars are visible.
    pub scroll_bars_visible: bool,
    /// Whether the window is resizable.
    pub resizable: bool,
    /// Whether the window is fullscreen.
    pub fullscreen: bool,
}

impl WindowFeatures {
    pub(crate) fn from_mb_window_features(features: &mbWindowFeatures) -> Self {
        Self {
            x: features.x,
            y: features.y,
            width: features.width,
            height: features.height,
            menu_bar_visible: features.menuBarVisible != 0,
            status_bar_visible: features.statusBarVisible != 0,
            tool_bar_visible: features.toolBarVisible != 0,
            location_bar_visible: features.locationBarVisible != 0,
            scroll_bars_visible: features.scrollbarsVisible != 0,
            resizable: features.resizable != 0,
            fullscreen: features.fullscreen != 0,
        }
    }
}

/// The windows message.
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WindowMessage {
    Null = 0x0000,
    GetMinMaxInfo = 0x0024,
    Move = 0x0003,
    Timer = 0x0113,
    Paint = 0x000F,
    Close = 0x0010,
    LeftButtonUp = 0x0202,
    MouseMove = 0x0200,
    MiddleButtonUp = 0x0208,
    RightButtonUp = 0x0205,
    SetCursor = 0x0020,
    LeftButtonDown = 0x0201,
    ImeChar = 0x0286,
    SystemCommand = 0x0112,
    MouseLeave = 0x02A3,
    NonClientMouseMove = 0x00A0,
    NonClientMouseHover = 0x02A0,
    NonClientHitTest = 0x0084,
    MiddleButtonDown = 0x0207,
    RightButtonDown = 0x0204,
    LeftButtonDoubleClick = 0x0203,
    Command = 0x0111,
    ExitMenuLoop = 0x0212,
    RenderFormat = 0x0305,
    RenderAllFormats = 0x0306,
    DrawClipboard = 0x0308,
    Destroy = 0x0002,
    ChangeClipboardChain = 0x030D,
    Size = 0x0005,
    CancelMode = 0x001F,
    MouseWheel = 0x020A,
    KeyUp = 0x0101,
    KeyDown = 0x0100,
    Char = 0x0102,
    SetFocus = 0x0007,
    KillFocus = 0x0008,
    Create = 0x0001,
    NonClientPaint = 0x0085,
    EraseBackground = 0x0014,
    DropFiles = 0x0233,
    NonClientDestroy = 0x0082,
    SysKeyDown = 0x0104,
    SysKeyUp = 0x0105,
    MiddleButtonDoubleClick = 0x0209,
    RightButtonDoubleClick = 0x0206,
    ImeComposition = 0x010F,
    Quit = 0x0012,
    User = 0x0400,
    SetFont = 0x0030,
    Touch = 0x0240,
    CaptureChanged = 0x0215,
}
