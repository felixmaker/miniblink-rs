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
    inner: *mut wkeWillSendRequestInfo,
}

impl WillSendRequestInfo {
    pub(crate) unsafe fn from_ptr(ptr: *mut wkeWillSendRequestInfo) -> Self {
        // macro_rules! wke_string {
        //     ($expr: expr) => {{
        //         let ptr = $expr;
        //         assert!(!ptr.is_null());
        //         unsafe { WkeStr::from_ptr(ptr) }.to_string()
        //     }};
        // }
        // Self {
        //     url: wke_string!(wke.url),
        //     new_url: wke_string!(wke.newUrl),
        //     resource_type: ResourceType::from_wke(wke.resourceType),
        //     http_response_code: wke.httpResponseCode,
        //     method: wke_string!(wke.method),
        //     referrer: wke_string!(wke.referrer),
        // }
        Self { inner: ptr }
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
    pub(crate) fn from_wke(wke: wkeHttBodyElementType) -> Self {
        match wke {
            wkeHttBodyElementType::wkeHttBodyElementTypeData => Self::Data,
            wkeHttBodyElementType::wkeHttBodyElementTypeFile => Self::File,
            _ => unimplemented!(),
        }
    }
}

#[allow(missing_docs)]
pub struct MemBuf {
    inner: *mut wkeMemBuf,
}

impl MemBuf {
    pub(crate) unsafe fn from_ptr(ptr: *mut wkeMemBuf) -> Self {
        Self { inner: ptr }
    }
}

/// See wkeNetJob.
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
    /// 在wkeOnLoadUrlBegin回调里调用，表示设置http请求（或者file:///协议）的 http header field。response一直要被设置成false
    pub fn set_http_header_field(key: &str, value: &str, response: bool) {
        todo!()
    }
    /// 在wkeOnLoadUrlBegin回调里调用，获取curl返回的原生请求头
    ///返回值：const wkeSlist*，是一个C语言链表，详情看头文件
    pub fn get_raw_http_head() -> Slist {
        todo!()
    }
    /// 在wkeOnLoadUrlBegin回调里调用，表示设置http请求的MIME type
    pub fn set_mime_type(mine_type: &str) {
        todo!()
    }
    /// 在wkeOnLoadUrlEnd里被调用，表示设置hook后缓存的数据
    pub fn set_data() {
        todo!()
    }
    /// 参数：第2个参数可以传nullptr
    pub fn get_mime_type(mime: Option<&str>) -> String {
        todo!()
    }
    /// 在wkeOnLoadUrlBegin回调里调用，设置后，此请求将被取消。
    pub fn cancel_request() {
        todo!()
    }
    /// 高级用法。在wkeOnLoadUrlBegin回调里调用。 有时候，wkeOnLoadUrlBegin里拦截到一个请求后，不能马上判断出结果。此时可以调用本接口，然后在 异步的某个时刻，调用wkeNetContinueJob来让此请求继续进行
    ///参数：略
    ///返回值：TRUE代表成功，FALSE代表调用失败，不能再调用wkeNetContinueJob了
    pub fn hold_job_to_asyn_commit() -> bool {
        todo!()
    }
    /// 获取此请求的method，如post还是get
    pub fn get_request_method() {
        todo!()
    }
    /// Get the post body.
    pub fn get_post_body(&self) -> PostBodyElements {
        let elements = unsafe { call_api_or_panic().wkeNetGetPostBody(self.inner) };
        assert!(!elements.is_null());
        unsafe { PostBodyElements::from_ptr(elements) }
    }
}

enum RequestType {
    Invalidation,
    Get,
    Post,
    Put,
}

impl From<wkeRequestType> for RequestType {
    fn from(value: wkeRequestType) -> Self {
        match value {
            wkeRequestType::kWkeRequestTypeInvalidation => Self::Invalidation,
            wkeRequestType::kWkeRequestTypeGet => Self::Get,
            wkeRequestType::kWkeRequestTypePost => Self::Post,
            wkeRequestType::kWkeRequestTypePut => Self::Put,
            _ => unimplemented!(),
        }
    }
}

