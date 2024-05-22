mod traits;

pub use traits::*;

use crate::call_api_or_panic;
use crate::util::*;
use miniblink_sys::*;
use std::ffi::{CStr, CString};

/// Window Type.
#[repr(i32)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WindowType {
    /// Popup type
    Popup = MB_WINDOW_TYPE_POPUP,
    /// Transparent type. Achieved using layer window.    
    Transparent = MB_WINDOW_TYPE_TRANSPARENT,
    /// Control type. Create window as child window. Requied parent.
    Control = MB_WINDOW_TYPE_CONTROL,
}

type WebViewWrapper = std::rc::Rc<mbWebView>;

#[derive(Clone, Debug)]
#[repr(transparent)]
/// Webview
pub struct WebView {
    inner: WebViewWrapper,
}

impl Default for WebView {
    fn default() -> Self {
        Self::create_web_window(WindowType::Popup, 0, 0, 0, 600, 400)
    }
}

impl WebView {
    /// Creates a webview window.
    ///
    /// Note: This method creates a real window.
    pub fn create_web_window(
        window_type: WindowType,
        handle: isize,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Self {
        let webview = unsafe {
            call_api_or_panic().mbCreateWebWindow(
                window_type as _,
                handle as _,
                x,
                y,
                width,
                height,
            )
        };
        assert!(webview != 0);
        WebView {
            inner: WebViewWrapper::new(webview),
        }
    }

    /// Wraps a raw `mbWebView`.
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_ptr(ptr: mbWebView) -> Self {
        assert!(ptr != 0);
        let ptr = WebViewWrapper::into_raw(WebViewWrapper::from(ptr));
        WebViewWrapper::increment_strong_count(ptr); // Avoid destroy the webview instance.
        let inner = WebViewWrapper::from_raw(ptr);
        WebView { inner }
    }

    /// Return the inner pointer to `mbWebView`.
    /// # Safety
    /// Can return multiple mutable pointers to the same item.
    pub unsafe fn as_ptr(&self) -> mbWebView {
        let ptr = WebViewWrapper::into_raw(WebViewWrapper::clone(&self.inner));
        WebViewWrapper::increment_strong_count(ptr);
        let inner = WebViewWrapper::from_raw(ptr);
        *inner
    }
}

impl Drop for WebView {
    fn drop(&mut self) {
        if WebViewWrapper::strong_count(&self.inner) == 1 {
            unsafe {
                call_api_or_panic().mbDestroyWebView(*self.inner);
            }
        }
    }
}

impl WebViewOperation for WebView {
    fn load_html_with_base_url(&self, html: &str, base_url: &str) {
        let html = CString::safe_new(html);
        let base_url = CString::safe_new(base_url);
        unsafe {
            call_api_or_panic().mbLoadHtmlWithBaseUrl(*self.inner, html.as_ptr(), base_url.as_ptr())
        }
    }

    fn load_url(&self, url: &str) {
        let url = CString::safe_new(url);
        unsafe { call_api_or_panic().mbLoadURL(*self.inner, url.as_ptr()) }
    }

    fn reload(&self) {
        unsafe { call_api_or_panic().mbReload(*self.inner) }
    }

    fn stop_loading(&self) {
        unsafe { call_api_or_panic().mbStopLoading(*self.inner) }
    }

    fn go_back(&self) {
        unsafe { call_api_or_panic().mbGoBack(*self.inner) }
    }

    fn go_forward(&self) {
        unsafe { call_api_or_panic().mbGoForward(*self.inner) }
    }

    fn set_user_agent(&self, user_agent: &str) {
        let user_agent = CString::safe_new(user_agent);
        unsafe { call_api_or_panic().mbSetUserAgent(*self.inner, user_agent.as_ptr()) }
    }

    fn is_mainframe(&self, frame_handle: WebFrameHandle) -> bool {
        unsafe { call_api_or_panic().mbIsMainFrame(*self.inner, frame_handle as _) != 0 }
    }
}

impl WebViewEditorOperation for WebView {
    fn editor_copy(&self) {
        unsafe { call_api_or_panic().mbEditorCopy(*self.inner) }
    }

