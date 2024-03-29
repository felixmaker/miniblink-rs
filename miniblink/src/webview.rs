use std::ffi::{CStr, CString};

use miniblink_sys::{wkeWindowType, HWND};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

use crate::error::{MBError, MBResult};
use crate::macros::{FromFFI, ToFFI};
use crate::proxy::ProxyConfig;
use crate::util::SafeCString;
use crate::value::{JsExecState, JsValue, MBExecStateValue};

use crate::wstr::WkeStr;
use crate::{bind_handler, bind_target, call_api, call_api_or_panic};

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

#[allow(missing_docs)]
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

/// Builder used to build [`WebView`]
#[allow(missing_docs)]
#[derive(Default)]
pub struct WebViewBuilder<'a> {
    pub attrs: WebViewAttributes,
    hwnd: Option<&'a dyn HasWindowHandle>,
}

impl<'a> WebViewBuilder<'a> {
    /// Create [`WebViewBuilder`] as a child window inside the provided [`HasWindowHandle`]
    pub fn with_parent<H>(mut self, parent: &'a H) -> Self
    where
        H: HasWindowHandle,
    {
        self.hwnd = Some(parent);
        self
    }

    /// Set a custom [user-agent](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent) for the WebView.
    pub fn with_user_agent<S>(mut self, user_agent: S) -> Self
    where
        S: Into<String>,
    {
        self.attrs.user_agent = Some(user_agent.into());
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

/// Wrapper to [`miniblink_sys::wkeWebView`]
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

        self.show_window(attributes.visible);
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

    /// Run the provided script. See `wkeRunJS`.
    pub fn run_js<T>(&self, script: &str) -> MBResult<T>
    where
        JsExecState: MBExecStateValue<T>,
    {
        let script = CString::safe_new(script);
        let js_value = JsValue::from_ptr(unsafe {
            call_api_or_panic().wkeRunJS(self.webview, script.as_ptr())
        });
        let es = self.global_exec();
        es.value(js_value)
    }

    /// Get JsExecState. See `wkeGlobalExec`.
    pub fn global_exec(&self) -> JsExecState {
        JsExecState::from_ptr(unsafe { call_api_or_panic().wkeGlobalExec(self.webview) })
    }
}

/// Navigation Type. See `wkeNavigationType`.
#[allow(missing_docs)]
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

impl FromFFI<miniblink_sys::wkeNavigationType> for NavigationType {
    fn from(value: miniblink_sys::wkeNavigationType) -> Self {
        From::from(value)
    }
}

pub(crate) type CCStr = *const ::std::os::raw::c_char;

impl FromFFI<CCStr> for String {
    fn from(value: CCStr) -> Self {
        let cstr = unsafe { CStr::from_ptr(value) };
        cstr.to_string_lossy().to_string()
    }
}

impl FromFFI<::std::os::raw::c_int> for i32 {
    fn from(value: ::std::os::raw::c_int) -> Self {
        From::from(value)
    }
}

impl ToFFI<::std::os::raw::c_int> for i32 {
    fn to(&self) -> ::std::os::raw::c_int {
        *self
    }
}

use miniblink_sys::{wkeNavigationType, wkeProxy, wkeString, wkeWebView};
impl FromFFI<wkeString> for String {
    fn from(value: wkeString) -> Self {
        let wke_str = WkeStr::from_ptr(value);
        wke_str.to_string()
    }
}

impl ToFFI<CCStr> for &str {
    fn to(&self) -> CCStr {
        let cstring = CString::safe_new(&self);
        cstring.into_raw()
    }
}

impl ToFFI<bool> for bool {
    fn to(&self) -> bool {
        *self
    }
}

impl FromFFI<f32> for f32 {
    fn from(value: f32) -> Self {
        value
    }
}

impl ToFFI<f32> for f32 {
    fn to(&self) -> Self {
        *self
    }
}

impl FromFFI<wkeWebView> for WebView {
    fn from(value: wkeWebView) -> Self {
        WebView { webview: value }
    }
}

impl ToFFI<wkeWebView> for WebView {
    fn to(&self) -> wkeWebView {
        self.webview
    }
}

impl ToFFI<*mut wkeProxy> for &ProxyConfig {
    fn to(&self) -> *mut wkeProxy {
        &mut self.to_wke_proxy()
    }
}

bind_handler! {
    WebViewHandler for WebView {
        // wkeOnCaretChanged => on_caret_changed
        wkeOnMouseOverUrlChanged(title: wkeString) => on_mouse_over_url_changed(String);
        wkeOnTitleChanged(title: wkeString) => on_title_changed(String);
        wkeOnURLChanged(url: wkeString) => on_url_changed(String);
        // wkeOnURLChanged2 => on_url_changed2
        // wkeOnPaintUpdated => on_paint_updated
        // wkeOnPaintBitUpdated => on_paint_bit_updated
        wkeOnAlertBox(msg: wkeString) => on_alert_box(String);
        wkeOnConfirmBox(msg: wkeString) -> bool => on_confirm_box(String) -> bool | false;
        wkeOnPromptBox(msg: wkeString, default_result: wkeString, result: wkeString) -> bool => on_prompt_box(String, String, String) -> bool | false;
        wkeOnNavigation(navigation_type: wkeNavigationType, url: wkeString) -> bool => on_navigation(NavigationType, String) -> bool | false;
        // wkeOnCreateView => on_create_view
        wkeOnDocumentReady() => on_document_ready();
        // wkeOnDocumentReady2 => on_document_ready2
        // wkeOnLoadingFinish => on_loading_finish
        wkeOnDownload(url: CCStr) -> bool => on_download(String) -> bool | false;
        // wkeOnDownload2 => on_download2
        // wkeOnConsole => on_console
        // wkeOnLoadUrlBegin => on_load_url_begin
        // wkeOnLoadUrlEnd => on_load_url_end
        // wkeOnLoadUrlHeadersReceived => on_load_url_headers_received
        // wkeOnLoadUrlFinish => on_load_url_finish
        // wkeOnLoadUrlFail => on_load_url_fail
        // wkeOnDidCreateScriptContext => on_did_create_script_context
        // wkeOnWillReleaseScriptContext => on_will_release_script_context
        wkeOnWindowClosing() -> bool => on_window_closing() -> bool | false;
        wkeOnWindowDestroy() => on_window_destroy()
        // wkeOnDraggableRegionsChanged => on_draggable_regions_changed
        // wkeOnWillMediaLoad => on_will_media_load
        // wkeOnStartDragging => on_start_dragging
        // wkeOnPrint => on_print
        // wkeScreenshot => screenshot
        // wkeOnOtherLoad => on_other_load
        // wkeOnContextMenuItemClick => on_context_menu_item_click
    }
}

bind_target! {
    WebViewGetter for WebView {
        wkeGetSource => get_source() -> String;
        // wkeGetCaret =>
        // wkeGetClientHandler =>
        // wkeGetDebugConfig =>
        wkeGetName => get_name() -> String;
        wkeGetUserAgent => get_user_agent() -> String;
        wkeGetURL => get_url() -> String;
        // wkeGetFrameUrl =>
        wkeGetWebviewId => get_webview_id() -> i32;
        // wkeGetDocumentCompleteURL =>
        wkeGetTitle => get_title() -> String;
        // wkeGetTitleW =>
        wkeGetWidth => get_width() -> i32;
        wkeGetHeight => get_height() -> i32;
        wkeGetContentWidth => get_content_width() -> i32;
        wkeGetContentHeight => get_content_height() -> i32;
        // wkeGetViewDC =>
        // wkeGetHostHWND =>
        wkeGetNavigateIndex => get_navigate_index() -> i32;
        // wkeGetCookieW =>
        wkeGetCookie => get_cookie() -> String;
        // wkeGetMediaVolume =>
        // wkeGetCaretRect =>
        // wkeGetCaretRect2 =>
        // wkeGetGlobalExecByFrame =>
        wkeGetZoomFactor => get_zoom_factor() -> f32
        // wkeGetString =>
        // wkeGetStringW =>
        // wkeGetStringLen =>
        // wkeGetWebViewForCurrentContext =>
        // wkeGetUserKeyValue =>
        // wkeGetCursorInfoType =>
        // wkeGetTempCallbackInfo =>
        // wkeGetBlinkMainThreadIsolate =>
        // wkeGetWindowHandle =>
        // wkeGetWebViewByNData =>
        // wkeGetContentAsMarkup =>
    }
}

bind_target! {
    WebViewSetter for WebView {
        wkeSetResourceGc => set_resource_gc(resource_gc: i32);
        // wkeSetFileSystem =>
        wkeSetWebViewName => set_webview_name(webview_name: &str);
        // wkeSetClientHandler =>
        // wkeSetViewSettings =>
        // wkeSetDebugConfig =>
        // wkeSetMemoryCacheEnable =>
        wkeSetMouseEnabled => set_mouse_enabled(mouse_enabled: bool);
        wkeSetTouchEnabled => set_touch_enabled(touch_enabled: bool);
        wkeSetSystemTouchEnabled => set_system_touch_enabled(system_touch_enabled: bool);
        wkeSetContextMenuEnabled => set_context_menu_enabled(context_menu_enabled: bool);
        wkeSetNavigationToNewWindowEnable => set_navigation_to_new_window_enabled(navigation_to_new_window_enabled: bool);
        wkeSetCspCheckEnable => set_csp_check_enabled(csp_check_enabled: bool);
        wkeSetNpapiPluginsEnabled => set_npapi_plugins_enabled(npapi_plugins_enabled: bool);
        wkeSetHeadlessEnabled => set_headless_enabled(headless_enabled: bool);
        wkeSetDragEnable => set_drag_enabled(drag_enabled: bool);
        wkeSetDragDropEnable => set_drag_drop_enable(drag_drop_enable: bool);
        // wkeSetContextMenuItemShow =>
        wkeSetLanguage => set_language(language: &str);
        // wkeSetViewNetInterface =>
        // wkeSetProxy =>
        wkeSetViewProxy => set_proxy(proxy: &ProxyConfig);
        wkeSetName => set_name(name: &str);
        // wkeSetHandle =>
        // wkeSetHandleOffset =>
        wkeSetTransparent => set_transparent(transparent: bool);
        wkeSetUserAgent => set_user_agent(user_agent: &str);
        // wkeSetUserAgentW =>
        // wkeSetDirty =>
        // wkeSetCookie => set_cookie(cookie: &str)
        wkeSetCookieEnabled => set_cookie_enabled(cookie_enabled: bool);
        // wkeSetCookieJarPath => cookie_jar_path: &str;
        // wkeSetCookieJarFullPath => cookie_jar_full_path: &str;
        // wkeSetLocalStorageFullPath => local_storage_full_path: &str;
        wkeSetMediaVolume => set_media_volume(media_volume: f32);
        wkeSetFocus => set_focus();
        wkeSetZoomFactor => set_zoom_factor(zoom_factor: f32);
        wkeSetEditable => set_editable(editable: bool);
        // wkeSetString =>
        // wkeSetStringWithoutNullTermination =>
        // wkeSetStringW =>
        // wkeSetUserKeyValue =>
        // wkeSetCursorInfoType =>
        // wkeSetDragFiles =>
        // wkeSetDeviceParameter =>
        // wkeSetUIThreadCallback =>
        wkeSetWindowTitle => set_window_title(window_title: &str)
        // wkeSetWindowTitleW =>
        // wkeSetMediaPlayerFactory =>
    }
}

bind_target! {
    WebViewOperation for WebView {
        wkeShowWindow => show_window(show: bool);
        wkeLoadHTML => load_html(html: &str);
        wkeLoadURL => load_url(url: &str);
        wkeResize => resize(width: i32, height: i32)
    }
}
