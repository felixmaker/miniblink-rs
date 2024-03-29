use std::ffi::CString;

use miniblink_sys::{wkeNavigationType, wkeString, wkeWindowType, HWND};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

use crate::error::{MBError, MBResult};
use crate::types::{CCStr, JsExecState, JsValue, MBExecStateValue, NavigationType, ProxyConfig};
use crate::types::{FromFFI, ToFFI};
use crate::util::SafeCString;

use crate::{bind_handler, bind_target, call_api_or_panic};

type InnerWebView = miniblink_sys::wkeWebView;

/// Wrapper to [`miniblink_sys::wkeWebView`]
pub struct WebView {
    pub(crate) webview: InnerWebView,
}

impl Default for WebView {
    fn default() -> Self {
        Self::new(0, 0, 200, 200)
    }
}

impl WebView {
    /// Create window with popup type.
    ///
    /// Notes: This method creates real window.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::create_popup_window(x, y, width, height)
    }

    /// Create window with control type.
    ///
    /// Notes: This method creates window as child window.
    pub fn new_as_child(
        hwnd: &impl HasWindowHandle,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> MBResult<Self> {
        match hwnd.window_handle().map(|x| x.as_raw()) {
            Ok(RawWindowHandle::Win32(handle)) => Ok(WebView::create_control_window(
                isize::from(handle.hwnd) as HWND,
                x,
                y,
                width,
                height,
            )),
            _ => Err(MBError::UnsupportedPlatform),
        }
    }

    fn create_popup_window(x: i32, y: i32, width: i32, height: i32) -> Self {
        let window = unsafe {
            call_api_or_panic().wkeCreateWebWindow(
                wkeWindowType::WKE_WINDOW_TYPE_POPUP,
                std::ptr::null_mut(),
                x,
                y,
                width,
                height,
            )
        };
        Self { webview: window }
    }

    fn create_control_window(parent: HWND, x: i32, y: i32, width: i32, height: i32) -> Self {
        let window = unsafe {
            call_api_or_panic().wkeCreateWebWindow(
                wkeWindowType::WKE_WINDOW_TYPE_CONTROL,
                parent,
                x,
                y,
                width,
                height,
            )
        };

        Self { webview: window }
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
        // wkeSetFileSystem => set_file_system(...);
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
        wkeSetCookie => set_cookie(url: &str, cookie: &str);
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
        wkeSetWindowTitle => set_window_title(window_title: &str);
        // wkeSetWindowTitleW =>
        // wkeSetMediaPlayerFactory =>
        wkeEnableWindow => enable_window(enable: bool)
    }
}

bind_target! {
    WebViewOperation for WebView {
        wkeShowWindow => show_window(show: bool);
        wkeLoadHTML => load_html(html: &str);
        wkeLoadURL => load_url(url: &str);
        wkeResize => resize(width: i32, height: i32);
        wkeMoveWindow => move_window(x: i32, y: i32, width: i32, height: i32);
        wkeMoveToCenter => move_to_center()
    }
}