    fn editor_cut(&self) {
        unsafe { call_api_or_panic().mbEditorCut(*self.inner) }
    }

    fn editor_delete(&self) {
        unsafe { call_api_or_panic().mbEditorDelete(*self.inner) }
    }

    fn editor_paste(&self) {
        unsafe { call_api_or_panic().mbEditorPaste(*self.inner) }
    }

    fn editor_redo(&self) {
        unsafe { call_api_or_panic().mbEditorRedo(*self.inner) }
    }

    fn editor_select_all(&self) {
        unsafe { call_api_or_panic().mbEditorSelectAll(*self.inner) }
    }

    fn editor_unselect(&self) {
        unsafe { call_api_or_panic().mbEditorUnSelect(*self.inner) }
    }

    fn editor_undo(&self) {
        unsafe { call_api_or_panic().mbEditorUndo(*self.inner) }
    }
}

impl WebWindowOperation for WebView {
    fn show(&self) {
        unsafe { call_api_or_panic().mbShowWindow(*self.inner, 1) }
    }

    fn hide(&self) {
        unsafe { call_api_or_panic().mbShowWindow(*self.inner, 0) }
    }

    fn resize(&self, w: i32, h: i32) {
        unsafe { call_api_or_panic().mbResize(*self.inner, w, h) }
    }

    fn set_focus(&self) {
        unsafe { call_api_or_panic().mbSetFocus(*self.inner) }
    }

    fn kill_focus(&self) {
        unsafe { call_api_or_panic().mbKillFocus(*self.inner) }
    }

    fn move_window(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { call_api_or_panic().mbMoveWindow(*self.inner, x, y, w, h) }
    }

    fn move_to_center(&self) {
        unsafe { call_api_or_panic().mbMoveToCenter(*self.inner) }
    }

    fn set_window_title(&self, title: &str) {
        let title = CString::safe_new(title);
        unsafe { call_api_or_panic().mbSetWindowTitle(*self.inner, title.as_ptr()) }
    }
}

impl WebViewSwitch for WebView {
    fn enable_context_menu(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetContextMenuEnabled(*self.inner, enabled) }
    }

    fn enable_cookie(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetCookieEnabled(*self.inner, enabled) }
    }

    fn enable_csp_check(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetCspCheckEnable(*self.inner, enabled) }
    }

    fn enable_disk_cache(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetDiskCacheEnabled(*self.inner, enabled) }
    }

    fn enable_drag_drop(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetDragDropEnable(*self.inner, enabled) }
    }

    fn enable_drag(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetDragEnable(*self.inner, enabled) }
    }

    fn enable_headless(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetHeadlessEnabled(*self.inner, enabled) }
    }

    fn enable_memory_cache(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetMemoryCacheEnable(*self.inner, enabled) }
    }

    fn enable_mouse(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetMouseEnabled(*self.inner, enabled) }
    }

    fn enable_navigation_to_new_window(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetNavigationToNewWindowEnable(*self.inner, enabled) }
    }

    fn enable_nodejs(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetNodeJsEnable(*self.inner, enabled) }
    }

    fn enable_npapi_plugins(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetNpapiPluginsEnabled(*self.inner, enabled) }
    }

    fn enable_system_touch(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetSystemTouchEnabled(*self.inner, enabled) }
    }

    fn enable_touch(&self, enabled: bool) {
        let enabled = if enabled { 1 } else { 0 };
        unsafe { call_api_or_panic().mbSetTouchEnabled(*self.inner, enabled) }
    }
}

