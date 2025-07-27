use std::{
    ffi::{CStr, CString},
    rc::{Rc, Weak},
};

use crate::call_api_or_panic;

unsafe extern "stdcall" fn shim_text(
    _: miniblink_sys::mbWebView,
    param: *mut ::std::os::raw::c_void,
    text: *const i8,
) {
    let data = param as *mut Box<dyn FnMut(&str)>;
    let f = &mut *data;
    let text = CStr::from_ptr(text).to_string_lossy().to_string();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&text)));
}

unsafe extern "stdcall" fn shim_text2(
    _: miniblink_sys::mbWebView,
    param: *mut ::std::os::raw::c_void,
    text: *const i8,
) -> i32 {
    let data = param as *mut Box<dyn FnMut(&str) -> bool>;
    let f = &mut *data;
    let text = CStr::from_ptr(text).to_string_lossy().to_string();
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&text)));
    if r.unwrap_or(true) {
        1
    } else {
        0
    }
}

/// The webview trait.
pub trait WebViewTrait {
    /// Returns the raw pointer of the webview.
    fn as_ptr(&self) -> miniblink_sys::mbWebView;

    /// Stop loading the page.
    fn stop_loading(&self) {
        unsafe { call_api_or_panic().mbStopLoading(self.as_ptr()) }
    }

    /// Reload page.
    fn reload(&self) {
        unsafe { call_api_or_panic().mbReload(self.as_ptr()) }
    }

    /// Go back.
    fn go_back(&self) {
        unsafe { call_api_or_panic().mbGoBack(self.as_ptr()) }
    }

    /// Go forward.
    fn go_forward(&self) {
        unsafe { call_api_or_panic().mbGoForward(self.as_ptr()) }
    }

    /// Resize the page.
    ///
    /// # Remarks
    ///
    /// This api will resize the window at the same time if using the internal api to create window.
    fn resize(&self, w: i32, h: i32) {
        unsafe { call_api_or_panic().mbResize(self.as_ptr(), w, h) }
    }

    /// Get the window handle.
    fn get_window_handle(&self) -> WindowHandle {
        let hwnd = unsafe { call_api_or_panic().mbGetHostHWND(self.as_ptr()) };
        WindowHandle { hwnd }
    }

    /// Send select command to editor.
    fn editor_select_all(&self) {
        unsafe { call_api_or_panic().mbEditorSelectAll(self.as_ptr()) }
    }

    /// Send unselect command to editor.
    fn editor_unselect(&self) {
        unsafe { call_api_or_panic().mbEditorUnSelect(self.as_ptr()) }
    }

    /// Send copy command to editor.
    fn editor_copy(&self) {
        unsafe { call_api_or_panic().mbEditorCopy(self.as_ptr()) }
    }

    /// Send cut command to editor.
    fn editor_cut(&self) {
        unsafe { call_api_or_panic().mbEditorCut(self.as_ptr()) }
    }

    /// Send delete command to editor.
    fn editor_delete(&self) {
        unsafe { call_api_or_panic().mbEditorDelete(self.as_ptr()) }
    }

    /// Send undo command to editor.
    fn editor_undo(&self) {
        unsafe { call_api_or_panic().mbEditorUndo(self.as_ptr()) }
    }

    /// Send redo command to editor.
    fn editor_redo(&self) {
        unsafe { call_api_or_panic().mbEditorRedo(self.as_ptr()) }
    }

    /// Send paste command to editor.
    fn editor_paste(&self) {
        unsafe { call_api_or_panic().mbEditorPaste(self.as_ptr()) }
    }

