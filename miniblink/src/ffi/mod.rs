/// In macros: param `x` as `y`
pub(crate) mod prefare_ffi;
/// In macros: from c to rust
pub(crate) mod from_ffi;
/// In macros: from rust to c
pub(crate) mod to_ffi;

pub(crate) use prefare_ffi::PrepareFFI;
pub(crate) use from_ffi::FromFFI;
pub(crate) use to_ffi::ToFFI;
