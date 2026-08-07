use crate::types::{FWORD, UFWORD, int16, uint16};
use std::fmt;

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

impl fmt::Debug for HheaTableRepr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HheaTable")
            .field("major_version", &self.major_version.get())
            .field("minor_version", &self.minor_version.get())
            .field("ascender", &self.ascender.get())
            .field("descender", &self.descender.get())
            .field("line_gap", &self.line_gap.get())
            .field("advance_width_max", &self.advance_width_max.get())
            .field("min_left_side_bearing", &self.min_left_side_bearing.get())
            .field("min_right_side_bearing", &self.min_right_side_bearing.get())
            .field("x_max_extent", &self.x_max_extent.get())
            .field("caret_slope_rise", &self.caret_slope_rise.get())
            .field("caret_slope_run", &self.caret_slope_run.get())
            .field("caret_offset", &self.caret_offset.get())
            .field("reserved0", &self.reserved0.get())
            .field("reserved1", &self.reserved1.get())
            .field("reserved2", &self.reserved2.get())
            .field("reserved3", &self.reserved3.get())
            .field("metric_data_format", &self.metric_data_format.get())
            .field("number_of_h_metrics", &self.number_of_h_metrics.get())
            .finish()
    }
}
