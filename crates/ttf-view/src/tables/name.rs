use crate::{
    platform::{EncodingError, EncodingId, LanguageId, PlatformId},
    types::{Offset16, Tag, tags, uint16},
    util::{Describe, Describer, StructDescriber, describe, describe_impl, iterator_map},
};
use std::{borrow::Cow, bstr::ByteStr};

#[repr(C)]
#[non_exhaustive]
pub struct NameTableRepr {
    // version ≥ 0:
    pub version: uint16,
    pub count: uint16,
    pub storage_offset: Offset16,
    name_records: [NameRecordRepr; 0],
    // version ≥ 1:
    // : lang_tag_count: uint16
    // : lang_tag_records: [LangTagRecordRepr; lang_tag_count]
}
#[repr(C)]
pub struct NameRecordRepr {
    pub platform_id: uint16,
    pub encoding_id: uint16,
    pub language_id: uint16,
    pub name_id: uint16,
    pub length: uint16,
    pub string_offset: Offset16,
}
#[repr(C)]
pub struct LangTagRecordRepr {
    pub length: uint16,
    pub lang_tag_offset: Offset16,
}

impl super::Table for NameTableRepr {
    const TAG: Tag = tags::name;
    type Handle<'a> = &'a Self;
}

impl NameTableRepr {
    pub const fn name_records(&self) -> &[NameRecordRepr] {
        unsafe { std::slice::from_raw_parts(self.name_records.as_ptr(), self.count.get() as _) }
    }
    pub const fn names(&self) -> NamesIter<'_> {
        NamesIter::new(self)
    }

    pub const fn lang_tag_count(&self) -> uint16 {
        if self.version.get() == 0 {
            return uint16::ZERO;
        }
        unsafe { *self.name_records().as_ptr_range().end.cast() }
    }
    pub const fn lang_tag_records(&self) -> &[LangTagRecordRepr] {
        if self.version.get() == 0 {
            return &[];
        }
        let len_ptr = self.name_records().as_ptr_range().end.cast::<uint16>();
        unsafe { std::slice::from_raw_parts(len_ptr.add(1).cast(), (*len_ptr).get() as _) }
    }
    pub const fn lang_tags(&self) -> LangTagsIter<'_> {
        LangTagsIter::new(self)
    }

    pub const fn string_storage(&self) -> *const u8 {
        let offset = self.storage_offset.get() as usize;
        unsafe { &*std::ptr::from_ref(self).byte_add(offset).cast() }
    }
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct NameHandle<'a>(&'a NameTableRepr, &'a NameRecordRepr);
#[derive(Copy)]
#[derive_const(Clone)]
pub struct LangTagHandle<'a>(&'a NameTableRepr, &'a LangTagRecordRepr);

const impl std::ops::Deref for NameHandle<'_> {
    type Target = NameRecordRepr;
    fn deref(&self) -> &Self::Target {
        self.1
    }
}
const impl std::ops::Deref for LangTagHandle<'_> {
    type Target = LangTagRecordRepr;
    fn deref(&self) -> &Self::Target {
        self.1
    }
}

impl<'a> NameHandle<'a> {
    pub const fn bytes(&self) -> &'a [u8] {
        unsafe {
            let start = self.0.string_storage().byte_add(self.1.string_offset.get() as usize);
            std::slice::from_raw_parts(start, self.1.length.get() as usize)
        }
    }
    pub fn string(&self) -> Result<String, EncodingError> {
        let encoding = EncodingId::new(self.1.platform_id.get(), self.1.encoding_id.get())?;
        encoding.decode_utf16be(self.bytes())
    }
}
impl<'a> LangTagHandle<'a> {
    pub const fn bytes(&self) -> &'a [u8] {
        unsafe {
            let start = self.0.string_storage().byte_add(self.1.lang_tag_offset.get() as usize);
            std::slice::from_raw_parts(start, self.1.length.get() as usize)
        }
    }
    pub fn tag(&self) -> String {
        // Note: LangTags are always encoded in UTF-16BE.
        String::from_utf16be_lossy(self.bytes())
    }
}

