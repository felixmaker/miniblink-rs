use std::ffi::CString;


#[doc(hidden)]
/// A helper trait to get CStrings from Strings without panicking
/// https://github.com/fltk-rs/fltk-rs/blob/b43f8889c82c419deb3cd909e202d04b4c51f34f/fltk/src/utils/mod.rs#L13
pub trait SafeCString {
    /// Get CStrings from Strings without panicking
    fn safe_new(s: &str) -> CString;
}

impl SafeCString for CString {
    fn safe_new(s: &str) -> CString {
        match CString::new(s) {
            Ok(v) => v,
            Err(r) => {
                let i = r.nul_position();
                CString::new(&r.into_vec()[0..i]).unwrap()
            }
        }
    }
}
