#![allow(unused)]
use crate::types::{int16, int32, uint16, uint32};

mod debug;
pub use debug::describe_debug;

#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "serde")]
pub use serde::describe_serde;

pub trait Describe {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error>;
}

pub trait Describer {
    type Ok;
    type Error;
    type Struct: StructDescriber<Ok = Self::Ok, Error = Self::Error>;
    type List: ListDescriber<Ok = Self::Ok, Error = Self::Error>;
    type Map: MapDescriber<Ok = Self::Ok, Error = Self::Error>;

    fn describe_bool(self, value: bool) -> Result<Self::Ok, Self::Error>;
    fn describe_u8(self, value: u8) -> Result<Self::Ok, Self::Error>;
    fn describe_u16(self, value: u16) -> Result<Self::Ok, Self::Error>;
    fn describe_u32(self, value: u32) -> Result<Self::Ok, Self::Error>;
    fn describe_u64(self, value: u64) -> Result<Self::Ok, Self::Error>;
    fn describe_u128(self, value: u128) -> Result<Self::Ok, Self::Error>;
    fn describe_i8(self, value: i8) -> Result<Self::Ok, Self::Error>;
    fn describe_i16(self, value: i16) -> Result<Self::Ok, Self::Error>;
    fn describe_i32(self, value: i32) -> Result<Self::Ok, Self::Error>;
    fn describe_i64(self, value: i64) -> Result<Self::Ok, Self::Error>;
    fn describe_i128(self, value: i128) -> Result<Self::Ok, Self::Error>;
    fn describe_f32(self, value: f32) -> Result<Self::Ok, Self::Error>;
    fn describe_f64(self, value: f64) -> Result<Self::Ok, Self::Error>;

    fn describe_char(self, value: char) -> Result<Self::Ok, Self::Error>;
    fn describe_str(self, value: &str) -> Result<Self::Ok, Self::Error>;
    fn describe_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error>;

    // Some other types in serde::Serializer:
    // none
    // some
    // unit
    // unit_struct
    // unit_variant
    // newtype_struct
    // newtype_variant
    // tuple
    // tuple struct
    // tuple variant
    // struct variant

    fn describe_struct(self, name: &'static str) -> Self::Struct;
    fn describe_list(self, len: Option<usize>) -> Self::List;
    fn describe_map(self, len: Option<usize>) -> Self::Map;

    fn describe_list_with<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        Self: Sized,
        I: IntoIterator<Item: Describe>,
    {
        let iter = iter.into_iter();
        let (low, high) = iter.size_hint();
        let len = if Some(low) == high { high } else { None };

        let mut d = self.describe_list(len);
        d.entries(iter);
        d.finish()
    }
}
pub trait StructDescriber {
    type Ok;
    type Error;

    fn field<T: Describe + ?Sized>(&mut self, name: &'static str, value: &T) -> &mut Self;
    fn field_fmt<T: Describe + ?Sized>(
        &mut self,
        name: &'static str,
        value: &T,
        _fmt: impl FnOnce(&mut std::fmt::Formatter, &T) -> std::fmt::Result,
    ) -> &mut Self {
        self.field(name, value)
    }
    fn finish(self) -> Result<Self::Ok, Self::Error>;
}
pub trait ListDescriber {
    type Ok;
    type Error;

    fn entry<T: Describe>(&mut self, item: &T) -> &mut Self;
    fn entry_fmt<T: Describe>(
        &mut self,
        item: &T,
        _fmt: impl FnOnce(&mut std::fmt::Formatter, &T) -> std::fmt::Result,
    ) -> &mut Self {
        self.entry(item)
    }
    fn entries<T: Describe, I: IntoIterator<Item = T>>(&mut self, items: I) -> &mut Self {
        for item in items {
            self.entry(&item);
        }
        self
    }
    fn finish(self) -> Result<Self::Ok, Self::Error>;
}
pub trait MapDescriber {
    type Ok;
    type Error;

    fn entry<K: Describe, V: Describe>(&mut self, key: &K, value: &V) -> &mut Self;
    fn entries<K: Describe, V: Describe, I: IntoIterator<Item = (K, V)>>(
        &mut self,
        entries: I,
    ) -> &mut Self {
        for (key, value) in entries {
            self.entry(&key, &value);
        }
        self
    }
    fn finish(self) -> Result<Self::Ok, Self::Error>;
}

macro_rules! describe_types {
    ($( $ty:ty: $fn:ident ),* $(,)?) => ($(
        impl Describe for $ty {
            fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
                d.$fn(*self)
            }
        }
    )*);
    ($( $ty:ty as $as:ty ),* $(,)?) => ($(
        impl Describe for $ty {
            fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
                <$as>::from(*self).describe(d)
            }
        }
    )*);
}

describe_types! {
    bool: describe_bool,
    u8: describe_u8,
    u16: describe_u16,
    u32: describe_u32,
    u64: describe_u64,
    u128: describe_u128,
    i8: describe_i8,
    i16: describe_i16,
    i32: describe_i32,
    i64: describe_i64,
    i128: describe_i128,
    f32: describe_f32,
    f64: describe_f64,
    char: describe_char,
    &str: describe_str,
}
describe_types! {
    uint16 as u16,
    uint32 as u32,
    int16 as i16,
    int32 as i32,
}

impl<T: Describe> Describe for &T {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        Describe::describe(*self, d)
    }
}
impl<T: Describe> Describe for [T] {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        let mut list = d.describe_list(Some(self.len()));
        list.entries(self.iter());
        list.finish()
    }
}
impl Describe for String {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        d.describe_str(self.as_str())
    }
}

macro_rules! describe {
    ( $describer:ident, $describee:ident, $field:ident ) => {
        $crate::util::StructDescriber::field(&mut $describer, stringify!($field), &$describee.$field)
    };
    ( $describer:ident, $describee:ident, $field:ident: $format:literal ) => {
        $crate::util::StructDescriber::field_fmt(&mut $describer, stringify!($field), &$describee.$field, |f, x| write!(f, $format, x))
    };

    (
        $describer:ident, $describee:ident {
            $( $field:ident $(: $format:literal )? ),* $(,)?
        }
    ) => {
        $( $crate::util::describe!($describer, $describee, $field $(: $format )? ); )*
    };

    (
        $describer:ident, $describee:ident as $struct_name:literal {
            $( $field:ident $(: $format:literal )? ),* $(,)?
        }
    ) => {{
        let mut d = $crate::util::Describer::describe_struct($describer, $struct_name);
        $( $crate::util::describe!(d, $describee, $field $(: $format )? ); )*
        $crate::util::StructDescriber::finish(d)
    }};
}

macro_rules! describe_impl {
    ($( $(#[$outer:meta])* $trait:ident ),* for $ty:ty) => {
        $( $(#[$outer])* $crate::util::describe_impl! { @impl $trait for $ty } )*
    };
    (@impl Debug for $ty:ty) => {
        impl std::fmt::Debug for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                $crate::util::describe_debug(self, f)
            }
        }
    };
    (@impl Serialize for $ty:ty) => {
        #[cfg(feature = "serde")]
        impl serde::Serialize for $ty {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                $crate::util::describe_serde(self, s)
            }
        }
    };
}

pub(crate) use {describe, describe_impl};
