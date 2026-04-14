use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ffi::{c_char, c_int, c_void, CStr, CString},
    rc::Rc,
    sync::{Arc, LazyLock, Mutex, RwLock},
};

use miniblink_sys::{
    mbJsExecState, mbNavigationType, mbString, mbWebFrameHandle, mbWebView, mbWindowFeatures,
    MbAsynRequestState,
};

use crate::{
    call_api_or_panic, mbstring::MbString, net_job::NetJob, params::*, types::*, webview::WebView,
    webwindow::WebViewWindow,
};

/// Defines the content.
#[derive(Default)]
pub(crate) struct WebViewContent {
    // WebView callbacks.
    pub(crate) on_download:
        Option<Rc<RefCell<dyn FnMut(&mut WebView, &DownloadParameters) -> bool + 'static>>>,
    pub(crate) on_document_ready:
        Option<Rc<RefCell<dyn FnMut(&mut WebView, &WebFrameHandle) + 'static>>>,
    pub(crate) on_navigation:
        Option<Rc<RefCell<dyn FnMut(&mut WebView, &NavigationParameters) -> bool + 'static>>>,
    pub(crate) on_create_view: Option<
        Rc<
            RefCell<
                dyn FnMut(&mut WebView, &CreateViewParameters) -> Option<WebViewWindow> + 'static,
            >,
        >,
    >,
    pub(crate) on_query:
        Option<Rc<RefCell<dyn FnMut(&mut WebView, &JsQueryParameters) -> JsQueryResult + 'static>>>,
    pub(crate) on_get_cookie:
        Option<Rc<RefCell<dyn FnMut(&mut WebView, &GetCookieParameters) + 'static>>>,
    pub(crate) on_url_changed:
        Option<Rc<RefCell<dyn FnMut(&mut WebView, &UrlChangedParameters) + 'static>>>,
    pub(crate) on_title_changed: Option<Rc<RefCell<dyn FnMut(&mut WebView, &str) + 'static>>>,

    // Dialog callbacks.
    pub(crate) on_alert_box: Option<Rc<RefCell<dyn FnMut(&mut WebView, &str) -> bool + 'static>>>,
    pub(crate) on_confirm_box: Option<Rc<RefCell<dyn FnMut(&mut WebView, &str) -> bool + 'static>>>,
    pub(crate) on_prompt_box:
        Option<Rc<RefCell<dyn FnMut(&mut WebView, &PromptParams) -> Option<String> + 'static>>>,

    // Window callbacks.
    pub(crate) on_close: Option<Rc<RefCell<dyn FnMut(&mut WebView) -> bool + 'static>>>,
    pub(crate) on_destroy: Option<Rc<RefCell<dyn FnMut(&mut WebView) -> bool + 'static>>>,

    pub(crate) parent: Option<mbWebView>,
    pub(crate) child: HashSet<mbWebView>,
}

#[derive(Default)]
pub(crate) struct WebWindowContentAsync {
    // Net callbacks.
    pub(crate) on_load_url_begin:
        Option<Arc<Mutex<dyn FnMut(&str, &NetJob) -> bool + Send + 'static>>>,
}

thread_local! {
    pub(crate) static WEBVIEW_CONTENT: RefCell<HashMap<mbWebView, WebViewContent>> = RefCell::new(HashMap::new());
}

pub(crate) static WEBVIEW_CONTENT2: LazyLock<RwLock<HashMap<mbWebView, WebWindowContentAsync>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

fn set_on_close_callback(webview: &WebView) {
    extern "system" fn on_close(
        mut view: mbWebView,
        _param: *mut c_void,
        _unuse: *mut c_void,
    ) -> c_int {
        let view: &mut WebView = unsafe { std::mem::transmute(&mut view) };
        WEBVIEW_CONTENT
            .with_borrow(|content| content.get(&view.as_ptr()).and_then(|x| x.on_close.clone()))
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f.borrow_mut()(view))).ok()
            })
            .map(|r| if r { 1 } else { 0 })
            .unwrap_or(1)
    }

    unsafe {
        call_api_or_panic().mbOnClose(webview.as_ptr(), Some(on_close), std::ptr::null_mut() as _);
    }
}

