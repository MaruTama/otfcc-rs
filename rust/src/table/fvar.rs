use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::font_reader::FontReader;
use crate::support::options::Options;
use crate::support::parsed_json::ParsedValue;
use crate::support::primitives::Pos;

use crate::font::caryll_sfnt::Packet;
use crate::support::built_json::BuiltValue;
use crate::support::primitives::otfcc_from_fixed;
use crate::vf::axis::{VfAxes, VfAxis};
use crate::vf::region::{VqAxisSpan, VqRegion};
use crate::vf::region::{vq_axis_span_is_one, vq_delete_region};
use crate::vf::vq::{VQ, VqSegment};
use crate::vf::vq::{vq_create_still, vq_get_still};
use crate::vf::vv::VV;
pub struct FvarInstance {
    pub subfamily_name_id: u16,
    pub flags: u16,
    pub coordinates: VV,
    pub post_script_name_id: u16,
}
// C由来の時点で素のベクタ形。要素は `coordinates: VV`(`Vec<Pos>`)を所有するが
// `Pos` はプリミティブなので `Vec<FvarInstance>` の `Drop` だけで再帰的に
// 解放できる——`SvgAssignment`/`NameRecord` のような raw ポインタ所有型と違い、
// 専用の要素dispose関数が不要（詳細は下の `dispose_fvar`）。テーブル全体の
// `.copy`（`FVAR_I_INSTANCE_LIST.copy`）は一度も呼ばれておらず削除。
pub type FvarInstanceList = Vec<FvarInstance>;
pub struct FvarMaster {
    pub name: Vec<u8>,
    pub region: *mut VqRegion,
}
// A `VqRegion` used to be a fixed header (`dimensions: ShapeId`) followed
// by a C "flexible array member" trailing `spans: [VqAxisSpan; 0]`,
// allocated as one contiguous block, so the original uthash table's
// `memcmp`-based key could walk it as a single byte range. `dimensions`/
// `spans` are no longer contiguous (`vf/region.rs`'s `Vec`-ification), and
// `VqAxisSpan`'s fields (`Pos` = `f64`) don't implement `Eq`/`Hash` on
// their own -- so `RegionKey` owns a byte-pattern copy of both, cloned
// once at construction (`f64::to_ne_bytes()` per field, `VqAxisSpan` being
// three `f64`s with no interior padding reproduces the same comparison the
// original's single memcmp'd range did, minus the few bytes of zeroed
// alignment padding that range also swept in between `dimensions` and
// `spans` -- not something this conversion set out to fix, just a side
// effect of comparing the two pieces separately). This never needed to be
// a `*const VqRegion` wrapper at all: the raw pointer it used to hold was
// about working around the `Eq`/`Hash` gap, not about sharing ownership
// with `masters`' canonical region.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RegionKey {
    dimensions: crate::support::primitives::ShapeId,
    spans: Vec<[u8; 24]>,
}
impl RegionKey {
    fn from_region(region: &VqRegion) -> Self {
        RegionKey {
            dimensions: region.dimensions,
            spans: region
                .spans
                .iter()
                .map(|s| {
                    let mut bytes = [0u8; 24];
                    bytes[0..8].copy_from_slice(&s.start.to_ne_bytes());
                    bytes[8..16].copy_from_slice(&s.peak.to_ne_bytes());
                    bytes[16..24].copy_from_slice(&s.end.to_ne_bytes());
                    bytes
                })
                .collect(),
        }
    }
}
// `axes: VfAxes`(`Vec<VfAxis>`)/`instances: FvarInstanceList`(`Vec<FvarInstance>`)
// を値で持つため `Copy` は落とす。`FvarTable` は crate 全体で常に `*mut`/
// `*const` 経由でしか触られておらず（`Font.fvar: *mut FvarTable`）、値渡し・
// 値コピーの箇所は無いため `Clone` すら不要（テーブル全体の `.copy` は
// 呼ばれておらず削除済み）。
//
// `masters`（uthash `FvarMaster` テーブル）は`IndexMap<RegionKey, FvarMaster>`
// に変換 —— `BTreeMap`ではない。挿入順に「m1」「m2」…と命名され
// （`fvar_register_region`が挿入直前の`.len()`から採番）、`otfcc_dump_fvar`が
// その順序のまま`masters`オブジェクトを書き出す。この uthash テーブルには
// `HASH_SORT`呼び出しが1つも無い（`grep`で確認済み）ので、出力は挿入順で
// タグ順ではない —— `ScriptStatHash`（`table/otl/build.rs`）と同じ理由で
// `BTreeMap`は不適格。`ScriptStatHash`との違いは規模: あちらはフォント1つ
// あたりのスクリプト数（一桁が普通）に収まるので線形走査で十分だったが、
// こちらは`gvar`の全グリフの全タプルバリエーションから登録されうるため
// 件数が実質的に無制限 —— 線形走査は実アルゴリズム的退行になり得るので
// `IndexMap`（挿入順を保ちつつO(1)平均ルックアップ）を使う。このcrateで
// 初めて`indexmap`に依存する箇所。
pub struct FvarTable {
    pub major_version: u16,
    pub minor_version: u16,
    pub axes: VfAxes,
    pub instances: FvarInstanceList,
    pub masters: indexmap::IndexMap<RegionKey, FvarMaster>,
}
// Stage 6-4 "Box化": `masters`' values own a raw pointer (`region: *mut
// VqRegion`) -- this `Drop` impl is the same "walk `masters`, dispose each
// master" shape `dispose_fvar` already had. `Copy`/`Clone` were already
// absent (no derive to drop).
//
// `Font.fvar` (and `Font.vdmx`) used to leak on font teardown: the old
// `dispose_font` explicitly null'd 31 of `Font`'s 33 table fields before the
// struct's memory was `free()`'d raw, and `fvar`/`vdmx` were the two it
// missed -- a raw `free()` runs no field `Drop` glue, so whatever `fvar`
// pointed to was never reclaimed. Fixed as a side effect of Stage 7-2-d's
// `Font` Box化: `otfcc_font_free` is now `drop(Box::from_raw(x))`, which
// drops every field (including this one) through its own `Drop` impl
// regardless of whether `dispose_font`'s hand-written list covered it.
impl Drop for FvarTable {
    fn drop(&mut self) {
        for (_, master) in ::core::mem::take(&mut self.masters) {
            dispose_fvar_master(&master);
        }
    }
}
#[inline]
fn dispose_fvar_master(m: &FvarMaster) {
    unsafe { vq_delete_region(m.region) };
}
// Deduplicates by `region`'s content (`RegionKey`), not identity: a
// `region` that content-matches an already-registered master is freed
// here and the existing master's own `region` is returned instead, so
// every caller ends up sharing one canonical `VqRegion` per distinct
// content -- callers (`glyf/read.rs`'s gvar tuple-variation parsing) rely
// on this to avoid allocating a fresh region per tuple when many tuples
// share the same region. First registration wins the name "m1", "m2", ...
// in registration order (`(*fvar).masters.len() + 1` at insert time,
// exactly reproducing the original's `HASH_COUNT`-at-insert-time scheme).
pub(crate) fn fvar_register_region(fvar: *mut FvarTable, region: *mut VqRegion) -> *const VqRegion {
    let fvar = unsafe { &mut *fvar };
    let key = RegionKey::from_region(unsafe { &*region });
    if let Some(existing) = fvar.masters.get(&key) {
        let canonical = existing.region;
        unsafe { vq_delete_region(region) };
        return canonical;
    }
    let name: Vec<u8> = format!("m{}", fvar.masters.len() + 1).into_bytes();
    fvar.masters.insert(key, FvarMaster { name, region });
    region as *const VqRegion
}
fn fvar_find_master_by_region(fvar: &FvarTable, region: *const VqRegion) -> Option<&FvarMaster> {
    let key = RegionKey::from_region(unsafe { &*region });
    fvar.masters.get(&key)
}
/// `axisSize`/`AXIS_RECORD_SIZE`(20 bytes): `axis_tag`(4) + `min`/`default`/
/// `max_value`(4 each) + `flags`(2) + `axis_name_id`(2).
const AXIS_RECORD_SIZE: usize = 20;
/// The original's overall-length guard computed `instance_size *
/// instance_count` in **32-bit signed** `c_int` arithmetic
/// (`be16(header.instance_size) as c_int * be16(header.instance_count) as
/// c_int`) -- both operands up to 65535, so the true product (up to
/// ~4.29 billion) overflows `i32::MAX` and wraps to a negative value in
/// release builds (checked overflow is off by default outside `cargo
/// test`/debug). That negative `c_int`, cast `as usize`, sign-extends into
/// a huge `usize` near `usize::MAX`; the outer `.wrapping_add` then wraps
/// *again* around `usize`'s own width, landing back on some small,
/// wrong-but-plausible-looking total. A `table.length` this small final
/// value passes against would then have `n_instances` up to 65535 records
/// of `instance_size` bytes each read via raw `.offset()` past the real
/// end of the table -- worse than `table/cpal.rs`'s single-wraparound bug,
/// this one wraps twice (`i32` overflow, then the `usize` sum). Every
/// multiplication and addition below goes through `checked_mul`/
/// `checked_add` (via `Option`'s own overflow-is-`None` propagation, `?`),
/// so an overflow anywhere in the chain rejects the table outright instead
/// of wrapping either width.
fn parse_fvar(data: &[u8]) -> Option<FvarTable> {
    let mut h = FontReader::new(data);
    let major_version = h.u16().ok()?;
    if major_version != 1 {
        return None;
    }
    let minor_version = h.u16().ok()?;
    if minor_version != 0 {
        return None;
    }
    let axes_array_offset = h.u16().ok()? as usize;
    if axes_array_offset == 0 {
        return None;
    }
    h.skip(2).ok()?; // reserved1
    let axis_count = h.u16().ok()?;
    if axis_count == 0 {
        return None;
    }
    let axis_size = h.u16().ok()?;
    if axis_size as usize != AXIS_RECORD_SIZE {
        return None;
    }
    let instance_count = h.u16().ok()?;
    let instance_size = h.u16().ok()?;

    let instance_size_without_psnid =
        4usize.checked_add((axis_count as usize).checked_mul(4)?)?;
    let instance_size_with_psnid = instance_size_without_psnid.checked_add(2)?;
    if instance_size as usize != instance_size_without_psnid
        && instance_size as usize != instance_size_with_psnid
    {
        return None;
    }

    let axes_bytes = AXIS_RECORD_SIZE.checked_mul(axis_count as usize)?;
    let instances_bytes = (instance_size as usize).checked_mul(instance_count as usize)?;
    let total_needed = axes_array_offset.checked_add(axes_bytes)?.checked_add(instances_bytes)?;
    if data.len() < total_needed {
        return None;
    }

    let mut axes: VfAxes = Vec::with_capacity(axis_count as usize);
    let mut r = FontReader::new(data).at(axes_array_offset).ok()?;
    for _ in 0..axis_count {
        let tag = r.u32().ok()?;
        let min_value = r.i32().ok()?;
        let default_value = r.i32().ok()?;
        let max_value = r.i32().ok()?;
        let flags = r.u16().ok()?;
        let axis_name_id = r.u16().ok()?;
        axes.push(VfAxis {
            tag,
            min_value: otfcc_from_fixed(min_value) as Pos,
            default_value: otfcc_from_fixed(default_value) as Pos,
            max_value: otfcc_from_fixed(max_value) as Pos,
            flags,
            axis_name_id,
        });
    }

    // `r` is now positioned right after the axis array, i.e. exactly where
    // the instance array starts -- each iteration below consumes exactly
    // `instance_size` bytes (2 + 2 + 4*axis_count [+ 2]), so it stays in
    // sync with the next record without needing to re-seek.
    let has_postscript_name_id = instance_size as usize == instance_size_with_psnid;
    let mut instances: FvarInstanceList = Vec::with_capacity(instance_count as usize);
    for _ in 0..instance_count {
        let subfamily_name_id = r.u16().ok()?;
        let flags = r.u16().ok()?;
        let mut coordinates: VV = Vec::with_capacity(axis_count as usize);
        for _ in 0..axis_count {
            let v = r.i32().ok()?;
            coordinates.push(otfcc_from_fixed(v) as Pos);
        }
        coordinates.shrink_to_fit();
        let post_script_name_id = if has_postscript_name_id { r.u16().ok()? } else { 0 };
        instances.push(FvarInstance {
            subfamily_name_id,
            flags,
            coordinates,
            post_script_name_id,
        });
    }

    axes.shrink_to_fit();
    instances.shrink_to_fit();
    Some(FvarTable {
        major_version,
        minor_version,
        axes,
        instances,
        masters: indexmap::IndexMap::new(),
    })
}
pub fn otfcc_read_fvar(packet: &Packet, options: &Options) -> Option<Box<FvarTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_FVAR)?;
    match parse_fvar(&table.data) {
        Some(fvar) => Some(Box::new(fvar)),
        None => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'fvar' corrupted.\n"),
            );
            None
        }
    }
}
pub fn otfcc_dump_fvar(table: Option<&FvarTable>, root: &mut BuiltValue, options: &Options) {
    let Some(table) = table else { return };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"fvar"),
    );
    let axes: &Vec<VfAxis> = &table.axes;
    let instances: &Vec<FvarInstance> = &table.instances;
    let mut t = BuiltValue::new_object(2);
    let mut _axes = BuiltValue::new_object(axes.len());
    for axis in axes.iter() {
        let mut _axis = BuiltValue::new_object(5);
        _axis.push_field(b"minValue", BuiltValue::Double(axis.min_value));
        _axis.push_field(b"defaultValue", BuiltValue::Double(axis.default_value));
        _axis.push_field(b"maxValue", BuiltValue::Double(axis.max_value));
        _axis.push_field(b"flags", BuiltValue::Int(axis.flags as i64));
        _axis.push_field(b"axisNameID", BuiltValue::Int(axis.axis_name_id as i64));
        _axes.push_tag(axis.tag, _axis);
    }
    t.push_field(b"axes", _axes);
    let mut _instances = BuiltValue::new_array(instances.len());
    for instance in instances.iter() {
        let mut _instance = BuiltValue::new_object(4);
        _instance.push_field(
            b"subfamilyNameID",
            BuiltValue::Int(instance.subfamily_name_id as i64),
        );
        if instance.post_script_name_id != 0 {
            _instance.push_field(
                b"postScriptNameID",
                BuiltValue::Int(instance.post_script_name_id as i64),
            );
        }
        _instance.push_field(b"flags", BuiltValue::Int(instance.flags as i64));
        _instance.push_field(
            b"coordinates",
            json_new_v_vp(&instance.coordinates, table),
        );
        _instances.push_item(_instance);
    }
    t.push_field(b"instances", _instances);
    let mut _masters = BuiltValue::new_object(table.masters.len());
    for master in table.masters.values() {
        _masters.push_field_bytes_key(
            &master.name,
            json_new_vq_region_explicit(master.region, table).preserialize(),
        );
    }
    t.push_field(b"masters", _masters);
    root.push_field(b"fvar", t);
    logger_finish(&mut *options.logger.borrow_mut());
}
pub fn json_new_vq_segment(s: &VqSegment, fvar: Option<&FvarTable>) -> BuiltValue {
    match *s {
        VqSegment::Still(still) => BuiltValue::position(still),
        VqSegment::Delta(delta) => {
            let mut d = BuiltValue::new_object(3);
            d.push_field(b"delta", BuiltValue::position(delta.quantity));
            if !delta.touched {
                d.push_field(b"implicit", BuiltValue::Bool(!delta.touched));
            }
            // A `Delta` segment only ever exists on a variable font's
            // glyphs, i.e. only when a real `fvar` table (the one that
            // registered `delta.region`) is present -- the same
            // precondition the old raw-pointer code silently assumed
            // (and would have dereferenced null under, UB, had it ever
            // been violated) is now a checked `expect`.
            let fvar = fvar.expect("a VQ delta segment implies a variable font's fvar table");
            d.push_field(b"on", json_new_vq_region(delta.region, fvar));
            d
        }
    }
}
pub fn json_new_vq(z: VQ, fvar: Option<&FvarTable>) -> BuiltValue {
    if z.shift.is_empty() {
        BuiltValue::position(vq_get_still(z)).preserialize()
    } else {
        let mut a = BuiltValue::new_array(z.shift.len() + 1);
        a.push_item(BuiltValue::position(z.kernel));
        for seg in &z.shift {
            a.push_item(json_new_vq_segment(seg, fvar));
        }
        a.preserialize()
    }
}
// `json_new_vv` (the by-value sibling of `json_new_v_vp` below) is never
// called anywhere in the crate -- confirmed dead the same way as every
// prior target's dead vtable-adjacent duplicate -- and deleted outright
// rather than ported (it would need `x: VV` to become `x: Vec<Pos>`, moving
// or cloning the caller's coordinates for no live caller).
pub fn json_new_v_vp(x: &VV, fvar: &FvarTable) -> BuiltValue {
    let axes: &Vec<VfAxis> = &fvar.axes;
    let coords: &Vec<Pos> = x;
    if axes.len() == coords.len() {
        let mut coord = BuiltValue::new_object(axes.len());
        for (m, axis) in axes.iter().enumerate() {
            coord.push_tag(axis.tag, BuiltValue::position(coords[m]));
        }
        coord.preserialize()
    } else {
        let mut coord = BuiltValue::new_array(coords.len());
        for &c in coords.iter() {
            coord.push_item(BuiltValue::position(c));
        }
        coord.preserialize()
    }
}
pub fn json_vq_of(cv: Option<&ParsedValue>) -> VQ {
    let n = cv.and_then(ParsedValue::as_num).unwrap_or(0.0);
    vq_create_still(n as Pos)
}
pub fn json_new_vq_axis_span(s: &VqAxisSpan) -> BuiltValue {
    if vq_axis_span_is_one(s) {
        BuiltValue::Str(b"*".to_vec())
    } else {
        let mut a = BuiltValue::new_object(3);
        a.push_field(b"start", BuiltValue::position(s.start));
        a.push_field(b"peak", BuiltValue::position(s.peak));
        a.push_field(b"end", BuiltValue::position(s.end));
        a
    }
}
fn json_new_vq_region_explicit(rs: *const VqRegion, fvar: &FvarTable) -> BuiltValue {
    let region = unsafe { &*rs };
    let axes: &Vec<VfAxis> = &fvar.axes;
    let dimensions = region.dimensions as usize;
    if axes.len() == dimensions {
        let mut r = BuiltValue::new_object(dimensions);
        for (axis, span) in axes.iter().zip(region.spans.iter()).take(dimensions) {
            r.push_tag(axis.tag, json_new_vq_axis_span(span));
        }
        r
    } else {
        let mut r_0 = BuiltValue::new_array(dimensions);
        for span in region.spans.iter().take(dimensions) {
            r_0.push_item(json_new_vq_axis_span(span));
        }
        r_0
    }
}
pub fn json_new_vq_region(rs: *const VqRegion, fvar: &FvarTable) -> BuiltValue {
    match fvar_find_master_by_region(fvar, rs) {
        Some(m) if !m.name.is_empty() => BuiltValue::str_truncated_at_nul(&m.name),
        _ => json_new_vq_region_explicit(rs, fvar),
    }
}

