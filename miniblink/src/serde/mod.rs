/// Deserializer for [`crate::types::JsValue`]
pub mod de;
/// Serializer for [`crate::types::JsValue`]
pub mod ser;

pub use de::from_value;
pub use ser::to_value;
