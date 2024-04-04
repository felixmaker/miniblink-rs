use std::ffi::{CStr, CString};
use std::rc::Rc;

use miniblink_sys::{wkeNavigationType, wkeString, wkeWebView};

use crate::error::MBResult;
use crate::prelude::MBExecStateValue;
use crate::types::{
    Handle, JsExecState, JsValue, NavigationType, Proxy, WebFrameHandle, WindowType, WkeStr,
    WkeString,
};

use crate::call_api_or_panic;
use crate::util::SafeCString;

// macro_rules! optional_string {
//     ($expr: expr) => {{
//         let result = $expr;
//         if result.is_null() {
//             None
//         } else {
//             let cstr = unsafe { CStr::from_ptr(result) };
//             Some(cstr.to_string_lossy().to_string())
//         }
//     }};
// }

type WebViewWrapper = Rc<wkeWebView>;

#[derive(Clone, Debug)]
#[repr(transparent)]
/// Wrapper to [`miniblink_sys::wkeWebView`]
pub struct WebView {
    inner: WebViewWrapper,
}

impl WebView {
    /// Wraps a raw `wkeWebView`.
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_ptr(ptr: wkeWebView) -> Self {
        assert!(!ptr.is_null());
        let ptr = WebViewWrapper::into_raw(WebViewWrapper::from(ptr));
        WebViewWrapper::increment_strong_count(ptr); // Avoid destroy the webview instance.
        let inner = WebViewWrapper::from_raw(ptr);
        WebView { inner }
    }

    /// Return the inner pointer to `wkeWebView`.
    /// # Safety
    /// Can return multiple mutable pointers to the same item
    pub unsafe fn as_ptr(&self) -> wkeWebView {
        let ptr = WebViewWrapper::into_raw(WebViewWrapper::clone(&self.inner));
        WebViewWrapper::increment_strong_count(ptr);
        let inner = WebViewWrapper::from_raw(ptr);
        *inner
    }

    /// Creates a webview window.
    pub fn create_web_window(
        window_type: WindowType,
        handle: Handle,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Self {
        let webview = unsafe {
            call_api_or_panic().wkeCreateWebWindow(
                window_type.into(),
                handle.into(),
                x,
                y,
                width,
                height,
            )
        };
        assert!(!webview.is_null());
        WebView {
            inner: Rc::new(webview),
        }
    }

    /// Create a window with popup type.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::create_web_window(WindowType::Popup, Handle::null(), x, y, width, height)
    }

    /// Create a window with control type. This method creates window as child window.
    #[cfg(feature = "rwh_06")]
    pub fn new_as_child<H>(hwnd: H, x: i32, y: i32, width: i32, height: i32) -> MBResult<Self>
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

    /// Show window.
    pub fn show_window(&self, show: bool) {
        unsafe { call_api_or_panic().wkeShowWindow(*self.inner, show) }
    }

    /// Load HTML.
    pub fn load_html(&self, html: &str) {
        let html = CString::safe_new(html);
        unsafe { call_api_or_panic().wkeLoadHTML(*self.inner, html.as_ptr()) }
    }

    /// Load URL.
    pub fn load_url(&self, url: &str) {
        let url = CString::safe_new(url);
        unsafe { call_api_or_panic().wkeLoadURL(*self.inner, url.as_ptr()) }
    }

    /// Load file.
    pub fn load_file(&self, file: &str) {
        let file = CString::safe_new(file);
        unsafe { call_api_or_panic().wkeLoadFile(*self.inner, file.as_ptr()) }
    }

    /// Resize the webview (and window).
    pub fn resize(&self, width: i32, height: i32) {
        unsafe { call_api_or_panic().wkeResize(*self.inner, width, height) }
    }

    /// Move the webview window.
    pub fn move_window(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { call_api_or_panic().wkeMoveWindow(*self.inner, x, y, width, height) }
    }
    /// Move the webview window to the center of screen.
    pub fn move_to_center(&self) {
        unsafe { call_api_or_panic().wkeMoveToCenter(*self.inner) }
    }
    /// Resize thew webview window.
    pub fn resize_window(&self, width: i32, height: i32) {
        unsafe { call_api_or_panic().wkeResizeWindow(*self.inner, width, height) }
    }
    /// Run a script in webview. Use `eval` for serde support.
    pub fn run_js(&self, script: &str) -> JsValue {
        let script = CString::safe_new(script);
        let value = unsafe { call_api_or_panic().wkeRunJS(*self.inner, script.as_ptr()) };
        assert!(value != 0);
        unsafe { JsValue::from_ptr(value) }
    }
    /// Stop loading page.
    pub fn stop_loading(&self) {
        unsafe { call_api_or_panic().wkeStopLoading(*self.inner) }
    }
    /// Reload page.
    pub fn reload(&self) {
        unsafe { call_api_or_panic().wkeReload(*self.inner) }
    }
    /// wkeClearCookie.
    pub fn clear_cookie(&self) {
        unsafe { call_api_or_panic().wkeClearCookie(*self.inner) }
    }

