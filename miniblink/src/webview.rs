use std::ffi::{CStr, CString};

use miniblink_sys::{wkeWindowType, HWND};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

use crate::error::{MBError, MBResult};
use crate::proxy::ProxyConfig;
use crate::util::SafeCString;
use crate::value::{JsValue, MBExecStateValue};

use crate::{call_api, call_api_or_panic, handler};

/// A rectangular region.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// x coordinate of top left corner
    pub x: i32,
    /// y coordinate of top left corner
    pub y: i32,
    /// width
    pub width: i32,
    /// height
    pub height: i32,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 200,
            height: 200,
        }
    }
}

/// WebView Attributes
pub struct WebViewAttributes {
    pub visible: bool,
    pub user_agent: Option<String>,
    pub bounds: Option<Rect>,
    pub proxy_config: Option<ProxyConfig>,
    pub url: Option<String>,
    pub html: Option<String>,

    pub on_navigation_handler: Option<Box<dyn FnMut(&mut WebView, NavigationType, String) -> bool>>,
    pub on_download_handler: Option<Box<dyn FnMut(&mut WebView, String) -> bool>>,
    pub on_title_changed_handler: Option<Box<dyn FnMut(&mut WebView, String)>>,
    pub on_document_ready_handler: Option<Box<dyn FnMut(&mut WebView)>>,

    // Window params
    pub window_title: Option<String>,
    pub on_window_closing_handler: Option<Box<dyn FnMut(&mut WebView) -> bool>>,
}

impl Default for WebViewAttributes {
    fn default() -> Self {
        Self {
            visible: true,
            user_agent: None,
            proxy_config: None,
            bounds: Some(Rect::default()),
            url: None,
            html: None,
            on_navigation_handler: None,
            on_download_handler: None,
            on_title_changed_handler: None,
            on_document_ready_handler: None,

            window_title: None,
            on_window_closing_handler: None,
        }
    }
}

#[derive(Default)]
pub struct WebViewBuilder<'a> {
    pub attrs: WebViewAttributes,
    hwnd: Option<&'a dyn HasWindowHandle>,
}

impl<'a> WebViewBuilder<'a> {
    /// Create [`WebViewBuilder`] as a child window inside the provided [`HasWindowHandle`]
    pub fn with_parent(mut self, parent: &'a impl HasWindowHandle) -> Self {
        self.hwnd = Some(parent);
        self
    }

    /// Set a custom [user-agent](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent) for the WebView.
    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.attrs.user_agent = Some(user_agent.to_string());
        self
    }

    /// Sets whether the WebView should be visible or not.
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.attrs.visible = visible;
        self
    }

    /// Set a proxy configuration for the webview. Supports HTTP CONNECT and SOCKSv4, SOCKSv4A, SOCKSv5, SOCKSv5HOSTNAME proxies
    pub fn with_proxy_config(mut self, configuration: ProxyConfig) -> Self {
        self.attrs.proxy_config = Some(configuration);
        self
    }

    /// Specify the webview position relative to its parent. Defaults to `x: 0, y: 0, width: 200, height: 200`.
    pub fn with_bounds(mut self, bounds: Rect) -> Self {
        self.attrs.bounds.replace(bounds);
        self
    }

    /// Load the provided URL when the builder calling [`WebViewBuilder::build`] to create the [`WebView`].
    /// The provided URL must be valid.
    pub fn with_url<S>(mut self, url: S) -> Self
    where
        S: Into<String>,
    {
        self.attrs.url = Some(url.into());
        self
    }

    /// Load the provided HTML string when the builder calling [`WebViewBuilder::build`] to create the [`WebView`].
    /// This will be ignored if `url` is provided.
    pub fn with_html<S>(mut self, html: S) -> Self
    where
        S: Into<String>,
    {
        self.attrs.html = Some(html.into());
        self
    }

    /// Set a navigation handler to decide if incoming url is allowed to navigate.
    ///
    /// The closure take a `String` parameter as url and returns a `bool` to determine whether the navigation should happen.
    /// `true` allows to navigate and `false` does not.
    pub fn with_on_navigation_handler<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&mut WebView, NavigationType, String) -> bool + 'static,
    {
        self.attrs.on_navigation_handler = Some(Box::new(callback));
        self
    }

    /// Set a download started handler to manage incoming downloads.
    ///
    /// The closure takes a `String` as the url being downloaded from and  returns a `bool` to allow or deny the download.
    pub fn with_on_download_handler<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&mut WebView, String) -> bool + 'static,
    {
        self.attrs.on_download_handler = Some(Box::new(callback));
        self
    }

    /// Set a handler closure to process the change of the webview's document title.
    pub fn with_on_title_changed_handler<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&mut WebView, String) + 'static,
    {
        self.attrs.on_title_changed_handler = Some(Box::new(callback));
        self
    }

    /// Set a handler closure on document ready.
    pub fn with_on_document_ready_handler<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&mut WebView) + 'static,
    {
        self.attrs.on_document_ready_handler = Some(Box::new(callback));
        self
    }

    /// Set a handler closure on window closing.
    pub fn with_on_window_closing_handler<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&mut WebView) -> bool + 'static,
    {
        self.attrs.on_window_closing_handler = Some(Box::new(callback));
        self
    }

    /// Set the title of native window.
    pub fn with_window_title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.attrs.window_title = Some(title.into());
        self
    }

    /// Consume the builder and create the [`WebView`].
    pub fn build(self) -> MBResult<WebView> {
        if let Some(hwnd) = &self.hwnd {
            WebView::new_as_child(hwnd, self.attrs)
        } else {
            WebView::new(self.attrs)
        }
    }
}

