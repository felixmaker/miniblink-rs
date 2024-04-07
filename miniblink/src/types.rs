use std::{
    ffi::{c_char, CStr, CString},
    mem::ManuallyDrop,
};

use miniblink_sys::*;

use crate::{
    call_api_or_panic,
    prelude::{MBError, MBResult},
    util::{string_to_slice, SafeCString},
    webview::WebView,
};

#[allow(missing_docs)]
pub struct TempCallbackInfo {
    pub size: i32,
    pub frame: WebFrameHandle,
    pub will_send_request_info: WillSendRequestInfo,
    pub url: String,
    pub post_body: PostBodyElements,
    pub job: NetJob,
}

impl TempCallbackInfo {
    pub(crate) fn from_wke(wke: wkeTempCallbackInfo) -> Self {
        let wkeTempCallbackInfo {
            size,
            frame,
            willSendRequestInfo,
            url,
            postBody,
            job,
        } = wke;

        assert!(!frame.is_null());
        assert!(!url.is_null());
        assert!(!willSendRequestInfo.is_null());
        assert!(!postBody.is_null());
        assert!(!job.is_null());

        Self {
            size,
            frame: unsafe { WebFrameHandle::from_ptr(frame) },
            will_send_request_info: unsafe { WillSendRequestInfo::from_ptr(willSendRequestInfo) },
            url: unsafe { CStr::from_ptr(url) }.to_string_lossy().to_string(),
            post_body: unsafe { PostBodyElements::from_ptr(postBody) },
            job: unsafe { NetJob::from_ptr(job) },
        }
    }
}

#[allow(missing_docs)]
pub struct WillSendRequestInfo {
    pub url: String,
    pub new_url: String,
    pub resource_type: ResourceType,
    pub http_response_code: i32,
    pub method: String,
    pub referrer: String,
}

