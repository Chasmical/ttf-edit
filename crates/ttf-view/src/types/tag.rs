use std::fmt;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Tag([TagByte; 4]);

// TODO: When pattern types are stabilized and constified, replace TagByte with a pattern type:
// type TagByte = u8 is 0x20 ..= 0x7E;

#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum TagByte {
    Space = 0x20, ExclamationMark, QuotationMark, NumberSign, DollarSign, PercentSign, Ampersand,
    Apostrophe, LeftParenthesis, RightParenthesis, Asterisk, PlusSign, Comma, HyphenMinus, FullStop,
    Solidus, Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9, Colon,
    Semicolon, LessThanSign, EqualsSign, GreaterThanSign, QuestionMark, CommercialAt, CapitalA,
    CapitalB, CapitalC, CapitalD, CapitalE, CapitalF, CapitalG, CapitalH, CapitalI, CapitalJ,
    CapitalK, CapitalL, CapitalM, CapitalN, CapitalO, CapitalP, CapitalQ, CapitalR, CapitalS,
    CapitalT, CapitalU, CapitalV, CapitalW, CapitalX, CapitalY, CapitalZ, LeftSquareBracket,
    ReverseSolidus, RightSquareBracket, CircumflexAccent, LowLine, GraveAccent, SmallA, SmallB,
    SmallC, SmallD, SmallE, SmallF, SmallG, SmallH, SmallI, SmallJ, SmallK, SmallL, SmallM, SmallN,
    SmallO, SmallP, SmallQ, SmallR, SmallS, SmallT, SmallU, SmallV, SmallW, SmallX, SmallY, SmallZ,
    LeftCurlyBracket, VerticalLine, RightCurlyBracket, Tilde, // 0x20 ..= 0x7E
}

#[derive(Debug, thiserror::Error)]
#[derive_const(Clone, PartialEq, Eq)]
pub enum ParseTagError {
    #[error("bytes not matching 0x20..=0x7E range")]
    InvalidBytes,
    #[error("tag length is not 3 or 4")]
    InvalidLength,
}

impl Tag {
    pub const fn from_bytes(bytes: [u8; 4]) -> Result<Self, ParseTagError> {
        if matches!(bytes, [0x20..=0x7E, 0x20..=0x7E, 0x20..=0x7E, 0x20..=0x7E]) {
            Ok(Self(unsafe { std::mem::transmute::<[u8; 4], [TagByte; 4]>(bytes) }))
        } else {
            Err(ParseTagError::InvalidBytes)
        }
    }
    pub const fn from_str(s: &str) -> Result<Self, ParseTagError> {
        Self::from_bytes(match *s.as_bytes() {
            [a, b, c, d] => [a, b, c, d],
            [a, b, c] => [a, b, c, b' '],
            _ => return Err(ParseTagError::InvalidLength),
        })
    }

    pub const fn to_bytes(self) -> [u8; 4] {
        unsafe { std::mem::transmute(self.0) }
    }
    pub const fn as_bytes(&self) -> &[u8; 4] {
        unsafe { std::mem::transmute(&self.0) }
    }
    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }
}

macro_rules! define_known_tags {
    ( $($tag:ident $(= $s:expr)?),* $(,)? ) => {
        impl Tag {
            pub const KNOWN_TAGS: &[Tag] = &[ $( tags::$tag, )* ];
        }

        #[allow(non_upper_case_globals)]
        pub mod tags {
            use super::Tag;

            $( pub const $tag: Tag = Tag::from_str(define_known_tags!(@value $tag $(= $s)?)).ok().unwrap(); )*
        }
    };
    (@value $tag:ident) => (stringify!($tag));
    (@value $tag:ident = $s:expr) => ($s);
}

define_known_tags! {
    avar, BASE, CBDT, CBLC, CFF, CFF2, cmap, COLR, CPAL, cvar, cvt, DSIG, EBDT, EBLC, EBSC, fpgm,
    fvar, gasp, GDEF, glyf, GPOS, GSUB, gvar, hdmx, head, hhea, hmtx, HVAR, JSTF, kern, loca, LTSH,
    MATH, maxp, MERG, meta, MVAR, name, OS_2 = "OS/2", PCLT, post, prep, sbix, STAT, SVG, VDMX,
    vhea, vmtx, VORG, VVAR,
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = [b'\'', 0, 0, 0, 0, b'\''];
        buf[1..5].copy_from_slice(self.as_bytes());
        f.write_str(unsafe { str::from_utf8_unchecked(&buf) })
    }
}
impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

const impl PartialEq<str> for Tag {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

const impl std::str::FromStr for Tag {
    type Err = ParseTagError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
const impl TryFrom<&str> for Tag {
    type Error = ParseTagError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
const impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
