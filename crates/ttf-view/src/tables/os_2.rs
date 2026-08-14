#![allow(non_camel_case_types)]
use crate::{
    tables::TableDirectoryRepr,
    types::{FWORD, Tag, UFWORD, int16, tags, uint16, uint32},
    util::{Describe, Describer, describe},
};

#[repr(C)]
pub struct Os_2TableRepr {
    // version ≥ 0:
    pub version: uint16,
    pub x_avg_char_width: FWORD,
    pub us_weight_class: uint16,
    pub us_width_class: uint16,
    pub fs_type: uint16,
    pub y_subscript_x_size: FWORD,
    pub y_subscript_y_size: FWORD,
    pub y_subscript_x_offset: FWORD,
    pub y_subscript_y_offset: FWORD,
    pub y_superscript_x_size: FWORD,
    pub y_superscript_y_size: FWORD,
    pub y_superscript_x_offset: FWORD,
    pub y_superscript_y_offset: FWORD,
    pub y_strikeout_size: FWORD,
    pub y_strikeout_position: FWORD,
    pub s_family_class: int16,
    pub panose: Panose,
    pub ul_unicode_range_1: uint32,
    pub ul_unicode_range_2: uint32,
    pub ul_unicode_range_3: uint32,
    pub ul_unicode_range_4: uint32,
    pub ach_vend_id: Tag,
    pub fs_selection: uint16,
    pub us_first_char_index: uint16,
    pub us_last_char_index: uint16,
    // Apple's docs attribute these 5 fields to v1 instead of v0, so they could be missing in malformed fonts
    s_typo_ascender: FWORD,
    s_typo_descender: FWORD,
    s_typo_line_gap: FWORD,
    us_win_ascent: UFWORD,
    us_win_descent: UFWORD,
    // version ≥ 1:
    ul_code_page_range_1: uint32,
    ul_code_page_range_2: uint32,
    // version ≥ 4:
    sx_height: FWORD,
    s_cap_height: FWORD,
    us_default_char: uint16,
    us_break_char: uint16,
    us_max_content: uint16,
    // version ≥ 5:
    us_lower_optical_point_size: uint16,
    us_upper_optical_point_size: uint16,
}

impl super::Table for Os_2TableRepr {
    const TAG: Tag = tags::OS_2;
    type Handle<'a> = Os_2TableHandle<'a>;
}
impl<'a> super::TableHandle<'a> for Os_2TableHandle<'a> {
    fn in_directory(dir: &'a TableDirectoryRepr) -> Option<Self> {
        let record = dir.table_record(tags::OS_2)?;
        Some(Self { os_2: record.table_as()?, len: record.length.get() })
    }
}

pub struct Os_2TableHandle<'a> {
    os_2: &'a Os_2TableRepr,
    len: u32,
}
const impl<'a> std::ops::Deref for Os_2TableHandle<'a> {
    type Target = Os_2TableRepr;
    fn deref(&self) -> &Self::Target {
        self.os_2
    }
}

impl<'a> Os_2TableHandle<'a> {
    pub const fn s_typo_ascender(&self) -> Option<FWORD> {
        if self.len >= 0x46 { Some(self.s_typo_ascender) } else { None }
    }
    pub const fn s_typo_descender(&self) -> Option<FWORD> {
        if self.len >= 0x48 { Some(self.s_typo_descender) } else { None }
    }
    pub const fn s_typo_line_gap(&self) -> Option<FWORD> {
        if self.len >= 0x4A { Some(self.s_typo_line_gap) } else { None }
    }
    pub const fn us_win_ascent(&self) -> Option<UFWORD> {
        if self.len >= 0x4C { Some(self.us_win_ascent) } else { None }
    }
    pub const fn us_win_descent(&self) -> Option<UFWORD> {
        if self.len >= 0x4E { Some(self.us_win_descent) } else { None }
    }
}

#[repr(C)]
pub struct Panose {
    pub b_family_type: u8,
    pub b_serif_style: u8,
    pub b_weight: u8,
    pub b_proportion: u8,
    pub b_contrast: u8,
    pub b_stroke_variation: u8,
    pub b_arm_style: u8,
    pub b_letterform: u8,
    pub b_midline: u8,
    pub b_x_height: u8,
}