    /// wkeSetFocus.
    pub fn set_focus(&self) {
        unsafe { call_api_or_panic().wkeSetFocus(*self.inner) }
    }
    /// See wkeKillFocus.
    pub fn kill_focus(&self) {
        unsafe { call_api_or_panic().wkeKillFocus(*self.inner) }
    }
    /// See wkeSleep.
    pub fn sleep(&self) {
        unsafe { call_api_or_panic().wkeSleep(*self.inner) }
    }
    /// See wkeWake.
    pub fn wake(&self) {
        unsafe { call_api_or_panic().wkeWake(*self.inner) }
    }
    /// See wkeRunJsByFrame.
    pub fn run_js_by_frame(
        &self,
        frame_id: WebFrameHandle,
        script: &str,
        is_in_closure: bool,
    ) -> JsValue {
        let script = CString::safe_new(script);
        let result = unsafe {
            call_api_or_panic().wkeRunJsByFrame(
                *self.inner,
                frame_id.as_ptr(),
                script.as_ptr(),
                is_in_closure,
            )
        };
        assert!(result != 0);
        unsafe { JsValue::from_ptr(result) }
    }

    /// See wkeEnableWindow.
    pub fn enable_window(&self, enable: bool) {
        unsafe { call_api_or_panic().wkeEnableWindow(*self.inner, enable) }
    }

    /// See wkeCanGoBack.
    pub fn can_go_back(&self) -> bool {
        (unsafe { call_api_or_panic().wkeCanGoBack(*self.inner) } != 0)
    }
    /// See wkeCanGoForward.
    pub fn can_go_forward(&self) -> bool {
        (unsafe { call_api_or_panic().wkeCanGoForward(*self.inner) } != 0)
    }
    /// See wkeIsDocumentReady.
    pub fn is_document_ready(&self) -> bool {
        (unsafe { call_api_or_panic().wkeIsDocumentReady(*self.inner) } != 0)
    }
    /// See wkeIsAwake.
    pub fn is_awake(&self) -> bool {
        (unsafe { call_api_or_panic().wkeIsAwake(*self.inner) } != 0)
    }

    /// See wkeIsTransparent.
    pub fn is_transparent(&self) -> bool {
        (unsafe { call_api_or_panic().wkeIsTransparent(*self.inner) } != 0)
    }

    /// See wkeGoBack.
    pub fn go_back(&self) -> bool {
        (unsafe { call_api_or_panic().wkeGoBack(*self.inner) } != 0)
    }
    /// See wkeEditorSelectAll.
    pub fn editor_select_all(&self) {
        unsafe { call_api_or_panic().wkeEditorSelectAll(*self.inner) }
    }
    /// See wkeEditorUnSelect.
    pub fn editor_unselect(&self) {
        unsafe { call_api_or_panic().wkeEditorUnSelect(*self.inner) }
    }
    /// See wkeEditorCopy.
    pub fn editor_copy(&self) {
        unsafe { call_api_or_panic().wkeEditorCopy(*self.inner) }
    }
    /// See wkeEditorCut.
    pub fn editor_cut(&self) {
        unsafe { call_api_or_panic().wkeEditorCut(*self.inner) }
    }
    /// See wkeEditorDelete.
    pub fn editor_delete(&self) {
        unsafe { call_api_or_panic().wkeEditorDelete(*self.inner) }
    }
    /// See wkeEditorUndo.
    pub fn editor_undo(&self) {
        unsafe { call_api_or_panic().wkeEditorUndo(*self.inner) }
    }
    /// See wkeEditorRedo.
    pub fn editor_redo(&self) {
        unsafe { call_api_or_panic().wkeEditorRedo(*self.inner) }
    }

