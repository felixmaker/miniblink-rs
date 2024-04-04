use std::ffi::CStr;

use miniblink_sys::{wkeNetJob, wkeRequestType, wkeSlist};

/// See wkeNetJob.
#[repr(transparent)]
pub struct NetJob {
    inner: wkeNetJob,
}

// impl NetJob {
//     bind_target! {
//         pub wkeNetSetHTTPHeaderField => set_http_header_field(key: &str as WkeString, value: &str as WkeString, response: bool);
//         // pub wkeNetGetRawHttpHead => get_raw_http_head() -> &Slist;
//         pub wkeNetSetMIMEType => set_mime_type(mine_type: &str as CString);
//         // pub wkeNetSetData => set_data()
//         // pub wkeNetGetMIMEType => get_mime_type(mime: Option<&str> as Option<WkeString>) -> String;
//         pub wkeNetCancelRequest => cancel_request();
//         pub wkeNetHoldJobToAsynCommit => hold_job_to_asyn_commit() -> bool;
//         pub wkeNetGetRequestMethod => get_request_method() -> RequestType;
//         // pub wkeNetGetPostBody => get_post_body()

//     }
// }

impl NetJob {
    /// #Safety
    /// Pointer should be valid.
    pub unsafe fn from_ptr(ptr: wkeNetJob) -> Self {
        Self { inner: ptr }
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