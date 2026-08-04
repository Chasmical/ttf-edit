use crate::types::impl_fmt_from_getter;
use std::num::NonZero;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlyphId(NonZero<u32>);

impl GlyphId {
    pub const fn new(value: u32) -> Option<Self> {
        NonZero::new(value).map(Self)
    }

    pub const fn get_nonzero(&self) -> NonZero<u32> {
        self.0
    }
    pub const fn get(&self) -> u32 {
        self.0.get()
    }
}

impl_fmt_from_getter! {
    Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp for GlyphId
}

// Conversions from std integer types to GlyphId
impl TryFrom<u8> for GlyphId {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value as u32).ok_or(())
    }
}
impl TryFrom<u16> for GlyphId {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value as u32).ok_or(())
    }
}
impl TryFrom<u32> for GlyphId {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
impl TryFrom<usize> for GlyphId {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value.try_into().or(Err(()))?).ok_or(())
    }
}

// Conversions from GlyphId to std integer types
impl TryFrom<GlyphId> for u8 {
    type Error = ();
    fn try_from(value: GlyphId) -> Result<Self, Self::Error> {
        value.get().try_into().or(Err(()))
    }
}
impl TryFrom<GlyphId> for u16 {
    type Error = ();
    fn try_from(value: GlyphId) -> Result<Self, Self::Error> {
        value.get().try_into().or(Err(()))
    }
}
impl From<GlyphId> for u32 {
    fn from(value: GlyphId) -> Self {
        value.get()
    }
}
impl From<GlyphId> for usize {
    fn from(value: GlyphId) -> Self {
        value.get() as usize
    }
}
