use crate::{
    tables::cmap::GlyphId,
    types::{int16, uint16},
};

#[repr(C)]
#[non_exhaustive]
pub struct CmapSubtableFormat2Repr {
    pub sub_header_keys: [uint16; 256],
    sub_headers: [SubHeaderRepr; 0],
    glyph_id_array: [uint16; 0],
}

#[repr(C)]
#[non_exhaustive]
pub struct SubHeaderRepr {
    pub first_code: uint16,
    pub entry_count: uint16,
    pub id_delta: int16,
    pub id_range_offset: uint16,
}

impl CmapSubtableFormat2Repr {
    pub fn sub_headers(&self) -> &[SubHeaderRepr] {
        let max_offset = self.sub_header_keys.iter().max().unwrap().get();
        let count = (max_offset / 8) + 1;
        unsafe { std::slice::from_raw_parts(self.sub_headers.as_ptr(), count as _) }
    }
    pub const fn sub_header(&self, high_byte: u8) -> &SubHeaderRepr {
        let offset = self.sub_header_keys[high_byte as usize].get();
        unsafe { &*self.sub_headers.as_ptr().byte_add(offset as usize) }
    }

    pub const fn map_one(&self, single_byte: u8) -> GlyphId {
        let sub_header_zero = unsafe { &*self.sub_headers.as_ptr() };
        sub_header_zero.map(single_byte)
    }
    pub const fn map_two(&self, high_byte: u8, low_byte: u8) -> GlyphId {
        self.sub_header(high_byte).map(low_byte)
    }
}

impl SubHeaderRepr {
    pub const fn map(&self, low_byte: u8) -> GlyphId {
        let offset = (low_byte as u16).checked_sub(self.first_code.get())?;
        if offset > self.entry_count.get() {
            return GlyphId::NOTDEF;
        }

        let result = unsafe { &*std::ptr::from_ref(&self.id_range_offset).add(offset as _) }.get();
        let id_delta = if result == 0 { 0 } else { self.id_delta.get() as u16 };

        GlyphId::new(result.wrapping_add(id_delta))
    }
}
