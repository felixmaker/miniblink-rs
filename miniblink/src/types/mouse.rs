
/// The mouse event.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MouseFlags {
    /// The control key is pressed.
    pub control: bool,
    /// The shift key is pressed.
    pub shift: bool,
    /// The left button is pressed.
    pub left_button: bool,
    /// The middle button is pressed.
    pub middle_button: bool,
    /// The right button is pressed.
    pub right_button: bool,
}

impl From<MouseFlags> for u32 {
    fn from(value: MouseFlags) -> Self {
        let mut flags = 0;
        if value.control {
            flags |= 0x08;
        }
        if value.shift {
            flags |= 0x04;
        }
        if value.left_button {
            flags |= 0x01;
        }
        if value.middle_button {
            flags |= 0x10;
        }
        if value.right_button {
            flags |= 0x02;
        }
        flags
    }
}
