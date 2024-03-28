/// Deserializer for [`crate::value::JsValue`]
pub mod de;
/// Serializer for [`crate::value::JsValue`]
pub mod ser;

pub use de::from_value;
pub use ser::to_value;
