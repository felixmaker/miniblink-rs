/// A rectangular region.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// x coordinate of top left corner
    pub x: i32,
    /// y coordinate of top left corner
    pub y: i32,
    /// width
    pub width: i32,
    /// height
    pub height: i32,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 200,
            height: 200,
        }
    }
}

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

impl From<miniblink_sys::wkeNavigationType> for NavigationType {
    fn from(value: miniblink_sys::wkeNavigationType) -> Self {
        match value {
            miniblink_sys::wkeNavigationType::WKE_NAVIGATION_TYPE_LINKCLICK => {
                NavigationType::LinkClick
            }
            miniblink_sys::wkeNavigationType::WKE_NAVIGATION_TYPE_FORMRESUBMITT => {
                NavigationType::FormSubmitte
            }
            miniblink_sys::wkeNavigationType::WKE_NAVIGATION_TYPE_BACKFORWARD => {
                NavigationType::BackForward
            }
            miniblink_sys::wkeNavigationType::WKE_NAVIGATION_TYPE_RELOAD => NavigationType::Reload,
            miniblink_sys::wkeNavigationType::WKE_NAVIGATION_TYPE_FORMSUBMITTE => {
                NavigationType::FormResubmit
            }
            _ => NavigationType::Other,
        }
    }
}
