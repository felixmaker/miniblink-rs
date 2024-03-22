use std::ffi::{CStr, CString};

use crate::{call_api_or_panic, util::SafeCString};

/// A type used in miniblink. See jsExecState.
#[derive(Clone, Copy)]
pub struct JsExecState {
    pub(crate) inner: *mut std::os::raw::c_void,
}

impl JsExecState {
    pub fn arg(&self, index: i32) -> JsValue {
        JsValue {
            inner: unsafe { call_api_or_panic().jsArg(self.inner, index) },
        }
    }

    pub fn arg_count(&self) -> i32 {
        unsafe { call_api_or_panic().jsArgCount(self.inner) }
    }
}

/// A type used in miniblink. See jsValue.
#[derive(Clone, Copy)]
pub struct JsValue {
    pub(crate) inner: miniblink_sys::jsValue,
}

impl JsValue {
    pub fn is_array(&self) -> bool {
        unsafe { call_api_or_panic().jsIsArray(self.inner) != 0 }
    }
    pub fn is_boolean(&self) -> bool {
        unsafe { call_api_or_panic().jsIsBoolean(self.inner) != 0 }
    }
    pub fn is_false(&self) -> bool {
        unsafe { call_api_or_panic().jsIsFalse(self.inner) != 0 }
    }
    pub fn is_function(&self) -> bool {
        unsafe { call_api_or_panic().jsIsFunction(self.inner) != 0 }
    }
    pub fn is_null(&self) -> bool {
        unsafe { call_api_or_panic().jsIsNull(self.inner) != 0 }
    }
    pub fn is_number(&self) -> bool {
        unsafe { call_api_or_panic().jsIsNumber(self.inner) != 0 }
    }
    pub fn is_object(&self) -> bool {
        unsafe { call_api_or_panic().jsIsObject(self.inner) != 0 }
    }
    pub fn is_string(&self) -> bool {
        unsafe { call_api_or_panic().jsIsString(self.inner) != 0 }
    }
    pub fn is_true(&self) -> bool {
        unsafe { call_api_or_panic().jsIsTrue(self.inner) != 0 }
    }
    pub fn is_undefine(&self) -> bool {
        unsafe { call_api_or_panic().jsIsUndefined(self.inner) != 0 }
    }
    pub fn to_string(&self, es: JsExecState) -> String {
        unsafe {
            let cstr = call_api_or_panic().jsToString(es.inner, self.inner);
            CStr::from_ptr(cstr).to_string_lossy().to_string()
        }
    }
    pub fn new_string(es: JsExecState, text: &str) -> Self {
        Self {
            inner: unsafe {
                call_api()
                    .unwrap()
                    .jsString(es.inner, CString::safe_new(text).into_raw())
            },
        }
    }
    pub fn new_null() -> Self {
        Self {
            inner: unsafe { call_api_or_panic().jsNull() },
        }
    }
    pub fn as_ptr(&self) -> miniblink_sys::jsValue {
        self.inner
    }
}
