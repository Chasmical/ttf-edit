use crate::types::{FWORD, UFWORD, int16, uint16};

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
