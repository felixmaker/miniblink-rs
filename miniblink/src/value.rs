use std::ffi::{CStr, CString};

use crate::error::{MBError, MBResult};
use crate::{call_api_or_panic, util::SafeCString};
use miniblink_sys::{jsExecState, jsType, jsValue};

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

/// A type used in miniblink. See jsExecState.
#[derive(Clone, Copy)]
pub struct JsExecState {
    pub(crate) inner: jsExecState,
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

    pub fn as_ptr(&self) -> jsExecState {
        self.inner
    }
}

/// A type used in miniblink. See jsValue.
#[derive(Debug, Clone, Copy)]
pub struct JsValue {
    pub(crate) inner: jsValue,
}

impl JsValue {
    pub fn get_type(&self) -> JsType {
        let js_type = unsafe { call_api_or_panic().jsTypeOf(self.inner) };
        js_type.into()
    }

    pub fn as_ptr(&self) -> jsValue {
        self.inner
    }
}

pub trait MBExecStateValue<T>: Sized {
    fn from_value(es: JsExecState, value: T) -> Self;
    fn to_value(&self, es: JsExecState) -> MBResult<T>;
}

impl MBExecStateValue<i32> for JsValue {
    fn from_value(_: JsExecState, value: i32) -> Self {
        let value = unsafe { call_api_or_panic().jsInt(value as i32) };
        Self { inner: value }
    }

    fn to_value(&self, es: JsExecState) -> MBResult<i32> {
        unsafe {
            match self.get_type() {
                JsType::Number => Ok(call_api_or_panic().jsToInt(es.inner, self.inner)),
                _ => Err(MBError::FromJsValueFailed(*self)),
            }
        }
    }
}

impl MBExecStateValue<f64> for JsValue {
    fn from_value(_es: JsExecState, value: f64) -> Self {
        let value = unsafe { call_api_or_panic().jsDouble(value) };
        Self { inner: value }
    }

    fn to_value(&self, es: JsExecState) -> MBResult<f64> {
        match self.get_type() {
            JsType::Number => Ok(unsafe { call_api_or_panic().jsToDouble(es.inner, self.inner) }),
            _ => Err(MBError::FromJsValueFailed(*self)),
        }
    }
}

impl MBExecStateValue<bool> for JsValue {
    fn from_value(_es: JsExecState, value: bool) -> Self {
        let value = unsafe { call_api_or_panic().jsBoolean(value) };
        Self { inner: value }
    }

    fn to_value(&self, es: JsExecState) -> MBResult<bool> {
        match self.get_type() {
            JsType::Boolean => {
                Ok(unsafe { call_api_or_panic().jsToBoolean(es.inner, self.inner) != 0 })
            }
            _ => Err(MBError::FromJsValueFailed(*self)),
        }
    }
}

impl MBExecStateValue<String> for JsValue {
    fn from_value(es: JsExecState, value: String) -> Self {
        let text = CString::safe_new(&value);
        let value = unsafe { call_api_or_panic().jsString(es.inner, text.as_ptr()) };
        Self { inner: value }
    }

    fn to_value(&self, es: JsExecState) -> MBResult<String> {
        unsafe {
            match self.get_type() {
                JsType::Boolean
                | JsType::Null
                | JsType::Number
                | JsType::String
                | JsType::Undefined => {
                    let cstr = call_api_or_panic().jsToTempString(es.inner, self.inner);
                    let cstr = CStr::from_ptr(cstr);
                    Ok(cstr.to_string_lossy().to_string())
                }
                _ => Err(MBError::FromJsValueFailed(*self)),
            }
        }
    }
}

impl MBExecStateValue<()> for JsValue {
    fn from_value(_es: JsExecState, _value: ()) -> Self {
        Self {
            inner: unsafe { call_api_or_panic().jsUndefined() },
        }
    }

    fn to_value(&self, _es: JsExecState) -> MBResult<()> {
        match self.get_type() {
            JsType::Undefined => Ok(()),
            _ => Err(MBError::FromJsValueFailed(*self)),
        }
    }
}
