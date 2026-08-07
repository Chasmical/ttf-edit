use crate::types::{Version16Dot16, uint16};
use std::fmt;

#[repr(C)]
pub struct MaxpTableRepr {
    // version ≥ 0.5:
    pub version: Version16Dot16,
    pub num_glyphs: uint16,
    // version ≥ 1.0:
    v1_fields: MaxpTableReprV1Fields,
}

#[repr(C)]
pub struct MaxpTableReprV1Fields {
    pub max_points: uint16,
    pub max_contours: uint16,
    pub max_composite_points: uint16,
    pub max_composite_contours: uint16,
    pub max_zones: uint16,
    pub max_twilight_points: uint16,
    pub max_storage: uint16,
    pub max_function_defs: uint16,
    pub max_instruction_defs: uint16,
    pub max_stack_elements: uint16,
    pub max_size_of_instructions: uint16,
    pub max_component_elements: uint16,
    pub max_component_depth: uint16,
}

impl MaxpTableRepr {
    pub const fn v1_fields(&self) -> Option<&MaxpTableReprV1Fields> {
        const V1_0: Version16Dot16 = Version16Dot16::new(1, 0).unwrap();
        if self.version >= V1_0 { Some(&self.v1_fields) } else { None }
    }
}

impl fmt::Debug for MaxpTableRepr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("MaxpTable");
        builder.field("version", &self.version).field("num_glyphs", &self.num_glyphs.get());

        if let Some(v1) = self.v1_fields() {
            builder
                .field("max_points", &v1.max_points.get())
                .field("max_contours", &v1.max_contours.get())
                .field("max_composite_points", &v1.max_composite_points.get())
                .field("max_composite_contours", &v1.max_composite_contours.get())
                .field("max_zones", &v1.max_zones.get())
                .field("max_twilight_points", &v1.max_twilight_points.get())
                .field("max_storage", &v1.max_storage.get())
                .field("max_function_defs", &v1.max_function_defs.get())
                .field("max_instruction_defs", &v1.max_instruction_defs.get())
                .field("max_stack_elements", &v1.max_stack_elements.get())
                .field("max_size_of_instructions", &v1.max_size_of_instructions.get())
                .field("max_component_elements", &v1.max_component_elements.get())
                .field("max_component_depth", &v1.max_component_depth.get());
        }

        builder.finish()
    }
}
