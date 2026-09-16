#[derive(Copy, Clone)]
pub struct VdmxRecord {
    pub y_pel_height: u16,
    pub y_max: i16,
    pub y_min: i16,
}
#[derive(Clone)]
pub struct VdmxRatioRange {
    pub b_charset: u8,
    pub x_ratio: u8,
    pub y_start_ratio: u8,
    pub y_end_ratio: u8,
    pub records: Vec<VdmxRecord>,
}
// Stage 6-4 "Box化": every field `VdmxTable` (transitively) owns is already
// a `Vec`/scalar, so no `Drop` impl is needed at all -- `Box::new`
// construction plus the standard drop glue is sufficient. The entire
// `VdmxTableElementInterface` vtable is deleted: grepping confirmed only
// `.create`/`.free` were ever called from outside this file (from
// `vdmx/funcs.rs`), and `.free`'s job (`table_vdmx_free`/`_dispose`) reduces
// to nothing once there's no raw pointer left to release -- `Box`'s own
// drop already runs `Vec`'s drop glue.
#[derive(Clone)]
pub struct VdmxTable {
    pub version: u16,
    pub ratios: Vec<VdmxRatioRange>,
}
