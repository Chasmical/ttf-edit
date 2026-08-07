use std::borrow::Cow;

#[cfg(feature = "non-standard-encodings")]
use encoding_rs::{BIG5, EUC_KR, Encoding as EncodingRs, GB18030, SHIFT_JIS};

macro_rules! define_ids {
    ($(
        $(#[$outer:meta])*
        $vis:vis struct $Name:ident {
            $($field:ident = $value:expr),* $(,)?
        }
    )+) => ($(
        $(#[$outer])*
        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
        $vis struct $Name(u16);

        #[allow(non_upper_case_globals)]
        impl $Name {
            pub const fn new(value: u16) -> Self {
                Self(value)
            }
            pub const fn get(&self) -> u16 {
                self.0
            }
            pub const fn name(&self) -> Option<&'static str> {
                Some(match *self {
                    $( Self::$field => stringify!($field), )*
                    _ => return None,
                })
            }

            $( pub const $field: Self = Self($value); )*
        }

        impl std::fmt::Debug for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} ({})", self.get(), self.name().unwrap_or("Unknown"))
            }
        }
    )+);
}

define_ids! {
    pub struct PlatformId {
        Unicode = 0, Macintosh = 1, Iso = 2, Windows = 3, Custom = 4,
    }
    pub struct UnicodeEncodingId {
        Unicode1_0 = 0, Unicode1_1 = 1, IsoIec10646 = 2, Unicode2_0BmpOnly = 3, Unicode2_0Full = 4,
        UnicodeVariations = 5, UnicodeFull = 6,
    }
    pub struct MacintoshEncodingId {
        Roman = 0, Japanese = 1, ChineseTraditional = 2, Korean = 3, Arabic = 4, Hebrew = 5,
        Greek = 6, Russian = 7, RSymbol = 8, Devanagari = 9, Gurmukhi = 10, Gujarati = 11,
        Odia = 12, Bangla = 13, Tamil = 14, Telugu = 15, Kannada = 16, Malayalam = 17,
        Sinhalese = 18, Burmese = 19, Khmer = 20, Thai = 21, Laotian = 22, Georgian = 23,
        Armenian = 24, ChineseSimplified = 25, Tibetan = 26, Mongolian = 27, Geez = 28,
        Slavic = 29, Vietnamese = 30, Sindhi = 31, Uninterpreted = 32,
    }
    pub struct IsoEncodingId {
        SevenBitAscii = 0, Iso10646 = 1, Iso8859_1 = 2,
    }
    pub struct WindowsEncodingId {
        Symbol = 0, UnicodeBmp = 1, ShiftJis = 2, Prc = 3, Big5 = 4, Wansung = 5, Johab = 6,
        Reserved7 = 7, Reserved8 = 8, Reserved9 = 9, UnicodeFull = 10,
    }
}

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Encoding {
    platform_id: PlatformId,
    encoding_id: u16,
}

#[derive(Debug, thiserror::Error)]
#[derive_const(Clone, PartialEq, Eq)]
pub enum EncodingError {
    #[error("unknown platform id")]
    UnknownPlatform,
    #[error("unknown encoding id")]
    UnknownEncoding,
    #[error("string data could not be decoded")]
    MalformedString,
    #[error("this encoding is not implemented")]
    Unimplemented,
}

impl Encoding {
    pub const fn new(platform_id: u16, encoding_id: u16) -> Self {
        Self { platform_id: PlatformId::new(platform_id), encoding_id }
    }

    pub const fn platform_id(&self) -> PlatformId {
        self.platform_id
    }
    pub const fn encoding_id(&self) -> u16 {
        self.encoding_id
    }

