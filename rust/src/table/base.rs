#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::parsed_json::{ParsedValue, json_numof, json_obj_get_type, json_obj_getstr_share, json_obj_key_at, json_obj_len, json_obj_val_at, json_type_of};
use crate::support::binio::{read_16u, read_16s, read_32u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, Pos, TableId};
use crate::vendor::json::{JsonType};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::bk::bkgraph::{bk_build_block};
use crate::support::built_json::{BuiltValue, json_object_new, json_object_push, json_string_new_length, json_new_position, json_object_push_tag};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseValue {
    pub tag: u32,
    pub coordinate: Pos,
}
/// `base_values_count` is gone -- `base_values.len()` is always the same
/// number now that the array is a `Vec` instead of a `__caryll_allocate_
/// clean`'d buffer sized separately from what actually got filled.
#[repr(C)]
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
#[repr(C)]
pub struct BaseAxis {
    pub entries: Vec<BaseScriptEntry>,
}
#[repr(C)]
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
#[repr(C)]
pub struct BaseTagList {
    pub items: Vec<u32>,
}
unsafe fn read_base_value(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u16,
) -> i16 {
    if table_length < (offset as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as u32 {
        return 0 as i16;
    } else {
        return read_16s(
            data.offset(offset as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        );
    };
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
/// Never a real FFI boundary -- internal call site only, same rationale
/// as every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
unsafe fn read_base_script(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u16,
    base_tag_list: &[u32],
    mut n_base_tags: u16,
) -> (u32, Vec<BaseValue>) {
    if table_length < (offset as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u32 {
        return (0, Vec::new());
    }
    let mut base_values_offset: u16 =
        read_16u(data.offset(offset as ::core::ffi::c_int as isize) as *const u8);
    if base_values_offset == 0 {
        return (0, Vec::new());
    }
    base_values_offset =
        (base_values_offset as ::core::ffi::c_int + offset as ::core::ffi::c_int) as u16;
    if table_length < (base_values_offset as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as u32 {
        return (0, Vec::new());
    }
    let default_index: u16 = (read_16u(
        data.offset(base_values_offset as ::core::ffi::c_int as isize) as *const u8,
    ) as ::core::ffi::c_int
        % n_base_tags as ::core::ffi::c_int) as u16;
    let default_baseline_tag: u32 = base_tag_list[default_index as usize];
    let base_values_count: TableId = read_16u(
        data.offset(base_values_offset as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as *const u8,
    ) as TableId;
    if base_values_count as ::core::ffi::c_int != n_base_tags as ::core::ffi::c_int {
        return (0, Vec::new());
    }
    if table_length
        < (base_values_offset as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * base_values_count as ::core::ffi::c_int) as u32
    {
        return (0, Vec::new());
    }
    let mut base_values: Vec<BaseValue> = Vec::with_capacity(base_values_count as usize);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < base_values_count as ::core::ffi::c_int {
        let tag = base_tag_list[j as usize];
        let _val_offset: u16 = read_16u(
            data.offset(base_values_offset as ::core::ffi::c_int as isize)
                .offset(4 as ::core::ffi::c_int as isize)
                .offset((2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize)
                as *const u8,
        );
        let coordinate = if _val_offset != 0 {
            read_base_value(
                data,
                table_length,
                (base_values_offset as ::core::ffi::c_int + _val_offset as ::core::ffi::c_int)
                    as u16,
            ) as Pos
        } else {
            0 as ::core::ffi::c_int as Pos
        };
        base_values.push(BaseValue { tag, coordinate });
        j = j.wrapping_add(1);
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
unsafe fn read_axis(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u16,
) -> Option<Box<BaseAxis>> {
    if table_length < (offset as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as u32 {
        return None;
    }
    let base_tag_list_offset: u16 = (offset as ::core::ffi::c_int
        + read_16u(data.offset(offset as ::core::ffi::c_int as isize) as *const u8)
            as ::core::ffi::c_int) as u16;
    if base_tag_list_offset as ::core::ffi::c_int <= offset as ::core::ffi::c_int {
        return None;
    }
    if table_length < (base_tag_list_offset as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u32
    {
        return None;
    }
    let n_base_tags: u16 =
        read_16u(data.offset(base_tag_list_offset as ::core::ffi::c_int as isize) as *const u8);
    if n_base_tags == 0 {
        return None;
    }
    if table_length
        < (base_tag_list_offset as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * n_base_tags as ::core::ffi::c_int) as u32
    {
        return None;
    }
    let mut base_tag_list: Vec<u32> = Vec::with_capacity(n_base_tags as usize);
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < n_base_tags as ::core::ffi::c_int {
        base_tag_list.push(read_32u(
            data.offset(base_tag_list_offset as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset((j as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                as *const u8,
        ));
        j = j.wrapping_add(1);
    }
    let base_script_list_offset: u16 = (offset as ::core::ffi::c_int
        + read_16u(
            data.offset(offset as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as ::core::ffi::c_int) as u16;
    if base_script_list_offset as ::core::ffi::c_int <= offset as ::core::ffi::c_int {
        return None;
    }
    if table_length
        < (base_script_list_offset as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u32
    {
        return None;
    }
    let n_base_scripts: TableId = read_16u(
        data.offset(base_script_list_offset as ::core::ffi::c_int as isize) as *const u8,
    ) as TableId;
    if table_length
        < (base_script_list_offset as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * n_base_scripts as ::core::ffi::c_int) as u32
    {
        return None;
    }
    let mut entries: Vec<BaseScriptEntry> = Vec::with_capacity(n_base_scripts as usize);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < n_base_scripts as ::core::ffi::c_int {
        let tag = read_32u(
            data.offset(base_script_list_offset as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize)
                as *const u8,
        );
        let base_script_offset: u16 = read_16u(
            data.offset(base_script_list_offset as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset((6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        );
        if base_script_offset != 0 {
            let (default_baseline_tag, base_values) = read_base_script(
                data,
                table_length,
                (base_script_list_offset as ::core::ffi::c_int
                    + base_script_offset as ::core::ffi::c_int) as u16,
                &base_tag_list,
                n_base_tags,
            );
            entries.push(BaseScriptEntry { tag, default_baseline_tag, base_values });
        } else {
            entries.push(BaseScriptEntry { tag, default_baseline_tag: 0, base_values: Vec::new() });
        }
        j_0 = j_0.wrapping_add(1);
    }
    Some(Box::new(BaseAxis { entries }))
}
pub unsafe fn otfcc_read_base(
    packet: &Packet,
    mut options: *const Options,
) -> Option<Box<BaseTable>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_BASE {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut offset_h: u16 = 0;
                    let mut offset_v: u16 = 0;
                    let mut data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    let mut table_length: u32 = table.length;
                    if table_length < 8 as u32 {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"Table 'BASE' Corrupted"),
                        );
                    } else {
                        let mut horizontal: Option<Box<BaseAxis>> = None;
                        let mut vertical: Option<Box<BaseAxis>> = None;
                        offset_h = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if offset_h != 0 {
                            horizontal = read_axis(data, table_length, offset_h);
                        }
                        offset_v = read_16u(
                            data.offset(6 as ::core::ffi::c_int as isize) as *const u8
                        );
                        if offset_v != 0 {
                            vertical = read_axis(data, table_length, offset_v);
                        }
                        return Some(Box::new(BaseTable { horizontal, vertical }));
                    }
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
unsafe fn axis_to_json(mut axis: *const BaseAxis) -> *mut BuiltValue {
    let mut _axis: *mut BuiltValue = json_object_new((*axis).entries.len());
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*axis).entries.len() {
        let entry = &(&(*axis).entries)[j as usize];
        if entry.tag != 0 {
            let mut _entry: *mut BuiltValue = json_object_new(3 as usize);
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
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let base = match base {
        Some(b) => b as *const BaseTable,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::bytesbuild!(b"BASE"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _base: *mut BuiltValue = json_object_new(2 as usize);
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
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
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
            entries.push(BaseScriptEntry { tag, default_baseline_tag, base_values });
        }
        j = j.wrapping_add(1);
    }
    entries.sort_by_key(|e| e.tag);
    Some(Box::new(BaseAxis { entries }))
}
pub unsafe fn otfcc_parse_base(
    mut root: *const ParsedValue,
    mut options: *const Options,
) -> Option<Box<BaseTable>> {
    let mut base: Option<Box<BaseTable>> = None;
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get_type(
        root,
        b"BASE\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
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
            base = Some(Box::new(BaseTable { horizontal, vertical }));
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return base;
}
pub unsafe fn axis_to_bk(mut axis: *const BaseAxis) -> *mut BkBlock {
    if axis.is_null() {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let mut taglist: BaseTagList = BaseTagList {
        items: Vec::new(),
    };
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
    let mut base_tag_list: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (taglist.items.len() as ::core::ffi::c_int) as u32)]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < taglist.items.len() {
        bk_push(base_tag_list, &[bk_int(BkCellType::B32, taglist.items[j_0 as usize] as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    let mut base_script_list: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*axis).entries.len() as ::core::ffi::c_int) as u32)]);
    let mut j_1: TableId = 0 as TableId;
    while (j_1 as usize) < (*axis).entries.len() {
        let entry_0: &BaseScriptEntry = &(&(*axis).entries)[j_1 as usize];
        let mut base_values: *mut BkBlock = bk_new_block(&[]);
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
        bk_push(base_values, &[bk_int(BkCellType::B16, (default_index as ::core::ffi::c_int) as u32)]);
        bk_push(base_values, &[bk_int(BkCellType::B16, (taglist.items.len() as ::core::ffi::c_int) as u32)]);
        let mut m_0: usize = 0 as usize;
        while m_0 < taglist.items.len() {
            let mut found_1: bool = false;
            let mut found_index: TableId = 0 as TableId;
            let mut k_0: TableId = 0 as TableId;
            while (k_0 as usize) < entry_0.base_values.len() {
                if (&entry_0.base_values)[k_0 as usize].tag == taglist.items[m_0 as usize]
                {
                    found_1 = true;
                    found_index = k_0;
                    break;
                } else {
                    k_0 = k_0.wrapping_add(1);
                }
            }
            if found_1 {
                bk_push(base_values, &[bk_ptr(BkCellType::P16, bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_int(BkCellType::B16, ((&entry_0.base_values)[found_index as usize].coordinate as i16
                            as ::core::ffi::c_int) as u32)]))]);
            } else {
                bk_push(base_values, &[bk_ptr(BkCellType::P16, bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_int(BkCellType::B16, 0 as u32)]))]);
            }
            m_0 = m_0.wrapping_add(1);
        }
        let mut script_record: *mut BkBlock = bk_new_block(&[bk_ptr(BkCellType::P16, base_values), bk_ptr(BkCellType::P16, ::core::ptr::null_mut()), bk_int(BkCellType::B16, 0 as u32)]);
        bk_push(base_script_list, &[bk_int(BkCellType::B32, (entry_0.tag) as u32), bk_ptr(BkCellType::P16, script_record)]);
        j_1 = j_1.wrapping_add(1);
    }
    return bk_new_block(&[bk_ptr(BkCellType::P16, base_tag_list), bk_ptr(BkCellType::P16, base_script_list)]);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_base(
    base: Option<&BaseTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let base = match base {
        Some(b) => b as *const BaseTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let horizontal_bk = (*base)
        .horizontal
        .as_deref()
        .map_or(::core::ptr::null_mut(), |a| axis_to_bk(a as *const BaseAxis));
    let vertical_bk = (*base)
        .vertical
        .as_deref()
        .map_or(::core::ptr::null_mut(), |a| axis_to_bk(a as *const BaseAxis));
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B32, 0x10000 as u32), bk_ptr(BkCellType::P16, horizontal_bk), bk_ptr(BkCellType::P16, vertical_bk)]);
    return bk_build_block(root);
}
#[inline]
unsafe fn tag2str(mut tag: u32, mut tags: *mut ::core::ffi::c_char) {
    *tags.offset(0 as ::core::ffi::c_int as isize) =
        (tag >> 24 as ::core::ffi::c_int & 0xff as u32) as ::core::ffi::c_char;
    *tags.offset(1 as ::core::ffi::c_int as isize) =
        (tag >> 16 as ::core::ffi::c_int & 0xff as u32) as ::core::ffi::c_char;
    *tags.offset(2 as ::core::ffi::c_int as isize) =
        (tag >> 8 as ::core::ffi::c_int & 0xff as u32) as ::core::ffi::c_char;
    *tags.offset(3 as ::core::ffi::c_int as isize) =
        (tag & 0xff as u32) as ::core::ffi::c_char;
}
#[inline]
unsafe fn str2tag(mut tags: *const ::core::ffi::c_char) -> u32 {
    if tags.is_null() {
        return 0 as u32;
    }
    let mut tag: u32 = 0 as u32;
    let mut len: u8 = 0 as u8;
    while *tags as ::core::ffi::c_int != 0 && (len as ::core::ffi::c_int) < 4 as ::core::ffi::c_int
    {
        tag = tag << 8 as ::core::ffi::c_int | *tags as u32;
        tags = tags.offset(1);
        len = len.wrapping_add(1);
    }
    while (len as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        tag = tag << 8 as ::core::ffi::c_int | ' ' as i32 as u32;
        len = len.wrapping_add(1);
    }
    return tag;
}
