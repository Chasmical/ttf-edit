use crate::types::impl_fmt_from_getter;
use std::{convert::Infallible, ops::FromResidual};

#[derive(Copy, Hash)]
#[derive_const(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlyphId(u16);

impl GlyphId {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn is_def(&self) -> bool {
        self.0 != 0
    }
    pub const fn is_notdef(&self) -> bool {
        self.0 == 0
    }

    pub const fn get(&self) -> u16 {
        self.0
    }

    pub const NOTDEF: Self = Self::new(0);
}

impl_fmt_from_getter! {
    Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp for GlyphId
}

// Conversions from std integer types to GlyphId
const impl From<u8> for GlyphId {
    fn from(value: u8) -> Self {
        Self::new(value as u16)
    }
}
const impl From<u16> for GlyphId {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}
const impl TryFrom<u32> for GlyphId {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self::new(value.try_into().or(Err(()))?))
    }
}
const impl TryFrom<usize> for GlyphId {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self::new(value.try_into().or(Err(()))?))
    }
}

// Conversions from GlyphId to std integer types
const impl TryFrom<GlyphId> for u8 {
    type Error = ();
    fn try_from(value: GlyphId) -> Result<Self, Self::Error> {
        value.get().try_into().or(Err(()))
    }
}
const impl From<GlyphId> for u16 {
    fn from(value: GlyphId) -> Self {
        value.get()
    }
}
const impl From<GlyphId> for u32 {
    fn from(value: GlyphId) -> Self {
        value.get() as u32
    }
}
const impl From<GlyphId> for usize {
    fn from(value: GlyphId) -> Self {
        value.get() as usize
    }
}

// Allow using ? to return .notdef in functions returning GlyphId
const impl FromResidual<Option<Infallible>> for GlyphId {
    fn from_residual(_residual: Option<Infallible>) -> Self {
        Self::NOTDEF
    }
}