/// See wkeSlist.
#[repr(transparent)]
pub struct Slist {
    inner: wkeSlist,
}

#[allow(missing_docs)]
pub struct SlintIter {
    current: *mut wkeSlist,
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

    fn into_iter(mut self) -> Self::IntoIter {
        SlintIter {
            current: &mut self.inner as *mut wkeSlist,
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

#[allow(missing_docs)]
/// see `wkeProxy`.
pub struct Proxy {
    pub type_: ProxyType,
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Proxy {
    pub(crate) fn to_wke_proxy(&self) -> wkeProxy {
        wkeProxy {
            type_: self.type_.to_wke_proxy_type(),
            hostname: string_to_slice(&self.hostname),
            port: self.port,
            username: string_to_slice(&self.username),
            password: string_to_slice(&self.password),
        }
    }
}

/// See [`jsType`].
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

impl From<jsType> for JsType {
    fn from(value: jsType) -> Self {
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

/// See `jsExecState`.
#[derive(Clone, Copy)]
pub struct JsExecState {
    pub(crate) inner: jsExecState,
}

impl JsExecState {
    /// See jsInt.
    pub fn int(&self, value: i32) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsInt(value) })
    }
    /// See jsDouble.
    pub fn double(&self, value: f64) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsDouble(value) })
    }
    /// See jsBoolean.
    pub fn boolean(&self, value: bool) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsBoolean(value) })
    }
    /// See jsUndefined.
    pub fn undefined(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsUndefined() })
    }
    /// See jsNull.
    pub fn null(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsNull() })
    }
    /// 获取第argIdx对应的参数的jsValue值。
    pub fn arg(&self, index: i32) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsArg(self.inner, index) })
    }
    /// See jsArgCount.
    pub fn arg_count(&self) -> i32 {
        unsafe { call_api_or_panic().jsArgCount(self.inner) }
    }
    /// 判断第argIdx个参数的参数类型。argIdx从是个0开始计数的值。如果超出jsArgCount返回的值，将发生崩溃
    pub fn arg_type() {
        todo!()
    }
    /// See jsEmptyArray.
    pub fn empty_array(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsEmptyArray(self.inner,) })
    }
    /// See jsEmptyObject.
    pub fn empty_object(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsEmptyObject(self.inner,) })
    }
    /// See jsString.
    pub fn string(&self, value: &str) -> JsValue {
        js_value!({
            let value = CString::safe_new(value);
            unsafe { call_api_or_panic().jsString(self.inner, value.as_ptr()) }
        })
    }
    ///
    pub fn array_buffer() {
        todo!()
    }
    ///
    pub fn get_array_buffer() {
        todo!()
    }
    /// See jsGetAt.
    pub fn get_at(&self, js_array: JsValue, index: i32) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsGetAt(self.inner, js_array.as_ptr(), index) })
    }
    /// See jsSetAt.
    pub fn set_at(&self, js_array: JsValue, index: i32, js_value: JsValue) {
        unsafe {
            call_api_or_panic().jsSetAt(self.inner, js_array.as_ptr(), index, js_value.as_ptr())
        }
    }
    /// See jsGetLength.
    pub fn get_length(&self, js_array: JsValue) -> i32 {
        unsafe { call_api_or_panic().jsGetLength(self.inner, js_array.as_ptr()) }
    }
    /// See jsSetLength.
    pub fn set_length(&self, js_array: JsValue, length: i32) {
        unsafe { call_api_or_panic().jsSetLength(self.inner, js_array.as_ptr(), length) }
    }
    /// See jsGet.
    pub fn get(&self, js_object: JsValue, prop: &str) -> JsValue {
        let prop = CString::safe_new(prop);
        js_value!({
            unsafe { call_api_or_panic().jsGet(self.inner, js_object.as_ptr(), prop.as_ptr()) }
        })
    }
    /// See jsSet.
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
    /// See jsGetKeys.
    pub fn get_keys(&self, js_object: JsValue) -> JsKeys {
        let keys = unsafe { call_api_or_panic().jsGetKeys(self.inner, js_object.as_ptr()) };
        assert!(!keys.is_null());
        unsafe { JsKeys::from_ptr(keys) }
    }
    /// 获取window上的属性
    pub fn get_global(&self, prop: &str) -> JsValue {
        js_value!({
            let prop = CString::safe_new(prop);
            unsafe { call_api_or_panic().jsGetGlobal(self.inner, prop.as_ptr()) }
        })
    }
    /// 设置window上的属性
    pub fn set_global(&self, prop: &str, value: JsValue) {
        let prop = CString::safe_new(prop);
        unsafe { call_api_or_panic().jsSetGlobal(self.inner, prop.as_ptr(), value.as_ptr()) }
    }
    /// 获取es对应的webview
    pub fn get_webview(&self) -> WebView {
        let webview = unsafe { call_api_or_panic().jsGetWebView(self.inner) };
        assert!(!webview.is_null());
        unsafe { WebView::from_ptr(webview) }
    }

    /// 执行一段js，并返回值。
    ///参数：略
    ///注意：str的代码会在mb内部自动被包裹在一个function(){}中。所以使用的变量会被隔离 注意：要获取返回值，请写return。这和wke不太一样。wke不需要写retrun
    pub fn eval(&self, script: &str) -> JsValue {
        js_value!({
            let script = WkeString::new(script);
            unsafe { call_api_or_panic().jsEvalW(self.inner, script.as_wcstr_ptr()) }
        })
    }
    /// See jsEvalExW.
    pub fn eval_ex(&self, script: &str, is_in_closure: bool) -> JsValue {
        let script = WkeString::new(script);
        js_value!(unsafe {
            call_api_or_panic().jsEvalExW(self.inner, script.as_wcstr_ptr(), is_in_closure)
        })
    }

    /// See `jsToInt`.
    pub fn to_int(&self, value: JsValue) -> MBResult<i32> {
        match value.type_of_() {
            JsType::Number => {
                Ok(unsafe { call_api_or_panic().jsToInt(self.inner, value.as_ptr()) })
            }
            other => Err(MBError::UnsupportedType(JsType::Number, other)),
        }
    }

    /// See `jsToDouble`.
    pub fn to_double(&self, value: JsValue) -> MBResult<f64> {
        match value.type_of_() {
            JsType::Number => {
                Ok(unsafe { call_api_or_panic().jsToDouble(self.inner, value.as_ptr()) })
            }
            other => Err(MBError::UnsupportedType(JsType::Number, other)),
        }
    }

    /// See `jsToBoolean`.
    pub fn to_boolean(&self, value: JsValue) -> MBResult<bool> {
        match value.type_of_() {
            JsType::Boolean => {
                Ok(unsafe { call_api_or_panic().jsToBoolean(self.inner, value.as_ptr()) != 0 })
            }
            other => Err(MBError::UnsupportedType(JsType::Boolean, other)),
        }
    }

    /// See `jsToTempString`.
    pub fn to_string(&self, value: JsValue) -> MBResult<String> {
        match value.type_of_() {
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

    /// Get inner ptr of [`JsExecState`]. See [`jsExecState`].
    pub fn as_ptr(&self) -> jsExecState {
        self.inner
    }

    /// Create [`JsExecState`] from ptr.
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_ptr(ptr: jsExecState) -> Self {
        assert!(!ptr.is_null());
        Self { inner: ptr }
    }

    /// 调用一个func对应的js函数。如果此js函数是成员函数，则需要填thisValue。 否则可以传jsUndefined。args是个数组，个数由argCount控制。 func可以是从js里取的，也可以是自行构造的。
    pub fn call() {
        todo!()
    }
    /// 调用window上的全局函数
    pub fn call_global() {
        todo!()
    }
    /// 强制垃圾回收
    pub fn gc() {
        todo!()
    }
    /// 创建一个主frame的全局函数。jsData的用法如上。js调用：XXX() 此时jsData的callAsFunction触发。 其实jsFunction和jsObject功能基本类似。且jsObject的功能更强大一些
    pub fn function() {
        todo!()
    }
    ///获取jsObject或jsFunction创建的jsValue对应的jsData指针。
    pub fn get_data() {
        todo!()
    }
    /// 当wkeRunJs、jsCall等接口调用时，如果执行的js代码有异常，此接口将获取到异常信息。否则返回nullptr。
    pub fn get_last_error_if_exception() {
        todo!()
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

/// See jsKeys.
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

/// See `jsValue`.
#[derive(Debug, Clone, Copy)]
pub struct JsValue {
    pub(crate) inner: jsValue,
}

impl JsValue {
    /// See jsTypeOf.
    pub fn type_of_(&self) -> JsType {
        let js_type = unsafe { call_api_or_panic().jsTypeOf(self.inner) };
        JsType::from(js_type)
    }
    /// See jsIsNumber.
    pub fn is_number(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsNumber(self.inner) } != 0)
    }
    /// See jsIsString.
    pub fn is_string(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsString(self.inner) } != 0)
    }
    /// See jsIsBoolean.
    pub fn is_boolean(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsBoolean(self.inner) } != 0)
    }
    /// See jsIsObject.
    pub fn is_object(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsObject(self.inner) } != 0)
    }
    /// See jsIsFunction.
    pub fn is_function(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsFunction(self.inner) } != 0)
    }
    /// See jsIsUndefined.
    pub fn is_undefined(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsUndefined(self.inner) } != 0)
    }
    /// See jsIsNull.
    pub fn is_null(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsNull(self.inner) } != 0)
    }
    /// See jsIsArray.
    pub fn is_array(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsArray(self.inner) } != 0)
    }
    /// See jsIsTrue.
    pub fn is_true(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsTrue(self.inner) } != 0)
    }
    /// See jsIsFalse.
    pub fn is_false(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsFalse(self.inner) } != 0)
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

