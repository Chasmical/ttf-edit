use crate::tables::cmap::{CmapSubtable, Codepoint, GlyphId};
use std::iter::Enumerate;

#[derive(Debug, Copy, Hash)]
#[derive_const(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct CmapSubtable0 {
    pub glyph_id_array: [u8; 256],
}

const impl Default for CmapSubtable0 {
    fn default() -> Self {
        Self { glyph_id_array: [0; 256] }
    }
}

impl CmapSubtable for CmapSubtable0 {
    type Iter<'a> = Iter<'a>;

    fn glyph_id(&self, codepoint: Codepoint) -> Option<GlyphId> {
        let id = *self.glyph_id_array.get(usize::from(codepoint))?;
        id.try_into().ok()
    }
    fn codepoint(&self, glyph_id: GlyphId) -> Option<Codepoint> {
        let glyph_id: u8 = glyph_id.try_into().ok()?;
        let idx = self.glyph_id_array.iter().position(|&id| id == glyph_id)?;
        Some((idx as u32).into())
    }
    fn iter(&self) -> Self::Iter<'_> {
        Iter(self.glyph_id_array.iter().enumerate())
    }
}

impl<'a> IntoIterator for &'a CmapSubtable0 {
    type Item = (Codepoint, GlyphId);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a>(Enumerate<std::slice::Iter<'a, u8>>);

impl<'a> Iterator for Iter<'a> {
    type Item = (Codepoint, GlyphId);

    fn next(&mut self) -> Option<Self::Item> {
        let (idx, &id) = self.0.next()?;
        Some(((idx as u32).into(), id.try_into().ok()?))
    }
}
