use crate::{
    platform::PlatformId,
    tables::name::{LangTagRecordRepr, StringStorage},
};
use std::borrow::Cow;

#[derive(Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum LanguageId {
    Tagged(u16) = 0,
    Macintosh(u16) = 1,
    Windows(u16) = 3,
}

impl LanguageId {
    pub const fn new(platform_id: u16, language_id: u16) -> Option<Self> {
        PlatformId::new(platform_id)?.language(language_id)
    }

    pub const fn get(&self) -> u16 {
        match *self {
            Self::Tagged(x) => x,
            Self::Macintosh(x) => x,
            Self::Windows(x) => x,
        }
    }

    pub fn tag(
        &self,
        lang_tags: &[LangTagRecordRepr],
        storage: &StringStorage,
    ) -> Option<Cow<'static, str>> {
        Some(match self {
            Self::Tagged(id) => {
                Cow::Owned(lang_tags.get((id - 0x8000) as usize).map(|x| x.tag(storage))?)
            },
            Self::Macintosh(id) => Cow::Borrowed(macintosh_language_tag(*id)?),

            Self::Windows(id) => {
                let res = <&lcid::LanguageId>::try_from(*id as u32);
                match res {
                    Ok(lcid) => Cow::Borrowed(lcid.name),
                    Err(lcid::LcidLookupError::Reserved(_, tag)) => Cow::Borrowed(tag),
                    Err(_) => return None,
                }
            },
        })
    }

    pub fn english_name(
        &self,
        lang_tags: &[LangTagRecordRepr],
        storage: &StringStorage,
    ) -> Option<Cow<'static, str>> {
        Some(match self {
            Self::Tagged(id) => {
                let lang_tag = lang_tags.get((id - 0x8000) as usize).map(|x| x.tag(storage))?;
                let lcid: &lcid::LanguageId = lang_tag.as_str().try_into().ok()?;
                Cow::Borrowed(lcid.english_name)
            },
            Self::Macintosh(id) => Cow::Borrowed(macintosh_language_name(*id)?),

            Self::Windows(id) => {
                let lcid: &lcid::LanguageId = (*id as u32).try_into().ok()?;
                Cow::Borrowed(lcid.english_name)
            },
        })
    }
}

