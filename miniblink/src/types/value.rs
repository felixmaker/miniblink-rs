use std::ffi::{CStr, CString};

use crate::call_api_or_panic;
use crate::error::{MBError, MBResult};
use crate::types::WkeString;
use crate::util::SafeCString;
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

macro_rules! js_value {
    ($expr: expr) => {{
        let value = $expr;
        assert!(value != 0);
        unsafe { JsValue::from_ptr(value) }
    }};
}

/// See `jsExecState`.
#[derive(Clone, Copy)]
pub struct JsExecState {
    pub(crate) inner: jsExecState,
}

impl JsExecState {
    /// See jsInt.
    pub fn int(&self, value: i32) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsInt(value) })
    }
    /// See jsDouble.
    pub fn double(&self, value: f64) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsDouble(value) })
    }
    /// See jsBoolean.
    pub fn boolean(&self, value: bool) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsBoolean(value) })
    }
    /// See jsUndefined.
    pub fn undefined(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsUndefined() })
    }
    /// See jsNull.
    pub fn null(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsNull() })
    }

    /// See jsArg.
    pub fn arg(&self, index: i32) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsArg(self.inner, index) })
    }
    /// See jsArgCount.
    pub fn arg_count(&self) -> i32 {
        unsafe { call_api_or_panic().jsArgCount(self.inner) }
    }
    /// See jsEmptyArray.
    pub fn empty_array(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsEmptyArray(self.inner,) })
    }
    /// See jsEmptyObject.
    pub fn empty_object(&self) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsEmptyObject(self.inner,) })
    }
    /// See jsString.
    pub fn string(&self, value: &str) -> JsValue {
        js_value!({
            let value = CString::safe_new(value);
            unsafe { call_api_or_panic().jsString(self.inner, value.as_ptr()) }
        })
    }
    /// See jsGetAt.
    pub fn get_at(&self, js_array: JsValue, index: i32) -> JsValue {
        js_value!(unsafe { call_api_or_panic().jsGetAt(self.inner, js_array.as_ptr(), index) })
    }
    /// See jsSetAt.
    pub fn set_at(&self, js_array: JsValue, index: i32, js_value: JsValue) {
        unsafe {
            call_api_or_panic().jsSetAt(self.inner, js_array.as_ptr(), index, js_value.as_ptr())
        }
    }
    /// See jsGetLength.
    pub fn get_length(&self, js_array: JsValue) -> i32 {
        unsafe { call_api_or_panic().jsGetLength(self.inner, js_array.as_ptr()) }
    }
    /// See jsSetLength.
    pub fn set_length(&self, js_array: JsValue, length: i32) {
        unsafe { call_api_or_panic().jsSetLength(self.inner, js_array.as_ptr(), length) }
    }
    /// See jsGet.
    pub fn get(&self, js_object: JsValue, prop: &str) -> JsValue {
        let prop = CString::safe_new(prop);
        js_value!({
            unsafe { call_api_or_panic().jsGet(self.inner, js_object.as_ptr(), prop.as_ptr()) }
        })
    }
    /// See jsSet.
    pub fn set(&self, js_object: JsValue, prop: &str, value: JsValue) {
        let prop = CString::safe_new(prop);
        unsafe {
            call_api_or_panic().jsSet(
                self.inner,
                js_object.as_ptr(),
                prop.as_ptr(),
                value.as_ptr(),
            )
        }
    }
    /// See jsGetKeys.
    pub fn get_keys(&self, js_object: JsValue) -> JsKeys {
        let keys = unsafe { call_api_or_panic().jsGetKeys(self.inner, js_object.as_ptr()) };
        assert!(!keys.is_null());
        unsafe { JsKeys::from_ptr(keys) }
    }
    /// See jsGetGlobal.
    pub fn get_global(&self, prop: &str) -> JsValue {
        js_value!({
            let prop = CString::safe_new(prop);
            unsafe { call_api_or_panic().jsGetGlobal(self.inner, prop.as_ptr()) }
        })
    }
    /// See jsSetGlobal.
    pub fn set_global(&self, prop: &str, value: JsValue) {
        let prop = CString::safe_new(prop);
        unsafe { call_api_or_panic().jsSetGlobal(self.inner, prop.as_ptr(), value.as_ptr()) }
    }
    /// See jsGetWebView.
    pub fn get_webview(&self) -> WebView {
        let webview = unsafe { call_api_or_panic().jsGetWebView(self.inner) };
        assert!(!webview.is_null());
        unsafe { WebView::from_ptr(webview) }
    }

    /// See jsEvalW.
    pub fn eval(&self, script: &str) -> JsValue {
        js_value!({
            let script = WkeString::new(script);
            unsafe { call_api_or_panic().jsEvalW(self.inner, script.as_wcstr_ptr()) }
        })
    }
    /// See jsEvalExW.
    pub fn eval_ex(&self, script: &str, is_in_closure: bool) -> JsValue {
        let script = WkeString::new(script);
        js_value!(unsafe {
            call_api_or_panic().jsEvalExW(self.inner, script.as_wcstr_ptr(), is_in_closure)
        })
    }

    /// See `jsToInt`.
    pub fn to_int(&self, value: JsValue) -> MBResult<i32> {
        match value.type_of_() {
            JsType::Number => {
                Ok(unsafe { call_api_or_panic().jsToInt(self.inner, value.as_ptr()) })
            }
            other => Err(MBError::UnsupportedType(JsType::Number, other)),
        }
    }

    /// See `jsToDouble`.
    pub fn to_double(&self, value: JsValue) -> MBResult<f64> {
        match value.type_of_() {
            JsType::Number => {
                Ok(unsafe { call_api_or_panic().jsToDouble(self.inner, value.as_ptr()) })
            }
            other => Err(MBError::UnsupportedType(JsType::Number, other)),
        }
    }

    /// See `jsToBoolean`.
    pub fn to_boolean(&self, value: JsValue) -> MBResult<bool> {
        match value.type_of_() {
            JsType::Boolean => {
                Ok(unsafe { call_api_or_panic().jsToBoolean(self.inner, value.as_ptr()) != 0 })
            }
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
            | JsType::Undefined => {
                let value = unsafe { call_api_or_panic().jsToString(self.inner, value.as_ptr()) };
                assert!(!value.is_null());
                let value = unsafe { CStr::from_ptr(value) }
                    .to_string_lossy()
                    .to_string();
                Ok(value)
            }
            other => Err(MBError::UnsupportedType(JsType::Boolean, other)),
        }
    }

    /// Get inner ptr of [`JsExecState`]. See [`jsExecState`].
    pub fn as_ptr(&self) -> jsExecState {
        self.inner
    }

    /// Create [`JsExecState`] from ptr.
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_ptr(ptr: jsExecState) -> Self {
        assert!(!ptr.is_null());
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

/// See jsKeys.
pub struct JsKeys {
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

    /// Wraps jsKeys
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_ptr(ptr: *mut jsKeys) -> Self {
        assert!(!ptr.is_null());
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
//         assert!(!ptr.is_null());
//         Self { inner: ptr }
//     }
// }

/// See `jsValue`.
#[derive(Debug, Clone, Copy)]
pub struct JsValue {
    pub(crate) inner: jsValue,
}

impl JsValue {
    /// See jsTypeOf.
    pub fn type_of_(&self) -> JsType {
        let js_type = unsafe { call_api_or_panic().jsTypeOf(self.inner) };
        JsType::from(js_type)
    }
    /// See jsIsNumber.
    pub fn is_number(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsNumber(self.inner) } != 0)
    }
    /// See jsIsString.
    pub fn is_string(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsString(self.inner) } != 0)
    }
    /// See jsIsBoolean.
    pub fn is_boolean(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsBoolean(self.inner) } != 0)
    }
    /// See jsIsObject.
    pub fn is_object(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsObject(self.inner) } != 0)
    }
    /// See jsIsFunction.
    pub fn is_function(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsFunction(self.inner) } != 0)
    }
    /// See jsIsUndefined.
    pub fn is_undefined(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsUndefined(self.inner) } != 0)
    }
    /// See jsIsNull.
    pub fn is_null(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsNull(self.inner) } != 0)
    }
    /// See jsIsArray.
    pub fn is_array(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsArray(self.inner) } != 0)
    }
    /// See jsIsTrue.
    pub fn is_true(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsTrue(self.inner) } != 0)
    }
    /// See jsIsFalse.
    pub fn is_false(&self) -> bool {
        (unsafe { call_api_or_panic().jsIsFalse(self.inner) } != 0)
    }

    /// Get the inner ptr of [`JsValue`]. See [`jsValue`].
    pub fn as_ptr(&self) -> jsValue {
        self.inner
    }

    /// Create [`JsValue`] from ptr.
    /// # Safety
    /// Pointer must not be 0
    pub unsafe fn from_ptr(ptr: jsValue) -> Self {
        assert!(ptr != 0);
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
