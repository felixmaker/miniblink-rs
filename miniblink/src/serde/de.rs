use serde::{
    de::{IntoDeserializer, Visitor},
    forward_to_deserialize_any, Deserialize,
};

use crate::{
    error::{MBError, MBResult},
    types::{JsExecState, JsType, JsValue},
};

/// Convert [`JsValue`] to `T` using serde.
pub fn from_value<'de, T>(es: JsExecState, value: JsValue) -> MBResult<T>
where
    T: ?Sized + Deserialize<'de>,
{
    let mut deserializer = Deserializer { es, value };
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

/// Deserializer for [`JsValue`]
pub struct Deserializer {
    es: JsExecState,
    value: JsValue,
}

impl Deserializer {
    /// Create [`Deserializer`] from [`JsValue`]
    pub fn from_value(es: JsExecState, value: JsValue) -> Self {
        Deserializer { es, value }
    }
}

impl<'de, 'a> serde::Deserializer<'de> for &'a mut Deserializer {
    type Error = MBError;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use crate::types::JsType::*;
        match self.value.type_of_() {
            Number => visitor.visit_i32(self.es.to_int(self.value)?),
            String => visitor.visit_string(self.es.to_string(self.value)?),
            Boolean => visitor.visit_bool(self.es.to_boolean(self.value)?),
            Object => self.deserialize_map(visitor),
            Array => self.deserialize_seq(visitor),
            Null | Undefined => visitor.visit_none(),
            _ => unimplemented!(),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes	byte_buf
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.type_of_() {
            JsType::Boolean => visitor.visit_bool(self.es.to_boolean(self.value)?),
            other => Err(MBError::UnsupportedType(JsType::Object, other)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.type_of_() {
            JsType::Null | JsType::Undefined => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.type_of_() {
            JsType::Null | JsType::Undefined => visitor.visit_unit(),
            other => Err(MBError::UnsupportedType(JsType::Object, other)),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use crate::types::JsType::*;
        match self.value.type_of_() {
            Array => {
                let len = self.es.get_length(self.value);
                let value = visitor.visit_seq(MBSeqAccess::new(self, len))?;
                Ok(value)
            }
            other => Err(MBError::UnsupportedType(JsType::Object, other)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.type_of_() {
            JsType::Object => {
                let keys = self.es.get_keys(self.value);
                let value = visitor.visit_map(MBMapAccess::new(self, keys.get_keys()))?;
                Ok(value)
            }
            other => Err(MBError::UnsupportedType(JsType::Object, other)),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value.type_of_() {
            JsType::Object => {
                let keys = self.es.get_keys(self.value).get_keys();
                let value = visitor.visit_enum(MBEnumAccess::new(self, keys[0].clone()))?;
                Ok(value)
            }
            JsType::String => {
                visitor.visit_enum(self.es.to_string(self.value)?.into_deserializer())
            }
            other => Err(MBError::UnsupportedType(JsType::Object, other)),
        }
    }
}

struct MBSeqAccess<'a> {
    de: &'a mut Deserializer,
    pos: usize,
    len: usize,
}

impl<'a> MBSeqAccess<'a> {
    fn new(de: &'a mut Deserializer, len: i32) -> Self {
        Self {
            de,
            pos: 0,
            len: len as usize,
        }
    }
}

impl<'a, 'de> serde::de::SeqAccess<'de> for MBSeqAccess<'a> {
    type Error = MBError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.pos < self.len {
            let v = self.de.es.get_at(self.de.value, self.pos as i32);
            let mut inner = Deserializer::from_value(self.de.es, v);
            self.pos += 1;
            seed.deserialize(&mut inner).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct MBMapAccess<'a> {
    de: &'a mut Deserializer,
    pos: usize,
    keys: Vec<String>,
}

impl<'a> MBMapAccess<'a> {
    fn new(de: &'a mut Deserializer, keys: Vec<String>) -> Self {
        Self { de, keys, pos: 0 }
    }

    fn get_key(&self, pos: usize) -> &str {
        self.keys[pos].as_str()
    }

    fn get_value(&self, pos: usize) -> JsValue {
        self.de.es.get(self.de.value, self.keys[pos].as_str())
    }
}

impl<'a, 'de> serde::de::MapAccess<'de> for MBMapAccess<'a> {
    type Error = MBError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.pos < self.keys.len() {
            let key = self.get_key(self.pos);
            let r = seed.deserialize(key.into_deserializer()).map(Some);
            self.pos += 1;
            r
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let mut inner = Deserializer::from_value(self.de.es, self.get_value(self.pos - 1));
        seed.deserialize(&mut inner)
    }
}

struct MBEnumAccess<'a> {
    de: &'a mut Deserializer,
    key: String,
}

impl<'a> MBEnumAccess<'a> {
    fn new(de: &'a mut Deserializer, key: String) -> Self {
        Self { de, key }
    }
}

impl<'a, 'de> serde::de::EnumAccess<'de> for MBEnumAccess<'a> {
    type Error = MBError;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let key = seed.deserialize(self.key.clone().into_deserializer())?;
        Ok((key, self))
    }
}

impl<'a, 'de> serde::de::VariantAccess<'de> for MBEnumAccess<'a> {
    type Error = MBError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let mut inner =
            Deserializer::from_value(self.de.es, self.de.es.get(self.de.value, &self.key));
        seed.deserialize(&mut inner)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut inner =
            Deserializer::from_value(self.de.es, self.de.es.get(self.de.value, &self.key));
        serde::Deserializer::deserialize_tuple(&mut inner, len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut inner =
            Deserializer::from_value(self.de.es, self.de.es.get(self.de.value, &self.key));

        serde::Deserializer::deserialize_struct(&mut inner, "", fields, visitor)
    }
}
