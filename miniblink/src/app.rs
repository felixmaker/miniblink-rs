use std::ffi::{CString, OsStr, OsString};

use miniblink_sys::Library;

use crate::{
    call_api_or_panic,
    error::{MBError, MBResult},
    handler,
    proxy::ProxyConfig,
    util::SafeCString,
    value::{JsExecState, JsValue, MBExecStateValue},
    LIB,
};

const DEFAULT_MINIBLINK_LIB: &'static str = "node.dll";

/// App Attributes, used to build [`App`]
#[allow(missing_docs)]
#[derive(Default)]
pub struct AppAttributes {
    pub lib_path: Option<OsString>,
    pub dpi_support: bool,
    pub proxy_config: Option<ProxyConfig>,
}

/// App Builder, used to build [`App`]
/// 
/// Note: make sure to call [`App::init`] before using miniblink
#[allow(missing_docs)]
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

/// Wrapper to wke global functions, like wkeInitialize et al.
pub struct App {}

impl App {
    pub(crate) fn new(attrs: AppAttributes) -> MBResult<Self> {
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
    pub fn init<P>(path: P) -> MBResult<Self>
    where
        P: AsRef<OsStr>,
    {
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
    pub fn js_bind_function<F>(&self, name: &str, func: F, arg_count: u32)
    where
        F: Fn(JsExecState) -> MBResult<JsValue> + 'static,
    {
        let name = CString::safe_new(name);
        let param: *mut F = Box::into_raw(Box::new(func));

        unsafe {
            call_api_or_panic().wkeJsBindFunction(
                name.as_ptr(),
                Some(handler::js_native_function_handler::<F>),
                param as _,
                arg_count,
            )
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

/// Extend api for [`App`].
/// 
/// Note: make sure to call [`App::init`] or [`AppBuilder::build`] before using miniblink.
pub trait AppExt {
    /// Bind function like `P1 -> R` to global window object.
    fn bind<P1, T, F>(&self, name: &str, func: F)
    where
        F: Fn(P1) -> MBResult<T> + 'static,
        JsExecState: MBExecStateValue<P1> + MBExecStateValue<T>;

    /// Bind function like `P1, P2 -> R` to global window object.
    fn bind2<P1, P2, T, F>(&self, name: &str, func: F)
    where
        F: Fn(P1, P2) -> MBResult<T> + 'static,
        JsExecState: MBExecStateValue<P1> + MBExecStateValue<P2> + MBExecStateValue<T>;
}

impl AppExt for App {
    fn bind<P1, T, F>(&self, name: &str, func: F)
    where
        F: Fn(P1) -> MBResult<T> + 'static,
        JsExecState: MBExecStateValue<P1> + MBExecStateValue<T>,
    {
        self.js_bind_function(
            name,
            move |es| es.js_value(func(es.arg_value(0).unwrap())?),
            1,
        );
    }

    fn bind2<P1, P2, T, F>(&self, name: &str, func: F)
    where
        F: Fn(P1, P2) -> MBResult<T> + 'static,
        JsExecState: MBExecStateValue<P1> + MBExecStateValue<P2> + MBExecStateValue<T>,
    {
        self.js_bind_function(
            name,
            move |es| es.js_value(func(es.arg_value(0).unwrap(), es.arg_value(1).unwrap())?),
            2,
        );
    }
}
