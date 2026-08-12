#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::json_funcs::{
    json_new_position, json_obj_get_type, json_obj_getnum, json_obj_getnum_fallback,
    json_obj_key_at, json_obj_key_len_at, json_obj_len, json_obj_val_at, json_str_ptr,
    preserialize,
};
use crate::table::otl::coverage::{Coverage};
use crate::support::handle::{handle_from_name, otfcc_handle_dup, Handle, GlyphHandle, HandleState};

use crate::support::binio::{pos_to_u16, read_16u, read_16s};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId, Pos};
use crate::vendor::json::{JsonType, JsonValue};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_push};
use crate::table::otl::{Anchor, MarkArray, MarkRecord, PositionValue};
use crate::support::buffer::{bufwrite16b};
use crate::vendor::json_builder::{json_null_new, json_object_new, json_object_push};
use crate::vendor::sds::{sdsnewlen};
// `MarkRecord` holds only a `GlyphHandle` plus a plain `Anchor`, so dropping
// the `Vec` runs `Handle`'s own `Drop` for every entry -- no per-element
// dtor needed anymore.
pub(crate) unsafe fn dispose_mark_array(arr: *mut MarkArray) {
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
                            (&(*cov))[j as usize].clone() as Handle,
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
                            (&(*cov))[j as usize].clone() as Handle,
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
pub unsafe extern "C" fn otl_parse_mark_array(
    mut _marks: *mut JsonValue,
    mut array: *mut MarkArray,
    mut h: *mut std::collections::BTreeMap<Vec<u8>, GlyphClass>,
    mut _options: *const Options,
) {
    let mut j: GlyphId = 0 as GlyphId;
    while (j as ::core::ffi::c_uint) < json_obj_len(_marks) {
        let mut mark: MarkRecord = MarkRecord {
            glyph: Handle {
                state: HandleState::Empty,
                index: 0,
                name: Vec::new(),
            },
            mark_class: 0,
            anchor: Anchor {
                present: false,
                x: 0.,
                y: 0.,
            },
        };
        let mut gname: *mut ::core::ffi::c_char = json_obj_key_at(_marks, j as u32);
        let mut anchor_record: *mut JsonValue = json_obj_val_at(_marks, j as u32);
        mark.glyph = handle_from_name(sdsnewlen(
            gname as *const ::core::ffi::c_void,
            json_obj_key_len_at(_marks, j as u32) as usize,
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
                // Deduplicates by class name, `strlen`-bounded (matching
                // the original's Bob Jenkins hash + `memcmp`, both driven
                // by `strlen`, not the JSON string's own `.length`) --
                // see `CffSidEntry` (rust/README.md) for why this
                // distinction is preserved rather than simplified away.
                // The id registered here is a placeholder, overwritten
                // below once every distinct class name is known and can
                // be renumbered in alphabetical order -- the original's
                // insert-time id (`HASH_COUNT` at insert time) is
                // equally provisional, later replaced by a
                // `HASH_SORT`-driven renumbering pass.
                let class_name: Vec<u8> = ::core::ffi::CStr::from_ptr(
                    json_str_ptr(_class_name) as *const ::core::ffi::c_char,
                )
                .to_bytes()
                .to_vec();
                (*h).entry(class_name).or_insert(0 as GlyphClass);
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
    // The original's `HASH_SORT`-with-`compare_class_hash` step sorts
    // hash nodes alphabetically by class name (`strcmp`), then renumbers
    // `class_id` sequentially over that order. `BTreeMap<Vec<u8>, _>`
    // already iterates in that same byte-wise-ascending order (matching
    // `strcmp` exactly on NUL-free byte sequences), so no separate sort
    // step is needed here -- just walk the already-sorted map and
    // replace each placeholder id with its final, alphabetical-rank one.
    for (rank, id) in (*h).values_mut().enumerate() {
        *id = rank as GlyphClass;
    }
    // Marks were pushed above with `mark_class` left at its placeholder;
    // re-walk them, re-deriving each one's class name from the same JSON
    // data the first pass read (mirroring the original's second
    // traversal -- marks don't carry their own class-name string, only
    // the resolved id) and looking up its now-final id. Every entry with
    // `.anchor.present` is guaranteed to have registered its class name
    // in the loop above, so (as in the original) there is no null/absent
    // check here before re-deriving it.
    let mut j_0: GlyphId = 0 as GlyphId;
    while (j_0 as usize) < (*array).len() {
        if (&(*array))[j_0 as usize].anchor.present {
            let mut anchor_record_0: *mut JsonValue = json_obj_val_at(_marks, j_0 as u32);
            let mut _class_name_0: *mut JsonValue = json_obj_get_type(
                anchor_record_0,
                b"class\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            let class_name_0: Vec<u8> = ::core::ffi::CStr::from_ptr(
                json_str_ptr(_class_name_0) as *const ::core::ffi::c_char,
            )
            .to_bytes()
            .to_vec();
            (&mut (*array))[j_0 as usize].mark_class = match (*h).get(&class_name_0) {
                Some(&id) => id,
                None => 0 as GlyphClass,
            };
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
