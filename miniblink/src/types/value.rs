use std::ffi::{CStr, CString};

use crate::call_api_or_panic;
use crate::error::{MBError, MBResult};
use miniblink_sys::{jsExecState, jsKeys, jsType, jsValue};

/// Types in JavaScript, see [`jsType`].
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

/// A type used in miniblink. See jsExecState.
#[derive(Clone, Copy)]
pub struct JsExecState {
    pub(crate) inner: jsExecState,
}

impl JsExecState {
    bind_target_global! {
        pub(crate) jsInt => int(value: i32) -> JsValue;
        pub(crate) jsDouble => double(value: f64) -> JsValue;
        pub(crate) jsBoolean => boolean(value: bool) -> JsValue;
        // pub(crate) jsUndefined => undefined() -> JsValue;
        pub(crate) jsNull => null() -> JsValue
    }

    bind_target! {
        pub(crate) jsArg => arg(index: i32) -> JsValue;
        pub(crate) jsArgCount => arg_count() -> i32;
        pub(crate) jsEmptyArray => empty_array() -> JsValue;
        pub(crate) jsEmptyObject => empty_object() -> JsValue;
        pub(crate) jsString => string(value: &str as CString) -> JsValue;
        pub(crate) jsGetAt => get_at(js_array: JsValue, index: i32) -> JsValue;
        pub(crate) jsSetAt => set_at(js_array: JsValue, index: i32, js_value: JsValue);
        pub(crate) jsGetLength => get_length(js_array: JsValue) -> i32;
        pub(crate) jsSetLength => set_length(js_array: JsValue, length: i32);
        pub(crate) jsGet => get(js_object: JsValue, prop: &str as CString) -> JsValue;
        pub(crate) jsSet => set(js_object: JsValue, prop: &str, value: JsValue);
        pub(crate) jsGetKeys => get_keys(js_object: JsValue) -> JsKeys;
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

pub(crate) trait ToValue {
    fn to_int(&self, value: JsValue) -> MBResult<i32>;
    fn to_double(&self, value: JsValue) -> MBResult<f64>;
    fn to_boolean(&self, value: JsValue) -> MBResult<bool>;
    fn to_string(&self, value: JsValue) -> MBResult<String>;
}

impl ToValue for JsExecState {
    fn to_int(&self, value: JsValue) -> MBResult<i32> {
        unsafe {
            match value.type_of_() {
                JsType::Number => Ok(call_api_or_panic().jsToInt(self.inner, value.inner)),
                other => Err(MBError::UnsupportedType(JsType::Number, other)),
            }
        }
    }

    fn to_double(&self, value: JsValue) -> MBResult<f64> {
        match value.type_of_() {
            JsType::Number => {
                Ok(unsafe { call_api_or_panic().jsToDouble(self.inner, value.inner) })
            }
            other => Err(MBError::UnsupportedType(JsType::Number, other)),
        }
    }

    fn to_boolean(&self, value: JsValue) -> MBResult<bool> {
        match value.type_of_() {
            JsType::Boolean => {
                Ok(unsafe { call_api_or_panic().jsToBoolean(self.inner, value.inner) != 0 })
            }
            other => Err(MBError::UnsupportedType(JsType::Boolean, other)),
        }
    }

    fn to_string(&self, value: JsValue) -> MBResult<String> {
        unsafe {
            match value.type_of_() {
                JsType::Boolean
                | JsType::Null
                | JsType::Number
                | JsType::String
                | JsType::Undefined => {
                    let cstr = call_api_or_panic().jsToTempString(self.inner, value.inner);
                    let cstr = CStr::from_ptr(cstr);
                    Ok(cstr.to_string_lossy().to_string())
                }
                other => Err(MBError::UnsupportedType(JsType::Boolean, other)),
            }
        }
    }
}

/// Extra api for JsExecState
pub trait JsExecStateExt {
    /// Get arg from execution state. See jsArg.
    fn get_arg(&self, index: i32) -> Option<JsValue>;

    /// Get arg value from execution state. Helper function.
    fn get_arg_value<T>(&self, index: i32) -> MBResult<T>
    where
        Self: MBExecStateValue<T>;
}

impl JsExecStateExt for JsExecState {
    fn get_arg(&self, index: i32) -> Option<JsValue> {
        if index < self.arg_count() {
            Some(self.arg(index))
        } else {
            None
        }
    }

    fn get_arg_value<T>(&self, index: i32) -> MBResult<T>
    where
        Self: MBExecStateValue<T>,
    {
        if let Some(value) = self.get_arg(index) {
            self.value(value).map_err(|e| match e {
                #[cfg(feature = "serde")]
                MBError::SerdeMessage(msg) => {
                    MBError::ArgNotMatch(format!("not match at arg index {index}, {msg}"))
                }
                MBError::UnsupportedType(expect, provided) => MBError::ArgNotMatch(format!(
                    "not match at arg index {index}, expect {expect} but {provided} provided"
                )),
                _ => MBError::ArgNotMatch(format!("not match at arg index {index}")),
            })
        } else {
            Err(MBError::ArgNotMatch(format!("arg index out of range")))
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

/// A type used in miniblink. See jsValue.
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
}

/// Extra api for [`JsValue`]
pub trait JsValueExt {
    /// Get the inner ptr of [`JsValue`]. See [`jsValue`].
    fn as_ptr(&self) -> jsValue;

    /// Create [`JsValue`] from ptr.
    unsafe fn from_ptr(ptr: jsValue) -> Self;
}

impl JsValueExt for JsValue {
    fn as_ptr(&self) -> jsValue {
        self.inner
    }

    unsafe fn from_ptr(ptr: jsValue) -> Self {
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
        match value.get_type() {
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
