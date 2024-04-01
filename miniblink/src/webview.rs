use std::ffi::CString;

use miniblink_sys::{wkeNavigationType, wkeString, wkeViewSettings, wkeWebView};

use crate::error::MBResult;
use crate::types::{
    CProxy, Handle, JsExecState, JsValue, MBExecStateValue, MenuItemId, NavigationType, Proxy,
    ViewSettings, WebFrameHandle, WindowType, WkeString,
};

use crate::{bind_global, bind_target, impl_handler};

/// Wrapper to [`miniblink_sys::wkeWebView`]
pub struct WebView {
    pub(crate) webview: wkeWebView,
}

impl WebView {
    bind_target! {
        pub wkeShowWindow => show_window(show: bool);
        pub wkeLoadHTML => load_html(html: &str as CString);
        pub wkeLoadURL => load_url(url: &str as CString);
        pub wkeLoadFile => load_file(file: &str as CString);
        pub wkeResize => resize(width: i32, height: i32);
        pub wkeMoveWindow => move_window(x: i32, y: i32, width: i32, height: i32);
        pub wkeMoveToCenter => move_to_center();
        pub wkeResizeWindow => resize_window(width: i32, height: i32);
        pub wkeRunJS => run_js(script: &str as CString) -> JsValue;
        pub wkeStopLoading => stop_loading();
        pub wkeReload => reload();
        // wkeVisitAllCookie
        // wkePerformCookieCommand
        pub wkeClearCookie => clear_cookie();
        pub wkeSetFocus => set_focus();
        pub wkeKillFocus => kill_focus();
        // pub wkeSleep => sleep();
        pub wkeWake => wake();
        pub wkeRunJsByFrame => run_js_by_frame(frame_id: WebFrameHandle, script: &str as CString, is_in_closure: bool);
        // pub(crate) wkeDestroyWebView => destroy_webview();
        pub wkeEnableWindow => enable_window(enable: bool);
        // pub wkeConfigure => configure();
    }

    bind_target! {
        pub wkeCanGoBack => can_go_back() -> bool;
        pub wkeCanGoForward => can_go_forward() -> bool;
        pub wkeIsDocumentReady => is_document_ready() -> bool;
        pub wkeIsAwake => is_awake() -> bool;
        // pub wkeIsMainFrame
        pub wkeIsTransparent => is_transparent() -> bool;
    }

    bind_target! {
        pub wkeGoBack => go_back();
        pub wkeEditorSelectAll => editor_select_all();
        pub wkeEditorUnSelect => editor_unselect();
        pub wkeEditorCopy => editor_copy();
        pub wkeEditorCut => editor_cut();
        pub wkeEditorDelete => editor_delete();
        pub wkeEditorUndo => editor_undo();
        pub wkeEditorRedo => editor_redo();
    }

    bind_target! {
        // wkeFireMouseEvent
        // wkeFireContextMenuEvent
        // wkeFireMouseWheelEvent
        // wkeFireKeyUpEvent
        // wkeFireKeyDownEvent
        // wkeFireKeyPressEvent
        // wkeFireWindowsMessage
    }

    bind_target! {
        // pub wkeNetSetHTTPHeaderField
        // pub wkeNetGetRawHttpHead
        // pub wkeNetSetMIMEType
        // pub wkeNetGetMIMEType
        // pub wkeNetSetData
        // pub wkeNetCancelRequest
        // pub wkeNetHoldJobToAsynCommit
        // pub wkeNetGetRequestMethod
        // pub wkeNetGetPostBody
        // wkeNetCreatePostBodyElements =>
        // wkeNetFreePostBodyElements
        // wkeNetCreatePostBodyElement =>
        // wkeNetFreePostBodyElement
    }

    bind_target! {
        pub wkeGetSource => get_source() -> String;
        // wkeGetCaret =>
        // wkeGetClientHandler => get_client_handler();
        // wkeGetDebugConfig => get_debug_config(debug_string: &str) -> String;
        pub wkeGetName => get_name() -> String;
        pub wkeGetUserAgent => get_user_agent() -> String;
        pub wkeGetURL => get_url() -> String;
        pub wkeGetFrameUrl => get_frame_url(frame_id: WebFrameHandle) -> String;
        pub wkeGetWebviewId => get_webview_id() -> i32;
        // wkeGetDocumentCompleteURL => get_document_complete_url();
        pub wkeGetTitle => get_title() -> String;
        pub wkeGetWidth => get_width() -> i32;
        pub wkeGetHeight => get_height() -> i32;
        pub wkeGetContentWidth => get_content_width() -> i32;
        pub wkeGetContentHeight => get_content_height() -> i32;
        // wkeGetViewDC =>
        pub wkeGetHostHWND => get_host_hwnd() -> Handle;
        pub wkeGetNavigateIndex => get_navigate_index() -> i32;
        pub wkeGetCookie => get_cookie() -> String;
        // wkeGetMediaVolume =>
        // wkeGetCaretRect =>
        // wkeGetCaretRect2 =>
        // wkeGetGlobalExecByFrame =>
        pub wkeGetZoomFactor => get_zoom_factor() -> f32;
        // wkeGetWebViewForCurrentContext =>
        // wkeGetUserKeyValue =>
        // wkeGetCursorInfoType =>
        // wkeGetTempCallbackInfo =>
        // wkeGetBlinkMainThreadIsolate =>
        pub wkeGetWindowHandle => get_window_handle() -> Handle;
        // wkeGetWebViewByNData =>
        // wkeGetContentAsMarkup =>
        pub wkeGlobalExec => global_exec() -> JsExecState;
        pub wkeWebFrameGetMainFrame => get_main_frame() -> WebFrameHandle;
    }

