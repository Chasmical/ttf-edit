use crate::types::{uint16, uint32};

#[repr(C)]
#[non_exhaustive]
pub struct CmapSubtableFormat10Repr {
    pub start_char_code: uint32,
    pub num_chars: uint32,
    glyph_id_array: [uint16; 0],
}

// TODO: 'cmap' subtable format 10
