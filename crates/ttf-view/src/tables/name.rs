use crate::{
    encodings::{Encoding, EncodingError},
    types::{Offset16, uint16},
};
use std::{borrow::Cow, bstr::ByteStr, fmt};

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

impl NameTableRepr {
    pub const fn name_records(&self) -> &[NameRecordRepr] {
        unsafe { std::slice::from_raw_parts(self.name_records.as_ptr(), self.count.get() as _) }
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

    pub const fn string_storage(&self) -> &StringStorage {
        let offset = self.storage_offset.get() as usize;
        unsafe { &*std::ptr::from_ref(self).byte_add(offset).cast() }
    }
}

#[non_exhaustive]
pub struct StringStorage;

impl StringStorage {
    pub const fn as_ptr(&self) -> *const u8 {
        std::ptr::from_ref(self).cast()
    }
    pub(crate) const unsafe fn get(&self, offset: uint16, length: uint16) -> &[u8] {
        unsafe {
            let start = self.as_ptr().add(offset.get() as _);
            std::slice::from_raw_parts(start, length.get() as _)
        }
    }
}

impl LangTagRecordRepr {
    pub const fn bytes<'a>(&'a self, storage: &'a StringStorage) -> &'a [u8] {
        unsafe { storage.get(self.lang_tag_offset, self.length) }
    }
    pub fn tag<'a>(&'a self, storage: &'a StringStorage) -> String {
        // Note: LangTags are always encoded in UTF-16BE.
        String::from_utf16be_lossy(self.bytes(storage))
    }
}

impl NameRecordRepr {
    pub const fn bytes<'a>(&'a self, storage: &'a StringStorage) -> &'a [u8] {
        unsafe { storage.get(self.string_offset, self.length) }
    }
    pub fn string<'a>(&'a self, storage: &'a StringStorage) -> Result<String, EncodingError> {
        let encoding = Encoding::new(self.platform_id.get(), self.encoding_id.get());
        encoding.decode_utf16be(self.bytes(storage))
    }
}

impl fmt::Debug for NameTableRepr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let storage = self.string_storage();
        let lang_tags = self.lang_tag_records();

        let mut builder = f.debug_struct("NameTableRepr");

        builder
            .field("version", &self.version.get())
            .field("count", &self.count.get())
            .field_with("storage_offset", |f| write!(f, "{:#06X}", self.storage_offset))
            .field_with("name_records", |f| {
                let names = self.name_records().iter();
                f.debug_list()
                    .entries(names.map(|x| NameRecordDebug(x, storage, lang_tags)))
                    .finish()
            });

        if self.version.get() != 0 {
            builder.field("lang_tag_count", &lang_tags.len());
            builder.field_with("lang_tag_records", |f| {
                f.debug_list()
                    .entries(lang_tags.iter().map(|x| LangTagRecordDebug(x, storage)))
                    .finish()
            });
        }

        builder.finish()
    }
}

struct NameRecordDebug<'a>(&'a NameRecordRepr, &'a StringStorage, &'a [LangTagRecordRepr]);

struct LangTagRecordDebug<'a>(&'a LangTagRecordRepr, &'a StringStorage);

impl<'a> fmt::Debug for NameRecordDebug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(name, storage, _lang_tags) = *self;

        let enc = Encoding::new(name.platform_id.get(), name.encoding_id.get());
        let enc_name = enc.encoding_name().unwrap_or(Cow::Borrowed("Unknown"));

        let value = name.string(storage).map_err(|_| ByteStr::new(name.bytes(storage)));

        f.debug_struct("NameRecordRepr")
            .field("platform_id", &enc.platform_id())
            .field_with("encoding_id", |f| write!(f, "{} ({})", name.encoding_id.get(), enc_name))
            // TODO: Parse language_id as either a platform-specific id or a LangTag
            .field_with("language_id", |f| write!(f, "{:#06X}", name.language_id))
            // TODO: Parse name_id and display its name
            .field("name_id", &name.name_id.get())
            .field("length", &name.length.get())
            .field_with("string_offset", |f| write!(f, "{:#06X}", name.string_offset))
            .field_with("value", |f| {
                let mut f = f.with_options(*f.options().alternate(false));
                value.fmt(&mut f)
            })
            .finish()
    }
}

impl<'a> fmt::Debug for LangTagRecordDebug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(lang, storage) = self;

        f.debug_struct("LangTagRecordRepr")
            .field("length", &lang.length.get())
            .field_with("lang_tag_offset", |f| write!(f, "{:#06X}", lang.lang_tag_offset))
            .field_with("tag", |f| {
                let mut f = f.with_options(*f.options().alternate(false));
                lang.tag(storage).fmt(&mut f)
            })
            .finish()
    }
}