type InnerWebView = miniblink_sys::wkeWebView;

pub struct WebView {
    pub(crate) webview: InnerWebView,
}

impl WebView {
    fn new(attributes: WebViewAttributes) -> MBResult<Self> {
        let bounds = attributes.bounds.unwrap_or(Rect::default());
        let webview = WebView::create_popup_window(bounds)?;

        webview.apply_attributes(attributes);
        Ok(webview)
    }

    fn new_as_child(hwnd: &impl HasWindowHandle, attributes: WebViewAttributes) -> MBResult<Self> {
        let bounds = attributes.bounds.unwrap_or(Rect::default());

        let webview = {
            match hwnd.window_handle().map(|x| x.as_raw()) {
                Ok(RawWindowHandle::Win32(handle)) => {
                    WebView::create_control_window(isize::from(handle.hwnd) as HWND, bounds)
                }
                _ => Err(MBError::UnsupportedPlatform),
            }
        }?;

        webview.apply_attributes(attributes);

        Ok(webview)
    }

    fn apply_attributes(&self, attributes: WebViewAttributes) {
        if let Some(proxy_config) = attributes.proxy_config {
            self.set_proxy(&proxy_config);
        }

        if let Some(window_title) = attributes.window_title {
            self.set_window_title(&window_title);
        }

        if let Some(user_agent) = attributes.user_agent {
            self.set_user_agent(user_agent.as_str());
        }

        if let Some(html) = attributes.html {
            self.load_html(html.as_str());
        }

        if let Some(url) = attributes.url {
            self.load_url(url.as_str());
        }

        if let Some(on_navigation_handler) = attributes.on_navigation_handler {
            self.on_navigation(on_navigation_handler);
        }

        if let Some(on_title_changed_handler) = attributes.on_title_changed_handler {
            self.on_title_changed(on_title_changed_handler);
        }

        if let Some(on_window_closing_handler) = attributes.on_window_closing_handler {
            self.on_window_closing(on_window_closing_handler);
        }

        if let Some(on_download_handler) = attributes.on_download_handler {
            self.on_download(on_download_handler);
        }

        if let Some(on_document_ready_handler) = attributes.on_document_ready_handler {
            self.on_document_ready(on_document_ready_handler);
        }

        self.set_visible(attributes.visible);
    }

    fn create_popup_window(bounds: Rect) -> MBResult<Self> {
        let window = unsafe {
            call_api()?.wkeCreateWebWindow(
                wkeWindowType::WKE_WINDOW_TYPE_POPUP,
                std::ptr::null_mut(),
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
            )
        };

        Ok(Self { webview: window })
    }

    fn create_control_window(parent: HWND, bounds: Rect) -> MBResult<Self> {
        let window = unsafe {
            call_api()?.wkeCreateWebWindow(
                wkeWindowType::WKE_WINDOW_TYPE_CONTROL,
                parent,
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
            )
        };

        Ok(Self { webview: window })
    }

    /// Set the title of native window. See wkeSetWindowTitle.
    pub fn set_window_title(&self, title: &str) {
        let title = CString::safe_new(title);
        unsafe {
            call_api_or_panic().wkeSetWindowTitle(self.webview, title.as_ptr());
        }
    }

    /// Set the user agent. See wkeSetUserAgent.
    pub fn set_user_agent(&self, user_agent: &str) {
        let user_agent = CString::safe_new(user_agent);
        unsafe {
            call_api_or_panic().wkeSetUserAgent(self.webview, user_agent.as_ptr());
        }
    }

    /// Set the visibility. See wkeShowWindow.
    pub fn set_visible(&self, visible: bool) {
        unsafe {
            call_api_or_panic().wkeShowWindow(self.webview, visible);
        }
    }

