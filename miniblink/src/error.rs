use crate::value::JsValue;

#[cfg(feature = "serde")]
use  crate::value::JsType;

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
    #[cfg(feature = "serde")]
    UnsupportedType(JsType, JsType),
    #[cfg(feature = "serde")]
    SerdeMessage(String),
    #[cfg(feature = "serde")]
    FailedToConvert(String, String),
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
            #[cfg(feature = "serde")]
            UnsupportedType(expected, but) => format!("Except {}, but {} provided!", expected, but),
            #[cfg(feature = "serde")]
            SerdeMessage(msg) => format!("SerdeMessage: {msg}"),
            #[cfg(feature = "serde")]
            FailedToConvert(from, to) => format!("Failed to convert from {from} to {to}"),
        }
    }
}

impl std::fmt::Display for MBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl std::error::Error for MBError {}

#[cfg(feature = "serde")]
impl serde::de::Error for MBError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::SerdeMessage(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Error for MBError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::SerdeMessage(msg.to_string())
    }
}