#[cfg(test)]
mod parse_fvar_tests {
    use super::*;

    // header(16) + one axis record(20) + one instance without PSNID(8)
    fn well_formed_fvar_table() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&1u16.to_be_bytes()); // majorVersion
        b.extend_from_slice(&0u16.to_be_bytes()); // minorVersion
        b.extend_from_slice(&16u16.to_be_bytes()); // axesArrayOffset
        b.extend_from_slice(&0u16.to_be_bytes()); // reserved1
        b.extend_from_slice(&1u16.to_be_bytes()); // axisCount
        b.extend_from_slice(&20u16.to_be_bytes()); // axisSize
        b.extend_from_slice(&1u16.to_be_bytes()); // instanceCount
        b.extend_from_slice(&8u16.to_be_bytes()); // instanceSize (4 + 4*1)
        // VariationAxisRecord @16
        b.extend_from_slice(b"wght"); // axisTag
        b.extend_from_slice(&3_276_800i32.to_be_bytes()); // minValue = 50.0
        b.extend_from_slice(&6_553_600i32.to_be_bytes()); // defaultValue = 100.0
        b.extend_from_slice(&13_107_200i32.to_be_bytes()); // maxValue = 200.0
        b.extend_from_slice(&0u16.to_be_bytes()); // flags
        b.extend_from_slice(&256u16.to_be_bytes()); // axisNameID
        // InstanceRecord @36 (no PostScript name ID)
        b.extend_from_slice(&2u16.to_be_bytes()); // subfamilyNameID
        b.extend_from_slice(&0u16.to_be_bytes()); // flags
        b.extend_from_slice(&6_553_600i32.to_be_bytes()); // coordinates[0] = 100.0
        b
    }

    #[test]
    fn well_formed_table_reads_the_axis_and_instance() {
        let data = well_formed_fvar_table();
        let fvar = parse_fvar(&data).unwrap();
        assert_eq!(fvar.axes.len(), 1);
        assert_eq!(fvar.axes[0].tag, u32::from_be_bytes(*b"wght"));
        assert_eq!(fvar.axes[0].min_value, 50.0);
        assert_eq!(fvar.axes[0].default_value, 100.0);
        assert_eq!(fvar.axes[0].max_value, 200.0);
        assert_eq!(fvar.axes[0].axis_name_id, 256);
        assert_eq!(fvar.instances.len(), 1);
        assert_eq!(fvar.instances[0].subfamily_name_id, 2);
        assert_eq!(fvar.instances[0].coordinates, vec![100.0]);
        assert_eq!(fvar.instances[0].post_script_name_id, 0);
    }

    #[test]
    fn instance_with_postscript_name_id_is_read() {
        let mut b = well_formed_fvar_table();
        b[14..16].copy_from_slice(&10u16.to_be_bytes()); // instanceSize = 8 + 2
        b.extend_from_slice(&300u16.to_be_bytes()); // postScriptNameID
        let fvar = parse_fvar(&b).unwrap();
        assert_eq!(fvar.instances[0].post_script_name_id, 300);
    }

    #[test]
    fn truncated_header_is_rejected() {
        let data = &well_formed_fvar_table()[..10];
        assert!(parse_fvar(data).is_none());
    }

    #[test]
    fn wrong_axis_size_is_rejected() {
        let mut data = well_formed_fvar_table();
        data[10..12].copy_from_slice(&18u16.to_be_bytes()); // axisSize != 20
        assert!(parse_fvar(&data).is_none());
    }

    #[test]
    fn instance_size_matching_neither_shape_is_rejected() {
        let mut data = well_formed_fvar_table();
        data[14..16].copy_from_slice(&9u16.to_be_bytes()); // neither 8 nor 10
        assert!(parse_fvar(&data).is_none());
    }

    #[test]
    fn instance_array_shorter_than_declared_is_rejected_not_read_oob() {
        let mut data = well_formed_fvar_table();
        data[12..14].copy_from_slice(&2u16.to_be_bytes()); // instanceCount = 2, only 1 present
        assert!(parse_fvar(&data).is_none());
    }

    #[test]
    fn instance_size_times_count_overflowing_i32_is_rejected_not_wrapped() {
        // The original computed `instance_size * instance_count` in
        // 32-bit signed `c_int` arithmetic. With `axis_count` = 16382,
        // `instance_size_without_psnid` = 4 + 4*16382 = 65532 -- both it
        // and `instance_count` (65535) are legal `u16` values individually,
        // but their true product (~4.29 billion) overflows `i32::MAX` and
        // wraps negative in a release build (checked overflow is off by
        // default outside `cargo test`/debug), which then sign-extended
        // into a huge `usize` and wrapped a *second* time in the outer
        // `wrapping_add`, potentially landing back on a small value that
        // passes the length guard even though the real table (here, still
        // just the 44-byte fixture) is nowhere near big enough.
        let mut data = well_formed_fvar_table();
        data[8..10].copy_from_slice(&16382u16.to_be_bytes()); // axisCount
        data[12..14].copy_from_slice(&0xFFFFu16.to_be_bytes()); // instanceCount
        data[14..16].copy_from_slice(&65532u16.to_be_bytes()); // instanceSize
        assert!(parse_fvar(&data).is_none());
    }

    #[test]
    fn zero_axes_array_offset_is_rejected() {
        let mut data = well_formed_fvar_table();
        data[4..6].copy_from_slice(&0u16.to_be_bytes());
        assert!(parse_fvar(&data).is_none());
    }
}