impl WillSendRequestInfo {
    pub(crate) unsafe fn from_ptr(ptr: *mut wkeWillSendRequestInfo) -> Self {
        macro_rules! wke_string {
            ($expr: expr) => {{
                let ptr = $expr;
                assert!(!ptr.is_null());
                unsafe { WkeStr::from_ptr(ptr) }.to_string()
            }};
        }

        assert!(!ptr.is_null());
        let wkeWillSendRequestInfo {
            url,
            newUrl,
            resourceType,
            httpResponseCode,
            method,
            referrer,
            ..
        } = *ptr;

        Self {
            url: wke_string!(url),
            new_url: wke_string!(newUrl),
            resource_type: ResourceType::from_wke(resourceType),
            http_response_code: httpResponseCode,
            method: wke_string!(method),
            referrer: wke_string!(referrer),
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    MainFrame,
    SubFrame,
    Stylesheet,
    Script,
    Image,
    FontResource,
    SubResource,
    Object,
    Media,
    Worker,
    SharedWorker,
    Prefetch,
    Favicon,
    Xhr,
    Ping,
    ServiceWorker,
    LastType,
}

impl ResourceType {
    pub(crate) fn from_wke(wke: wkeResourceType) -> Self {
        match wke {
            wkeResourceType::WKE_RESOURCE_TYPE_MAIN_FRAME => ResourceType::MainFrame,
            wkeResourceType::WKE_RESOURCE_TYPE_SUB_FRAME => ResourceType::SubFrame,
            wkeResourceType::WKE_RESOURCE_TYPE_STYLESHEET => ResourceType::Stylesheet,
            wkeResourceType::WKE_RESOURCE_TYPE_SCRIPT => ResourceType::Script,
            wkeResourceType::WKE_RESOURCE_TYPE_IMAGE => ResourceType::Image,
            wkeResourceType::WKE_RESOURCE_TYPE_FONT_RESOURCE => ResourceType::FontResource,
            wkeResourceType::WKE_RESOURCE_TYPE_SUB_RESOURCE => ResourceType::SubResource,
            wkeResourceType::WKE_RESOURCE_TYPE_OBJECT => ResourceType::Object,
            wkeResourceType::WKE_RESOURCE_TYPE_MEDIA => ResourceType::Media,
            wkeResourceType::WKE_RESOURCE_TYPE_WORKER => ResourceType::Worker,
            wkeResourceType::WKE_RESOURCE_TYPE_SHARED_WORKER => ResourceType::SharedWorker,
            wkeResourceType::WKE_RESOURCE_TYPE_PREFETCH => ResourceType::Prefetch,
            wkeResourceType::WKE_RESOURCE_TYPE_FAVICON => ResourceType::Favicon,
            wkeResourceType::WKE_RESOURCE_TYPE_XHR => ResourceType::Xhr,
            wkeResourceType::WKE_RESOURCE_TYPE_PING => ResourceType::Ping,
            wkeResourceType::WKE_RESOURCE_TYPE_SERVICE_WORKER => ResourceType::ServiceWorker,
            wkeResourceType::WKE_RESOURCE_TYPE_LAST_TYPE => ResourceType::LastType,
            _ => unimplemented!(),
        }
    }
}

#[allow(missing_docs)]
pub struct PostBodyElement {
    // pub size: i32,
    // pub type_: HttpBodyElementType,
    // pub data: MemBuf,
    // pub file_path: String,
    // pub file_start: i64,
    // pub file_length: i64,
    inner: *mut wkePostBodyElement,
}

impl PostBodyElement {
    pub(crate) unsafe fn from_ptr(ptr: *mut wkePostBodyElement) -> Self {
        // let wkePostBodyElement {
        //     size,
        //     type_,
        //     data,
        //     filePath,
        //     fileStart,
        //     fileLength,
        // } = wke;
        // assert!(!data.is_null());
        // assert!(!filePath.is_null());
        // Self {
        //     size,
        //     type_: HttpBodyElementType::from_wke(type_),
        //     data: unsafe { MemBuf::from_ptr(data) },
        //     file_path: unsafe { WkeStr::from_ptr(filePath) }.to_string(),
        //     file_start: fileStart,
        //     file_length: fileLength,
        // }
        Self { inner: ptr }
    }

    /// Create the post body element.
    pub fn create(webview: &WebView) -> PostBodyElement {
        webview.net_create_post_body_element()
    }

    /// Free the post body element.
    pub fn free(element: &PostBodyElement) {
        unsafe { call_api_or_panic().wkeNetFreePostBodyElement(element.inner) }
    }
}

#[allow(missing_docs)]
pub struct PostBodyElements {
    inner: *mut wkePostBodyElements,
}

impl PostBodyElements {
    pub(crate) unsafe fn from_ptr(ptr: *mut wkePostBodyElements) -> Self {
        // let wkePostBodyElements {
        //     size,
        //     element,
        //     elementSize,
        //     isDirty,
        // } = wke;
        // let slice = std::ptr::slice_from_raw_parts_mut(element, elementSize);
        // assert!(!slice.is_null());
        // let mut elements = Vec::with_capacity(elementSize);
        // for item in &unsafe { *slice } {
        //     elements.push(PostBodyElement::from_wke(unsafe { **item }))
        // }
        // Self {
        //     size,
        //     elements,
        //     is_dirty: isDirty,
        // }
        Self { inner: ptr }
    }

    /// Create the post body elements.
    pub fn create(webview: &WebView, length: usize) -> PostBodyElements {
        webview.net_create_post_body_elements(length)
    }

    /// Free the post body elements.
    pub fn free(elements: &PostBodyElements) {
        unsafe { call_api_or_panic().wkeNetFreePostBodyElements(elements.inner) }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpBodyElementType {
    Data,
    File,
}

impl HttpBodyElementType {
    #[allow(unused)]
    pub(crate) fn from_wke(wke: wkeHttBodyElementType) -> Self {
        match wke {
            wkeHttBodyElementType::wkeHttBodyElementTypeData => Self::Data,
            wkeHttBodyElementType::wkeHttBodyElementTypeFile => Self::File,
            _ => unimplemented!(),
        }
    }
}

/// MemBuf. May change!
pub struct MemBuf {
    inner: *mut wkeMemBuf,
}

impl MemBuf {
    pub(crate) unsafe fn from_ptr(ptr: *mut wkeMemBuf) -> Self {
        Self { inner: ptr }
    }

    /// Get unuse.
    pub fn get_unuse(&self) -> i32 {
        unsafe { *self.inner }.unuse
    }

    /// Get length.
    pub fn get_length(&self) -> usize {
        unsafe { *self.inner }.length
    }

    /// Get data.
    pub unsafe fn get_data(&self) -> *mut ::std::ffi::c_void {
        (*self.inner).data
    }
}

/// NetJob. Wraps to wkeNetJob.
#[repr(transparent)]
pub struct NetJob {
    inner: wkeNetJob,
}
impl NetJob {
    /// #Safety
    /// Pointer should be valid.
    pub unsafe fn from_ptr(ptr: wkeNetJob) -> Self {
        Self { inner: ptr }
    }
    /// Set http header field.
    pub fn set_http_header_field(&self, key: &str, value: &str) {
        let key = WkeString::new(key);
        let value = WkeString::new(value);
        unsafe {
            call_api_or_panic().wkeNetSetHTTPHeaderField(
                self.inner,
                key.as_wcstr_ptr(),
                value.as_wcstr_ptr(),
                false,
            )
        }
    }
    /// Set http header field.
    pub fn get_raw_http_head(&self) -> Slist {
        let slist = unsafe { call_api_or_panic().wkeNetGetRawHttpHead(self.inner) };
        assert!(!slist.is_null());
        unsafe { Slist::from_ptr(slist) }
    }
    /// Set MIME type.
    pub fn set_mime_type(&self, mine_type: &str) {
        let mine_type = CString::safe_new(mine_type);
        unsafe { call_api_or_panic().wkeNetSetMIMEType(self.inner, mine_type.as_ptr()) };
    }
    /// Get the mine type.
    pub fn get_mime_type(&self, mime_type: Option<&str>) -> String {
        let mime_type = if let Some(mine_type) = mime_type {
            let mine_type = WkeString::new(mine_type);
            unsafe { call_api_or_panic().wkeNetGetMIMEType(self.inner, mine_type.as_ptr()) }
        } else {
            unsafe { call_api_or_panic().wkeNetGetMIMEType(self.inner, std::ptr::null_mut()) }
        };
        unsafe { CStr::from_ptr(mime_type) }
            .to_string_lossy()
            .to_string()
    }
    /// Cancel request.
    pub fn cancel_request(&self) {
        unsafe { call_api_or_panic().wkeNetCancelRequest(self.inner) }
    }
    /// Hold job to async commit. Call `continue_job` to continue.
    /// Ture means success.
    pub fn hold_job_to_asyn_commit(&self) -> bool {
        (unsafe { call_api_or_panic().wkeNetHoldJobToAsynCommit(self.inner) }).as_bool()
    }
    /// Continue the job. Use after `hold_job_to_asyn_commit`.
    pub fn continue_job(&self) {
        unsafe { call_api_or_panic().wkeNetContinueJob(self.inner) }
    }
    /// Get request method.
    pub fn get_request_method(&self) -> RequestType {
        let method = unsafe { call_api_or_panic().wkeNetGetRequestMethod(self.inner) };
        RequestType::from_wke(method)
    }
    /// Get the post body.
    pub fn get_post_body(&self) -> PostBodyElements {
        let elements = unsafe { call_api_or_panic().wkeNetGetPostBody(self.inner) };
        assert!(!elements.is_null());
        unsafe { PostBodyElements::from_ptr(elements) }
    }
    /// Get the post body.
    pub fn hook_request(&self) {
        unsafe { call_api_or_panic().wkeNetHookRequest(self.inner) }
    }
}

#[allow(missing_docs)]
pub enum RequestType {
    Invalidation,
    Get,
    Post,
    Put,
}

impl RequestType {
    pub(crate) fn from_wke(value: wkeRequestType) -> Self {
        match value {
            wkeRequestType::kWkeRequestTypeInvalidation => Self::Invalidation,
            wkeRequestType::kWkeRequestTypeGet => Self::Get,
            wkeRequestType::kWkeRequestTypePost => Self::Post,
            wkeRequestType::kWkeRequestTypePut => Self::Put,
            _ => unimplemented!(),
        }
    }
}

/// Slist. Wraps to wkeSlist.
#[repr(transparent)]
pub struct Slist {
    inner: *const wkeSlist,
}

impl Slist {
    pub(crate) unsafe fn from_ptr(ptr: *const wkeSlist) -> Self {
        Self { inner: ptr }
    }
}

#[allow(missing_docs)]
pub struct SlintIter {
    current: *const wkeSlist,
}

impl Iterator for SlintIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            let current = unsafe { *self.current };
            match (current.data.is_null(), current.next.is_null()) {
                (false, _) => {
                    self.current = current.next;
                    assert!(!current.data.is_null());
                    let data = unsafe { CStr::from_ptr(current.data) };
                    Some(data.to_string_lossy().to_string())
                }
                (true, false) => {
                    self.current = current.next;
                    Some(String::new())
                }
                (true, true) => None,
            }
        }
    }
}

impl IntoIterator for Slist {
    type Item = String;

