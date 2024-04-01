use std::ffi::{CString, OsStr};

use miniblink_sys::Library;

use crate::{
    bind_global, call_api_or_panic,
    error::{MBError, MBResult},
    types::{CProxy, JsExecState, JsValue, MBExecStateValue, Proxy},
    util::SafeCString,
    LIB,
};

bind_global! {
    wkeInitialize => _initialize();
    pub wkeSetProxy => set_proxy(config: &Proxy as CProxy);
    pub wkeEnableHighDPISupport => enable_high_dpi_support();
    pub wkeRunMessageLoop => run_message_loop();
    // pub jsGC => gc();
    // pub jsBindGetter => 
    // pub jsBindSetter
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

/// Experimantal app api
pub mod app_ext {
    use super::*;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! count_one {
        ($any: ident) => {
            1
        };
    }

    macro_rules! js_bind_function_ext {
        ($(
            $vis: vis $func: ident ($($param: ident),*)
        );*) => {
            $(
                #[doc=concat!("Js bind function, with params `", $(stringify!($param),)* "`")]
                $vis fn $func<$($param,)* T, F>(name: &str, func: F)
                where
                    F: Fn($($param,)*) -> MBResult<T> + 'static,
                    JsExecState: $(MBExecStateValue<$param> +)* MBExecStateValue<T>,
                {
                    #[allow(unused)]
                    use crate::types::JsExecStateExt;
                    js_bind_function(
                        name,
                        move |es| {
                            $(
                                #[allow(non_snake_case)]
                                let $param = es.arg_value(0).unwrap();
                            )*
                            es.js_value(func($($param,)*)?)
                        },
                        0 $(+ crate::count_one!($param))*,
                    );
                }
            )*
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
}