fn set_on_destroy_callback(webview: &WebView) {
    extern "system" fn on_destroy(
        mut view: mbWebView,
        _param: *mut c_void,
        _unuse: *mut c_void,
    ) -> c_int {
        let view: &mut WebView = unsafe { std::mem::transmute(&mut view) };
        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&view.as_ptr())
                    .and_then(|x| x.on_destroy.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f.borrow_mut()(view))).ok()
            })
            .map(|r| if r { 1 } else { 0 })
            .unwrap_or(1)
    }

    unsafe {
        call_api_or_panic().mbOnDestroy(
            webview.as_ptr(),
            Some(on_destroy),
            std::ptr::null_mut() as _,
        );
    }
}

fn set_get_cookie_callback(webview: &WebView) {
    extern "system" fn get_cookie(
        mut webview: mbWebView,
        _param: *mut c_void,
        state: MbAsynRequestState,
        cookie: *const i8,
    ) {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let cookies = unsafe { std::ffi::CStr::from_ptr(cookie) };
        let param = GetCookieParameters {
            state: unsafe { std::mem::transmute(state) },
            cookie: cookies.to_string_lossy().to_string(),
        };
        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_get_cookie.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f.borrow_mut()(webview, &param)
                }))
                .ok()
            });
    }

    unsafe {
        call_api_or_panic().mbGetCookie(webview.as_ptr(), Some(get_cookie), std::ptr::null_mut())
    }
}

fn set_query_callback(webview: &WebView) {
    extern "system" fn on_query(
        mut webview: mbWebView,
        _param: *mut c_void,
        _es: mbJsExecState,
        query_id: i64,
        custom_msg: c_int,
        request: *const i8,
    ) {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let request = unsafe { CStr::from_ptr(request).to_string_lossy().to_string() };
        let query_message = JsQueryParameters {
            custom_message: custom_msg,
            request,
        };

        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_query.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f.borrow_mut()(webview, &query_message)
                }))
                .ok()
            })
            .and_then(|result| {
                let response = CString::new(result.response).unwrap();
                unsafe {
                    call_api_or_panic().mbResponseQuery(
                        webview.as_ptr(),
                        query_id,
                        result.custom_message,
                        response.as_ptr(),
                    )
                };
                Some(())
            });
    }

    unsafe {
        call_api_or_panic().mbOnJsQuery(webview.as_ptr(), Some(on_query), std::ptr::null_mut())
    }
}

fn set_url_changed_callback(webview: &WebView) {
    extern "system" fn on_url_changed(
        mut webview: mbWebView,
        _param: *mut c_void,
        url: *const i8,
        can_go_back: i32,
        can_go_forward: i32,
    ) {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
        let param = UrlChangedParameters {
            url,
            can_go_back: can_go_back != 0,
            can_go_forward: can_go_forward != 0,
        };
        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_url_changed.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f.borrow_mut()(webview, &param)
                }))
                .ok()
            });
    }

    unsafe {
        call_api_or_panic().mbOnURLChanged(
            webview.as_ptr(),
            Some(on_url_changed),
            std::ptr::null_mut(),
        );
    }
}

fn set_navigation_callback(webview: &WebView) {
    extern "system" fn on_navigation(
        mut webview: mbWebView,
        _param: *mut c_void,
        navigation_type: i32,
        url: *const i8,
    ) -> i32 {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
        let param = NavigationParameters {
            navigation_type: unsafe { std::mem::transmute(navigation_type) },
            url,
        };
        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_navigation.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f.borrow_mut()(webview, &param)
                }))
                .ok()
            })
            .map(|x| if x { 1 } else { 0 })
            .unwrap_or(1)
    }

    unsafe {
        call_api_or_panic().mbOnNavigation(
            webview.as_ptr(),
            Some(on_navigation),
            std::ptr::null_mut(),
        );
    }
}

