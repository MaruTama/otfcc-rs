#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::built_json::{json_new_position, json_object_push_tag, preserialize};
use crate::support::parsed_json::{ParsedValue, json_numof};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, logger_finish, logger_log_sds, logger_start_sds};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, FontFilePointer, Pos};

use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::vf::axis::{VfAxes, VfAxis};
use crate::vf::region::{VqAxisSpan, VqRegion};
use crate::vf::vq::{VQ, VqSegment};
use crate::vf::vv::VV;
use crate::support::primitives::{otfcc_from_fixed};
use crate::support::built_json::{BuiltValue, json_array_new, json_array_push, json_boolean_new, json_double_new, json_integer_new, json_object_new, json_object_push, json_object_push_bytes_key, json_object_push_length, json_string_new, json_string_new_from_bytes};
use crate::vf::region::{vq_axis_span_is_one, vq_delete_region};
use crate::vf::vq::{vq_create_still, vq_get_still};
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
// A `VqRegion` is a fixed header (`dimensions: ShapeId`) followed by a C
// "flexible array member" trailing `spans: [VqAxisSpan; 0]`, allocated as
// one contiguous block (see `vf/region.rs`) -- not yet `Vec`-ified, out of
// scope here. Wraps a `*const VqRegion` so it can be used as an `IndexMap`
// key, comparing/hashing exactly the bytes the original uthash table's
// `memcmp`-based key did: the whole allocation, header plus trailing
// spans, by content, not by pointer identity.
#[derive(Clone, Copy)]
pub struct RegionKey(*const VqRegion);
impl RegionKey {
    unsafe fn as_bytes(&self) -> &[u8] {
        let len = ::core::mem::size_of::<VqRegion>()
            + ::core::mem::size_of::<VqAxisSpan>() * (*self.0).dimensions as usize;
        ::core::slice::from_raw_parts(self.0 as *const u8, len)
    }
}
impl PartialEq for RegionKey {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.as_bytes() == other.as_bytes() }
    }
}
impl Eq for RegionKey {}
impl ::core::hash::Hash for RegionKey {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        unsafe { self.as_bytes().hash(state) }
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
#[repr(C)]
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
// Note: as of this PR, `Font.fvar` still has no disposal call anywhere in
// `caryll_font.rs`'s `dispose_font` -- that is a pre-existing leak (the
// crate never freed the OTF-read `FvarTable` on font teardown even before
// this conversion), not introduced here. Preserved rather than silently
// fixed, matching this migration's discipline elsewhere (e.g. `BaseTable`'s
// `delete_base_axis` leak, `otf_reader/unconsolidate.rs`'s
// `unconsolidate_chaining` move) -- an opportunistic fix here would need
// its own verification pass and is out of scope for a Box化-only PR.
impl Drop for FvarTable {
    fn drop(&mut self) {
        unsafe {
            for (_, master) in ::core::mem::take(&mut self.masters) {
                dispose_fvar_master(&master);
            }
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct InstanceRecord {
    pub subfamily_name_id: u16,
    pub flags: u16,
    pub coordinates: [F16Dot16; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct FVARHeader {
    pub major_version: u16,
    pub minor_version: u16,
    pub axes_array_offset: u16,
    pub reserved1: u16,
    pub axis_count: u16,
    pub axis_size: u16,
    pub instance_count: u16,
    pub instance_size: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct VariationAxisRecord {
    pub axis_tag: u32,
    pub min_value: F16Dot16,
    pub default_value: F16Dot16,
    pub max_value: F16Dot16,
    pub flags: u16,
    pub axis_name_id: u16,
}
#[inline]
unsafe fn dispose_fvar_master(m: &FvarMaster) {
    vq_delete_region(m.region);
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
pub unsafe fn fvar_register_region(
    mut fvar: *mut FvarTable,
    mut region: *mut VqRegion,
) -> *const VqRegion {
    let key = RegionKey(region as *const VqRegion);
    if let Some(existing) = (*fvar).masters.get(&key) {
        let canonical = existing.region;
        vq_delete_region(region);
        return canonical;
    }
    let name: Vec<u8> = format!("m{}", (*fvar).masters.len() + 1).into_bytes();
    (*fvar).masters.insert(key, FvarMaster { name, region });
    region as *const VqRegion
}
unsafe fn fvar_find_master_by_region(
    mut fvar: *const FvarTable,
    mut region: *const VqRegion,
) -> *const FvarMaster {
    match (*fvar).masters.get(&RegionKey(region)) {
        Some(m) => m as *const FvarMaster,
        None => ::core::ptr::null::<FvarMaster>(),
    }
}
pub unsafe fn otfcc_read_fvar(
    packet: &Packet,
    options: &Options,
) -> Option<Box<FvarTable>> {
    let mut header: *mut FVARHeader = ::core::ptr::null_mut::<FVARHeader>();
    let mut n_axes: u16 = 0;
    let mut instance_size_without_psnid: u16 = 0;
    let mut instance_size_with_psnid: u16 = 0;
    let mut axis_record: *mut VariationAxisRecord = ::core::ptr::null_mut::<VariationAxisRecord>();
    let mut n_instances: u16 = 0;
    let mut has_postscript_name_id: bool = false;
    let mut instance: *mut InstanceRecord = ::core::ptr::null_mut::<InstanceRecord>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_FVAR {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    if !((table.length as usize) < ::core::mem::size_of::<FVARHeader>()) {
                        header = data as *mut FVARHeader;
                        if !(be16((*header).major_version) as ::core::ffi::c_int
                            != 1 as ::core::ffi::c_int)
                        {
                            if !(be16((*header).minor_version) as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int)
                            {
                                if !(be16((*header).axes_array_offset) as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int)
                                {
                                    if !(be16((*header).axis_count) as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int)
                                    {
                                        if !(be16((*header).axis_size) as usize
                                            != ::core::mem::size_of::<VariationAxisRecord>())
                                        {
                                            n_axes = be16((*header).axis_count);
                                            instance_size_without_psnid = 4_usize.wrapping_add(
                                                (n_axes as usize).wrapping_mul(
                                                    ::core::mem::size_of::<F16Dot16>(),
                                                ),
                                            )
                                                as u16;
                                            instance_size_with_psnid = (2 as ::core::ffi::c_int
                                                + instance_size_without_psnid as ::core::ffi::c_int)
                                                as u16;
                                            if !(be16((*header).instance_size) as ::core::ffi::c_int
                                                != instance_size_without_psnid as ::core::ffi::c_int
                                                && be16((*header).instance_size)
                                                    as ::core::ffi::c_int
                                                    != instance_size_with_psnid as ::core::ffi::c_int)
                                            {
                                                if !((table.length as usize)
                                                    < (be16((*header).axes_array_offset) as usize)
                                                        .wrapping_add(
                                                            ::core::mem::size_of::<
                                                                VariationAxisRecord,
                                                            >(
                                                            )
                                                                .wrapping_mul(n_axes as usize),
                                                        )
                                                        .wrapping_add(
                                                            (be16((*header).instance_size)
                                                                as ::core::ffi::c_int
                                                                * be16((*header).instance_count)
                                                                    as ::core::ffi::c_int)
                                                                as usize,
                                                        ))
                                                {
                                                    let mut fvar_box: Box<FvarTable> = Box::new(FvarTable {
                                                        major_version: 0,
                                                        minor_version: 0,
                                                        axes: Vec::new(),
                                                        instances: Vec::new(),
                                                        masters: indexmap::IndexMap::new(),
                                                    });
                                                    let fvar: *mut FvarTable = fvar_box.as_mut() as *mut FvarTable;
                                                    axis_record =
                                                        data.offset(be16((*header).axes_array_offset)
                                                            as ::core::ffi::c_int
                                                            as isize)
                                                            as *mut VariationAxisRecord;
                                                    let mut j: u16 = 0 as u16;
                                                    while (j as ::core::ffi::c_int)
                                                        < n_axes as ::core::ffi::c_int
                                                    {
                                                        let mut axis: VfAxis = VfAxis {
                                                            tag: be32((*axis_record).axis_tag),
                                                            min_value: otfcc_from_fixed(be32(
                                                                (*axis_record).min_value as u32,
                                                            )
                                                                as F16Dot16)
                                                                as Pos,
                                                            default_value: otfcc_from_fixed(be32(
                                                                (*axis_record).default_value
                                                                    as u32,
                                                            )
                                                                as F16Dot16)
                                                                as Pos,
                                                            max_value: otfcc_from_fixed(be32(
                                                                (*axis_record).max_value as u32,
                                                            )
                                                                as F16Dot16)
                                                                as Pos,
                                                            flags: be16((*axis_record).flags),
                                                            axis_name_id: be16(
                                                                (*axis_record).axis_name_id,
                                                            ),
                                                        };
                                                        (*fvar).axes.push(axis);
                                                        axis_record = axis_record.offset(1);
                                                        j = j.wrapping_add(1);
                                                    }
                                                    n_instances = be16((*header).instance_count);
                                                    has_postscript_name_id =
                                                        be16((*header).instance_size)
                                                            as ::core::ffi::c_int
                                                            == instance_size_with_psnid
                                                                as ::core::ffi::c_int;
                                                    instance = axis_record as *mut InstanceRecord;
                                                    let mut j_0: u16 = 0 as u16;
                                                    while (j_0 as ::core::ffi::c_int)
                                                        < n_instances as ::core::ffi::c_int
                                                    {
                                                        let mut inst: FvarInstance =
                                                            FvarInstance {
                                                                subfamily_name_id: 0,
                                                                flags: 0,
                                                                coordinates: Vec::new(),
                                                                post_script_name_id: 0,
                                                            };
                                                        // `FVAR_I_INSTANCE.init` deleted: it only
                                                        // (re-)zeroed fields the literal above
                                                        // already set, field for field -- fully
                                                        // redundant, checked before removing (the
                                                        // `CpalPalette`/`init_palette` lesson).
                                                        inst.subfamily_name_id =
                                                            be16((*instance).subfamily_name_id);
                                                        inst.flags = be16((*instance).flags);
                                                        let mut k: u16 = 0 as u16;
                                                        while (k as ::core::ffi::c_int)
                                                            < n_axes as ::core::ffi::c_int
                                                        {
                                                            inst.coordinates.push(
                                                                otfcc_from_fixed(be32(
                                                                    *(&raw mut (*instance)
                                                                        .coordinates
                                                                        as *mut F16Dot16)
                                                                        .offset(k as isize)
                                                                        as u32,
                                                                )
                                                                    as F16Dot16)
                                                                    as Pos,
                                                            );
                                                            k = k.wrapping_add(1);
                                                        }
                                                        inst.coordinates.shrink_to_fit();
                                                        if has_postscript_name_id {
                                                            inst.post_script_name_id = be16(
                                                                *((instance as FontFilePointer)
                                                                    .offset(
                                                                        instance_size_without_psnid
                                                                            as ::core::ffi::c_int
                                                                            as isize,
                                                                    )
                                                                    as *mut u16),
                                                            );
                                                        }
                                                        (*fvar).instances.push(inst);
                                                        instance = (instance as FontFilePointer)
                                                            .offset(be16((*header).instance_size)
                                                                as ::core::ffi::c_int
                                                                as isize)
                                                            as *mut InstanceRecord;
                                                        j_0 = j_0.wrapping_add(1);
                                                    }
                                                    (*fvar).axes.shrink_to_fit();
                                                    (*fvar).instances.shrink_to_fit();
                                                    return Some(fvar_box);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(b"table 'fvar' corrupted.\n"),
                    );
                    // No `fvar` to free here: every path that constructs one
                    // (deep inside the nested guards above) returns
                    // immediately afterward, so this branch is only ever
                    // reached before any allocation happens.
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
}
pub unsafe fn otfcc_dump_fvar(
    table: Option<&FvarTable>,
    mut root: *mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t as *const FvarTable,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"fvar"),
    );
    let axes: &Vec<VfAxis> = &(*table).axes;
    let instances: &Vec<FvarInstance> = &(*table).instances;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut t: *mut BuiltValue = json_object_new(2 as usize);
        let mut _axes: *mut BuiltValue = json_object_new(axes.len());
        let mut __caryll_index: usize = 0 as usize;
        let mut keep: usize = 1 as usize;
        while keep != 0 && __caryll_index < axes.len() {
            let axis: &VfAxis = &axes[__caryll_index];
            while keep != 0 {
                let mut _axis: *mut BuiltValue = json_object_new(5 as usize);
                json_object_push(
                    _axis,
                    b"minValue\0" as *const u8 as *const ::core::ffi::c_char,
                    json_double_new((*axis).min_value as ::core::ffi::c_double),
                );
                json_object_push(
                    _axis,
                    b"defaultValue\0" as *const u8 as *const ::core::ffi::c_char,
                    json_double_new((*axis).default_value as ::core::ffi::c_double),
                );
                json_object_push(
                    _axis,
                    b"maxValue\0" as *const u8 as *const ::core::ffi::c_char,
                    json_double_new((*axis).max_value as ::core::ffi::c_double),
                );
                json_object_push(
                    _axis,
                    b"flags\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*axis).flags as i64),
                );
                json_object_push(
                    _axis,
                    b"axisNameID\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*axis).axis_name_id as i64),
                );
                json_object_push_tag(_axes, (*axis).tag, _axis);
                keep = (keep == 0) as ::core::ffi::c_int as usize;
            }
            keep = (keep == 0) as ::core::ffi::c_int as usize;
            __caryll_index = __caryll_index.wrapping_add(1);
        }
        json_object_push(
            t,
            b"axes\0" as *const u8 as *const ::core::ffi::c_char,
            _axes,
        );
        let mut _instances: *mut BuiltValue = json_array_new(instances.len());
        let mut __caryll_index_0: usize = 0 as usize;
        let mut keep_0: usize = 1 as usize;
        while keep_0 != 0 && __caryll_index_0 < instances.len() {
            let instance: &FvarInstance = &instances[__caryll_index_0];
            while keep_0 != 0 {
                let mut _instance: *mut BuiltValue = json_object_new(4 as usize);
                json_object_push(
                    _instance,
                    b"subfamilyNameID\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*instance).subfamily_name_id as i64),
                );
                if (*instance).post_script_name_id != 0 {
                    json_object_push(
                        _instance,
                        b"postScriptNameID\0" as *const u8 as *const ::core::ffi::c_char,
                        json_integer_new((*instance).post_script_name_id as i64),
                    );
                }
                json_object_push(
                    _instance,
                    b"flags\0" as *const u8 as *const ::core::ffi::c_char,
                    json_integer_new((*instance).flags as i64),
                );
                json_object_push(
                    _instance,
                    b"coordinates\0" as *const u8 as *const ::core::ffi::c_char,
                    json_new_v_vp(&raw const instance.coordinates, table),
                );
                json_array_push(_instances, _instance);
                keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
            }
            keep_0 = (keep_0 == 0) as ::core::ffi::c_int as usize;
            __caryll_index_0 = __caryll_index_0.wrapping_add(1);
        }
        json_object_push(
            t,
            b"instances\0" as *const u8 as *const ::core::ffi::c_char,
            _instances,
        );
        let mut _masters: *mut BuiltValue = json_object_new((*table).masters.len());
        for master in (*table).masters.values() {
            json_object_push_bytes_key(
                _masters,
                &master.name,
                preserialize(json_new_vq_region_explicit(master.region, table)),
            );
        }
        json_object_push(
            t,
            b"masters\0" as *const u8 as *const ::core::ffi::c_char,
            _masters,
        );
        json_object_push(
            root,
            b"fvar\0" as *const u8 as *const ::core::ffi::c_char,
            t,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn json_new_vq_segment(
    mut s: *const VqSegment,
    mut fvar: *const FvarTable,
) -> *mut BuiltValue {
    match *s {
        VqSegment::Still(still) => return json_new_position(still),
        VqSegment::Delta(delta) => {
            let d: *mut BuiltValue = json_object_new(3 as usize);
            json_object_push(
                d,
                b"delta\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_position(delta.quantity),
            );
            if !delta.touched {
                json_object_push(
                    d,
                    b"implicit\0" as *const u8 as *const ::core::ffi::c_char,
                    json_boolean_new(!delta.touched as ::core::ffi::c_int),
                );
            }
            json_object_push(
                d,
                b"on\0" as *const u8 as *const ::core::ffi::c_char,
                json_new_vq_region(delta.region, fvar),
            );
            return d;
        }
    };
}
pub unsafe fn json_new_vq(mut z: VQ, mut fvar: *const FvarTable) -> *mut BuiltValue {
    if z.shift.is_empty() {
        return preserialize(json_new_position(vq_get_still(
            z
        )));
    } else {
        let mut a: *mut BuiltValue = json_array_new(z.shift.len().wrapping_add(1 as usize));
        json_array_push(a, json_new_position(z.kernel));
        let mut j: usize = 0 as usize;
        while j < z.shift.len() {
            json_array_push(
                a,
                json_new_vq_segment(&raw mut z.shift[j] as *mut VqSegment, fvar),
            );
            j = j.wrapping_add(1);
        }
        return preserialize(a);
    };
}
// `json_new_vv` (the by-value sibling of `json_new_v_vp` below) is never
// called anywhere in the crate -- confirmed dead the same way as every
// prior target's dead vtable-adjacent duplicate -- and deleted outright
// rather than ported (it would need `x: VV` to become `x: Vec<Pos>`, moving
// or cloning the caller's coordinates for no live caller).
pub unsafe fn json_new_v_vp(
    x: *const VV,
    fvar: *const FvarTable,
) -> *mut BuiltValue {
    let axes: &Vec<VfAxis> = &(*fvar).axes;
    let coords: &Vec<Pos> = &*x;
    if axes.len() == coords.len() {
        let mut _coord: *mut BuiltValue = json_object_new(axes.len());
        let mut m: usize = 0 as usize;
        while m < coords.len() {
            let axis: &VfAxis = &axes[m];
            let mut tag: [::core::ffi::c_char; 4] = [
                (((*axis).tag & 0xff000000 as u32) >> 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                (((*axis).tag & 0xff0000 as u32) >> 16 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                (((*axis).tag & 0xff00 as u32) >> 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_char,
                ((*axis).tag & 0xff as u32) as ::core::ffi::c_char,
            ];
            json_object_push_length(
                _coord,
                4 as ::core::ffi::c_uint,
                &raw mut tag as *mut ::core::ffi::c_char,
                json_new_position(coords[m]),
            );
            m = m.wrapping_add(1);
        }
        return preserialize(_coord);
    } else {
        let mut _coord_0: *mut BuiltValue = json_array_new(coords.len());
        let mut m_0: usize = 0 as usize;
        while m_0 < coords.len() {
            json_array_push(_coord_0, json_new_position(coords[m_0]));
            m_0 = m_0.wrapping_add(1);
        }
        return preserialize(_coord_0);
    };
}
pub unsafe fn json_vq_of(mut cv: *const ParsedValue, mut _fvar: *const FvarTable) -> VQ {
    return vq_create_still(json_numof(cv) as Pos);
}
pub unsafe fn json_new_vq_axis_span(mut s: *const VqAxisSpan) -> *mut BuiltValue {
    if vq_axis_span_is_one(s) {
        return json_string_new(b"*\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        let mut a: *mut BuiltValue = json_object_new(3 as usize);
        json_object_push(
            a,
            b"start\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*s).start),
        );
        json_object_push(
            a,
            b"peak\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*s).peak),
        );
        json_object_push(
            a,
            b"end\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position((*s).end),
        );
        return a;
    };
}
pub unsafe fn json_new_vq_region_explicit(
    mut rs: *const VqRegion,
    fvar: *const FvarTable,
) -> *mut BuiltValue {
    let axes: &Vec<VfAxis> = &(*fvar).axes;
    if axes.len() == (*rs).dimensions as usize {
        let mut r: *mut BuiltValue = json_object_new((*rs).dimensions as usize);
        let mut j: usize = 0 as usize;
        while j < (*rs).dimensions as usize {
            json_object_push_tag(
                r,
                axes[j].tag,
                json_new_vq_axis_span(
                    (&raw const (*rs).spans as *const VqAxisSpan).offset(j as isize)
                        as *const VqAxisSpan,
                ),
            );
            j = j.wrapping_add(1);
        }
        return r;
    } else {
        let mut r_0: *mut BuiltValue = json_array_new((*rs).dimensions as usize);
        let mut j_0: usize = 0 as usize;
        while j_0 < (*rs).dimensions as usize {
            json_array_push(
                r_0,
                json_new_vq_axis_span(
                    (&raw const (*rs).spans as *const VqAxisSpan).offset(j_0 as isize)
                        as *const VqAxisSpan,
                ),
            );
            j_0 = j_0.wrapping_add(1);
        }
        return r_0;
    };
}
pub unsafe fn json_new_vq_region(
    mut rs: *const VqRegion,
    mut fvar: *const FvarTable,
) -> *mut BuiltValue {
    let mut m: *const FvarMaster = fvar_find_master_by_region(fvar, rs);
    if !m.is_null() && !(*m).name.is_empty() {
        return json_string_new_from_bytes(&(*m).name);
    } else {
        return json_new_vq_region_explicit(rs, fvar);
    };
}
#[inline]
unsafe fn be16(mut x: u16) -> u16 {
    return ((x as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | (x as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int)
        as u16;
}
#[inline]
unsafe fn be32(mut x: u32) -> u32 {
    return (x & 0xff as u32) << 24 as ::core::ffi::c_int
        | (x & 0xff00 as u32) << 8 as ::core::ffi::c_int
        | (x & 0xff0000 as u32) >> 8 as ::core::ffi::c_int
        | (x & 0xff000000 as u32) >> 24 as ::core::ffi::c_int;
}
