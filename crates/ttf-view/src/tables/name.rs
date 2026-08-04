use crate::{
    tables::cmap::{Encoding, encodings::EncodingError},
    types::{Offset16, uint16},
};

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
    pub const fn bytes<'a>(&'a self, store: &'a StringStorage) -> &'a [u8] {
        unsafe { store.get(self.lang_tag_offset, self.length) }
    }
    pub fn tag<'a>(&'a self, store: &'a StringStorage) -> String {
        // Note: LangTags are always encoded in UTF-16BE.
        String::from_utf16be_lossy(self.bytes(store))
    }
}

impl NameRecordRepr {
    pub const fn bytes<'a>(&'a self, store: &'a StringStorage) -> &'a [u8] {
        unsafe { store.get(self.string_offset, self.length) }
    }
    pub fn string<'a>(&'a self, store: &'a StringStorage) -> Result<String, EncodingError> {
        let raw_bytes = self.bytes(store);

        if Encoding::is_unicode(self.platform_id, self.encoding_id) {
            return Ok(String::from_utf16be_lossy(raw_bytes));
        }

        // TODO: After the issue with encodings is resolved, use them to decode strings here.
        Err(EncodingError)
    }
}
