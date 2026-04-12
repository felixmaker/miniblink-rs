/// The keyboard flags.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyboardFlags {
    /// repeat
    Repeat = 0,
    /// extended
    Extended = 1,
}

/// The virtual key code.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VirtualKeyCode {
    /// Left mouse button
    LeftButton = 0x01,
    /// Right mouse button
    RightButton = 0x02,
    /// Control-break processing
    Cancel = 0x03,
    /// Middle mouse button
    MiddleButton = 0x04,
    /// X1 mouse button
    XButton1 = 0x05,
    /// X2 mouse button
    XButton2 = 0x06,
    /// Backspace key
    Backspace = 0x08,
    /// Tab key
    Tab = 0x09,
    /// Clear key
    Clear = 0x0C,
    /// Enter key
    Enter = 0x0D,
    /// Shift key
    Shift = 0x10,
    /// Ctrl key
    Control = 0x11,
    /// Alt key
    Menu = 0x12,
    /// Pause key
    Pause = 0x13,
    /// Caps lock key
    CapsLock = 0x14,
    /// IME Kana / Hangul  mode
    Kana = 0x15,
    /// IME On
    ImeOn = 0x16,
    /// IME Junja mode
    Junja = 0x17,
    /// IME final mode
    Final = 0x18,
    /// IME Hanja / Kanji mode
    Hanja = 0x19,
    /// IME Off
    ImeOff = 0x1A,
    /// Esc key
    Escape = 0x1B,
    /// IME convert
    Convert = 0x1C,
    /// IME nonconvert
    Nonconvert = 0x1D,
    /// IME accept
    Accept = 0x1E,
    /// IME mode change request
    ModeChange = 0x1F,
    /// Spacebar key
    Space = 0x20,
    /// Page up key
    PageUp = 0x21,
    /// Page down key
    PageDown = 0x22,
    /// End key
    End = 0x23,
    /// Home key
    Home = 0x24,
    /// Left arrow key
    Left = 0x25,
    /// Up arrow key
    Up = 0x26,
    /// Right arrow key
    Right = 0x27,
    /// Down arrow key
    Down = 0x28,
    /// Select key
    Select = 0x29,
    /// Print key
    Print = 0x2A,
    /// Execute key
    Execute = 0x2B,
    /// Print screen key
    Snapshot = 0x2C,
    /// Insert key
    Insert = 0x2D,
    /// Delete key
    Delete = 0x2E,
    /// Help key
    Help = 0x2F,
    /// 0 key
    Key0 = 0x30,
    /// 1 key
    Key1 = 0x31,
    /// 2 key
    Key2 = 0x32,
    /// 3 key
    Key3 = 0x33,
    /// 4 key
    Key4 = 0x34,
    /// 5 key
    Key5 = 0x35,
    /// 6 key
    Key6 = 0x36,
    /// 7 key
    Key7 = 0x37,
    /// 8 key
    Key8 = 0x38,
    /// 9 key
    Key9 = 0x39,
    /// A key
    KeyA = 0x41,
    /// B key
    KeyB = 0x42,
    /// C key
    KeyC = 0x43,
    /// D key
    KeyD = 0x44,
    /// E key
    KeyE = 0x45,
    /// F key
    KeyF = 0x46,
    /// G key
    KeyG = 0x47,
    /// H key
    KeyH = 0x48,
    /// I key
    KeyI = 0x49,
    /// J key
    KeyJ = 0x4A,
    /// K key
    KeyK = 0x4B,
    /// L key
    KeyL = 0x4C,
    /// M key
    KeyM = 0x4D,
    /// N key
    KeyN = 0x4E,
    /// O key
    KeyO = 0x4F,
    /// P key
    KeyP = 0x50,
    /// Q key
    KeyQ = 0x51,
    /// R key
    KeyR = 0x52,
    /// S key
    KeyS = 0x53,
    /// T key
    KeyT = 0x54,
    /// U key
    KeyU = 0x55,
    /// V key
    KeyV = 0x56,
    /// W key
    KeyW = 0x57,
    /// X key
    KeyX = 0x58,
    /// Y key
    KeyY = 0x59,
    /// Z key
    KeyZ = 0x5A,
    /// Left Windows logo key
    LeftWin = 0x5B,
    /// Right Windows logo key
    RightWin = 0x5C,
    /// Application key
    Apps = 0x5D,
    /// Computer Sleep key
    Sleep = 0x5F,
    /// Numeric keypad 0 key
    Numpad0 = 0x60,
    /// Numeric keypad 1 key
    Numpad1 = 0x61,
    /// Numeric keypad 2 key
    Numpad2 = 0x62,
    /// Numeric keypad 3 key
    Numpad3 = 0x63,
    /// Numeric keypad 4 key
    Numpad4 = 0x64,
    /// Numeric keypad 5 key
    Numpad5 = 0x65,
    /// Numeric keypad 6 key
    Numpad6 = 0x66,
    /// Numeric keypad 7 key
    Numpad7 = 0x67,
    /// Numeric keypad 8 key
    Numpad8 = 0x68,
    /// Numeric keypad 9 key
    Numpad9 = 0x69,
    /// Multiply key
    Multiply = 0x6A,
    /// Add key
    Add = 0x6B,
    /// Separator key
    Separator = 0x6C,
    /// Subtract key
    Subtract = 0x6D,
    /// Decimal key
    Decimal = 0x6E,
    /// Divide key
    Divide = 0x6F,
    /// F1 key
    F1 = 0x70,
    /// F2 key
    F2 = 0x71,
    /// F3 key
    F3 = 0x72,
    /// F4 key
    F4 = 0x73,
    /// F5 key
    F5 = 0x74,
    /// F6 key
    F6 = 0x75,
    /// F7 key
    F7 = 0x76,
    /// F8 key
    F8 = 0x77,
    /// F9 key
    F9 = 0x78,
    /// F10 key
    F10 = 0x79,
    /// F11 key
    F11 = 0x7A,
    /// F12 key
    F12 = 0x7B,
    /// F13 key
    F13 = 0x7C,
    /// F14 key
    F14 = 0x7D,
    /// F15 key
    F15 = 0x7E,
    /// F16 key
    F16 = 0x7F,
    /// F17 key
    F17 = 0x80,
    /// F18 key
    F18 = 0x81,
    /// F19 key
    F19 = 0x82,
    /// F20 key
    F20 = 0x83,
    /// F21 key
    F21 = 0x84,
    /// F22 key
    F22 = 0x85,
    /// F23 key
    F23 = 0x86,
    /// F24 key
    F24 = 0x87,
    /// Num lock key
    NumLock = 0x90,
    /// Scroll lock key
    ScrollLock = 0x91,
    /// Left Shift key
    LeftShift = 0xA0,
    /// Right Shift key
    RightShift = 0xA1,
    /// Left Ctrl key
    LeftCtrl = 0xA2,
    /// Right Ctrl key
    RightCtrl = 0xA3,
    /// Left Alt key
    LeftAlt = 0xA4,
    /// Right Alt key
    RightAlt = 0xA5,
    /// Browser Back key
    BrowserBack = 0xA6,
    /// Browser Forward key
    BrowserForward = 0xA7,
    /// Browser Refresh key
    BrowserRefresh = 0xA8,
    /// Browser Stop key
    BrowserStop = 0xA9,
    /// Browser Search key
    BrowserSearch = 0xAA,
    /// Browser Favorites key
    BrowserFavorites = 0xAB,
    /// Browser Start and Home key
    BrowserHome = 0xAC,
    /// Volume Mute key
    VolumeMute = 0xAD,
    /// Volume Down key
    VolumeDown = 0xAE,
    /// Volume Up key
    VolumeUp = 0xAF,
    /// Media Next Track key
    MediaNextTrack = 0xB0,
    /// Media Previous Track key
    MediaPrevTrack = 0xB1,
    /// Media Stop key
    MediaStop = 0xB2,
    /// Media Play/Pause key
    MediaPlayPause = 0xB3,
    /// Start Mail key
    LaunchMail = 0xB4,
    /// Select Media key
    LaunchMediaSelect = 0xB5,
    /// Start Application 1 key
    LaunchApp1 = 0xB6,
    /// Start Application 2 key
    LaunchApp2 = 0xB7,
    /// It can vary by keyboard. For the US ANSI keyboard , the Semiсolon and Colon key
    OEM1 = 0xBA,
    /// For any country/region, the Equals and Plus key
    OEMPlus = 0xBB,
    /// For any country/region, the Comma and Less Than key
    OEMComma = 0xBC,
    /// For any country/region, the Dash and Underscore key
    OEMMinus = 0xBD,
    /// For any country/region, the Period and Greater Than key
    OEMPeriod = 0xBE,
    /// It can vary by keyboard. For the US ANSI keyboard, the Forward Slash and Question Mark key
    OEM2 = 0xBF,
    /// It can vary by keyboard. For the US ANSI keyboard, the Grave Accent and Tilde key
    OEM3 = 0xC0,
    /// It can vary by keyboard. For the US ANSI keyboard, the Left Brace key
    OEM4 = 0xDB,
    /// It can vary by keyboard. For the US ANSI keyboard, the Backslash and Pipe key
    OEM5 = 0xDC,
    /// It can vary by keyboard. For the US ANSI keyboard, the Right Brace key
    OEM6 = 0xDD,
    /// It can vary by keyboard. For the US ANSI keyboard, the Apostrophe and Double Quotation Mark key
    OEM7 = 0xDE,
    /// It can vary by keyboard. For the Canadian CSA keyboard, the Right Ctrl key
    OEM8 = 0xDF,
    /// It can vary by keyboard. For the European ISO keyboard, the Backslash and Pipe key
    OEM102 = 0xE2,
    /// IME PROCESS key
    ProcessKey = 0xE5,
    /// Used to pass Unicode characters as if they were keystrokes. The VK_PACKET key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see Remark in KEYBDINPUT, SendInput, WM_KEYDOWN, and WM_KEYUP
    Packet = 0xE7,
    /// Attn key
    Attn = 0xF6,
    /// CrSel key
    CrSel = 0xF7,
    /// ExSel key
    ExSel = 0xF8,
    /// Erase EOF key
    EraseEOF = 0xF9,
    /// Play key
    Play = 0xFA,
    /// Zoom key
    Zoom = 0xFB,
    /// Reserved
    NonName = 0xFC,
    /// PA1 key
    PA1 = 0xFD,
    /// Clear key
    OEMClear = 0xFE,
}
