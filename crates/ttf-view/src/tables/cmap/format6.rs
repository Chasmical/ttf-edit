use crate::types::uint16;

#[repr(C)]
#[non_exhaustive]
pub struct CmapSubtableFormat6Repr {
    pub first_code: uint16,
    pub entry_count: uint16,
    glyph_id_array: [uint16; 0],
}

// TODO: 'cmap' subtable format 6
