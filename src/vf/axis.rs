use crate::support::primitives::Pos;

#[derive(Copy, Clone, Debug)]
pub struct VfAxis {
    pub tag: u32,
    pub min_value: Pos,
    pub default_value: Pos,
    pub max_value: Pos,
    pub flags: u16,
    pub axis_name_id: u16,
}
pub type VfAxes = Vec<VfAxis>;
