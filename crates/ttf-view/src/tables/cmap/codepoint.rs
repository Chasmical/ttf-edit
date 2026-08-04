use crate::types::impl_fmt_from_getter;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Codepoint(u32);

impl Codepoint {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
    pub const fn get(&self) -> u32 {
        self.0
    }
}

impl_fmt_from_getter! {
    Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp for Codepoint
}

// Conversions from std integer types to Codepoint
impl From<u8> for Codepoint {
    fn from(value: u8) -> Self {
        Self::new(value as u32)
    }
}
impl From<u16> for Codepoint {
    fn from(value: u16) -> Self {
        Self::new(value as u32)
    }
}
impl From<u32> for Codepoint {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}
impl TryFrom<usize> for Codepoint {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        value.try_into().or(Err(())).map(Self::new)
    }
}

// Conversions from Codepoint to std integer types
impl TryFrom<Codepoint> for u8 {
    type Error = ();
    fn try_from(value: Codepoint) -> Result<Self, Self::Error> {
        value.get().try_into().or(Err(()))
    }
}
impl TryFrom<Codepoint> for u16 {
    type Error = ();
    fn try_from(value: Codepoint) -> Result<Self, Self::Error> {
        value.get().try_into().or(Err(()))
    }
}
impl From<Codepoint> for u32 {
    fn from(value: Codepoint) -> Self {
        value.get()
    }
}
impl From<Codepoint> for usize {
    fn from(value: Codepoint) -> Self {
        value.get() as usize
    }
}
