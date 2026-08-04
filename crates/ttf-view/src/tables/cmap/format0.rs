use crate::tables::cmap::GlyphId;

#[repr(C)]
pub struct CmapSubtableFormat0Repr {
    pub glyph_id_array: [u8; 256],
}

impl CmapSubtableFormat0Repr {
    pub const fn map(&self, codepoint: u8) -> GlyphId {
        self.glyph_id_array[codepoint as usize].into()
    }
}
