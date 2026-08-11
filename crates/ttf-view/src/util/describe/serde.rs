use crate::util::{Describe, Describer, ListDescriber, MapDescriber, StructDescriber};
use serde::{
    Serialize, Serializer,
    ser::{SerializeMap, SerializeSeq, SerializeStruct},
};

pub fn describe_serde<T: Describe + ?Sized, S: Serializer>(
    this: &T,
    s: S,
) -> Result<S::Ok, S::Error> {
    this.describe(SerdeDescriber(s))
}

struct SerdeDescriber<S: Serializer>(S);
struct SerdeStructDescriber<S: Serializer>(Result<S::SerializeStruct, S::Error>);
struct SerdeListDescriber<S: Serializer>(Result<S::SerializeSeq, S::Error>);
struct SerdeMapDescriber<S: Serializer>(Result<S::SerializeMap, S::Error>);

impl<S: Serializer> Describer for SerdeDescriber<S> {
    type Ok = S::Ok;
    type Error = S::Error;
    type Struct = SerdeStructDescriber<S>;
    type List = SerdeListDescriber<S>;
    type Map = SerdeMapDescriber<S>;

    fn describe_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_bool(value)
    }
    fn describe_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u8(value)
    }
    fn describe_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u16(value)
    }
    fn describe_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u32(value)
    }
    fn describe_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u64(value)
    }
    fn describe_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u128(value)
    }
    fn describe_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i8(value)
    }
    fn describe_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i16(value)
    }
    fn describe_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i32(value)
    }
    fn describe_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i64(value)
    }
    fn describe_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i128(value)
    }
    fn describe_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_f32(value)
    }
    fn describe_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_f64(value)
    }

    fn describe_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_char(value)
    }
    fn describe_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_str(value)
    }
    fn describe_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_bytes(value)
    }

    fn describe_struct(self, name: &'static str) -> Self::Struct {
        SerdeStructDescriber(self.0.serialize_struct(name, 10))
    }
    fn describe_list(self, len: Option<usize>) -> Self::List {
        SerdeListDescriber(self.0.serialize_seq(len))
    }
    fn describe_map(self, len: Option<usize>) -> Self::Map {
        SerdeMapDescriber(self.0.serialize_map(len))
    }
}

struct Proxy<'a, T: Describe + ?Sized>(&'a T);

impl<'a, T: Describe + ?Sized> Serialize for Proxy<'a, T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        describe_serde(self.0, s)
    }
}

impl<S: Serializer> StructDescriber for SerdeStructDescriber<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn field<T: Describe + ?Sized>(&mut self, name: &'static str, value: &T) -> &mut Self {
        if let Ok(x) = self.0.as_mut() {
            if let Err(e) = x.serialize_field(name, &Proxy(value)) {
                self.0 = Err(e);
            }
        }
        self
    }
    fn finish(self) -> Result<Self::Ok, Self::Error> {
        self.0.and_then(|x| x.end())
    }
}
impl<S: Serializer> ListDescriber for SerdeListDescriber<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn entry<T: Describe>(&mut self, item: &T) -> &mut Self {
        if let Ok(x) = self.0.as_mut() {
            if let Err(e) = x.serialize_element(&Proxy(item)) {
                self.0 = Err(e);
            }
        }
        self
    }
    fn finish(self) -> Result<Self::Ok, Self::Error> {
        self.0.and_then(|x| x.end())
    }
}
impl<S: Serializer> MapDescriber for SerdeMapDescriber<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn entry<K: Describe, V: Describe>(&mut self, key: &K, value: &V) -> &mut Self {
        if let Ok(x) = self.0.as_mut() {
            if let Err(e) = x.serialize_entry(&Proxy(key), &Proxy(value)) {
                self.0 = Err(e);
            }
        }
        self
    }
    fn finish(self) -> Result<Self::Ok, Self::Error> {
        self.0.and_then(|x| x.end())
    }
}
