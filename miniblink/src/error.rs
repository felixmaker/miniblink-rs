use crate::value::JsValue;

/// Convenient type alias of Result type for miniblink.
pub type MBResult<T> = std::result::Result<T, MBError>;

/// Errors returned by miniblink.
#[non_exhaustive]
#[derive(Debug)]
pub enum MBError {
    UnsupportedPlatform,
    NotInitialized,
    LibraryUnloaded(String),
    FromJsValueFailed(JsValue),
    TypeError(i32),
}

impl MBError {
    pub(crate) fn to_string(&self) -> String {
        use MBError::*;
        match self {
            UnsupportedPlatform => {
                "Failed to create as child window. Only windows is supported!".into()
            }
            NotInitialized => "The miniblink is not initialized".into(),
            LibraryUnloaded(error) => format!("Failed to load miniblink! {error}"),
            FromJsValueFailed(value) => format!("Failed to convert jsValue `{value:?}`!"),
            TypeError(index) => format!("TypeError: param of index `{index}`"),
        }
    }
}

impl std::fmt::Display for MBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl std::error::Error for MBError {}