#[cfg(test)]
mod fvar_register_region_tests {
    use super::*;

    fn region_with_spans(spans: Vec<VqAxisSpan>) -> *mut VqRegion {
        let dimensions = spans.len() as crate::support::primitives::ShapeId;
        Box::into_raw(Box::new(VqRegion { dimensions, spans }))
    }

    fn empty_fvar_table() -> FvarTable {
        FvarTable {
            major_version: 1,
            minor_version: 0,
            axes: Vec::new(),
            instances: Vec::new(),
            masters: indexmap::IndexMap::new(),
        }
    }

    // Pins `RegionKey`'s content-based dedup, the behavior the whole
    // `RegionKey`-owns-its-data rewrite (replacing a `*const VqRegion`-
    // wrapping key that compared by viewing the pointee's bytes) is meant
    // to preserve exactly: two independently-allocated `VqRegion`s with
    // identical content must register as the same master, with the
    // second's allocation freed rather than kept as a duplicate entry.
    #[test]
    fn two_content_identical_regions_coalesce_into_one_master() {
        let mut fvar = empty_fvar_table();
        let span = VqAxisSpan { start: -1.0, peak: 0.0, end: 1.0 };
        let region_a = region_with_spans(vec![span]);
        let region_b = region_with_spans(vec![span]);
        let canonical_a = fvar_register_region(&mut fvar, region_a);
        let canonical_b = fvar_register_region(&mut fvar, region_b);
        assert_eq!(canonical_a, canonical_b);
        assert_eq!(fvar.masters.len(), 1);
    }

    // The mirror case: distinct content must NOT be coalesced, and each
    // gets its own "m1"/"m2" name in registration order.
    #[test]
    fn content_distinct_regions_register_separately() {
        let mut fvar = empty_fvar_table();
        let region_a = region_with_spans(vec![VqAxisSpan { start: -1.0, peak: 0.0, end: 1.0 }]);
        let region_b = region_with_spans(vec![VqAxisSpan { start: 0.0, peak: 1.0, end: 1.0 }]);
        let canonical_a = fvar_register_region(&mut fvar, region_a);
        let canonical_b = fvar_register_region(&mut fvar, region_b);
        assert_ne!(canonical_a, canonical_b);
        assert_eq!(fvar.masters.len(), 2);
        let names: Vec<&[u8]> = fvar.masters.values().map(|m| m.name.as_slice()).collect();
        assert_eq!(names, vec![b"m1".as_slice(), b"m2".as_slice()]);
    }
}