    /// Set the proxy of current webview. Use `app::set_proxy`` to set global proxy. See wkeSetViewProxy.
    pub fn set_proxy(&self, proxy: &ProxyConfig) {
        unsafe {
            call_api_or_panic()
                .wkeSetViewProxy(self.webview, Box::into_raw(Box::new(proxy.to_wke_proxy())));
        }
    }

    /// Load the provided HTML. See wkeLoadHTML.
    pub fn load_html(&self, html: &str) {
        let html = CString::safe_new(html);
        unsafe {
            call_api_or_panic().wkeLoadHTML(self.webview, html.as_ptr());
        }
    }

    /// Set the size. See wkeResize.
    pub fn set_size(&self, width: u32, height: u32) {
        unsafe {
            call_api_or_panic().wkeResize(self.webview, width as i32, height as i32);
        }
    }

    /// Load the provided URL. See wkeLoadURL.
    pub fn load_url(&self, url: &str) {
        let url = CString::safe_new(url);
        unsafe {
            call_api_or_panic().wkeLoadURL(self.webview, url.as_ptr());
        }
    }

    /// Run the provided script. See wkeRunJS.
    pub fn run_js<T>(&self, script: &str) -> MBResult<T>
    where
        JsValue: MBExecStateValue<T>,
    {
        let script = CString::safe_new(script);
        let js_value = JsValue {
            inner: unsafe { call_api_or_panic().wkeRunJS(self.webview, script.as_ptr()) },
        };
        let es = crate::value::JsExecState {
            inner: unsafe { call_api_or_panic().wkeGlobalExec(self.webview) },
        };
        js_value.to_value(es)
    }

    /// Get the cookie from web page. See wkeGetCookie.
    pub fn get_cookie(&self) -> String {
        let cstr = unsafe {
            let cstr = call_api_or_panic().wkeGetCookie(self.webview);
            let cstr = CStr::from_ptr(cstr);
            cstr
        };
        cstr.to_string_lossy().to_string()
    }

    /// Set the cookie to url. See wkeSetCookie.
    ///
    /// Cookie is a curl cookie, like "PERSONALIZE=123;expires=Monday, 13-Jun-2022 03:04:55 GMT; domain=.fidelity.com; path=/; secure"
    pub fn set_cookie(&self, url: &str, cookie: &str) {
        let url = CString::safe_new(url);
        let cookie = CString::safe_new(cookie);
        unsafe {
            call_api_or_panic().wkeSetCookie(self.webview, url.as_ptr(), cookie.as_ptr());
        }
    }

    fn on_navigation<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, NavigationType, String) -> bool,
    {
        let callback: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnNavigation(
                self.webview,
                Some(handler::navigation_handler::<F>),
                callback as _,
            );
        }
    }

    fn on_title_changed<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String),
    {
        let callback: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnTitleChanged(
                self.webview,
                Some(handler::wkestring_handler::<F>),
                callback as *mut _,
            );
        }
    }

    fn on_download<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String) -> bool,
    {
        let callback: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnDownload(
                self.webview,
                Some(handler::cstr_to_bool_handler::<F>),
                callback as *mut _,
            );
        }
    }

    fn on_document_ready<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView),
    {
        let callback: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnDocumentReady(
                self.webview,
                Some(handler::void_handler::<F>),
                callback as *mut _,
            );
        }
    }

    fn on_window_closing<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView) -> bool + 'static,
    {
        let callback: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnWindowClosing(
                self.webview,
                Some(handler::void_to_bool_handler::<F>),
                callback as _,
            );
        }
    }
}

/// Navigation Type. See wkeNavigationType.
pub enum NavigationType {
    LinkClick,
    FormSubmitte,
    BackForward,
    Reload,
    FormResubmit,
    Other,
}

impl From<miniblink_sys::wkeNavigationType> for NavigationType {
    fn from(value: miniblink_sys::wkeNavigationType) -> Self {
        match value {
            miniblink_sys::wkeNavigationType::WKE_NAVIGATION_TYPE_LINKCLICK => {
                NavigationType::LinkClick
            }
            miniblink_sys::wkeNavigationType::WKE_NAVIGATION_TYPE_FORMRESUBMITT => {
                NavigationType::FormSubmitte
            }
            miniblink_sys::wkeNavigationType::WKE_NAVIGATION_TYPE_BACKFORWARD => {
                NavigationType::BackForward
            }
            miniblink_sys::wkeNavigationType::WKE_NAVIGATION_TYPE_RELOAD => NavigationType::Reload,
            miniblink_sys::wkeNavigationType::WKE_NAVIGATION_TYPE_FORMSUBMITTE => {
                NavigationType::FormResubmit
            }
            _ => NavigationType::Other,
        }
    }
}
