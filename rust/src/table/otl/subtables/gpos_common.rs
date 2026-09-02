#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::handle::{
    GlyphHandle, Handle, HandleState, handle_from_name, otfcc_handle_dup,
};
use crate::support::parsed_json::{
    ParsedValue, json_obj_get_type, json_obj_getnum, json_obj_getnum_fallback,
    json_obj_key_bytes_at, json_obj_len, json_obj_val_at, json_str_ptr, json_type_of,
};
use crate::table::otl::coverage::Coverage;

use crate::support::binio::pos_to_u16;
use crate::support::font_reader::FontReader;

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_push};
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::primitives::{FontFilePointer, GlyphClass, GlyphId, Pos};
use crate::table::otl::{Anchor, MarkArray, MarkRecord, PositionValue};
use crate::vendor::json::JsonType;
// `MarkRecord` holds only a `GlyphHandle` plus a plain `Anchor`, so dropping
// the `Vec` runs `Handle`'s own `Drop` for every entry -- no per-element
// dtor needed anymore.
pub(crate) unsafe fn dispose_mark_array(arr: *mut MarkArray) {
    *arr = Vec::new();
}
/// The original checked only that `MarkCount` itself (2 bytes at `offset`)
/// was in bounds, then read `mark_count` 4-byte records with no room check
/// at all -- a `mark_count` large enough to run past `table_length` read
/// straight off the end of the table. `require_room` closes that.
///
/// It also indexed `cov[j]` for every `j` up to `mark_count` with no check
/// against `cov`'s own length: `mark_count` (read from this MarkArray) and
/// `cov.len()` (the sibling Coverage table's glyph count) are two
/// independent, both attacker-controlled fields that a well-formed font
/// happens to keep equal but nothing here enforced -- a `MarkCount` larger
/// than the Coverage table's glyph count panicked on `Vec` index out of
/// bounds (a real, crafted-font-reachable crash, not a memory-safety bug
/// but still a DoS). Capping the loop at `cov.len()` too fixes it.
pub unsafe fn otl_read_mark_array(
    array: *mut MarkArray,
    cov: *mut Coverage,
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
) {
    let slice = ::core::slice::from_raw_parts(data as *const u8, table_length as usize);
    let Ok(mut r) = FontReader::new(slice).at(offset as usize) else {
        return;
    };
    let Ok(mark_count) = r.u16() else { return };
    if r.require_room(mark_count as usize, 4).is_err() {
        return;
    }
    let n = (mark_count as usize).min((*cov).len());
    for j in 0..n {
        let mark_class = r.u16().unwrap() as GlyphClass;
        let delta = r.u16().unwrap();
        let anchor = if delta != 0 {
            otl_read_anchor(data, table_length, offset.wrapping_add(delta as u32))
        } else {
            otl_anchor_absent()
        };
        (*array).push(MarkRecord {
            glyph: otfcc_handle_dup((&(*cov))[j].clone() as Handle) as GlyphHandle,
            mark_class,
            anchor,
        });
    }
}
pub unsafe fn otl_parse_mark_array(
    mut _marks: *const ParsedValue,
    array: *mut MarkArray,
    h: *mut std::collections::BTreeMap<Vec<u8>, GlyphClass>,
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
        let anchor_record: *const ParsedValue = json_obj_val_at(_marks, j as u32);
        mark.glyph = handle_from_name(Some(json_obj_key_bytes_at(_marks, j as u32))) as GlyphHandle;
        mark.mark_class = 0 as GlyphClass;
        mark.anchor = otl_anchor_absent();
        if anchor_record.is_null() || json_type_of(anchor_record) != JsonType::Object {
            (*array).push(mark);
        } else {
            let mut _class_name: *const ParsedValue = json_obj_get_type(
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
                    json_str_ptr(_class_name) as *const ::core::ffi::c_char
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
            let anchor_record_0: *const ParsedValue = json_obj_val_at(_marks, j_0 as u32);
            let mut _class_name_0: *const ParsedValue = json_obj_get_type(
                anchor_record_0,
                b"class\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            let class_name_0: Vec<u8> = ::core::ffi::CStr::from_ptr(
                json_str_ptr(_class_name_0) as *const ::core::ffi::c_char
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
pub unsafe fn otl_anchor_absent() -> Anchor {
    let anchor: Anchor = Anchor {
        present: false,
        x: 0_i32 as Pos,
        y: 0_i32 as Pos,
    };
    return anchor;
}
pub unsafe fn otl_read_anchor(data: FontFilePointer, table_length: u32, offset: u32) -> Anchor {
    let mut anchor: Anchor = Anchor {
        present: false,
        x: 0_i32 as Pos,
        y: 0_i32 as Pos,
    };
    let slice = ::core::slice::from_raw_parts(data as *const u8, table_length as usize);
    let Ok(bytes) =
        FontReader::new(slice).at(offset as usize).and_then(|mut r| r.bytes(6))
    else {
        return anchor;
    };
    anchor.present = true;
    anchor.x = i16::from_be_bytes([bytes[2], bytes[3]]) as Pos;
    anchor.y = i16::from_be_bytes([bytes[4], bytes[5]]) as Pos;
    anchor
}
pub fn otl_dump_anchor(a: Anchor) -> BuiltValue {
    if a.present {
        let mut v = BuiltValue::new_object(2);
        v.push_field(b"x", BuiltValue::position(a.x));
        v.push_field(b"y", BuiltValue::position(a.y));
        v
    } else {
        BuiltValue::Null
    }
}
pub unsafe fn otl_parse_anchor(v: *const ParsedValue) -> Anchor {
    let mut anchor: Anchor = Anchor {
        present: false,
        x: 0_i32 as Pos,
        y: 0_i32 as Pos,
    };
    if v.is_null() || json_type_of(v) != JsonType::Object {
        return anchor;
    }
    anchor.present = true;
    anchor.x = json_obj_getnum_fallback(
        v,
        b"x\0" as *const u8 as *const ::core::ffi::c_char,
        0_i32 as ::core::ffi::c_double,
    ) as Pos;
    anchor.y = json_obj_getnum_fallback(
        v,
        b"y\0" as *const u8 as *const ::core::ffi::c_char,
        0_i32 as ::core::ffi::c_double,
    ) as Pos;
    return anchor;
}
pub unsafe fn bk_from_anchor(a: Anchor) -> *mut BkBlock {
    if !a.present {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    return bk_new_block(&[
        bk_int(BkCellType::B16, 1_u32),
        bk_int(BkCellType::B16, (a.x as i16 as i32) as u32),
        bk_int(BkCellType::B16, (a.y as i16 as i32) as u32),
    ]);
}
pub static FORMAT_DX: u8 = 1_u8;
pub static FORMAT_DY: u8 = 2_u8;
pub static FORMAT_DWIDTH: u8 = 4_u8;
pub static FORMAT_DHEIGHT: u8 = 8_u8;
pub static BITS_IN: [u8; 256] = [
    0_i32 as u8,
    1_i32 as u8,
    1_i32 as u8,
    2_i32 as u8,
    1_i32 as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    1_i32 as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    2_i32 as u8,
    (2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32) as u8,
    1_i32 as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32) as u8,
    1_i32 as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32) as u8,
    2_i32 as u8,
    (2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 2_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 2_i32) as u8,
    1_i32 as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    ((1_i32
        + 2_i32)
        + 1_i32) as u8,
    ((1_i32
        + 2_i32)
        + 1_i32) as u8,
    ((1_i32
        + 2_i32)
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32
        + 2_i32) as u8,
    1_i32 as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 1_i32) as u8,
    ((1_i32
        + 1_i32)
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 1_i32
        + 2_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32) as u8,
    ((1_i32
        + 2_i32)
        + 1_i32) as u8,
    ((1_i32
        + 2_i32)
        + 1_i32) as u8,
    ((1_i32
        + 2_i32)
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 1_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32
        + 1_i32) as u8,
    (1_i32
        + 2_i32
        + 2_i32
        + 2_i32) as u8,
    2_i32 as u8,
    (2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 2_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32) as u8,
    ((2_i32
        + 1_i32)
        + 1_i32) as u8,
    ((2_i32
        + 1_i32)
        + 1_i32) as u8,
    ((2_i32
        + 1_i32)
        + 2_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32) as u8,
    ((2_i32
        + 1_i32)
        + 1_i32) as u8,
    ((2_i32
        + 1_i32)
        + 1_i32) as u8,
    ((2_i32
        + 1_i32)
        + 2_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 1_i32
        + 2_i32
        + 2_i32) as u8,
    (2_i32
        + 2_i32) as u8,
    ((2_i32
        + 2_i32)
        + 1_i32) as u8,
    ((2_i32
        + 2_i32)
        + 1_i32) as u8,
    ((2_i32
        + 2_i32)
        + 2_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 1_i32
        + 2_i32) as u8,
    (2_i32
        + 2_i32
        + 2_i32) as u8,
    (2_i32
        + 2_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 2_i32
        + 1_i32) as u8,
    (2_i32
        + 2_i32
        + 2_i32
        + 2_i32) as u8,
];
pub unsafe fn position_format_length(format: u16) -> u8 {
    return ((BITS_IN[(format as i32 & 0xff_i32) as usize]
        as i32)
        << 1_i32) as u8;
}
pub unsafe fn position_zero() -> PositionValue {
    let v: PositionValue = PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        d_width: 0.0f64,
        d_height: 0.0f64,
    };
    return v;
}
pub unsafe fn read_gpos_value(
    data: FontFilePointer,
    table_length: u32,
    offset: u32,
    format: u16,
) -> PositionValue {
    let mut v: PositionValue = PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        d_width: 0.0f64,
        d_height: 0.0f64,
    };
    let slice = ::core::slice::from_raw_parts(data as *const u8, table_length as usize);
    let len = position_format_length(format) as usize;
    let Ok(bytes) =
        FontReader::new(slice).at(offset as usize).and_then(|mut r| r.bytes(len))
    else {
        return v;
    };
    let mut pos = 0;
    if format & FORMAT_DX as u16 != 0 {
        v.dx = i16::from_be_bytes([bytes[pos], bytes[pos + 1]]) as Pos;
        pos += 2;
    }
    if format & FORMAT_DY as u16 != 0 {
        v.dy = i16::from_be_bytes([bytes[pos], bytes[pos + 1]]) as Pos;
        pos += 2;
    }
    if format & FORMAT_DWIDTH as u16 != 0 {
        v.d_width = i16::from_be_bytes([bytes[pos], bytes[pos + 1]]) as Pos;
        pos += 2;
    }
    if format & FORMAT_DHEIGHT as u16 != 0 {
        v.d_height = i16::from_be_bytes([bytes[pos], bytes[pos + 1]]) as Pos;
    }
    v
}
pub fn gpos_dump_value(value: PositionValue) -> BuiltValue {
    let mut v = BuiltValue::new_object(4);
    if value.dx != 0. {
        v.push_field(b"dx", BuiltValue::position(value.dx));
    }
    if value.dy != 0. {
        v.push_field(b"dy", BuiltValue::position(value.dy));
    }
    if value.d_width != 0. {
        v.push_field(b"dWidth", BuiltValue::position(value.d_width));
    }
    if value.d_height != 0. {
        v.push_field(b"dHeight", BuiltValue::position(value.d_height));
    }
    v.preserialize()
}
pub unsafe fn gpos_parse_value(pos: *const ParsedValue) -> PositionValue {
    let mut v: PositionValue = PositionValue {
        dx: 0.0f64,
        dy: 0.0f64,
        d_width: 0.0f64,
        d_height: 0.0f64,
    };
    if pos.is_null() || json_type_of(pos) != JsonType::Object {
        return v;
    }
    v.dx = json_obj_getnum(pos, b"dx\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
    v.dy = json_obj_getnum(pos, b"dy\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
    v.d_width = json_obj_getnum(pos, b"dWidth\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
    v.d_height =
        json_obj_getnum(pos, b"dHeight\0" as *const u8 as *const ::core::ffi::c_char) as Pos;
    return v;
}
pub unsafe fn required_position_format(v: PositionValue) -> u8 {
    return ((if v.dx != 0. {
        FORMAT_DX as i32
    } else {
        0_i32
    }) | (if v.dy != 0. {
        FORMAT_DY as i32
    } else {
        0_i32
    }) | (if v.d_width != 0. {
        FORMAT_DWIDTH as i32
    } else {
        0_i32
    }) | (if v.d_height != 0. {
        FORMAT_DHEIGHT as i32
    } else {
        0_i32
    })) as u8;
}
pub fn write_gpos_value(buf: &mut Buffer, v: PositionValue, format: u16) {
    if format as i32 & FORMAT_DX as i32 != 0 {
        buf.write_u16be(pos_to_u16(v.dx));
    }
    if format as i32 & FORMAT_DY as i32 != 0 {
        buf.write_u16be(pos_to_u16(v.dy));
    }
    if format as i32 & FORMAT_DWIDTH as i32 != 0 {
        buf.write_u16be(pos_to_u16(v.d_width));
    }
    if format as i32 & FORMAT_DHEIGHT as i32 != 0 {
        buf.write_u16be(pos_to_u16(v.d_height));
    }
}
pub unsafe fn bk_gpos_value(v: PositionValue, format: u16) -> *mut BkBlock {
    let b: *mut BkBlock = bk_new_block(&[]);
    if format as i32 & FORMAT_DX as i32 != 0 {
        bk_push(
            b,
            &[bk_int(
                BkCellType::B16,
                (v.dx as i16 as i32) as u32,
            )],
        );
    }
    if format as i32 & FORMAT_DY as i32 != 0 {
        bk_push(
            b,
            &[bk_int(
                BkCellType::B16,
                (v.dy as i16 as i32) as u32,
            )],
        );
    }
    if format as i32 & FORMAT_DWIDTH as i32 != 0 {
        bk_push(
            b,
            &[bk_int(
                BkCellType::B16,
                (v.d_width as i16 as i32) as u32,
            )],
        );
    }
    if format as i32 & FORMAT_DHEIGHT as i32 != 0 {
        bk_push(
            b,
            &[bk_int(
                BkCellType::B16,
                (v.d_height as i16 as i32) as u32,
            )],
        );
    }
    return b;
}

#[cfg(test)]
mod read_anchor_and_value_tests {
    use super::*;
    use crate::support::handle::handle_from_index;
    use crate::table::otl::coverage::{otl_coverage_create, otl_coverage_free, push_to_coverage};

    #[test]
    fn otl_read_anchor_well_formed_reads_x_and_y() {
        let mut data = vec![0u8; 6];
        data[2..4].copy_from_slice(&100i16.to_be_bytes());
        data[4..6].copy_from_slice(&(-50i16).to_be_bytes());
        let anchor = unsafe { otl_read_anchor(data.as_mut_ptr(), data.len() as u32, 0) };
        assert!(anchor.present);
        assert_eq!(anchor.x, 100.0);
        assert_eq!(anchor.y, -50.0);
    }

    #[test]
    fn otl_read_anchor_truncated_is_absent_not_oob() {
        let mut data = vec![0u8; 4];
        let anchor = unsafe { otl_read_anchor(data.as_mut_ptr(), data.len() as u32, 0) };
        assert!(!anchor.present);
    }

    #[test]
    fn read_gpos_value_reads_only_the_fields_the_format_selects() {
        let format = FORMAT_DX as u16 | FORMAT_DY as u16;
        let mut data = vec![0u8; 4];
        data[0..2].copy_from_slice(&10i16.to_be_bytes());
        data[2..4].copy_from_slice(&20i16.to_be_bytes());
        let v = unsafe { read_gpos_value(data.as_mut_ptr(), data.len() as u32, 0, format) };
        assert_eq!((v.dx, v.dy, v.d_width, v.d_height), (10.0, 20.0, 0.0, 0.0));
    }

    #[test]
    fn read_gpos_value_truncated_is_zero_not_oob() {
        let format = FORMAT_DX as u16 | FORMAT_DY as u16;
        let mut data = vec![0u8; 2]; // needs 4 bytes for dx+dy, only 2 present
        let v = unsafe { read_gpos_value(data.as_mut_ptr(), data.len() as u32, 0, format) };
        assert_eq!((v.dx, v.dy), (0.0, 0.0));
    }

    unsafe fn coverage_of(gids: &[GlyphId]) -> *mut Coverage {
        let cov = otl_coverage_create();
        for &gid in gids {
            push_to_coverage(cov, handle_from_index(gid) as GlyphHandle);
        }
        cov
    }

    #[test]
    fn otl_read_mark_array_reads_one_record_per_covered_glyph() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // MarkCount
        data.extend_from_slice(&2u16.to_be_bytes()); // Class
        data.extend_from_slice(&6u16.to_be_bytes()); // MarkAnchorOffset (rel to 0)
        data.extend_from_slice(&1u16.to_be_bytes()); // Anchor format (unread)
        data.extend_from_slice(&100i16.to_be_bytes()); // x
        data.extend_from_slice(&(-30i16).to_be_bytes()); // y
        let cov = unsafe { coverage_of(&[5]) };
        let mut array: MarkArray = Vec::new();
        unsafe {
            otl_read_mark_array(&raw mut array, cov, data.as_mut_ptr(), data.len() as u32, 0);
            otl_coverage_free(cov);
        }
        assert_eq!(array.len(), 1);
        assert_eq!(array[0].glyph.index, 5);
        assert_eq!(array[0].mark_class, 2);
        assert!(array[0].anchor.present);
        assert_eq!(array[0].anchor.x, 100.0);
        assert_eq!(array[0].anchor.y, -30.0);
    }

    #[test]
    fn otl_read_mark_array_zero_delta_is_an_absent_anchor() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // MarkCount
        data.extend_from_slice(&0u16.to_be_bytes()); // Class
        data.extend_from_slice(&0u16.to_be_bytes()); // MarkAnchorOffset = 0
        let cov = unsafe { coverage_of(&[5]) };
        let mut array: MarkArray = Vec::new();
        unsafe {
            otl_read_mark_array(&raw mut array, cov, data.as_mut_ptr(), data.len() as u32, 0);
            otl_coverage_free(cov);
        }
        assert!(!array[0].anchor.present);
    }

    #[test]
    fn otl_read_mark_array_count_larger_than_coverage_is_capped_not_a_panic() {
        // MarkCount (2) claiming more marks than the sibling Coverage
        // table actually lists (1 glyph) used to index the Coverage
        // `Vec` out of bounds and panic -- must clamp to `cov.len()`
        // instead.
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_be_bytes()); // MarkCount
        data.extend_from_slice(&0u16.to_be_bytes()); // record[0].Class
        data.extend_from_slice(&0u16.to_be_bytes()); // record[0].MarkAnchorOffset
        data.extend_from_slice(&0u16.to_be_bytes()); // record[1].Class
        data.extend_from_slice(&0u16.to_be_bytes()); // record[1].MarkAnchorOffset
        let cov = unsafe { coverage_of(&[5]) };
        let mut array: MarkArray = Vec::new();
        unsafe {
            otl_read_mark_array(&raw mut array, cov, data.as_mut_ptr(), data.len() as u32, 0);
            otl_coverage_free(cov);
        }
        assert_eq!(array.len(), 1);
    }

    #[test]
    fn otl_read_mark_array_count_larger_than_buffer_is_rejected_not_read_oob() {
        // The original had no room check at all for the `mark_count`
        // 4-byte records -- a `MarkCount` this large against a 2-byte
        // buffer used to read straight off the end.
        let mut data = 1000u16.to_be_bytes().to_vec();
        let cov = unsafe { coverage_of(&[5]) };
        let mut array: MarkArray = Vec::new();
        unsafe {
            otl_read_mark_array(&raw mut array, cov, data.as_mut_ptr(), data.len() as u32, 0);
            otl_coverage_free(cov);
        }
        assert!(array.is_empty());
    }
}
