use super::*;

use otfcc_json::BuiltValue;
use crate::table::fvar::json_new_vq;

fn pd_delta_to_json(target: &mut BuiltValue, field: &[u8], values: &[f64]) {
    if values.is_empty() {
        return;
    }
    let mut a = BuiltValue::new_array(values.len());
    for &x in values {
        a.push_item(BuiltValue::Double(x));
    }
    target.push_field(field, a);
}
fn pd_to_json(pd: &CffPrivateDict) -> BuiltValue {
    let mut _pd = BuiltValue::new_object(24);
    pd_delta_to_json(&mut _pd, b"blueValues", &pd.blue_values);
    pd_delta_to_json(&mut _pd, b"otherBlues", &pd.other_blues);
    pd_delta_to_json(&mut _pd, b"familyBlues", &pd.family_blues);
    pd_delta_to_json(&mut _pd, b"familyOtherBlues", &pd.family_other_blues);
    pd_delta_to_json(&mut _pd, b"stemSnapH", &pd.stem_snap_h);
    pd_delta_to_json(&mut _pd, b"stemSnapV", &pd.stem_snap_v);
    if pd.blue_scale != DEFAULT_BLUE_SCALE {
        _pd.push_field(b"blueScale", BuiltValue::Double(pd.blue_scale));
    }
    if pd.blue_shift != DEFAULT_BLUE_SHIFT {
        _pd.push_field(b"blueShift", BuiltValue::Double(pd.blue_shift));
    }
    if pd.blue_fuzz != DEFAULT_BLUE_FUZZ {
        _pd.push_field(b"blueFuzz", BuiltValue::Double(pd.blue_fuzz));
    }
    if pd.std_hw != 0. {
        _pd.push_field(b"stdHW", BuiltValue::Double(pd.std_hw));
    }
    if pd.std_vw != 0. {
        _pd.push_field(b"stdVW", BuiltValue::Double(pd.std_vw));
    }
    if pd.force_bold {
        _pd.push_field(b"forceBold", BuiltValue::Bool(pd.force_bold));
    }
    if pd.language_group != 0 {
        _pd.push_field(
            b"languageGroup",
            BuiltValue::Double(pd.language_group as f64),
        );
    }
    if pd.expansion_factor != DEFAULT_EXPANSION_FACTOR {
        _pd.push_field(b"expansionFactor", BuiltValue::Double(pd.expansion_factor));
    }
    if pd.initial_random_seed != 0. {
        _pd.push_field(
            b"initialRandomSeed",
            BuiltValue::Double(pd.initial_random_seed),
        );
    }
    if pd.default_width_x != 0. {
        _pd.push_field(b"defaultWidthX", BuiltValue::Double(pd.default_width_x));
    }
    if pd.nominal_width_x != 0. {
        _pd.push_field(b"nominalWidthX", BuiltValue::Double(pd.nominal_width_x));
    }
    _pd
}
fn fd_to_json(table: &CffTable) -> BuiltValue {
    let mut _cff = BuiltValue::new_object(24);
    if table.is_cid {
        _cff.push_field(b"isCID", BuiltValue::Bool(table.is_cid));
    }
    if !table.version.is_empty() {
        _cff.push_field(b"version", json_from_sds(&table.version));
    }
    if !table.notice.is_empty() {
        _cff.push_field(b"notice", json_from_sds(&table.notice));
    }
    if !table.copyright.is_empty() {
        _cff.push_field(b"copyright", json_from_sds(&table.copyright));
    }
    if !table.font_name.is_empty() {
        _cff.push_field(b"fontName", json_from_sds(&table.font_name));
    }
    if !table.full_name.is_empty() {
        _cff.push_field(b"fullName", json_from_sds(&table.full_name));
    }
    if !table.family_name.is_empty() {
        _cff.push_field(b"familyName", json_from_sds(&table.family_name));
    }
    if !table.weight.is_empty() {
        _cff.push_field(b"weight", json_from_sds(&table.weight));
    }
    if table.is_fixed_pitch {
        _cff.push_field(b"isFixedPitch", BuiltValue::Bool(table.is_fixed_pitch));
    }
    if table.italic_angle != 0. {
        _cff.push_field(b"italicAngle", BuiltValue::Double(table.italic_angle));
    }
    if table.underline_position != -100.0 {
        _cff.push_field(
            b"underlinePosition",
            BuiltValue::Double(table.underline_position),
        );
    }
    if table.underline_thickness != 50.0 {
        _cff.push_field(
            b"underlineThickness",
            BuiltValue::Double(table.underline_thickness),
        );
    }
    if table.stroke_width != 0. {
        _cff.push_field(b"strokeWidth", BuiltValue::Double(table.stroke_width));
    }
    if table.font_b_box_left != 0. {
        _cff.push_field(b"fontBBoxLeft", BuiltValue::Double(table.font_b_box_left));
    }
    if table.font_b_box_bottom != 0. {
        _cff.push_field(
            b"fontBBoxBottom",
            BuiltValue::Double(table.font_b_box_bottom),
        );
    }
    if table.font_b_box_right != 0. {
        _cff.push_field(b"fontBBoxRight", BuiltValue::Double(table.font_b_box_right));
    }
    if table.font_b_box_top != 0. {
        _cff.push_field(b"fontBBoxTop", BuiltValue::Double(table.font_b_box_top));
    }
    if let Some(fm) = table.font_matrix.as_deref() {
        let mut _font_matrix = BuiltValue::new_object(6);
        _font_matrix.push_field(b"a", BuiltValue::Double(fm.a));
        _font_matrix.push_field(b"b", BuiltValue::Double(fm.b));
        _font_matrix.push_field(b"c", BuiltValue::Double(fm.c));
        _font_matrix.push_field(b"d", BuiltValue::Double(fm.d));
        // CFF fonts have no `fvar` table (no `Delta` segments can occur
        // here -- `json_new_vq` panics if that invariant is ever wrong).
        _font_matrix.push_field(b"x", json_new_vq(fm.x.clone(), None));
        _font_matrix.push_field(b"y", json_new_vq(fm.y.clone(), None));
        _cff.push_field(b"fontMatrix", _font_matrix);
    }
    if let Some(pd) = table.private_dict.as_deref() {
        _cff.push_field(b"privates", pd_to_json(pd));
    }
    if !table.cid_registry.is_empty() && !table.cid_ordering.is_empty() {
        _cff.push_field(b"cidRegistry", json_from_sds(&table.cid_registry));
        _cff.push_field(b"cidOrdering", json_from_sds(&table.cid_ordering));
        _cff.push_field(
            b"cidSupplement",
            BuiltValue::Int(table.cid_supplement as i64),
        );
    }
    if !table.fd_array.is_empty() {
        let mut _fd_array = BuiltValue::new_object(table.fd_array.len());
        for fd in &table.fd_array {
            // An FD's name is already its key in `fdArray`, so drop the
            // `fontName` field from its object.
            let mut child = fd_to_json(fd);
            if let BuiltValue::Object(fields) = &mut child {
                fields.retain(|(k, _)| k != b"fontName");
            }
            _fd_array.push_field_bytes_key(&fd.font_name, child);
        }
        _cff.push_field(b"fdArray", _fd_array);
    }
    _cff
}
pub fn dump_cff(table: Option<&CffTable>, root: &mut BuiltValue) {
    let Some(table) = table else {
        return;
    };
    let stage = crate::logger::stage("CFF");
    root.push_field(b"CFF_", fd_to_json(table));
    stage.finish();
}
#[inline]
fn json_from_sds(str: &[u8]) -> BuiltValue {
    BuiltValue::Str(str.to_vec())
}