// TODO: When std::slice::Iter's Clone is constified, replace this with #[derive_const]
#[derive(Clone)]
pub struct NamesIter<'a> {
    table: &'a NameTableRepr,
    inner: std::slice::Iter<'a, NameRecordRepr>,
}
impl<'a> NamesIter<'a> {
    pub const fn new(table: &'a NameTableRepr) -> Self {
        Self { table, inner: table.name_records().iter() }
    }
    // TODO: When std::slice::Iter's as_slice() is constified, make as_records() const
    pub fn as_records(&self) -> &'a [NameRecordRepr] {
        self.inner.as_slice()
    }
}
iterator_map!(NamesIter<'a> {
    type Item = NameHandle<'a>;
    |this, x| NameHandle(this.table, x)
});

// TODO: When std::slice::Iter's Clone is constified, replace this with #[derive_const]
#[derive(Clone)]
pub struct LangTagsIter<'a> {
    table: &'a NameTableRepr,
    inner: std::slice::Iter<'a, LangTagRecordRepr>,
}
impl<'a> LangTagsIter<'a> {
    pub const fn new(table: &'a NameTableRepr) -> Self {
        Self { table, inner: table.lang_tag_records().iter() }
    }
    // TODO: When std::slice::Iter's as_slice() is constified, make as_records() const
    pub fn as_records(&self) -> &'a [LangTagRecordRepr] {
        self.inner.as_slice()
    }
}
iterator_map!(LangTagsIter<'a> {
    type Item = LangTagHandle<'a>;
    |this, x| LangTagHandle(this.table, x)
});

impl Describe for NameTableRepr {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        let mut d = d.describe_struct("NameTable");

        describe!(d, self {
            version,
            count,
            storage_offset ["{:#06X}"],
            name_records: self.names(),
        });

        if self.version.get() > 0 {
            d.field("lang_tag_count", &self.lang_tag_count());
            d.field("lang_tag_records", &self.lang_tags());
        }

        d.finish()
    }
}
describe_impl! { Debug, Serialize for NameTableRepr }

impl std::fmt::Debug for NamesIter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("NamesIter").field(&Vec::from_iter(self.clone())).finish()
    }
}
impl Describe for NamesIter<'_> {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        d.describe_list_with(self.clone())
    }
}
describe_impl! { Serialize for NamesIter<'_> }

impl std::fmt::Debug for LangTagsIter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("LangTagsIter").field(&Vec::from_iter(self.clone())).finish()
    }
}
impl Describe for LangTagsIter<'_> {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        d.describe_list_with(self.clone())
    }
}
describe_impl! { Serialize for LangTagsIter<'_> }

impl Describe for NameHandle<'_> {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        let mut d = d.describe_struct("NameRecord");

        d.field_fmt("platform_id", &self.platform_id, |f, x| {
            let name = PlatformId::new(x.get()).map_or("Unknown", |x| x.name());
            write!(f, "{} ({})", x, name)
        });

        d.field_fmt("encoding_id", &self.encoding_id, |f, x| {
            let enc = EncodingId::new(self.platform_id.get(), x.get());
            let name = enc.map_or(Cow::Borrowed("Unknown"), |x| x.name());
            write!(f, "{} ({})", x, name)
        });

        d.field_fmt("language_id", &self.language_id, |f, x| {
            let lang = LanguageId::new(self.platform_id.get(), x.get());
            let tag = lang.and_then(|x| x.tag(self.0)).unwrap_or(Cow::Borrowed("und"));
            let name =
                lang.and_then(|x| x.english_name(self.0)).unwrap_or(Cow::Borrowed("Unknown"));
            write!(f, "{:#06X} ({}: {})", x, tag, name)
        });

        d.field_fmt("name_id", &self.name_id, |f, x| {
            // TODO: Parse name_id and display its purpose
            write!(f, "{}", x)
        });

        describe!(d, self { length, string_offset ["{:#06X}"] });

        match self.string() {
            Ok(string) => {
                d.field_fmt("value", &string, |f, x| write!(f, "{:?}", Ok::<_, !>(x)));
            },
            Err(_) => {
                d.field_fmt("value", self.bytes(), |f, x| write!(f, "{:?}", ByteStr::new(x)));
            },
        };

        d.finish()
    }
}
describe_impl! { Debug, Serialize for NameHandle<'_> }

impl Describe for LangTagHandle<'_> {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        describe!(d, self as "LangTagRecord" {
            length,
            lang_tag_offset ["{:#06X}"],
            tag: self.tag(),
        })
    }
}
describe_impl! { Debug, Serialize for LangTagHandle<'_> }

// TODO: impls for records
