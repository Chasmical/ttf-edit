use crate::tables::cmap::CmapSubtable;
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

    fn glyph_id(&self, codepoint: char) -> Option<u32> {
        let id = *self.glyph_id_array.get(codepoint as usize)?;
        if id != 0 { Some(id as u32) } else { None }
    }
    fn codepoint(&self, glyph_id: u32) -> Option<char> {
        let glyph_id: u8 = glyph_id.try_into().ok()?;
        let idx = self.glyph_id_array.iter().position(|&id| id == glyph_id)?;
        Some(unsafe { char::from_u32_unchecked(idx as u32) })
    }
    fn iter(&self) -> Self::Iter<'_> {
        Iter(self.glyph_id_array.iter().enumerate())
    }
}

impl<'a> IntoIterator for &'a CmapSubtable0 {
    type Item = (char, u32);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a>(Enumerate<std::slice::Iter<'a, u8>>);

impl<'a> Iterator for Iter<'a> {
    type Item = (char, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let (idx, &id) = self.0.next()?;
        let cp = unsafe { char::from_u32_unchecked(idx as u32) };
        Some((cp, id as u32))
    }
}