    pub const fn platform_name(&self) -> Option<&'static str> {
        self.platform_id.name()
    }
    pub fn encoding_name(&self) -> Option<Cow<'static, str>> {
        Some(Cow::Borrowed(match self.platform_id {
            PlatformId::Unicode => UnicodeEncodingId::new(self.encoding_id).name()?,
            PlatformId::Macintosh => MacintoshEncodingId::new(self.encoding_id).name()?,
            PlatformId::Iso => IsoEncodingId::new(self.encoding_id).name()?,
            PlatformId::Windows => WindowsEncodingId::new(self.encoding_id).name()?,
            PlatformId::Custom => {
                let charset: u8 = self.encoding_id.try_into().ok()?;
                return Some(Cow::Owned(format!("Charset {}", charset)));
            },
            _ => return None,
        }))
    }

    pub fn decode_utf16be(&self, bytes: &[u8]) -> Result<String, EncodingError> {
        use EncodingError::*;

        // fn decode_utf8(bytes: &[u8]) -> Result<String, EncodingError> {
        //     str::from_utf8(bytes).map(|s| s.to_owned()).map_err(|_| Malformed)
        // }
        fn decode_utf16be(bytes: &[u8]) -> Result<String, EncodingError> {
            String::from_utf16be(bytes).map_err(|_| MalformedString)
        }

        #[cfg(feature = "non-standard-encodings")]
        fn decode(bytes: &[u8], encoding: &'static EncodingRs) -> Result<String, EncodingError> {
            let decoded = encoding.decode_without_bom_handling_and_without_replacement(bytes);
            decoded.ok_or(MalformedString).map(|x| x.into_owned())
        }

        #[allow(clippy::match_overlapping_arm)]
        match self.platform_id {
            PlatformId::Unicode => match self.encoding_id {
                ..=6 => decode_utf16be(bytes), // All Unicode encodings
                _ => Err(UnknownEncoding),
            },

            PlatformId::Macintosh => match self.encoding_id {
                0 => Ok(decode_macos_roman(bytes)), // Mac OS Roman
                ..=32 => Err(Unimplemented),
                _ => Err(UnknownEncoding),
            },

            PlatformId::Iso => Err(Unimplemented),

            PlatformId::Windows => {
                match self.encoding_id {
                    0 | 1 | 10 => decode_utf16be(bytes), // Symbol | UnicodeBmp | UnicodeFull

                    #[cfg(feature = "non-standard-encodings")]
                    2 => decode(bytes, SHIFT_JIS), // ShiftJIS / CP 932
                    #[cfg(feature = "non-standard-encodings")]
                    3 => decode(bytes, GB18030), // PRC / GB 18030 / CP 54936
                    #[cfg(feature = "non-standard-encodings")]
                    4 => decode(bytes, BIG5), // Big5 / CP 950
                    #[cfg(feature = "non-standard-encodings")]
                    5 => decode(bytes, EUC_KR), // Wansung / EUC-KR / CP 949

                    // #[cfg(feature = "non-standard-encodings")]
                    // 6 => , // Johab / KS C 5601-1992 / CP 1361
                    ..=10 => Err(Unimplemented),
                    _ => Err(UnknownEncoding),
                }
            },

            PlatformId::Custom => match self.encoding_id {
                ..=255 => Err(Unimplemented),
                _ => Err(UnknownEncoding),
            },

            _ => Err(UnknownPlatform),
        }
    }
}

impl From<UnicodeEncodingId> for Encoding {
    fn from(value: UnicodeEncodingId) -> Self {
        Self::new(PlatformId::Unicode.get(), value.get())
    }
}
impl From<MacintoshEncodingId> for Encoding {
    fn from(value: MacintoshEncodingId) -> Self {
        Self::new(PlatformId::Macintosh.get(), value.get())
    }
}
impl From<IsoEncodingId> for Encoding {
    fn from(value: IsoEncodingId) -> Self {
        Self::new(PlatformId::Iso.get(), value.get())
    }
}
impl From<WindowsEncodingId> for Encoding {
    fn from(value: WindowsEncodingId) -> Self {
        Self::new(PlatformId::Windows.get(), value.get())
    }
}

// We'll include the MacOS Roman table by default, since it's used often.
fn decode_macos_roman(bytes: &[u8]) -> String {
    bytes.iter().copied().map(map_macos_roman).collect()
}
fn map_macos_roman(byte: u8) -> char {
    if byte <= 0x7F {
        byte as char
    } else {
        unsafe {
            char::from_u32_unchecked(*MACOS_ROMAN.get_unchecked((byte - 0x80) as usize) as u32)
        }
    }
}

const MACOS_ROMAN: [u16; 128] = [
    196, 197, 199, 201, 209, 214, 220, 225, 224, 226, 228, 227, 229, 231, 233, 232, 234, 235, 237,
    236, 238, 239, 241, 243, 242, 244, 246, 245, 250, 249, 251, 252, 8224, 176, 162, 163, 167,
    8226, 182, 223, 174, 169, 8482, 180, 168, 8800, 198, 216, 8734, 177, 8804, 8805, 165, 181,
    8706, 8721, 8719, 960, 8747, 170, 186, 937, 230, 248, 191, 161, 172, 8730, 402, 8776, 8710,
    171, 187, 8230, 160, 192, 195, 213, 338, 339, 8211, 8212, 8220, 8221, 8216, 8217, 247, 9674,
    255, 376, 8260, 8364, 8249, 8250, 64257, 64258, 8225, 183, 8218, 8222, 8240, 194, 202, 193,
    203, 200, 205, 206, 207, 204, 211, 212, 63743, 210, 218, 219, 217, 305, 710, 732, 175, 728,
    729, 730, 184, 733, 731, 711,
];