#[rustfmt::skip]
fn macintosh_language_tag(id: u16) -> Option<&'static str> {
    Some(match id {
        0 => "en", 1 => "fr", 2 => "de", 3 => "it", 4 => "nl", 5 => "sv", 6 => "es", 7 => "da",
        8 => "pt", 9 => "no", 10 => "he", 11 => "ja", 12 => "ar", 13 => "fi", 14 => "el",
        15 => "is", 16 => "mt", 17 => "tr", 18 => "hr", 19 => "zh-Hant", 20 => "ur", 21 => "hi",
        22 => "th", 23 => "ko", 24 => "lt", 25 => "pl", 26 => "hu", 27 => "et", 28 => "lv",
        29 => "se", 30 => "fo", 31 => "fa", 32 => "ru", 33 => "zh-Hans", 34 => "nl", 35 => "ga",
        36 => "sq", 37 => "ro", 38 => "cs", 39 => "sk", 40 => "sl", 41 => "yi", 42 => "sr",
        43 => "mk", 44 => "bg", 45 => "uk", 46 => "be", 47 => "uz", 48 => "kk", 49 => "az-Cyrl",
        50 => "az-Arab", 51 => "hy", 52 => "ka", 53 => "ro", 54 => "ky", 55 => "tg", 56 => "tk",
        57 => "mn-Mong", 58 => "mn-Cyrl", 59 => "ps", 60 => "ku", 61 => "ks", 62 => "sd",
        63 => "bo", 64 => "ne", 65 => "sa", 66 => "mr", 67 => "bn", 68 => "as", 69 => "gu",
        70 => "pa", 71 => "or", 72 => "ml", 73 => "kn", 74 => "ta", 75 => "te", 76 => "si",
        77 => "my", 78 => "km", 79 => "lo", 80 => "vi", 81 => "id", 82 => "tl", 83 => "ms-Latn",
        84 => "ms-Arab", 85 => "am", 86 => "ti", 87 => "om", 88 => "so", 89 => "sw", 90 => "rw",
        91 => "rn", 92 => "ny", 93 => "mg", 94 => "eo", 128 => "cy", 129 => "eu", 130 => "ca",
        131 => "la", 132 => "qu", 133 => "gn", 134 => "ay", 135 => "tt", 136 => "ug", 137 => "dz",
        138 => "jv-Latn", 139 => "su-Latn", 140 => "gl", 141 => "af", 142 => "br", 143 => "iu",
        144 => "gd", 145 => "gv", 146 => "ga", 147 => "to", 148 => "el", 149 => "kl",
        150 => "az-Latn",
        _ => return None,
    })
}
#[rustfmt::skip]
fn macintosh_language_name(id: u16) -> Option<&'static str> {
    Some(match id {
        0 => "English", 1 => "French", 2 => "German", 3 => "Italian", 4 => "Dutch", 5 => "Swedish",
        6 => "Spanish", 7 => "Danish", 8 => "Portuguese", 9 => "Norwegian", 10 => "Hebrew",
        11 => "Japanese", 12 => "Arabic", 13 => "Finnish", 14 => "Greek", 15 => "Icelandic",
        16 => "Maltese", 17 => "Turkish", 18 => "Croatian", 19 => "Chinese (traditional)",
        20 => "Urdu", 21 => "Hindi", 22 => "Thai", 23 => "Korean", 24 => "Lithuanian",
        25 => "Polish", 26 => "Hungarian", 27 => "Estonian", 28 => "Latvian", 29 => "Sami",
        30 => "Faroese", 31 => "Farsi/Persian", 32 => "Russian", 33 => "Chinese (simplified)",
        34 => "Flemish", 35 => "Irish Gaelic", 36 => "Albanian", 37 => "Romanian", 38 => "Czech",
        39 => "Slovak", 40 => "Slovenian", 41 => "Yiddish", 42 => "Serbian", 43 => "Macedonian",
        44 => "Bulgarian", 45 => "Ukrainian", 46 => "Byelorussian", 47 => "Uzbek", 48 => "Kazakh",
        49 => "Azerbaijani (Cyrillic script)", 50 => "Azerbaijani (Arabic script)",
        51 => "Armenian", 52 => "Georgian", 53 => "Moldavian", 54 => "Kirghiz", 55 => "Tajiki",
        56 => "Turkmen", 57 => "Mongolian (Mongolian script)", 58 => "Mongolian (Cyrillic script)",
        59 => "Pashto", 60 => "Kurdish", 61 => "Kashmiri", 62 => "Sindhi", 63 => "Tibetan",
        64 => "Nepali", 65 => "Sanskrit", 66 => "Marathi", 67 => "Bengali", 68 => "Assamese",
        69 => "Gujarati", 70 => "Punjabi", 71 => "Oriya", 72 => "Malayalam", 73 => "Kannada",
        74 => "Tamil", 75 => "Telugu", 76 => "Sinhalese", 77 => "Burmese", 78 => "Khmer",
        79 => "Lao", 80 => "Vietnamese", 81 => "Indonesian", 82 => "Tagalog",
        83 => "Malay (Roman script)", 84 => "Malay (Arabic script)", 85 => "Amharic",
        86 => "Tigrinya", 87 => "Galla", 88 => "Somali", 89 => "Swahili",
        90 => "Kinyarwanda/Ruanda", 91 => "Rundi", 92 => "Nyanja/Chewa", 93 => "Malagasy",
        94 => "Esperanto", 128 => "Welsh", 129 => "Basque", 130 => "Catalan", 131 => "Latin",
        132 => "Quechua", 133 => "Guarani", 134 => "Aymara", 135 => "Tatar", 136 => "Uighur",
        137 => "Dzongkha", 138 => "Javanese (Roman script)", 139 => "Sundanese (Roman script)",
        140 => "Galician", 141 => "Afrikaans", 142 => "Breton", 143 => "Inuktitut",
        144 => "Scottish Gaelic", 145 => "Manx Gaelic", 146 => "Irish Gaelic (with dot above)",
        147 => "Tongan", 148 => "Greek (polytonic)", 149 => "Greenlandic",
        150 => "Azerbaijani (Roman script)",
        _ => return None,
    })
}
