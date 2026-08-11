#![allow(non_camel_case_types)]
use crate::util::{Describe, Describer};
use zerocopy::network_endian::{I16, I32, U16, U32};

pub type int16 = I16;
pub type uint16 = U16;
pub type int32 = I32;
pub type uint32 = U32;

pub type FWORD = int16;
pub type UFWORD = uint16;

// TODO: Is there any point in this distinction?
pub type Offset8 = u8;
pub type Offset16 = uint16;
pub type Offset24 = uint24;
pub type Offset32 = uint32;

mod fixed_point;
mod longdatetime;
mod tag;
mod uint24mod;
mod version16dot16;

pub use fixed_point::*;
pub use longdatetime::*;
pub use tag::*;
pub use uint24mod::*;
pub use version16dot16::*;

// Utility macro for formatting wrapper types, like uint24
macro_rules! impl_fmt_with {
    ($($Trait:ident),*: |$arg:ident: &$Name:ty| $closure:expr) => ($(
        impl std::fmt::$Trait for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let $arg = self;
                std::fmt::$Trait::fmt(&$closure, f)
            }
        }
    )*);
    ($($Trait:ident),*: |$arg:ident: &$Name:ty, $f:ident| $closure:expr) => ($(
        impl std::fmt::$Trait for $Name {
            fn fmt(&self, $f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let $arg = self;
                std::fmt::$Trait::fmt(&$closure, $f)
            }
        }
    )*);
}
pub(crate) use impl_fmt_with;

impl Describe for Tag {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        d.describe_str(self.as_str())
    }
}
impl Describe for uint24 {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        d.describe_u32(self.get())
    }
}
impl Describe for Fixed {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        d.describe_f64(self.get())
    }
}
impl Describe for F2DOT14 {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        d.describe_f32(self.get())
    }
}
impl Describe for Version16Dot16 {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        d.describe_str(&self.to_string())
    }
}
impl Describe for LongDateTime {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        if let Some(datetime) = self.datetime() {
            d.describe_str(&datetime.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
        } else {
            d.describe_i64(self.epoch_seconds())
        }
    }
}