fn set_document_ready_callback(webview: &WebView) {
    extern "system" fn on_document_ready(
        mut webview: mbWebView,
        _param: *mut c_void,
        frame_id: *mut c_void,
    ) {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let frame_id = WebFrameHandle { inner: frame_id };
        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_document_ready.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f.borrow_mut()(webview, &frame_id)
                }))
                .ok()
            });
    }

    unsafe {
        call_api_or_panic().mbOnDocumentReady(
            webview.as_ptr(),
            Some(on_document_ready),
            std::ptr::null_mut(),
        );
    }
}

fn set_download_callback(webview: &WebView) {
    extern "system" fn on_download(
        mut webview: mbWebView,
        _param: *mut c_void,
        frame_id: mbWebFrameHandle,
        url: *const c_char,
        download_job: *mut c_void,
    ) -> i32 {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let frame_id = WebFrameHandle { inner: frame_id };
        let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
        let download_job = DownloadJob {
            inner: download_job,
        };
        let download_parameters = DownloadParameters {
            frame_id,
            url,
            download_job,
        };
        println!("Download callback: {:?}", download_parameters.url);
        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_download.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f.borrow_mut()(webview, &download_parameters)
                }))
                .ok()
            })
            .map(|x| if x { 1 } else { 0 })
            .unwrap_or(1)
    }

    unsafe {
        call_api_or_panic().mbOnDownload(webview.as_ptr(), Some(on_download), std::ptr::null_mut());
    }
}

fn set_title_changed_callback(webview: &WebView) {
    extern "system" fn on_title_changed(
        mut webview: mbWebView,
        _param: *mut c_void,
        title: *const c_char,
    ) {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let title = unsafe { CStr::from_ptr(title).to_string_lossy().to_string() };
        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_title_changed.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f.borrow_mut()(webview, &title)
                }))
                .ok()
            });
    }

    unsafe {
        call_api_or_panic().mbOnTitleChanged(
            webview.as_ptr(),
            Some(on_title_changed),
            std::ptr::null_mut(),
        );
    }
}

fn set_alert_box_callback(webview: &WebView) {
    extern "system" fn on_alert_box(
        mut webview: mbWebView,
        _param: *mut c_void,
        message: *const c_char,
    ) {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let message = unsafe { CStr::from_ptr(message).to_string_lossy().to_string() };
        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_alert_box.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f.borrow_mut()(webview, &message)
                }))
                .ok()
            });
    }

    unsafe {
        call_api_or_panic().mbOnAlertBox(
            webview.as_ptr(),
            Some(on_alert_box),
            std::ptr::null_mut(),
        );
    }
}

fn set_confirm_box_callback(webview: &WebView) {
    extern "system" fn on_confirm_box(
        mut webview: mbWebView,
        _param: *mut c_void,
        message: *const c_char,
    ) -> i32 {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let message = unsafe { CStr::from_ptr(message).to_string_lossy().to_string() };
        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_confirm_box.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f.borrow_mut()(webview, &message)
                }))
                .ok()
            })
            .map(|r| if r { 1 } else { 0 })
            .unwrap_or(1)
    }

    unsafe {
        call_api_or_panic().mbOnConfirmBox(
            webview.as_ptr(),
            Some(on_confirm_box),
            std::ptr::null_mut(),
        );
    }
}

fn set_prompt_box_callback(webview: &WebView) {
    extern "system" fn on_prompt_box(
        mut webview: mbWebView,
        _param: *mut c_void,
        message: *const c_char,
        default_value: *const c_char,
        reject: *mut i32,
    ) -> *mut mbString {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let message = unsafe { CStr::from_ptr(message).to_string_lossy().to_string() };
        let default_value = unsafe { CStr::from_ptr(default_value).to_string_lossy().to_string() };
        let prompt_params = PromptParams {
            message,
            default_value,
        };

        WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_prompt_box.clone())
            })
            .and_then(|f| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f.borrow_mut()(webview, &prompt_params)
                }))
                .ok()
            })
            .and_then(|result| match result {
                Some(r) => {
                    unsafe { *reject = 1 };
                    Some(MbString::new(r).unwrap().into_raw())
                }
                None => {
                    unsafe { *reject = 0 };
                    None
                }
            })
            .unwrap_or(std::ptr::null_mut())
    }

    unsafe {
        call_api_or_panic().mbOnPromptBox(
            webview.as_ptr(),
            Some(on_prompt_box),
            std::ptr::null_mut(),
        );
    }
}

