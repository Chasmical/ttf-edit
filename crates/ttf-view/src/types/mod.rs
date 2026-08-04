#![allow(non_camel_case_types)]
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
macro_rules! impl_fmt_from_getter {
    ($($Trait:ident),* for $Struct:ty) => ($(
        impl std::fmt::$Trait for $Struct {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::$Trait::fmt(&self.get(), f)
            }
        }
    )*);
}
pub(crate) use impl_fmt_from_getter;
