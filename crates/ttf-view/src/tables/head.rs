use crate::{
    types::{Fixed, LongDateTime, Tag, int16, tags, uint16, uint32},
    util::{Describe, Describer, describe, describe_impl},
};

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

impl super::Table for HeadTableRepr {
    const TAG: Tag = tags::head;
    type Handle<'a> = &'a Self;
}

impl Describe for HeadTableRepr {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        describe!(d, self as "HeadTable" {
            major_version,
            minor_version,
            font_revision ["{}"],
            checksum_adjustment ["{:#010X}"],
            magic_number ["{:#010X}"],
            flags ["{:#018b}"],
            units_per_em,
            created ["{:?}"],
            modified ["{:?}"],
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style ["{:#018b}"],
            lowest_rec_ppem,
            font_direction_hint,
            index_to_loc_format,
            glyph_data_format,
        })
    }
}
describe_impl! { Debug, Serialize for HeadTableRepr }
