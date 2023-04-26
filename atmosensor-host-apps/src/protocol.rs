use serde::{ser, Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    WontImplement,
    SerializeBufferFull,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        write!(
            f,
            "{}",
            match self {
                WontImplement => "This will not be implemented",
                SerializeBufferFull => "The serialize buffer is full",
            }
        )
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::WontImplement
    }
}

impl serde::ser::StdError for Error {}

pub struct Message {
    pub data: MessageData,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.data.discriminant())?;
        self.data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(unused, non_camel_case_types)]
        struct discriminant;
        impl discriminant {
            const SetMeasurementInterval: u16 = 0x0100;
            const SetAltitude: u16 = 0x0101;
        }
        Ok(match <u16 as Deserialize>::deserialize(deserializer)? {
            discriminant::SetMeasurementInterval => {
                let data = <SetMeasurementInterval as Deserialize>::deserialize(deserializer)?;
                MessageData::SetMeasurementInterval(data)
            }
            discriminant::SetAltitude => {
                let data = <SetAltitude as Deserialize>::deserialize(deserializer)?;
                MessageData::SetAltitude(data)
            }
        }
        .into())
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
#[repr(u16)]
pub enum MessageData {
    SetMeasurementInterval(SetMeasurementInterval) = 0x0100,
    SetAltitude(SetAltitude) = 0x0101,
}

#[derive(Serialize, Deserialize)]
pub struct SetMeasurementInterval {
    pub MeasurementInterval: u16,
}

#[derive(Serialize, Deserialize)]
pub struct SetAltitude {
    pub Altitude: u16,
}

impl MessageData {
    fn discriminant(&self) -> u16 {
        // SAFETY: Because `Self` is marked `repr(u16)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u16` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u16>() }
    }
}

impl From<MessageData> for Message {
    fn from(value: MessageData) -> Self {
        Message { data: value }
    }
}

pub struct Serializer {
    pub output: Vec<u8>,
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(if v { 1 } else { 0 })
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(v.to_be_bytes()[0])
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&[v]);
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::WontImplement)
    }

    #[inline]
    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        Err(Error::WontImplement)
    }

    #[inline]
    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        Err(Error::WontImplement)
    }

    #[inline]
    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: std::fmt::Display,
    {
        Err(Error::WontImplement)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(Error::WontImplement)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(Error::WontImplement)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {}

mod tests {
    use super::{Message, MessageData};

    #[test]
    fn test_serialize() {
        let msg = MessageData::SetMeasurementInterval {
            MeasurementInterval: 0xabcd,
        };
    }
}
