use std::ffi::CStr;

use miniblink_sys::{
    wkeHttBodyElementType, wkeMemBuf, wkePostBodyElement, wkePostBodyElements, wkeResourceType,
    wkeTempCallbackInfo, wkeWillSendRequestInfo,
};

use crate::{call_api_or_panic, webview::WebView};

use super::{NetJob, WebFrameHandle};

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
