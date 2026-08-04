use crate::types::{Offset32, uint16, uint32};
use std::mem::ManuallyDrop;

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
#[non_exhaustive]
pub struct CmapSubtableRepr {
    pub format: uint16,
    meta: SubtableMeta,
}

/// The offsets and sizes of `length` and `language` depend on the subtable's format:
///
/// ```text
///                                       CmapSubtableRepr
///                 ┌──────────────────────────────┐────────────────────────────┐
///                                               SubtableMeta
///                             ┌───────────────────────────────────────────────┐
///                 0    1      2    3    4    5    6
///  ShortMeta     ┌────┬────┐┈┌────┬────┬────┬────┐
///  f 0,2,4,6     │ format  │ │ length  │language │
///                └─────────┘┈└─────────┴─────────┘
///                 0    1      2    3    4    5    6    7    8    9    10   11   12
///  LongMeta      ┌────┬────┐┈┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐
///  f 8,10,12,13  │ format  │ │reserved │      length       │     language      │
///                └─────────┘┈└─────────┴───────────────────┴───────────────────┘
///                 0    1      2    3    4    5    6
///  LenOnlyMeta   ┌────┬────┐┈┌────┬────┬────┬────┐
///  f 14          │ format  │ │      length       │
///                └─────────┘┈└───────────────────┘
/// ```
#[repr(C)]
union SubtableMeta {
    short: ManuallyDrop<ShortMeta>,
    long: ManuallyDrop<LongMeta>,
    len_only: ManuallyDrop<LenOnlyMeta>,
}

#[repr(C)]
#[non_exhaustive]
struct ShortMeta {
    length: uint16,
    language: uint16,
    data: [u8; 0],
}
#[repr(C)]
#[non_exhaustive]
struct LongMeta {
    reserved: uint16,
    length: uint32,
    language: uint32,
    data: [u8; 0],
}
#[repr(C)]
#[non_exhaustive]
struct LenOnlyMeta {
    length: uint32,
    data: [u8; 0],
}

impl CmapSubtableRepr {
    pub const fn length(&self) -> Option<u32> {
        Some(match self.format.get() {
            0 | 2 | 4 | 6 => unsafe { self.meta.short.length.get() as _ },
            8 | 10 | 12 | 13 => unsafe { self.meta.long.length.get() },
            14 => unsafe { self.meta.len_only.length.get() },
            _ => return None,
        })
    }
    pub const fn language(&self) -> Option<u32> {
        Some(match self.format.get() {
            0 | 2 | 4 | 6 => unsafe { self.meta.short.language.get() as _ },
            8 | 10 | 12 | 13 => unsafe { self.meta.long.language.get() },
            _ => return None,
        })
    }
    pub const fn data_ptr(&self) -> Option<&u8> {
        Some(match self.format.get() {
            0 | 2 | 4 | 6 => unsafe { &*self.meta.short.data.as_ptr() },
            8 | 10 | 12 | 13 => unsafe { &*self.meta.long.data.as_ptr() },
            14 => unsafe { &*self.meta.len_only.data.as_ptr() },
            _ => return None,
        })
    }
    pub const fn data(&self) -> Option<&[u8]> {
        let (start, len) = match self.format.get() {
            0 | 2 | 4 | 6 => unsafe { (&self.meta.short.data, self.meta.short.length.get() as _) },
            8 | 10 | 12 | 13 => unsafe { (&self.meta.long.data, self.meta.long.length.get()) },
            14 => unsafe { (&self.meta.len_only.data, self.meta.len_only.length.get()) },
            _ => return None,
        };
        Some(unsafe { std::slice::from_raw_parts(start.as_ptr(), len as _) })
    }
}
