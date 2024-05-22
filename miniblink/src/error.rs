/// Convenient type alias of Result type for miniblink.
pub type MBResult<T> = std::result::Result<T, MBError>;

/// Errors returned by miniblink.
#[non_exhaustive]
#[derive(Debug)]
pub enum MBError {
    /// Raised when miniblink not initialized. Make sure to call `app::init` before using miniblink.
    NotInitialized,
    /// Raised when unable to load miniblink.
    LibraryUnloaded(String),
}

impl MBError {
    pub(crate) fn to_string(&self) -> String {
        use MBError::*;
        match self {
            NotInitialized => "The miniblink is not initialized".into(),
            LibraryUnloaded(error) => format!("Failed to load miniblink! {error}"),
        }
    }
}

impl std::fmt::Display for MBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl std::error::Error for MBError {}
