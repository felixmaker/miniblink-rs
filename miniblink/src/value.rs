use std::ffi::{CStr, CString};

use crate::{util::SafeCString, call_api};

#[derive(Clone, Copy)]
pub struct JsExecState {
    pub(crate) inner: *mut std::os::raw::c_void,
}

impl JsExecState {
    pub fn arg(&self, index: i32) -> JsValue {
        JsValue {
            inner: unsafe { call_api().jsArg(self.inner, index) },
        }
    }

    pub fn arg_count(&self) -> i32 {
        unsafe { call_api().jsArgCount(self.inner) }
    }
}

#[derive(Clone, Copy)]
pub struct JsValue {
    pub(crate) inner: miniblink_sys::jsValue,
}

impl JsValue {
    pub fn is_array(&self) -> bool {
        unsafe { call_api().jsIsArray(self.inner) != 0 }
    }
    pub fn is_boolean(&self) -> bool {
        unsafe { call_api().jsIsBoolean(self.inner) != 0 }
    }
    pub fn is_false(&self) -> bool {
        unsafe { call_api().jsIsFalse(self.inner) != 0 }
    }
    pub fn is_function(&self) -> bool {
        unsafe { call_api().jsIsFunction(self.inner) != 0 }
    }
    pub fn is_null(&self) -> bool {
        unsafe { call_api().jsIsNull(self.inner) != 0 }
    }
    pub fn is_number(&self) -> bool {
        unsafe { call_api().jsIsNumber(self.inner) != 0 }
    }
    pub fn is_object(&self) -> bool {
        unsafe { call_api().jsIsObject(self.inner) != 0 }
    }
    pub fn is_string(&self) -> bool {
        unsafe { call_api().jsIsString(self.inner) != 0 }
    }
    pub fn is_true(&self) -> bool {
        unsafe { call_api().jsIsTrue(self.inner) != 0 }
    }
    pub fn is_undefine(&self) -> bool {
        unsafe { call_api().jsIsUndefined(self.inner) != 0 }
    }
    pub fn to_string(&self, es: JsExecState) -> String {
        unsafe {
            let cstr = call_api().jsToString(es.inner, self.inner);
            CStr::from_ptr(cstr).to_string_lossy().to_string()
        }
    }
    pub fn new_string(es: JsExecState, text: &str) -> Self {
        Self {
            inner: unsafe { call_api().jsString(es.inner, CString::safe_new(text).into_raw()) },
        }
    }
    pub fn new_null() -> Self {
        Self {
            inner: unsafe { call_api().jsNull() },
        }
    }
    pub fn as_ptr(&self) -> miniblink_sys::jsValue {
        self.inner
    }
}
