use std::ffi::CString;

use miniblink_sys::{wkeWindowType, HWND};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

use crate::error::{MBError, MBResult};
use crate::proxy::ProxyConfig;
use crate::util::SafeCString;
use crate::value::JsValue;

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

pub struct WebViewAttributes {
    pub visible: bool,
    pub user_agent: Option<String>,
    pub window_title: Option<String>,
    pub bounds: Option<Rect>,
    pub proxy_config: Option<ProxyConfig>,
    pub url: Option<String>,
    pub html: Option<String>,

    pub on_navigation_handler: Option<Box<dyn FnMut(&mut WebView, NavigationType, String) -> bool>>,
    pub on_download_handler: Option<Box<dyn FnMut(&mut WebView, String) -> bool>>,
    pub on_title_changed_handler: Option<Box<dyn FnMut(&mut WebView, String) -> bool>>,
    pub on_document_ready_handler: Option<Box<dyn FnMut(&mut WebView)>>,
    pub on_window_closing_handler: Option<Box<dyn FnMut(&mut WebView) -> bool>>,
}

impl Default for WebViewAttributes {
    fn default() -> Self {
        Self {
            visible: true,
            user_agent: None,
            window_title: None,
            proxy_config: None,
            bounds: Some(Rect::default()),
            url: None,
            html: None,
            on_navigation_handler: None,
            on_download_handler: None,
            on_title_changed_handler: None,
            on_document_ready_handler: None,
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
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.attrs.url = Some(url.into());
        self
    }

    /// Load the provided HTML string when the builder calling [`WebViewBuilder::build`] to create the [`WebView`].
    /// This will be ignored if `url` is provided.
    pub fn with_html(mut self, html: impl Into<String>) -> Self {
        self.attrs.html = Some(html.into());
        self
    }

    /// Set a navigation handler to decide if incoming url is allowed to navigate.
    ///
    /// The closure take a `String` parameter as url and returns a `bool` to determine whether the navigation should happen.
    /// `true` allows to navigate and `false` does not.
    pub fn with_on_navigation_handler(
        mut self,
        callback: impl FnMut(&mut WebView, NavigationType, String) -> bool + 'static,
    ) -> Self {
        self.attrs.on_navigation_handler = Some(Box::new(callback));
        self
    }

    /// Set a download started handler to manage incoming downloads.
    ///
    /// The closure takes a `String` as the url being downloaded from and  returns a `bool` to allow or deny the download.
    pub fn with_on_download_handler(
        mut self,
        callback: impl FnMut(&mut WebView, String) -> bool + 'static,
    ) -> Self {
        self.attrs.on_download_handler = Some(Box::new(callback));
        self
    }

    /// Set a handler closure to process the change of the webview's document title.
    pub fn with_on_title_changed_handler(
        mut self,
        callback: impl FnMut(&mut WebView, String) -> bool + 'static,
    ) -> Self {
        self.attrs.on_title_changed_handler = Some(Box::new(callback));
        self
    }

    /// Set a handler closure on document ready.
    pub fn with_on_document_ready_handler(
        mut self,
        callback: impl FnMut(&mut WebView) + 'static,
    ) -> Self {
        self.attrs.on_document_ready_handler = Some(Box::new(callback));
        self
    }

    /// Set a handler closure on window closing.
    pub fn with_on_window_closing_handler(
        mut self,
        callback: impl FnMut(&mut WebView) -> bool + 'static,
    ) -> Self {
        self.attrs.on_window_closing_handler = Some(Box::new(callback));
        self
    }

    /// Set the title of native window.
    pub fn with_window_title(mut self, title: impl Into<String>) -> Self {
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
            self.on_navigation(Box::into_raw(Box::new(on_navigation_handler)));
        }

        if let Some(on_title_changed_handler) = attributes.on_title_changed_handler {
            self.on_title_changed(Box::into_raw(Box::new(on_title_changed_handler)));
        }

        if let Some(on_window_closing_handler) = attributes.on_window_closing_handler {
            self.on_window_closing(Box::into_raw(Box::new(on_window_closing_handler)));
        }

        if let Some(on_download_handler) = attributes.on_download_handler {
            self.on_download(Box::into_raw(Box::new(on_download_handler)));
        }

        if let Some(on_document_ready_handler) = attributes.on_document_ready_handler {
            self.on_document_ready(Box::into_raw(Box::new(on_document_ready_handler)));
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
        unsafe {
            call_api_or_panic()
                .wkeSetWindowTitle(self.webview, CString::safe_new(title).into_raw());
        }
    }

    /// Set the user agent. See wkeSetUserAgent.
    pub fn set_user_agent(&self, user_agent: &str) {
        unsafe {
            call_api_or_panic()
                .wkeSetUserAgent(self.webview, CString::safe_new(user_agent).into_raw());
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
        unsafe {
            call_api_or_panic().wkeLoadHTML(self.webview, CString::safe_new(html).into_raw());
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
        unsafe {
            call_api_or_panic().wkeLoadURL(self.webview, CString::safe_new(url).into_raw());
        }
    }

    /// Run the provided script. See wkeRunJS.
    pub fn run_js(&self, script: &str) -> JsValue {
        let js_value = unsafe {
            call_api_or_panic().wkeRunJS(self.webview, CString::safe_new(script).into_raw())
        };
        JsValue { inner: js_value }
    }

    fn on_navigation(
        &self,
        callback: *mut Box<dyn FnMut(&mut WebView, NavigationType, String) -> bool>,
    ) {
        unsafe {
            call_api_or_panic().wkeOnNavigation(
                self.webview,
                Some(handler::navigation_handler),
                callback as *mut _,
            );
        }
    }

    fn on_title_changed(&self, callback: *mut Box<dyn FnMut(&mut WebView, String) -> bool>) {
        unsafe {
            call_api_or_panic().wkeOnTitleChanged(
                self.webview,
                Some(handler::wkestring_handler),
                callback as *mut _,
            );
        }
    }

    fn on_download(&self, callback: *mut Box<dyn FnMut(&mut WebView, String) -> bool>) {
        unsafe {
            call_api_or_panic().wkeOnDownload(
                self.webview,
                Some(handler::cstr_to_bool_handler),
                callback as *mut _,
            );
        }
    }

    fn on_document_ready(&self, callback: *mut Box<dyn FnMut(&mut WebView)>) {
        unsafe {
            call_api_or_panic().wkeOnDocumentReady(
                self.webview,
                Some(handler::void_handler),
                callback as *mut _,
            );
        }
    }

    fn on_window_closing(&self, callback: *mut Box<dyn FnMut(&mut WebView) -> bool>) {
        unsafe {
            call_api_or_panic().wkeOnWindowClosing(
                self.webview,
                Some(handler::void_to_bool_handler),
                callback as *mut _,
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
