use std::ffi::OsStr;

use miniblink_sys::Library;

use crate::{call_api_or_panic, error::{MBError, MBResult}, LIB};


/// Enable high dpi support.
pub fn enable_high_dpi_support() {
    unsafe { call_api_or_panic().mbEnableHighDPISupport() }
}

/// Run message loop provided by miniblink. Note: You may write your own message loop.
pub fn run_message_loop() {
    unsafe { call_api_or_panic().mbRunMessageLoop() }
}

/// Initialize miniblink from `path`. Panic if failed to initialize. See `wkeInitialize`.
pub fn init<P>(path: P) -> MBResult<&'static Library>
where
    P: AsRef<OsStr>,
{
    if let Some(lib) = LIB.get() {
        Ok(lib)
    } else {
        let lib =
            unsafe { Library::new(path) }.map_err(|e| MBError::LibraryUnloaded(e.to_string()))?;
        let lib = LIB.get_or_init(|| lib);
        unsafe {
            let settings = std::mem::zeroed();
            lib.mbInit(settings)
        };
        Ok(lib)
    }
}
