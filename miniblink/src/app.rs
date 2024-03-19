use std::ffi::CString;

use crate::{
    util::SafeCString,
    value::{JsExecState, JsValue},
    API,
};

pub fn run_message_loop() {
    unsafe {
        API.wkeRunMessageLoop();
    }
}

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
        API.wkeJsBindFunction(
            CString::safe_new(name).into_raw(),
            Some(shim),
            param as _,
            1,
        )
    }
}