    type IntoIter = SlintIter;

    fn into_iter(self) -> Self::IntoIter {
        SlintIter {
            current: self.inner,
        }
    }
}

#[allow(missing_docs)]
/// Proxy type
pub enum ProxyType {
    None,
    Http,
    Socks4,
    Socks4A,
    Socks5,
    Socks5Hostname,
}

impl ProxyType {
    fn to_wke_proxy_type(&self) -> wkeProxyType {
        match self {
            ProxyType::None => wkeProxyType::WKE_PROXY_NONE,
            ProxyType::Http => wkeProxyType::WKE_PROXY_HTTP,
            ProxyType::Socks4 => wkeProxyType::WKE_PROXY_SOCKS4,
            ProxyType::Socks4A => wkeProxyType::WKE_PROXY_SOCKS4A,
            ProxyType::Socks5 => wkeProxyType::WKE_PROXY_SOCKS5,
            ProxyType::Socks5Hostname => wkeProxyType::WKE_PROXY_SOCKS5HOSTNAME,
        }
    }
}

/// Proxy
pub struct Proxy {
    /// Proxy type
    pub type_: ProxyType,
    /// Hostname: eg. 127.0.0.1
    pub hostname: String,
    /// Port: eg. 443
    pub port: u16,
    /// Proxy username
    pub username: String,
    /// Proxy password
    pub password: String,
}

impl Proxy {
    pub(crate) fn to_wke(&self) -> wkeProxy {
        wkeProxy {
            type_: self.type_.to_wke_proxy_type(),
            hostname: string_to_slice(&self.hostname),
            port: self.port,
            username: string_to_slice(&self.username),
            password: string_to_slice(&self.password),
        }
    }
}

/// JsType
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsType {
    Number,
    String,
    Boolean,
    Object,
    Function,
    Undefined,
    Array,
    Null,
}

impl JsType {
    pub(crate) fn from_wke(value: jsType) -> Self {
        match value {
            jsType::JSTYPE_ARRAY => Self::Array,
            jsType::JSTYPE_BOOLEAN => Self::Boolean,
            jsType::JSTYPE_FUNCTION => Self::Function,
            jsType::JSTYPE_NULL => Self::Null,
            jsType::JSTYPE_NUMBER => Self::Number,
            jsType::JSTYPE_OBJECT => Self::Object,
            jsType::JSTYPE_STRING => Self::String,
            jsType::JSTYPE_UNDEFINED => Self::Undefined,
            _ => unimplemented!(),
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            JsType::Number => "NUMBER",
            JsType::String => "STRING",
            JsType::Boolean => "BOOLEAN",
            JsType::Object => "OBJECT",
            JsType::Function => "FUNCTION",
            JsType::Undefined => "UNDEFINED",
            JsType::Array => "ARRAY",
            JsType::Null => "NULL",
        }
    }
}

impl std::fmt::Display for JsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

macro_rules! js_value {
    ($expr: expr) => {{
        let value = $expr;
        assert!(value != 0);
        unsafe { JsValue::from_ptr(value) }
    }};
}

/// JsExecState. Wraps to jsExecState.
#[derive(Clone, Copy)]
pub struct JsExecState {
    pub(crate) inner: jsExecState,
}

impl JsExecState {
    /// Create an int.
    pub fn int(&self, value: i32) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsInt(value) })
    }
    /// Create a double.
    pub fn double(&self, value: f64) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsDouble(value) })
    }
    /// Create a boolean.
    pub fn boolean(&self, value: bool) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsBoolean(value) })
    }
    /// Create an undefined.
    pub fn undefined(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsUndefined() })
    }
    /// Create a null.
    pub fn null(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsNull() })
    }
    /// Get the arg value.
    pub fn arg(&self, index: i32) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsArg(self.inner, index) })
    }
    /// Get the arg count.
    pub fn arg_count(&self) -> i32 {
        unsafe { call_api_or_panic().jsArgCount(self.inner) }
    }
    /// Get the arg type.
    pub fn arg_type(&self, index: i32) -> Option<JsType> {
        if index > self.arg_count() {
            return None;
        }
        let js_type = unsafe { call_api_or_panic().jsArgType(self.inner, index) };
        Some(JsType::from_wke(js_type))
    }
    /// Create an empty array.
    pub fn empty_array(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsEmptyArray(self.inner,) })
    }
    /// Create an empty object.
    pub fn empty_object(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsEmptyObject(self.inner,) })
    }
    /// Create a string.
    pub fn string(&self, value: &str) -> JsValue {
        js_value!({
            let value = CString::safe_new(value);
            unsafe { call_api_or_panic().jsString(self.inner, value.as_ptr()) }
        })
    }
    /// Get a js value from array using index.
    pub fn get_at(&self, js_array: JsValue, index: i32) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsGetAt(self.inner, js_array.as_ptr(), index) })
    }
    /// Set a js value from array using index.
    pub fn set_at(&self, js_array: JsValue, index: i32, js_value: JsValue) {
        unsafe {
            call_api_or_panic().jsSetAt(self.inner, js_array.as_ptr(), index, js_value.as_ptr())
        }
    }
    /// Get the length of an array.
    pub fn get_length(&self, js_array: JsValue) -> i32 {
        unsafe { call_api_or_panic().jsGetLength(self.inner, js_array.as_ptr()) }
    }
    /// Set the length of an array. Can be used to extend the length of js array.
    pub fn set_length(&self, js_array: JsValue, length: i32) {
        unsafe { call_api_or_panic().jsSetLength(self.inner, js_array.as_ptr(), length) }
    }
    /// Get a value from an object
    pub fn get(&self, js_object: JsValue, prop: &str) -> JsValue {
        let prop = CString::safe_new(prop);
        js_value!({
            unsafe { call_api_or_panic().jsGet(self.inner, js_object.as_ptr(), prop.as_ptr()) }
        })
    }
    /// Set a value to an object
    pub fn set(&self, js_object: JsValue, prop: &str, value: JsValue) {
        let prop = CString::safe_new(prop);
        unsafe {
            call_api_or_panic().jsSet(
                self.inner,
                js_object.as_ptr(),
                prop.as_ptr(),
                value.as_ptr(),
            )
        }
    }
    /// Get the keys of an object.
    pub fn get_keys(&self, js_object: JsValue) -> JsKeys {
        let keys = unsafe { call_api_or_panic().jsGetKeys(self.inner, js_object.as_ptr()) };
        assert!(!keys.is_null());
        unsafe { JsKeys::from_ptr(keys) }
    }
    /// Get js value from global window object.
    pub fn get_global(&self, prop: &str) -> JsValue {
        js_value!({
            let prop = CString::safe_new(prop);
            unsafe { call_api_or_panic().jsGetGlobal(self.inner, prop.as_ptr()) }
        })
    }
    /// Get a js value to global window object.
    pub fn set_global(&self, prop: &str, value: JsValue) {
        let prop = CString::safe_new(prop);
        unsafe { call_api_or_panic().jsSetGlobal(self.inner, prop.as_ptr(), value.as_ptr()) }
    }
    /// Get webview where state belongs to.
    pub fn get_webview(&self) -> WebView {
        let webview = unsafe { call_api_or_panic().jsGetWebView(self.inner) };
        assert!(!webview.is_null());
        unsafe { WebView::from_ptr(webview) }
    }

    /// Execute a script. The script is wrapped in a `function(){}` block.
    /// 
    /// Required key word `return` if want to return a value.
    pub fn eval(&self, script: &str) -> JsValue {
        js_value!({
            let script = WkeString::new(script);
            unsafe { call_api_or_panic().jsEvalW(self.inner, script.as_wcstr_ptr()) }
        })
    }
    /// Execute a script.
    pub fn eval_ex(&self, script: &str, is_in_closure: bool) -> JsValue {
        let script = WkeString::new(script);
        js_value!(unsafe {
            call_api_or_panic().jsEvalExW(self.inner, script.as_wcstr_ptr(), is_in_closure)
        })
    }

    /// Convert a value to int.
    pub fn to_int(&self, value: JsValue) -> MBResult<i32> {
        match value.type_of() {
            JsType::Number => {
                Ok(unsafe { call_api_or_panic().jsToInt(self.inner, value.as_ptr()) })
            }
            other => Err(MBError::UnsupportedType(JsType::Number, other)),
        }
    }

    /// Convert a value to double.
    pub fn to_double(&self, value: JsValue) -> MBResult<f64> {
        match value.type_of() {
            JsType::Number => {
                Ok(unsafe { call_api_or_panic().jsToDouble(self.inner, value.as_ptr()) })
            }
            other => Err(MBError::UnsupportedType(JsType::Number, other)),
        }
    }

    /// Convert a value to boolean.
    pub fn to_boolean(&self, value: JsValue) -> MBResult<bool> {
        match value.type_of() {
            JsType::Boolean => {
                Ok(
                    (unsafe { call_api_or_panic().jsToBoolean(self.inner, value.as_ptr()) })
                        .as_bool(),
                )
            }
            other => Err(MBError::UnsupportedType(JsType::Boolean, other)),
        }
    }

    /// Convert a value to string.
    pub fn to_string(&self, value: JsValue) -> MBResult<String> {
        match value.type_of() {
            JsType::Boolean
            | JsType::Null
            | JsType::Number
            | JsType::String
            | JsType::Undefined => {
                let value = unsafe { call_api_or_panic().jsToString(self.inner, value.as_ptr()) };
                assert!(!value.is_null());
                let value = unsafe { CStr::from_ptr(value) }
                    .to_string_lossy()
                    .to_string();
                Ok(value)
            }
            other => Err(MBError::UnsupportedType(JsType::Boolean, other)),
        }
    }

    /// Get inner pointer.
    pub fn as_ptr(&self) -> jsExecState {
        self.inner
    }

    /// Create [`JsExecState`] from ptr.
    /// 
    /// Safety: The pointer must be valid
    pub unsafe fn from_ptr(ptr: jsExecState) -> Self {
        assert!(!ptr.is_null());
        Self { inner: ptr }
    }

    /// Call js function. If js function is a member, `this` is required to set.
    pub fn call(
        &self,
        func: JsValue,
        this: Option<JsValue>,
        args: &[JsValue],
        arg_count: i32,
    ) -> JsValue {
        let this = this.unwrap_or(self.undefined());
        let mut args: Box<[jsValue]> = args.iter().map(|v| v.as_ptr()).collect();
        js_value!(unsafe {
            call_api_or_panic().jsCall(
                self.inner,
                func.as_ptr(),
                this.as_ptr(),
                args.as_mut_ptr(),
                arg_count,
            )
        })
    }
    /// Call js function on window object.
    pub fn call_global(&self, func: JsValue, args: &[JsValue], arg_count: i32) -> JsValue {
        let mut args: Box<[jsValue]> = args.iter().map(|v| v.as_ptr()).collect();
        js_value!(unsafe {
            call_api_or_panic().jsCallGlobal(
                self.inner,
                func.as_ptr(),
                args.as_mut_ptr(),
                arg_count,
            )
        })
    }
    /// Force garbage collection
    pub fn gc(&self) {
        unsafe { call_api_or_panic().jsGC() }
    }
    /// Get last error if exception when calling run_js, call, at el. api.
    pub fn get_last_error_if_exception(&self) -> Option<JsExceptionInfo> {
        let error = unsafe { call_api_or_panic().jsGetLastErrorIfException(self.inner) };
        if error.is_null() {
            None
        } else {
            Some(JsExceptionInfo::from_wke(unsafe { *error }))
        }
    }
}

