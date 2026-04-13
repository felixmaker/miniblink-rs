use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::rc::Rc;

use crate::call_api_or_panic;
use crate::content::{set_webview_handler, WebViewContent, WEBVIEW_CONTENT};
use crate::params::*;
use crate::types::*;
use crate::webwindow::WebViewWindow;
use miniblink_sys::mbWebView;

/// Wraps to WebView.
#[repr(transparent)]
pub struct WebView {
    pub(crate) inner: mbWebView,
}

impl WebView {
    /// Create a new webview.
    ///
    /// # Remarks
    /// This is for advanced users. It is recommended to use `WebViewWindow` to create a webview.
    pub fn new() -> Self {
        let inner = unsafe { call_api_or_panic().mbCreateWebView() };
        unsafe { Self::from_raw(inner) }
    }

    /// Retake the inner pointer.
    ///
    /// # Remarks
    /// Only accept ptr from `mbCreateWebView` or `mbCreateWebWindow`, and make sure the pointer is valid,
    /// otherwise it will cause undefined behavior.
    pub(crate) unsafe fn from_raw(ptr: mbWebView) -> Self {
        assert!(ptr != 0, "Failed to create webview");
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            if content.contains_key(&ptr) {
                return;
            }
            content.insert(ptr, WebViewContent::default());
        });
        let webview = Self { inner: ptr };
        set_webview_handler(&webview);
        webview
    }

    /// Get the inner pointer.
    pub fn as_ptr(&self) -> mbWebView {
        self.inner
    }

    /// Stop loading the page.
    pub fn stop_loading(&self) {
        unsafe { call_api_or_panic().mbStopLoading(self.as_ptr()) }
    }

    /// Reload page.
    pub fn reload(&self) {
        unsafe { call_api_or_panic().mbReload(self.as_ptr()) }
    }

    /// Go back.
    pub fn go_back(&self) {
        unsafe { call_api_or_panic().mbGoBack(self.as_ptr()) }
    }

    /// Go forward.
    pub fn go_forward(&self) {
        unsafe { call_api_or_panic().mbGoForward(self.as_ptr()) }
    }

    /// Resize the page.
    ///
    /// # Remarks
    ///
    /// This api will resize the window at the same time if using the internal api to create window.
    pub fn resize(&self, w: i32, h: i32) {
        unsafe { call_api_or_panic().mbResize(self.as_ptr(), w, h) }
    }

    /// Get the window handle.
    pub fn get_window_handle(&self) -> WindowHandle {
        let hwnd = unsafe { call_api_or_panic().mbGetHostHWND(self.as_ptr()) };
        WindowHandle { inner: hwnd }
    }

    /// Send select command to editor.
    pub fn editor_select_all(&self) {
        unsafe { call_api_or_panic().mbEditorSelectAll(self.as_ptr()) }
    }

    /// Send unselect command to editor.
    pub fn editor_unselect(&self) {
        unsafe { call_api_or_panic().mbEditorUnSelect(self.as_ptr()) }
    }

    /// Send copy command to editor.
    pub fn editor_copy(&self) {
        unsafe { call_api_or_panic().mbEditorCopy(self.as_ptr()) }
    }

    /// Send cut command to editor.
    pub fn editor_cut(&self) {
        unsafe { call_api_or_panic().mbEditorCut(self.as_ptr()) }
    }

    /// Send delete command to editor.
    pub fn editor_delete(&self) {
        unsafe { call_api_or_panic().mbEditorDelete(self.as_ptr()) }
    }

    /// Send undo command to editor.
    pub fn editor_undo(&self) {
        unsafe { call_api_or_panic().mbEditorUndo(self.as_ptr()) }
    }

    /// Send redo command to editor.
    pub fn editor_redo(&self) {
        unsafe { call_api_or_panic().mbEditorRedo(self.as_ptr()) }
    }

    /// Send paste command to editor.
    pub fn editor_paste(&self) {
        unsafe { call_api_or_panic().mbEditorPaste(self.as_ptr()) }
    }

    /// Get the page cookies asynchronously.
    ///
    /// # Remarks
    /// Cookie information will be returned in the callback function.
    pub fn get_cookie_async<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &GetCookieParameters) + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_get_cookie = Some(callback);
        })
    }

    /// Set the page cookies.
    ///
    /// # Remarks
    ///
    /// This cookie should follow the curl format of `PERSONALIZE=123;expires=Monday, 13-Jun-2022 03:04:55 GMT; domain=.fidelity.com; path=/; secure`
    pub fn set_cookie(&self, url: &str, cookie: &str) {
        let url = CString::new(url).unwrap();
        let cookie = CString::new(cookie).unwrap();
        unsafe { call_api_or_panic().mbSetCookie(self.as_ptr(), url.as_ptr(), cookie.as_ptr()) }
    }

    /// Perform cookie command.
    ///
    /// # Remarks
    /// This api only affects the curl settings, does not change the javascript content.
    pub fn perform_cookie_command<F>(&self, command: CookieCommand) {
        unsafe {
            call_api_or_panic().mbPerformCookieCommand(self.as_ptr(), command as _);
        }
    }

    /// Clear all cookies.
    pub fn clear_cookie(&self) {
        unsafe { call_api_or_panic().mbClearCookie(self.as_ptr()) };
    }

    /// Set cookie jar path.
    pub fn set_cookie_jar_path(&self, path: &str) {
        let path = widestring::WideCString::from_str(path).unwrap();
        unsafe { call_api_or_panic().mbSetCookieJarPath(self.as_ptr(), path.as_ptr()) };
    }

    /// Set cookie jar full path.
    pub fn set_cookie_jar_full_path(&self, path: &str) {
        let path = widestring::WideCString::from_str(path).unwrap();
        unsafe { call_api_or_panic().mbSetCookieJarFullPath(self.as_ptr(), path.as_ptr()) };
    }

    /// Set local storage full path.
    ///
    /// # Remarks
    ///
    /// Only directory paths can be set, file paths cannot be set.
    pub fn set_local_storage_full_path(&self, path: &str) {
        let path = widestring::WideCString::from_str(path).unwrap();
        unsafe { call_api_or_panic().mbSetLocalStorageFullPath(self.as_ptr(), path.as_ptr()) };
    }

    /// Fire mouse event.
    pub fn fire_mouse_event(&self, message: WindowMessage, x: i32, y: i32, flags: MouseFlags) {
        unsafe {
            call_api_or_panic().mbFireMouseEvent(self.as_ptr(), message as _, x, y, flags.into())
        };
    }

    /// Fire mouse wheel event.
    pub fn fire_mouse_wheel_event(
        &self,
        message: WindowMessage,
        x: i32,
        y: i32,
        flags: MouseFlags,
    ) {
        unsafe {
            call_api_or_panic().mbFireMouseWheelEvent(
                self.as_ptr(),
                message as _,
                x,
                y,
                flags.into(),
            )
        };
    }

    /// Fire key up event.
    pub fn fire_key_up_event(&self, key: VirtualKeyCode, flags: KeyboardFlags, system_key: bool) {
        unsafe {
            call_api_or_panic().mbFireKeyUpEvent(
                self.as_ptr(),
                key as _,
                flags as _,
                system_key as _,
            );
        }
    }

    /// Fire key down event.
    pub fn fire_key_down_event(&self, key: VirtualKeyCode, flags: KeyboardFlags, system_key: bool) {
        unsafe {
            call_api_or_panic().mbFireKeyDownEvent(
                self.as_ptr(),
                key as _,
                flags as _,
                system_key as _,
            );
        }
    }

    /// Fire key press event. Char code todo!
    pub fn fire_key_press_event(&self, char_code: u32, flags: KeyboardFlags, system_key: bool) {
        unsafe {
            call_api_or_panic().mbFireKeyPressEvent(
                self.as_ptr(),
                char_code,
                flags as _,
                system_key as _,
            );
        }
    }

    /// Set focus.
    pub fn set_focus(&self) {
        unsafe {
            call_api_or_panic().mbSetFocus(self.as_ptr());
        }
    }

    /// Kill focus.
    pub fn kill_focus(&self) {
        unsafe {
            call_api_or_panic().mbKillFocus(self.as_ptr());
        }
    }

    /// Run js.
    pub fn eval(&self, frame_handle: WebFrameHandle, script: &str, is_in_closure: bool) -> String {
        let script = CString::new(script).unwrap();
        let is_in_closure = if is_in_closure { 1 } else { 0 };
        unsafe {
            let result = call_api_or_panic().mbRunJsSync(
                self.as_ptr(),
                frame_handle.as_ptr(),
                script.as_ptr(),
                is_in_closure,
            );

            let es =
                call_api_or_panic().mbGetGlobalExecByFrame(self.as_ptr(), frame_handle.as_ptr());
            let result = call_api_or_panic().mbJsToString(es, result);
            CStr::from_ptr(result).to_string_lossy().to_string()
        }
    }

    /// Eval a script on the frame.
    pub fn on_query<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &JsQueryParameters) -> JsQueryResult + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_query = Some(callback);
        });
    }

    /// Set zoom factor.
    pub fn set_zoom_factor(&self, factor: f32) {
        unsafe {
            call_api_or_panic().mbSetZoomFactor(self.as_ptr(), factor);
        }
    }

    /// Get zoom factor.
    pub fn get_zoom_factor(&self) -> f32 {
        unsafe { call_api_or_panic().mbGetZoomFactor(self.as_ptr()) }
    }

    /// Set title changed callback.
    pub fn on_title_changed<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &str) + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_title_changed = Some(callback);
        });
    }

    /// Set URL changed callback.
    pub fn on_url_changed<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &UrlChangedParameters) + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_url_changed = Some(callback);
        });
    }

    /// Set alert box callback.
    pub fn on_alert_box<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &str) -> bool + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_alert_box = Some(callback);
        });
    }

    /// Set confirm box callback.
    pub fn on_confirm_box<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &str) -> bool + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_confirm_box = Some(callback);
        });
    }

    /// Set prompt box callback.
    pub fn on_prompt_box<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &PromptParams) -> Option<String> + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_prompt_box = Some(callback);
        });
    }

    /// Set navigation callback.
    /// 
    /// Returns true to continue navigation, false to cancel navigation.
    pub fn on_navigation<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &NavigationParameters) -> bool + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_navigation = Some(callback);
        });
    }

    /// Set create view callback.
    /// 
    /// Invoked when a new webview is created after <a> link click.
    pub fn on_create_view<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &CreateViewParameters) -> Option<WebViewWindow> + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_create_view = Some(callback);
        });
    }

    /// Set a callback when the page DOM emits a ready event. It is possible to determine whether it is the main frame or not.
    pub fn on_document_ready<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &WebFrameHandle) + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_document_ready = Some(callback);
        });
    }

    /// Set a callback when the page emits download event. Some links are called when they trigger a download.
    pub fn on_download<F>(&self, callback: F)
    where
        F: FnMut(&mut WebView, &DownloadParameters) -> bool + 'static,
    {
        let callback = Rc::new(RefCell::new(callback));
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let content = content.get_mut(&self.as_ptr()).unwrap();
            content.on_download = Some(callback);
        });
    }

    /// Set debug config: show dev tools.
    pub fn set_debug_show_dev_tools(&self, show_dev_tools: &str) {
        let show_dev_tools = CString::new(show_dev_tools).unwrap();
        unsafe {
            call_api_or_panic().mbSetDebugConfig(
                self.as_ptr(),
                c"showDevTools".as_ptr(),
                show_dev_tools.as_ptr(),
            );
        }
    }

    /// Set debug config: wake min interval (the higher the value, the lower the frame rate).
    pub fn set_debug_wake_min_interval(&self, interval: u32) {
        let interval = CString::new(interval.to_string()).unwrap();
        unsafe {
            call_api_or_panic().mbSetDebugConfig(
                self.as_ptr(),
                c"wakeMinInterval".as_ptr(),
                interval.as_ptr(),
            );
        }
    }

    /// Set debug config: draw min interval (the higher the value, the lower the frame rate).
    pub fn set_debug_draw_min_interval(&self, interval: u32) {
        let interval = CString::new(interval.to_string()).unwrap();
        unsafe {
            call_api_or_panic().mbSetDebugConfig(
                self.as_ptr(),
                c"drawMinInterval".as_ptr(),
                interval.as_ptr(),
            );
        }
    }

    /// Set debug config: anti-aliasing rendering.
    pub fn set_debug_draw_max_interval(&self, interval: u32) {
        let interval = CString::new(interval.to_string()).unwrap();
        unsafe {
            call_api_or_panic().mbSetDebugConfig(
                self.as_ptr(),
                c"antiAlias".as_ptr(),
                interval.as_ptr(),
            );
        }
    }

    /// Set debug config: minimum font size.
    pub fn set_debug_minimal_font_size(&self, size: u32) {
        let size = CString::new(size.to_string()).unwrap();
        unsafe {
            call_api_or_panic().mbSetDebugConfig(
                self.as_ptr(),
                c"minimumFontSize".as_ptr(),
                size.as_ptr(),
            );
        }
    }

    /// Set debug config: minimum logical font size.
    pub fn set_debug_minimum_logical_font_size(&self, size: u32) {
        let size = CString::new(size.to_string()).unwrap();
        unsafe {
            call_api_or_panic().mbSetDebugConfig(
                self.as_ptr(),
                c"minimumLogicalFontSize".as_ptr(),
                size.as_ptr(),
            );
        }
    }

    /// Set debug config: default font size.
    pub fn set_debug_default_font_size(&self, size: u32) {
        let size = CString::new(size.to_string()).unwrap();
        unsafe {
            call_api_or_panic().mbSetDebugConfig(
                self.as_ptr(),
                c"defaultFontSize".as_ptr(),
                size.as_ptr(),
            );
        }
    }

    /// Set debug config: default fixed font size.
    pub fn set_debug_default_fixed_font_size(&self, size: u32) {
        let size = CString::new(size.to_string()).unwrap();
        unsafe {
            call_api_or_panic().mbSetDebugConfig(
                self.as_ptr(),
                c"defaultFixedFontSize".as_ptr(),
                size.as_ptr(),
            );
        }
    }

    /// Set window handle.
    ///
    /// # Remarks
    ///
    /// This function should only used in off screen render mode.
    pub fn set_handle(&self, handle: WindowHandle) {
        unsafe { call_api_or_panic().mbSetHandle(self.as_ptr(), handle.inner) };
    }

    /// Set handle offset.
    ///
    /// # Remarks
    ///
    /// This function should only used in off screen render mode.
    pub fn set_handle_offset(&self, x: i32, y: i32) {
        unsafe { call_api_or_panic().mbSetHandleOffset(self.as_ptr(), x, y) };
    }

    /// Set user agent.
    pub fn set_user_agent(&self, user_agent: &str) {
        let user_agent = CString::new(user_agent).unwrap();
        unsafe {
            call_api_or_panic().mbSetUserAgent(self.as_ptr(), user_agent.as_ptr());
        }
    }

    /// Load URL.
    pub fn load_url(&self, url: &str) {
        let url = CString::new(url).unwrap();
        unsafe {
            call_api_or_panic().mbLoadURL(self.as_ptr(), url.as_ptr());
        }
    }

    /// Load HTML with base URL.
    pub fn load_html_with_base_url(&self, html: &str, base_url: &str) {
        let html = CString::new(html).unwrap();
        let base_url = CString::new(base_url).unwrap();
        unsafe {
            call_api_or_panic().mbLoadHtmlWithBaseUrl(
                self.as_ptr(),
                html.as_ptr(),
                base_url.as_ptr(),
            )
        }
    }

    /// Enable context menu.
    pub fn enable_context_menu(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetContextMenuEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable cookie.
    pub fn enable_cookie(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetCookieEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable CSP check.
    pub fn enable_csp_check(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetCspCheckEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable disk cache.
    pub fn enable_disk_cache(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetDiskCacheEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable drag and drop.
    pub fn enable_drag_drop(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetDragDropEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable drag.
    pub fn enable_drag(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetDragEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable headless mode.
    pub fn enable_headless_mode(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetHeadlessEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable memory cache.
    pub fn enable_memory_cache(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetMemoryCacheEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable mouse.
    pub fn enable_mouse(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetMouseEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable navigation to new window.
    pub fn enable_navigation_to_new_window(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetNavigationToNewWindowEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable nodejs.
    pub fn enable_nodejs(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetNodeJsEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable npapi plugins.
    pub fn enable_npapi_plugins(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetNpapiPluginsEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable system touch.
    pub fn enable_system_touch(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetSystemTouchEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable touch.
    pub fn enable_touch(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetTouchEnabled(self.as_ptr(), enabled as _) }
    }

    /// Check if is the mainframe.
    pub fn is_mainframe(&self, frame_handle: WebFrameHandle) -> bool {
        unsafe { call_api_or_panic().mbIsMainFrame(self.as_ptr(), frame_handle.as_ptr()) != 0 }
    }

    /// Set global proxy.
    pub fn set_proxy(&self, proxy: &Proxy) {
        let proxy = proxy.to_mb_proxy();
        unsafe {
            call_api_or_panic().mbSetProxy(self.as_ptr(), &proxy);
        }
    }

    /// Destroy the webview.
    ///
    /// # Safety
    /// This function will destroy the webview, and the webview should not be used after.
    pub(crate) unsafe fn destroy(&self) {
        WEBVIEW_CONTENT.with_borrow_mut(|content| {
            let current = content.get_mut(&self.as_ptr()).unwrap();
            for child in current.child.iter() {
                drop(Self::from_raw(*child));
            }
            if let Some(parent) = current.parent {
                let parrent = content.get_mut(&parent).unwrap();
                parrent.child.remove(&self.as_ptr());
            }
            content.remove(&self.as_ptr());
        });
        call_api_or_panic().mbDestroyWebView(self.as_ptr());
    }
}

impl Drop for WebView {
    fn drop(&mut self) {
        unsafe { self.destroy() }
    }
}
