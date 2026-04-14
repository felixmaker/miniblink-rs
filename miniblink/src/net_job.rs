use std::ffi::{CStr, CString};

use miniblink_sys::mbNetJob;
use widestring::WideCString;

use crate::call_api_or_panic;

/// Wraps to mbNetJob.
///
/// Used in callbacks.
#[repr(transparent)]
pub struct NetJob {
    pub(crate) inner: mbNetJob,
}

impl NetJob {
    /// Set the http header field.
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

    /// Get the raw http head.
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

    /// Set the mime type.
    pub fn set_mime_type(&self, mime_type: &str) {
        let mime_type = CString::new(mime_type).unwrap();
        unsafe {
            call_api_or_panic().mbNetSetMIMEType(self.inner, mime_type.as_ptr());
        }
    }

    /// Get the mime type.
    pub fn get_mime_type(&self) -> String {
        let mime_type_ptr = unsafe { call_api_or_panic().mbNetGetMIMEType(self.inner) };
        unsafe { CStr::from_ptr(mime_type_ptr).to_string_lossy().to_string() }
    }
}
