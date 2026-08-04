use crate::types::{Offset32, uint16};

mod codepoint;
mod glyph_id;

pub use codepoint::*;
pub use glyph_id::*;

pub mod format0;
pub mod format2;
pub mod format4;
pub mod format6;
pub mod format8;

pub mod format10;
pub mod format12;
pub mod format13;
pub mod format14;

#[repr(C)]
#[non_exhaustive]
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
