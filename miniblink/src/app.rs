use std::ffi::{CString, OsStr};

use miniblink_sys::Library;

use crate::{
    bind_global, call_api_or_panic,
    error::{MBError, MBResult},
    js_bind_function_ext,
    types::{JsExecState, JsValue, MBExecStateValue, Proxy, WindowType, HWND},
    util::SafeCString,
    webview::WebView,
    LIB,
};

bind_global! {
    wkeInitialize => _initialize();
    pub wkeSetProxy => set_proxy(config: &Proxy as CProxy);
    pub wkeEnableHighDPISupport => enable_high_dpi_support();
    pub wkeRunMessageLoop => run_message_loop();
    pub wkeCreateWebWindow => create_web_window(window_type: WindowType, handle: HWND, x: i32, y: i32, width: i32, height: i32) -> WebView
}

// const DEFAULT_MINIBLINK_LIB: &'static str = "node.dll";

/// Initialize miniblink from `path`. Panic if failed to initialize. See `wkeInitialize`.
pub fn initialize<P>(path: P) -> MBResult<&'static Library>
where
    P: AsRef<OsStr>,
{
    if let Some(lib) = LIB.get() {
        Ok(lib)
    } else {
        let lib =
            unsafe { Library::new(path) }.map_err(|e| MBError::LibraryUnloaded(e.to_string()))?;
        let lib = LIB.get_or_init(|| lib);
        _initialize();
        Ok(lib)
    }
}

/// Bind function to global `window` object. See `wkeJsBindFunction`.
pub fn js_bind_function<F>(name: &str, func: F, arg_count: u32)
where
    F: Fn(JsExecState) -> MBResult<JsValue> + 'static,
{
    unsafe extern "C" fn shim<F>(
        es: miniblink_sys::jsExecState,
        param: *mut std::os::raw::c_void,
    ) -> miniblink_sys::jsValue
    where
        F: Fn(JsExecState) -> MBResult<JsValue>,
    {
        let es = JsExecState::from_ptr(es);
        let cb = param as *mut F;
        let f = &mut *cb;

        if let Ok(Ok(r)) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(es))) {
            r.as_ptr()
        } else {
            es.null().as_ptr()
        }
    }

    let name = CString::safe_new(name);
    let param: *mut F = Box::into_raw(Box::new(func));

    unsafe {
        call_api_or_panic().wkeJsBindFunction(name.as_ptr(), Some(shim::<F>), param as _, arg_count)
    }
}

js_bind_function_ext! {
    pub bind(P1);
    pub bind0();
    pub bind1(P1);
    pub bind2(P1, P2);
    pub bind3(P1, P2, P3);
    pub bind4(P1, P2, P3, P4);
    pub bind5(P1, P2, P3, P4, P5);
    pub bind6(P1, P2, P3, P4, P5, P6)
}
