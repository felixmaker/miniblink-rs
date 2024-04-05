use std::ffi::{CStr, CString};
use std::rc::Rc;

use miniblink_sys::{wkeMemBuf, wkeNavigationType, wkeString, wkeViewSettings, wkeWebView};

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
/// Wrapper to [`miniblink_sys::wkeWebView`].
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
    /// #Note
    /// This method creates a real window.
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

    /// Set if show the window.
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
    /// Resize the webview window. Same as [`resize`].
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
    /// # Note
    /// Only support clearing all page cookies.
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
    /// Sleep. Unimplemented!
    pub fn sleep(&self) {
        unsafe { call_api_or_panic().wkeSleep(*self.inner) }
    }
    /// Wake. Unimplemented!
    pub fn wake(&self) {
        unsafe { call_api_or_panic().wkeWake(*self.inner) }
    }
    /// Run a script by frame. Param `is_in_closure` means if script needs to be in the form of `function() {}`.
    ///
    /// #Note
    /// if `is_in_closure` is `true`, keyword `return` is required to get returned value.
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

    /// Set if the window is enabled.
    pub fn enable_window(&self, enable: bool) {
        unsafe { call_api_or_panic().wkeEnableWindow(*self.inner, enable) }
    }

    /// Check if the webview can go back.
    pub fn can_go_back(&self) -> bool {
        (unsafe { call_api_or_panic().wkeCanGoBack(*self.inner) } != 0)
    }
    /// Check if the webview can go forward.
    pub fn can_go_forward(&self) -> bool {
        (unsafe { call_api_or_panic().wkeCanGoForward(*self.inner) } != 0)
    }
    /// Check if the document is ready.
    pub fn is_document_ready(&self) -> bool {
        (unsafe { call_api_or_panic().wkeIsDocumentReady(*self.inner) } != 0)
    }
    /// Check if the webview is awake! Unimplemented!
    pub fn is_awake(&self) -> bool {
        (unsafe { call_api_or_panic().wkeIsAwake(*self.inner) } != 0)
    }

    /// Check if the window is transparent.
    pub fn is_transparent(&self) -> bool {
        (unsafe { call_api_or_panic().wkeIsTransparent(*self.inner) } != 0)
    }

    /// Force the webview to go back.
    pub fn go_back(&self) -> bool {
        (unsafe { call_api_or_panic().wkeGoBack(*self.inner) } != 0)
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
    /// See wkeGetSource.
    pub fn get_source(&self) -> String {
        let source = unsafe { call_api_or_panic().wkeGetSource(*self.inner) };
        assert!(!source.is_null());
        unsafe { CStr::from_ptr(source) }
            .to_string_lossy()
            .to_string()
    }
    /// See wkeGetName.
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
    /// See wkeGetWebviewId.
    pub fn get_webview_id(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetWebviewId(*self.inner) }
    }
    /// See the page title.
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
    /// See the page content width.
    pub fn get_content_width(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetContentWidth(*self.inner) }
    }
    /// See the page content height.
    pub fn get_content_height(&self) -> i32 {
        unsafe { call_api_or_panic().wkeGetContentHeight(*self.inner) }
    }
    /// Get the host HWND. Same as [`get_window_handle`].
    pub fn get_host_hwnd(&self) -> Handle {
        let hwnd = unsafe { call_api_or_panic().wkeGetHostHWND(*self.inner) };
        Handle::from(hwnd)
    }
    /// Get the host HWND.
    pub fn get_window_handle(&self) -> Handle {
        let hwnd = unsafe { call_api_or_panic().wkeGetWindowHandle(*self.inner) };
        Handle::from(hwnd)
    }
    /// See wkeGetNavigateIndex.
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
    /// See wkeSetResourceGc.
    pub fn set_resource_gc(&self, resource_gc: i32) {
        unsafe { call_api_or_panic().wkeSetResourceGc(*self.inner, resource_gc) }
    }
    /// See wkeSetWebViewName.
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
    ///  - "showDevTools"  Enable devtools, set param as file:///c:/miniblink-release/front_end/inspector.html (UTF8 encoded)
    ///  - "wakeMinInterval"	Set the minimum of wake interval, default is 10
    ///  - "drawMinInterval"	Set the minimum of draw interval, default is 3
    ///  - "minimumFontSize"	Set the minimum font size
    ///  - "minimumLogicalFontSize"	Set the minimum logical font size
    ///  - "defaultFontSize"    Set the default font size
    ///  - "defaultFixedFontSize"	Set the default fixed font size
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
    /// See wkeSetSystemTouchEnabled.
    pub fn set_system_touch_enabled(&self, system_touch_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetSystemTouchEnabled(*self.inner, system_touch_enabled) }
    }
    /// See wkeSetContextMenuEnabled.
    pub fn set_context_menu_enabled(&self, context_menu_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetContextMenuEnabled(*self.inner, context_menu_enabled) }
    }
    /// Enable if navigation to new window on clicking on <a> link.
    pub fn set_navigation_to_new_window_enabled(&self, navigation_to_new_window_enabled: bool) {
        unsafe {
            call_api_or_panic()
                .wkeSetNavigationToNewWindowEnable(*self.inner, navigation_to_new_window_enabled)
        }
    }
    /// Set if enable csp check. See [`Same-origin_policy`](https://developer.mozilla.org/en-US/docs/Web/Security/Same-origin_policy).
    pub fn set_csp_check_enable(&self, csp_check_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetCspCheckEnable(*self.inner, csp_check_enabled) }
    }
    /// Set if enable npapi plugins, such as flash.
    pub fn set_npapi_plugins_enabled(&self, npapi_plugins_enabled: bool) {
        unsafe { call_api_or_panic().wkeSetNpapiPluginsEnabled(*self.inner, npapi_plugins_enabled) }
    }
    /// Set if enable headless.
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
    /// Set the proxy of the webview.
    pub fn set_view_proxy(&self, proxy: &Proxy) {
        let mut proxy = proxy.to_wke();
        unsafe { call_api_or_panic().wkeSetViewProxy(*self.inner, &mut proxy) }
    }
    /// See wkeSetName.
    pub fn set_name(&self, name: &str) {
        let name = CString::safe_new(name);
        unsafe { call_api_or_panic().wkeSetName(*self.inner, name.as_ptr()) }
    }
    /// Set the HWND of the webview.
    ///
    /// # Note
    /// Only works to the webview created using [`create_web_view`]
    pub fn set_handle(&self, hwnd: Handle) {
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
    /// Set media volume. Unimplemented!
    pub fn set_media_volume(&self, media_volume: f32) {
        unsafe { call_api_or_panic().wkeSetMediaVolume(*self.inner, media_volume) }
    }
    /// Get media volume. Unimplemented!
    pub fn get_media_volume(&self) -> f32 {
        unsafe { call_api_or_panic().wkeGetMediaVolume(*self.inner) }
    }
    /// Set zoom factor. Defaults to 1.0.
    pub fn set_zoom_factor(&self, zoom_factor: f32) {
        unsafe { call_api_or_panic().wkeSetZoomFactor(*self.inner, zoom_factor) }
    }
    /// See wkeSetEditable. Unimplemented!
    pub fn set_editable(&self, editable: bool) {
        unsafe { call_api_or_panic().wkeSetEditable(*self.inner, editable) }
    }
    /// Perform operation on `cookie` using curl embedded in miniblink.
    ///
    /// Note: This api just executes curl command and does not change javascript content.
    pub fn perform_cookie_command(&self, command: CookieCommand) {
        unsafe { call_api_or_panic().wkePerformCookieCommand(*self.inner, command.into_wke()) }
    }
    /// Set a use value.
    pub fn set_user_key_value() {
        todo!()
    }
    /// Get a use value.
    pub fn get_user_key_value() {
        todo!()
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
        client_pos: &Point,
        screen_pos: &Point,
        files: &[&str],
        files_count: i32,
    ) {
        let client_pos = client_pos.to_wke();
        let screen_pos = screen_pos.to_wke();
        let files: Box<[WkeString]> = files.iter().map(|file| WkeString::new(&file)).collect();
        let mut files: Box<[wkeString]> = files.iter().map(|file| file.as_ptr()).collect();
        unsafe {
            call_api_or_panic().wkeSetDragFiles(
                *self.inner,
                &client_pos,
                &screen_pos,
                files.as_mut_ptr(),
                files_count,
            )
        }
    }
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

    /// Delay miniblink garbage collection with miniseconds.
    pub fn gc(&self, delay_ms: i32) {
        unsafe { call_api_or_panic().wkeGC(*self.inner, delay_ms) }
    }

    ///
    ///
    /// 获取页面的像素的简化版函数
    ///
    /// bits：外部申请并传递给mb的buffer，大小是webview宽度 * 高度 * 4 字节。
    /// pitch：填0即可。这个参数玩过directX的人应该懂
    pub fn paint() {
        todo!()
    }

    /// 参数：
    /// bits	外部申请并传递给mb的buffer，大小是bufWid * bufHei * 4 字节
    /// bufWid、bufHei	bits的宽高
    /// xDst、yDst	绘制到bits的哪个坐标
    /// w、h、xSrc、ySrc	mb需要取的画面的起始坐标
    /// bCopyAlpha	是否拷贝画面的透明度值
    /// 注意：此函数一般给3d游戏使用。另外频繁使用此接口并拷贝像素有性能问题。最好用wkeGetViewDC再去拷贝dc。
    pub fn paint2() {
        todo!()
    }

    /// 获取webview的DC
    pub fn get_view_dc() {
        todo!()
    }

    /// 向mb发送鼠标消息
    /// 参数：
    /// message：可取WM_MOUSELEAVE等Windows相关鼠标消息
    /// x、y：坐标
    /// flags：可取值有WKE_CONTROL、WKE_SHIFT、WKE_LBUTTON、WKE_MBUTTON、WKE_RBUTTON，可通过“或”操作并联。
    pub fn fire_mouse_event() {
        todo!()
    }

    /// 向mb发送菜单消息（未实现）
    pub fn fire_context_menu_event() {
        todo!()
    }

    /// 向mb发送滚轮消息，用法和参数类似wkeFireMouseEvent。
    pub fn fire_mouse_wheel_event() {
        todo!()
    }

    /// 向mb发送WM_KEYUP消息，
    /// 参数：
    /// virtualKeyCode：见https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx flags：可取值有WKE_REPEAT、WKE_EXTENDED，可通过“或”操作并联。 systemKey：暂时没用
    pub fn fire_key_up_event() {
        todo!()
    }

    /// 向mb发送WM_KEYUP消息，
    /// 参数：
    /// virtualKeyCode：见https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx flags：可取值有WKE_REPEAT、WKE_EXTENDED，可通过“或”操作并联。 systemKey：暂时没用
    pub fn fire_key_down_event() {
        todo!()
    }
    /// 向mb发送WM_KEYUP消息，
    /// charCode：WM_CHAR消息的The character code of the key.见https://msdn.microsoft.com/en-us/library/windows/desktop/ms646276(v=vs.85).aspx
    /// 参数：
    /// virtualKeyCode：见https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx flags：可取值有WKE_REPEAT、WKE_EXTENDED，可通过“或”操作并联。 systemKey：暂时没用
    pub fn fire_key_press_event() {
        todo!()
    }

    /// 向mb发送任意windows消息。不过目前mb主要用来处理光标相关。mb在无窗口模式下，要响应光标事件，需要通过本函数手动发送光标消息
    pub fn fire_windows_message() {
        todo!()
    }

    /// 鼠标划过的元素，如果是，则调用此回调，并发送a标签的url
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

    /// 设置标题变化的通知回调      
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
    /// url改变回调         
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
    /// 网页调用alert会走到这个接口填入的回调
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
            call_api_or_panic().wkeOnConfirmBox(*self.inner, Some(shim::<F>), cb as *mut _);
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
            call_api_or_panic().wkeOnPromptBox(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
    /// 网页开始浏览将触发回调        
    /// 注意：wkeNavigationCallback回调的返回值，如果是true，表示可以继续进行浏览，false表示阻止本次浏览。
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

            r.unwrap_or(true)
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().wkeOnNavigation(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    /// 对应js里的body onload事件         
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

    /// 页面下载事件回调。点击某些链接，触发下载会调用       
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

    /// wkeWebView如果是真窗口模式，则在收到WM_CLODE消息时触发此回调。可以通过在回调中返回false拒绝关闭窗口    
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

    /// 窗口即将被销毁时触发回调。不像wkeOnWindowClosing，这个操作无法取消
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
    /// 和上个接口不同的是，回调多了个参数
    pub fn on_url_changed2() {
        todo!()
    }
    /// 页面有任何需要刷新的地方，将调用此回调
    pub fn on_paint_updated() {
        todo!()
    }
    /// 同上。不同的是回调过来的是填充好像素的buffer，而不是DC。方便嵌入到游戏中做离屏渲染
    pub fn on_paint_bit_updated() {
        todo!()
    }
    /// 网页点击a标签创建新窗口时将触发回调
    pub fn on_create_view() {
        todo!()
    }
    /// 同上。区别是wkeDocumentReady2Callback多了wkeWebFrameHandle frameId参数。可以判断是否是主frame
    pub fn on_document_ready2() {
        todo!()
    }
    /// 一个网络请求发送后，收到服务器response触发回调
    pub fn net_on_response() {
        todo!()
    }
    /// 网页调用console触发
    pub fn on_console() {
        todo!()
    }
    /// 暂时未实现
    pub fn set_ui_thread_callback() {
        todo!()
    }
    ///任何网络请求发起前会触发此回调
    /// 参数：typedef bool(*wkeLoadUrlBeginCallback)(wkeWebView webView, void* param, const char *url, void *job)
    /// 注意：
    /// 1，此回调功能强大，在回调里，如果对job设置了wkeNetHookRequest，则表示mb会缓存获取到的网络数据，并在这次网络请求 结束后调用wkeOnLoadUrlEnd设置的回调，同时传递缓存的数据。在此期间，mb不会处理网络数据。
    /// 2，如果在wkeLoadUrlBeginCallback里没设置wkeNetHookRequest，则不会触发wkeOnLoadUrlEnd回调。
    /// 3，如果wkeLoadUrlBeginCallback回调里返回true，表示mb不处理此网络请求（既不会发送网络请求）。返回false，表示mb依然会发送网络请求。
    /// 用法举例：
    pub fn on_load_url_begin() {
        todo!()
    }
    /// javascript的v8执行环境被创建时触发此回调
    /// 注意：每个frame创建时都会触发此回调
    pub fn on_did_create_script_context() {
        todo!()
    }
    ///每个frame的javascript的v8执行环境被关闭时触发此回调
    pub fn on_will_release_script_context() {
        todo!()
    }
    /// video等多媒体标签创建时触发此回调
    pub fn on_will_media_load() {
        todo!()
    }
    ///判断frameId是否是主frame
    pub fn is_main_frame() {
        todo!()
    }
    /// 获取主frame的句柄
    pub fn web_frame_get_main_frame() {
        todo!()
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
