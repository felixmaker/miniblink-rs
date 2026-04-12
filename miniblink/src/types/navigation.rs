/// The navigation type.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NavigationType {
    /// Click on `a` tag.
    LinkClick = 0,
    /// Click on `form` tag.
    FormSubmit = 1,
    /// Click on back button.
    BackForward = 2,
    /// Click on reload button.
    Reload = 3,
    /// Form resubmit.
    FormResubmit = 4,
    /// Other navigation type.
    Other = 5,
}