impl WebViewEvent for WebView {
    fn on_title_changed<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String) + 'static,
    {
        unsafe extern "stdcall" fn shim<F>(
            webview: mbWebView,
            param: *mut ::std::os::raw::c_void,
            title: *const utf8,
        ) where
            F: FnMut(&mut WebView, String) + 'static,
        {
            let mut wv = WebView::from_ptr(webview);
            let cb: *mut F = param as _;
            let f = &mut *cb;
            let title = CStr::from_ptr(title).to_string_lossy().to_string();

            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv, title)));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnTitleChanged(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    fn on_url_changed<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, String, bool, bool) + 'static,
    {
        unsafe extern "stdcall" fn shim<F>(
            webview: mbWebView,
            param: *mut ::std::os::raw::c_void,
            url: *const utf8,
            can_go_back: BOOL,
            can_go_forward: BOOL,
        ) where
            F: FnMut(&mut WebView, String, bool, bool) + 'static,
        {
            let mut wv = WebView::from_ptr(webview);
            let cb: *mut F = param as _;
            let f = &mut *cb;
            let url = CStr::from_ptr(url).to_string_lossy().to_string();
            let can_go_back = can_go_back != 0;
            let can_go_forward = can_go_forward != 0;

            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(&mut wv, url, can_go_back, can_go_forward)
            }));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnURLChanged(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    fn on_document_ready<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, WebFrameHandle) + 'static,
    {
        unsafe extern "stdcall" fn shim<F>(
            webwiew: mbWebView,
            param: *mut ::std::os::raw::c_void,
            frame_id: mbWebFrameHandle,
        ) where
            F: FnMut(&mut WebView, WebFrameHandle) + 'static,
        {
            let mut wv = WebView::from_ptr(webwiew);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            let _r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(&mut wv, frame_id as _)
            }));
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnDocumentReady(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    fn on_close<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView) -> bool + 'static,
    {
        unsafe extern "stdcall" fn shim<F>(
            webview: mbWebView,
            param: *mut ::std::os::raw::c_void,
            _unuse: *mut ::std::os::raw::c_void,
        ) -> BOOL
        where
            F: FnMut(&mut WebView) -> bool + 'static,
        {
            let mut wv = WebView::from_ptr(webview);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv)));

            if r.unwrap_or(true) {
                1
            } else {
                0
            }
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnClose(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }

    fn on_destroy<F>(&self, callback: F)
    where
        F: FnMut(&mut Self) -> bool + 'static,
    {
        unsafe extern "stdcall" fn shim<F>(
            webview: mbWebView,
            param: *mut ::std::os::raw::c_void,
            _unuse: *mut ::std::os::raw::c_void,
        ) -> BOOL
        where
            F: FnMut(&mut WebView) -> bool + 'static,
        {
            let mut wv = WebView::from_ptr(webview);
            let cb: *mut F = param as _;
            let f = &mut *cb;

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wv)));

            if r.unwrap_or(true) {
                1
            } else {
                0
            }
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnDestroy(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
}

impl WebViewJsCall for WebView {
    fn eval(&self, frame_id: isize, script: &str, is_in_closure: bool) -> String {
        let script = CString::safe_new(script);
        let is_in_closure = if is_in_closure { 1 } else { 0 };
        unsafe {
            let result = call_api_or_panic().mbRunJsSync(
                *self.inner,
                frame_id as _,
                script.as_ptr(),
                is_in_closure,
            );

            let es = call_api_or_panic().mbGetGlobalExecByFrame(*self.inner, frame_id as _);
            let result = call_api_or_panic().mbJsToString(es, result);
            CStr::from_ptr(result).to_string_lossy().to_string()
        }
    }

    fn on_query<F>(&self, callback: F)
    where
        F: FnMut(&mut Self, QueryMessage, String) -> (QueryMessage, String) + 'static,
    {
        unsafe extern "stdcall" fn shim<F>(
            webview: mbWebView,
            param: *mut ::std::os::raw::c_void,
            _es: mbJsExecState,
            query_id: i64,
            custom_msg: ::std::os::raw::c_int,
            request: *const utf8,
        ) where
            F: FnMut(&mut WebView, QueryMessage, String) -> (QueryMessage, String) + 'static,
        {
            let mut wv = WebView::from_ptr(webview);
            let cb: *mut F = param as _;
            let f = &mut *cb;
            let request = CStr::from_ptr(request).to_string_lossy().to_string();

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                f(&mut wv, custom_msg, request)
            }));

            if let Ok((query_message, response)) = r {
                let response = CString::safe_new(&response);
                call_api_or_panic().mbResponseQuery(
                    webview,
                    query_id,
                    query_message,
                    response.as_ptr(),
                )
            }
        }

        let cb: *mut F = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnJsQuery(*self.inner, Some(shim::<F>), cb as *mut _);
        }
    }
}
