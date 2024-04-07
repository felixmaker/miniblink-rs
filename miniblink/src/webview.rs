use std::ffi::{CStr, CString};
use std::rc::Rc;

use miniblink_sys::{
    wkeConsoleLevel, wkeMediaLoadInfo, wkeMemBuf, wkeNavigationType, wkeNetJob, wkeRect, wkeString,
    wkeViewSettings, wkeWebView, wkeWindowFeatures, BOOL, HDC, HWND, LPARAM, LRESULT, POINT, UINT,
    WPARAM,
};

use crate::error::MBResult;
use crate::prelude::MBExecStateValue;
use crate::types::*;

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
/// Webview. Wraps to `wkeWebView`.
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
    /// Can return multiple mutable pointers to the same item.
    pub unsafe fn as_ptr(&self) -> wkeWebView {
        let ptr = WebViewWrapper::into_raw(WebViewWrapper::clone(&self.inner));
        WebViewWrapper::increment_strong_count(ptr);
        let inner = WebViewWrapper::from_raw(ptr);
        *inner
    }

    /// Creates a webview window.
    ///
    /// Note: This method creates a real window.
    pub fn create_web_window(
        window_type: WindowType,
        handle: HWND,
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
        Self::create_web_window(WindowType::Popup, HWND(0), x, y, width, height)
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
                HWND(isize::from(handle.hwnd)),
                x,
                y,
                width,
                height,
            )),
            _ => Err(crate::error::MBError::UnsupportedPlatform),
        }
    }

    /// Show the window.
    pub fn show_window(&self, show: bool) {
        unsafe { call_api_or_panic().wkeShowWindow(*self.inner, show) }
    }

    /// Load HTML.
    pub fn load_html(&self, html: &str) {
        let html = CString::safe_new(html);
        unsafe { call_api_or_panic().wkeLoadHTML(*self.inner, html.as_ptr()) }
    }

    /// Load HTML with base URL.
    pub fn load_html_with_base_url(&self, html: &str, base_url: &str) {
        let html = CString::safe_new(html);
        let base_url = CString::safe_new(base_url);
        unsafe {
            call_api_or_panic().wkeLoadHtmlWithBaseUrl(
                *self.inner,
                html.as_ptr(),
                base_url.as_ptr(),
            )
        }
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
    ///
    /// Note: if the webview has window, it will also set the width and height of the window.
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
    /// Resize the webview window. Same as `resize`.
    pub fn resize_window(&self, width: i32, height: i32) {
        unsafe { call_api_or_panic().wkeResizeWindow(*self.inner, width, height) }
    }
    /// Run a script.
    pub fn run_js(&self, script: &str) -> JsValue {
        let script = CString::safe_new(script);
        let value = unsafe { call_api_or_panic().wkeRunJS(*self.inner, script.as_ptr()) };
        assert!(value != 0);
        unsafe { JsValue::from_ptr(value) }
    }
    /// Stop loading pages.
    pub fn stop_loading(&self) {
        unsafe { call_api_or_panic().wkeStopLoading(*self.inner) }
    }
    /// Reload pages.
    pub fn reload(&self) {
        unsafe { call_api_or_panic().wkeReload(*self.inner) }
    }
    /// Clear cookie.
    ///
    /// Note: Only support clearing all page cookies.
    pub fn clear_cookie(&self) {
        unsafe { call_api_or_panic().wkeClearCookie(*self.inner) }
    }

    /// Set webview focus. If the webview has window, it will also set focus to window.
    pub fn set_focus(&self) {
        unsafe { call_api_or_panic().wkeSetFocus(*self.inner) }
    }
    /// Kill webview focus. If the webview has window, it will also set focus to window.
    pub fn kill_focus(&self) {
        unsafe { call_api_or_panic().wkeKillFocus(*self.inner) }
    }
    /// Get caret rect of editor.
    pub fn get_caret_rect(&self) -> Rect {
        let rect = unsafe { call_api_or_panic().wkeGetCaretRect(*self.inner) };
        Rect::new(rect.x, rect.y, rect.w, rect.h)
    }
    /// Sleep. May unimplemented!
    pub fn sleep(&self) {
        unsafe { call_api_or_panic().wkeSleep(*self.inner) }
    }
    /// Wake. May unimplemented!
    pub fn wake(&self) {
        unsafe { call_api_or_panic().wkeWake(*self.inner) }
    }
    /// Run a script by frame. Param `is_in_closure` means if script needs to be in the form of `function() {}`.
    ///
    /// Note: if `is_in_closure` is `true`, keyword `return` is required to get returned value.
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

    /// Enable the window.
    pub fn enable_window(&self, enable: bool) {
        unsafe { call_api_or_panic().wkeEnableWindow(*self.inner, enable) }
    }

    /// Check if the webview can go back.
    pub fn can_go_back(&self) -> bool {
        (unsafe { call_api_or_panic().wkeCanGoBack(*self.inner) }).as_bool()
    }
    /// Check if the webview can go forward.
    pub fn can_go_forward(&self) -> bool {
        (unsafe { call_api_or_panic().wkeCanGoForward(*self.inner) }).as_bool()
    }
    /// Check if the document is ready.
    pub fn is_document_ready(&self) -> bool {
        (unsafe { call_api_or_panic().wkeIsDocumentReady(*self.inner) }).as_bool()
    }
    /// Check if the webview is awake! May unimplemented!
    pub fn is_awake(&self) -> bool {
        (unsafe { call_api_or_panic().wkeIsAwake(*self.inner) }).as_bool()
    }

    /// Check if the window is transparent.
    pub fn is_transparent(&self) -> bool {
        (unsafe { call_api_or_panic().wkeIsTransparent(*self.inner) }).as_bool()
    }

    /// Force the webview to go back.
    pub fn go_back(&self) -> bool {
        (unsafe { call_api_or_panic().wkeGoBack(*self.inner) }).as_bool()
    }
    /// Send select all command to editor.
    pub fn editor_select_all(&self) {
        unsafe { call_api_or_panic().wkeEditorSelectAll(*self.inner) }
    }
    /// Send unselect all command to editor.
    pub fn editor_unselect(&self) {
        unsafe { call_api_or_panic().wkeEditorUnSelect(*self.inner) }
    }
    /// Send copy command to editor.
    pub fn editor_copy(&self) {
        unsafe { call_api_or_panic().wkeEditorCopy(*self.inner) }
    }
    /// Send cut command to editor.
    pub fn editor_cut(&self) {
        unsafe { call_api_or_panic().wkeEditorCut(*self.inner) }
    }
    /// Send delete command to editor.
    pub fn editor_delete(&self) {
        unsafe { call_api_or_panic().wkeEditorDelete(*self.inner) }
    }
    /// Send undo to editor.
    pub fn editor_undo(&self) {
        unsafe { call_api_or_panic().wkeEditorUndo(*self.inner) }
    }
    /// Send redo command to editor.
    pub fn editor_redo(&self) {
        unsafe { call_api_or_panic().wkeEditorRedo(*self.inner) }
    }
    /// Get the page source.
    pub fn get_source(&self) -> String {
        let source = unsafe { call_api_or_panic().wkeGetSource(*self.inner) };
        assert!(!source.is_null());
        unsafe { CStr::from_ptr(source) }
            .to_string_lossy()
            .to_string()
    }
    /// Get the name. TBD
    pub fn get_name(&self) -> String {
        let name = unsafe { call_api_or_panic().wkeGetName(*self.inner) };
        assert!(!name.is_null());
        unsafe { CStr::from_ptr(name) }
            .to_string_lossy()
            .to_string()
    }
    /// Get the user agent.
    pub fn get_user_agent(&self) -> String {
        let user_agent = unsafe { call_api_or_panic().wkeGetUserAgent(*self.inner) };
        assert!(!user_agent.is_null());
        unsafe { CStr::from_ptr(user_agent) }
            .to_string_lossy()
            .to_string()
    }
    /// Get the url of main frame.
    pub fn get_url(&self) -> String {
        let url = unsafe { call_api_or_panic().wkeGetURL(*self.inner) };
        assert!(!url.is_null());
        unsafe { CStr::from_ptr(url) }.to_string_lossy().to_string()
    }
    /// Get the url of a specified frame.
    pub fn get_frame_url(&self, frame_id: WebFrameHandle) -> String {
        let url = unsafe { call_api_or_panic().wkeGetFrameUrl(*self.inner, frame_id.as_ptr()) };
        assert!(!url.is_null());
        unsafe { CStr::from_ptr(url) }.to_string_lossy().to_string()
    }
    /// Get the webview ID.
    pub fn get_webview_id(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetWebviewId(*self.inner) }
    }
    /// Get the page title.
    pub fn get_title(&self) -> String {
        let title = unsafe { call_api_or_panic().wkeGetTitle(*self.inner) };
        assert!(!title.is_null());
        unsafe { CStr::from_ptr(title) }
            .to_string_lossy()
            .to_string()
    }
    /// Get the page width.
    pub fn get_width(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetWidth(*self.inner) }
    }
    /// Get the page height.
    pub fn get_height(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetHeight(*self.inner) }
    }
    /// Get the page content width.
    pub fn get_content_width(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetContentWidth(*self.inner) }
    }
    /// Get the page content height.
    pub fn get_content_height(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetContentHeight(*self.inner) }
    }
    /// Get the host HWND. Same as `get_window_handle`.
    pub fn get_host_hwnd(&self) -> HWND {
        unsafe { call_api_or_panic().wkeGetHostHWND(*self.inner) }
    }
    /// Get the host HWND.
    pub fn get_window_handle(&self) -> HWND {
        unsafe { call_api_or_panic().wkeGetWindowHandle(*self.inner) }
    }
    /// Get the navigate index.
    pub fn get_navigate_index(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetNavigateIndex(*self.inner) }
    }
    /// Get the cookie.
    pub fn get_cookie(&self) -> String {
        let cookie = unsafe { call_api_or_panic().wkeGetCookie(*self.inner) };
        assert!(!cookie.is_null());
        unsafe { CStr::from_ptr(cookie) }
            .to_string_lossy()
            .to_string()
    }
    /// Get the zoom factor.
    pub fn get_zoom_factor(&self) -> f32 {
        unsafe { call_api_or_panic().wkeGetZoomFactor(*self.inner) }
    }
    /// Get the [`JsExecState`] of main frame.
    pub fn global_exec(&self) -> JsExecState {
        let es = unsafe { call_api_or_panic().wkeGlobalExec(*self.inner) };
        assert!(!es.is_null());
        unsafe { JsExecState::from_ptr(es) }
    }
    /// Get the [`WebFrameHandle`] of main frame.
    pub fn get_main_frame_handle(&self) -> WebFrameHandle {
        let handle = unsafe { call_api_or_panic().wkeWebFrameGetMainFrame(*self.inner) };
        assert!(!handle.is_null());
        unsafe { WebFrameHandle::from_ptr(handle) }
    }
    /// Set the resource GC. TBD
    pub fn set_resource_gc(&self, resource_gc: i32) {
        unsafe { call_api_or_panic().wkeSetResourceGc(*self.inner, resource_gc) }
    }
    /// Set the name of webview.
    pub fn set_webview_name(&self, webview_name: &str) {
        let webview_name = CString::safe_new(webview_name);
        unsafe { call_api_or_panic().wkeSetWebViewName(*self.inner, webview_name.as_ptr()) }
    }
    /// Set view setting [`ViewSettings`] to the webview. Only background color is supported.
    pub fn set_view_settings(&self, setting: &ViewSettings) {
        let setting = wkeViewSettings {
            size: setting.size,
            bgColor: setting.backgroud_color,
        };
        unsafe { call_api_or_panic().wkeSetViewSettings(*self.inner, &setting) }
    }
    /// Enable some experimental function.
    /// debugString：
    ///  - showDevTools: Enable devtools, set param as file:///c:/miniblink-release/front_end/inspector.html (UTF8 encoded)
    ///  - wakeMinInterval: Set the minimum of wake interval, default is 10
    ///  - drawMinInterval: Set the minimum of draw interval, default is 3
    ///  - minimumFontSize: Set the minimum font size
    ///  - minimumLogicalFontSize: Set the minimum logical font size
    ///  - defaultFontSize: Set the default font size
    ///  - defaultFixedFontSize: Set the default fixed font size
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
    /// Enable memory cache. If enabled, pictures in web pages will store in memory.
    pub fn set_memory_cache_enable(&self, memory_cache_enable: bool) {
        unsafe { call_api_or_panic().wkeSetMemoryCacheEnable(*self.inner, memory_cache_enable) }
    }
    /// Enable mouse event.
    pub fn set_mouse_enabled(&self, mouse_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetMouseEnabled(*self.inner, mouse_enabled) }
    }
    /// Enable touch event. If enabled, the mouse event will convert to touch event.
    pub fn set_touch_enabled(&self, touch_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetTouchEnabled(*self.inner, touch_enabled) }
    }
    /// Enable system touch.
    pub fn set_system_touch_enabled(&self, system_touch_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetSystemTouchEnabled(*self.inner, system_touch_enabled) }
    }
    /// Enable context menu.
    pub fn set_context_menu_enabled(&self, context_menu_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetContextMenuEnabled(*self.inner, context_menu_enabled) }
    }
    /// Enable if navigation to new window on clicking on `<a>` link.
    pub fn set_navigation_to_new_window_enabled(&self, navigation_to_new_window_enabled: bool) {
        unsafe {
            call_api_or_panic()
                .wkeSetNavigationToNewWindowEnable(*self.inner, navigation_to_new_window_enabled)
        }
    }
    /// Enable csp check. See [`Same-origin_policy`](https://developer.mozilla.org/en-US/docs/Web/Security/Same-origin_policy).
    pub fn set_csp_check_enable(&self, csp_check_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetCspCheckEnable(*self.inner, csp_check_enabled) }
    }
    /// Enable npapi plugins, such as flash.
    pub fn set_npapi_plugins_enabled(&self, npapi_plugins_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetNpapiPluginsEnabled(*self.inner, npapi_plugins_enabled) }
    }
    /// Enable headless mode.
    pub fn set_headless_enabled(&self, headless_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetHeadlessEnabled(*self.inner, headless_enabled) }
    }
    /// Enable drag.
    pub fn set_drag_enabled(&self, drag_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetDragEnable(*self.inner, drag_enabled) }
    }
    /// Enable drag drop.
    pub fn set_drag_drop_enable(&self, drag_drop_enable: bool) {
        unsafe { call_api_or_panic().wkeSetDragDropEnable(*self.inner, drag_drop_enable) }
    }
    /// Show context menu items.
    pub fn set_context_menu_item_show(&self, item_id: MenuItemId, show: bool) {
        unsafe {
            call_api_or_panic().wkeSetContextMenuItemShow(*self.inner, item_id.to_wke(), show)
        }
    }
    /// Set language.
    pub fn set_language(&self, language: &str) {
        let language = CString::safe_new(language);
        unsafe { call_api_or_panic().wkeSetLanguage(*self.inner, language.as_ptr()) }
    }
    /// Set view net interface.
    pub fn set_view_net_interface(&self, net_interface: &str) {
        let net_interface = CString::safe_new(net_interface);
        unsafe { call_api_or_panic().wkeSetViewNetInterface(*self.inner, net_interface.as_ptr()) }
    }
    /// Set the proxy of the webview.
    pub fn set_view_proxy(&self, proxy: &Proxy) {
        let mut proxy = proxy.to_wke();
        unsafe { call_api_or_panic().wkeSetViewProxy(*self.inner, &mut proxy) }
    }
    /// Set the name. TBD.
    pub fn set_name(&self, name: &str) {
        let name = CString::safe_new(name);
        unsafe { call_api_or_panic().wkeSetName(*self.inner, name.as_ptr()) }
    }
    /// Set the HWND of the webview.
    ///
    /// Note: Only works to the webview created using `create_web_view`.
    pub fn set_handle(&self, hwnd: HWND) {
        unsafe { call_api_or_panic().wkeSetHandle(*self.inner, hwnd.into()) }
    }
    /// Set the webview handle offset.
    pub fn set_handle_offset(&self, x: i32, y: i32) {
        unsafe { call_api_or_panic().wkeSetHandleOffset(*self.inner, x, y) }
    }
    /// Notify the webview with no window to be transparent.
    pub fn set_transparent(&self, transparent: bool) {
        unsafe { call_api_or_panic().wkeSetTransparent(*self.inner, transparent) }
    }
    /// Set the webview user agent.
    pub fn set_user_agent(&self, user_agent: &str) {
        let user_agent = CString::safe_new(user_agent);
        unsafe { call_api_or_panic().wkeSetUserAgent(*self.inner, user_agent.as_ptr()) }
    }
    /// Set the cookie of a url.
    ///
    /// Note: `cookie` needs to be `curl` form, such as `PERSONALIZE=123;expires=Monday, 13-Jun-2022 03:04:55 GMT; domain=.fidelity.com; path=/; secure`.
    pub fn set_cookie(&self, url: &str, cookie: &str) {
        let url = CString::safe_new(url);
        let cookie = CString::safe_new(cookie);
        unsafe { call_api_or_panic().wkeSetCookie(*self.inner, url.as_ptr(), cookie.as_ptr()) }
    }
    /// Visit all cookie using visitor.
    pub fn visit_all_cookie<F>(&self, visitor: F)
    where
        F: Fn(CookieVisitor) -> bool + 'static,
    {
        unsafe extern "C" fn shim<F>(
            params: *mut ::std::os::raw::c_void,
            name: *const ::std::os::raw::c_char,
            value: *const ::std::os::raw::c_char,
            domain: *const ::std::os::raw::c_char,
            path: *const ::std::os::raw::c_char,
            secure: ::std::os::raw::c_int,
            http_only: ::std::os::raw::c_int,
            expires: *mut ::std::os::raw::c_int,
        ) -> bool
        where
            F: Fn(CookieVisitor) -> bool + 'static,
        {
            let visitor =
                CookieVisitor::from_wke(name, value, domain, path, secure, http_only, expires);
            let cb: *mut F = params as _;
            let f = &mut *cb;
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(visitor)));
            r.unwrap_or(false)
        }
        let cb: *mut F = Box::into_raw(Box::new(visitor));
        unsafe { call_api_or_panic().wkeVisitAllCookie(*self.inner, cb as _, Some(shim::<F>)) }
    }
    /// Get favicon. This api must call in `on_loading_finish`.
    pub fn net_get_favicon<F>(&self, callback: F) -> i32
    where
        F: FnMut(&WebView, String, MemBuf) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            c_ptr: *mut ::std::os::raw::c_void,
            url: *const i8,
            buf: *mut wkeMemBuf,
        ) where
            F: FnMut(&WebView, String, MemBuf),
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = c_ptr as _;
            let f = &mut *cb;
            assert!(!url.is_null());
            assert!(!buf.is_null());

            let url = CStr::from_ptr(url).to_string_lossy().to_string();
            let buf = MemBuf::from_ptr(buf);

            let _r =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, url, buf)));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe { call_api_or_panic().wkeNetGetFavicon(*self.inner, Some(shim::<F>), cb as *mut _) }
    }
    /// Get temp callback info.
    pub fn get_temp_callback_info(&self) -> TempCallbackInfo {
        let callback_info = unsafe { call_api_or_panic().wkeGetTempCallbackInfo(*self.inner) };
        assert!(!callback_info.is_null());
        TempCallbackInfo::from_wke(unsafe { *callback_info })
    }
    /// Create the post body elements.
    pub(crate) fn net_create_post_body_elements(&self, length: usize) -> PostBodyElements {
        let elements =
            unsafe { call_api_or_panic().wkeNetCreatePostBodyElements(*self.inner, length) };
        assert!(!elements.is_null());
        unsafe { PostBodyElements::from_ptr(elements) }
    }
    /// Create the post body element.
    pub(crate) fn net_create_post_body_element(&self) -> PostBodyElement {
        let element = unsafe { call_api_or_panic().wkeNetCreatePostBodyElement(*self.inner) };
        assert!(!element.is_null());
        unsafe { PostBodyElement::from_ptr(element) }
    }
    /// Enable cookies. This api will not set curl.
    pub fn set_cookie_enabled(&self, cookie_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetCookieEnabled(*self.inner, cookie_enabled) }
    }
    /// Set local cookie jar path. Defaults to `cookie.dat`.
    pub fn set_cookie_jar_path(&self, path: &str) {
        let path = WkeString::new(path);
        unsafe { call_api_or_panic().wkeSetCookieJarPath(*self.inner, path.as_wcstr_ptr()) }
    }
    /// Set local cookie jar full path.
    pub fn set_cookie_jar_full_path(&self, path: &str) {
        let path = WkeString::new(path);
        unsafe { call_api_or_panic().wkeSetCookieJarFullPath(*self.inner, path.as_wcstr_ptr()) }
    }
    /// Set local storage full path. This api only support dir.
    pub fn set_local_storage_full_path(&self, path: &str) {
        let path = WkeString::new(path);
        unsafe { call_api_or_panic().wkeSetLocalStorageFullPath(*self.inner, path.as_wcstr_ptr()) }
    }
    /// Set media volume. May unimplemented!
    pub fn set_media_volume(&self, media_volume: f32) {
        unsafe { call_api_or_panic().wkeSetMediaVolume(*self.inner, media_volume) }
    }
    /// Get media volume. May unimplemented!
    pub fn get_media_volume(&self) -> f32 {
        unsafe { call_api_or_panic().wkeGetMediaVolume(*self.inner) }
    }
    /// Set zoom factor. Defaults to 1.0.
    pub fn set_zoom_factor(&self, zoom_factor: f32) {
        unsafe { call_api_or_panic().wkeSetZoomFactor(*self.inner, zoom_factor) }
    }
    /// Set if editable. May unimplemented!
    pub fn set_editable(&self, editable: bool) {
        unsafe { call_api_or_panic().wkeSetEditable(*self.inner, editable) }
    }
    /// Perform operation on `cookie` using curl embedded in miniblink.
    ///
    /// Note: This api just executes curl command and does not change javascript content.
    pub fn perform_cookie_command(&self, command: CookieCommand) {
        unsafe { call_api_or_panic().wkePerformCookieCommand(*self.inner, command.into_wke()) }
    }
    /// Get the cursor info type.
    pub fn get_cursor_info_type(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetCursorInfoType(*self.inner) }
    }
    /// Create webview without window, used as offscreen rendering.
    pub fn create_web_view() -> Self {
        let webview = unsafe { call_api_or_panic().wkeCreateWebView() };
        assert!(!webview.is_null());
        Self {
            inner: Rc::new(webview),
        }
    }
    /// Set the cursor info type.
    pub fn set_cursor_info_type(&self, cursor_info_type: i32) {
        unsafe { call_api_or_panic().wkeSetCursorInfoType(*self.inner, cursor_info_type) }
    }
    /// Set drag files.
    pub fn set_drag_files(
        &self,
        client_pos: &POINT,
        screen_pos: &POINT,
        files: &[&str],
        files_count: i32,
    ) {
        let files: Box<[WkeString]> = files.iter().map(|file| WkeString::new(&file)).collect();
        let mut files: Box<[wkeString]> = files.iter().map(|file| file.as_ptr()).collect();
        unsafe {
            call_api_or_panic().wkeSetDragFiles(
                *self.inner,
                client_pos,
                screen_pos,
                files.as_mut_ptr(),
                files_count,
            )
        }
    }
    /// set the device parameter.
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
    /// Set window title.
    pub fn set_window_title(&self, window_title: &str) {
        let window_title = CString::safe_new(window_title);
        unsafe { call_api_or_panic().wkeSetWindowTitle(*self.inner, window_title.as_ptr()) }
    }

    /// Delay miniblink garbage collection with miniseconds.
    pub fn gc(&self, delay_ms: i32) {
        unsafe { call_api_or_panic().wkeGC(*self.inner, delay_ms) }
    }

    /// Get the page pixels.
    ///
    /// bits: a buffer with length width * height * 4 bytes.
    /// pitch: 0
    pub fn paint(&self, bits: &mut [u8], pitch: i32) {
        assert_eq!(
            bits.len(),
            (self.get_width() * self.get_height() * 4) as usize
        );
        unsafe { call_api_or_panic().wkePaint(*self.inner, bits.as_mut_ptr() as _, pitch) }
    }

    /// Get the page pixels.
    ///
    /// bits: a buffer with length of buf_width * buf_height * 4
    /// x_dst, y_dst: where to paint
    /// w, h, x_src, y_src: where to get
    /// copy_alpha: Whether to copy the transparency value of the picture
    ///
    /// Note: This function is generally used in 3D games.
    /// In addition, there are performance issues with frequently using this interface and copying pixels.
    pub fn paint2(
        &self,
        bits: &mut [u8],
        buf_width: i32,
        buf_height: i32,
        x_dst: i32,
        y_dst: i32,
        w: i32,
        h: i32,
        x_src: i32,
        y_src: i32,
        copy_alpha: bool,
    ) {
        unsafe {
            call_api_or_panic().wkePaint2(
                *self.inner,
                bits.as_mut_ptr() as _,
                buf_width,
                buf_height,
                x_dst,
                y_dst,
                w,
                h,
                x_src,
                y_src,
                copy_alpha,
            )
        }
    }

    /// Get the view dc.
    pub fn get_view_dc(&self) -> HDC {
        unsafe { call_api_or_panic().wkeGetViewDC(*self.inner) }
    }

    /// Fire mouse event to miniblink.
    pub fn fire_mouse_event<M, U>(&self, message: M, x: i32, y: i32, flags: U) -> bool
    where
        M: Into<u32>,
        U: Into<u32>,
    {
        unsafe {
            call_api_or_panic()
                .wkeFireMouseEvent(*self.inner, message.into(), x, y, flags.into())
                .as_bool()
        }
    }

    /// Fire context menu event to miniblink.
    pub fn fire_context_menu_event<U>(&self, x: i32, y: i32, flags: U) -> bool
    where
        U: Into<u32>,
    {
        unsafe {
            call_api_or_panic()
                .wkeFireContextMenuEvent(*self.inner, x, y, flags.into())
                .as_bool()
        }
    }

    /// Fire mouse wheel to miniblink.
    pub fn fire_mouse_wheel_event<U>(&self, x: i32, y: i32, delta: i32, flags: U) -> bool
    where
        U: Into<u32>,
    {
        unsafe {
            call_api_or_panic()
                .wkeFireMouseWheelEvent(*self.inner, x, y, delta, flags.into())
                .as_bool()
        }
    }

    /// Send key up event to miniblink.
    ///
    /// vkey_code: <https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx>
    pub fn fire_key_up_event<U>(&self, vkey_code: u32, flags: U, system_key: bool) -> bool
    where
        U: Into<u32>,
    {
        unsafe {
            call_api_or_panic()
                .wkeFireKeyUpEvent(*self.inner, vkey_code, flags.into(), system_key)
                .as_bool()
        }
    }

    /// Send key down event to miniblink.
    ///
    /// vkey_code: <https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx>
    pub fn fire_key_down_event<U>(&self, vkey_code: u32, flags: U, system_key: bool) -> bool
    where
        U: Into<u32>,
    {
        unsafe {
            call_api_or_panic()
                .wkeFireKeyDownEvent(*self.inner, vkey_code, flags.into(), system_key)
                .as_bool()
        }
    }

    /// Send key press event to miniblink.
    ///
    /// char_code: <https://msdn.microsoft.com/en-us/library/windows/desktop/ms646276(v=vs.85).aspx>
    /// vkey_code: <https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx>
    pub fn fire_key_press_event<U>(&self, vkey_code: u32, flags: U, system_key: bool) -> bool
    where
        U: Into<u32>,
    {
        unsafe {
            call_api_or_panic()
                .wkeFireKeyPressEvent(*self.inner, vkey_code, flags.into(), system_key)
                .as_bool()
        }
    }

    /// Fire any message to miniblink.
    pub unsafe fn fire_windows_message(
        &self,
        hwnd: HWND,
        message: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
        result: *mut LRESULT,
    ) -> BOOL {
        call_api_or_panic().wkeFireWindowsMessage(
            *self.inner,
            hwnd,
            message,
            wparam,
            lparam,
            result,
        )
    }

    /// Call on mouse over url changed.
    ///
    /// - param1: a href
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
            call_api_or_panic().wkeOnMouseOverUrlChanged(
                *self.inner,
                Some(shim::<F>),
                cb as *mut _,
            );
        }
    }

    /// Call on title changed.
    ///
    /// - param1: document title
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
            call_api_or_panic().wkeOnTitleChanged(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on url changed.
    ///
    /// - param1: url
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
            call_api_or_panic().wkeOnURLChanged(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call when JavaScript call `alert`.
    ///
    /// - param1: message
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
            call_api_or_panic().wkeOnAlertBox(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call when JavaScript call `confirm`.
    ///
    /// - param1: message        
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
            call_api_or_panic().wkeOnConfirmBox(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call when JavaScript call `prompt`.
    ///
    /// - param1: message
    /// - param2: default result
    /// - param3: result
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
            call_api_or_panic().wkeOnPromptBox(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on navigation
    ///
    /// - param1: navigation type
    /// - param2: url
    /// - return: bool, means if continue navigation
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

            let navigation_type = NavigationType::from_wke(navigation_type);
            let url = WkeStr::from_ptr(url).to_string();

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(&mut wv, navigation_type, url)
            }));

            r.unwrap_or(true)
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnNavigation(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    /// Call on document ready (in JavaScript is body onload event).
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
            call_api_or_panic().wkeOnDocumentReady(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    /// Call on download.
    ///
    /// - param1: download url.
    /// - return: bool, means if continue to download.
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
            call_api_or_panic().wkeOnDownload(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    /// Call on window closing if the WebView is created as real window.
    ///
    /// - return: bool, means if close window.    
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
            call_api_or_panic().wkeOnWindowClosing(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    /// Call on window destroy if the WebView is created as real window.
    ///
    /// - return: bool. The api can not prevent window closing.
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
            call_api_or_panic().wkeOnWindowDestroy(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on paint updated. You may call `BitBlt` to copy pixels from one to anthor.
    ///
    /// - param1: HDC of the webview.
    /// - param2: logical position x
    /// - param3: logical position y
    /// - param4: logical width to x
    /// - param5: logical height to y
    pub fn on_paint_updated<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, HDC, i32, i32, i32, i32) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv_ptr: miniblink_sys::wkeWebView,
            param: *mut ::std::os::raw::c_void,
            hdc: HDC,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            cx: ::std::os::raw::c_int,
            cy: ::std::os::raw::c_int,
        ) where
            F: FnMut(&mut WebView, HDC, i32, i32, i32, i32) + 'static,
        {
            let mut wv = WebView::from_ptr(wv_ptr);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(&mut wv, hdc, x, y, cx, cy)
            }));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnPaintUpdated(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on paint updated. May change!
    ///
    /// - param1: buffer, with length width * height * 4
    /// - param2: rect
    /// - param3: width
    /// - param4: height
    pub fn on_paint_bit_updated<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &[u8], Rect, i32, i32) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv: wkeWebView,
            param: *mut ::std::os::raw::c_void,
            buffer: *const ::std::os::raw::c_void,
            r: *const wkeRect,
            width: ::std::os::raw::c_int,
            height: ::std::os::raw::c_int,
        ) where
            F: FnMut(&mut WebView, &[u8], Rect, i32, i32) + 'static,
        {
            let mut wv = WebView::from_ptr(wv);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            let wkeRect { x, y, w, h } = *r;
            let buffer =
                std::ptr::slice_from_raw_parts(buffer as *const u8, (width * height * 4) as usize);
            let buffer = &*buffer;
            let r = Rect::new(x, y, w, h);

            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(&mut wv, buffer, r, width, height)
            }));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnPaintBitUpdated(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on create new webview window after clicking `<a>` tag.
    ///
    /// - param1: navigation type
    /// - param2: url
    /// - param3: window features
    pub fn on_create_view<F>(&self, callback: F)
    where
        F: Fn(WebView, NavigationType, String, WindowFeatures) -> WebView + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv: wkeWebView,
            param: *mut ::std::os::raw::c_void,
            navigation_type: wkeNavigationType,
            url: wkeString,
            window_features: *const wkeWindowFeatures,
        ) -> wkeWebView
        where
            F: Fn(WebView, NavigationType, String, WindowFeatures) -> WebView + 'static,
        {
            let wv = WebView::from_ptr(wv);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            let navigation_type = NavigationType::from_wke(navigation_type);
            assert!(!url.is_null());
            assert!(!window_features.is_null());

            let url = WkeStr::from_ptr(url).to_string();
            let window_features = WindowFeatures::from_wke(*window_features);
            let wv1 = wv.clone();

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(wv, navigation_type, url, window_features)
            }));

            let wv = r.unwrap_or(wv1);
            *wv.inner
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnCreateView(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on reponse from server after request sent.
    ///
    /// - param1: url
    /// - param2: netjob
    /// - return: bool, need document!
    pub fn net_on_response<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String, NetJob) -> bool + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv: wkeWebView,
            param: *mut ::std::os::raw::c_void,
            url: *const ::std::ffi::c_char,
            job: wkeNetJob,
        ) -> bool
        where
            F: FnMut(&mut WebView, String, NetJob) -> bool + 'static,
        {
            let mut wv = WebView::from_ptr(wv);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            assert!(!url.is_null());
            assert!(!job.is_null());

            let url = CStr::from_ptr(url).to_string_lossy().to_string();
            let job = NetJob::from_ptr(job);

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, url, job)));
            r.unwrap_or(false)
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeNetOnResponse(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on JavaScript calling `console`.
    ///
    /// param1: console level.
    /// param2: message.
    /// param3: source name.
    /// param4: source line.
    /// param5: stack trace.
    pub fn on_console<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, ConsoleLevel, String, String, u32, String) -> WebView + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv: wkeWebView,
            param: *mut ::std::os::raw::c_void,
            level: wkeConsoleLevel,
            message: wkeString,
            source_name: wkeString,
            source_line: ::std::os::raw::c_uint,
            stack_trace: wkeString,
        ) where
            F: FnMut(&mut WebView, ConsoleLevel, String, String, u32, String) -> WebView + 'static,
        {
            let mut wv = WebView::from_ptr(wv);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            let level = ConsoleLevel::from_wke(level);

            assert!(!message.is_null());
            assert!(!source_name.is_null());
            assert!(!stack_trace.is_null());

            let message = WkeStr::from_ptr(message).to_string();
            let source_name = WkeStr::from_ptr(source_name).to_string();
            let stack_trace = WkeStr::from_ptr(stack_trace).to_string();

            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(
                    &mut wv,
                    level,
                    message,
                    source_name,
                    source_line,
                    stack_trace,
                )
            }));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnConsole(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on load url begin (before any web request).
    ///
    /// - param1: url
    /// - param2: netjob
    /// - return: bool, means if reject web requet. panic then reject.
    ///
    /// Note: if call hook_request on netjob, miniblink will not process the request, instead hook the request and send request to on_load_url_end handler.
    pub fn on_load_url_begin<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String, NetJob) -> bool + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv: wkeWebView,
            param: *mut ::std::os::raw::c_void,
            url: *const ::std::ffi::c_char,
            job: wkeNetJob,
        ) -> bool
        where
            F: FnMut(&mut WebView, String, NetJob) -> bool + 'static,
        {
            let mut wv = WebView::from_ptr(wv);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            assert!(!url.is_null());
            assert!(!job.is_null());

            let url = CStr::from_ptr(url).to_string_lossy().to_string();
            let job = NetJob::from_ptr(job);

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, url, job)));
            r.unwrap_or(false)
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnLoadUrlBegin(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on load url finish.
    ///
    /// - param1: url
    /// - param2: netjob
    /// - param3: length
    pub fn on_load_url_finish<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String, NetJob, i32) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv: wkeWebView,
            param: *mut ::std::os::raw::c_void,
            url: *const ::std::ffi::c_char,
            job: wkeNetJob,
            len: ::std::os::raw::c_int,
        ) where
            F: FnMut(&mut WebView, String, NetJob, i32) + 'static,
        {
            let mut wv = WebView::from_ptr(wv);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            assert!(!url.is_null());
            assert!(!job.is_null());

            let url = CStr::from_ptr(url).to_string_lossy().to_string();
            let job = NetJob::from_ptr(job);

            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(&mut wv, url, job, len)
            }));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnLoadUrlFinish(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on load url fail.
    ///
    /// - param1: url
    /// - param2: netjob
    pub fn on_load_url_fail<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String, NetJob) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv: wkeWebView,
            param: *mut ::std::os::raw::c_void,
            url: *const ::std::ffi::c_char,
            job: wkeNetJob,
        ) where
            F: FnMut(&mut WebView, String, NetJob) + 'static,
        {
            let mut wv = WebView::from_ptr(wv);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            assert!(!url.is_null());
            assert!(!job.is_null());

            let url = CStr::from_ptr(url).to_string_lossy().to_string();
            let job = NetJob::from_ptr(job);

            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, url, job)));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnLoadUrlFail(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Call on will media load. May change!
    /// 
    /// - param1: url
    /// - param2: info
    pub fn on_will_media_load<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String, MediaLoadInfo) + 'static,
    {
        unsafe extern "C" fn shim<F>(
            wv: wkeWebView,
            param: *mut ::std::os::raw::c_void,
            url: *const ::std::os::raw::c_char,
            info: *mut wkeMediaLoadInfo,
        ) where
            F: FnMut(&mut WebView, String, MediaLoadInfo) + 'static,
        {
            let mut wv = WebView::from_ptr(wv);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            assert!(!url.is_null());
            assert!(!info.is_null());

            let url = CStr::from_ptr(url).to_string_lossy().to_string();
            let info = MediaLoadInfo::from_wke(*info);

            let _ =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, url, info)));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnWillMediaLoad(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// Check is the frame is the main frame.
    pub fn is_main_frame(&self, frame: WebFrameHandle) -> bool {
        (unsafe { call_api_or_panic().wkeIsMainFrame(*self.inner, frame.as_ptr()) }).as_bool()
    }
    /// Get main frame.
    pub fn web_frame_get_main_frame(&self) -> WebFrameHandle {
        let main_frame = unsafe { call_api_or_panic().wkeWebFrameGetMainFrame(*self.inner) };
        assert!(!main_frame.is_null());
        unsafe { WebFrameHandle::from_ptr(main_frame) }
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
        Self::create_web_window(WindowType::Popup, HWND(0), 0, 0, 200, 200)
    }
}

/// Extra API for WebView
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
