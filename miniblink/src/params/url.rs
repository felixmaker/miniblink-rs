/// Parameters in url change callback.
pub struct UrlChangedParameters {
    /// The url.
    pub url: String,
    /// Whether can go back.
    pub can_go_back: bool,
    /// Whether can go forward.
    pub can_go_forward: bool,
}
