use crate::types::NavigationType;

/// Parameters in navigation callback.
pub struct NavigationParameters {
    /// The navigation type.
    pub navigation_type: NavigationType,
    /// The url.
    pub url: String,
}
