use crate::types::uint32;

#[repr(C)]
#[non_exhaustive]
pub struct CmapSubtableFormat8Repr {
    pub is32: [u8; 8192],
    pub num_groups: uint32,
    groups: [SequentialMapGroupRepr; 0],
}

#[repr(C)]
pub struct SequentialMapGroupRepr {
    pub start_char_code: uint32,
    pub end_char_code: uint32,
    pub start_glyph_id: uint32,
}

// TODO: 'cmap' subtable format 8
