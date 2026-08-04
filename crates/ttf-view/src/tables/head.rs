use crate::types::{Fixed, LongDateTime, int16, uint16, uint32};
use std::fmt;

#[repr(C)]
pub struct HeadTableRepr {
    pub major_version: uint16,
    pub minor_version: uint16,
    pub font_revision: Fixed,
    pub checksum_adjustment: uint32,
    pub magic_number: uint32,
    pub flags: uint16,
    pub units_per_em: uint16,
    pub created: LongDateTime,
    pub modified: LongDateTime,
    pub x_min: int16,
    pub y_min: int16,
    pub x_max: int16,
    pub y_max: int16,
    pub mac_style: uint16,
    pub lowest_rec_ppem: uint16,
    pub font_direction_hint: int16,
    pub index_to_loc_format: int16,
    pub glyph_data_format: int16,
}

impl fmt::Debug for HeadTableRepr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HeadTableRepr")
            .field("major_version", &self.major_version.get())
            .field("minor_version", &self.minor_version.get())
            .field("font_revision", &self.font_revision)
            .field_with("checksum_adjustment", |f| write!(f, "{:#010X}", self.checksum_adjustment))
            .field_with("magic_number", |f| write!(f, "{:#010X}", self.magic_number))
            .field_with("flags", |f| write!(f, "{:#017b}", self.flags))
            .field("units_per_em", &self.units_per_em.get())
            .field("created", &self.created)
            .field("modified", &self.modified)
            .field("x_min", &self.x_min.get())
            .field("y_min", &self.y_min.get())
            .field("x_max", &self.x_max.get())
            .field("y_max", &self.y_max.get())
            .field_with("mac_style", |f| write!(f, "{:#09b}", self.mac_style))
            .field("lowest_rec_ppem", &self.lowest_rec_ppem.get())
            .field("font_direction_hint", &self.font_direction_hint.get())
            .field("index_to_loc_format", &self.index_to_loc_format.get())
            .field("glyph_data_format", &self.glyph_data_format.get())
            .finish()
    }
}
