#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset, strcmp, strlen};
use crate::support::json_funcs::{json_new_position, json_obj_get_type, json_obj_getnum, json_obj_getnum_fallback, preserialize};
use crate::table::otl::coverage::{Coverage};
use crate::support::handle::{handle_from_name, otfcc_handle_dispose, otfcc_handle_dup, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::binio::{pos_to_u16, read_16u, read_16s};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId, Pos};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_push};
use crate::support::{NULL};
use crate::table::otl::{Anchor, MarkArray, MarkRecord, PositionValue};
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
use crate::support::buffer::{bufwrite16b};
use crate::vendor::json_builder::{json_null_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsfree, sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClassNameHash {
    pub class_name: SdsRaw,
    pub class_id: GlyphClass,
    pub hh: UtHashHandle,
}
unsafe extern "C" fn delete_mark_array_item(mut entry: *mut MarkRecord) {
    otfcc_handle_dispose(&raw mut (*entry).glyph);
}
pub(crate) unsafe fn dispose_mark_array(arr: *mut MarkArray) {
    for e in (*arr).iter_mut() {
        delete_mark_array_item(e);
    }
    *arr = Vec::new();
}
pub unsafe extern "C" fn otl_read_mark_array(
    mut array: *mut MarkArray,
    mut cov: *mut Coverage,
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
) {
    let mut mark_count: GlyphId = 0;
    if !(table_length < offset.wrapping_add(2 as u32)) {
        mark_count = read_16u(data.offset(offset as isize) as *const u8) as GlyphId;
        let mut j: GlyphId = 0 as GlyphId;
        while (j as ::core::ffi::c_int) < mark_count as ::core::ffi::c_int {
            let mut mark_class: GlyphClass = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset((j as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as *const u8,
            ) as GlyphClass;
            let mut delta: u16 = read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset((j as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            );
            if delta != 0 {
                (*array).push(
                    MarkRecord {
                        glyph: otfcc_handle_dup(
                            (*(*cov).glyphs.offset(j as isize)).clone() as Handle,
                        ) as GlyphHandle,
                        mark_class: mark_class,
                        anchor: otl_read_anchor(
                            data,
                            table_length,
                            offset.wrapping_add(delta as u32),
                        ),
                    },
                );
            } else {
                (*array).push(
                    MarkRecord {
                        glyph: otfcc_handle_dup(
                            (*(*cov).glyphs.offset(j as isize)).clone() as Handle,
                        ) as GlyphHandle,
                        mark_class: mark_class,
                        anchor: otl_anchor_absent(),
                    },
                );
            }
            j = j.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn compare_class_hash(
    mut a: *mut ClassNameHash,
    mut b: *mut ClassNameHash,
) -> ::core::ffi::c_int {
    return strcmp(
        (*a).class_name as *const ::core::ffi::c_char,
        (*b).class_name as *const ::core::ffi::c_char,
    );
}
pub unsafe extern "C" fn otl_parse_mark_array(
    mut _marks: *mut JsonValue,
    mut array: *mut MarkArray,
    mut h: *mut *mut ClassNameHash,
    mut _options: *const Options,
) {
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < (*_marks).u.object.length {
        let mut mark: MarkRecord = MarkRecord {
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            mark_class: 0,
            anchor: Anchor {
                present: false,
                x: 0.,
                y: 0.,
            },
        };
        let mut gname: *mut ::core::ffi::c_char =
            (*(*_marks).u.object.values.offset(j as isize)).name;
        let mut anchor_record: *mut JsonValue =
            (*(*_marks).u.object.values.offset(j as isize)).value as *mut JsonValue;
        mark.glyph = handle_from_name(sdsnewlen(
            gname as *const ::core::ffi::c_void,
            (*(*_marks).u.object.values.offset(j as isize)).name_length as usize,
        )) as GlyphHandle;
        mark.mark_class = 0 as GlyphClass;
        mark.anchor = otl_anchor_absent();
        if anchor_record.is_null()
            || (*anchor_record).type_0 != JsonType::Object
        {
            (*array).push(mark);
        } else {
            let mut _class_name: *mut JsonValue = json_obj_get_type(
                anchor_record,
                b"class\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if _class_name.is_null() {
                (*array).push(mark);
            } else {
                let mut class_name: SdsRaw = sdsnewlen(
                    (*_class_name).u.string.ptr as *const ::core::ffi::c_void,
                    (*_class_name).u.string.length as usize,
                );
                let mut s: *mut ClassNameHash = ::core::ptr::null_mut::<ClassNameHash>();
                let mut _hf_hashv: ::core::ffi::c_uint = 0;
                let mut _hj_i: ::core::ffi::c_uint = 0;
                let mut _hj_j: ::core::ffi::c_uint = 0;
                let mut _hj_k: ::core::ffi::c_uint = 0;
                let mut _hj_key: *const ::core::ffi::c_uchar =
                    class_name as *const ::core::ffi::c_uchar;
                _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
                _hj_i = _hj_j;
                _hj_k = strlen(class_name as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                while _hj_k >= 12 as ::core::ffi::c_uint {
                    _hj_i = _hj_i.wrapping_add(
                        (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hj_j = _hj_j.wrapping_add(
                        (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            ),
                    );
                    _hf_hashv = _hf_hashv.wrapping_add(
                        (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(11 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
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
                _hf_hashv = _hf_hashv.wrapping_add(
                    strlen(class_name as *const ::core::ffi::c_char) as ::core::ffi::c_uint
                );
                let mut current_block_55: u64;
                match _hj_k {
                    11 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_55 = 4184970055425330224;
                    }
                    10 => {
                        current_block_55 = 4184970055425330224;
                    }
                    9 => {
                        current_block_55 = 13800538852034404314;
                    }
                    8 => {
                        current_block_55 = 15463853910180707538;
                    }
                    7 => {
                        current_block_55 = 17555299131552860298;
                    }
                    6 => {
                        current_block_55 = 3137062391300010043;
                    }
                    5 => {
                        current_block_55 = 14607299304144994639;
                    }
                    4 => {
                        current_block_55 = 1223993795581701498;
                    }
                    3 => {
                        current_block_55 = 4239524923856774895;
                    }
                    2 => {
                        current_block_55 = 9705619680375831020;
                    }
                    1 => {
                        current_block_55 = 7019012577490997641;
                    }
                    _ => {
                        current_block_55 = 5141539773904409130;
                    }
                }
                match current_block_55 {
                    4184970055425330224 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_55 = 13800538852034404314;
                    }
                    _ => {}
                }
                match current_block_55 {
                    13800538852034404314 => {
                        _hf_hashv = _hf_hashv.wrapping_add(
                            (*_hj_key.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_55 = 15463853910180707538;
                    }
                    _ => {}
                }
                match current_block_55 {
                    15463853910180707538 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_55 = 17555299131552860298;
                    }
                    _ => {}
                }
                match current_block_55 {
                    17555299131552860298 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_55 = 3137062391300010043;
                    }
                    _ => {}
                }
                match current_block_55 {
                    3137062391300010043 => {
                        _hj_j = _hj_j.wrapping_add(
                            (*_hj_key.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_55 = 14607299304144994639;
                    }
                    _ => {}
                }
                match current_block_55 {
                    14607299304144994639 => {
                        _hj_j = _hj_j
                            .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint);
                        current_block_55 = 1223993795581701498;
                    }
                    _ => {}
                }
                match current_block_55 {
                    1223993795581701498 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        );
                        current_block_55 = 4239524923856774895;
                    }
                    _ => {}
                }
                match current_block_55 {
                    4239524923856774895 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        );
                        current_block_55 = 9705619680375831020;
                    }
                    _ => {}
                }
                match current_block_55 {
                    9705619680375831020 => {
                        _hj_i = _hj_i.wrapping_add(
                            (*_hj_key.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        );
                        current_block_55 = 7019012577490997641;
                    }
                    _ => {}
                }
                match current_block_55 {
                    7019012577490997641 => {
                        _hj_i = _hj_i
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
                s = ::core::ptr::null_mut::<ClassNameHash>();
                if !(*h).is_null() {
                    let mut _hf_bkt: ::core::ffi::c_uint = 0;
                    _hf_bkt = _hf_hashv
                        & (*(**h).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize))
                            .hh_head
                            .is_null()
                        {
                            s = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                                as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut ClassNameHash
                                as *mut ClassNameHash;
                        } else {
                            s = ::core::ptr::null_mut::<ClassNameHash>();
                        }
                        while !s.is_null() {
                            if (*s).hh.hashv == _hf_hashv
                                && (*s).hh.keylen
                                    == strlen(class_name as *const ::core::ffi::c_char)
                                        as ::core::ffi::c_uint
                            {
                                if memcmp(
                                    (*s).hh.key,
                                    class_name as *const ::core::ffi::c_void,
                                    strlen(class_name as *const ::core::ffi::c_char)
                                        as ::core::ffi::c_uint
                                        as usize,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    break;
                                }
                            }
                            if !(*s).hh.hh_next.is_null() {
                                s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                                    .offset(-(*(**h).hh.tbl).hho)
                                    as *mut ::core::ffi::c_void
                                    as *mut ClassNameHash
                                    as *mut ClassNameHash;
                            } else {
                                s = ::core::ptr::null_mut::<ClassNameHash>();
                            }
                        }
                    }
                }
                if s.is_null() {
                    s = __caryll_allocate_clean(
                        ::core::mem::size_of::<ClassNameHash>() as usize,
                        61 as ::core::ffi::c_ulong,
                    ) as *mut ClassNameHash;
                    (*s).class_name = class_name;
                    (*s).class_id = (if !(*h).is_null() {
                        (*(**h).hh.tbl).num_items
                    } else {
                        0 as ::core::ffi::c_uint
                    }) as GlyphClass;
                    let mut _ha_hashv: ::core::ffi::c_uint = 0;
                    let mut _hj_i_0: ::core::ffi::c_uint = 0;
                    let mut _hj_j_0: ::core::ffi::c_uint = 0;
                    let mut _hj_k_0: ::core::ffi::c_uint = 0;
                    let mut _hj_key_0: *const ::core::ffi::c_uchar =
                        (*s).class_name.offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_uchar;
                    _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
                    _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
                    _hj_i_0 = _hj_j_0;
                    _hj_k_0 =
                        strlen((*s).class_name as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                    while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                        _hj_i_0 = _hj_i_0.wrapping_add(
                            (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                            (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
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
                        .wrapping_add(strlen((*s).class_name as *const ::core::ffi::c_char)
                            as ::core::ffi::c_uint);
                    let mut current_block_172: u64;
                    match _hj_k_0 {
                        11 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_172 = 898468936263527080;
                        }
                        10 => {
                            current_block_172 = 898468936263527080;
                        }
                        9 => {
                            current_block_172 = 9698739436995956065;
                        }
                        8 => {
                            current_block_172 = 10600459493909746493;
                        }
                        7 => {
                            current_block_172 = 9773837153719584703;
                        }
                        6 => {
                            current_block_172 = 6790636896960429817;
                        }
                        5 => {
                            current_block_172 = 1376536996662481425;
                        }
                        4 => {
                            current_block_172 = 16671533464018076928;
                        }
                        3 => {
                            current_block_172 = 6609393363672095876;
                        }
                        2 => {
                            current_block_172 = 9029570263382640504;
                        }
                        1 => {
                            current_block_172 = 9698912694449281435;
                        }
                        _ => {
                            current_block_172 = 5219368551394180541;
                        }
                    }
                    match current_block_172 {
                        898468936263527080 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_172 = 9698739436995956065;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        9698739436995956065 => {
                            _ha_hashv = _ha_hashv.wrapping_add(
                                (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_172 = 10600459493909746493;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        10600459493909746493 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_172 = 9773837153719584703;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        9773837153719584703 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_172 = 6790636896960429817;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        6790636896960429817 => {
                            _hj_j_0 = _hj_j_0.wrapping_add(
                                (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_172 = 1376536996662481425;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        1376536996662481425 => {
                            _hj_j_0 = _hj_j_0
                                .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint);
                            current_block_172 = 16671533464018076928;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        16671533464018076928 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int,
                            );
                            current_block_172 = 6609393363672095876;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        6609393363672095876 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 16 as ::core::ffi::c_int,
                            );
                            current_block_172 = 9029570263382640504;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        9029570263382640504 => {
                            _hj_i_0 = _hj_i_0.wrapping_add(
                                (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int,
                            );
                            current_block_172 = 9698912694449281435;
                        }
                        _ => {}
                    }
                    match current_block_172 {
                        9698912694449281435 => {
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
                    (*s).hh.key = (*s).class_name.offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void;
                    (*s).hh.keylen =
                        strlen((*s).class_name as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
                    if (*h).is_null() {
                        (*s).hh.next = NULL;
                        (*s).hh.prev = NULL;
                        (*s).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                            as *mut UtHashTable
                            as *mut UtHashTable;
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
                            (*(*s).hh.tbl).buckets =
                                malloc((32 as usize).wrapping_mul(::core::mem::size_of::<
                                    UtHashBucket,
                                >(
                                )
                                    as usize))
                                    as *mut UtHashBucket;
                            (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                            if (*(*s).hh.tbl).buckets.is_null() {
                                exit(-(1 as ::core::ffi::c_int));
                            } else {
                                memset(
                                    (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                                    '\0' as i32,
                                    (32 as usize).wrapping_mul(
                                        ::core::mem::size_of::<UtHashBucket>() as usize,
                                    ),
                                );
                            }
                        }
                        *h = s;
                    } else {
                        (*s).hh.tbl = (**h).hh.tbl;
                        (*s).hh.next = NULL;
                        (*s).hh.prev = ((*(**h).hh.tbl).tail as *mut ::core::ffi::c_char)
                            .offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void;
                        (*(*(**h).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                        (*(**h).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
                    }
                    let mut _ha_bkt: ::core::ffi::c_uint = 0;
                    (*(**h).hh.tbl).num_items = (*(**h).hh.tbl).num_items.wrapping_add(1);
                    _ha_bkt = _ha_hashv
                        & (*(**h).hh.tbl)
                            .num_buckets
                            .wrapping_sub(1 as ::core::ffi::c_uint);
                    let mut _ha_head: *mut UtHashBucket =
                        (*(**h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
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
                        let mut _he_thh: *mut UtHashHandle =
                            ::core::ptr::null_mut::<UtHashHandle>();
                        let mut _he_hh_nxt: *mut UtHashHandle =
                            ::core::ptr::null_mut::<UtHashHandle>();
                        let mut _he_new_buckets: *mut UtHashBucket =
                            ::core::ptr::null_mut::<UtHashBucket>();
                        let mut _he_newbkt: *mut UtHashBucket =
                            ::core::ptr::null_mut::<UtHashBucket>();
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
                                    .wrapping_mul(
                                        ::core::mem::size_of::<UtHashBucket>() as usize
                                    ),
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
                                _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize))
                                    .hh_head
                                    as *mut UtHashHandle;
                                while !_he_thh.is_null() {
                                    _he_hh_nxt = (*_he_thh).hh_next;
                                    _he_bkt = (*_he_thh).hashv
                                        & (*(*s).hh.tbl)
                                            .num_buckets
                                            .wrapping_mul(2 as ::core::ffi::c_uint)
                                            .wrapping_sub(1 as ::core::ffi::c_uint);
                                    _he_newbkt = _he_new_buckets.offset(_he_bkt as isize)
                                        as *mut UtHashBucket;
                                    (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                                    if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                        (*(*s).hh.tbl).nonideal_items =
                                            (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                        (*_he_newbkt).expand_mult = (*_he_newbkt)
                                            .count
                                            .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
                                    }
                                    (*_he_thh).hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                                    (*_he_thh).hh_next =
                                        (*_he_newbkt).hh_head as *mut UtHashHandle;
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
                } else {
                    sdsfree(class_name);
                }
                mark.mark_class = (*s).class_id;
                mark.anchor.present = true;
                mark.anchor.x = json_obj_getnum(
                    anchor_record,
                    b"x\0" as *const u8 as *const ::core::ffi::c_char,
                ) as Pos;
                mark.anchor.y = json_obj_getnum(
                    anchor_record,
                    b"y\0" as *const u8 as *const ::core::ffi::c_char,
                ) as Pos;
                (*array).push(mark);
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
    if !(*h).is_null() {
        _hs_insize = 1 as ::core::ffi::c_uint;
        _hs_looping = 1 as ::core::ffi::c_uint;
        _hs_list = &raw mut (**h).hh as *mut UtHashHandle;
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
                            .offset((*(**h).hh.tbl).hho)
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
                                .offset((*(**h).hh.tbl).hho)
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
                                    .offset((*(**h).hh.tbl).hho)
                                    as *mut UtHashHandle
                            } else {
                                ::core::ptr::null_mut::<UtHashHandle>()
                            }) as *mut UtHashHandle;
                        }
                        _hs_psize = _hs_psize.wrapping_sub(1);
                    } else if compare_class_hash(
                        (_hs_p as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ClassNameHash,
                        (_hs_q as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ClassNameHash,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        _hs_e = _hs_p;
                        if !_hs_p.is_null() {
                            _hs_p = (if !(*_hs_p).next.is_null() {
                                ((*_hs_p).next as *mut ::core::ffi::c_char)
                                    .offset((*(**h).hh.tbl).hho)
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
                                .offset((*(**h).hh.tbl).hho)
                                as *mut UtHashHandle
                        } else {
                            ::core::ptr::null_mut::<UtHashHandle>()
                        }) as *mut UtHashHandle;
                        _hs_qsize = _hs_qsize.wrapping_sub(1);
                    }
                    if !_hs_tail.is_null() {
                        (*_hs_tail).next = if !_hs_e.is_null() {
                            (_hs_e as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
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
                                .offset(-(*(**h).hh.tbl).hho)
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
                (*(**h).hh.tbl).tail = _hs_tail;
                *h = (_hs_list as *mut ::core::ffi::c_char).offset(-(*(**h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void as *mut ClassNameHash
                    as *mut ClassNameHash;
            }
            _hs_insize = _hs_insize.wrapping_mul(2 as ::core::ffi::c_uint);
        }
    }
    let mut j_anchor_index: GlyphId = 0 as GlyphId;
    let mut s_0: *mut ClassNameHash = ::core::ptr::null_mut::<ClassNameHash>();
    s_0 = *h;
    while !s_0.is_null() {
        (*s_0).class_id = j_anchor_index as GlyphClass;
        j_anchor_index = j_anchor_index.wrapping_add(1);
        s_0 = (*s_0).hh.next as *mut ClassNameHash;
    }
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*array).len() {
        if (&(*array))[j_0 as usize].anchor.present {
            let mut anchor_record_0: *mut JsonValue =
                (*(*_marks).u.object.values.offset(j_0 as isize)).value as *mut JsonValue;
            let mut _class_name_0: *mut JsonValue = json_obj_get_type(
                anchor_record_0,
                b"class\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            let mut class_name_0: SdsRaw = sdsnewlen(
                (*_class_name_0).u.string.ptr as *const ::core::ffi::c_void,
                (*_class_name_0).u.string.length as usize,
            );
            let mut s_1: *mut ClassNameHash = ::core::ptr::null_mut::<ClassNameHash>();
            let mut _hf_hashv_0: ::core::ffi::c_uint = 0;
            let mut _hj_i_1: ::core::ffi::c_uint = 0;
            let mut _hj_j_1: ::core::ffi::c_uint = 0;
            let mut _hj_k_1: ::core::ffi::c_uint = 0;
            let mut _hj_key_1: *const ::core::ffi::c_uchar =
                class_name_0 as *const ::core::ffi::c_uchar;
            _hf_hashv_0 = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_1 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_1 = _hj_j_1;
            _hj_k_1 = strlen(class_name_0 as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
            while _hj_k_1 >= 12 as ::core::ffi::c_uint {
                _hj_i_1 = _hj_i_1.wrapping_add(
                    (*_hj_key_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j_1 = _hj_j_1.wrapping_add(
                    (*_hj_key_1.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                    (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_1.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
                _hj_i_1 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
                _hf_hashv_0 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
                _hj_i_1 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
                _hf_hashv_0 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
                _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
                _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
                _hj_i_1 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
                _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
                _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
                _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
                _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
                _hf_hashv_0 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
                _hj_key_1 = _hj_key_1.offset(12 as ::core::ffi::c_int as isize);
                _hj_k_1 = _hj_k_1.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                strlen(class_name_0 as *const ::core::ffi::c_char) as ::core::ffi::c_uint,
            );
            let mut current_block_445: u64;
            match _hj_k_1 {
                11 => {
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                        (*_hj_key_1.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_445 = 4247882375651910782;
                }
                10 => {
                    current_block_445 = 4247882375651910782;
                }
                9 => {
                    current_block_445 = 16226128822898203720;
                }
                8 => {
                    current_block_445 = 9931377106148496891;
                }
                7 => {
                    current_block_445 = 14712804663128342644;
                }
                6 => {
                    current_block_445 = 5445440423012481275;
                }
                5 => {
                    current_block_445 = 62599083018845058;
                }
                4 => {
                    current_block_445 = 161668369318445431;
                }
                3 => {
                    current_block_445 = 10547560897237185998;
                }
                2 => {
                    current_block_445 = 1296394692977688829;
                }
                1 => {
                    current_block_445 = 15921629307266798929;
                }
                _ => {
                    current_block_445 = 18272884058186558579;
                }
            }
            match current_block_445 {
                4247882375651910782 => {
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                        (*_hj_key_1.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_445 = 16226128822898203720;
                }
                _ => {}
            }
            match current_block_445 {
                16226128822898203720 => {
                    _hf_hashv_0 = _hf_hashv_0.wrapping_add(
                        (*_hj_key_1.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_445 = 9931377106148496891;
                }
                _ => {}
            }
            match current_block_445 {
                9931377106148496891 => {
                    _hj_j_1 = _hj_j_1.wrapping_add(
                        (*_hj_key_1.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_445 = 14712804663128342644;
                }
                _ => {}
            }
            match current_block_445 {
                14712804663128342644 => {
                    _hj_j_1 = _hj_j_1.wrapping_add(
                        (*_hj_key_1.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_445 = 5445440423012481275;
                }
                _ => {}
            }
            match current_block_445 {
                5445440423012481275 => {
                    _hj_j_1 = _hj_j_1.wrapping_add(
                        (*_hj_key_1.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_445 = 62599083018845058;
                }
                _ => {}
            }
            match current_block_445 {
                62599083018845058 => {
                    _hj_j_1 = _hj_j_1
                        .wrapping_add(*_hj_key_1.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_445 = 161668369318445431;
                }
                _ => {}
            }
            match current_block_445 {
                161668369318445431 => {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_445 = 10547560897237185998;
                }
                _ => {}
            }
            match current_block_445 {
                10547560897237185998 => {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_445 = 1296394692977688829;
                }
                _ => {}
            }
            match current_block_445 {
                1296394692977688829 => {
                    _hj_i_1 = _hj_i_1.wrapping_add(
                        (*_hj_key_1.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_445 = 15921629307266798929;
                }
                _ => {}
            }
            match current_block_445 {
                15921629307266798929 => {
                    _hj_i_1 = _hj_i_1
                        .wrapping_add(*_hj_key_1.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                }
                _ => {}
            }
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 13 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 8 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 13 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 12 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 16 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 5 as ::core::ffi::c_int;
            _hj_i_1 = _hj_i_1.wrapping_sub(_hj_j_1);
            _hj_i_1 = _hj_i_1.wrapping_sub(_hf_hashv_0);
            _hj_i_1 ^= _hf_hashv_0 >> 3 as ::core::ffi::c_int;
            _hj_j_1 = _hj_j_1.wrapping_sub(_hf_hashv_0);
            _hj_j_1 = _hj_j_1.wrapping_sub(_hj_i_1);
            _hj_j_1 ^= _hj_i_1 << 10 as ::core::ffi::c_int;
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_i_1);
            _hf_hashv_0 = _hf_hashv_0.wrapping_sub(_hj_j_1);
            _hf_hashv_0 ^= _hj_j_1 >> 15 as ::core::ffi::c_int;
            s_1 = ::core::ptr::null_mut::<ClassNameHash>();
            if !(*h).is_null() {
                let mut _hf_bkt_0: ::core::ffi::c_uint = 0;
                _hf_bkt_0 = _hf_hashv_0
                    & (*(**h).hh.tbl)
                        .num_buckets
                        .wrapping_sub(1 as ::core::ffi::c_uint);
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if !(*(*(**h).hh.tbl).buckets.offset(_hf_bkt_0 as isize))
                        .hh_head
                        .is_null()
                    {
                        s_1 = ((*(*(**h).hh.tbl).buckets.offset(_hf_bkt_0 as isize)).hh_head
                            as *mut ::core::ffi::c_char)
                            .offset(-(*(**h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ClassNameHash
                            as *mut ClassNameHash;
                    } else {
                        s_1 = ::core::ptr::null_mut::<ClassNameHash>();
                    }
                    while !s_1.is_null() {
                        if (*s_1).hh.hashv == _hf_hashv_0
                            && (*s_1).hh.keylen
                                == strlen(class_name_0 as *const ::core::ffi::c_char)
                                    as ::core::ffi::c_uint
                        {
                            if memcmp(
                                (*s_1).hh.key,
                                class_name_0 as *const ::core::ffi::c_void,
                                strlen(class_name_0 as *const ::core::ffi::c_char)
                                    as ::core::ffi::c_uint
                                    as usize,
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                        }
                        if !(*s_1).hh.hh_next.is_null() {
                            s_1 = ((*s_1).hh.hh_next as *mut ::core::ffi::c_char)
                                .offset(-(*(**h).hh.tbl).hho)
                                as *mut ::core::ffi::c_void
                                as *mut ClassNameHash
                                as *mut ClassNameHash;
                        } else {
                            s_1 = ::core::ptr::null_mut::<ClassNameHash>();
                        }
                    }
                }
            }
            if !s_1.is_null() {
                (&mut (*array))[j_0 as usize].mark_class = (*s_1).class_id;
            } else {
                (&mut (*array))[j_0 as usize].mark_class = 0 as GlyphClass;
            }
            sdsfree(class_name_0);
        }
        j_0 = j_0.wrapping_add(1);
    }
}
pub unsafe extern "C" fn otl_anchor_absent() -> Anchor {
    let mut anchor: Anchor = Anchor {
        present: false,
        x: 0 as ::core::ffi::c_int as Pos,
        y: 0 as ::core::ffi::c_int as Pos,
    };
    return anchor;
}
pub unsafe extern "C" fn otl_read_anchor(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
) -> Anchor {
    let mut anchor: Anchor = Anchor {
        present: false,
        x: 0 as ::core::ffi::c_int as Pos,
        y: 0 as ::core::ffi::c_int as Pos,
    };
    if table_length < offset.wrapping_add(6 as u32) {
        anchor.present = false;
        anchor.x = 0 as ::core::ffi::c_int as Pos;
        anchor.y = 0 as ::core::ffi::c_int as Pos;
        return anchor;
    } else {
        anchor.present = true;
        anchor.x = read_16s(
            data.offset(offset as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as Pos;
        anchor.y = read_16s(
            data.offset(offset as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as Pos;
        return anchor;
    };
}
pub unsafe extern "C" fn otl_dump_anchor(mut a: Anchor) -> *mut JsonValue {
    if a.present {
        let mut v: *mut JsonValue = json_object_new(2 as usize);
        json_object_push(
            v,
            b"x\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(a.x),
        );
        json_object_push(
            v,
            b"y\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(a.y),
        );
        return v;
    } else {
        return json_null_new();
    };
}
pub unsafe extern "C" fn otl_parse_anchor(mut v: *mut JsonValue) -> Anchor {
    let mut anchor: Anchor = Anchor {
        present: false,
        x: 0 as ::core::ffi::c_int as Pos,
        y: 0 as ::core::ffi::c_int as Pos,
    };
    if v.is_null()
        || (*v).type_0 != JsonType::Object
    {
        return anchor;
    }
    anchor.present = true;
    anchor.x = json_obj_getnum_fallback(
        v,
        b"x\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as Pos;
    anchor.y = json_obj_getnum_fallback(
        v,
        b"y\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    ) as Pos;
    return anchor;
}
pub unsafe extern "C" fn bk_from_anchor(mut a: Anchor) -> *mut BkBlock {
    if !a.present {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    return bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_int(BkCellType::B16, (a.x as i16 as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (a.y as i16 as ::core::ffi::c_int) as u32)]);
}
pub static FORMAT_DX: u8 = 1 as u8;
pub static FORMAT_DY: u8 = 2 as u8;
pub static FORMAT_DWIDTH: u8 = 4 as u8;
pub static FORMAT_DHEIGHT: u8 = 8 as u8;
pub static BITS_IN: [u8; 256] = [
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 0 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as u8,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as u8,
];
pub unsafe extern "C" fn position_format_length(mut format: u16) -> u8 {
    return ((BITS_IN[(format as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int) as u8;
}
pub unsafe extern "C" fn position_zero() -> PositionValue {
    let mut v: PositionValue = PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        d_width: 0.0f64,
        d_height: 0.0f64,
    };
    return v;
}
pub unsafe extern "C" fn read_gpos_value(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u32,
    mut format: u16,
) -> PositionValue {
    let mut v: PositionValue = PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        d_width: 0.0f64,
        d_height: 0.0f64,
    };
    if table_length < offset.wrapping_add(position_format_length(format) as u32) {
        return v;
    }
    if format as ::core::ffi::c_int & FORMAT_DX as ::core::ffi::c_int != 0 {
        v.dx = read_16s(data.offset(offset as isize) as *const u8) as Pos;
        offset = offset.wrapping_add(2 as u32);
    }
    if format as ::core::ffi::c_int & FORMAT_DY as ::core::ffi::c_int != 0 {
        v.dy = read_16s(data.offset(offset as isize) as *const u8) as Pos;
        offset = offset.wrapping_add(2 as u32);
    }
    if format as ::core::ffi::c_int & FORMAT_DWIDTH as ::core::ffi::c_int != 0 {
        v.d_width = read_16s(data.offset(offset as isize) as *const u8) as Pos;
        offset = offset.wrapping_add(2 as u32);
    }
    if format as ::core::ffi::c_int & FORMAT_DHEIGHT as ::core::ffi::c_int != 0 {
        v.d_height = read_16s(data.offset(offset as isize) as *const u8) as Pos;
        offset = offset.wrapping_add(2 as u32);
    }
    return v;
}
pub unsafe extern "C" fn gpos_dump_value(mut value: PositionValue) -> *mut JsonValue {
    let mut v: *mut JsonValue = json_object_new(4 as usize);
    if value.dx != 0. {
        json_object_push(
            v,
            b"dx\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(value.dx),
        );
    }
    if value.dy != 0. {
        json_object_push(
            v,
            b"dy\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(value.dy),
        );
    }
    if value.d_width != 0. {
        json_object_push(
            v,
            b"dWidth\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(value.d_width),
        );
    }
    if value.d_height != 0. {
        json_object_push(
            v,
            b"dHeight\0" as *const u8 as *const ::core::ffi::c_char,
            json_new_position(value.d_height),
        );
    }
    return preserialize(v);
}
pub unsafe extern "C" fn gpos_parse_value(mut pos: *mut JsonValue) -> PositionValue {
    let mut v: PositionValue = PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        d_width: 0.0f64,
        d_height: 0.0f64,
    };
    if pos.is_null()
        || (*pos).type_0 != JsonType::Object
    {
        return v;
    }
    v.dx = json_obj_getnum(pos, b"dx\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
    v.dy = json_obj_getnum(pos, b"dy\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
    v.d_width =
        json_obj_getnum(pos, b"dWidth\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
    v.d_height =
        json_obj_getnum(pos, b"dHeight\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
    return v;
}
pub unsafe extern "C" fn required_position_format(mut v: PositionValue) -> u8 {
    return ((if v.dx != 0. {
        FORMAT_DX as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) | (if v.dy != 0. {
        FORMAT_DY as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) | (if v.d_width != 0. {
        FORMAT_DWIDTH as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) | (if v.d_height != 0. {
        FORMAT_DHEIGHT as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    })) as u8;
}
pub unsafe extern "C" fn write_gpos_value(
    mut buf: *mut Buffer,
    mut v: PositionValue,
    mut format: u16,
) {
    if format as ::core::ffi::c_int & FORMAT_DX as ::core::ffi::c_int != 0 {
        bufwrite16b(buf, pos_to_u16(v.dx));
    }
    if format as ::core::ffi::c_int & FORMAT_DY as ::core::ffi::c_int != 0 {
        bufwrite16b(buf, pos_to_u16(v.dy));
    }
    if format as ::core::ffi::c_int & FORMAT_DWIDTH as ::core::ffi::c_int != 0 {
        bufwrite16b(buf, pos_to_u16(v.d_width));
    }
    if format as ::core::ffi::c_int & FORMAT_DHEIGHT as ::core::ffi::c_int != 0 {
        bufwrite16b(buf, pos_to_u16(v.d_height));
    }
}
pub unsafe extern "C" fn bk_gpos_value(
    mut v: PositionValue,
    mut format: u16,
) -> *mut BkBlock {
    let mut b: *mut BkBlock = bk_new_block(&[]);
    if format as ::core::ffi::c_int & FORMAT_DX as ::core::ffi::c_int != 0 {
        bk_push(b, &[bk_int(BkCellType::B16, (v.dx as i16 as ::core::ffi::c_int) as u32)]);
    }
    if format as ::core::ffi::c_int & FORMAT_DY as ::core::ffi::c_int != 0 {
        bk_push(b, &[bk_int(BkCellType::B16, (v.dy as i16 as ::core::ffi::c_int) as u32)]);
    }
    if format as ::core::ffi::c_int & FORMAT_DWIDTH as ::core::ffi::c_int != 0 {
        bk_push(b, &[bk_int(BkCellType::B16, (v.d_width as i16 as ::core::ffi::c_int) as u32)]);
    }
    if format as ::core::ffi::c_int & FORMAT_DHEIGHT as ::core::ffi::c_int != 0 {
        bk_push(b, &[bk_int(BkCellType::B16, (v.d_height as i16 as ::core::ffi::c_int) as u32)]);
    }
    return b;
}
