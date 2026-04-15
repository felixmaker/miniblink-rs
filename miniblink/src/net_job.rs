use std::ffi::{CStr, CString};

use miniblink_sys::mbNetJob;
use widestring::WideCString;

use crate::{call_api_or_panic, types::RequestType};

/// Wraps to mbNetJob.
///
/// Used in callbacks.
#[repr(transparent)]
pub struct NetJob {
    pub(crate) inner: mbNetJob,
}

impl NetJob {
    /// Set the http header field. Called in the on_load_url_begin callback.
    pub fn set_http_header_field(&self, name: &str, value: &str) {
        let name = WideCString::from_str(name).unwrap();
        let value = WideCString::from_str(value).unwrap();
        unsafe {
            call_api_or_panic().mbNetSetHTTPHeaderField(
                self.inner,
                name.as_ptr(),
                value.as_ptr(),
                0,
            )
        };
    }

    /// Get the raw http head. Called in the on_load_url_begin callback.
    pub fn get_raw_http_head(&self) -> Vec<String> {
        let mut item_ptr =
            unsafe { call_api_or_panic().mbNetGetRawHttpHeadInBlinkThread(self.inner) };
        let mut vec = Vec::new();
        while item_ptr != std::ptr::null_mut() {
            let item = unsafe { *item_ptr };
            let head = unsafe { CStr::from_ptr(item.data).to_string_lossy().to_string() };
            vec.push(head);
            item_ptr = item.next;
        }
        vec
    }

    /// Set the mime type. Called in the on_load_url_begin callback.
    pub fn set_mime_type(&self, mime_type: &str) {
        let mime_type = CString::new(mime_type).unwrap();
        unsafe {
            call_api_or_panic().mbNetSetMIMEType(self.inner, mime_type.as_ptr());
        }
    }

    /// Get the mime type. Called in the on_load_url_begin callback.
    pub fn get_mime_type(&self) -> String {
        let mime_type_ptr = unsafe { call_api_or_panic().mbNetGetMIMEType(self.inner) };
        unsafe { CStr::from_ptr(mime_type_ptr).to_string_lossy().to_string() }
    }

    /// Get the request method.
    pub fn get_request_method(&self) -> RequestType {
        let method_ptr = unsafe { call_api_or_panic().mbNetGetRequestMethod(self.inner) };
        unsafe { std::mem::transmute(method_ptr) }
    }

    /// Set the request data. Called in the on_load_url_end callback.
    pub fn set_request_data<I>(&self, data: I) where I: Into<Vec<u8>> {
        let mut data = data.into();
        data.shrink_to_fit();
        let mut data = std::mem::ManuallyDrop::new(data);
        let data_len = data.len();
        let data = data.as_mut_ptr();
        
        unsafe {
            call_api_or_panic().mbNetSetData(self.inner, data as _, data_len as i32);
        }
    }

    /// Hook the request. The on_load_url_begin callback will be called if and only if the request is hooked.
    pub fn hook_request(&self) {
        unsafe {
            call_api_or_panic().mbNetHookRequest(self.inner);
        }
    }
}