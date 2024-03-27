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

    pub fn arg_value<T>(&self, index: i32) -> MBResult<T>
    where
        Self: MBExecStateValue<T>,
    {
        let value = self.arg(index);
        self.value(value).map_err(|_| MBError::TypeError(index))
    }

    pub fn arg_count(&self) -> i32 {
        unsafe { call_api_or_panic().jsArgCount(self.inner) }
    }

    pub fn as_ptr(&self) -> jsExecState {
        self.inner
    }

    fn int(&self, value: i32) -> JsValue {
        JsValue {
            inner: unsafe { call_api_or_panic().jsInt(value as i32) },
        }
    }

    fn to_int(&self, value: JsValue) -> MBResult<i32> {
        unsafe {
            match value.get_type() {
                JsType::Number => Ok(call_api_or_panic().jsToInt(self.inner, value.inner)),
                _ => Err(MBError::FromJsValueFailed(value)),
            }
        }
    }

    fn double(&self, value: f64) -> JsValue {
        JsValue {
            inner: unsafe { call_api_or_panic().jsDouble(value) },
        }
    }

    fn to_double(&self, value: JsValue) -> MBResult<f64> {
        match value.get_type() {
            JsType::Number => {
                Ok(unsafe { call_api_or_panic().jsToDouble(self.inner, value.inner) })
            }
            _ => Err(MBError::FromJsValueFailed(value)),
        }
    }

    fn boolean(&self, value: bool) -> JsValue {
        JsValue {
            inner: unsafe { call_api_or_panic().jsBoolean(value) },
        }
    }

    fn to_boolean(&self, value: JsValue) -> MBResult<bool> {
        match value.get_type() {
            JsType::Boolean => {
                Ok(unsafe { call_api_or_panic().jsToBoolean(self.inner, value.inner) != 0 })
            }
            _ => Err(MBError::FromJsValueFailed(value)),
        }
    }

    fn string(&self, value: String) -> JsValue {
        let text = CString::safe_new(&value);
        let value = unsafe { call_api_or_panic().jsString(self.inner, text.as_ptr()) };
        JsValue { inner: value }
    }

    fn to_string(&self, value: JsValue) -> MBResult<String> {
        unsafe {
            match value.get_type() {
                JsType::Boolean
                | JsType::Null
                | JsType::Number
                | JsType::String
                | JsType::Undefined => {
                    let cstr = call_api_or_panic().jsToTempString(self.inner, value.inner);
                    let cstr = CStr::from_ptr(cstr);
                    Ok(cstr.to_string_lossy().to_string())
                }
                _ => Err(MBError::FromJsValueFailed(value)),
            }
        }
    }

    fn undefined(&self) -> JsValue {
        JsValue {
            inner: unsafe { call_api_or_panic().jsUndefined() },
        }
    }

    #[allow(dead_code)]
    fn null(&self) -> JsValue {
        JsValue {
            inner: unsafe { call_api_or_panic().jsNull() },
        }
    }

    fn get_at(&self, js_array: JsValue, index: i32) -> JsValue {
        JsValue {
            inner: unsafe { call_api_or_panic().jsGetAt(self.as_ptr(), js_array.as_ptr(), index) },
        }
    }

    fn set_at(&self, js_array: JsValue, index: i32, js_value: JsValue) {
        unsafe {
            call_api_or_panic().jsSetAt(self.as_ptr(), js_array.as_ptr(), index, js_value.as_ptr())
        }
    }

    fn get_length(&self, js_array: JsValue) -> i32 {
        unsafe { call_api_or_panic().jsGetLength(self.as_ptr(), js_array.as_ptr()) }
    }

    fn set_length(&self, js_array: JsValue, length: i32) {
        unsafe { call_api_or_panic().jsSetLength(self.as_ptr(), js_array.as_ptr(), length) }
    }

    fn empty_array(&self) -> JsValue {
        JsValue {
            inner: unsafe { call_api_or_panic().jsEmptyArray(self.as_ptr()) },
        }
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

pub trait MBExecStateValue<T> {
    fn js_value(&self, value: T) -> JsValue;
    fn value(&self, value: JsValue) -> MBResult<T>;
}

impl MBExecStateValue<i32> for JsExecState {
    fn js_value(&self, value: i32) -> JsValue {
        self.int(value)
    }

    fn value(&self, value: JsValue) -> MBResult<i32> {
        self.to_int(value)
    }
}

impl MBExecStateValue<f64> for JsExecState {
    fn js_value(&self, value: f64) -> JsValue {
        self.double(value)
    }

    fn value(&self, value: JsValue) -> MBResult<f64> {
        self.to_double(value)
    }
}

impl MBExecStateValue<bool> for JsExecState {
    fn js_value(&self, value: bool) -> JsValue {
        self.boolean(value)
    }

    fn value(&self, value: JsValue) -> MBResult<bool> {
        self.to_boolean(value)
    }
}

impl MBExecStateValue<String> for JsExecState {
    fn js_value(&self, value: String) -> JsValue {
        self.string(value)
    }

    fn value(&self, value: JsValue) -> MBResult<String> {
        self.to_string(value)
    }
}

impl MBExecStateValue<()> for JsExecState {
    fn js_value(&self, _value: ()) -> JsValue {
        self.undefined()
    }

    fn value(&self, value: JsValue) -> MBResult<()> {
        match value.get_type() {
            JsType::Undefined => Ok(()),
            _ => Err(MBError::FromJsValueFailed(value)),
        }
    }
}

impl<T> MBExecStateValue<Vec<T>> for JsExecState
where
    Self: MBExecStateValue<T>,
{
    fn js_value(&self, value: Vec<T>) -> JsValue {
        let array = self.empty_array();
        self.set_length(array, value.len() as i32);
        for (i, v) in value.into_iter().enumerate() {
            self.set_at(array, i as i32, self.js_value(v))
        }
        array
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
