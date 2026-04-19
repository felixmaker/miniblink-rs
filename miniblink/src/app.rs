use std::{
    ffi::{c_void, OsStr},
    panic::AssertUnwindSafe,
};

use miniblink_sys::Library;

use crate::{
    call_api_or_panic,
    error::{MBError, MBResult},
    LIB,
};

/// Enable high dpi support.
pub fn enable_high_dpi_support() {
    unsafe { call_api_or_panic().mbEnableHighDPISupport() }
}

/// Run message loop provided by miniblink. Note: You may write your own message loop.
pub fn run_message_loop() {
    unsafe { call_api_or_panic().mbRunMessageLoop() }
}

/// Initialize miniblink from `path`. Panic if failed to initialize.
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

pub(crate) fn invoke_command_sync<F, R>(handler: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send,
{
    struct Param<R> {
        sender: std::sync::mpsc::Sender<R>,
        handler: Box<dyn FnOnce() -> R + Send + 'static>,
    }

    extern "system" fn callback<R>(param: *mut c_void, _: *mut c_void) {
        let param = unsafe { Box::from_raw(param as *mut Param<R>) };
        let handler = param.handler;
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| handler()));
        let result = result.unwrap();
        param
            .sender
            .send(result)
            .expect("Failed to send result from UI thread");
    }

    let (sender, receiver) = std::sync::mpsc::channel::<R>();
    let handler = Box::new(handler);
    let param = Box::into_raw(Box::new(Param { sender, handler }));

    unsafe {
        call_api_or_panic().mbCallUiThreadSync(
            Some(callback::<R>),
            param as *mut c_void,
            std::ptr::null_mut(),
        )
    };

    receiver
        .recv()
        .expect("Failed to receive result from UI thread")
}
