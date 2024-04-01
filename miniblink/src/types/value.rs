use std::ffi::{CStr, CString};

use crate::call_api_or_panic;
use crate::error::{MBError, MBResult};
use crate::types::WkeString;
use crate::webview::WebView;
use miniblink_sys::{jsExecState, jsKeys, jsType, jsValue};

/// See [`jsType`].
#[allow(missing_docs)]
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

impl std::fmt::Display for JsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

use crate::{bind_target, bind_target_global};

/// See `jsExecState`.
#[derive(Clone, Copy)]
pub struct JsExecState {
    pub(crate) inner: jsExecState,
}

impl JsExecState {
    bind_target_global! {
        pub jsInt => int(value: i32) -> JsValue;
        pub jsDouble => double(value: f64) -> JsValue;
        pub jsBoolean => boolean(value: bool) -> JsValue;
        pub jsUndefined => undefined() -> JsValue;
        pub jsNull => null() -> JsValue
    }

    bind_target! {
        pub jsArg => arg(index: i32) -> JsValue;
        pub jsArgCount => arg_count() -> i32;
        pub jsEmptyArray => empty_array() -> JsValue;
        pub jsEmptyObject => empty_object() -> JsValue;
        pub jsString => string(value: &str as CString) -> JsValue;
        pub jsGetAt => get_at(js_array: JsValue, index: i32) -> JsValue;
        pub jsSetAt => set_at(js_array: JsValue, index: i32, js_value: JsValue);
        pub jsGetLength => get_length(js_array: JsValue) -> i32;
        pub jsSetLength => set_length(js_array: JsValue, length: i32);
        pub jsGet => get(js_object: JsValue, prop: &str as CString) -> JsValue;
        pub jsSet => set(js_object: JsValue, prop: &str as CString, value: JsValue);
        pub(crate) jsGetKeys => get_keys(js_object: JsValue) -> JsKeys;
        pub jsGetGlobal => get_global(prop: &str as CString) -> JsValue;
        pub jsSetGlobal => set_global(prop: &str as CString, value: JsValue);
        pub jsGetWebView => get_webview() -> WebView;
        // pub jsGetData => 
        // pub jsGetLastErrorIfException => get_last_error_if_exception() -> ;
        // pub jsFunction
        // pub jsObject
    }

    bind_target! {
        pub jsEvalW => eval(script: &str as WkeString) -> JsValue;
        pub jsEvalExW => eval_ex(script: &str as WkeString, is_in_closure: bool) -> JsValue;
        // pub jsCall => call(func: JsValue, this_value: JsValue, args: jsValue* , int arg_count);
        // pub jsCallGlobal => call(func: JsValue, args: jsValue* , int arg_count);
    }

    bind_target! {
        jsToInt => _to_int(value: JsValue) -> i32;
        jsToDouble => _to_double(value: JsValue) -> f64;
        jsToTempString => _to_string(value: JsValue) -> String;
        jsToBoolean => _to_boolean(value: JsValue) -> bool;
    }

    /// See `jsToInt`.
    pub fn to_int(&self, value: JsValue) -> MBResult<i32> {
        match value.type_of_() {
            JsType::Number => Ok(self._to_int(value)),
            other => Err(MBError::UnsupportedType(JsType::Number, other)),
        }
    }

    /// See `jsToDouble`.
    pub fn to_double(&self, value: JsValue) -> MBResult<f64> {
        match value.type_of_() {
            JsType::Number => Ok(self._to_double(value)),
            other => Err(MBError::UnsupportedType(JsType::Number, other)),
        }
    }

    /// See `jsToBoolean`.
    pub fn to_boolean(&self, value: JsValue) -> MBResult<bool> {
        match value.type_of_() {
            JsType::Boolean => Ok(self._to_boolean(value)),
            other => Err(MBError::UnsupportedType(JsType::Boolean, other)),
        }
    }

    /// See `jsToTempString`.
    pub fn to_string(&self, value: JsValue) -> MBResult<String> {
        match value.type_of_() {
            JsType::Boolean
            | JsType::Null
            | JsType::Number
            | JsType::String
            | JsType::Undefined => Ok(self._to_string(value)),
            other => Err(MBError::UnsupportedType(JsType::Boolean, other)),
        }
    }

    /// Get inner ptr of [`JsExecState`]. See [`jsExecState`].
    pub fn as_ptr(&self) -> jsExecState {
        self.inner
    }

    /// Create [`JsExecState`] from ptr.
    pub unsafe fn from_ptr(ptr: jsExecState) -> Self {
        Self { inner: ptr }
    }
}

/// Extra api for JsExecState
pub trait JsExecStateExt {
    /// Get arg value from execution state. Helper function.
    fn arg_value<T>(&self, index: i32) -> MBResult<T>
    where
        Self: MBExecStateValue<T>;
}