fn set_create_view_callback(webview: &WebView) {
    extern "system" fn on_create_view(
        mut webview: mbWebView,
        _param: *mut c_void,
        navigation_type: mbNavigationType,
        url: *const c_char,
        window_features: *const mbWindowFeatures,
    ) -> mbWebView {
        let webview: &mut WebView = unsafe { std::mem::transmute(&mut webview) };
        let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
        let navigation_type = unsafe { std::mem::transmute(navigation_type) };
        let window_features = WindowFeatures::from_mb_window_features(&unsafe { *window_features });
        let params = CreateViewParameters {
            navigation_type,
            url,
            window_features,
        };

        let view = WEBVIEW_CONTENT
            .with_borrow(|content| {
                content
                    .get(&webview.as_ptr())
                    .and_then(|x| x.on_create_view.clone())
            })
            .and_then(|f| f.borrow_mut()(webview, &params));

        WEBVIEW_CONTENT
            .with_borrow_mut(|content| {
                let content = content.get_mut(&webview.as_ptr()).unwrap();
                match view {
                    Some(view) => {
                        content.parent = Some(webview.as_ptr());
                        content.child.insert(view.as_ptr());
                        Some(std::mem::ManuallyDrop::new(view))
                    }
                    None => None,
                }
            })
            .and_then(|x| {
                x.on_close(|webview| {
                    unsafe { webview.destroy() };
                    true
                });
                Some(x.as_ptr())
            })
            .unwrap_or(0)
    }

    unsafe {
        call_api_or_panic().mbOnCreateView(
            webview.as_ptr(),
            Some(on_create_view),
            std::ptr::null_mut(),
        )
    };
}

pub(crate) fn set_on_load_url_begin_callback(webview: &WebView) {
    extern "system" fn on_load_url_begin(
        webview: mbWebView,
        _param: *mut c_void,
        url: *const c_char,
        job: *mut c_void,
    ) -> i32 {
        let url = unsafe { CStr::from_ptr(url).to_string_lossy().to_string() };
        let job: NetJob = NetJob { inner: job };

        let callback = {
            let content = WEBVIEW_CONTENT2.read().unwrap();
            let content = content.get(&webview).unwrap();
            content.on_load_url_begin.clone()
        };

        callback
            .and_then(|f| {
                let mut f = f.lock().unwrap();
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&url, &job))).ok()
            })
            .map(|x| if x { 1 } else { 0 })
            .unwrap_or(0)
    }

    unsafe {
        call_api_or_panic().mbOnLoadUrlBegin(
            webview.as_ptr(),
            Some(on_load_url_begin),
            std::ptr::null_mut(),
        );
    }

    unsafe {
        call_api_or_panic().mbOnLoadUrlBegin(
            webview.as_ptr(),
            Some(on_load_url_begin),
            std::ptr::null_mut(),
        );
    }
}

pub(crate) fn set_webwindow_handler(webview: &WebView) {
    set_on_close_callback(webview);
    set_on_destroy_callback(webview);
}

pub(crate) fn set_webview_handler(webview: &WebView) {
    set_on_close_callback(webview);
    set_on_destroy_callback(webview);
    set_get_cookie_callback(webview);
    set_query_callback(webview);
    set_url_changed_callback(webview);
    set_navigation_callback(webview);
    set_document_ready_callback(webview);
    set_download_callback(webview);
    set_title_changed_callback(webview);
    set_alert_box_callback(webview);
    set_confirm_box_callback(webview);
    set_prompt_box_callback(webview);
    set_create_view_callback(webview);
    set_on_load_url_begin_callback(webview);
}
