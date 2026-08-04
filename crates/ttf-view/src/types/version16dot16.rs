use std::{fmt, num::ParseIntError};

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Version16Dot16([u8; 4]);

impl Version16Dot16 {
    pub const fn new(major: u16, minor: u8) -> Option<Self> {
        if minor <= 9 { Some(unsafe { Self::new_unchecked(major, minor) }) } else { None }
    }
    pub const unsafe fn new_unchecked(major: u16, minor: u8) -> Self {
        debug_assert!(minor <= 9);
        let raw = ((major as u32) << 16) | ((minor as u32) << 12);
        Self(raw.to_be_bytes())
    }

    pub const fn from_be_bytes(bytes: [u8; 4]) -> Option<Self> {
        match bytes {
            [_, _, x @ 0x00..=0x90, 0] if (x & 0x0F) == 0 => Some(Self(bytes)),
            _ => None,
        }
    }
    pub const unsafe fn from_be_bytes_unchecked(bytes: [u8; 4]) -> Self {
        debug_assert!(Self::from_be_bytes(bytes).is_some());
        Self(bytes)
    }
    pub const fn to_be_bytes(self) -> [u8; 4] {
        self.0
    }

    pub const fn major(&self) -> u16 {
        u16::from_be_bytes(*self.0.first_chunk::<2>().unwrap())
    }
    pub const fn minor(&self) -> u8 {
        self.0[2] >> 4
    }
    pub const fn tuple(&self) -> (u16, u8) {
        (self.major(), self.minor())
    }
}

// TODO: When [u8; 4]'s Default is constified, replace with #[derive_const]
#[allow(clippy::derivable_impls)]
const impl Default for Version16Dot16 {
    fn default() -> Self {
        Self([0; 4])
    }
}

impl fmt::Debug for Version16Dot16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl fmt::Display for Version16Dot16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major(), self.minor())
    }
}
impl fmt::LowerHex for Version16Dot16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        u32::from_be_bytes(self.to_be_bytes()).fmt(f)
    }
}
impl fmt::UpperHex for Version16Dot16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        u32::from_be_bytes(self.to_be_bytes()).fmt(f)
    }
}

// TODO: When ParseIntError's Clone + PartialEq + Eq are constified, replace with #[derive_const]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseVersion16Dot16Error {
    NoDot,
    Invalid,
    Number(ParseIntError),
}

impl std::str::FromStr for Version16Dot16 {
    type Err = ParseVersion16Dot16Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, minor) = s.split_once('.').ok_or(ParseVersion16Dot16Error::NoDot)?;
        Self::new(
            major.parse().map_err(ParseVersion16Dot16Error::Number)?,
            minor.parse().map_err(ParseVersion16Dot16Error::Number)?,
        )
        .ok_or(ParseVersion16Dot16Error::Invalid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv() {
        fn version_to_raw(major: u16, minor: u8) -> Option<u32> {
            Version16Dot16::new(major, minor).map(|x| u32::from_be_bytes(x.to_be_bytes()))
        }

        assert_eq!(version_to_raw(0, 0), Some(0x00000000));
        assert_eq!(version_to_raw(0, 5), Some(0x00005000));
        assert_eq!(version_to_raw(1, 0), Some(0x00010000));
        assert_eq!(version_to_raw(1, 1), Some(0x00011000));
        assert_eq!(version_to_raw(65535, 9), Some(0xFFFF9000));

        assert_eq!(version_to_raw(0, 10), None);
        assert_eq!(version_to_raw(29, 199), None);
    }
}
