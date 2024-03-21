use std::ffi::CString;

use crate::proxy::ProxyConfig;
use crate::util::SafeCString;
use crate::value::JsValue;
use crate::{handler, API};

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
    /// Whether the WebView window should be visible.
    pub visible: bool,

    /// Whether the WebView should have a custom user-agent.
    pub user_agent: Option<String>,

    /// The webview bounds. Defaults to `x: 0, y: 0, width: 200, height: 200`.
    /// This is only effective if the webview was created by [`WebView::new_as_child`] or [`WebViewBuilder::new_as_child`]
    /// or on Linux, if was created by [`WebViewExtUnix::new_gtk`] or [`WebViewBuilderExtUnix::new_gtk`] with [`gtk::Fixed`].
    pub bounds: Option<Rect>,

    /// Set a proxy configuration for the webview. Supports HTTP CONNECT and SOCKSv4, SOCKSv4A, SOCKSv5, Socks5Hostname proxies
    pub proxy_config: Option<ProxyConfig>,

    /// Whether load the provided URL to [`WebView`].
    pub url: Option<String>,

    /// Whether load the provided html string to [`WebView`].
    /// This will be ignored if the `url` is provided.
    pub html: Option<String>,

    /// A navigation handler to decide if incoming url is allowed to navigate.
    ///
    /// The closure take a `String` parameter as url and returns a `bool` to determine whether the navigation should happen.
    /// `true` allows to navigate and `false` does not.
    pub on_navigation_handler: Option<Box<dyn FnMut(&mut WebView, NavigationType, String) -> bool>>,

    /// A download started handler to manage incoming downloads.
    ///
    /// The closure takes a `String` as the url being downloaded from and  returns a `bool` to allow or deny the download.
    pub on_download_handler: Option<Box<dyn FnMut(&mut WebView, String) -> bool>>,

    /// Set a handler closure to process the change of the webview's document title.
    pub on_title_changed_handler: Option<Box<dyn FnMut(&mut WebView, String) -> bool>>,

    /// Set a handler closure on document ready.
    pub on_document_ready_handler: Option<Box<dyn FnMut(&mut WebView)>>,

    /// Set a handler closure on window closing.
    pub on_window_closing_handler: Option<Box<dyn FnMut(&mut WebView) -> bool>>,
}

impl Default for WebViewAttributes {
    fn default() -> Self {
        Self {
            visible: false,
            user_agent: None,
            proxy_config: None,
            bounds: Some(Rect {
                x: 0,
                y: 0,
                width: 200,
                height: 200,
            }),
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
pub struct WebViewBuilder {
    pub attrs: WebViewAttributes,
}

impl WebViewBuilder {
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

    /// Set a proxy configuration for the webview. Supports HTTP CONNECT and SOCKSv4, SOCKSv4A, SOCKSv5, Socks5Hostname proxies
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

    /// Consume the builder and create the [`WebView`].
    pub fn build(self) -> WebView {
        WebView::new(self.attrs)
    }
}

type InnerWebView = miniblink_sys::wkeWebView;

pub struct WebView {
    pub(crate) webview: InnerWebView,
}

impl WebView {
    fn new(attributes: WebViewAttributes) -> Self {
        let bounds = attributes.bounds.unwrap_or(Rect::default());
        let webview = WebView::create_window(bounds);

        if let Some(user_agent) = attributes.user_agent {
            webview.set_user_agent(user_agent.as_str());
        }

        if let Some(html) = attributes.html {
            webview.load_html(html.as_str());
        }

        if let Some(url) = attributes.url {
            webview.load_url(url.as_str());
        }

        if let Some(on_navigation_handler) = attributes.on_navigation_handler {
            webview.on_navigation(Box::into_raw(Box::new(on_navigation_handler)));
        }

        if let Some(on_title_changed_handler) = attributes.on_title_changed_handler {
            webview.on_title_changed(Box::into_raw(Box::new(on_title_changed_handler)));
        }

        if let Some(on_window_closing_handler) = attributes.on_window_closing_handler {
            webview.on_window_closing(Box::into_raw(Box::new(on_window_closing_handler)));
        }

        if let Some(on_download_handler) = attributes.on_download_handler {
            webview.on_download(Box::into_raw(Box::new(on_download_handler)));
        }

        if let Some(on_document_ready_handler) = attributes.on_document_ready_handler {
            webview.on_document_ready(Box::into_raw(Box::new(on_document_ready_handler)));
        }

        webview.set_visible(attributes.visible);

        webview
    }

    fn create_window(bounds: Rect) -> Self {
        let window = unsafe {
            API.wkeCreateWebWindow(
                miniblink_sys::wkeWindowType::WKE_WINDOW_TYPE_POPUP,
                std::ptr::null_mut(),
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
            )
        };

        Self { webview: window }
    }

    pub fn set_user_agent(&self, user_agent: &str) {
        unsafe {
            API.wkeSetUserAgent(self.webview, CString::safe_new(user_agent).into_raw());
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            API.wkeShowWindow(self.webview, visible);
        }
    }

    /// Load the provided HTML string when the builder calling [`WebViewBuilder::build`] to create the [`WebView`].
    /// This will be ignored if `url` is provided.
    pub fn load_html(&self, html: &str) {
        unsafe {
            API.wkeLoadHTML(self.webview, CString::safe_new(html).into_raw());
        }
    }

    pub fn load_url(&self, url: &str) {
        unsafe {
            API.wkeLoadURL(self.webview, CString::safe_new(url).into_raw());
        }
    }

    pub fn run_js(&self, script: &str) -> JsValue {
        let js_value = unsafe { API.wkeRunJS(self.webview, CString::safe_new(script).into_raw()) };
        JsValue { inner: js_value }
    }

    fn on_navigation(
        &self,
        callback: *mut Box<dyn FnMut(&mut WebView, NavigationType, String) -> bool>,
    ) {
        unsafe {
            API.wkeOnNavigation(
                self.webview,
                Some(handler::navigation_handler),
                callback as *mut _,
            );
        }
    }

    fn on_title_changed(&self, callback: *mut Box<dyn FnMut(&mut WebView, String) -> bool>) {
        unsafe {
            API.wkeOnTitleChanged(
                self.webview,
                Some(handler::wkestring_handler),
                callback as *mut _,
            );
        }
    }

    fn on_download(&self, callback: *mut Box<dyn FnMut(&mut WebView, String) -> bool>) {
        unsafe {
            API.wkeOnDownload(
                self.webview,
                Some(handler::cstr_to_bool_handler),
                callback as *mut _,
            );
        }
    }

    fn on_document_ready(&self, callback: *mut Box<dyn FnMut(&mut WebView)>) {
        unsafe {
            API.wkeOnDocumentReady(
                self.webview,
                Some(handler::void_handler),
                callback as *mut _,
            );
        }
    }

    fn on_window_closing(&self, callback: *mut Box<dyn FnMut(&mut WebView) -> bool>) {
        unsafe {
            API.wkeOnWindowClosing(
                self.webview,
                Some(handler::void_to_bool_handler),
                callback as *mut _,
            );
        }
    }
}

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
