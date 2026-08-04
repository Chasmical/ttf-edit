use crate::types::{Version16Dot16, uint16};

#[repr(C)]
pub struct MaxpTableRepr {
    // version ≥ 0.5:
    pub version: Version16Dot16,
    pub num_glyphs: uint16,
    // version ≥ 1.0:
    max_points: uint16,
    max_contours: uint16,
    max_composite_points: uint16,
    max_composite_contours: uint16,
    max_zones: uint16,
    max_twilight_points: uint16,
    max_storage: uint16,
    max_function_defs: uint16,
    max_instruction_defs: uint16,
    max_stack_elements: uint16,
    max_size_of_instructions: uint16,
    max_component_elements: uint16,
    max_component_depth: uint16,
}

impl MaxpTableRepr {
    const fn v1_0<T: [const] std::marker::Destruct>(&self, value: T) -> Option<T> {
        const V1_0: Version16Dot16 = Version16Dot16::new(1, 0).unwrap();
        if self.version >= V1_0 { Some(value) } else { None }
    }

    pub const fn max_points(&self) -> Option<uint16> {
        self.v1_0(self.max_points)
    }
    pub const fn max_contours(&self) -> Option<uint16> {
        self.v1_0(self.max_contours)
    }
    pub const fn max_composite_points(&self) -> Option<uint16> {
        self.v1_0(self.max_composite_points)
    }
    pub const fn max_composite_contours(&self) -> Option<uint16> {
        self.v1_0(self.max_composite_contours)
    }
    pub const fn max_zones(&self) -> Option<uint16> {
        self.v1_0(self.max_zones)
    }
    pub const fn max_twilight_points(&self) -> Option<uint16> {
        self.v1_0(self.max_twilight_points)
    }
    pub const fn max_storage(&self) -> Option<uint16> {
        self.v1_0(self.max_storage)
    }
    pub const fn max_function_defs(&self) -> Option<uint16> {
        self.v1_0(self.max_function_defs)
    }
    pub const fn max_instruction_defs(&self) -> Option<uint16> {
        self.v1_0(self.max_instruction_defs)
    }
    pub const fn max_stack_elements(&self) -> Option<uint16> {
        self.v1_0(self.max_stack_elements)
    }
    pub const fn max_size_of_instructions(&self) -> Option<uint16> {
        self.v1_0(self.max_size_of_instructions)
    }
    pub const fn max_component_elements(&self) -> Option<uint16> {
        self.v1_0(self.max_component_elements)
    }
    pub const fn max_component_depth(&self) -> Option<uint16> {
        self.v1_0(self.max_component_depth)
    }
}
