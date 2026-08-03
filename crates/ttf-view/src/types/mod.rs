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
// TODO: pub type Offset24 = uint24;
pub type Offset32 = uint32;

// TODO: Version16Dot16
pub type Tag = [u8; 4]; // TODO: Tag
// TODO: LongDateTime
// TODO: Fixed
// TODO: F2DOT14