    /// Get the page cookies asynchronously.
    ///
    /// # Remarks
    /// Cookie information will be returned in the callback function.
    fn get_cookie_async<F>(&self, callback: F)
    where
        F: FnMut(&GetCookieParameters) + 'static,
    {
        unsafe extern "stdcall" fn shim(
            _: miniblink_sys::mbWebView,
            param: *mut ::std::os::raw::c_void,
            state: miniblink_sys::MbAsynRequestState,
            cookie: *const i8,
        ) {
            let data = param as *mut Box<dyn FnMut(&GetCookieParameters)>;
            let callback = &mut *data;
            let cookies = std::ffi::CStr::from_ptr(cookie);
            let param = GetCookieParameters {
                state: state.try_into().unwrap(),
                cookie: cookies.to_string_lossy().to_string(),
            };
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(&param)));
        }
        let callback: Box<dyn FnMut(&GetCookieParameters)> = Box::new(callback);
        let data: *mut Box<dyn FnMut(&GetCookieParameters)> = Box::into_raw(Box::new(callback));
        unsafe { call_api_or_panic().mbGetCookie(self.as_ptr(), Some(shim), data as _) }
    }

    /// Set the page cookies.
    ///
    /// # Remarks
    ///
    /// This cookie should follow the curl format of `PERSONALIZE=123;expires=Monday, 13-Jun-2022 03:04:55 GMT; domain=.fidelity.com; path=/; secure`
    fn set_cookie(&self, url: &str, cookie: &str) {
        let url = CString::new(url).unwrap();
        let cookie = CString::new(cookie).unwrap();
        unsafe { call_api_or_panic().mbSetCookie(self.as_ptr(), url.as_ptr(), cookie.as_ptr()) }
    }

    /// Perform cookie command.
    ///
    /// # Remarks
    /// This api only affects the curl settings, does not change the javascript content.
    fn perform_cookie_command<F>(&self, command: CookieCommand) {
        unsafe {
            call_api_or_panic().mbPerformCookieCommand(self.as_ptr(), command as _);
        }
    }

    /// Clear all cookies.
    fn clear_cookie(&self) {
        unsafe { call_api_or_panic().mbClearCookie(self.as_ptr()) };
    }

    /// Set cookie jar path.
    fn set_cookie_jar_path(&self, path: &str) {
        let path = widestring::WideCString::from_str(path).unwrap();
        unsafe { call_api_or_panic().mbSetCookieJarPath(self.as_ptr(), path.as_ptr()) };
    }

    /// Set cookie jar full path.
    fn set_cookie_jar_full_path(&self, path: &str) {
        let path = widestring::WideCString::from_str(path).unwrap();
        unsafe { call_api_or_panic().mbSetCookieJarFullPath(self.as_ptr(), path.as_ptr()) };
    }

    /// Set local storage full path.
    ///
    /// # Remarks
    ///
    /// Only directory paths can be set, file paths cannot be set.
    fn set_local_storage_full_path(&self, path: &str) {
        let path = widestring::WideCString::from_str(path).unwrap();
        unsafe { call_api_or_panic().mbSetLocalStorageFullPath(self.as_ptr(), path.as_ptr()) };
    }

    /// Fire mouse event.
    fn fire_mouse_event(&self, message: WindowMessage, x: i32, y: i32, flags: MouseFlags) {
        unsafe {
            call_api_or_panic().mbFireMouseEvent(self.as_ptr(), message as _, x, y, flags.into())
        };
    }

    /// Fire mouse wheel event.
    fn fire_mouse_wheel_event(&self, message: WindowMessage, x: i32, y: i32, flags: MouseFlags) {
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
    fn fire_key_up_event(&self, key: VirtualKeyCode, flags: KeyboardFlags, system_key: bool) {
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
    fn fire_key_down_event(&self, key: VirtualKeyCode, flags: KeyboardFlags, system_key: bool) {
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
    fn fire_key_press_event(&self, char_code: u32, flags: KeyboardFlags, system_key: bool) {
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
    fn set_focus(&self) {
        unsafe {
            call_api_or_panic().mbSetFocus(self.as_ptr());
        }
    }

    /// Kill focus.
    fn kill_focus(&self) {
        unsafe {
            call_api_or_panic().mbKillFocus(self.as_ptr());
        }
    }

    /// Run js.
    fn eval<T>(&self, frame_handle: T, script: &str, is_in_closure: bool) -> String
    where
        T: WebFrameHandleTrait,
    {
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
    fn on_query<F>(&self, callback: F)
    where
        F: FnMut(&JsQueryParameters) -> JsQueryResult + 'static,
    {
        unsafe extern "stdcall" fn shim(
            webview: miniblink_sys::mbWebView,
            param: *mut ::std::os::raw::c_void,
            _es: miniblink_sys::mbJsExecState,
            query_id: i64,
            custom_msg: ::std::os::raw::c_int,
            request: *const i8,
        ) {
            let cb: *mut Box<dyn FnMut(&JsQueryParameters) -> JsQueryResult> = param as _;
            let f = &mut *cb;
            let request = CStr::from_ptr(request).to_string_lossy().to_string();
            let query_message = JsQueryParameters {
                custom_message: custom_msg,
                request,
            };

            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&query_message)));

            if let Ok(JsQueryResult {
                custom_message,
                response,
            }) = r
            {
                let response = CString::new(response).unwrap();
                call_api_or_panic().mbResponseQuery(
                    webview,
                    query_id,
                    custom_message,
                    response.as_ptr(),
                )
            }
        }

        let callback: Box<dyn FnMut(&JsQueryParameters) -> JsQueryResult> = Box::new(callback);
        let cb: *mut Box<dyn FnMut(&JsQueryParameters) -> JsQueryResult> =
            Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnJsQuery(self.as_ptr(), Some(shim), cb as *mut _);
        }
    }

    /// Set zoom factor.
    fn set_zoom_factor(&self, factor: f32) {
        unsafe {
            call_api_or_panic().mbSetZoomFactor(self.as_ptr(), factor);
        }
    }

    /// Get zoom factor.
    fn get_zoom_factor(&self) -> f32 {
        unsafe { call_api_or_panic().mbGetZoomFactor(self.as_ptr()) }
    }

    /// Set title changed callback.
    fn on_title_changed<F>(&self, callback: F)
    where
        F: FnMut(&str) + 'static,
    {
        let callback: Box<dyn FnMut(&str)> = Box::new(callback);
        let data: *mut Box<dyn FnMut(&str)> = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnTitleChanged(self.as_ptr(), Some(shim_text), data as _);
        }
    }

    /// Set URL changed callback.
    fn on_url_changed<F>(&self, callback: F)
    where
        F: FnMut(&UrlChangeParameters) + 'static,
    {
        unsafe extern "stdcall" fn shim(
            _: miniblink_sys::mbWebView,
            param: *mut ::std::os::raw::c_void,
            url: *const i8,
            can_go_back: i32,
            can_go_forward: i32,
        ) {
            let data = param as *mut Box<dyn FnMut(&UrlChangeParameters)>;
            let f = &mut *data;
            let url = CStr::from_ptr(url).to_string_lossy().to_string();
            let param = UrlChangeParameters {
                url,
                can_go_back: can_go_back != 0,
                can_go_forward: can_go_forward != 0,
            };
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&param)));
        }
        let callback: Box<dyn FnMut(&UrlChangeParameters)> = Box::new(callback);
        let data: *mut Box<dyn FnMut(&UrlChangeParameters)> = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnURLChanged(self.as_ptr(), Some(shim), data as _);
        }
    }

    /// Set alert box callback.
    fn on_alert_box<F>(&self, callback: F)
    where
        F: FnMut(&str) + 'static,
    {
        let callback: Box<dyn FnMut(&str)> = Box::new(callback);
        let data: *mut Box<dyn FnMut(&str)> = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnAlertBox(self.as_ptr(), Some(shim_text), data as _);
        }
    }

    /// Set confirm box callback.
    fn on_confirm_box<F>(&self, callback: F)
    where
        F: FnMut(&str) + 'static,
    {
        let callback: Box<dyn FnMut(&str)> = Box::new(callback);
        let data: *mut Box<dyn FnMut(&str)> = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnConfirmBox(self.as_ptr(), Some(shim_text2), data as _);
        }
    }

    /// Set navigation callback.
    fn on_navigation<F>(&self, callback: F)
    where
        F: FnMut(&NavigationParameters) -> bool + 'static,
    {
        unsafe extern "stdcall" fn shim(
            _: isize,
            param: *mut ::std::os::raw::c_void,
            navigation_type: i32,
            url: *const i8,
        ) -> i32 {
            let data = param as *mut Box<dyn FnMut(&NavigationParameters) -> bool>;
            let f = &mut *data;
            let url = CStr::from_ptr(url).to_string_lossy().to_string();
            let param = NavigationParameters {
                navigation_type: navigation_type.try_into().unwrap(),
                url,
            };
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&param)));
            if r.unwrap_or(true) {
                1
            } else {
                0
            }
        }
        let callback: Box<dyn FnMut(&NavigationParameters) -> bool> = Box::new(callback);
        let data: *mut Box<dyn FnMut(&NavigationParameters) -> bool> =
            Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnNavigation(self.as_ptr(), Some(shim), data as _);
        }
    }

    /// Set a callback when the page DOM emits a ready event. It is possible to determine whether it is the main frame or not.
    fn on_document_ready<F>(&self, callback: F)
    where
        F: FnMut(&WebFrameHandle) + 'static,
    {
        unsafe extern "stdcall" fn shim(
            _: isize,
            param: *mut ::std::os::raw::c_void,
            frame_id: *mut ::std::os::raw::c_void,
        ) {
            let data = param as *mut Box<dyn FnMut(&WebFrameHandle)>;
            let f = &mut *data;
            let frame_id = WebFrameHandle { inner: frame_id };
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&frame_id)));
        }

        let callback: Box<dyn FnMut(&WebFrameHandle)> = Box::new(callback);
        let data: *mut Box<dyn FnMut(&WebFrameHandle)> = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnDocumentReady(self.as_ptr(), Some(shim), data as _);
        }
    }

    /// Set a callback when the page emits download event. Some links are called when they trigger a download.
    fn on_download<F>(&self, callback: F)
    where
        F: FnMut(&DownloadParameters) -> bool + 'static,
    {
        unsafe extern "stdcall" fn shim(
            _: isize,
            param: *mut ::std::os::raw::c_void,
            frame_id: miniblink_sys::mbWebFrameHandle,
            url: *const ::std::os::raw::c_char,
            download_job: *mut ::std::os::raw::c_void,
        ) -> i32 {
            let data = param as *mut Box<dyn FnMut(&DownloadParameters) -> bool>;
            let f = &mut *data;
            let frame_id = WebFrameHandle { inner: frame_id };
            let url = CStr::from_ptr(url).to_string_lossy().to_string();
            let download_job = DownloadJob {
                inner: download_job,
            };
            let download_parameters = DownloadParameters {
                frame_id,
                url,
                download_job,
            };
            let r =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&download_parameters)));
            if r.unwrap_or(true) {
                1
            } else {
                0
            }
        }

        let callback: Box<dyn FnMut(&DownloadParameters) -> bool> = Box::new(callback);
        let data: *mut Box<dyn FnMut(&DownloadParameters) -> bool> =
            Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnDownload(self.as_ptr(), Some(shim), data as _);
        }
    }

    /// Set debug config: show dev tools.
    fn set_debug_show_dev_tools(&self, show_dev_tools: &str) {
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
    fn set_debug_wake_min_interval(&self, interval: u32) {
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
    fn set_debug_draw_min_interval(&self, interval: u32) {
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
    fn set_debug_draw_max_interval(&self, interval: u32) {
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
    fn set_debug_minimal_font_size(&self, size: u32) {
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
    fn set_debug_minimum_logical_font_size(&self, size: u32) {
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
    fn set_debug_default_font_size(&self, size: u32) {
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
    fn set_debug_default_fixed_font_size(&self, size: u32) {
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
    fn set_handle<T>(&self, handle: T)
    where
        T: WindowHandleTrait,
    {
        unsafe { call_api_or_panic().mbSetHandle(self.as_ptr(), handle.as_ptr()) };
    }

    /// Set handle offset.
    ///
    /// # Remarks
    ///
    /// This function should only used in off screen render mode.
    fn set_handle_offset(&self, x: i32, y: i32) {
        unsafe { call_api_or_panic().mbSetHandleOffset(self.as_ptr(), x, y) };
    }

    /// Set user agent.
    fn set_user_agent(&self, user_agent: &str) {
        let user_agent = CString::new(user_agent).unwrap();
        unsafe {
            call_api_or_panic().mbSetUserAgent(self.as_ptr(), user_agent.as_ptr());
        }
    }

    /// Load URL.
    fn load_url(&self, url: &str) {
        let url = CString::new(url).unwrap();
        unsafe {
            call_api_or_panic().mbLoadURL(self.as_ptr(), url.as_ptr());
        }
    }

    /// Load HTML with base URL.
    fn load_html_with_base_url(&self, html: &str, base_url: &str) {
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
    fn enable_context_menu(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetContextMenuEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable cookie.
    fn enable_cookie(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetCookieEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable CSP check.
    fn enable_csp_check(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetCspCheckEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable disk cache.
    fn enable_disk_cache(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetDiskCacheEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable drag and drop.
    fn enable_drag_drop(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetDragDropEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable drag.
    fn enable_drag(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetDragEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable headless mode.
    fn enable_headless_mode(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetHeadlessEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable memory cache.
    fn enable_memory_cache(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetMemoryCacheEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable mouse.
    fn enable_mouse(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetMouseEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable navigation to new window.
    fn enable_navigation_to_new_window(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetNavigationToNewWindowEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable nodejs.
    fn enable_nodejs(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetNodeJsEnable(self.as_ptr(), enabled as _) }
    }

    /// Enable npapi plugins.
    fn enable_npapi_plugins(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetNpapiPluginsEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable system touch.
    fn enable_system_touch(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetSystemTouchEnabled(self.as_ptr(), enabled as _) }
    }

    /// Enable touch.
    fn enable_touch(&self, enabled: bool) {
        unsafe { call_api_or_panic().mbSetTouchEnabled(self.as_ptr(), enabled as _) }
    }

    /// Check if is the mainframe.
    fn is_mainframe<T>(&self, frame_handle: T) -> bool
    where
        T: WebFrameHandleTrait,
    {
        unsafe { call_api_or_panic().mbIsMainFrame(self.as_ptr(), frame_handle.as_ptr()) != 0 }
    }
}

/// The webview window trait.
pub trait WebViewWindowTrait {
    /// Returns the raw pointer to the web view window.
    fn as_ptr(&self) -> miniblink_sys::mbWebView;

    /// Set close callback.
    fn on_close<F>(&self, callback: F)
    where
        F: FnMut() -> bool + 'static,
    {
        unsafe extern "stdcall" fn shim(
            _: miniblink_sys::mbWebView,
            param: *mut ::std::os::raw::c_void,
            _unuse: *mut ::std::os::raw::c_void,
        ) -> miniblink_sys::BOOL {
            let data = param as *mut Box<dyn FnMut() -> bool>;
            let f = &mut *data;
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f()));

            if r.unwrap_or(true) {
                1
            } else {
                0
            }
        }

        let callback: Box<dyn FnMut() -> bool> = Box::new(callback);
        let data: *mut Box<dyn FnMut() -> bool> = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnClose(self.as_ptr(), Some(shim), data as _);
        }
    }

    /// Set destroy callback.
    fn on_destroy<F>(&self, callback: F)
    where
        F: FnMut() -> bool + 'static,
    {
        unsafe extern "stdcall" fn shim(
            _: miniblink_sys::mbWebView,
            param: *mut ::std::os::raw::c_void,
            _unuse: *mut ::std::os::raw::c_void,
        ) -> miniblink_sys::BOOL {
            let data = param as *mut Box<dyn FnMut() -> bool>;
            let f = &mut *data;
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f()));

            if r.unwrap_or(true) {
                1
            } else {
                0
            }
        }

        let callback: Box<dyn FnMut() -> bool> = Box::new(callback);
        let data: *mut Box<dyn FnMut() -> bool> = Box::into_raw(Box::new(callback));
        unsafe {
            call_api_or_panic().mbOnDestroy(self.as_ptr(), Some(shim), data as _);
        }
    }

    /// Show the window.
    fn show(&self) {
        unsafe {
            call_api_or_panic().mbShowWindow(self.as_ptr(), 1);
        }
    }

    /// Hide the window.
    fn hide(&self) {
        unsafe {
            call_api_or_panic().mbShowWindow(self.as_ptr(), 0);
        }
    }

    /// Resize the window.
    fn resize(&self, w: i32, h: i32) {
        unsafe { call_api_or_panic().mbResize(self.as_ptr(), w, h) }
    }

    /// Move the window.
    fn move_window(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            call_api_or_panic().mbMoveWindow(self.as_ptr(), x, y, w, h);
        }
    }

    /// Move the window to center.
    fn move_to_center(&self) {
        unsafe {
            call_api_or_panic().mbMoveToCenter(self.as_ptr());
        }
    }

    /// Set the window title.
    fn set_window_title(&self, title: &str) {
        let title = CString::new(title).unwrap();
        unsafe { call_api_or_panic().mbSetWindowTitle(self.as_ptr(), title.as_ptr()) }
    }
}

/// The web frame handle.
#[repr(transparent)]
pub struct WebFrameHandle {
    pub(crate) inner: *mut std::ffi::c_void,
}

/// The web frame handle trait.
pub trait WebFrameHandleTrait {
    /// Returns the raw pointer to the web frame handle.
    fn as_ptr(&self) -> *mut std::ffi::c_void;
}

/// The keyboard flags.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyboardFlags {
    /// repeat
    Repeat = 0,
    /// extended
    Extended = 1,
}

/// The virtual key code.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VirtualKeyCode {
    /// Left mouse button
    LeftButton = 0x01,
    /// Right mouse button
    RightButton = 0x02,
    /// Control-break processing
    Cancel = 0x03,
    /// Middle mouse button
    MiddleButton = 0x04,
    /// X1 mouse button
    XButton1 = 0x05,
    /// X2 mouse button
    XButton2 = 0x06,
    /// Backspace key
    Backspace = 0x08,
    /// Tab key
    Tab = 0x09,
    /// Clear key
    Clear = 0x0C,
    /// Enter key
    Enter = 0x0D,
    /// Shift key
    Shift = 0x10,
    /// Ctrl key
    Control = 0x11,
    /// Alt key
    Menu = 0x12,
    /// Pause key
    Pause = 0x13,
    /// Caps lock key
    CapsLock = 0x14,
    /// IME Kana / Hangul  mode
    Kana = 0x15,
    /// IME On
    ImeOn = 0x16,
    /// IME Junja mode
    Junja = 0x17,
    /// IME final mode
    Final = 0x18,
    /// IME Hanja / Kanji mode
    Hanja = 0x19,
    /// IME Off
    ImeOff = 0x1A,
    /// Esc key
    Escape = 0x1B,
    /// IME convert
    Convert = 0x1C,
    /// IME nonconvert
    Nonconvert = 0x1D,
    /// IME accept
    Accept = 0x1E,
    /// IME mode change request
    ModeChange = 0x1F,
    /// Spacebar key
    Space = 0x20,
    /// Page up key
    PageUp = 0x21,
    /// Page down key
    PageDown = 0x22,
    /// End key
    End = 0x23,
    /// Home key
    Home = 0x24,
    /// Left arrow key
    Left = 0x25,
    /// Up arrow key
    Up = 0x26,
    /// Right arrow key
    Right = 0x27,
    /// Down arrow key
    Down = 0x28,
    /// Select key
    Select = 0x29,
    /// Print key
    Print = 0x2A,
    /// Execute key
    Execute = 0x2B,
    /// Print screen key
    Snapshot = 0x2C,
    /// Insert key
    Insert = 0x2D,
    /// Delete key
    Delete = 0x2E,
    /// Help key
    Help = 0x2F,
    /// 0 key
    Key0 = 0x30,
    /// 1 key
    Key1 = 0x31,
    /// 2 key
    Key2 = 0x32,
    /// 3 key
    Key3 = 0x33,
    /// 4 key
    Key4 = 0x34,
    /// 5 key
    Key5 = 0x35,
    /// 6 key
    Key6 = 0x36,
    /// 7 key
    Key7 = 0x37,
    /// 8 key
    Key8 = 0x38,
    /// 9 key
    Key9 = 0x39,
    /// A key
    KeyA = 0x41,
    /// B key
    KeyB = 0x42,
    /// C key
    KeyC = 0x43,
    /// D key
    KeyD = 0x44,
    /// E key
    KeyE = 0x45,
    /// F key
    KeyF = 0x46,
    /// G key
    KeyG = 0x47,
    /// H key
    KeyH = 0x48,
    /// I key
    KeyI = 0x49,
    /// J key
    KeyJ = 0x4A,
    /// K key
    KeyK = 0x4B,
    /// L key
    KeyL = 0x4C,
    /// M key
    KeyM = 0x4D,
    /// N key
    KeyN = 0x4E,
    /// O key
    KeyO = 0x4F,
    /// P key
    KeyP = 0x50,
    /// Q key
    KeyQ = 0x51,
    /// R key
    KeyR = 0x52,
    /// S key
    KeyS = 0x53,
    /// T key
    KeyT = 0x54,
    /// U key
    KeyU = 0x55,
    /// V key
    KeyV = 0x56,
    /// W key
    KeyW = 0x57,
    /// X key
    KeyX = 0x58,
    /// Y key
    KeyY = 0x59,
    /// Z key
    KeyZ = 0x5A,
    /// Left Windows logo key
    LeftWin = 0x5B,
    /// Right Windows logo key
    RightWin = 0x5C,
    /// Application key
    Apps = 0x5D,
    /// Computer Sleep key
    Sleep = 0x5F,
    /// Numeric keypad 0 key
    Numpad0 = 0x60,
    /// Numeric keypad 1 key
    Numpad1 = 0x61,
    /// Numeric keypad 2 key
    Numpad2 = 0x62,
    /// Numeric keypad 3 key
    Numpad3 = 0x63,
    /// Numeric keypad 4 key
    Numpad4 = 0x64,
    /// Numeric keypad 5 key
    Numpad5 = 0x65,
    /// Numeric keypad 6 key
    Numpad6 = 0x66,
    /// Numeric keypad 7 key
    Numpad7 = 0x67,
    /// Numeric keypad 8 key
    Numpad8 = 0x68,
    /// Numeric keypad 9 key
    Numpad9 = 0x69,
    /// Multiply key
    Multiply = 0x6A,
    /// Add key
    Add = 0x6B,
    /// Separator key
    Separator = 0x6C,
    /// Subtract key
    Subtract = 0x6D,
    /// Decimal key
    Decimal = 0x6E,
    /// Divide key
    Divide = 0x6F,
    /// F1 key
    F1 = 0x70,
    /// F2 key
    F2 = 0x71,
    /// F3 key
    F3 = 0x72,
    /// F4 key
    F4 = 0x73,
    /// F5 key
    F5 = 0x74,
    /// F6 key
    F6 = 0x75,
    /// F7 key
    F7 = 0x76,
    /// F8 key
    F8 = 0x77,
    /// F9 key
    F9 = 0x78,
    /// F10 key
    F10 = 0x79,
    /// F11 key
    F11 = 0x7A,
    /// F12 key
    F12 = 0x7B,
    /// F13 key
    F13 = 0x7C,
    /// F14 key
    F14 = 0x7D,
    /// F15 key
    F15 = 0x7E,
    /// F16 key
    F16 = 0x7F,
    /// F17 key
    F17 = 0x80,
    /// F18 key
    F18 = 0x81,
    /// F19 key
    F19 = 0x82,
    /// F20 key
    F20 = 0x83,
    /// F21 key
    F21 = 0x84,
    /// F22 key
    F22 = 0x85,
    /// F23 key
    F23 = 0x86,
    /// F24 key
    F24 = 0x87,
    /// Num lock key
    NumLock = 0x90,
    /// Scroll lock key
    ScrollLock = 0x91,
    /// Left Shift key
    LeftShift = 0xA0,
    /// Right Shift key
    RightShift = 0xA1,
    /// Left Ctrl key
    LeftCtrl = 0xA2,
    /// Right Ctrl key
    RightCtrl = 0xA3,
    /// Left Alt key
    LeftAlt = 0xA4,
    /// Right Alt key
    RightAlt = 0xA5,
    /// Browser Back key
    BrowserBack = 0xA6,
    /// Browser Forward key
    BrowserForward = 0xA7,
    /// Browser Refresh key
    BrowserRefresh = 0xA8,
    /// Browser Stop key
    BrowserStop = 0xA9,
    /// Browser Search key
    BrowserSearch = 0xAA,
    /// Browser Favorites key
    BrowserFavorites = 0xAB,
    /// Browser Start and Home key
    BrowserHome = 0xAC,
    /// Volume Mute key
    VolumeMute = 0xAD,
    /// Volume Down key
    VolumeDown = 0xAE,
    /// Volume Up key
    VolumeUp = 0xAF,
    /// Media Next Track key
    MediaNextTrack = 0xB0,
    /// Media Previous Track key
    MediaPrevTrack = 0xB1,
    /// Media Stop key
    MediaStop = 0xB2,
    /// Media Play/Pause key
    MediaPlayPause = 0xB3,
    /// Start Mail key
    LaunchMail = 0xB4,
    /// Select Media key
    LaunchMediaSelect = 0xB5,
    /// Start Application 1 key
    LaunchApp1 = 0xB6,
    /// Start Application 2 key
    LaunchApp2 = 0xB7,
    /// It can vary by keyboard. For the US ANSI keyboard , the Semiсolon and Colon key
    OEM1 = 0xBA,
    /// For any country/region, the Equals and Plus key
    OEMPlus = 0xBB,
    /// For any country/region, the Comma and Less Than key
    OEMComma = 0xBC,
    /// For any country/region, the Dash and Underscore key
    OEMMinus = 0xBD,
    /// For any country/region, the Period and Greater Than key
    OEMPeriod = 0xBE,
    /// It can vary by keyboard. For the US ANSI keyboard, the Forward Slash and Question Mark key
    OEM2 = 0xBF,
    /// It can vary by keyboard. For the US ANSI keyboard, the Grave Accent and Tilde key
    OEM3 = 0xC0,
    /// It can vary by keyboard. For the US ANSI keyboard, the Left Brace key
    OEM4 = 0xDB,
    /// It can vary by keyboard. For the US ANSI keyboard, the Backslash and Pipe key
    OEM5 = 0xDC,
    /// It can vary by keyboard. For the US ANSI keyboard, the Right Brace key
    OEM6 = 0xDD,
    /// It can vary by keyboard. For the US ANSI keyboard, the Apostrophe and Double Quotation Mark key
    OEM7 = 0xDE,
    /// It can vary by keyboard. For the Canadian CSA keyboard, the Right Ctrl key
    OEM8 = 0xDF,
    /// It can vary by keyboard. For the European ISO keyboard, the Backslash and Pipe key
    OEM102 = 0xE2,
    /// IME PROCESS key
    ProcessKey = 0xE5,
    /// Used to pass Unicode characters as if they were keystrokes. The VK_PACKET key is the low word of a 32-bit Virtual Key value used for non-keyboard input methods. For more information, see Remark in KEYBDINPUT, SendInput, WM_KEYDOWN, and WM_KEYUP
    Packet = 0xE7,
    /// Attn key
    Attn = 0xF6,
    /// CrSel key
    CrSel = 0xF7,
    /// ExSel key
    ExSel = 0xF8,
    /// Erase EOF key
    EraseEOF = 0xF9,
    /// Play key
    Play = 0xFA,
    /// Zoom key
    Zoom = 0xFB,
    /// Reserved
    NonName = 0xFC,
    /// PA1 key
    PA1 = 0xFD,
    /// Clear key
    OEMClear = 0xFE,
}

/// The mouse event.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MouseFlags {
    /// The control key is pressed.
    pub control: bool,
    /// The shift key is pressed.
    pub shift: bool,
    /// The left button is pressed.
    pub left_button: bool,
    /// The middle button is pressed.
    pub middle_button: bool,
    /// The right button is pressed.
    pub right_button: bool,
}

impl From<MouseFlags> for u32 {
    fn from(value: MouseFlags) -> Self {
        let mut flags = 0;
        if value.control {
            flags |= 0x08;
        }
        if value.shift {
            flags |= 0x04;
        }
        if value.left_button {
            flags |= 0x01;
        }
        if value.middle_button {
            flags |= 0x10;
        }
        if value.right_button {
            flags |= 0x02;
        }
        flags
    }
}

/// The windows message.
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WindowMessage {
    Null = 0x0000,
    GetMinMaxInfo = 0x0024,
    Move = 0x0003,
    Timer = 0x0113,
    Paint = 0x000F,
    Close = 0x0010,
    LeftButtonUp = 0x0202,
    MouseMove = 0x0200,
    MiddleButtonUp = 0x0208,
    RightButtonUp = 0x0205,
    SetCursor = 0x0020,
    LeftButtonDown = 0x0201,
    ImeChar = 0x0286,
    SystemCommand = 0x0112,
    MouseLeave = 0x02A3,
    NonClientMouseMove = 0x00A0,
    NonClientMouseHover = 0x02A0,
    NonClientHitTest = 0x0084,
    MiddleButtonDown = 0x0207,
    RightButtonDown = 0x0204,
    LeftButtonDoubleClick = 0x0203,
    Command = 0x0111,
    ExitMenuLoop = 0x0212,
    RenderFormat = 0x0305,
    RenderAllFormats = 0x0306,
    DrawClipboard = 0x0308,
    Destroy = 0x0002,
    ChangeClipboardChain = 0x030D,
    Size = 0x0005,
    CancelMode = 0x001F,
    MouseWheel = 0x020A,
    KeyUp = 0x0101,
    KeyDown = 0x0100,
    Char = 0x0102,
    SetFocus = 0x0007,
    KillFocus = 0x0008,
    Create = 0x0001,
    NonClientPaint = 0x0085,
    EraseBackground = 0x0014,
    DropFiles = 0x0233,
    NonClientDestroy = 0x0082,
    SysKeyDown = 0x0104,
    SysKeyUp = 0x0105,
    MiddleButtonDoubleClick = 0x0209,
    RightButtonDoubleClick = 0x0206,
    ImeComposition = 0x010F,
    Quit = 0x0012,
    User = 0x0400,
    SetFont = 0x0030,
    Touch = 0x0240,
    CaptureChanged = 0x0215,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// The cookie command.
pub enum CookieCommand {
    /// Clear all cookies. Same as curl command: CURLOPT_COOKIELIST, "ALL".
    ClearAllCookies = 0,
    /// Clear session cookies. Same as curl command: CURLOPT_COOKIELIST, "SESS"
    ClearSessionCookies = 1,
    /// Flush cookies to file. Same as curl command: CURLOPT_COOKIELIST, "FLUSH".
    FlushCookiesToFile = 2,
    /// Reload cookies from file. Same as curl command: CURLOPT_COOKIELIST, "RELOAD".
    ReloadCookiesFromFile = 3,
}

macro_rules! impl_i32_to_enum {
    ($type: ty, $range: expr) => {
        impl TryFrom<i32> for $type {
            type Error = crate::error::MBError;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if !($range).contains(&value) {
                    return Err(crate::error::MBError::UndefinedEnumTransmute);
                } else {
                    Ok(unsafe { std::mem::transmute(value) })
                }
            }
        }
    };
}

/// Parameters in get cookie callback.
pub struct GetCookieParameters {
    /// The state of the request.
    pub state: AsynRequestState,
    /// The cookie.
    pub cookie: String,
}

/// Parameters in url change callback.
pub struct UrlChangeParameters {
    /// The url.
    pub url: String,
    /// Whether can go back.
    pub can_go_back: bool,
    /// Whether can go forward.
    pub can_go_forward: bool,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// The asynchronous request state.
pub enum AsynRequestState {
    /// The request is ok.
    Ok = 0,
    /// The request is fail.
    Fail = 1,
}

impl_i32_to_enum!(AsynRequestState, 0..=1);

/// The window handle.
#[repr(transparent)]
pub struct WindowHandle {
    hwnd: miniblink_sys::HWND,
}

/// The window handle trait.
pub trait WindowHandleTrait {
    /// Returns the raw pointer to the window handle.
    fn as_ptr(&self) -> miniblink_sys::HWND;
}

/// The navigation type.
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NavigationType {
    /// Click on `a` tag.
    LinkClick = 0,
    /// Click on `form` tag.
    FormSubmit = 1,
    /// Click on back button.
    BackForward = 2,
    /// Click on reload button.
    Reload = 3,
}

impl_i32_to_enum!(NavigationType, 0..=3);

/// Parameters in navigation callback.
pub struct NavigationParameters {
    /// The navigation type.
    pub navigation_type: NavigationType,
    /// The url.
    pub url: String,
}

/// Parameters in create view callback.
pub struct CreateViewParameters {
    /// The navigation type.
    pub navigation_type: NavigationType,
    /// The url.
    pub url: String,
    /// The window features.
    pub window_features: WindowFeatures,
}

/// The window features.
#[derive(Debug, Copy, Clone)]
pub struct WindowFeatures {
    /// The x position.
    pub x: i32,
    /// The y position.
    pub y: i32,
    /// The width.
    pub width: i32,
    /// The height.
    pub height: i32,
    /// Whether the menu bar is visible.
    pub menu_bar_visible: bool,
    /// Whether the status bar is visible.
    pub status_bar_visible: bool,
    /// Whether the tool bar is visible.
    pub tool_bar_visible: bool,
    /// Whether the location bar is visible.
    pub location_bar_visible: bool,
    /// Whether the scroll bars are visible.
    pub scroll_bars_visible: bool,
    /// Whether the window is resizable.
    pub resizable: bool,
    /// Whether the window is fullscreen.
    pub fullscreen: bool,
}

/// The download parameters.
pub struct DownloadParameters {
    /// The frame handler.
    pub frame_id: WebFrameHandle,
    /// The url.
    pub url: String,
    /// The download job.
    pub download_job: DownloadJob,
}

#[allow(unused)]
/// The download job. todo!
pub struct DownloadJob {
    pub(crate) inner: *mut std::ffi::c_void,
}

/// The js query parameters.
pub struct JsQueryParameters {
    /// The custom message.
    pub custom_message: i32,
    /// The request.
    pub request: String,
}

/// The js query result.
pub struct JsQueryResult {
    /// The custom message.
    pub custom_message: i32,
    /// The response.
    pub response: String,
}

/// Wrappers to webview raw pointer.
#[doc(hidden)]
#[repr(transparent)]
pub struct WebViewWindowInner {
    pub(crate) inner: miniblink_sys::mbWebView,
}

/// Wrappers to webview object.
pub struct WebViewWindow {
    pub(crate) inner: Rc<WebViewWindowInner>,
}

/// Window Type.
#[repr(i32)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WindowType {
    /// Popup type
    Popup = 0,
    /// Transparent type. Achieved using layer window.    
    Transparent = 1,
    /// Control type. Create window as child window. Requied parent.
    Control = 2,
}

impl WebViewWindow {
    /// Creates a new webview.
    pub fn new(
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
        WebViewWindow {
            inner: Rc::new(WebViewWindowInner { inner: webview }),
        }
    }

    /// Returns a weak reference to the webwiew object.
    pub fn as_weak(&self) -> Weak<WebViewWindowInner> {
        Rc::downgrade(&self.inner)
    }
}

impl WebViewTrait for Weak<WebViewWindowInner> {
    fn as_ptr(&self) -> miniblink_sys::mbWebView {
        self.upgrade().unwrap().inner
    }
}

impl WebViewWindowTrait for Weak<WebViewWindowInner> {
    fn as_ptr(&self) -> miniblink_sys::mbWebView {
        self.upgrade().unwrap().inner
    }
}

impl Drop for WebViewWindow {
    fn drop(&mut self) {
        if Rc::strong_count(&self.inner) == 1 {
            unsafe {
                call_api_or_panic().mbDestroyWebView(self.inner.inner);
            }
        }
    }
}

impl Default for WebViewWindow {
    fn default() -> Self {
        Self::new(WindowType::Popup, 0, 0, 0, 600, 400)
    }
}

impl WebViewTrait for WebViewWindow {
    fn as_ptr(&self) -> miniblink_sys::mbWebView {
        self.inner.inner
    }
}

impl WebViewWindowTrait for WebViewWindow {
    fn as_ptr(&self) -> miniblink_sys::mbWebView {
        self.inner.inner
    }
}