impl JsExecStateExt for JsExecState {
    fn arg_value<T>(&self, index: i32) -> MBResult<T>
    where
        Self: MBExecStateValue<T>,
    {
        if index >= self.arg_count() {
            Err(MBError::ArgNotMatch(format!("arg index out of range")))
        } else {
            self.value(self.arg(index)).map_err(|e| match e {
                #[cfg(feature = "serde")]
                MBError::SerdeMessage(msg) => {
                    MBError::ArgNotMatch(format!("not match at arg index {index}, {msg}"))
                }
                MBError::UnsupportedType(expect, provided) => MBError::ArgNotMatch(format!(
                    "not match at arg index {index}, expect {expect} but {provided} provided"
                )),
                _ => MBError::ArgNotMatch(format!("not match at arg index {index}")),
            })
        }
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

    pub unsafe fn from_ptr(ptr: *mut jsKeys) -> Self {
        Self { inner: ptr }
    }
}

// #[allow(dead_code)]
// pub(crate) struct JsArrayBuffer {
//     inner: *mut wkeMemBuf,
// }

// impl JsArrayBuffer {
//     #[allow(dead_code)]
//     pub unsafe fn from_ptr(ptr: *mut wkeMemBuf) -> Self {
//         Self { inner: ptr }
//     }
// }

/// See `jsValue`.
#[derive(Debug, Clone, Copy)]
pub struct JsValue {
    pub(crate) inner: jsValue,
}

impl JsValue {
    bind_target! {
        pub jsTypeOf => type_of_() -> JsType;
        pub jsIsNumber => is_number() -> bool;
        pub jsIsString => is_string() -> bool;
        pub jsIsBoolean => is_boolean() -> bool;
        pub jsIsObject => is_object() -> bool;
        pub jsIsFunction => is_function() -> bool;
        pub jsIsUndefined => is_undefined() -> bool;
        pub jsIsNull => is_null() -> bool;
        pub jsIsArray => is_array() -> bool;
        pub jsIsTrue => is_true() -> bool;
        pub jsIsFalse => is_false() -> bool;
    }

    /// Get the inner ptr of [`JsValue`]. See [`jsValue`].
    pub fn as_ptr(&self) -> jsValue {
        self.inner
    }

    /// Create [`JsValue`] from ptr.
    pub unsafe fn from_ptr(ptr: jsValue) -> Self {
        Self { inner: ptr }
    }
}

/// Trait for converting between [`JsValue`] and `T`.
pub trait MBExecStateValue<T> {
    /// Convert from `T` to [`JsValue`].
    fn js_value(&self, value: T) -> MBResult<JsValue>;
    /// Convert from [`JsValue`] to `T`.
    fn value(&self, value: JsValue) -> MBResult<T>;
}

#[cfg(not(feature = "serde"))]
impl MBExecStateValue<i32> for JsExecState {
    fn js_value(&self, value: i32) -> MBResult<JsValue> {
        Ok(self.int(value))
    }

    fn value(&self, value: JsValue) -> MBResult<i32> {
        self.to_int(value)
    }
}

#[cfg(not(feature = "serde"))]
impl MBExecStateValue<f64> for JsExecState {
    fn js_value(&self, value: f64) -> MBResult<JsValue> {
        Ok(self.double(value))
    }

    fn value(&self, value: JsValue) -> MBResult<f64> {
        self.to_double(value)
    }
}

#[cfg(not(feature = "serde"))]
impl MBExecStateValue<bool> for JsExecState {
    fn js_value(&self, value: bool) -> MBResult<JsValue> {
        Ok(self.boolean(value))
    }

    fn value(&self, value: JsValue) -> MBResult<bool> {
        self.to_boolean(value)
    }
}

#[cfg(not(feature = "serde"))]
impl MBExecStateValue<String> for JsExecState {
    fn js_value(&self, value: String) -> MBResult<JsValue> {
        Ok(self.string(value.as_str()))
    }

    fn value(&self, value: JsValue) -> MBResult<String> {
        self.to_string(value)
    }
}

#[cfg(not(feature = "serde"))]
impl MBExecStateValue<()> for JsExecState {
    fn js_value(&self, _value: ()) -> MBResult<JsValue> {
        Ok(self.undefined())
    }

    fn value(&self, value: JsValue) -> MBResult<()> {
        match value.type_of_() {
            JsType::Undefined => Ok(()),
            other => Err(MBError::UnsupportedType(JsType::Undefined, other)),
        }
    }
}

#[cfg(not(feature = "serde"))]
impl<T> MBExecStateValue<Vec<T>> for JsExecState
where
    Self: MBExecStateValue<T>,
{
    fn js_value(&self, value: Vec<T>) -> MBResult<JsValue> {
        let array = self.empty_array();
        self.set_length(array, value.len() as i32);
        for (i, v) in value.into_iter().enumerate() {
            self.set_at(array, i as i32, self.js_value(v)?)
        }
        Ok(array)
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

#[cfg(not(feature = "serde"))]
use std::collections::HashMap;
#[cfg(not(feature = "serde"))]
impl<V> MBExecStateValue<HashMap<String, V>> for JsExecState
where
    Self: MBExecStateValue<V>,
{
    fn js_value(&self, value: HashMap<String, V>) -> MBResult<JsValue> {
        let object = self.empty_object();
        for (k, v) in value.into_iter() {
            self.set(object, k.as_str(), self.js_value(v)?);
        }
        Ok(object)
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

#[cfg(feature = "serde")]
impl<T> MBExecStateValue<T> for JsExecState
where
    T: for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    fn js_value(&self, value: T) -> MBResult<JsValue> {
        crate::serde::to_value(*self, &value)
    }

    fn value(&self, value: JsValue) -> MBResult<T> {
        crate::serde::from_value(*self, value)
    }
}
