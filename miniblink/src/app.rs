use std::ffi::{CString, OsStr};

use miniblink_sys::Library;

use crate::{
    call_api,
    proxy::ProxyConfig,
    util::SafeCString,
    value::{JsExecState, JsValue},
    LIB,
};

/// Initialize miniblink from `path`. Panic if failed to initialize. See wkeInitialize.
pub fn init<P: AsRef<OsStr>>(path: P) {
    let lib =
        LIB.get_or_init(|| unsafe { Library::new(path).expect("Failed to initialize miniblink") });
    unsafe { lib.wkeInitialize() };
}

/// Run the miniblink message loop. See wkeRunMessageLoop.
pub fn run_message_loop() {
    unsafe {
        call_api().wkeRunMessageLoop();
    }
}

/// Bind function to global `window` object. See wkeJsBindFunction.
pub fn bind(name: &str, func: impl Fn(String) -> String + 'static) {
    unsafe extern "C" fn shim(
        es: miniblink_sys::jsExecState,
        param: *mut std::os::raw::c_void,
    ) -> miniblink_sys::jsValue {
        let es = JsExecState { inner: es };
        let arg = es.arg(0).to_string(es);
        let cb = param as *mut Box<dyn Fn(String) -> String>;
        let f = &mut **cb;

        if let Ok(r) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(arg))) {
            JsValue::new_string(es, &r).as_ptr()
        } else {
            JsValue::new_null().as_ptr()
        }
    }

    let param: *mut Box<dyn Fn(String) -> String> = Box::into_raw(Box::new(Box::new(func)));

    unsafe {
        call_api().wkeJsBindFunction(
            CString::safe_new(name).into_raw(),
            Some(shim),
            param as _,
            1,
        )
    }
}


/// Set the global proxy. See wkeSetProxy.
pub fn set_proxy(config: &ProxyConfig) {
    unsafe { call_api().wkeSetProxy(&config.to_wke_proxy()) }
}