/// Extra api for JsExecState
pub trait JsExecStateExt {
    /// Get arg value from execution state. Helper function.
    fn arg_value<T>(&self, index: i32) -> MBResult<T>
    where
        Self: MBExecStateValue<T>;
}

impl JsExecStateExt for JsExecState {
    fn arg_value<T>(&self, index: i32) -> MBResult<T>
    where
        Self: MBExecStateValue<T>,
    {
        if index >= self.arg_count() {
            Err(MBError::ArgNotMatch(format!("arg index out of range")))
        } else {
            self.value(self.arg(index)).map_err(|e| match e {
                #[cfg(feature = "serde")]
                MBError::SerdeMessage(msg) => {
                    MBError::ArgNotMatch(format!("not match at arg index {index}, {msg}"))
                }
                MBError::UnsupportedType(expect, provided) => MBError::ArgNotMatch(format!(
                    "not match at arg index {index}, expect {expect} but {provided} provided"
                )),
                _ => MBError::ArgNotMatch(format!("not match at arg index {index}")),
            })
        }
    }
}

#[allow(missing_docs)]
pub struct JsExceptionInfo {
    pub message: String,
    pub source_line: String,
    pub script_resource_name: String,
    pub line_number: i32,
    pub start_position: i32,
    pub end_position: i32,
    pub start_column: i32,
    pub end_column: i32,
    pub callstack_string: String,
}

