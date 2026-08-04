use crate::types::{Offset32, uint16, uint24, uint32};

#[repr(C)]
#[non_exhaustive]
pub struct CmapSubtableFormat14Repr {
    pub num_var_selector_records: uint32,
    var_selectors: [VariationSelectorRepr; 0],
}

#[repr(C)]
pub struct VariationSelectorRepr {
    pub var_selector: uint24,
    pub default_uvs_offset: Offset32,
    pub non_default_uvs_offset: Offset32,
}

#[repr(C)]
#[non_exhaustive]
pub struct DefaultUvsTableRepr {
    pub num_unicode_value_ranges: uint32,
    ranges: [UnicodeRangeRepr; 0],
}

#[repr(C)]
pub struct UnicodeRangeRepr {
    pub start_unicode_value: uint24,
    pub additional_count: u8,
}

#[repr(C)]
#[non_exhaustive]
pub struct NonDefaultUvsTableRepr {
    pub num_uvs_mappings: uint32,
    uvs_mappings: [UvsMappingRepr; 0],
}

#[repr(C)]
pub struct UvsMappingRepr {
    pub unicode_value: uint24,
    pub glyph_id: uint16,
}

// TODO: 'cmap' subtable format 14
