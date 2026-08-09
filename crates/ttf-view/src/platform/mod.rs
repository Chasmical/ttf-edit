mod encodings;
mod languages;

pub use encodings::*;
pub use languages::*;

macro_rules! define_ids {
    ($(
        $(#[$outer:meta])*
        $vis:vis enum $Name:ident {
            $($variant:ident = $value:expr),* $(,)?
        }
    )+) => ($(
        $(#[$outer])*
        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(u16)]
        $vis enum $Name {
            $( $variant = $value, )*
        }

        impl $Name {
            pub const fn new(value: u16) -> Option<Self> {
                Some(match value {
                    $( $value => Self::$variant, )*
                    _ => return None,
                })
            }
            pub const fn get(&self) -> u16 {
                *self as u16
            }
            pub const fn name(&self) -> &'static str {
                match *self {
                    $( Self::$variant => stringify!($variant), )*
                }
            }
        }

        impl std::fmt::Debug for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} ({})", self.get(), self.name())
            }
        }
    )+);
}

pub(crate) use define_ids;

define_ids! {
    pub enum PlatformId {
        Unicode = 0, Macintosh = 1, Iso = 2, Windows = 3, Custom = 4,
    }
}

impl PlatformId {
    pub const fn encoding(&self, enc_id: u16) -> Option<EncodingId> {
        Some(match self {
            Self::Unicode => EncodingId::Unicode(UnicodeEncodingId::new(enc_id)?),
            Self::Macintosh => EncodingId::Macintosh(MacintoshEncodingId::new(enc_id)?),
            Self::Iso => EncodingId::Iso(IsoEncodingId::new(enc_id)?),
            Self::Windows => EncodingId::Windows(WindowsEncodingId::new(enc_id)?),
            Self::Custom => EncodingId::Custom(enc_id.try_into().ok()?),
        })
    }

    pub const fn language(&self, lang_id: u16) -> Option<LanguageId> {
        if lang_id >= 0x8000 {
            return Some(LanguageId::Tagged(lang_id));
        }
        Some(match self {
            Self::Macintosh => LanguageId::Macintosh(lang_id),
            Self::Windows => LanguageId::Windows(lang_id),
            _ => return None,
        })
    }
}
