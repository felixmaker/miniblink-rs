use std::ffi::{CString, OsStr, OsString};

use miniblink_sys::Library;

use crate::{
    call_api_or_panic,
    error::{MBError, MBResult},
    proxy::ProxyConfig,
    util::SafeCString,
    value::{JsExecState, JsValue, MBExecStateValue},
    LIB,
};

const DEFAULT_MINIBLINK_LIB: &'static str = "node.dll";

#[derive(Default)]
pub struct AppAttributes {
    pub lib_path: Option<OsString>,
    pub dpi_support: bool,
    pub proxy_config: Option<ProxyConfig>,
}

pub struct AppBuilder {
    pub attrs: AppAttributes,
}

impl Default for AppBuilder {
    fn default() -> Self {
        let attrs = AppAttributes {
            lib_path: Some(DEFAULT_MINIBLINK_LIB.into()),
            ..Default::default()
        };
        Self { attrs }
    }
}

impl AppBuilder {
    /// Set the location of miniblink shared library.
    pub fn with_lib_path<P: AsRef<OsStr>>(mut self, lib: P) -> Self {
        self.attrs.lib_path = Some(lib.as_ref().to_owned());
        self
    }

    /// Set a global proxy configuration.
    pub fn with_proxy_config(mut self, configuration: ProxyConfig) -> Self {
        self.attrs.proxy_config = Some(configuration);
        self
    }

    /// Enable high DPI support
    pub fn with_dpi_support(mut self, dpi_support: bool) -> Self {
        self.attrs.dpi_support = dpi_support;
        self
    }

    /// Consume the builder and create the [`App`].
    pub fn build(self) -> MBResult<App> {
        App::new(self.attrs)
    }
}

pub struct App {}

impl App {
    fn new(attrs: AppAttributes) -> MBResult<Self> {
        let lib_path = attrs.lib_path.unwrap_or(DEFAULT_MINIBLINK_LIB.into());
        let app = Self::init(lib_path)?;

        if attrs.dpi_support {
            app.enable_dpi_support();
        }

        if let Some(proxy_config) = attrs.proxy_config {
            app.set_proxy(&proxy_config);
        }

        Ok(app)
    }

    /// Initialize miniblink from `path`. Panic if failed to initialize. See wkeInitialize.
    pub fn init<P: AsRef<OsStr>>(path: P) -> MBResult<Self> {
        let lib =
            unsafe { Library::new(path) }.map_err(|e| MBError::LibraryUnloaded(e.to_string()))?;
        let lib = LIB.get_or_init(|| lib);
        unsafe { lib.wkeInitialize() };
        Ok(Self {})
    }

    /// Run the miniblink message loop. See wkeRunMessageLoop.
    pub fn run_message_loop(&self) {
        unsafe {
            call_api_or_panic().wkeRunMessageLoop();
        }
    }

    /// Bind function to global `window` object. See wkeJsBindFunction.
    pub fn bind<P, T>(&self, name: &str, func: impl Fn(P) -> T + 'static)
    where
        JsValue: MBExecStateValue<P>,
        JsValue: MBExecStateValue<T>,
    {
        self.js_bind_function(
            name,
            move |es| {
                let arg = es.arg(0);
                JsValue::from_value(es, func(arg.to_value(es).unwrap()))
            },
            1,
        );
    }

    /// Bind function to global `window` object. See wkeJsBindFunction.
    fn js_bind_function(
        &self,
        name: &str,
        func: impl Fn(JsExecState) -> JsValue + 'static,
        arg_count: u32,
    ) {
        unsafe extern "C" fn shim(
            es: miniblink_sys::jsExecState,
            param: *mut std::os::raw::c_void,
        ) -> miniblink_sys::jsValue {
            let es = JsExecState { inner: es };
            let cb = param as *mut Box<dyn Fn(JsExecState) -> JsValue>;
            let f = &mut **cb;

            if let Ok(r) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(es))) {
                r.as_ptr()
            } else {
                JsValue::from_value(es, ()).as_ptr()
            }
        }

        let name = CString::safe_new(name);
        let param: *mut Box<dyn Fn(JsExecState) -> JsValue> =
            Box::into_raw(Box::new(Box::new(func)));

        unsafe {
            call_api_or_panic().wkeJsBindFunction(name.as_ptr(), Some(shim), param as _, arg_count)
        }
    }

    /// Set the global proxy. See wkeSetProxy.
    pub fn set_proxy(&self, config: &ProxyConfig) {
        unsafe { call_api_or_panic().wkeSetProxy(&config.to_wke_proxy()) }
    }

    /// Enable high DPI support. See wkeEnableHighDPISupport.
    pub fn enable_dpi_support(&self) {
        unsafe { call_api_or_panic().wkeEnableHighDPISupport() }
    }
}
