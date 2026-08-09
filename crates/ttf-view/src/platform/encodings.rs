use crate::platform::{PlatformId, define_ids};
use std::borrow::Cow;

#[cfg(feature = "non-standard-encodings")]
use encoding_rs::{BIG5, EUC_KR, Encoding as EncodingRs, GB18030, SHIFT_JIS};

#[repr(u16)]
pub enum EncodingId {
    Unicode(UnicodeEncodingId) = 0,
    Macintosh(MacintoshEncodingId) = 1,
    Iso(IsoEncodingId) = 2,
    Windows(WindowsEncodingId) = 3,
    Custom(u8) = 4,
}

define_ids! {
    pub enum UnicodeEncodingId {
        Unicode1_0 = 0, Unicode1_1 = 1, IsoIec10646 = 2, Unicode2_0BmpOnly = 3, Unicode2_0Full = 4,
        UnicodeVariations = 5, UnicodeFull = 6,
    }
    pub enum MacintoshEncodingId {
        Roman = 0, Japanese = 1, ChineseTraditional = 2, Korean = 3, Arabic = 4, Hebrew = 5,
        Greek = 6, Russian = 7, RSymbol = 8, Devanagari = 9, Gurmukhi = 10, Gujarati = 11,
        Odia = 12, Bangla = 13, Tamil = 14, Telugu = 15, Kannada = 16, Malayalam = 17,
        Sinhalese = 18, Burmese = 19, Khmer = 20, Thai = 21, Laotian = 22, Georgian = 23,
        Armenian = 24, ChineseSimplified = 25, Tibetan = 26, Mongolian = 27, Geez = 28,
        Slavic = 29, Vietnamese = 30, Sindhi = 31, Uninterpreted = 32,
    }
    pub enum IsoEncodingId {
        SevenBitAscii = 0, Iso10646 = 1, Iso8859_1 = 2,
    }
    pub enum WindowsEncodingId {
        Symbol = 0, UnicodeBmp = 1, ShiftJis = 2, Prc = 3, Big5 = 4, Wansung = 5, Johab = 6,
        Reserved7 = 7, Reserved8 = 8, Reserved9 = 9, UnicodeFull = 10,
    }
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

impl EncodingId {
    pub const fn new(platform_id: u16, encoding_id: u16) -> Result<Self, EncodingError> {
        let platform = PlatformId::new(platform_id).ok_or(EncodingError::UnknownPlatform)?;
        platform.encoding(encoding_id).ok_or(EncodingError::UnknownEncoding)
    }

    pub fn get(&self) -> u16 {
        match self {
            Self::Unicode(_) => 0,
            Self::Macintosh(_) => 1,
            Self::Iso(_) => 2,
            Self::Windows(_) => 3,
            Self::Custom(_) => 4,
        }
    }
    pub fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Unicode(x) => x.name(),
            Self::Macintosh(x) => x.name(),
            Self::Iso(x) => x.name(),
            Self::Windows(x) => x.name(),
            Self::Custom(charset) => {
                return Cow::Owned(format!("Charset {}", charset));
            },
        })
    }

    pub fn decode_utf16be(&self, bytes: &[u8]) -> Result<String, EncodingError> {
        use EncodingError::*;

        fn decode_utf16be(bytes: &[u8]) -> Result<String, EncodingError> {
            String::from_utf16be(bytes).map_err(|_| MalformedString)
        }

        #[cfg(feature = "non-standard-encodings")]
        fn decode(bytes: &[u8], encoding: &'static EncodingRs) -> Result<String, EncodingError> {
            let decoded = encoding.decode_without_bom_handling_and_without_replacement(bytes);
            decoded.ok_or(MalformedString).map(|x| x.into_owned())
        }

        match self {
            // All Unicode encodings use UTF-16BE
            Self::Unicode(_) => decode_utf16be(bytes),

            Self::Macintosh(encoding) => {
                use MacintoshEncodingId::*;

                match encoding {
                    Roman => Ok(decode_macos_roman(bytes)), // Mac OS Roman
                    _ => Err(Unimplemented),
                }
            },

            Self::Iso(_) => Err(Unimplemented),

            Self::Windows(encoding) => {
                use WindowsEncodingId::*;

                match encoding {
                    Symbol | UnicodeBmp | UnicodeFull => decode_utf16be(bytes),

                    #[cfg(feature = "non-standard-encodings")]
                    ShiftJis => decode(bytes, SHIFT_JIS), // aka. CP 932
                    #[cfg(feature = "non-standard-encodings")]
                    Prc => decode(bytes, GB18030), // aka. GB 18030 / CP 54936
                    #[cfg(feature = "non-standard-encodings")]
                    Big5 => decode(bytes, BIG5), // aka. CP 950
                    #[cfg(feature = "non-standard-encodings")]
                    Wansung => decode(bytes, EUC_KR), // aka. EUC-KR / CP 949

                    // #[cfg(feature = "non-standard-encodings")]
                    // Johab => , // aka. KS C 5601-1992 / CP 1361
                    _ => Err(Unimplemented),
                }
            },

            Self::Custom(_) => Err(Unimplemented),
        }
    }
}

impl From<UnicodeEncodingId> for EncodingId {
    fn from(value: UnicodeEncodingId) -> Self {
        Self::Unicode(value)
    }
}
impl From<MacintoshEncodingId> for EncodingId {
    fn from(value: MacintoshEncodingId) -> Self {
        Self::Macintosh(value)
    }
}
impl From<IsoEncodingId> for EncodingId {
    fn from(value: IsoEncodingId) -> Self {
        Self::Iso(value)
    }
}
impl From<WindowsEncodingId> for EncodingId {
    fn from(value: WindowsEncodingId) -> Self {
        Self::Windows(value)
    }
}

// We'll include the MacOS Roman table by default, since it's used often.
fn decode_macos_roman(bytes: &[u8]) -> String {
    bytes.iter().copied().map(map_macos_roman).collect()
}
fn map_macos_roman(byte: u8) -> char {
    if byte < 0x80 {
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
