use crate::util::{Describe, Describer, ListDescriber, MapDescriber, StructDescriber};

pub fn describe_debug<T: Describe + ?Sized>(
    this: &T,
    f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    this.describe(DebugDescriber(f))
}

struct DebugDescriber<'a, 'b: 'a>(&'a mut std::fmt::Formatter<'b>);

impl<'a, 'b: 'a> Describer for DebugDescriber<'a, 'b> {
    type Ok = ();
    type Error = std::fmt::Error;
    type Struct = std::fmt::DebugStruct<'a, 'b>;
    type List = std::fmt::DebugList<'a, 'b>;
    type Map = std::fmt::DebugMap<'a, 'b>;

    fn describe_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }

    fn describe_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }
    fn describe_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        std::fmt::Debug::fmt(&value, self.0)
    }

    fn describe_struct(self, name: &'static str) -> Self::Struct {
        self.0.debug_struct(name)
    }
    fn describe_list(self, _len: Option<usize>) -> Self::List {
        self.0.debug_list()
    }
    fn describe_map(self, _len: Option<usize>) -> Self::Map {
        self.0.debug_map()
    }
}

struct Proxy<'a, T: Describe>(&'a T);

impl<'a, T: Describe> std::fmt::Debug for Proxy<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        describe_debug(self.0, f)
    }
}

impl<'a, 'b> StructDescriber for std::fmt::DebugStruct<'a, 'b> {
    type Ok = ();
    type Error = std::fmt::Error;

    fn field<T: Describe + ?Sized>(&mut self, name: &'static str, value: &T) -> &mut Self {
        Self::field_with(self, name, |f| describe_debug(value, f))
    }
    fn field_fmt<T: Describe + ?Sized>(
        &mut self,
        name: &'static str,
        value: &T,
        fmt: impl FnOnce(&mut std::fmt::Formatter, &T) -> std::fmt::Result,
    ) -> &mut Self {
        Self::field_with(self, name, |f| fmt(f, value))
    }
    fn finish(mut self) -> Result<Self::Ok, Self::Error> {
        Self::finish(&mut self)
    }
}
impl<'a, 'b> ListDescriber for std::fmt::DebugList<'a, 'b> {
    type Ok = ();
    type Error = std::fmt::Error;

    fn entry<T: Describe>(&mut self, item: &T) -> &mut Self {
        Self::entry_with(self, |f| describe_debug(item, f))
    }
    fn entry_fmt<T: Describe>(
        &mut self,
        item: &T,
        fmt: impl FnOnce(&mut std::fmt::Formatter, &T) -> std::fmt::Result,
    ) -> &mut Self {
        Self::entry_with(self, |f| fmt(f, item))
    }
    fn finish(mut self) -> Result<Self::Ok, Self::Error> {
        Self::finish(&mut self)
    }
}
impl<'a, 'b> MapDescriber for std::fmt::DebugMap<'a, 'b> {
    type Ok = ();
    type Error = std::fmt::Error;

    fn entry<K: Describe, V: Describe>(&mut self, key: &K, value: &V) -> &mut Self {
        Self::entry(self, &Proxy(key), &Proxy(value))
    }
    fn finish(mut self) -> Result<Self::Ok, Self::Error> {
        Self::finish(&mut self)
    }
}