impl JsExceptionInfo {
    pub(crate) fn from_wke(value: jsExceptionInfo) -> Self {
        let jsExceptionInfo {
            message,
            sourceLine,
            scriptResourceName,
            lineNumber,
            startPosition,
            endPosition,
            startColumn,
            endColumn,
            callstackString,
        } = value;

        assert!(!message.is_null());
        assert!(!sourceLine.is_null());
        assert!(!scriptResourceName.is_null());
        assert!(!callstackString.is_null());

        let message = unsafe { CStr::from_ptr(message) };
        let source_line = unsafe { CStr::from_ptr(sourceLine) };
        let script_resource_name = unsafe { CStr::from_ptr(scriptResourceName) };
        let callstack_string = unsafe { CStr::from_ptr(callstackString) };

        Self {
            message: message.to_string_lossy().to_string(),
            source_line: source_line.to_string_lossy().to_string(),
            script_resource_name: script_resource_name.to_string_lossy().to_string(),
            line_number: lineNumber,
            start_position: startPosition,
            end_position: endPosition,
            start_column: startColumn,
            end_column: endColumn,
            callstack_string: callstack_string.to_string_lossy().to_string(),
        }
    }
}

/// JsKeys. Wraps to jsKeys.
pub struct JsKeys {
    inner: *mut jsKeys,
}

impl JsKeys {
    pub(crate) fn get_length(&self) -> usize {
        unsafe { (*self.inner).length as usize }
    }

    pub(crate) fn get_keys(&self) -> Vec<String> {
        let keys = unsafe { std::slice::from_raw_parts((*self.inner).keys, self.get_length()) };
        let mut vec = Vec::with_capacity(self.get_length());
        for key in keys {
            let cstr = unsafe { CStr::from_ptr(key.clone()) };
            vec.push(cstr.to_string_lossy().to_string())
        }
        vec
    }

    /// Wraps jsKeys
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_ptr(ptr: *mut jsKeys) -> Self {
        assert!(!ptr.is_null());
        Self { inner: ptr }
    }
}

// #[allow(dead_code)]
// pub(crate) struct JsArrayBuffer {
//     inner: *mut wkeMemBuf,
// }

// impl JsArrayBuffer {
//     #[allow(dead_code)]
//     pub unsafe fn from_ptr(ptr: *mut wkeMemBuf) -> Self {
//         assert!(!ptr.is_null());
//         Self { inner: ptr }
//     }
// }

/// JsValue. Wraps to `jsValue`.
#[derive(Debug, Clone, Copy)]
pub struct JsValue {
    pub(crate) inner: jsValue,
}

impl JsValue {
    /// get the type.
    pub fn type_of(&self) -> JsType {
        let js_type = unsafe { call_api_or_panic().jsTypeOf(self.inner) };
        JsType::from_wke(js_type)
    }
    /// Check if is a number.
    pub fn is_number(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsNumber(self.inner) }).as_bool()
    }
    /// Check if is a string.
    pub fn is_string(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsString(self.inner) }).as_bool()
    }
    /// Check if is a boolean.
    pub fn is_boolean(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsBoolean(self.inner) }).as_bool()
    }
    /// Check if is an object.
    pub fn is_object(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsObject(self.inner) }).as_bool()
    }
    /// Check if is a function.
    pub fn is_function(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsFunction(self.inner) }).as_bool()
    }
    /// Check if is a undefined.
    pub fn is_undefined(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsUndefined(self.inner) }).as_bool()
    }
    /// Check if is a null.
    pub fn is_null(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsNull(self.inner) }).as_bool()
    }
    /// Check if is an array.
    pub fn is_array(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsArray(self.inner) }).as_bool()
    }
    /// Check if is true.
    pub fn is_true(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsTrue(self.inner) }).as_bool()
    }
    /// Check if is false.
    pub fn is_false(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsFalse(self.inner) }).as_bool()
    }

    /// Get the inner ptr of [`JsValue`]. See [`jsValue`].
    pub fn as_ptr(&self) -> jsValue {
        self.inner
    }

    /// Create [`JsValue`] from ptr.
    /// # Safety
    /// Pointer must not be 0
    pub unsafe fn from_ptr(ptr: jsValue) -> Self {
        assert!(ptr != 0);
        Self { inner: ptr }
    }
}

/// Trait for converting between [`JsValue`] and `T`.
pub trait MBExecStateValue<T> {
    /// Convert from `T` to [`JsValue`].
    fn js_value(&self, value: T) -> MBResult<JsValue>;
    /// Convert from [`JsValue`] to `T`.
    fn value(&self, value: JsValue) -> MBResult<T>;
}

#[cfg(not(feature = "serde"))]
impl MBExecStateValue<i32> for JsExecState {
    fn js_value(&self, value: i32) -> MBResult<JsValue> {
        Ok(self.int(value))
    }

    fn value(&self, value: JsValue) -> MBResult<i32> {
        self.to_int(value)
    }
}

