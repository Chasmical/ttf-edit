use crate::types::uint16;

#[repr(C)]
#[non_exhaustive]
pub struct CmapSubtableFormat4Repr {
    seg_count_x2: uint16,
    pub search_range: uint16,
    pub entry_selector: uint16,
    pub range_shift: uint16,
    data: [uint16; 0],
    // : end_code: [uint16; seg_count]
    // : reserved_pad: uint16
    // : start_code: [uint16; seg_count]
    // : id_delta: [uint16; seg_count]
    // : id_range_offset: [uint16; seg_count]
    // : glyph_id_array: [uint16; arbitrary length]
}
