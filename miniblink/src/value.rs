use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt::Display;

use crate::error::{MBError, MBResult};
use crate::{call_api_or_panic, util::SafeCString};
use miniblink_sys::{jsExecState, jsKeys, jsType, jsValue, wkeMemBuf};

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

impl Display for JsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A type used in miniblink. See jsExecState.
#[derive(Clone, Copy)]
pub struct JsExecState {
    pub(crate) inner: jsExecState,
}

impl JsExecState {
    pub fn arg(&self, index: i32) -> JsValue {
        JsValue::from_ptr(unsafe { call_api_or_panic().jsArg(self.inner, index) })
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

    pub fn from_ptr(ptr: jsExecState) -> Self {
        Self { inner: ptr }
    }

    pub(crate) fn int(&self, value: i32) -> JsValue {
        JsValue::from_ptr(unsafe { call_api_or_panic().jsInt(value as i32) })
    }

    pub(crate) fn to_int(&self, value: JsValue) -> MBResult<i32> {
        unsafe {
            match value.get_type() {
                JsType::Number => Ok(call_api_or_panic().jsToInt(self.inner, value.inner)),
                _ => Err(MBError::FromJsValueFailed(value)),
            }
        }
    }

    pub(crate) fn double(&self, value: f64) -> JsValue {
        JsValue::from_ptr(unsafe { call_api_or_panic().jsDouble(value) })
    }

    pub(crate) fn to_double(&self, value: JsValue) -> MBResult<f64> {
        match value.get_type() {
            JsType::Number => {
                Ok(unsafe { call_api_or_panic().jsToDouble(self.inner, value.inner) })
            }
            _ => Err(MBError::FromJsValueFailed(value)),
        }
    }

    pub(crate) fn boolean(&self, value: bool) -> JsValue {
        JsValue::from_ptr(unsafe { call_api_or_panic().jsBoolean(value) })
    }

    pub(crate) fn to_boolean(&self, value: JsValue) -> MBResult<bool> {
        match value.get_type() {
            JsType::Boolean => {
                Ok(unsafe { call_api_or_panic().jsToBoolean(self.inner, value.inner) != 0 })
            }
            _ => Err(MBError::FromJsValueFailed(value)),
        }
    }

    pub fn string(&self, value: &str) -> JsValue {
        let text = CString::safe_new(&value);
        let value = unsafe { call_api_or_panic().jsString(self.inner, text.as_ptr()) };
        JsValue::from_ptr(value)
    }

    pub(crate) fn to_string(&self, value: JsValue) -> MBResult<String> {
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

    pub(crate) fn undefined(&self) -> JsValue {
        JsValue::from_ptr(unsafe { call_api_or_panic().jsUndefined() })
    }

    pub(crate) fn null(&self) -> JsValue {
        JsValue::from_ptr(unsafe { call_api_or_panic().jsNull() })
    }

    pub(crate) fn get_at(&self, js_array: JsValue, index: i32) -> JsValue {
        JsValue::from_ptr(unsafe {
            call_api_or_panic().jsGetAt(self.as_ptr(), js_array.as_ptr(), index)
        })
    }

    pub(crate) fn set_at(&self, js_array: JsValue, index: i32, js_value: JsValue) {
        unsafe {
            call_api_or_panic().jsSetAt(self.as_ptr(), js_array.as_ptr(), index, js_value.as_ptr())
        }
    }

    pub(crate) fn get_length(&self, js_array: JsValue) -> i32 {
        unsafe { call_api_or_panic().jsGetLength(self.as_ptr(), js_array.as_ptr()) }
    }

    pub(crate) fn set_length(&self, js_array: JsValue, length: i32) {
        unsafe { call_api_or_panic().jsSetLength(self.as_ptr(), js_array.as_ptr(), length) }
    }

    pub(crate) fn empty_array(&self) -> JsValue {
        JsValue::from_ptr(unsafe { call_api_or_panic().jsEmptyArray(self.as_ptr()) })
    }

    pub fn empty_object(&self) -> JsValue {
        JsValue::from_ptr(unsafe { call_api_or_panic().jsEmptyObject(self.as_ptr()) })
    }

    pub fn get(&self, js_object: JsValue, prop: &str) -> JsValue {
        let prop = CString::safe_new(prop);
        JsValue::from_ptr(unsafe {
            call_api_or_panic().jsGet(self.as_ptr(), js_object.as_ptr(), prop.as_ptr())
        })
    }

    pub fn set(&self, js_object: JsValue, prop: &str, value: JsValue) {
        let prop = CString::safe_new(prop);
        unsafe {
            call_api_or_panic().jsSet(
                self.as_ptr(),
                js_object.as_ptr(),
                prop.as_ptr(),
                value.as_ptr(),
            )
        }
    }

    pub(crate) fn get_keys(&self, js_object: JsValue) -> JsKeys {
        let js_keys = unsafe { call_api_or_panic().jsGetKeys(self.as_ptr(), js_object.as_ptr()) };
        JsKeys { inner: js_keys }
    }

    #[allow(dead_code)]
    pub(crate) fn array_buffer(&self, _buffer: &[u8], size: usize) -> JsValue {
        JsValue::from_ptr(unsafe {
            call_api_or_panic().jsArrayBuffer(self.as_ptr(), std::ptr::null(), size)
        })
    }

    #[allow(dead_code)]
    pub(crate) fn get_array_buffer(&self, buffer: JsValue) -> JsArrayBuffer {
        JsArrayBuffer::from_ptr(unsafe {
            call_api_or_panic().jsGetArrayBuffer(self.as_ptr(), buffer.as_ptr())
        })
    }
}

pub(crate) struct JsKeys {
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

    #[allow(dead_code)]
    pub fn from_ptr(ptr: *mut jsKeys) -> Self {
        Self { inner: ptr }
    }
}

#[allow(dead_code)]
pub(crate) struct JsArrayBuffer {
    inner: *mut wkeMemBuf,
}

impl JsArrayBuffer {
    pub fn from_ptr(ptr: *mut wkeMemBuf) -> Self {
        Self { inner: ptr }
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

    pub fn from_ptr(ptr: jsValue) -> Self {
        Self { inner: ptr }
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
        self.string(value.as_str())
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

impl<V> MBExecStateValue<HashMap<String, V>> for JsExecState
where
    Self: MBExecStateValue<V>,
{
    fn js_value(&self, value: HashMap<String, V>) -> JsValue {
        let object = self.empty_object();
        for (k, v) in value.into_iter() {
            self.set(object, k.as_str(), self.js_value(v));
        }
        object
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
