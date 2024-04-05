use std::ffi::{CStr, CString, OsStr};

use miniblink_sys::Library;

use crate::{
    call_api_or_panic,
    error::{MBError, MBResult},
    types::{JsExecState, JsValue, MBExecStateValue, Proxy},
    util::SafeCString,
    LIB,
};

// const DEFAULT_MINIBLINK_LIB: &'static str = "node.dll";

/// See wkeEnableHighDPISupport.
pub fn enable_high_dpi_support() {
    unsafe { call_api_or_panic().wkeEnableHighDPISupport() }
}

/// See wkeRunMessageLoop.
pub fn run_message_loop() {
    unsafe { call_api_or_panic().wkeRunMessageLoop() }
}

/// 设置整个mb的代码。此句是全局生效
pub fn set_proxy(config: &Proxy) {
    let config = config.to_wke_proxy();
    unsafe { call_api_or_panic().wkeSetProxy(&config) }
}

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
        unsafe { lib.wkeInitialize() };
        Ok(lib)
    }
}

/// Bind function to global `window` object. See `wkeJsBindFunction`.
/// 和jsBindFunction功能类似，但更方便一点，可以传一个param做自定义数据。
/// 此接口和wkeJsBindFunction必须在webview创建前调用
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

/// Wraps to global functions
pub struct App {}

impl App {
    /// Get the version of miniblink
    ///
    /// 获取目前api版本号
    pub fn version() -> u32 {
        unsafe { call_api_or_panic().wkeVersion() }
    }

    /// Get the version string of miniblink
    ///
    /// 获取版本字符串
    pub fn version_string() -> String {
        let version = unsafe { call_api_or_panic().wkeGetVersionString() };
        assert!(!version.is_null());
        unsafe { CStr::from_ptr(version).to_string_lossy().to_string() }
    }

    /// 设置一些配置项
    pub fn wke_configure() {
        todo!()
    }
    /// 对js winows绑定一个属性访问器，在js里windows.XXX这种形式调用时，fn会被调用
    pub fn bind_getter() {
        todo!()
    }
    /// 对js winows绑定一个属性设置器。
    pub fn bind_setter() {
        todo!()
    }
    /// 通过访问器visitor访问所有cookie。
    pub fn visit_all_cookie() {
        todo!()
    }
    /// Perform operation on `cookie` using curl embedded in miniblink.
    /// command：
    /// wkeCookieCommandClearAllCookies: 内部实际执行了curl_easy_setopt(curl, CURLOPT_COOKIELIST, "ALL");
    /// wkeCookieCommandClearSessionCookies: curl_easy_setopt(curl, CURLOPT_COOKIELIST, "SESS");
    /// wkeCookieCommandFlushCookiesToFile: curl_easy_setopt(curl, CURLOPT_COOKIELIST, "FLUSH");
    /// wkeCookieCommandReloadCookiesFromFile: curl_easy_setopt(curl, CURLOPT_COOKIELIST, "RELOAD");
    /// 注意：这个接口只是调用curl设置命令，并不会去修改js里的内容
    pub fn perform_cookie_command() {
        todo!()
    }
}