    /// See wkeGetSource.
    pub fn get_source(&self) -> Option<String> {
        let source = unsafe { call_api_or_panic().wkeGetSource(*self.inner) };
        if source.is_null() {
            None
        } else {
            let cstr = unsafe { CStr::from_ptr(source) };
            Some(cstr.to_string_lossy().to_string())
        }
    }
    /// See wkeGetName.
    pub fn get_name(&self) -> String {
        let name = unsafe { call_api_or_panic().wkeGetName(*self.inner) };
        assert!(!name.is_null());
        unsafe { CStr::from_ptr(name) }
            .to_string_lossy()
            .to_string()
    }
    /// See wkeGetUserAgent.
    pub fn get_user_agent(&self) -> String {
        let user_agent = unsafe { call_api_or_panic().wkeGetUserAgent(*self.inner) };
        assert!(!user_agent.is_null());
        unsafe { CStr::from_ptr(user_agent) }
            .to_string_lossy()
            .to_string()
    }
    /// See wkeGetURL.
    pub fn get_url(&self) -> String {
        let url = unsafe { call_api_or_panic().wkeGetURL(*self.inner) };
        assert!(!url.is_null());
        unsafe { CStr::from_ptr(url) }.to_string_lossy().to_string()
    }
    /// See wkeGetFrameUrl.
    pub fn get_frame_url(&self, frame_id: WebFrameHandle) -> String {
        let url = unsafe { call_api_or_panic().wkeGetFrameUrl(*self.inner, frame_id.as_ptr()) };
        assert!(!url.is_null());
        unsafe { CStr::from_ptr(url) }.to_string_lossy().to_string()
    }
    /// See wkeGetWebviewId.
    pub fn get_webview_id(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetWebviewId(*self.inner) }
    }
    /// See wkeGetTitle.
    pub fn get_title(&self) -> String {
        let title = unsafe { call_api_or_panic().wkeGetTitle(*self.inner) };
        assert!(!title.is_null());
        unsafe { CStr::from_ptr(title) }
            .to_string_lossy()
            .to_string()
    }
    /// See wkeGetWidth.
    pub fn get_width(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetWidth(*self.inner) }
    }
    /// See wkeGetHeight.
    pub fn get_height(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetHeight(*self.inner) }
    }
    /// See wkeGetContentWidth.
    pub fn get_content_width(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetContentWidth(*self.inner) }
    }
    /// See wkeGetContentHeight.
    pub fn get_content_height(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetContentHeight(*self.inner) }
    }
    /// See wkeGetHostHWND.
    pub fn get_host_hwnd(&self) -> Handle {
        let hwnd = unsafe { call_api_or_panic().wkeGetHostHWND(*self.inner) };
        Handle::from(hwnd)
    }
    /// See wkeGetNavigateIndex.
    pub fn get_navigate_index(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetNavigateIndex(*self.inner) }
    }
    /// See wkeGetCookie.
    pub fn get_cookie(&self) -> String {
        let cookie = unsafe { call_api_or_panic().wkeGetCookie(*self.inner) };
        assert!(!cookie.is_null());
        unsafe { CStr::from_ptr(cookie) }
            .to_string_lossy()
            .to_string()
    }

    /// See wkeGetZoomFactor.
    pub fn get_zoom_factor(&self) -> f32 {
        unsafe { call_api_or_panic().wkeGetZoomFactor(*self.inner) }
    }

    /// See wkeGetWindowHandle.
    pub fn get_window_handle(&self) -> Handle {
        let hwnd = unsafe { call_api_or_panic().wkeGetWindowHandle(*self.inner) };
        Handle::from(hwnd)
    }

    /// See wkeGlobalExec.
    pub fn global_exec(&self) -> JsExecState {
        let es = unsafe { call_api_or_panic().wkeGlobalExec(*self.inner) };
        assert!(!es.is_null());
        unsafe { JsExecState::from_ptr(es) }
    }
    /// See wkeWebFrameGetMainFrame.
    pub fn get_main_frame(&self) -> WebFrameHandle {
        let handle = unsafe { call_api_or_panic().wkeWebFrameGetMainFrame(*self.inner) };
        assert!(!handle.is_null());
        unsafe { WebFrameHandle::from_ptr(handle) }
    }

    /// See wkeSetResourceGc.
    pub fn set_resource_gc(&self, resource_gc: i32) {
        unsafe { call_api_or_panic().wkeSetResourceGc(*self.inner, resource_gc) }
    }
    /// See wkeSetWebViewName.
    pub fn set_webview_name(&self, webview_name: &str) {
        let webview_name = CString::safe_new(webview_name);
        unsafe { call_api_or_panic().wkeSetWebViewName(*self.inner, webview_name.as_ptr()) }
    }
    /// See wkeSetViewSettings.
    // pub fn set_view_settings(&self, settings: &ViewSettings ) {
    // unsafe {call_api_or_panic().wkeSetViewSettings(*self.inner, )}
    // }
    /// See wkeSetDebugConfig.
    pub fn set_debug_config(&self, debug_string: &str, param: &str) {
        let debug_string = CString::safe_new(debug_string);
        let param = CString::safe_new(param);
        unsafe {
            call_api_or_panic().wkeSetDebugConfig(
                *self.inner,
                debug_string.as_ptr(),
                param.as_ptr(),
            )
        }
    }
    /// See wkeSetMemoryCacheEnable.
    pub fn set_memory_cache_enable(&self, memory_cache_enable: bool) {
        unsafe { call_api_or_panic().wkeSetMemoryCacheEnable(*self.inner, memory_cache_enable) }
    }
    /// See wkeSetMouseEnabled.
    pub fn set_mouse_enabled(&self, mouse_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetMouseEnabled(*self.inner, mouse_enabled) }
    }
    /// See wkeSetTouchEnabled.
    pub fn set_touch_enabled(&self, touch_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetTouchEnabled(*self.inner, touch_enabled) }
    }
    /// See wkeSetSystemTouchEnabled.
    pub fn set_system_touch_enabled(&self, system_touch_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetSystemTouchEnabled(*self.inner, system_touch_enabled) }
    }
    /// See wkeSetContextMenuEnabled.
    pub fn set_context_menu_enabled(&self, context_menu_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetContextMenuEnabled(*self.inner, context_menu_enabled) }
    }
    /// See wkeSetNavigationToNewWindowEnable.
    pub fn set_navigation_to_new_window_enabled(&self, navigation_to_new_window_enabled: bool) {
        unsafe {
            call_api_or_panic()
                .wkeSetNavigationToNewWindowEnable(*self.inner, navigation_to_new_window_enabled)
        }
    }
    /// See wkeSetCspCheckEnable.
    pub fn set_csp_check_enable(&self, csp_check_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetCspCheckEnable(*self.inner, csp_check_enabled) }
    }
    /// See wkeSetNpapiPluginsEnabled.
    pub fn set_npapi_plugins_enabled(&self, npapi_plugins_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetNpapiPluginsEnabled(*self.inner, npapi_plugins_enabled) }
    }
    /// See wkeSetHeadlessEnabled.
    pub fn set_headless_enabled(&self, headless_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetHeadlessEnabled(*self.inner, headless_enabled) }
    }
    /// See wkeSetDragEnable.
    pub fn set_drag_enabled(&self, drag_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetDragEnable(*self.inner, drag_enabled) }
    }
    /// See wkeSetDragDropEnable.
    pub fn set_drag_drop_enable(&self, drag_drop_enable: bool) {
        unsafe { call_api_or_panic().wkeSetDragDropEnable(*self.inner, drag_drop_enable) }
    }
    /// See wkeSetContextMenuItemShow.
    // pub fn set_context_menu_item_show(&self, item_id: MenuItemId, show: bool) {
    // unsafe {call_api_or_panic().wkeSetContextMenuItemShow(*self.inner, item_id.into(), show)}
    // }
    /// See wkeSetLanguage.
    pub fn set_language(&self, language: &str) {
        let language = CString::safe_new(language);
        unsafe { call_api_or_panic().wkeSetLanguage(*self.inner, language.as_ptr()) }
    }
    /// See wkeSetViewNetInterface.
    pub fn set_view_net_interface(&self, net_interface: &str) {
        let net_interface = CString::safe_new(net_interface);
        unsafe { call_api_or_panic().wkeSetViewNetInterface(*self.inner, net_interface.as_ptr()) }
    }
    /// See wkeSetViewProxy.
    pub fn set_proxy(&self, proxy: &Proxy) {
        let mut proxy = proxy.to_wke_proxy();
        unsafe { call_api_or_panic().wkeSetViewProxy(*self.inner, &mut proxy) }
    }
    /// See wkeSetName.
    pub fn set_name(&self, name: &str) {
        let name = CString::safe_new(name);
        unsafe { call_api_or_panic().wkeSetName(*self.inner, name.as_ptr()) }
    }
    /// See wkeSetHandle.
    pub fn set_handle(&self, hwnd: Handle) {
        unsafe { call_api_or_panic().wkeSetHandle(*self.inner, hwnd.into()) }
    }
    /// See wkeSetHandleOffset.
    pub fn set_handle_offset(&self, x: i32, y: i32) {
        unsafe { call_api_or_panic().wkeSetHandleOffset(*self.inner, x, y) }
    }
    /// See wkeSetTransparent.
    pub fn set_transparent(&self, transparent: bool) {
        unsafe { call_api_or_panic().wkeSetTransparent(*self.inner, transparent) }
    }
    /// See wkeSetUserAgent.
    pub fn set_user_agent(&self, user_agent: &str) {
        let user_agent = CString::safe_new(user_agent);
        unsafe { call_api_or_panic().wkeSetUserAgent(*self.inner, user_agent.as_ptr()) }
    }
    /// See wkeSetCookie.
    pub fn set_cookie(&self, url: &str, cookie: &str) {
        let url = CString::safe_new(url);
        let cookie = CString::safe_new(cookie);
        unsafe { call_api_or_panic().wkeSetCookie(*self.inner, url.as_ptr(), cookie.as_ptr()) }
    }
    /// See wkeSetCookieEnabled.
    pub fn set_cookie_enabled(&self, cookie_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetCookieEnabled(*self.inner, cookie_enabled) }
    }
    /// See wkeSetCookieJarPath.
    pub fn set_cookie_jar_path(&self, path: &str) {
        let path = WkeString::new(path);
        unsafe { call_api_or_panic().wkeSetCookieJarPath(*self.inner, path.as_wcstr_ptr()) }
    }
    /// See wkeSetCookieJarFullPath.
    pub fn set_cookie_jar_full_path(&self, path: &str) {
        let path = WkeString::new(path);
        unsafe { call_api_or_panic().wkeSetCookieJarFullPath(*self.inner, path.as_wcstr_ptr()) }
    }
    /// See wkeSetLocalStorageFullPath.
    pub fn set_local_storage_full_path(&self, path: &str) {
        let path = WkeString::new(path);
        unsafe { call_api_or_panic().wkeSetLocalStorageFullPath(*self.inner, path.as_wcstr_ptr()) }
    }
    // /// See wkeSetMediaVolume.
    // pub fn set_media_volume(&self, media_volume: f32) {
    // unsafe {call_api_or_panic().wkeSetMediaVolume(*self.inner, )}
    // }
    /// See wkeSetZoomFactor.
    pub fn set_zoom_factor(&self, zoom_factor: f32) {
        unsafe { call_api_or_panic().wkeSetZoomFactor(*self.inner, zoom_factor) }
    }
    /// See wkeSetEditable.
    pub fn set_editable(&self, editable: bool) {
        unsafe { call_api_or_panic().wkeSetEditable(*self.inner, editable) }
    }
    // wkeSetUserKeyValue =>
    /// See wkeSetCursorInfoType.
    pub fn set_cursor_info_type(&self, cursor_info_type: i32) {
        unsafe { call_api_or_panic().wkeSetCursorInfoType(*self.inner, cursor_info_type) }
    }
    // wkeSetDragFiles => set_drag_files(clint_pos: &Point as POINT, screen_pos: *const POINT, files: &[&str], files_count: i32);
    /// See wkeSetDeviceParameter.
    pub fn set_device_parameter(
        &self,
        device: &str,
        param_str: &str,
        param_int: i32,
        param_float: f32,
    ) {
        let device = CString::safe_new(device);
        let param_str = CString::safe_new(param_str);
        unsafe {
            call_api_or_panic().wkeSetDeviceParameter(
                *self.inner,
                device.as_ptr(),
                param_str.as_ptr(),
                param_int,
                param_float,
            )
        }
    }
    /// See wkeSetWindowTitle.
    pub fn set_window_title(&self, window_title: &str) {
        let window_title = CString::safe_new(window_title);
        unsafe { call_api_or_panic().wkeSetWindowTitle(*self.inner, window_title.as_ptr()) }
    }

    /// See wkeOnMouseOverUrlChanged.
    pub fn on_mouse_over_url_changed<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
            title: wkeString,
        ) where
            F: FnMut(&mut WebView, String) + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;
            assert!(!title.is_null());
            let title = WkeStr::from_ptr(title).to_string();

            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, title)));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnMouseOverUrlChanged(
                *self.inner,
                Some(shim::<F>),
                cb as *mut _,
            );
        }
    }

    /// See wkeOnTitleChanged.           
    pub fn on_title_changed<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
            title: wkeString,
        ) where
            F: FnMut(&mut WebView, String) + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;
            let title = WkeStr::from_ptr(title).to_string();

            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, title)));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnTitleChanged(
                *self.inner,
                Some(shim::<F>),
                cb as *mut _,
            );
        }
    }
    /// See wkeOnURLChanged.           
    pub fn on_url_changed<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
            url: wkeString,
        ) where
            F: FnMut(&mut WebView, String) + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;
            let url = WkeStr::from_ptr(url).to_string();

            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, url)));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnURLChanged(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// See wkeOnAlertBox.           
    pub fn on_alert_box<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
            msg: wkeString,
        ) where
            F: FnMut(&mut WebView, String) + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;
            let msg = WkeStr::from_ptr(msg).to_string();

            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, msg)));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnAlertBox(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// See wkeOnConfirmBox.           
    pub fn on_confirm_box<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String) -> bool + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
            msg: wkeString,
        ) -> bool
        where
            F: FnMut(&mut WebView, String) -> bool + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;
            let msg = WkeStr::from_ptr(msg).to_string();

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, msg)));
            r.unwrap_or(false)
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnConfirmBox(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// See wkeOnPromptBox.           
    pub fn on_prompt_box<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String, String, String) -> bool + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
            msg: wkeString,
            default_result: wkeString,
            result: wkeString,
        ) -> bool
        where
            F: FnMut(&mut WebView, String, String, String) -> bool + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;
            let msg = WkeStr::from_ptr(msg).to_string();
            let default_result = WkeStr::from_ptr(default_result).to_string();
            let result = WkeStr::from_ptr(result).to_string();

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(&mut wv, msg, default_result, result)
            }));
            r.unwrap_or(false)
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnPromptBox(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// See wkeOnNavigation.           
    pub fn on_navigation<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, NavigationType, String) -> bool + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
            navigation_type: wkeNavigationType,
            url: wkeString,
        ) -> bool
        where
            F: FnMut(&mut WebView, NavigationType, String) -> bool + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;

            let navigation_type = NavigationType::from(navigation_type);
            let url = WkeStr::from_ptr(url).to_string();

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(&mut wv, navigation_type, url)
            }));

            r.unwrap_or(false)
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnNavigation(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    /// See wkeOnDocumentReady.           
    pub fn on_document_ready<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
        ) where
            F: FnMut(&mut WebView) + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;

            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv)));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnDocumentReady(
                *self.inner,
                Some(shim::<F>),
                cb as *mut _,
            );
        }
    }

    /// See wkeOnDownload.           
    pub fn on_download<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String) -> bool + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
            url: *const i8,
        ) -> bool
        where
            F: FnMut(&mut WebView, String) -> bool + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;

            let url = CStr::from_ptr(url).to_string_lossy().to_string();

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, url)));
            r.unwrap_or(false)
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnDownload(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    /// See wkeOnWindowClosing.           
    pub fn on_window_closing<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView) -> bool + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
        ) -> bool
        where
            F: FnMut(&mut WebView) -> bool + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv)));
            r.unwrap_or(false)
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnWindowClosing(
                *self.inner,
                Some(shim::<F>),
                cb as *mut _,
            );
        }
    }

    /// See wkeOnWindowDestroy.
    pub fn on_window_destroy<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView) -> bool + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
        ) where
            F: FnMut(&mut WebView) -> bool + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;

            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv)));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            crate::call_api_or_panic().wkeOnWindowDestroy(
                *self.inner,
                Some(shim::<F>),
                cb as *mut _,
            );
        }
    }
}

impl Drop for WebView {
    fn drop(&mut self) {
        if Rc::strong_count(&self.inner) == 1 {
            unsafe {
                call_api_or_panic().wkeDestroyWebWindow(*self.inner);
            }
        }
    }
}

impl Default for WebView {
    fn default() -> Self {
        Self::create_web_window(WindowType::Popup, Handle::null(), 0, 0, 200, 200)
    }
}

/// Extra API for MBWebView
pub trait WebViewExt {
    /// Eval a script on webview
    fn eval<T>(&self, script: &str) -> MBResult<T>
    where
        JsExecState: MBExecStateValue<T>;
}

impl WebViewExt for WebView {
    fn eval<T>(&self, script: &str) -> MBResult<T>
    where
        JsExecState: MBExecStateValue<T>,
    {
        let js_value = self.run_js(script);
        let es = self.global_exec();
        es.value(js_value)
    }
}
