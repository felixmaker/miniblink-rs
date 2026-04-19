use std::any::Any;
use std::collections::HashSet;
use std::ffi::*;
use std::hash::Hash;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex, Weak};

use crate::call_api_or_panic;
use crate::command::invoke_command_sync;
use crate::callback::*;
use crate::mbstring::MbString;
use crate::net_job::NetJob;
use crate::params::*;
use crate::types::*;

/// Webview ID.
pub type WebViewID = miniblink_sys::mbWebView;

/// Wraps to WebView
#[repr(transparent)]
pub struct WebView {
    inner: Arc<WebViewInner>,
}

/// Wraps to WebView.
pub(crate) struct WebViewInner {
    pub(crate) id: WebViewID,
    pub(crate) callbacks: Mutex<Vec<Box<dyn Any>>>,
    pub(crate) parent: Mutex<Option<Weak<WebViewInner>>>,
    pub(crate) childset: Mutex<HashSet<WebView>>,
}

pub(crate) struct CallBackContext<T> {
    webview: Weak<WebViewInner>,
    content: T,
}

impl<T> CallBackContext<T>
where
    T: Send + 'static,
{
    pub(crate) fn new(webview: &WebView, content: T) -> Box<Self> {
        Box::new(CallBackContext {
            webview: Arc::downgrade(&webview.inner),
            content: content,
        })
    }
}

impl WebView {
    /// Create a new offscreen webview.
    ///
    /// # Remarks
    /// This is for advanced users. It is recommended to use `WebViewWindow` to create a webview.
    pub fn new_offscreen() -> Self {
        let inner = unsafe { call_api_or_panic().mbCreateWebView() };
        unsafe { Self::from_raw(inner) }
    }

    /// Retake the inner pointer.
    ///
    /// # Remarks
    /// Only accept ptr from `mbCreateWebView` or `mbCreateWebWindow`, and make sure the pointer is valid,
    /// otherwise it will cause undefined behavior.
    pub(crate) unsafe fn from_raw(ptr: WebViewID) -> Self {
        assert!(ptr != 0, "Failed to create webview");
        // WEBVIEW_CONTENT.with_borrow_mut(|content| {
        //     if content.contains_key(&ptr) {
        //         return;
        //     }
        //     content.insert(ptr, WebViewContent::default());
        // });
        // let mut content = WEBVIEW_CONTENT_ASYNC.write().unwrap();
        // if !content.contains_key(&ptr) {
        //     content.insert(ptr, WebWindowContentAsync::default());
        // }
        let webview = WebViewInner {
            id: ptr,
            callbacks: Mutex::new(Vec::new()),
            parent: Mutex::new(None),
            childset: Mutex::new(HashSet::new()),
        };
        let webview = WebView {
            inner: Arc::new(webview),
        };
        webview
    }

