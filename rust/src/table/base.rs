#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::bkgraph::bk_build_block;
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::built_json::{
    BuiltValue, json_new_position, json_object_new, json_object_push, json_object_push_tag,
    json_string_new_length,
};
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_numof, json_obj_get_type, json_obj_getstr_share, json_obj_key_at,
    json_obj_len, json_obj_val_at, json_type_of,
};
use crate::support::primitives::{Pos, TableId};
use crate::vendor::json::JsonType;

#[derive(Copy, Clone)]
pub struct BaseValue {
    pub tag: u32,
    pub coordinate: Pos,
}
/// `base_values_count` is gone -- `base_values.len()` is always the same
/// number now that the array is a `Vec` instead of a `__caryll_allocate_
/// clean`'d buffer sized separately from what actually got filled.
pub struct BaseScriptEntry {
    pub tag: u32,
    pub default_baseline_tag: u32,
    pub base_values: Vec<BaseValue>,
}
/// `script_count` is gone the same way `base_values_count` is: `entries.
/// len()`. `axis_from_json` used to allocate at the JSON object's full
/// length, fill only the entries that passed a type check, then shrink
/// `script_count` down to how many actually landed -- a `Vec` built with
/// `.push()` only for entries that pass the check arrives at the same
/// final content directly, with no separate count to keep in sync.
pub struct BaseAxis {
    pub entries: Vec<BaseScriptEntry>,
}
pub struct BaseTable {
    pub horizontal: Option<Box<BaseAxis>>,
    pub vertical: Option<Box<BaseAxis>>,
}
// Stage 6-4 "Box化" finished: `horizontal`/`vertical` are `Option<Box<
// BaseAxis>>`, and `BaseAxis`'s own `entries: Vec<BaseScriptEntry>` (each
// entry's `base_values: Vec<BaseValue>`) means the whole tree is now
// ordinary owned Rust data -- no manual dispose function, no `Drop` impl
// on `BaseTable` at all, `Option`/`Box`/`Vec`'s own drop glue reaches
// every allocation on their own.
//
// This closes a documented pre-existing leak by construction, not by an
// explicit fix: the previous Box化 pass on this file (converting only
// `horizontal`/`vertical` themselves) left a comment recording that
// `delete_base_axis` never freed `axis` itself, only its `entries` --
// true in the original C too. A raw `*mut BaseAxis` freed via a hand-
// written dispose function can leak that way; a `Box<BaseAxis>` cannot
// -- there is no code path left where a `BaseAxis` allocation exists
// without something owning it. Same shape as the `otfccbuild.rs` binary
// entry point's use-after-free earlier in this migration: converting the
// ownership model made a bug stop being expressible, without this PR
// needing to hunt it down and patch it as a separate step.
// `items` was `__caryll_reallocate`'d one tag at a time by a hand-written
// "search, then grow-by-one-and-append" loop in `axis_to_bk` -- exactly
// `Vec::contains`/`Vec::push`. `size` duplicated `.len()` and is dropped.
pub struct BaseTagList {
    pub items: Vec<u32>,
}
fn read_base_value(data: &[u8], offset: usize) -> i16 {
    FontReader::new(data)
        .at(offset)
        .and_then(|mut r| {
            r.skip(2)?;
            r.i16()
        })
        .unwrap_or(0)
}
/// Returns `(default_baseline_tag, base_values)` instead of writing
/// through a `*mut BaseScriptEntry` out-param: every failure branch in
/// the original reset the entry's fields back to `(0, empty)` regardless
/// of what had been partially written along the way (`default_baseline_
/// tag`/`base_values_count` could be set non-zero by an intermediate
/// step before a later check failed and reset them), so the two
/// representations agree on every observable outcome -- this version
/// just never writes the intermediate values that were always going to
/// be thrown away.
///
/// `offset` is a plain `usize` (not the `u16` the on-disk `BaseValuesOffset`
/// field is), and every offset this function derives from it stays `usize`
/// too: the original computed `(base_values_offset as c_int + offset as
/// c_int) as u16`, adding in 32-bit `c_int` (safe -- both operands are
/// ≤ 65535) but then *truncating the sum back down to `u16`*, silently
/// wrapping whenever the real combined offset exceeded 65535. That's the
/// same "offset arithmetic wraps and defeats the length guard that follows
/// it" bug shape `otl/coverage.rs`'s `read_coverage` docs and
/// `table/cmap.rs`'s plan writeup both describe, just reached through a
/// narrowing cast instead of `wrapping_add`. Keeping every derived offset
/// as `usize` (max here: two `u16`s summed, nowhere near `usize::MAX`)
/// removes the wraparound outright instead of just moving where it hides.
fn read_base_script(
    data: &[u8],
    offset: usize,
    base_tag_list: &[u32],
    n_base_tags: u16,
) -> (u32, Vec<BaseValue>) {
    let Ok(mut r) = FontReader::new(data).at(offset) else {
        return (0, Vec::new());
    };
    let Ok(base_values_rel) = r.u16() else {
        return (0, Vec::new());
    };
    if base_values_rel == 0 {
        return (0, Vec::new());
    }
    let base_values_offset = offset + base_values_rel as usize;
    let Ok(mut r2) = FontReader::new(data).at(base_values_offset) else {
        return (0, Vec::new());
    };
    let Ok(default_index_raw) = r2.u16() else {
        return (0, Vec::new());
    };
    let default_index = (default_index_raw % n_base_tags) as usize;
    let default_baseline_tag: u32 = base_tag_list[default_index];
    let Ok(base_values_count) = r2.u16() else {
        return (0, Vec::new());
    };
    if base_values_count != n_base_tags {
        return (0, Vec::new());
    }
    if r2.require_room(base_values_count as usize, 2).is_err() {
        return (0, Vec::new());
    }
    let mut base_values: Vec<BaseValue> = Vec::with_capacity(base_values_count as usize);
    for j in 0..base_values_count {
        let tag = base_tag_list[j as usize];
        let val_offset = r2.u16().unwrap();
        let coordinate = if val_offset != 0 {
            read_base_value(data, base_values_offset + val_offset as usize) as Pos
        } else {
            0_i32 as Pos
        };
        base_values.push(BaseValue { tag, coordinate });
    }
    (default_baseline_tag, base_values)
}
/// Returns `None` on any of the format checks failing, `Some` otherwise
/// -- the original's fallthrough cleanup (`free(base_tag_list)` then
/// `delete_base_axis(axis)`) only ever ran with `axis` still null: every
/// path that allocates `axis` also fills it completely and returns
/// immediately, so `delete_base_axis(axis)` at the bottom was always a
/// no-op by the time it could run. `base_tag_list` is a local `Vec<u32>`
/// now, so it needs no explicit free on any exit path either.
///
/// `offset` and every offset derived from it stay `usize` for the same
/// reason `read_base_script` does -- the original's `(x as c_int + offset
/// as c_int) as u16` truncation could wrap a real out-of-range offset back
/// into range.
fn read_axis(data: &[u8], offset: usize) -> Option<Box<BaseAxis>> {
    let mut r = FontReader::new(data).at(offset).ok()?;
    let base_tag_list_rel = r.u16().ok()?;
    let base_script_list_rel = r.u16().ok()?;
    if base_tag_list_rel == 0 || base_script_list_rel == 0 {
        return None;
    }
    let base_tag_list_offset = offset + base_tag_list_rel as usize;
    let mut tl = FontReader::new(data).at(base_tag_list_offset).ok()?;
    let n_base_tags = tl.u16().ok()?;
    if n_base_tags == 0 {
        return None;
    }
    tl.require_room(n_base_tags as usize, 4).ok()?;
    let mut base_tag_list: Vec<u32> = Vec::with_capacity(n_base_tags as usize);
    for _ in 0..n_base_tags {
        base_tag_list.push(tl.u32().unwrap());
    }
    let base_script_list_offset = offset + base_script_list_rel as usize;
    let mut sl = FontReader::new(data).at(base_script_list_offset).ok()?;
    let n_base_scripts = sl.u16().ok()?;
    sl.require_room(n_base_scripts as usize, 6).ok()?;
    let mut entries: Vec<BaseScriptEntry> = Vec::with_capacity(n_base_scripts as usize);
    for _ in 0..n_base_scripts {
        let tag = sl.u32().unwrap();
        let base_script_rel = sl.u16().unwrap();
        if base_script_rel != 0 {
            let (default_baseline_tag, base_values) = read_base_script(
                data,
                base_script_list_offset + base_script_rel as usize,
                &base_tag_list,
                n_base_tags,
            );
            entries.push(BaseScriptEntry {
                tag,
                default_baseline_tag,
                base_values,
            });
        } else {
            entries.push(BaseScriptEntry {
                tag,
                default_baseline_tag: 0,
                base_values: Vec::new(),
            });
        }
    }
    Some(Box::new(BaseAxis { entries }))
}
fn parse_base(data: &[u8]) -> Result<(Option<Box<BaseAxis>>, Option<Box<BaseAxis>>), ReadError> {
    let mut r = FontReader::new(data);
    r.skip(4)?; // majorVersion(2) + minorVersion(2), unused
    let offset_h = r.u16()?;
    let offset_v = r.u16()?;
    let horizontal = (offset_h != 0)
        .then(|| read_axis(data, offset_h as usize))
        .flatten();
    let vertical = (offset_v != 0)
        .then(|| read_axis(data, offset_v as usize))
        .flatten();
    Ok((horizontal, vertical))
}
pub fn otfcc_read_base(packet: &Packet, options: &Options) -> Option<Box<BaseTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_BASE)?;
    let (horizontal, vertical) = match parse_base(&table.data) {
        Ok(parsed) => parsed,
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"Table 'BASE' Corrupted"),
            );
            return None;
        }
    };
    Some(Box::new(BaseTable {
        horizontal,
        vertical,
    }))
}
unsafe fn axis_to_json(axis: *const BaseAxis) -> *mut BuiltValue {
    let mut _axis: *mut BuiltValue = json_object_new((*axis).entries.len());
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*axis).entries.len() {
        let entry = &(&(*axis).entries)[j as usize];
        if entry.tag != 0 {
            let mut _entry: *mut BuiltValue = json_object_new(3_usize);
            if entry.default_baseline_tag != 0 {
                let mut tag: [::core::ffi::c_char; 4] = [0; 4];
                tag2str(
                    entry.default_baseline_tag,
                    &raw mut tag as *mut ::core::ffi::c_char,
                );
                json_object_push(
                    _entry,
                    b"defaultBaseline\0" as *const u8 as *const ::core::ffi::c_char,
                    json_string_new_length(
                        4 as ::core::ffi::c_uint,
                        &raw mut tag as *mut ::core::ffi::c_char,
                    ),
                );
            }
            let mut _values: *mut BuiltValue = json_object_new(entry.base_values.len());
            let mut k: TableId = 0 as TableId;
            while (k as usize) < entry.base_values.len() {
                let bv = &(&entry.base_values)[k as usize];
                if bv.tag != 0 {
                    json_object_push_tag(_values, bv.tag, json_new_position(bv.coordinate));
                }
                k = k.wrapping_add(1);
            }
            json_object_push(
                _entry,
                b"baselines\0" as *const u8 as *const ::core::ffi::c_char,
                _values,
            );
            json_object_push_tag(_axis, entry.tag, _entry);
        }
        j = j.wrapping_add(1);
    }
    return _axis;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_base(
    base: Option<&BaseTable>,
    root: *mut BuiltValue,
    options: &Options,
) {
    let base = match base {
        Some(b) => b as *const BaseTable,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"BASE"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _base: *mut BuiltValue = json_object_new(2_usize);
        if let Some(horizontal) = (*base).horizontal.as_deref() {
            json_object_push(
                _base,
                b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
                axis_to_json(horizontal),
            );
        }
        if let Some(vertical) = (*base).vertical.as_deref() {
            json_object_push(
                _base,
                b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
                axis_to_json(vertical),
            );
        }
        json_object_push(
            root,
            b"BASE\0" as *const u8 as *const ::core::ffi::c_char,
            _base,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
/// Returns `(default_baseline_tag, base_values)`, the JSON-side twin of
/// `read_base_script`.
///
/// Never a real FFI boundary -- internal call site only, same rationale
/// as every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
unsafe fn base_script_from_json(mut _sr: *const ParsedValue) -> (u32, Vec<BaseValue>) {
    let default_baseline_tag = str2tag(json_obj_getstr_share(
        _sr,
        b"defaultBaseline\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    let _basevalues: *const ParsedValue = json_obj_get_type(
        _sr,
        b"baselines\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _basevalues.is_null() {
        return (default_baseline_tag, Vec::new());
    }
    let base_values_count = json_obj_len(_basevalues);
    let mut base_values: Vec<BaseValue> = Vec::with_capacity(base_values_count as usize);
    let mut j: TableId = 0 as TableId;
    while (j as u32) < base_values_count {
        base_values.push(BaseValue {
            tag: str2tag(json_obj_key_at(_basevalues, j as u32)),
            coordinate: json_numof(json_obj_val_at(_basevalues, j as u32)) as Pos,
        });
        j = j.wrapping_add(1);
    }
    (default_baseline_tag, base_values)
}
/// `axis_from_json` builds `entries` with `.push()` only for the object-
/// typed values (matching the original's allocate-then-shrink-count
/// dance, but arriving at the same final content directly), then sorts
/// by tag -- stable, not `sort_unstable_by_key`, the same deliberately
/// conservative choice made for `Coverage`/`ClassDef`/`gpos_pair.rs`
/// since `qsort` itself gives no stability guarantee.
unsafe fn axis_from_json(mut _axis: *const ParsedValue) -> Option<Box<BaseAxis>> {
    if _axis.is_null() {
        return None;
    }
    let script_count = json_obj_len(_axis);
    let mut entries: Vec<BaseScriptEntry> = Vec::with_capacity(script_count as usize);
    let mut j: TableId = 0 as TableId;
    while (j as u32) < script_count {
        let script_val = json_obj_val_at(_axis, j as u32);
        if !script_val.is_null() && json_type_of(script_val) == JsonType::Object {
            let tag = str2tag(json_obj_key_at(_axis, j as u32));
            let (default_baseline_tag, base_values) = base_script_from_json(script_val);
            entries.push(BaseScriptEntry {
                tag,
                default_baseline_tag,
                base_values,
            });
        }
        j = j.wrapping_add(1);
    }
    entries.sort_by_key(|e| e.tag);
    Some(Box::new(BaseAxis { entries }))
}
pub unsafe fn otfcc_parse_base(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<BaseTable>> {
    let mut base: Option<Box<BaseTable>> = None;
    let table: *const ParsedValue;
    table = json_obj_get_type(
        root,
        b"BASE\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"BASE"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let horizontal = axis_from_json(json_obj_get_type(
                table,
                b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Object,
            ));
            let vertical = axis_from_json(json_obj_get_type(
                table,
                b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Object,
            ));
            base = Some(Box::new(BaseTable {
                horizontal,
                vertical,
            }));
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    return base;
}
pub unsafe fn axis_to_bk(axis: *const BaseAxis) -> *mut BkBlock {
    if axis.is_null() {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let mut taglist: BaseTagList = BaseTagList { items: Vec::new() };
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*axis).entries.len() {
        let entry: &BaseScriptEntry = &(&(*axis).entries)[j as usize];
        if entry.default_baseline_tag != 0 {
            if !taglist.items.contains(&entry.default_baseline_tag) {
                taglist.items.push(entry.default_baseline_tag);
            }
        }
        let mut k: TableId = 0 as TableId;
        while (k as usize) < entry.base_values.len() {
            let tag: u32 = (&entry.base_values)[k as usize].tag;
            if !taglist.items.contains(&tag) {
                taglist.items.push(tag);
            }
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    taglist.items.sort();
    let base_tag_list: *mut BkBlock = bk_new_block(&[bk_int(
        BkCellType::B16,
        (taglist.items.len() as i32) as u32,
    )]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < taglist.items.len() {
        bk_push(
            base_tag_list,
            &[bk_int(BkCellType::B32, taglist.items[j_0 as usize] as u32)],
        );
        j_0 = j_0.wrapping_add(1);
    }
    let base_script_list: *mut BkBlock = bk_new_block(&[bk_int(
        BkCellType::B16,
        ((*axis).entries.len() as i32) as u32,
    )]);
    let mut j_1: TableId = 0 as TableId;
    while (j_1 as usize) < (*axis).entries.len() {
        let entry_0: &BaseScriptEntry = &(&(*axis).entries)[j_1 as usize];
        let base_values: *mut BkBlock = bk_new_block(&[]);
        let mut default_index: TableId = 0 as TableId;
        let mut m: TableId = 0 as TableId;
        while (m as usize) < taglist.items.len() {
            if taglist.items[m as usize] == entry_0.default_baseline_tag {
                default_index = m;
                break;
            } else {
                m = m.wrapping_add(1);
            }
        }
        bk_push(
            base_values,
            &[bk_int(
                BkCellType::B16,
                (default_index as i32) as u32,
            )],
        );
        bk_push(
            base_values,
            &[bk_int(
                BkCellType::B16,
                (taglist.items.len() as i32) as u32,
            )],
        );
        let mut m_0: usize = 0_usize;
        while m_0 < taglist.items.len() {
            let mut found_1: bool = false;
            let mut found_index: TableId = 0 as TableId;
            let mut k_0: TableId = 0 as TableId;
            while (k_0 as usize) < entry_0.base_values.len() {
                if (&entry_0.base_values)[k_0 as usize].tag == taglist.items[m_0] {
                    found_1 = true;
                    found_index = k_0;
                    break;
                } else {
                    k_0 = k_0.wrapping_add(1);
                }
            }
            if found_1 {
                bk_push(
                    base_values,
                    &[bk_ptr(
                        BkCellType::P16,
                        bk_new_block(&[
                            bk_int(BkCellType::B16, 1_u32),
                            bk_int(
                                BkCellType::B16,
                                ((&entry_0.base_values)[found_index as usize].coordinate as i16
                                    as i32) as u32,
                            ),
                        ]),
                    )],
                );
            } else {
                bk_push(
                    base_values,
                    &[bk_ptr(
                        BkCellType::P16,
                        bk_new_block(&[
                            bk_int(BkCellType::B16, 1_u32),
                            bk_int(BkCellType::B16, 0_u32),
                        ]),
                    )],
                );
            }
            m_0 = m_0.wrapping_add(1);
        }
        let script_record: *mut BkBlock = bk_new_block(&[
            bk_ptr(BkCellType::P16, base_values),
            bk_ptr(BkCellType::P16, ::core::ptr::null_mut()),
            bk_int(BkCellType::B16, 0_u32),
        ]);
        bk_push(
            base_script_list,
            &[
                bk_int(BkCellType::B32, (entry_0.tag) as u32),
                bk_ptr(BkCellType::P16, script_record),
            ],
        );
        j_1 = j_1.wrapping_add(1);
    }
    return bk_new_block(&[
        bk_ptr(BkCellType::P16, base_tag_list),
        bk_ptr(BkCellType::P16, base_script_list),
    ]);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_base(base: Option<&BaseTable>) -> Option<Buffer> {
    let base = base? as *const BaseTable;
    let horizontal_bk = (*base)
        .horizontal
        .as_deref()
        .map_or(::core::ptr::null_mut(), |a| {
            axis_to_bk(a as *const BaseAxis)
        });
    let vertical_bk = (*base)
        .vertical
        .as_deref()
        .map_or(::core::ptr::null_mut(), |a| {
            axis_to_bk(a as *const BaseAxis)
        });
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(BkCellType::B32, 0x10000_u32),
        bk_ptr(BkCellType::P16, horizontal_bk),
        bk_ptr(BkCellType::P16, vertical_bk),
    ]);
    Some(bk_build_block(root))
}
#[inline]
unsafe fn tag2str(tag: u32, tags: *mut ::core::ffi::c_char) {
    *tags.offset(0_i32 as isize) =
        (tag >> 24_i32 & 0xff_u32) as ::core::ffi::c_char;
    *tags.offset(1_i32 as isize) =
        (tag >> 16_i32 & 0xff_u32) as ::core::ffi::c_char;
    *tags.offset(2_i32 as isize) =
        (tag >> 8_i32 & 0xff_u32) as ::core::ffi::c_char;
    *tags.offset(3_i32 as isize) = (tag & 0xff_u32) as ::core::ffi::c_char;
}
#[inline]
unsafe fn str2tag(mut tags: *const ::core::ffi::c_char) -> u32 {
    if tags.is_null() {
        return 0_u32;
    }
    let mut tag: u32 = 0_u32;
    let mut len: u8 = 0_u8;
    while *tags as i32 != 0 && (len as i32) < 4_i32
    {
        tag = tag << 8_i32 | *tags as u32;
        tags = tags.offset(1);
        len = len.wrapping_add(1);
    }
    while (len as i32) < 4_i32 {
        tag = tag << 8_i32 | ' ' as i32 as u32;
        len = len.wrapping_add(1);
    }
    return tag;
}

#[cfg(test)]
mod parse_base_tests {
    use super::*;

    const HANG: u32 = 0x68616e67; // "hang"

    // header(8) + axis(4) + tag list(6, one tag) + script list(8, one
    // record) + script table(2) + base values(6, one coord) + coord(4)
    fn well_formed_base_table() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0u16.to_be_bytes()); // majorVersion
        b.extend_from_slice(&1u16.to_be_bytes()); // minorVersion
        b.extend_from_slice(&8u16.to_be_bytes()); // HorizAxisOffset
        b.extend_from_slice(&0u16.to_be_bytes()); // VertAxisOffset (none)
        // Axis table @8
        b.extend_from_slice(&4u16.to_be_bytes()); // BaseTagListOffset (rel to 8)
        b.extend_from_slice(&10u16.to_be_bytes()); // BaseScriptListOffset (rel to 8)
        // BaseTagList @12
        b.extend_from_slice(&1u16.to_be_bytes()); // BaseTagCount
        b.extend_from_slice(&HANG.to_be_bytes());
        // BaseScriptList @18
        b.extend_from_slice(&1u16.to_be_bytes()); // BaseScriptCount
        b.extend_from_slice(&HANG.to_be_bytes()); // BaseScriptTag
        b.extend_from_slice(&8u16.to_be_bytes()); // BaseScriptOffset (rel to 18)
        // BaseScript table @26
        b.extend_from_slice(&2u16.to_be_bytes()); // BaseValuesOffset (rel to 26)
        // BaseValues table @28
        b.extend_from_slice(&0u16.to_be_bytes()); // DefaultIndex
        b.extend_from_slice(&1u16.to_be_bytes()); // BaseCoordCount
        b.extend_from_slice(&6u16.to_be_bytes()); // BaseCoordOffset[0] (rel to 28)
        // BaseCoord @34
        b.extend_from_slice(&1u16.to_be_bytes()); // format (unread, format-agnostic)
        b.extend_from_slice(&500i16.to_be_bytes()); // Coordinate
        b
    }

    #[test]
    fn well_formed_table_reads_the_horizontal_axis() {
        let data = well_formed_base_table();
        let (horizontal, vertical) = parse_base(&data).unwrap();
        assert!(vertical.is_none());
        let axis = horizontal.unwrap();
        assert_eq!(axis.entries.len(), 1);
        assert_eq!(axis.entries[0].tag, HANG);
        assert_eq!(axis.entries[0].default_baseline_tag, HANG);
        assert_eq!(axis.entries[0].base_values.len(), 1);
        assert_eq!(axis.entries[0].base_values[0].tag, HANG);
        assert_eq!(axis.entries[0].base_values[0].coordinate, 500.0);
    }

    #[test]
    fn truncated_header_errs_instead_of_reading_oob() {
        assert!(parse_base(&well_formed_base_table()[..6]).is_err());
    }

    #[test]
    fn zero_axis_offset_is_absent_not_an_error() {
        let mut data = well_formed_base_table();
        data[4..6].copy_from_slice(&0u16.to_be_bytes()); // HorizAxisOffset = 0
        let (horizontal, vertical) = parse_base(&data).unwrap();
        assert!(horizontal.is_none());
        assert!(vertical.is_none());
    }

    #[test]
    fn zero_base_tag_count_makes_the_axis_absent() {
        let mut data = well_formed_base_table();
        data[12..14].copy_from_slice(&0u16.to_be_bytes()); // BaseTagCount = 0
        let (horizontal, _) = parse_base(&data).unwrap();
        assert!(horizontal.is_none());
    }

    #[test]
    fn base_coord_count_mismatched_with_tag_count_is_rejected() {
        // BaseCoordCount (absolute offset 30) is 1 in the fixture, but
        // n_base_tags here is 2 -- must be rejected, not read with the
        // wrong count.
        let base_tag_list = vec![HANG, HANG];
        let data = well_formed_base_table();
        let (tag, values) = read_base_script(&data, 26, &base_tag_list, 2);
        assert_eq!(tag, 0);
        assert!(values.is_empty());
    }

    #[test]
    fn base_script_offset_sum_near_u16_boundary_does_not_wrap() {
        // The original computed `(base_values_offset as c_int + offset as
        // c_int) as u16` -- truncating the sum back into u16 range. With
        // `offset` = 60000 and a BaseValuesOffset field of 10000, the true
        // combined offset is 70000, but the old cast wrapped it down to
        // 70000 - 65536 = 4464. A well-formed BaseValues structure placed
        // only at the true offset (70000, left as zeros at the wrapped
        // address) must be read from there, not from the wrapped address.
        let base_tag_list = vec![HANG];
        let mut data = vec![0u8; 70010];
        data[60000..60002].copy_from_slice(&10000u16.to_be_bytes());
        data[70000..70002].copy_from_slice(&0u16.to_be_bytes()); // DefaultIndex
        data[70002..70004].copy_from_slice(&1u16.to_be_bytes()); // BaseCoordCount
        data[70004..70006].copy_from_slice(&6u16.to_be_bytes()); // BaseCoordOffset[0]
        data[70006..70008].copy_from_slice(&1u16.to_be_bytes()); // format
        data[70008..70010].copy_from_slice(&500i16.to_be_bytes()); // Coordinate
        let (default_baseline_tag, base_values) = read_base_script(&data, 60000, &base_tag_list, 1);
        assert_eq!(default_baseline_tag, HANG);
        assert_eq!(base_values.len(), 1);
        assert_eq!(base_values[0].coordinate, 500.0);
    }

    #[test]
    fn axis_tag_list_offset_sum_near_u16_boundary_does_not_wrap() {
        // Same wraparound shape as the BaseScript test above, but for
        // `read_axis`'s own `BaseTagListOffset`/`BaseScriptListOffset`
        // fields.
        let mut data = vec![0u8; 70020];
        data[60000..60002].copy_from_slice(&10000u16.to_be_bytes()); // BaseTagListOffset (rel)
        data[60002..60004].copy_from_slice(&10010u16.to_be_bytes()); // BaseScriptListOffset (rel)
        // BaseTagList @70000
        data[70000..70002].copy_from_slice(&1u16.to_be_bytes());
        data[70002..70006].copy_from_slice(&HANG.to_be_bytes());
        // BaseScriptList @70010
        data[70010..70012].copy_from_slice(&1u16.to_be_bytes());
        data[70012..70016].copy_from_slice(&HANG.to_be_bytes());
        data[70016..70018].copy_from_slice(&0u16.to_be_bytes()); // BaseScriptOffset = 0 (absent)
        let axis = read_axis(&data, 60000).unwrap();
        assert_eq!(axis.entries.len(), 1);
        assert_eq!(axis.entries[0].tag, HANG);
        assert_eq!(axis.entries[0].default_baseline_tag, 0);
        assert!(axis.entries[0].base_values.is_empty());
    }
}
