#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset};


use crate::support::json_funcs::{json_obj_get_type, preserialize};
use crate::table::otl::coverage::{Coverage, otl_coverage_create, otl_coverage_free, push_to_coverage, read_coverage};
use crate::support::handle::{handle_from_index, handle_from_name, otfcc_handle_dispose, GlyphHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{read_16u};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::{NULL};
use crate::table::otl::{GsubLigatureEntry, Subtable, GsubLigatureSubtable};
use crate::table::otl::subtables::{BuildHeuristics};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::coverage::{OTL_I_COVERAGE};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_object_new, json_object_push, json_string_new_length};
use crate::vendor::sds::{sdslen, sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LigatureAggregator {
    pub gid: ::core::ffi::c_int,
    pub hh: UtHashHandle,
}
unsafe extern "C" fn delete_gsub_ligature_entry(mut entry: *mut GsubLigatureEntry) {
    otfcc_handle_dispose(&raw mut (*entry).to);
    otl_coverage_free((*entry).from);
    (*entry).from = ::core::ptr::null_mut::<Coverage>();
}
pub(crate) unsafe fn dispose_gsub_ligature_subtable(arr: *mut GsubLigatureSubtable) {
    for e in (*arr).iter_mut() {
        delete_gsub_ligature_entry(e);
    }
    *arr = Vec::new();
}
pub(crate) unsafe extern "C" fn subtable_gsub_ligature_free(x: *mut GsubLigatureSubtable) {
    if x.is_null() {
        return;
    }
    dispose_gsub_ligature_subtable(x);
    free(x as *mut ::core::ffi::c_void);
}
/// The one live `.replace` among all the `Subtable`-union-blocked
/// containers: `consolidate_gsub_ligature` builds a fresh, empty `nt`,
/// filters/moves entries into it, then swaps it in here. `src` is always
/// that fresh local -- never reused by the caller afterward -- so disposing
/// the old `*dst` and move-assigning `src` in is equivalent to (and safer
/// than) the original's dispose-then-`memcpy`.
#[allow(improper_ctypes_definitions)]
pub(crate) unsafe extern "C" fn subtable_gsub_ligature_replace(
    dst: *mut GsubLigatureSubtable,
    src: GsubLigatureSubtable,
) {
    dispose_gsub_ligature_subtable(dst);
    *dst = src;
}
unsafe extern "C" fn subtable_gsub_ligature_create() -> *mut GsubLigatureSubtable {
    let x: *mut GsubLigatureSubtable =
        malloc(::core::mem::size_of::<GsubLigatureSubtable>() as usize)
            as *mut GsubLigatureSubtable;
    x.write(Vec::new());
    x
}
pub unsafe extern "C" fn otl_read_gsub_ligature(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    _max_glyphs: GlyphId,
    mut _options: *const Options,
) -> *mut Subtable {
    let mut start_coverage: *mut Coverage = ::core::ptr::null_mut::<Coverage>();
    let mut set_count: GlyphId = 0;
    let mut ligature_count: u32 = 0;
    let mut current_block: u64;
    let subtable: *mut GsubLigatureSubtable = subtable_gsub_ligature_create();
    if !(table_length < offset.wrapping_add(6 as u32)) {
        start_coverage = read_coverage(
            data as *const u8,
            table_length,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !start_coverage.is_null() {
            set_count = read_16u(
                data.offset(offset as isize)
                    .offset(4 as ::core::ffi::c_int as isize) as *const u8,
            ) as GlyphId;
            if !(set_count as usize != (*start_coverage).len())
            {
                if !(table_length
                    < offset.wrapping_add(6 as u32).wrapping_add(
                        (set_count as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as u32,
                    ))
                {
                    ligature_count = 0 as u32;
                    let mut j: GlyphId = 0 as GlyphId;
                    loop {
                        if !((j as ::core::ffi::c_int) < set_count as ::core::ffi::c_int) {
                            current_block = 17860125682698302841;
                            break;
                        }
                        let mut set_offset: u32 = offset.wrapping_add(read_16u(
                            data.offset(offset as isize)
                                .offset(6 as ::core::ffi::c_int as isize)
                                .offset(
                                    (j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                                ) as *const u8,
                        )
                            as u32);
                        if table_length < set_offset.wrapping_add(2 as u32) {
                            current_block = 3443835632518673764;
                            break;
                        }
                        ligature_count = ligature_count.wrapping_add(read_16u(
                            data.offset(set_offset as isize) as *const u8,
                        )
                            as u32);
                        if table_length
                            < set_offset.wrapping_add(2 as u32).wrapping_add(
                                (read_16u(data.offset(set_offset as isize) as *const u8)
                                    as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int)
                                    as u32,
                            )
                        {
                            current_block = 3443835632518673764;
                            break;
                        }
                        j = j.wrapping_add(1);
                    }
                    match current_block {
                        3443835632518673764 => {}
                        _ => {
                            let mut j_0: GlyphId = 0 as GlyphId;
                            's_77: loop {
                                if !((j_0 as ::core::ffi::c_int) < set_count as ::core::ffi::c_int) {
                                    current_block = 11932355480408055363;
                                    break;
                                }
                                let mut set_offset_0: u32 = offset.wrapping_add(read_16u(
                                    data.offset(offset as isize)
                                        .offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j_0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                let mut lc: GlyphId =
                                    read_16u(data.offset(set_offset_0 as isize) as *const u8)
                                        as GlyphId;
                                let mut k: GlyphId = 0 as GlyphId;
                                while (k as ::core::ffi::c_int) < lc as ::core::ffi::c_int {
                                    let mut lig_offset: u32 = set_offset_0.wrapping_add(
                                        read_16u(
                                            data.offset(set_offset_0 as isize)
                                                .offset(2 as ::core::ffi::c_int as isize)
                                                .offset(
                                                    (k as ::core::ffi::c_int
                                                        * 2 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                as *const u8,
                                        ) as u32,
                                    );
                                    if table_length < lig_offset.wrapping_add(4 as u32) {
                                        current_block = 3443835632518673764;
                                        break 's_77;
                                    }
                                    let mut lig_components: GlyphId = read_16u(
                                        data.offset(lig_offset as isize)
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            as *const u8,
                                    )
                                        as GlyphId;
                                    if table_length
                                        < lig_offset.wrapping_add(2 as u32).wrapping_add(
                                            (lig_components as ::core::ffi::c_int
                                                * 2 as ::core::ffi::c_int)
                                                as u32,
                                        )
                                    {
                                        current_block = 3443835632518673764;
                                        break 's_77;
                                    }
                                    let mut cov: *mut Coverage =
                                        otl_coverage_create();
                                    push_to_coverage(
                                        cov,
                                        handle_from_index(
                                            (&(*start_coverage))[j_0 as usize].index,
                                        )
                                            as GlyphHandle,
                                    );
                                    let mut m: GlyphId = 1 as GlyphId;
                                    while (m as ::core::ffi::c_int)
                                        < lig_components as ::core::ffi::c_int
                                    {
                                        push_to_coverage(
                                            cov,
                                            handle_from_index(
                                                read_16u(
                                                    data.offset(lig_offset as isize)
                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (m as ::core::ffi::c_int
                                                                * 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        as *const u8,
                                                )
                                                    as GlyphId,
                                            )
                                                as GlyphHandle,
                                        );
                                        m = m.wrapping_add(1);
                                    }
                                    (*subtable).push(GsubLigatureEntry {
                                        from: cov,
                                        to: handle_from_index(
                                            read_16u(data.offset(lig_offset as isize)
                                                as *const u8)
                                                as GlyphId,
                                        )
                                            as GlyphHandle,
                                    });
                                    k = k.wrapping_add(1);
                                }
                                j_0 = j_0.wrapping_add(1);
                            }
                            match current_block {
                                3443835632518673764 => {}
                                _ => {
                                    otl_coverage_free(
                                        start_coverage,
                                    );
                                    return subtable as *mut Subtable;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    subtable_gsub_ligature_free(subtable);
    return ::core::ptr::null_mut::<Subtable>();
}
pub unsafe extern "C" fn otl_gsub_dump_ligature(
    mut _subtable: *const Subtable,
) -> *mut JsonValue {
    let subtable: *const GsubLigatureSubtable = &raw const (*_subtable).gsub_ligature as *const GsubLigatureSubtable;
    let mut st: *mut JsonValue = json_array_new((*subtable).len());
    let mut j: GlyphId = 0 as GlyphId;
    while (j as usize) < (*subtable).len() {
        let mut entry: *mut JsonValue = json_object_new(2 as usize);
        json_object_push(
            entry,
            b"from\0" as *const u8 as *const ::core::ffi::c_char,
            OTL_I_COVERAGE.dump.expect("non-null function pointer")(
                (&(*subtable))[j as usize].from,
            ),
        );
        json_object_push(
            entry,
            b"to\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new_length(
                sdslen((&(*subtable))[j as usize].to.name) as ::core::ffi::c_uint,
                (&(*subtable))[j as usize].to.name as *const ::core::ffi::c_char,
            ),
        );
        json_array_push(st, preserialize(entry));
        j = j.wrapping_add(1);
    }
    let mut ret: *mut JsonValue = json_object_new(1 as usize);
    json_object_push(
        ret,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        st,
    );
    return ret;
}
pub unsafe extern "C" fn otl_gsub_parse_ligature(
    mut _subtable: *const JsonValue,
    mut _options: *const Options,
) -> *mut Subtable {
    if !json_obj_get_type(
        _subtable,
        b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    )
    .is_null()
    {
        _subtable = json_obj_get_type(
            _subtable,
            b"substitutions\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        );
        let st: *mut GsubLigatureSubtable = subtable_gsub_ligature_create();
        let mut n: GlyphId = (*_subtable).u.array.length as GlyphId;
        let mut k: GlyphId = 0 as GlyphId;
        while (k as ::core::ffi::c_int) < n as ::core::ffi::c_int {
            let mut entry: *mut JsonValue =
                *(*_subtable).u.array.values.offset(k as isize) as *mut JsonValue;
            let mut _from: *mut JsonValue = json_obj_get_type(
                entry,
                b"from\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            );
            let mut _to: *mut JsonValue = json_obj_get_type(
                entry,
                b"to\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !(_from.is_null() || _to.is_null()) {
                (*st).push(GsubLigatureEntry {
                    from: OTL_I_COVERAGE.parse.expect("non-null function pointer")(_from),
                    to: handle_from_name(sdsnewlen(
                        (*_to).u.string.ptr as *const ::core::ffi::c_void,
                        (*_to).u.string.length as usize,
                    )) as GlyphHandle,
                });
            }
            k = k.wrapping_add(1);
        }
        return st as *mut Subtable;
    } else {
        let st_0: *mut GsubLigatureSubtable = subtable_gsub_ligature_create();
        let mut n_0: GlyphId = (*_subtable).u.array.length as GlyphId;
        let mut k_0: GlyphId = 0 as GlyphId;
        while (k_0 as ::core::ffi::c_int) < n_0 as ::core::ffi::c_int {
            let mut _from_0: *mut JsonValue =
                (*(*_subtable).u.object.values.offset(k_0 as isize)).value as *mut JsonValue;
            if !(_from_0.is_null()
                || (*_from_0).type_0 != JsonType::Array)
            {
                (*st_0).push(GsubLigatureEntry {
                    from: OTL_I_COVERAGE.parse.expect("non-null function pointer")(_from_0),
                    to: handle_from_name(sdsnewlen(
                        (*(*_subtable).u.object.values.offset(k_0 as isize)).name
                            as *const ::core::ffi::c_void,
                        (*(*_subtable).u.object.values.offset(k_0 as isize)).name_length
                            as usize,
                    )) as GlyphHandle,
                });
            }
            k_0 = k_0.wrapping_add(1);
        }
        return st_0 as *mut Subtable;
    };
}
unsafe extern "C" fn by_gid(
    mut a: *mut LigatureAggregator,
    mut b: *mut LigatureAggregator,
) -> ::core::ffi::c_int {
    return (*a).gid - (*b).gid;
}
pub unsafe extern "C" fn otfcc_build_gsub_ligature_subtable(
    mut _subtable: *const Subtable,
    mut _heuristics: BuildHeuristics,
) -> *mut Buffer {
    let subtable: *const GsubLigatureSubtable = &raw const (*_subtable).gsub_ligature as *const GsubLigatureSubtable;
    let mut h: *mut LigatureAggregator = ::core::ptr::null_mut::<LigatureAggregator>();
    let mut s: *mut LigatureAggregator = ::core::ptr::null_mut::<LigatureAggregator>();
    let mut tmp: *mut LigatureAggregator = ::core::ptr::null_mut::<LigatureAggregator>();
    let mut n_ligatures: GlyphId = (*subtable).len() as GlyphId;
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_int) < n_ligatures as ::core::ffi::c_int {
        let mut sgid: ::core::ffi::c_int = (&(*(&(*subtable))[j as usize].from))[0]
            .index as ::core::ffi::c_int;
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key: *const ::core::ffi::c_uchar = &raw mut sgid as *const ::core::ffi::c_uchar;
        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
        while _hj_k >= 12 as ::core::ffi::c_uint {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _hf_hashv = _hf_hashv
            .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
        let mut current_block_50: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 483194043190260627;
            }
            10 => {
                current_block_50 = 483194043190260627;
            }
            9 => {
                current_block_50 = 9392992341002218192;
            }
            8 => {
                current_block_50 = 14840068175916424037;
            }
            7 => {
                current_block_50 = 2003362535987825465;
            }
            6 => {
                current_block_50 = 7629293359809983242;
            }
            5 => {
                current_block_50 = 11376947495104746217;
            }
            4 => {
                current_block_50 = 16637993436199044631;
            }
            3 => {
                current_block_50 = 6546859112865444725;
            }
            2 => {
                current_block_50 = 10505030521387687196;
            }
            1 => {
                current_block_50 = 5259327757700886538;
            }
            _ => {
                current_block_50 = 1356832168064818221;
            }
        }
        match current_block_50 {
            483194043190260627 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 9392992341002218192;
            }
            _ => {}
        }
        match current_block_50 {
            9392992341002218192 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 14840068175916424037;
            }
            _ => {}
        }
        match current_block_50 {
            14840068175916424037 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 2003362535987825465;
            }
            _ => {}
        }
        match current_block_50 {
            2003362535987825465 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 7629293359809983242;
            }
            _ => {}
        }
        match current_block_50 {
            7629293359809983242 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 11376947495104746217;
            }
            _ => {}
        }
        match current_block_50 {
            11376947495104746217 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_50 = 16637993436199044631;
            }
            _ => {}
        }
        match current_block_50 {
            16637993436199044631 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 6546859112865444725;
            }
            _ => {}
        }
        match current_block_50 {
            6546859112865444725 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 10505030521387687196;
            }
            _ => {}
        }
        match current_block_50 {
            10505030521387687196 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 5259327757700886538;
            }
            _ => {}
        }
        match current_block_50 {
            5259327757700886538 => {
                _hj_i =
                    _hj_i
                        .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        s = ::core::ptr::null_mut::<LigatureAggregator>();
        if !h.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if !(*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize))
                    .hh_head
                    .is_null()
                {
                    s = ((*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                        as *mut ::core::ffi::c_char)
                        .offset(-(*(*h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void
                        as *mut LigatureAggregator
                        as *mut LigatureAggregator;
                } else {
                    s = ::core::ptr::null_mut::<LigatureAggregator>();
                }
                while !s.is_null() {
                    if (*s).hh.hashv == _hf_hashv
                        && (*s).hh.keylen as usize
                            == ::core::mem::size_of::<::core::ffi::c_int>()
                    {
                        if memcmp(
                            (*s).hh.key,
                            &raw mut sgid as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*s).hh.hh_next.is_null() {
                        s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut LigatureAggregator
                            as *mut LigatureAggregator;
                    } else {
                        s = ::core::ptr::null_mut::<LigatureAggregator>();
                    }
                }
            }
        }
        if s.is_null() {
            s = __caryll_allocate_clean(
                ::core::mem::size_of::<LigatureAggregator>() as usize,
                132 as ::core::ffi::c_ulong,
            ) as *mut LigatureAggregator;
            (*s).gid = sgid;
            let mut _ha_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i_0: ::core::ffi::c_uint = 0;
            let mut _hj_j_0: ::core::ffi::c_uint = 0;
            let mut _hj_k_0: ::core::ffi::c_uint = 0;
            let mut _hj_key_0: *const ::core::ffi::c_uchar =
                &raw mut (*s).gid as *const ::core::ffi::c_uchar;
            _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_0 = _hj_j_0;
            _hj_k_0 = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
                _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
                _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _ha_hashv = _ha_hashv
                .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint);
            let mut current_block_166: u64;
            match _hj_k_0 {
                11 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 3105948935974009916;
                }
                10 => {
                    current_block_166 = 3105948935974009916;
                }
                9 => {
                    current_block_166 = 16488506295619998735;
                }
                8 => {
                    current_block_166 = 2165477741955893522;
                }
                7 => {
                    current_block_166 = 16420434121503669123;
                }
                6 => {
                    current_block_166 = 4773154127383362184;
                }
                5 => {
                    current_block_166 = 11477443392506837243;
                }
                4 => {
                    current_block_166 = 4049670823543782160;
                }
                3 => {
                    current_block_166 = 8032402168972998897;
                }
                2 => {
                    current_block_166 = 13476765613733092207;
                }
                1 => {
                    current_block_166 = 7622613029762834038;
                }
                _ => {
                    current_block_166 = 7157669805658135323;
                }
            }
            match current_block_166 {
                3105948935974009916 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 16488506295619998735;
                }
                _ => {}
            }
            match current_block_166 {
                16488506295619998735 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 2165477741955893522;
                }
                _ => {}
            }
            match current_block_166 {
                2165477741955893522 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 16420434121503669123;
                }
                _ => {}
            }
            match current_block_166 {
                16420434121503669123 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 4773154127383362184;
                }
                _ => {}
            }
            match current_block_166 {
                4773154127383362184 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 11477443392506837243;
                }
                _ => {}
            }
            match current_block_166 {
                11477443392506837243 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_166 = 4049670823543782160;
                }
                _ => {}
            }
            match current_block_166 {
                4049670823543782160 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_166 = 8032402168972998897;
                }
                _ => {}
            }
            match current_block_166 {
                8032402168972998897 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_166 = 13476765613733092207;
                }
                _ => {}
            }
            match current_block_166 {
                13476765613733092207 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_166 = 7622613029762834038;
                }
                _ => {}
            }
            match current_block_166 {
                7622613029762834038 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                }
                _ => {}
            }
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            (*s).hh.hashv = _ha_hashv;
            (*s).hh.key = &raw mut (*s).gid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            (*s).hh.keylen = ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_uint;
            if h.is_null() {
                (*s).hh.next = NULL;
                (*s).hh.prev = NULL;
                (*s).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                    as *mut UtHashTable as *mut UtHashTable;
                if (*s).hh.tbl.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*s).hh.tbl as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<UtHashTable>() as usize,
                    );
                    (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
                    (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                    (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                    (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                        .offset_from(s as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as isize;
                    (*(*s).hh.tbl).buckets = malloc(
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    ) as *mut UtHashBucket;
                    (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                    if (*(*s).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        );
                    }
                }
                h = s;
            } else {
                (*s).hh.tbl = (*h).hh.tbl;
                (*s).hh.next = NULL;
                (*s).hh.prev = ((*(*h).hh.tbl).tail as *mut ::core::ffi::c_char)
                    .offset(-(*(*h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void;
                (*(*(*h).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                (*(*h).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
            }
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UtHashBucket =
                (*(*h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
            (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
            (*s).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
            (*s).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
            if !(*_ha_head).hh_head.is_null() {
                (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UtHashHandle;
            }
            (*_ha_head).hh_head = &raw mut (*s).hh as *mut UtHashHandle;
            if (*_ha_head).count
                >= (*_ha_head)
                    .expand_mult
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                && (*(*s).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: ::core::ffi::c_uint = 0;
                let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                let mut _he_thh: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _he_hh_nxt: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _he_new_buckets: *mut UtHashBucket =
                    ::core::ptr::null_mut::<UtHashBucket>();
                let mut _he_newbkt: *mut UtHashBucket = ::core::ptr::null_mut::<UtHashBucket>();
                _he_new_buckets = malloc(
                    (2 as usize)
                        .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                if _he_new_buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        _he_new_buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (2 as usize)
                            .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    );
                    (*(*s).hh.tbl).ideal_chain_maxlen = ((*(*s).hh.tbl).num_items
                        >> (*(*s).hh.tbl)
                            .log2_num_buckets
                            .wrapping_add(1 as ::core::ffi::c_uint))
                    .wrapping_add(
                        if (*(*s).hh.tbl).num_items
                            & (*(*s).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                            != 0 as ::core::ffi::c_uint
                        {
                            1 as ::core::ffi::c_uint
                        } else {
                            0 as ::core::ffi::c_uint
                        },
                    );
                    (*(*s).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                    _he_bkt_i = 0 as ::core::ffi::c_uint;
                    while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                        _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UtHashHandle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*s).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                            (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                            if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                (*(*s).hh.tbl).nonideal_items =
                                    (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt).expand_mult = (*_he_newbkt)
                                    .count
                                    .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
                            }
                            (*_he_thh).hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UtHashHandle;
                            if !(*_he_newbkt).hh_head.is_null() {
                                (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                            }
                            (*_he_newbkt).hh_head = _he_thh as *mut UtHashHandle;
                            _he_thh = _he_hh_nxt;
                        }
                        _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    }
                    free((*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    (*(*s).hh.tbl).num_buckets = (*(*s).hh.tbl)
                        .num_buckets
                        .wrapping_mul(2 as ::core::ffi::c_uint);
                    (*(*s).hh.tbl).log2_num_buckets =
                        (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
                    (*(*s).hh.tbl).buckets = _he_new_buckets;
                    (*(*s).hh.tbl).ineff_expands = if (*(*s).hh.tbl).nonideal_items
                        > (*(*s).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                    {
                        (*(*s).hh.tbl)
                            .ineff_expands
                            .wrapping_add(1 as ::core::ffi::c_uint)
                    } else {
                        0 as ::core::ffi::c_uint
                    };
                    if (*(*s).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                        (*(*s).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                    }
                }
            }
        }
        j = j.wrapping_add(1);
    }
    let mut _hs_i: ::core::ffi::c_uint = 0;
    let mut _hs_looping: ::core::ffi::c_uint = 0;
    let mut _hs_nmerges: ::core::ffi::c_uint = 0;
    let mut _hs_insize: ::core::ffi::c_uint = 0;
    let mut _hs_psize: ::core::ffi::c_uint = 0;
    let mut _hs_qsize: ::core::ffi::c_uint = 0;
    let mut _hs_p: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_q: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_e: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_list: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    let mut _hs_tail: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
    if !h.is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (*h).hh as *mut UtHashHandle;
        while _hs_looping != 0 as ::core::ffi::c_uint {
            _hs_p = _hs_list;
            _hs_list = ::core::ptr::null_mut::<UtHashHandle>();
            _hs_tail = ::core::ptr::null_mut::<UtHashHandle>();
            _hs_nmerges = 0 as ::core::ffi::c_uint;
            while !_hs_p.is_null() {
                _hs_nmerges = _hs_nmerges.wrapping_add(1);
                _hs_q = _hs_p;
                _hs_psize = 0 as ::core::ffi::c_uint;
                _hs_i = 0 as ::core::ffi::c_uint;
                while _hs_i < _hs_insize {
                    _hs_psize = _hs_psize.wrapping_add(1);
                    _hs_q = (if !(*_hs_q).next.is_null() {
                        ((*_hs_q).next as *mut ::core::ffi::c_char)
                            .offset((*(*h).hh.tbl).hho)
                            as *mut UtHashHandle
                    } else {
                        ::core::ptr::null_mut::<UtHashHandle>()
                    }) as *mut UtHashHandle;
                    if _hs_q.is_null() {
                        break;
                    }
                    _hs_i = _hs_i.wrapping_add(1);
                }
                _hs_qsize = _hs_insize;
                while _hs_psize != 0 as ::core::ffi::c_uint
                    || _hs_qsize != 0 as ::core::ffi::c_uint && !_hs_q.is_null()
                {
                    if _hs_psize == 0 as ::core::ffi::c_uint {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(*h).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    } else if _hs_qsize == 0 as ::core::ffi::c_uint || _hs_q.is_null() {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*h).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if by_gid(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut LigatureAggregator,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut LigatureAggregator,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(*h).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else {
                        _hs_e = _hs_q;
                        _hs_q = (if !(*_hs_q).next.is_null() {
                            ((*_hs_q).next as *mut ::core::ffi::c_char)
                                .offset((*(*h).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(*h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    } else {
                        _hs_list = _hs_e;
                    }
                    if !_hs_e.is_null() {
                        (*_hs_e).prev = if !_hs_tail.is_null() {
                            (_hs_tail as *mut ::core::ffi::c_char)
                                .offset(-(*(*h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                        } else {
                            NULL
                        };
                    }
                    _hs_tail = _hs_e;
                }
                _hs_p = _hs_q;
            }
            if !_hs_tail.is_null() {
                (*_hs_tail).next = NULL;
            }
            if _hs_nmerges <= 1 as ::core::ffi::c_uint {
                _hs_looping = 0 as ::core::ffi::c_uint;
                (*(*h).hh.tbl).tail = _hs_tail;
                h = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(*h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut LigatureAggregator
                    as *mut LigatureAggregator;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut startcov: *mut Coverage = otl_coverage_create();
    s = h;
    while !s.is_null() {
        push_to_coverage(
            startcov,
            handle_from_index((*s).gid as GlyphId)
                as GlyphHandle,
        );
        s = (*s).hh.next as *mut LigatureAggregator;
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_ptr(BkCellType::P16, bk_new_block_from_buffer(OTL_I_COVERAGE.build.expect("non-null function pointer")(
            startcov,
        ))), bk_int(BkCellType::B16, ((*startcov).len() as ::core::ffi::c_int) as u32)]);
    s = h;
    while !s.is_null() {
        let mut n_ligs_here: GlyphId = 0 as GlyphId;
        let mut j_0: GlyphId = 0 as GlyphId;
        while (j_0 as ::core::ffi::c_int) < n_ligatures as ::core::ffi::c_int {
            if (&(*(&(*subtable))[j_0 as usize].from))[0]
            .index as ::core::ffi::c_int
                == (*s).gid
            {
                n_ligs_here = n_ligs_here.wrapping_add(1);
            }
            j_0 = j_0.wrapping_add(1);
        }
        let mut ligset: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (n_ligs_here as ::core::ffi::c_int) as u32)]);
        let mut j_1: GlyphId = 0 as GlyphId;
        while (j_1 as ::core::ffi::c_int) < n_ligatures as ::core::ffi::c_int {
            if (&(*(&(*subtable))[j_1 as usize].from))[0]
            .index as ::core::ffi::c_int
                == (*s).gid
            {
                let mut ligdef: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((&(*subtable))[j_1 as usize].to.index as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*(&(*subtable))[j_1 as usize].from).len()
                        as ::core::ffi::c_int) as u32)]);
                let mut m: GlyphId = 1 as GlyphId;
                while (m as ::core::ffi::c_int)
                    < (*(&(*subtable))[j_1 as usize].from).len()
                        as ::core::ffi::c_int
                {
                    bk_push(ligdef, &[bk_int(BkCellType::B16, ((&(*(&(*subtable))[j_1 as usize].from))[m as usize]
                        .index as ::core::ffi::c_int) as u32)]);
                    m = m.wrapping_add(1);
                }
                bk_push(ligset, &[bk_ptr(BkCellType::P16, ligdef)]);
            }
            j_1 = j_1.wrapping_add(1);
        }
        bk_push(root, &[bk_ptr(BkCellType::P16, ligset)]);
        s = (*s).hh.next as *mut LigatureAggregator;
    }
    otl_coverage_free(startcov);
    s = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut LigatureAggregator
        as *mut LigatureAggregator;
    while !s.is_null() {
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<LigatureAggregator>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh2 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh2 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut LigatureAggregator as *mut LigatureAggregator;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh3 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh3 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket =
                (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
            (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_sub(1);
        }
        free(s as *mut ::core::ffi::c_void);
        s = ::core::ptr::null_mut::<LigatureAggregator>();
        s = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut LigatureAggregator
            as *mut LigatureAggregator;
    }
    return bk_build_block(root);
}
