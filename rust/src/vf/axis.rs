use crate::support::primitives::Pos;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfAxis {
    pub tag: u32,
    pub min_value: Pos,
    pub default_value: Pos,
    pub max_value: Pos,
    pub flags: u16,
    pub axis_name_id: u16,
}
// C由来の時点で素のベクタ形。要素は所有物なし（旧 `vf_axis_dispose` は空実装
// だった）。テーブル全体の `.copy`（`VF_I_AXES.copy`）は crate 全体で一度も
// 呼ばれておらず削除——生存していたのは `.init`/`.push`/`.shrink_to_fit`/
// `.dispose` のみで、いずれも `table/fvar.rs` から直接 `Vec` のメソッドに
// 置き換えられる。
pub type VfAxes = Vec<VfAxis>;
