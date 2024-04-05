use std::ffi::CStr;

use miniblink_sys::{wkeNetJob, wkeRequestType, wkeSlist};

use crate::call_api_or_panic;

use super::PostBodyElements;

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
        unsafe { PostBodyElements::from_ptr( elements ) }
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn slist() {
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
