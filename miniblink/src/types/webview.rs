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
