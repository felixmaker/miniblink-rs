use serde::Serialize;

use crate::{
    error::{MBError, MBResult},
    value::{JsExecState, JsValue},
};

pub struct Serializer {
    es: JsExecState,
    value: JsValue,
}

pub fn to_value<T>(es: JsExecState, value: &T) -> MBResult<JsValue>
where
    T: ?Sized + Serialize,
{
    let mut p = Serializer {
        es,
        value: es.empty_object(),
    };
    value.serialize(&mut p)?;
    Ok(p.value)
}

impl<'a> serde::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = MBError;

    type SerializeSeq = MBSerializeSeq<'a>;
    type SerializeTuple = MBSerializeSeq<'a>;
    type SerializeTupleStruct = MBSerializeSeq<'a>;
    type SerializeTupleVariant = MBSerializeSeq<'a>;
    type SerializeMap = MBSerializeMap<'a>;
    type SerializeStruct = MBSerializeMap<'a>;
    type SerializeStructVariant = MBSerializeMap<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.value = self.es.boolean(v);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.value = self.es.int(v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        match i32::try_from(v) {
            Ok(v) => self.value = self.es.int(v),
            Err(_) => Err(MBError::FailedToConvert("i32".into(), "i64".into()))?,
        }
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match i32::try_from(v) {
            Ok(v) => self.value = self.es.int(v),
            Err(_) => Err(MBError::FailedToConvert("i32".into(), "u64".into()))?,
        }
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.value = self.es.double(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.value = self.es.string(v);
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.value = self.es.null();
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.value = self.es.empty_object();
        self.es.set(self.value, variant, to_value(self.es, value)?);
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let array = self.es.empty_array();
        self.es.set_length(array, len.unwrap_or(0) as i32);
        let seq = MBSerializeSeq::new(self, array);
        Ok(seq)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let array = self.es.empty_array();
        self.es.set_length(array, len as i32);
        Ok(MBSerializeSeq::new_with_wrapper(self, array, variant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let object = self.es.empty_object();
        Ok(MBSerializeMap::new(self, object))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let object = self.es.empty_object();
        Ok(MBSerializeMap::new_with_wrapper(self, object, variant))
    }
}

pub struct MBSerializeSeq<'a> {
    ser: &'a mut Serializer,
    array: JsValue,
    pos: i32,
    wrapper: Option<&'static str>,
}

impl<'a> MBSerializeSeq<'a> {
    fn new(ser: &'a mut Serializer, array: JsValue) -> Self {
        Self {
            ser,
            array,
            pos: 0,
            wrapper: None,
        }
    }

    fn new_with_wrapper(ser: &'a mut Serializer, array: JsValue, wrapper: &'static str) -> Self {
        Self {
            ser,
            array,
            pos: 0,
            wrapper: Some(wrapper),
        }
    }
}

impl<'a> serde::ser::SerializeSeq for MBSerializeSeq<'a> {
    type Ok = ();

    type Error = MBError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.ser
            .es
            .set_at(self.array, self.pos, to_value(self.ser.es, value)?);
        self.pos += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.value = self.array;
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTuple for MBSerializeSeq<'a> {
    type Ok = ();

    type Error = MBError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'a> serde::ser::SerializeTupleStruct for MBSerializeSeq<'a> {
    type Ok = ();

    type Error = MBError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'a> serde::ser::SerializeTupleVariant for MBSerializeSeq<'a> {
    type Ok = ();

    type Error = MBError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let obj = self.ser.es.empty_object();
        self.ser.es.set(obj, self.wrapper.unwrap(), self.array);
        self.ser.value = obj;
        Ok(())
    }
}

pub struct MBSerializeMap<'a> {
    ser: &'a mut Serializer,
    object: JsValue,
    wrapper: Option<&'static str>,
}

impl<'a> MBSerializeMap<'a> {
    fn new(ser: &'a mut Serializer, object: JsValue) -> Self {
        Self {
            ser,
            object,
            wrapper: None,
        }
    }

    fn new_with_wrapper(ser: &'a mut Serializer, object: JsValue, wrapper: &'static str) -> Self {
        Self {
            ser,
            object,
            wrapper: Some(wrapper),
        }
    }
}

impl<'a> serde::ser::SerializeMap for MBSerializeMap<'a> {
    type Ok = ();

    type Error = MBError;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> serde::ser::SerializeStruct for MBSerializeMap<'a> {
    type Ok = ();

    type Error = MBError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.ser
            .es
            .set(self.object, key, to_value(self.ser.es, value)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.value = self.object;
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStructVariant for MBSerializeMap<'a> {
    type Ok = ();

    type Error = MBError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        serde::ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let obj = self.ser.es.empty_object();
        self.ser.es.set(obj, self.wrapper.unwrap(), self.object);
        self.ser.value = obj;
        Ok(())
    }
}
