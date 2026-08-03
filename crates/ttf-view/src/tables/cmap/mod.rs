use crate::types::{Offset32, uint16};

pub mod format0;

#[repr(C)]
pub struct CmapTableRepr {
    pub version: uint16,
    pub num_tables: uint16,
    encoding_records: [EncodingRecordRepr; 0],
}

#[repr(C)]
pub struct EncodingRecordRepr {
    pub platform_id: uint16,
    pub encoding_id: uint16,
    pub subtable_offset: Offset32,
}

impl CmapTableRepr {
    pub const fn encodings(&self) -> &[EncodingRecordRepr] {
        unsafe {
            std::slice::from_raw_parts(self.encoding_records.as_ptr(), self.num_tables.get() as _)
        }
    }
}
impl EncodingRecordRepr {
    pub const fn subtable<'a>(&'a self, cmap: &'a CmapTableRepr) -> &'a CmapSubtableRepr {
        unsafe { &*std::ptr::from_ref(cmap).byte_add(self.subtable_offset.get() as _).cast() }
    }
}

#[repr(C)]
pub struct CmapSubtableRepr {
    pub format: uint16,
    pub length: uint16,
    pub language: uint16,
    data: [u8; 0],
}

impl CmapSubtableRepr {
    pub const fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), self.length.get() as _) }
    }
}

pub trait CmapSubtable {
    type Iter<'a>: Iterator<Item = (char, u32)>
    where Self: 'a;
    fn glyph_id(&self, codepoint: char) -> Option<u32>;
    fn codepoint(&self, glyph_id: u32) -> Option<char>;
    fn iter(&self) -> Self::Iter<'_>;
}
