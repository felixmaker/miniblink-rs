use crate::types::{NavigationType, WindowFeatures};

/// Parameters in create view callback.
pub struct CreateViewParameters {
    /// The navigation type.
    pub navigation_type: NavigationType,
    /// The url.
    pub url: String,
    /// The window features.
    pub window_features: WindowFeatures,
}

