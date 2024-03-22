use std::{
    collections::HashMap,
    ffi::{CString, OsStr, OsString},
};

use miniblink_sys::Library;

use crate::{
    call_api,
    error::{MBError, MBResult},
    proxy::ProxyConfig,
    util::SafeCString,
    value::{JsExecState, JsValue},
    LIB,
};

const DEFAULT_MINIBLINK_LIB: &'static str = "node.dll";

#[derive(Default)]
pub struct AppAttributes {
    pub lib_path: Option<OsString>,
    pub proxy_config: Option<ProxyConfig>,
    pub js_bind: HashMap<String, Box<dyn Fn(String) -> String>>,
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

    /// Bind a javascript function to window object.
    pub fn with_js_bind(mut self, name: &str, func: impl Fn(String) -> String + 'static) -> Self {
        self.attrs.js_bind.insert(name.into(), Box::new(func));
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

        if let Some(proxy_config) = attrs.proxy_config {
            app.set_proxy(&proxy_config);
        }

        for (name, func) in attrs.js_bind {
            app.bind(&name, func);
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
            call_api().unwrap().wkeRunMessageLoop();
        }
    }

    /// Bind function to global `window` object. See wkeJsBindFunction.
    pub fn bind(&self, name: &str, func: impl Fn(String) -> String + 'static) {
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
            call_api().unwrap().wkeJsBindFunction(
                CString::safe_new(name).into_raw(),
                Some(shim),
                param as _,
                1,
            )
        }
    }

    /// Set the global proxy. See wkeSetProxy.
    pub fn set_proxy(&self, config: &ProxyConfig) {
        unsafe { call_api().unwrap().wkeSetProxy(&config.to_wke_proxy()) }
    }
}