#[cfg(not(feature = "serde"))]
impl MBExecStateValue<f64> for JsExecState {
    fn js_value(&self, value: f64) -> MBResult<JsValue> {
        Ok(self.double(value))
    }

    fn value(&self, value: JsValue) -> MBResult<f64> {
        self.to_double(value)
    }
}

#[cfg(not(feature = "serde"))]
impl MBExecStateValue<bool> for JsExecState {
    fn js_value(&self, value: bool) -> MBResult<JsValue> {
        Ok(self.boolean(value))
    }

    fn value(&self, value: JsValue) -> MBResult<bool> {
        self.to_boolean(value)
    }
}

#[cfg(not(feature = "serde"))]
impl MBExecStateValue<String> for JsExecState {
    fn js_value(&self, value: String) -> MBResult<JsValue> {
        Ok(self.string(value.as_str()))
    }

    fn value(&self, value: JsValue) -> MBResult<String> {
        self.to_string(value)
    }
}

#[cfg(not(feature = "serde"))]
impl MBExecStateValue<()> for JsExecState {
    fn js_value(&self, _value: ()) -> MBResult<JsValue> {
        Ok(self.undefined())
    }

    fn value(&self, value: JsValue) -> MBResult<()> {
        match value.type_of_() {
            JsType::Undefined => Ok(()),
            other => Err(MBError::UnsupportedType(JsType::Undefined, other)),
        }
    }
}

#[cfg(not(feature = "serde"))]
impl<T> MBExecStateValue<Vec<T>> for JsExecState
where
    Self: MBExecStateValue<T>,
{
    fn js_value(&self, value: Vec<T>) -> MBResult<JsValue> {
        let array = self.empty_array();
        self.set_length(array, value.len() as i32);
        for (i, v) in value.into_iter().enumerate() {
            self.set_at(array, i as i32, self.js_value(v)?)
        }
        Ok(array)
    }

    fn value(&self, js_array: JsValue) -> MBResult<Vec<T>> {
        let length = self.get_length(js_array);
        let mut vec = Vec::with_capacity(length as usize);
        for (i, v) in vec.iter_mut().enumerate() {
            *v = self.value(self.get_at(js_array, i as i32))?
        }
        Ok(vec)
    }
}

#[cfg(not(feature = "serde"))]
use std::collections::HashMap;
#[cfg(not(feature = "serde"))]
impl<V> MBExecStateValue<HashMap<String, V>> for JsExecState
where
    Self: MBExecStateValue<V>,
{
    fn js_value(&self, value: HashMap<String, V>) -> MBResult<JsValue> {
        let object = self.empty_object();
        for (k, v) in value.into_iter() {
            self.set(object, k.as_str(), self.js_value(v)?);
        }
        Ok(object)
    }

    fn value(&self, js_object: JsValue) -> MBResult<HashMap<String, V>> {
        let mut map = HashMap::new();
        let keys = self.get_keys(js_object);
        for key in keys.get_keys().iter() {
            map.insert(key.to_owned(), self.value(self.get(js_object, key))?);
        }
        Ok(map)
    }
}

#[cfg(feature = "serde")]
impl<T> MBExecStateValue<T> for JsExecState
where
    T: for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    fn js_value(&self, value: T) -> MBResult<JsValue> {
        crate::serde::to_value(*self, &value)
    }

    fn value(&self, value: JsValue) -> MBResult<T> {
        crate::serde::from_value(*self, value)
    }
}

/// Navigation Type.
#[allow(missing_docs)]
pub enum NavigationType {
    /// Click on <a>
    LinkClick,
    /// On <form> submit
    FormSubmitte,
    BackForward,
    Reload,
    FormResubmit,
    Other,
}

impl NavigationType {
    pub(crate) fn from_wke(value: wkeNavigationType) -> Self {
        match value {
            wkeNavigationType::WKE_NAVIGATION_TYPE_LINKCLICK => Self::LinkClick,
            wkeNavigationType::WKE_NAVIGATION_TYPE_FORMRESUBMITT => Self::FormSubmitte,
            wkeNavigationType::WKE_NAVIGATION_TYPE_BACKFORWARD => Self::BackForward,
            wkeNavigationType::WKE_NAVIGATION_TYPE_RELOAD => Self::Reload,
            wkeNavigationType::WKE_NAVIGATION_TYPE_FORMSUBMITTE => Self::FormResubmit,
            _ => Self::Other,
        }
    }
}

/// Window Type.
#[allow(missing_docs)]
pub enum WindowType {
    /// Popup type
    Popup,
    /// Transparent type. Achieved using layer window.    
    Transparent,
    /// Control type. Create window as child window. Requied parent.
    Control,
}

impl From<WindowType> for wkeWindowType {
    fn from(value: WindowType) -> Self {
        match value {
            WindowType::Control => wkeWindowType::WKE_WINDOW_TYPE_CONTROL,
            WindowType::Popup => wkeWindowType::WKE_WINDOW_TYPE_POPUP,
            WindowType::Transparent => wkeWindowType::WKE_WINDOW_TYPE_TRANSPARENT,
        }
    }
}

/// MenuItemID
#[allow(missing_docs)]
pub enum MenuItemId {
    MenuSelectedAllId,
    MenuSelectedTextId,
    MenuUndoId,
    MenuCopyImageId,
    MenuInspectElementAtId,
    MenuCutId,
    MenuPasteId,
    MenuPrintId,
    MenuGoForwardId,
    MenuGoBackId,
    MenuReloadId,
    MenuSaveImageId,
}

impl MenuItemId {
    pub(crate) fn to_wke(&self) -> wkeMenuItemId {
        match self {
            Self::MenuSelectedAllId => wkeMenuItemId::kWkeMenuSelectedAllId,
            Self::MenuSelectedTextId => wkeMenuItemId::kWkeMenuSelectedTextId,
            Self::MenuUndoId => wkeMenuItemId::kWkeMenuUndoId,
            Self::MenuCopyImageId => wkeMenuItemId::kWkeMenuCopyImageId,
            Self::MenuInspectElementAtId => wkeMenuItemId::kWkeMenuInspectElementAtId,
            Self::MenuCutId => wkeMenuItemId::kWkeMenuCutId,
            Self::MenuPasteId => wkeMenuItemId::kWkeMenuPasteId,
            Self::MenuPrintId => wkeMenuItemId::kWkeMenuPrintId,
            Self::MenuGoForwardId => wkeMenuItemId::kWkeMenuGoForwardId,
            Self::MenuGoBackId => wkeMenuItemId::kWkeMenuGoBackId,
            Self::MenuReloadId => wkeMenuItemId::kWkeMenuReloadId,
            Self::MenuSaveImageId => wkeMenuItemId::kWkeMenuSaveImageId,
        }
    }
}

