use crate::types::AsynRequestState;

/// Parameters in get cookie callback.
pub struct GetCookieParameters {
    /// The state of the request.
    pub state: AsynRequestState,
    /// The cookie.
    pub cookie: String,
}