/// Navigation Type. See `wkeNavigationType`.
#[allow(missing_docs)]
pub enum NavigationType {
    /// 点击a标签触发
    LinkClick,
    /// 点击form触发
    FormSubmitte,
    /// 前进后退触发
    BackForward,
    /// 重新加载触发
    Reload,
    FormResubmit,
    Other,
}

impl From<wkeNavigationType> for NavigationType {
    fn from(value: wkeNavigationType) -> Self {
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

/// Navigation Type. See `wkeWindowType`.
#[allow(missing_docs)]
pub enum WindowType {
    /// 普通窗口
    Control,
    /// 透明窗口。mb内部通过layer window实现
    Popup,
    /// 嵌入在父窗口里的子窗口。此时parent需要被设置
    Transparent,
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

#[repr(transparent)]
/// Represent a Windows HWND
pub struct Handle(pub isize);

impl Handle {
    /// Handle NULL
    pub fn null() -> Self {
        Handle(0)
    }
}

impl From<HWND> for Handle {
    fn from(value: HWND) -> Self {
        Self(value as _)
    }
}

impl From<Handle> for HWND {
    fn from(value: Handle) -> Self {
        value.0 as _
    }
}

/// See `wkeMenuItemId`
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

/// see `wkeViewSettings`
#[allow(missing_docs)]
pub struct ViewSettings {
    pub size: i32,
    pub backgroud_color: u32,
}

/// see `POINT`
#[allow(missing_docs)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(transparent)]
/// see `wkeWebFrameHandle`
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

    pub(crate) fn to_string(&self) -> String {
        let cstr = unsafe {
            let ptr = call_api_or_panic().wkeGetString(self.inner);
            CStr::from_ptr(ptr)
        };
        cstr.to_string_lossy().to_string()
    }

    /// See `wkeGetStringW`.
    pub fn as_wcstr_ptr(&self) -> *const wchar_t {
        unsafe { call_api_or_panic().wkeGetStringW(self.inner) }
    }

    /// See `wkeGetString`.
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

        let slist = Slist { inner: slist1 };

        let mut slist_iter = slist.into_iter();
        assert_eq!(slist_iter.next(), Some("Are".into()));
        assert_eq!(slist_iter.next(), Some("You".into()));
    }
}