/// View Settings.
pub struct ViewSettings {
    /// TBD
    pub size: i32,
    /// Background color
    pub backgroud_color: u32,
}

/// WebFrameHandle
pub struct WebFrameHandle {
    frame: wkeWebFrameHandle,
}

impl WebFrameHandle {
    /// Create from wkeWebFrameHandle to WebFrameHandle
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_ptr(ptr: wkeWebFrameHandle) -> Self {
        assert!(!ptr.is_null());
        Self { frame: ptr }
    }

    /// Get the inner wkeWebFrameHandle ptr.
    pub fn as_ptr(&self) -> wkeWebFrameHandle {
        self.frame
    }
}

#[allow(missing_docs)]
/// A rect type with x, y, width, height params.
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    /// Create a rect type
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug)]
/// Raw wraps to wkeString. See `wkeString`.
#[repr(transparent)]
pub struct WkeStr {
    inner: wkeString,
}

impl WkeStr {
    /// Wraps a wkeString from pointer.
    /// # Safety
    /// The pointer must be valid.
    pub unsafe fn from_ptr(ptr: wkeString) -> Self {
        assert!(!ptr.is_null());
        Self { inner: ptr }
    }

    pub(crate) fn as_ptr(&self) -> wkeString {
        self.inner
    }

    pub(crate) fn to_string(&self) -> String {
        let cstr = unsafe {
            let ptr = call_api_or_panic().wkeGetString(self.inner);
            CStr::from_ptr(ptr)
        };
        cstr.to_string_lossy().to_string()
    }

    /// Call wkeGetStringW to get a *const wchar_t pointer.
    pub fn as_wcstr_ptr(&self) -> *const wchar_t {
        unsafe { call_api_or_panic().wkeGetStringW(self.inner) }
    }

    /// Call wkeGetString to get a *const c_char pointer.
    pub fn as_cstr_ptr(&self) -> *const c_char {
        unsafe { call_api_or_panic().wkeGetString(self.inner) }
    }
}

/// Wraps to wkeString. Auto drop the inner wkeString.
pub struct WkeString {
    inner: wkeString,
}

impl Drop for WkeString {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe { call_api_or_panic().wkeDeleteString(self.inner) }
        }
    }
}

impl WkeString {
    /// Create a wkeString. See `wkeCreateString`.
    pub fn new(s: &str) -> Self {
        let cstring = CString::safe_new(s);
        let inner = unsafe {
            call_api_or_panic().wkeCreateString(cstring.as_ptr(), cstring.as_bytes().len())
        };
        Self { inner }
    }

    /// Consumes the WkeString and transfers ownership to a C caller.
    pub fn into_raw(self) -> wkeString {
        let ptr = self.inner;
        let _ = ManuallyDrop::new(self);
        ptr
    }

    /// Retakes ownership of a WkeString that was transferred to C via WkeString::into_raw.
    pub unsafe fn from_raw(ptr: wkeString) -> Self {
        Self { inner: ptr }
    }
}

impl std::ops::Deref for WkeString {
    type Target = WkeStr;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.inner as *const wkeString as *const WkeStr) }
    }
}

/// Defines cookie commands performed using inner curl.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CookieCommand {
    /// Execute `curl_easy_setopt(curl, CURLOPT_COOKIELIST, "ALL");`
    ClearAllCookies,
    /// Execute `curl_easy_setopt(curl, CURLOPT_COOKIELIST, "SESS");`
    ClearSessionCookies,
    /// Execute `curl_easy_setopt(curl, CURLOPT_COOKIELIST, "FLUSH");`
    FlushCookiesToFile,
    /// Execute `curl_easy_setopt(curl, CURLOPT_COOKIELIST, "RELOAD");`
    ReloadCookiesFromFile,
}

impl CookieCommand {
    pub(crate) fn into_wke(self) -> wkeCookieCommand {
        match self {
            Self::ClearAllCookies => wkeCookieCommand::wkeCookieCommandClearAllCookies,
            Self::ClearSessionCookies => wkeCookieCommand::wkeCookieCommandClearSessionCookies,
            Self::FlushCookiesToFile => wkeCookieCommand::wkeCookieCommandFlushCookiesToFile,
            Self::ReloadCookiesFromFile => wkeCookieCommand::wkeCookieCommandReloadCookiesFromFile,
        }
    }
}

#[allow(missing_docs)]
/// Wraps wkeSettings.
pub struct Settings {
    pub proxy: Proxy,
    pub mask: u32,
    pub extension: CString,
}

impl Settings {
    pub(crate) fn to_wke(&self) -> wkeSettings {
        let Settings {
            proxy,
            mask,
            extension,
        } = self;
        wkeSettings {
            proxy: proxy.to_wke(),
            mask: *mask,
            extension: extension.as_ptr(),
        }
    }

    /// Creates settings.
    pub fn new(proxy: Proxy, mask: u32, extension: &str) -> Self {
        Self {
            proxy,
            mask,
            extension: CString::safe_new(extension),
        }
    }
}

#[allow(missing_docs)]
/// Cookie vistor, make it easier to use in callbacks
pub struct CookieVisitor {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: i32,
    pub http_only: i32,
    pub expires: *mut i32,
}

impl CookieVisitor {
    pub(crate) fn from_wke(
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
        domain: *const ::std::os::raw::c_char,
        path: *const ::std::os::raw::c_char,
        secure: i32,
        http_only: i32,
        expires: *mut i32,
    ) -> Self {
        assert!(!name.is_null());
        assert!(!value.is_null());
        assert!(!domain.is_null());
        assert!(!path.is_null());
        assert!(!expires.is_null());

        let name = unsafe { CStr::from_ptr(name) };
        let value = unsafe { CStr::from_ptr(value) };
        let domain = unsafe { CStr::from_ptr(domain) };
        let path = unsafe { CStr::from_ptr(path) };

        Self {
            name: name.to_string_lossy().to_string(),
            value: value.to_string_lossy().to_string(),
            domain: domain.to_string_lossy().to_string(),
            path: path.to_string_lossy().to_string(),
            secure,
            http_only,
            expires,
        }
    }

