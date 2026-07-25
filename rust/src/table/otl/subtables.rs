pub mod chaining;
pub mod extend;
pub mod gpos_common;
pub mod gpos_cursive;
pub mod gpos_mark_to_ligature;
pub mod gpos_mark_to_single;
pub mod gpos_pair;
pub mod gpos_single;
pub mod gsub_ligature;
pub mod gsub_multi;
pub mod gsub_reverse;
pub mod gsub_single;

pub type otl_BuildHeuristics = ::core::ffi::c_uint;

pub const OTL_BH_GSUB_VERT: otl_BuildHeuristics = 1;

pub const OTL_BH_NORMAL: otl_BuildHeuristics = 0;
