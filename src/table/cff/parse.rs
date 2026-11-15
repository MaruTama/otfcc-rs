use super::*;

use crate::support::options::Options;
use otfcc_json::JsonType;

use otfcc_json::ParsedValue;

fn pd_delta_from_json(dump: Option<&ParsedValue>) -> Vec<f64> {
    let Some(items) = dump.and_then(ParsedValue::as_array) else {
        return Vec::new();
    };
    items.iter().map(|v| v.as_num().unwrap_or(0.0)).collect()
}
fn pd_from_json(dump: Option<&ParsedValue>) -> Option<Box<CffPrivateDict>> {
    let dump = dump.filter(|v| v.as_object().is_some())?;
    let mut pd_box: Box<CffPrivateDict> = new_cff_private();
    pd_box.blue_values = pd_delta_from_json(dump.get(b"blueValues"));
    pd_box.other_blues = pd_delta_from_json(dump.get(b"otherBlues"));
    pd_box.family_blues = pd_delta_from_json(dump.get(b"familyBlues"));
    pd_box.family_other_blues = pd_delta_from_json(dump.get(b"familyOtherBlues"));
    pd_box.stem_snap_h = pd_delta_from_json(dump.get(b"stemSnapH"));
    pd_box.stem_snap_v = pd_delta_from_json(dump.get(b"stemSnapV"));
    pd_box.blue_scale = dump.get_num_or(b"blueScale", DEFAULT_BLUE_SCALE);
    pd_box.blue_shift = dump.get_num_or(b"blueShift", DEFAULT_BLUE_SHIFT);
    pd_box.blue_fuzz = dump.get_num_or(b"blueFuzz", DEFAULT_BLUE_FUZZ);
    pd_box.std_hw = dump.get_num(b"stdHW");
    pd_box.std_vw = dump.get_num(b"stdVW");
    pd_box.force_bold = dump.get_bool(b"forceBold");
    pd_box.language_group = dump.get_num(b"languageGroup") as u32;
    pd_box.expansion_factor = dump.get_num_or(b"expansionFactor", DEFAULT_EXPANSION_FACTOR);
    pd_box.initial_random_seed = dump.get_num(b"initialRandomSeed");
    Some(pd_box)
}
// Builds the whole tree of `CffTable`/`fd_array` children as owned local
// values first, `Box`ing each one only once it's fully populated.
fn fd_from_json(dump: Option<&ParsedValue>, options: &Options, top_level: bool) -> Box<CffTable> {
    let mut table = table_cff_new();
    let Some(dump) = dump.filter(|v| v.as_object().is_some()) else {
        return table;
    };
    table.version = dump.get_bytes_owned(b"version").unwrap_or_default();
    table.notice = dump.get_bytes_owned(b"notice").unwrap_or_default();
    table.copyright = dump.get_bytes_owned(b"copyright").unwrap_or_default();
    table.font_name = dump.get_bytes_owned(b"fontName").unwrap_or_default();
    table.full_name = dump.get_bytes_owned(b"fullName").unwrap_or_default();
    table.family_name = dump.get_bytes_owned(b"familyName").unwrap_or_default();
    table.weight = dump.get_bytes_owned(b"weight").unwrap_or_default();
    table.is_fixed_pitch = dump.get_bool(b"isFixedPitch");
    table.italic_angle = dump.get_num(b"italicAngle");
    table.underline_position = dump.get_num_or(b"underlinePosition", -100.0);
    table.underline_thickness = dump.get_num_or(b"underlineThickness", 50.0);
    table.stroke_width = dump.get_num(b"strokeWidth");
    table.font_b_box_left = dump.get_num(b"fontBBoxLeft");
    table.font_b_box_bottom = dump.get_num(b"fontBBoxBottom");
    table.font_b_box_right = dump.get_num(b"fontBBoxRight");
    table.font_b_box_top = dump.get_num(b"fontBBoxTop");
    table.private_dict = pd_from_json(dump.get_typed(b"privates", JsonType::Object));
    table.cid_registry = dump.get_bytes_owned(b"cidRegistry").unwrap_or_default();
    table.cid_ordering = dump.get_bytes_owned(b"cidOrdering").unwrap_or_default();
    table.cid_supplement = dump.get_int(b"cidSupplement") as u32;
    table.uid_base = dump.get_int(b"UIDBase") as u32;
    table.cid_count = dump.get_int(b"cidCount") as u32;
    table.cid_font_version = dump.get_num(b"cidFontVersion");
    table.cid_font_revision = dump.get_num(b"cidFontRevision");
    if let Some(fields) = dump
        .get_typed(b"fdArray", JsonType::Object)
        .and_then(ParsedValue::as_object)
    {
        table.is_cid = true;
        table.fd_array = Vec::with_capacity(fields.len());
        for (key, val) in fields {
            // `fd_from_json` builds each child fully before returning
            // (unlike the binary-read path, which populates a `fd_array`
            // slot incrementally via a recursive callback) -- so there's
            // no need to push an empty placeholder first here.
            let mut fd_box: Box<CffTable> = fd_from_json(Some(val), options, false);
            fd_box.font_name = key[..key.len() - 1].to_vec();
            table.fd_array.push(fd_box);
        }
    }
    if table.font_name.is_empty() {
        table.font_name = b"CARYLL_CFFFONT".to_vec();
    }
    if table.private_dict.is_none() {
        table.private_dict = Some(new_cff_private());
    }
    if top_level && options.force_cid && table.fd_array.is_empty() {
        let mut fd0_box: Box<CffTable> = table_cff_new();
        fd0_box.private_dict = table.private_dict.take();
        table.private_dict = Some(new_cff_private());
        let mut subfont0_name = table.font_name.clone();
        subfont0_name.extend_from_slice(b"-subfont0");
        fd0_box.font_name = subfont0_name;
        table.fd_array.push(fd0_box);
        table.is_cid = true;
    }
    if table.is_cid && table.cid_registry.is_empty() {
        table.cid_registry = b"CARYLL".to_vec();
    }
    if table.is_cid && table.cid_ordering.is_empty() {
        table.cid_ordering = b"OTFCCAUTOCID".to_vec();
    }
    table
}
pub fn parse_cff(root: &ParsedValue, options: &Options) -> Option<Box<CffTable>> {
    let dump = root.get_typed(b"CFF_", JsonType::Object)?;
    let stage = crate::logger::stage("CFF");
    let cff = fd_from_json(Some(dump), options, true);
    stage.finish();
    Some(cff)
}