    /// Get the inner pointer.
    pub fn as_ptr(&self) -> WebViewID {
        self.inner.id
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
        F: FnOnce(&WebView, &Option<String>) + Send + 'static,
    {
        use std::ffi::{c_int, c_void};

        let context = CallBackContext::new(self, callback);

        extern "system" fn shim<F>(
            _: WebViewID,
            param: *mut c_void,
            state: c_int,
            cookie: *const i8,
        ) where
            F: FnOnce(&WebView, &Option<String>) + Send + 'static,
        {
            let callback = unsafe { Box::from_raw(param as *mut Box<CallBackContext<F>>) };

            if let Some(webview) = callback.webview.upgrade() {
                let webview = WebView { inner: webview };
                let cookie = (state == 0)
                    .then_some(unsafe { CStr::from_ptr(cookie).to_string_lossy().to_string() });

                let callback = callback.content;
                let _ = catch_unwind(AssertUnwindSafe(|| callback(&webview, &cookie)));
            }
        }

        let param = Box::into_raw(Box::new(context));

        unsafe {
            call_api_or_panic().mbGetCookie(
                self.as_ptr(),
                Some(shim::<F>),
                param as *mut std::ffi::c_void,
            )
        };
    }

    /// Get the page cookies.
    pub fn get_cookie(&self) -> Option<String> {
        let ptr = self.as_ptr();
        invoke_command_sync(move || {
            let cookie = unsafe { call_api_or_panic().mbGetCookieOnBlinkThread(ptr) };
            if cookie.is_null() {
                None
            } else {
                Some(unsafe { CStr::from_ptr(cookie).to_string_lossy().to_string() })
            }
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
        F: OnQuery,
    {
        use miniblink_sys::{mbJsExecState, mbWebView};
        let context = self.store_callback_context(callback);

        extern "system" fn on_query<F>(
            _: mbWebView,
            context: *mut c_void,
            _es: mbJsExecState,
            query_id: i64,
            custom_msg: c_int,
            request: *const i8,
        ) where
            F: OnQuery,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };

            let Some(inner) = context.webview.upgrade() else {
                return;
            };
            let webview = WebView { inner };

            let request = unsafe { CStr::from_ptr(request).to_string_lossy().to_string() };
            let query_params = JsQueryParameters {
                custom_message: custom_msg,
                request,
            };

            if let Ok(result) = catch_unwind(AssertUnwindSafe(|| {
                (context.content)(&webview, &query_params)
            })) {
                let response = CString::new(result.response).unwrap();
                unsafe {
                    call_api_or_panic().mbResponseQuery(
                        webview.as_ptr(),
                        query_id,
                        result.custom_message,
                        response.as_ptr(),
                    )
                };
            }
        }

        unsafe { call_api_or_panic().mbOnJsQuery(self.as_ptr(), Some(on_query::<F>), context as _) }
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
        F: OnTitleChanged,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(_: WebViewID, context: *mut c_void, title: *const c_char)
        where
            F: OnTitleChanged,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return;
            };

            let webview = WebView { inner };
            let title = unsafe { CStr::from_ptr(title).to_string_lossy().to_string() };
            let _ = catch_unwind(AssertUnwindSafe(|| (context.content)(&webview, &title)));
        }

        unsafe {
            call_api_or_panic().mbOnTitleChanged(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set URL changed callback.
    pub fn on_url_changed<F>(&self, callback: F)
    where
        F: OnUrlChanged,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(
            _: WebViewID,
            context: *mut c_void,
            url: *const c_char,
            can_go_back: c_int,
            can_go_forward: c_int,
        ) where
            F: OnUrlChanged,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return;
            };

            let webview = WebView { inner };
            let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
            let param = UrlChangedParameters {
                url,
                can_go_back: can_go_back != 0,
                can_go_forward: can_go_forward != 0,
            };
            let _ = catch_unwind(AssertUnwindSafe(|| (context.content)(&webview, &param)));
        }

        unsafe {
            call_api_or_panic().mbOnURLChanged(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set alert box callback.
    pub fn on_alert_box<F>(&self, callback: F)
    where
        F: OnAlertBox,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(_: WebViewID, context: *mut c_void, message: *const c_char)
        where
            F: OnAlertBox,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return;
            };

            let webview = WebView { inner };
            let message = unsafe { CStr::from_ptr(message).to_string_lossy().to_string() };
            let _ = catch_unwind(AssertUnwindSafe(|| (context.content)(&webview, &message)));
        }

        unsafe {
            call_api_or_panic().mbOnAlertBox(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set confirm box callback.
    pub fn on_confirm_box<F>(&self, callback: F)
    where
        F: OnConfirmBox,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(
            _: WebViewID,
            context: *mut c_void,
            message: *const c_char,
        ) -> c_int
        where
            F: OnConfirmBox,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return 0;
            };

            let webview = WebView { inner };
            let message = unsafe { CStr::from_ptr(message).to_string_lossy().to_string() };
            match catch_unwind(AssertUnwindSafe(|| (context.content)(&webview, &message))) {
                Ok(true) => 1,
                _ => 0,
            }
        }

        unsafe {
            call_api_or_panic().mbOnConfirmBox(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set prompt box callback.
    pub fn on_prompt_box<F>(&self, callback: F)
    where
        F: OnPromptBox,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(
            _: WebViewID,
            context: *mut c_void,
            message: *const c_char,
            default_value: *const c_char,
            reject: *mut c_int,
        ) -> *mut miniblink_sys::mbString
        where
            F: OnPromptBox,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                unsafe { *reject = 0 }
                return std::ptr::null_mut();
            };

            let webview = WebView { inner };
            let message = unsafe { CStr::from_ptr(message).to_string_lossy().to_string() };
            let default_value =
                unsafe { CStr::from_ptr(default_value).to_string_lossy().to_string() };
            let prompt_params = PromptParams {
                message,
                default_value,
            };
            match catch_unwind(AssertUnwindSafe(|| {
                (context.content)(&webview, &prompt_params)
            })) {
                Ok(Some(result)) => {
                    unsafe { *reject = 1 };
                    MbString::new(result).unwrap().into_raw()
                }
                _ => {
                    unsafe { *reject = 0 };
                    std::ptr::null_mut()
                }
            }
        }

        unsafe {
            call_api_or_panic().mbOnPromptBox(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set navigation callback.
    ///
    /// Returns true to continue navigation, false to cancel navigation.
    pub fn on_navigation<F>(&self, callback: F)
    where
        F: OnNavigation,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(
            _: WebViewID,
            context: *mut c_void,
            navigation_type: c_int,
            url: *const c_char,
        ) -> c_int
        where
            F: OnNavigation,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return 1;
            };

            let webview = WebView { inner };
            let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
            let param = NavigationParameters {
                navigation_type: unsafe { std::mem::transmute(navigation_type) },
                url,
            };

            if let Ok(result) =
                catch_unwind(AssertUnwindSafe(|| (context.content)(&webview, &param)))
            {
                result as c_int
            } else {
                1
            }
        }

        unsafe {
            call_api_or_panic().mbOnNavigation(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set create view callback.
    ///
    /// Invoked when a new webview is created after <a> link click.
    pub fn on_create_view<F>(&self, callback: F)
    where
        F: OnCreateView,
    {
        use miniblink_sys::mbWindowFeatures;
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(
            _: WebViewID,
            context: *mut c_void,
            navigation_type: c_int,
            url: *const c_char,
            window_features: *const mbWindowFeatures,
        ) -> WebViewID
        where
            F: OnCreateView,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return 1;
            };

            let webview = WebView { inner };
            let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
            let navigation_type = unsafe { std::mem::transmute(navigation_type) };
            let window_features =
                WindowFeatures::from_mb_window_features(&unsafe { *window_features });
            let params = CreateViewParameters {
                navigation_type,
                url,
                window_features,
            };
            match catch_unwind(AssertUnwindSafe(|| (context.content)(&webview, &params))) {
                Ok(Some(child)) => webview.push_child(child),
                _ => 0,
            }
        }

        unsafe {
            call_api_or_panic().mbOnCreateView(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set a callback when the page DOM emits a ready event. It is possible to determine whether it is the main frame or not.
    pub fn on_document_ready<F>(&self, callback: F)
    where
        F: OnDocumentReady,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(_: WebViewID, context: *mut c_void, frame_id: *mut c_void)
        where
            F: OnDocumentReady,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return;
            };

            let webview = WebView { inner };
            let frame_id = WebFrameHandle { inner: frame_id };

            let _ = catch_unwind(AssertUnwindSafe(|| (context.content)(&webview, &frame_id)));
        }

        unsafe {
            call_api_or_panic().mbOnDocumentReady(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set a callback when the page emits download event. Some links are called when they trigger a download.
    pub fn on_download<F>(&self, callback: F)
    where
        F: OnDownload,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(
            _: WebViewID,
            context: *mut c_void,
            frame_id: miniblink_sys::mbWebFrameHandle,
            url: *const c_char,
            download_job: *mut c_void,
        ) -> c_int
        where
            F: OnDownload,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return 1;
            };

            let webview = WebView { inner };
            let frame_id = WebFrameHandle { inner: frame_id };
            let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
            let download_job = DownloadJob {
                inner: download_job,
            };
            let params = DownloadParameters {
                frame_id,
                url,
                download_job,
            };

            match catch_unwind(AssertUnwindSafe(|| (context.content)(&webview, &params))) {
                Ok(true) => 1,
                _ => 0,
            }
        }

        unsafe {
            call_api_or_panic().mbOnDownload(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set load URL begin callback.
    ///
    /// # Returns
    /// Returns true to cancel loading, false to continue loading.
    pub fn on_load_url_begin<F>(&self, callback: F)
    where
        F: OnLoadUrlBegin,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(
            _: WebViewID,
            context: *mut c_void,
            url: *const c_char,
            job: *mut c_void,
        ) -> c_int
        where
            F: OnLoadUrlBegin,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return 0;
            };

            let webview = WebView { inner };
            let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
            let job: NetJob = NetJob { inner: job };

            match catch_unwind(AssertUnwindSafe(|| (context.content)(&webview, &url, &job))) {
                Ok(false) => 0,
                _ => 1,
            }
        }

        unsafe {
            call_api_or_panic().mbOnLoadUrlBegin(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set load URL end callback.
    pub fn on_load_url_end<F>(&self, callback: F)
    where
        F: OnLoadUrlEnd,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(
            _: WebViewID,
            context: *mut c_void,
            url: *const c_char,
            job: *mut c_void,
            buf: *mut c_void,
            len: c_int,
        ) where
            F: OnLoadUrlEnd,
        {
            let context = unsafe { &*(context as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return;
            };

            let webview = WebView { inner };
            let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
            let job: NetJob = NetJob { inner: job };
            let buf = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, len as usize) };

            let _ = catch_unwind(AssertUnwindSafe(|| {
                (context.content)(&webview, &url, &job, buf)
            }));

            unsafe {
                std::ptr::drop_in_place(buf);
            }
        }

        unsafe {
            call_api_or_panic().mbOnLoadUrlEnd(self.as_ptr(), Some(shim::<F>), context as _);
        }
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

    /// Create a new webview window.
    pub fn new(typ: WindowType, x: i32, y: i32, width: i32, height: i32) -> Self {
        let id = unsafe {
            call_api_or_panic().mbCreateWebWindow(
                typ as _,
                std::ptr::null_mut(),
                x,
                y,
                width,
                height,
            )
        };
        let webview = unsafe { Self::from_raw(id) };
        // set_webwindow_handler(&webview);
        webview
    }

    /// Set close callback.
    pub fn on_close<F>(&self, callback: F)
    where
        F: OnClose,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(_: WebViewID, param: *mut c_void, _: *mut c_void) -> c_int
        where
            F: OnClose,
        {
            let context = unsafe { &*(param as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return 1;
            };
            let webview = WebView { inner };

            if let Ok(result) = catch_unwind(AssertUnwindSafe(|| (context.content)(&webview))) {
                result as c_int
            } else {
                1
            }
        }

        unsafe {
            call_api_or_panic().mbOnClose(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Set destroy callback.
    pub fn on_destroy<F>(&self, callback: F)
    where
        F: OnDestroy,
    {
        let context = self.store_callback_context(callback);

        extern "system" fn shim<F>(_: WebViewID, param: *mut c_void, _: *mut c_void) -> c_int
        where
            F: OnDestroy,
        {
            let context = unsafe { &*(param as *const CallBackContext<F>) };
            let Some(inner) = context.webview.upgrade() else {
                return 1;
            };
            let webview = WebView { inner };

            if let Ok(result) = catch_unwind(AssertUnwindSafe(|| (context.content)(&webview))) {
                result as c_int
            } else {
                1
            }
        }

        unsafe {
            call_api_or_panic().mbOnDestroy(self.as_ptr(), Some(shim::<F>), context as _);
        }
    }

    /// Show the window.
    pub fn show(&self) {
        unsafe {
            call_api_or_panic().mbShowWindow(self.as_ptr(), 1);
        }
    }

    /// Hide the window.
    pub fn hide(&self) {
        unsafe {
            call_api_or_panic().mbShowWindow(self.as_ptr(), 0);
        }
    }

    // /// Resize the window.
    // pub fn resize(&self, width: i32, height: i32) {
    //     unsafe { call_api_or_panic().mbResize(self.as_ptr(), width, height) }
    // }

    /// Move the window.
    pub fn move_window(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            call_api_or_panic().mbMoveWindow(self.as_ptr(), x, y, width, height);
        }
    }

    /// Move the window to center.
    pub fn move_to_center(&self) {
        unsafe {
            call_api_or_panic().mbMoveToCenter(self.as_ptr());
        }
    }

    /// Set the window title.
    pub fn set_window_title(&self, title: &str) {
        let title = CString::new(title).unwrap();
        unsafe { call_api_or_panic().mbSetWindowTitle(self.as_ptr(), title.as_ptr()) }
    }

    fn store_callback_context<T>(&self, callback: T) -> *const CallBackContext<T>
    where
        T: Send + 'static,
    {
        let context = CallBackContext::new(self, callback);
        let param = &*context as *const _;
        self.inner.callbacks.lock().unwrap().push(context);
        param
    }

    fn push_child(&self, child: WebView) -> WebViewID {
        let id = child.as_ptr();
        *child.inner.parent.lock().unwrap() = Some(Arc::downgrade(&self.inner));
        self.inner.childset.lock().unwrap().insert(child);
        id
    }

    /// Get the parent window handle.
    pub fn parent(&self) -> Option<WebView> {
        self.inner
            .parent
            .lock()
            .unwrap()
            .as_ref()
            .and_then(|x| x.upgrade())
            .map(|inner| WebView { inner })
    }
}

impl PartialEq for WebView {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl Eq for WebView {}

impl Hash for WebView {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.id.hash(state);
    }
}

impl Default for WebView {
    fn default() -> Self {
        let window = Self::new(WindowType::Popup, 0, 0, 800, 600);
        window.move_to_center();
        window
    }
}

impl Drop for WebViewInner {
    fn drop(&mut self) {
        unsafe { call_api_or_panic().mbDestroyWebView(self.id) };
    }
}
