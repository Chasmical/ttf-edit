use crate::{
    types::{Version16Dot16, uint16},
    util::{Describe, Describer, StructDescriber, describe, describe_impl},
};

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

impl Describe for MaxpTableRepr {
    fn describe<D: Describer>(&self, d: D) -> Result<D::Ok, D::Error> {
        let mut d = d.describe_struct("MaxpTable");
        describe!(d, self { version, num_glyphs });

        if let Some(v1) = self.v1_fields() {
            describe!(d, v1 {
                max_points,
                max_contours,
                max_composite_points,
                max_composite_contours,
                max_zones,
                max_twilight_points,
                max_storage,
                max_function_defs,
                max_instruction_defs,
                max_stack_elements,
                max_size_of_instructions,
                max_component_elements,
                max_component_depth,
            });
        }

        d.finish()
    }
}

describe_impl! { Debug, Serialize for MaxpTableRepr }