    /// Set the cookie expires.
    pub fn set_expires(&self, expires: i32) {
        assert!(!self.expires.is_null());
        unsafe { *self.expires = expires }
    }
}

#[allow(missing_docs)]
pub struct WindowFeatures {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub menu_bar_visible: bool,
    pub status_bar_visible: bool,
    pub tool_bar_visible: bool,
    pub location_bar_visible: bool,
    pub scrollbars_visible: bool,
    pub resizable: bool,
    pub fullscreen: bool,
}

impl WindowFeatures {
    pub(crate) fn from_wke(value: wkeWindowFeatures) -> Self {
        let wkeWindowFeatures {
            x,
            y,
            width,
            height,
            menuBarVisible: menu_bar_visible,
            statusBarVisible: status_bar_visible,
            toolBarVisible: tool_bar_visible,
            locationBarVisible: location_bar_visible,
            scrollbarsVisible: scrollbars_visible,
            resizable,
            fullscreen,
        } = value;

        WindowFeatures {
            x,
            y,
            width,
            height,
            menu_bar_visible,
            status_bar_visible,
            tool_bar_visible,
            location_bar_visible,
            scrollbars_visible,
            resizable,
            fullscreen,
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsoleLevel {
    Debug,
    Log,
    Info,
    Warning,
    Error,
    RevokedError,
    // Last,
}

impl ConsoleLevel {
    pub(crate) fn from_wke(value: wkeConsoleLevel) -> Self {
        match value {
            wkeConsoleLevel::wkeLevelDebug => Self::Debug,
            wkeConsoleLevel::wkeLevelLog => Self::Log,
            wkeConsoleLevel::wkeLevelInfo => Self::Info,
            wkeConsoleLevel::wkeLevelWarning => Self::Warning,
            wkeConsoleLevel::wkeLevelError => Self::Error,
            wkeConsoleLevel::wkeLevelRevokedError => Self::RevokedError,
            // wkeConsoleLevel::wkeLevelLast => Self::Last,
            _ => unimplemented!(),
        }
    }
}

#[allow(missing_docs)]
pub struct MediaLoadInfo {
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub duration: f64,
}

impl MediaLoadInfo {
    pub(crate) fn from_wke(value: wkeMediaLoadInfo) -> Self {
        let wkeMediaLoadInfo {
            size,
            width,
            height,
            duration,
        } = value;

        Self {
            size,
            width,
            height,
            duration,
        }
    }
}

#[allow(missing_docs)]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseFlags {
    LBUTTON = wkeMouseFlags::WKE_LBUTTON.0 as u32,
    RBUTTON = wkeMouseFlags::WKE_RBUTTON.0 as u32,
    SHIFT = wkeMouseFlags::WKE_SHIFT.0 as u32,
    CONTROL = wkeMouseFlags::WKE_CONTROL.0 as u32,
    MBUTTON = wkeMouseFlags::WKE_MBUTTON.0 as u32,
}

impl From<MouseFlags> for u32 {
    fn from(value: MouseFlags) -> Self {
        value as u32
    }
}

impl std::ops::BitOr for MouseFlags {
    type Output = u32;
    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u32) | (rhs as u32)
    }
}

#[allow(missing_docs)]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseMessage {
    MOUSEMOVE = wkeMouseMsg::WKE_MSG_MOUSEMOVE.0 as u32,
    LBUTTONDOWN = wkeMouseMsg::WKE_MSG_LBUTTONDOWN.0 as u32,
    LBUTTONUP = wkeMouseMsg::WKE_MSG_LBUTTONUP.0 as u32,
    LBUTTONDBLCLK = wkeMouseMsg::WKE_MSG_LBUTTONDBLCLK.0 as u32,
    RBUTTONDOWN = wkeMouseMsg::WKE_MSG_RBUTTONDOWN.0 as u32,
    RBUTTONUP = wkeMouseMsg::WKE_MSG_RBUTTONUP.0 as u32,
    RBUTTONDBLCLK = wkeMouseMsg::WKE_MSG_RBUTTONDBLCLK.0 as u32,
    MBUTTONDOWN = wkeMouseMsg::WKE_MSG_MBUTTONDOWN.0 as u32,
    MBUTTONUP = wkeMouseMsg::WKE_MSG_MBUTTONUP.0 as u32,
    MBUTTONDBLCLK = wkeMouseMsg::WKE_MSG_MBUTTONDBLCLK.0 as u32,
    MOUSEWHEEL = wkeMouseMsg::WKE_MSG_MOUSEWHEEL.0 as u32,
}

impl From<MouseMessage> for u32 {
    fn from(value: MouseMessage) -> Self {
        value as u32
    }
}

#[allow(missing_docs)]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyFlags {
    EXTENDED = wkeKeyFlags::WKE_EXTENDED.0 as u32,
    REPEAT = wkeKeyFlags::WKE_REPEAT.0 as u32,
}

impl From<KeyFlags> for u32 {
    fn from(value: KeyFlags) -> Self {
        value as u32
    }
}

mod tests {

    #[test]
    fn test_wkestring() {
        use super::WkeString;
        use crate::app;
        app::initialize("node.dll").unwrap();
        let wke_string = WkeString::new("Hello");
        let wke_str = wke_string.to_string();
        assert_eq!(wke_str, "Hello");
    }

    #[test]
    fn test_slist() {
        use super::*;

        let mut slist2 = wkeSlist {
            data: c"You".as_ptr() as *mut i8,
            next: std::ptr::null_mut() as _,
        };
        let slist1 = wkeSlist {
            data: c"Are".as_ptr() as *mut i8,
            next: std::ptr::from_mut(&mut slist2),
        };

        let slist = unsafe { Slist::from_ptr(&slist1) };

        let mut slist_iter = slist.into_iter();
        assert_eq!(slist_iter.next(), Some("Are".into()));
        assert_eq!(slist_iter.next(), Some("You".into()));
    }
}
