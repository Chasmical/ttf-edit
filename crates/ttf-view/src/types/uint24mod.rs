use crate::types::impl_fmt_from_getter;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct uint24([u8; 3]);

// TODO: When [u8; 3]'s Default is constified, replace with #[derive_const]
#[allow(clippy::derivable_impls)]
const impl Default for uint24 {
    fn default() -> Self {
        Self([0; 3])
    }
}

impl uint24 {
    pub const BITS: u32 = 24;
    pub const MIN: Self = Self::new(0x000000).unwrap();
    pub const MAX: Self = Self::new(0xFFFFFF).unwrap();

    pub const fn new(num: u32) -> Option<Self> {
        if num <= 0xFFFFFF { Some(unsafe { Self::new_unchecked(num) }) } else { None }
    }
    pub const unsafe fn new_unchecked(num: u32) -> Self {
        debug_assert!(num <= 0xFFFFFF);
        let buf = num.to_be_bytes();
        Self(*buf.last_chunk::<3>().unwrap())
    }

    pub const fn from_be_bytes(bytes: [u8; 3]) -> Self {
        Self(bytes)
    }

    pub const fn get(&self) -> u32 {
        let mut buf = [0; 4];
        buf.last_chunk_mut::<3>().unwrap().copy_from_slice(&self.0);
        u32::from_be_bytes(buf)
    }
}

impl_fmt_from_getter! { Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp for uint24 }

const impl PartialEq<u32> for uint24 {
    fn eq(&self, other: &u32) -> bool {
        self.get().eq(other)
    }
}
const impl PartialOrd<u32> for uint24 {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        Some(self.get().cmp(other))
    }
}

const impl std::str::FromStr for uint24 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str(s).or(Err(())).and_then(Self::try_from)
    }
}
const impl From<u16> for uint24 {
    fn from(value: u16) -> Self {
        Self::new(value as u32).unwrap()
    }
}
const impl TryFrom<u32> for uint24 {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}
const impl From<uint24> for u32 {
    fn from(value: uint24) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv() {
        assert_eq!(uint24::new(0).unwrap(), 0);
        assert_eq!(uint24::new(27).unwrap(), 27);
        assert_eq!(uint24::new(256).unwrap(), 256);
        assert_eq!(uint24::new(0xFFFFFF).unwrap(), 0xFFFFFF);
        assert_eq!(uint24::new(0x01000000), None);
        assert_eq!(uint24::new(0xFFFFFFFF), None);
    }
}