    bind_target! {
        pub wkeSetResourceGc => set_resource_gc(resource_gc: i32);
        pub wkeSetWebViewName => set_webview_name(webview_name: &str as CString);
        pub wkeSetViewSettings => set_view_settings(settings: &ViewSettings as wkeViewSettings);
        pub wkeSetDebugConfig => set_debug_config(debug_string: &str as CString, param: &str as CString);
        pub wkeSetMemoryCacheEnable => set_memory_cache_enable(memory_cache_enable: bool);
        pub wkeSetMouseEnabled => set_mouse_enabled(mouse_enabled: bool);
        pub wkeSetTouchEnabled => set_touch_enabled(touch_enabled: bool);
        pub wkeSetSystemTouchEnabled => set_system_touch_enabled(system_touch_enabled: bool);
        pub wkeSetContextMenuEnabled => set_context_menu_enabled(context_menu_enabled: bool);
        pub wkeSetNavigationToNewWindowEnable => set_navigation_to_new_window_enabled(navigation_to_new_window_enabled: bool);
        pub wkeSetCspCheckEnable => set_csp_check_enable(csp_check_enabled: bool);
        pub wkeSetNpapiPluginsEnabled => set_npapi_plugins_enabled(npapi_plugins_enabled: bool);
        pub wkeSetHeadlessEnabled => set_headless_enabled(headless_enabled: bool);
        pub wkeSetDragEnable => set_drag_enabled(drag_enabled: bool);
        pub wkeSetDragDropEnable => set_drag_drop_enable(drag_drop_enable: bool);
        pub wkeSetContextMenuItemShow => set_context_menu_item_show(item_id: MenuItemId, show: bool);
        pub wkeSetLanguage => set_language(language: &str as CString);
        pub wkeSetViewNetInterface => set_view_net_interface(net_interface: &str as CString);
        pub wkeSetViewProxy => set_proxy(proxy: &Proxy as CProxy);
        pub wkeSetName => set_name(name: &str as CString);
        pub wkeSetHandle => set_handle(hwnd: Handle);
        pub wkeSetHandleOffset => set_handle_offset(x: i32, y: i32);
        pub wkeSetTransparent => set_transparent(transparent: bool);
        pub wkeSetUserAgent => set_user_agent(user_agent: &str as CString);
        pub wkeSetCookie => set_cookie(url: &str as CString, cookie: &str as CString);
        pub wkeSetCookieEnabled => set_cookie_enabled(cookie_enabled: bool);
        pub wkeSetCookieJarPath => set_cookie_jar_path(path: &str as WkeString);
        pub wkeSetCookieJarFullPath => set_cookie_jar_full_path(path: &str as WkeString);
        pub wkeSetLocalStorageFullPath => set_local_storage_full_path(path: &str as WkeString);
        // pub wkeSetMediaVolume => set_media_volume(media_volume: f32);
        pub wkeSetZoomFactor => set_zoom_factor(zoom_factor: f32);
        pub wkeSetEditable => set_editable(editable: bool);
        // wkeSetUserKeyValue =>
        pub wkeSetCursorInfoType => set_cursor_info_type(cursor_info_type: i32);
        // wkeSetDragFiles => set_drag_files(clint_pos: &Point as POINT, screen_pos: *const POINT, files: &[&str], files_count: i32);
        // pub wkeSetDeviceParameter => set_device_parameter(device: &str as CString, param_str: &str as CString, param_int: i32, param_float: f32);
        pub wkeSetWindowTitle => set_window_title(window_title: &str as CString);
    }

    bind_global! {
        pub wkeCreateWebWindow => create_web_window(window_type: WindowType, handle: Handle, x: i32, y: i32, width: i32, height: i32) -> WebView
    }
}

impl_handler! {
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
        wkeOnDownload(url: *const i8) -> bool => on_download(String) -> bool | false;
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
        // pub wkeNetGetFavicon =>
    }
}

/// Extra api for WebView
pub trait WebViewExt: Sized {
    /// Create a popup type window.
    ///
    /// Notes: This method creates real window.
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self;

    #[cfg(feature = "rwh_06")]
    /// Create a control type window.
    ///
    /// Notes: This method creates window as child window.
    fn new_as_child<H>(hwnd: H, x: i32, y: i32, width: i32, height: i32) -> MBResult<Self>
    where
        H: raw_window_handle::HasWindowHandle;

    /// Eval the script
    fn eval<T>(&self, script: &str) -> MBResult<T>
    where
        JsExecState: MBExecStateValue<T>;
}

impl WebViewExt for WebView {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::create_web_window(WindowType::Popup, Handle::null(), x, y, width, height)
    }

    #[cfg(feature = "rwh_06")]
    fn new_as_child<H>(hwnd: H, x: i32, y: i32, width: i32, height: i32) -> MBResult<Self>
    where
        H: raw_window_handle::HasWindowHandle,
    {
        match hwnd.window_handle().map(|x| x.as_raw()) {
            Ok(raw_window_handle::RawWindowHandle::Win32(handle)) => Ok(Self::create_web_window(
                WindowType::Control,
                Handle(isize::from(handle.hwnd)),
                x,
                y,
                width,
                height,
            )),
            _ => Err(crate::error::MBError::UnsupportedPlatform),
        }
    }

    fn eval<T>(&self, script: &str) -> MBResult<T>
    where
        JsExecState: MBExecStateValue<T>,
    {
        let js_value = self.run_js(script);
        let es = self.global_exec();
        es.value(js_value)
    }
}

impl Default for WebView {
    fn default() -> Self {
        Self::create_web_window(WindowType::Popup, Handle::null(), 0, 0, 200, 200)
    }
}
