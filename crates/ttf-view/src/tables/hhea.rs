use crate::{
    types::{FWORD, UFWORD, int16, uint16},
    util::{Describe, Describer, describe, describe_impl},
};

#[repr(C)]
pub struct HheaTableRepr {
    pub major_version: uint16,
    pub minor_version: uint16,
    pub ascender: FWORD,
    pub descender: FWORD,
    pub line_gap: FWORD,
    pub advance_width_max: UFWORD,
    pub min_left_side_bearing: FWORD,
    pub min_right_side_bearing: FWORD,
    pub x_max_extent: FWORD,
    pub caret_slope_rise: int16,
    pub caret_slope_run: int16,
    pub caret_offset: int16,
    pub reserved0: int16,
    pub reserved1: int16,
    pub reserved2: int16,
    pub reserved3: int16,
    pub metric_data_format: int16,
    pub number_of_h_metrics: uint16,
}

impl Describe for HheaTableRepr {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        describe!(d, self as "HheaTable" {
            major_version,
            minor_version,
            ascender,
            descender,
            line_gap,
            advance_width_max,
            min_left_side_bearing,
            min_right_side_bearing,
            x_max_extent,
            caret_slope_rise,
            caret_slope_run,
            caret_offset,
            reserved0,
            reserved1,
            reserved2,
            reserved3,
            metric_data_format,
            number_of_h_metrics,
        })
    }
}

describe_impl! { Debug, Serialize for HheaTableRepr }
