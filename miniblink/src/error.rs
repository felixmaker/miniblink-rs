/// Convenient type alias of Result type for miniblink.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by miniblink.
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    NulError(#[from] std::ffi::NulError)
}