impl Os_2TableRepr {
    pub const fn ul_code_page_range_1(&self) -> Option<uint32> {
        if self.version.get() >= 1 { Some(self.ul_code_page_range_1) } else { None }
    }
    pub const fn ul_code_page_range_2(&self) -> Option<uint32> {
        if self.version.get() >= 1 { Some(self.ul_code_page_range_2) } else { None }
    }
    pub const fn code_page_range(&self) -> Option<CodePageRange> {
        if self.version.get() >= 1 {
            Some(CodePageRange::from_bits_retain(
                (self.ul_code_page_range_1.get() as u64)
                    | ((self.ul_code_page_range_2.get() as u64) << 32),
            ))
        } else {
            None
        }
    }

    pub const fn unicode_range(&self) -> UnicodeRange {
        UnicodeRange::from_bits_retain(
            (self.ul_unicode_range_1.get() as u128)
                | ((self.ul_unicode_range_2.get() as u128) << 32)
                | ((self.ul_unicode_range_3.get() as u128) << 64)
                | ((self.ul_unicode_range_4.get() as u128) << 96),
        )
    }
}

bitflags::bitflags! {
    pub struct CodePageRange: u64 {
        const Latin_1 = 1 << 0; // CP 1252
        const Latin_2_Eastern_Europe = 1 << 1; // CP 1250
        const Cyrillic = 1 << 2; // CP 1251
        const Greek = 1 << 3; // CP 1253
        const Turkish = 1 << 4; // CP 1254
        const Hebrew = 1 << 5; // CP 1255
        const Arabic = 1 << 6; // CP 1256
        const Windows_Baltic = 1 << 7; // CP 1257
        const Vietnamese = 1 << 8; // CP 1258
        // 9-15 reserved for Alternate ANSI
        const Thai = 1 << 16; // CP 874
        const JIS_Japan = 1 << 17; // CP 932
        const Chinese_Simplified_chars_PRC_and_Singapore = 1 << 18; // CP 936
        const Korean_Wansung = 1 << 19; // CP 949
        const Chinese_Traditional_chars_Taiwan_and_Hong_Kong_SAR = 1 << 20; // CP 950
        const Korean_Johab = 1 << 21; // CP 1361
        // 22-28 reserved for Alternate ANSI or OEM
        const Macintosh_Character_Set_US_Roman_ = 1 << 29;
        const OEM_Character_Set = 1 << 30;
        const Symbol_Character_Set = 1 << 31;
        // 32-47 reserved for OEM
        const IBM_Greek = 1 << 48; // CP 869
        const MS_DOS_Russian = 1 << 49; // CP 866
        const MS_DOS_Nordic = 1 << 50; // CP 865
        const Arabic2 = 1 << 51; // CP 864
        const MS_DOS_Canadian_French = 1 << 52; // CP 863
        const Hebrew2 = 1 << 53; // CP 862
        const MS_DOS_Icelandic = 1 << 54; // CP 861
        const MS_DOS_Portuguese = 1 << 55; // CP 860
        const IBM_Turkish = 1 << 56; // CP 857
        const IBM_Cyrillic_primarily_Russian = 1 << 57; // CP 855
        const Latin_2 = 1 << 58; // CP 852
        const MS_DOS_Baltic = 1 << 59; // CP 775
        const Greek_former_437_G = 1 << 60; // CP 737
        const Arabic_ASMO_708 = 1 << 61; // CP 708
        const WE_Latin_1 = 1 << 62; // CP 850
        const US = 1 << 63; // CP 437
    }
}
bitflags::bitflags! {
    pub struct UnicodeRange: u128 {
        // version ≥ 1:
        const Basic_Latin = 1 << 0; // 0000-007F
        const Latin_1_Supplement = 1 << 1; // 0080-00FF
        const Latin_Extended_A = 1 << 2; // 0100-017F
        const Latin_Extended_B = 1 << 3; // 0180-024F
        const IPA_Extensions = 1 << 4; // 0250-02AF
        // const Phonetic_Extensions = 1 << ; // 1D00-1D7F
        // const Phonetic_Extensions_Supplement = 1 << ; // 1D80-1DBF
        const Spacing_Modifier_Letters = 1 << 5; // 02B0-02FF
        // const Modifier_Tone_Letters = 1 << ; // A700-A71F
        const Combining_Diacritical_Marks = 1 << 6; // 0300-036F
        // const Combining_Diacritical_Marks_Supplement = 1 << ; // 1DC0-1DFF
        const Greek_and_Coptic = 1 << 7; // 0370-03FF
        const Coptic = 1 << 8; // 2C80-2CFF
        const Cyrillic = 1 << 9; // 0400-04FF
        // const Cyrillic_Supplement = 1 << ; // 0500-052F
        // const Cyrillic_Extended_A = 1 << ; // 2DE0-2DFF
        // const Cyrillic_Extended_B = 1 << ; // A640-A69F
        const Armenian = 1 << 10; // 0530-058F
        const Hebrew = 1 << 11; // 0590-05FF
        const Vai = 1 << 12; // A500-A63F
        const Arabic = 1 << 13; // 0600-06FF
        // const Arabic_Supplement = 1 << ; // 0750-077F
        const NKo = 1 << 14; // 07C0-07FF
        const Devanagari = 1 << 15; // 0900-097F
        const Bangla = 1 << 16; // 0980-09FF
        const Gurmukhi = 1 << 17; // 0A00-0A7F
        const Gujarati = 1 << 18; // 0A80-0AFF
        const Odia = 1 << 19; // 0B00-0B7F
        const Tamil = 1 << 20; // 0B80-0BFF
        const Telugu = 1 << 21; // 0C00-0C7F
        const Kannada = 1 << 22; // 0C80-0CFF
        const Malayalam = 1 << 23; // 0D00-0D7F
        const Thai = 1 << 24; // 0E00-0E7F
        const Lao = 1 << 25; // 0E80-0EFF
        const Georgian = 1 << 26; // 10A0-10FF
        // const Georgian_Supplement = 1 << ; // 2D00-2D2F
        const Balinese = 1 << 27; // 1B00-1B7F
        const Hangul_Jamo = 1 << 28; // 1100-11FF
        const Latin_Extended_Additional = 1 << 29; // 1E00-1EFF
        // const Latin_Extended_C = 1 << ; // 2C60-2C7F
        // const Latin_Extended_D = 1 << ; // A720-A7FF
        const Greek_Extended = 1 << 30; // 1F00-1FFF
        const General_Punctuation = 1 << 31; // 2000-206F
        // const Supplemental_Punctuation = 1 << ; // 2E00-2E7F
        const Superscripts_And_Subscripts = 1 << 32; // 2070-209F
        const Currency_Symbols = 1 << 33; // 20A0-20CF
        const Combining_Diacritical_Marks_For_Symbols = 1 << 34; // 20D0-20FF
        const Letterlike_Symbols = 1 << 35; // 2100-214F
        const Number_Forms = 1 << 36; // 2150-218F
        const Arrows = 1 << 37; // 2190-21FF
        // const Supplemental_Arrows_A = 1 << ; // 27F0-27FF
        // const Supplemental_Arrows_B = 1 << ; // 2900-297F
        // const Miscellaneous_Symbols_and_Arrows = 1 << ; // 2B00-2BFF
        const Mathematical_Operators = 1 << 38; // 2200-22FF
        // const Supplemental_Mathematical_Operators = 1 << ; // 2A00-2AFF
        // const Miscellaneous_Mathematical_Symbols_A = 1 << ; // 27C0-27EF
        // const Miscellaneous_Mathematical_Symbols_B = 1 << ; // 2980-29FF
        const Miscellaneous_Technical = 1 << 39; // 2300-23FF
        const Control_Pictures = 1 << 40; // 2400-243F
        const Optical_Character_Recognition = 1 << 41; // 2440-245F
        const Enclosed_Alphanumerics = 1 << 42; // 2460-24FF
        const Box_Drawing = 1 << 43; // 2500-257F
        const Block_Elements = 1 << 44; // 2580-259F
        const Geometric_Shapes = 1 << 45; // 25A0-25FF
        const Miscellaneous_Symbols = 1 << 46; // 2600-26FF
        const Dingbats = 1 << 47; // 2700-27BF
        const CJK_Symbols_And_Punctuation = 1 << 48; // 3000-303F
        const Hiragana = 1 << 49; // 3040-309F
        const Katakana = 1 << 50; // 30A0-30FF
        // const Katakana_Phonetic_Extensions = 1 << ; // 31F0-31FF
        const Bopomofo = 1 << 51; // 3100-312F
        // const Bopomofo_Extended = 1 << ; // 31A0-31BF
        const Hangul_Compatibility_Jamo = 1 << 52; // 3130-318F
        const Phags_pa = 1 << 53; // A840-A87F
        const Enclosed_CJK_Letters_And_Months = 1 << 54; // 3200-32FF
        const CJK_Compatibility = 1 << 55; // 3300-33FF
        const Hangul_Syllables = 1 << 56; // AC00-D7AF
        const Non_Plane_0 = 1 << 57; // 10000-10FFFF
        const Phoenician = 1 << 58; // 10900-1091F
        const CJK_Unified_Ideographs = 1 << 59; // 4E00-9FFF
        // const CJK_Radicals_Supplement = 1 << ; // 2E80-2EFF
        // const Kangxi_Radicals = 1 << ; // 2F00-2FDF
        // const Ideographic_Description_Characters = 1 << ; // 2FF0-2FFF
        // const CJK_Unified_Ideographs_Extension_A = 1 << ; // 3400-4DBF
        // const CJK_Unified_Ideographs_Extension_B = 1 << ; // 20000-2A6DF
        // const Kanbun = 1 << ; // 3190-319F
        const Private_Use_Area_plane_0_ = 1 << 60; // E000-F8FF
        const CJK_Strokes = 1 << 61; // 31C0-31EF
        // const CJK_Compatibility_Ideographs = 1 << ; // F900-FAFF
        // const CJK_Compatibility_Ideographs_Supplement = 1 << ; // 2F800-2FA1F
        const Alphabetic_Presentation_Forms = 1 << 62; // FB00-FB4F
        const Arabic_Presentation_Forms_A = 1 << 63; // FB50-FDFF
        const Combining_Half_Marks = 1 << 64; // FE20-FE2F
        const Vertical_Forms = 1 << 65; // FE10-FE1F
        // const CJK_Compatibility_Forms = 1 << ; // FE30-FE4F
        const Small_Form_Variants = 1 << 66; // FE50-FE6F
        const Arabic_Presentation_Forms_B = 1 << 67; // FE70-FEFF
        const Halfwidth_And_Fullwidth_Forms = 1 << 68; // FF00-FFEF
        const Specials = 1 << 69; // FFF0-FFFF
        // version ≥ 2:
        const Tibetan = 1 << 70; // 0F00-0FFF
        const Syriac = 1 << 71; // 0700-074F
        const Thaana = 1 << 72; // 0780-07BF
        const Sinhala = 1 << 73; // 0D80-0DFF
        const Myanmar = 1 << 74; // 1000-109F
        const Ethiopic = 1 << 75; // 1200-137F
        // const Ethiopic_Supplement = 1 << ; // 1380-139F
        // const Ethiopic_Extended = 1 << ; // 2D80-2DDF
        const Cherokee = 1 << 76; // 13A0-13FF
        const Unified_Canadian_Aboriginal_Syllabics = 1 << 77; // 1400-167F
        const Ogham = 1 << 78; // 1680-169F
        const Runic = 1 << 79; // 16A0-16FF
        const Khmer = 1 << 80; // 1780-17FF
        // const Khmer_Symbols = 1 << ; // 19E0-19FF
        const Mongolian = 1 << 81; // 1800-18AF
        const Braille_Patterns = 1 << 82; // 2800-28FF
        const Yi_Syllables = 1 << 83; // A000-A48F
        // version ≥ 3:
        // const Yi_Radicals = 1 << ; // A490-A4CF
        const Tagalog = 1 << 84; // 1700-171F
        // const Hanunoo = 1 << ; // 1720-173F
        // const Buhid = 1 << ; // 1740-175F
        // const Tagbanwa = 1 << ; // 1760-177F
        const Old_Italic = 1 << 85; // 10300-1032F
        const Gothic = 1 << 86; // 10330-1034F
        const Deseret = 1 << 87; // 10400-1044F
        const Byzantine_Musical_Symbols = 1 << 88; // 1D000-1D0FF
        // const Musical_Symbols = 1 << ; // 1D100-1D1FF
        // const Ancient_Greek_Musical_Notation = 1 << ; // 1D200-1D24F
        const Mathematical_Alphanumeric_Symbols = 1 << 89; // 1D400-1D7FF
        const Private_Use_plane_15_ = 1 << 90; // F0000-FFFFD
        // const Private_Use_plane_16_ = 1 << ; // 100000-10FFFD
        const Variation_Selectors = 1 << 91; // FE00-FE0F
        // const Variation_Selectors_Supplement = 1 << ; // E0100-E01EF
        const Tags = 1 << 92; // E0000-E007F
        // version ≥ 4:
        const Limbu = 1 << 93; // 1900-194F
        const Tai_Le = 1 << 94; // 1950-197F
        const New_Tai_Lue = 1 << 95; // 1980-19DF
        const Buginese = 1 << 96; // 1A00-1A1F
        const Glagolitic = 1 << 97; // 2C00-2C5F
        const Tifinagh = 1 << 98; // 2D30-2D7F
        const Yijing_Hexagram_Symbols = 1 << 99; // 4DC0-4DFF
        const Syloti_Nagri = 1 << 100; // A800-A82F
        const Linear_B_Syllabary = 1 << 101; // 10000-1007F
        // const Linear_B_Ideograms = 1 << ; // 10080-100FF
        // const Aegean_Numbers = 1 << ; // 10100-1013F
        const Ancient_Greek_Numbers = 1 << 102; // 10140-1018F
        const Ugaritic = 1 << 103; // 10380-1039F
        const Old_Persian = 1 << 104; // 103A0-103DF
        const Shavian = 1 << 105; // 10450-1047F
        const Osmanya = 1 << 106; // 10480-104AF
        const Cypriot_Syllabary = 1 << 107; // 10800-1083F
        const Kharoshthi = 1 << 108; // 10A00-10A5F
        const Tai_Xuan_Jing_Symbols = 1 << 109; // 1D300-1D35F
        const Cuneiform = 1 << 110; // 12000-123FF
        // const Cuneiform_Numbers_and_Punctuation = 1 << ; // 12400-1247F
        const Counting_Rod_Numerals = 1 << 111; // 1D360-1D37F
        const Sundanese = 1 << 112; // 1B80-1BBF
        const Lepcha = 1 << 113; // 1C00-1C4F
        const Ol_Chiki = 1 << 114; // 1C50-1C7F
        const Saurashtra = 1 << 115; // A880-A8DF
        const Kayah_Li = 1 << 116; // A900-A92F
        const Rejang = 1 << 117; // A930-A95F
        const Cham = 1 << 118; // AA00-AA5F
        const Ancient_Symbols = 1 << 119; // 10190-101CF
        const Phaistos_Disc = 1 << 120; // 101D0-101FF
        const Carian = 1 << 121; // 102A0-102DF
        // const Lycian = 1 << ; // 10280-1029F
        // const Lydian = 1 << ; // 10920-1093F
        const Domino_Tiles = 1 << 122; // 1F030-1F09F
        // const Mahjong_Tiles = 1 << ; // 1F000-1F02F
        // 123-127 reserved
    }
}

impl Describe for Os_2TableRepr {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        describe!(d, self as "Os_2Table" {
            version,
            x_avg_char_width,
            us_weight_class,
            us_width_class,
            fs_type,
            y_subscript_x_size,
            y_subscript_y_size,
            y_subscript_x_offset,
            y_subscript_y_offset,
            y_superscript_x_size,
            y_superscript_y_size,
            y_superscript_x_offset,
            y_superscript_y_offset,
            y_strikeout_size,
            y_strikeout_position,
            s_family_class,
            panose,
            ul_unicode_range_1,
            ul_unicode_range_2,
            ul_unicode_range_3,
            ul_unicode_range_4,
            ach_vend_id,
            fs_selection,
            us_first_char_index,
            us_last_char_index,
        })
    }
}

impl Describe for Panose {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        describe!(d, self as "Panose" {
            b_family_type,
            b_serif_style,
            b_weight,
            b_proportion,
            b_contrast,
            b_stroke_variation,
            b_arm_style,
            b_letterform,
            b_midline,
            b_x_height,
        })
    }
}
