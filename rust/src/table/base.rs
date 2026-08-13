#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, qsort};

use crate::support::parsed_json::{ParsedValue, json_numof, json_obj_get_type, json_obj_getstr_share, json_obj_key_at, json_obj_len, json_obj_val_at, json_type_of};
use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseScriptEntry {
    pub tag: u32,
    pub default_baseline_tag: u32,
    pub base_values_count: TableId,
    pub base_values: *mut BaseValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseAxis {
    pub script_count: TableId,
    pub entries: *mut BaseScriptEntry,
}
#[repr(C)]
pub struct BaseTable {
    pub horizontal: *mut BaseAxis,
    pub vertical: *mut BaseAxis,
}
// Stage 6-4 "Box化": `horizontal`/`vertical` are the only allocations this
// struct owns (each a `*mut BaseAxis`, itself owning `entries`/nested
// `base_values` -- left as raw pointers for this PR, freed the same way
// `dispose_base` always did). `Copy`/`Clone` dropped: a `Drop` impl and
// `Copy` are mutually exclusive, matching `LtshTable`/`VorgTable`/`CmapTable`.
//
// Preserves an existing leak, not introduced by this PR: `delete_base_axis`
// only frees `(*axis).entries` (and each entry's `base_values`), never
// `axis` itself, so the `BaseAxis` allocations from `read_axis`/
// `axis_from_json` are never freed on disposal -- true in the pre-Box化 C
// translation too (`dispose_base` never called `free()` on `horizontal`/
// `vertical`). Not fixed here, same discipline as the `unconsolidate.rs`
// move in the `ChainingRule.apply` PR: preserving byte-for-byte disposal
// behavior takes priority over opportunistic bug fixes within a Box化 PR.
impl Drop for BaseTable {
    fn drop(&mut self) {
        unsafe {
            delete_base_axis(self.horizontal);
            delete_base_axis(self.vertical);
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseTagList {
    pub size: TableId,
    pub items: *mut u32,
}
unsafe extern "C" fn delete_base_axis(mut axis: *mut BaseAxis) {
    if axis.is_null() {
        return;
    }
    if !(*axis).entries.is_null() {
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < (*axis).script_count as ::core::ffi::c_int {
            if !(*(*axis).entries.offset(j as isize)).base_values.is_null() {
                free((*(*axis).entries.offset(j as isize)).base_values as *mut ::core::ffi::c_void);
                let ref mut fresh0 = (*(*axis).entries.offset(j as isize)).base_values;
                *fresh0 = ::core::ptr::null_mut::<BaseValue>();
            }
            j = j.wrapping_add(1);
        }
        free((*axis).entries as *mut ::core::ffi::c_void);
        (*axis).entries = ::core::ptr::null_mut::<BaseScriptEntry>();
    }
}
unsafe extern "C" fn read_base_value(
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
unsafe extern "C" fn read_base_script(
    data: FontFilePointer,
    mut table_length: u32,
    mut offset: u16,
    mut entry: *mut BaseScriptEntry,
    mut base_tag_list: *mut u32,
    mut n_base_tags: u16,
) {
    let mut base_values_offset: u16 = 0;
    (*entry).base_values_count = 0 as TableId;
    (*entry).base_values = ::core::ptr::null_mut::<BaseValue>();
    (*entry).default_baseline_tag = 0 as u32;
    if !(table_length < (offset as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u32) {
        base_values_offset =
            read_16u(data.offset(offset as ::core::ffi::c_int as isize) as *const u8);
        if base_values_offset != 0 {
            base_values_offset =
                (base_values_offset as ::core::ffi::c_int + offset as ::core::ffi::c_int) as u16;
            if !(table_length
                < (base_values_offset as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as u32)
            {
                let mut default_index: u16 =
                    (read_16u(data.offset(base_values_offset as ::core::ffi::c_int as isize)
                        as *const u8) as ::core::ffi::c_int
                        % n_base_tags as ::core::ffi::c_int) as u16;
                (*entry).default_baseline_tag = *base_tag_list.offset(default_index as isize);
                (*entry).base_values_count = read_16u(
                    data.offset(base_values_offset as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const u8,
                ) as TableId;
                if !((*entry).base_values_count as ::core::ffi::c_int
                    != n_base_tags as ::core::ffi::c_int)
                {
                    if !(table_length
                        < (base_values_offset as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * (*entry).base_values_count as ::core::ffi::c_int)
                            as u32)
                    {
                        (*entry).base_values = __caryll_allocate_clean(
                            (::core::mem::size_of::<BaseValue>() as usize)
                                .wrapping_mul((*entry).base_values_count as usize),
                            44 as ::core::ffi::c_ulong,
                        ) as *mut BaseValue;
                        let mut j: TableId = 0 as TableId;
                        while (j as ::core::ffi::c_int)
                            < (*entry).base_values_count as ::core::ffi::c_int
                        {
                            (*(*entry).base_values.offset(j as isize)).tag =
                                *base_tag_list.offset(j as isize);
                            let mut _val_offset: u16 = read_16u(
                                data.offset(base_values_offset as ::core::ffi::c_int as isize)
                                    .offset(4 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (2 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            );
                            if _val_offset != 0 {
                                (*(*entry).base_values.offset(j as isize)).coordinate = read_base_value(
                                    data,
                                    table_length,
                                    (base_values_offset as ::core::ffi::c_int
                                        + _val_offset as ::core::ffi::c_int)
                                        as u16,
                                )
                                    as Pos;
                            } else {
                                (*(*entry).base_values.offset(j as isize)).coordinate =
                                    0 as ::core::ffi::c_int as Pos;
                            }
                            j = j.wrapping_add(1);
                        }
                        return;
                    }
                }
            }
        }
    }
    (*entry).base_values_count = 0 as TableId;
    if !(*entry).base_values.is_null() {
        free((*entry).base_values as *mut ::core::ffi::c_void);
        (*entry).base_values = ::core::ptr::null_mut::<BaseValue>();
    }
    (*entry).base_values = ::core::ptr::null_mut::<BaseValue>();
    (*entry).default_baseline_tag = 0 as u32;
}
unsafe extern "C" fn read_axis(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut offset: u16,
) -> *mut BaseAxis {
    let mut base_tag_list_offset: u16 = 0;
    let mut n_base_tags: u16 = 0;
    let mut base_script_list_offset: u16 = 0;
    let mut n_base_scripts: TableId = 0;
    let mut axis: *mut BaseAxis = ::core::ptr::null_mut::<BaseAxis>();
    let mut base_tag_list: *mut u32 = ::core::ptr::null_mut::<u32>();
    if !(table_length < (offset as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as u32) {
        base_tag_list_offset = (offset as ::core::ffi::c_int
            + read_16u(data.offset(offset as ::core::ffi::c_int as isize) as *const u8)
                as ::core::ffi::c_int) as u16;
        if !(base_tag_list_offset as ::core::ffi::c_int <= offset as ::core::ffi::c_int) {
            if !(table_length
                < (base_tag_list_offset as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u32)
            {
                n_base_tags = read_16u(
                    data.offset(base_tag_list_offset as ::core::ffi::c_int as isize) as *const u8,
                );
                if !(n_base_tags == 0) {
                    if !(table_length
                        < (base_tag_list_offset as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int * n_base_tags as ::core::ffi::c_int)
                            as u32)
                    {
                        base_tag_list = __caryll_allocate_clean(
                            (::core::mem::size_of::<u32>() as usize)
                                .wrapping_mul(n_base_tags as usize),
                            77 as ::core::ffi::c_ulong,
                        ) as *mut u32;
                        let mut j: u16 = 0 as u16;
                        while (j as ::core::ffi::c_int) < n_base_tags as ::core::ffi::c_int {
                            *base_tag_list.offset(j as isize) = read_32u(
                                data.offset(base_tag_list_offset as ::core::ffi::c_int as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(
                                        (j as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                            );
                            j = j.wrapping_add(1);
                        }
                        base_script_list_offset = (offset as ::core::ffi::c_int
                            + read_16u(
                                data.offset(offset as ::core::ffi::c_int as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as *const u8,
                            ) as ::core::ffi::c_int)
                            as u16;
                        if !(base_script_list_offset as ::core::ffi::c_int
                            <= offset as ::core::ffi::c_int)
                        {
                            if !(table_length
                                < (base_script_list_offset as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int)
                                    as u32)
                            {
                                n_base_scripts = read_16u(
                                    data.offset(base_script_list_offset as ::core::ffi::c_int as isize)
                                        as *const u8,
                                ) as TableId;
                                if !(table_length
                                    < (base_script_list_offset as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int
                                        + 6 as ::core::ffi::c_int
                                            * n_base_scripts as ::core::ffi::c_int)
                                        as u32)
                                {
                                    axis = __caryll_allocate_clean(
                                        ::core::mem::size_of::<BaseAxis>() as usize,
                                        87 as ::core::ffi::c_ulong,
                                    )
                                        as *mut BaseAxis;
                                    (*axis).script_count = n_base_scripts;
                                    (*axis).entries = __caryll_allocate_clean(
                                        (::core::mem::size_of::<BaseScriptEntry>() as usize)
                                            .wrapping_mul(n_base_scripts as usize),
                                        89 as ::core::ffi::c_ulong,
                                    )
                                        as *mut BaseScriptEntry;
                                    let mut j_0: TableId = 0 as TableId;
                                    while (j_0 as ::core::ffi::c_int)
                                        < n_base_scripts as ::core::ffi::c_int
                                    {
                                        (*(*axis).entries.offset(j_0 as isize)).tag = read_32u(
                                            data.offset(
                                                base_script_list_offset as ::core::ffi::c_int as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (6 as ::core::ffi::c_int
                                                    * j_0 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as *const u8,
                                        );
                                        let mut base_script_offset: u16 = read_16u(
                                            data.offset(
                                                base_script_list_offset as ::core::ffi::c_int as isize,
                                            )
                                            .offset(2 as ::core::ffi::c_int as isize)
                                            .offset(
                                                (6 as ::core::ffi::c_int
                                                    * j_0 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                            .offset(4 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        );
                                        if base_script_offset != 0 {
                                            read_base_script(
                                                data,
                                                table_length,
                                                (base_script_list_offset as ::core::ffi::c_int
                                                    + base_script_offset as ::core::ffi::c_int)
                                                    as u16,
                                                (*axis).entries.offset(j_0 as isize)
                                                    as *mut BaseScriptEntry,
                                                base_tag_list,
                                                n_base_tags,
                                            );
                                        } else {
                                            (*(*axis).entries.offset(j_0 as isize))
                                                .base_values_count = 0 as TableId;
                                            let ref mut fresh1 =
                                                (*(*axis).entries.offset(j_0 as isize)).base_values;
                                            *fresh1 = ::core::ptr::null_mut::<BaseValue>();
                                            (*(*axis).entries.offset(j_0 as isize))
                                                .default_baseline_tag = 0 as u32;
                                        }
                                        j_0 = j_0.wrapping_add(1);
                                    }
                                    return axis;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !base_tag_list.is_null() {
        free(base_tag_list as *mut ::core::ffi::c_void);
        base_tag_list = ::core::ptr::null_mut::<u32>();
    }
    delete_base_axis(axis);
    axis = ::core::ptr::null_mut::<BaseAxis>();
    return axis;
}
pub unsafe extern "C" fn otfcc_read_base(
    packet: Packet,
    mut options: *const Options,
) -> Option<Box<BaseTable>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_BASE {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut offset_h: u16 = 0;
                    let mut offset_v: u16 = 0;
                    let mut data: FontFilePointer = table.data as FontFilePointer;
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
                        let mut horizontal: *mut BaseAxis = ::core::ptr::null_mut::<BaseAxis>();
                        let mut vertical: *mut BaseAxis = ::core::ptr::null_mut::<BaseAxis>();
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
unsafe extern "C" fn axis_to_json(mut axis: *const BaseAxis) -> *mut BuiltValue {
    let mut _axis: *mut BuiltValue = json_object_new((*axis).script_count as usize);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*axis).script_count as ::core::ffi::c_int {
        if !((*(*axis).entries.offset(j as isize)).tag == 0) {
            let mut _entry: *mut BuiltValue = json_object_new(3 as usize);
            if (*(*axis).entries.offset(j as isize)).default_baseline_tag != 0 {
                let mut tag: [::core::ffi::c_char; 4] = [0; 4];
                tag2str(
                    (*(*axis).entries.offset(j as isize)).default_baseline_tag,
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
            let mut _values: *mut BuiltValue =
                json_object_new((*(*axis).entries.offset(j as isize)).base_values_count as usize);
            let mut k: TableId = 0 as TableId;
            while (k as ::core::ffi::c_int)
                < (*(*axis).entries.offset(j as isize)).base_values_count as ::core::ffi::c_int
            {
                if (*(*(*axis).entries.offset(j as isize))
                    .base_values
                    .offset(k as isize))
                .tag != 0
                {
                    json_object_push_tag(
                        _values,
                        (*(*(*axis).entries.offset(j as isize))
                            .base_values
                            .offset(k as isize))
                        .tag,
                        json_new_position(
                            (*(*(*axis).entries.offset(j as isize))
                                .base_values
                                .offset(k as isize))
                            .coordinate,
                        ),
                    );
                }
                k = k.wrapping_add(1);
            }
            json_object_push(
                _entry,
                b"baselines\0" as *const u8 as *const ::core::ffi::c_char,
                _values,
            );
            json_object_push_tag(_axis, (*(*axis).entries.offset(j as isize)).tag, _entry);
        }
        j = j.wrapping_add(1);
    }
    return _axis;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_dump_base(
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
        if !(*base).horizontal.is_null() {
            json_object_push(
                _base,
                b"horizontal\0" as *const u8 as *const ::core::ffi::c_char,
                axis_to_json((*base).horizontal),
            );
        }
        if !(*base).vertical.is_null() {
            json_object_push(
                _base,
                b"vertical\0" as *const u8 as *const ::core::ffi::c_char,
                axis_to_json((*base).vertical),
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
unsafe extern "C" fn base_script_from_json(
    mut _sr: *const ParsedValue,
    mut entry: *mut BaseScriptEntry,
) {
    (*entry).default_baseline_tag = str2tag(json_obj_getstr_share(
        _sr,
        b"defaultBaseline\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    let mut _basevalues: *const ParsedValue = json_obj_get_type(
        _sr,
        b"baselines\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if _basevalues.is_null() {
        (*entry).base_values_count = 0 as TableId;
        (*entry).base_values = ::core::ptr::null_mut::<BaseValue>();
    } else {
        (*entry).base_values_count = json_obj_len(_basevalues) as TableId;
        (*entry).base_values = __caryll_allocate_clean(
            (::core::mem::size_of::<BaseValue>() as usize)
                .wrapping_mul((*entry).base_values_count as usize),
            171 as ::core::ffi::c_ulong,
        ) as *mut BaseValue;
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < (*entry).base_values_count as ::core::ffi::c_int {
            (*(*entry).base_values.offset(j as isize)).tag =
                str2tag(json_obj_key_at(_basevalues, j as u32));
            (*(*entry).base_values.offset(j as isize)).coordinate =
                json_numof(json_obj_val_at(_basevalues, j as u32)) as Pos;
            j = j.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn by_script_tag(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut BaseScriptEntry))
        .tag
        .wrapping_sub((*(b as *mut BaseScriptEntry)).tag) as ::core::ffi::c_int;
}
unsafe extern "C" fn axis_from_json(mut _axis: *const ParsedValue) -> *mut BaseAxis {
    if _axis.is_null() {
        return ::core::ptr::null_mut::<BaseAxis>();
    }
    let mut axis: *mut BaseAxis = ::core::ptr::null_mut::<BaseAxis>();
    axis = __caryll_allocate_clean(
        ::core::mem::size_of::<BaseAxis>() as usize,
        186 as ::core::ffi::c_ulong,
    ) as *mut BaseAxis;
    (*axis).script_count = json_obj_len(_axis) as TableId;
    (*axis).entries = __caryll_allocate_clean(
        (::core::mem::size_of::<BaseScriptEntry>() as usize)
            .wrapping_mul((*axis).script_count as usize),
        188 as ::core::ffi::c_ulong,
    ) as *mut BaseScriptEntry;
    let mut jj: TableId = 0 as TableId;
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*axis).script_count as ::core::ffi::c_int {
        let script_val = json_obj_val_at(_axis, j as u32);
        if !script_val
            .is_null()
            && json_type_of(script_val) == JsonType::Object
        {
            (*(*axis).entries.offset(jj as isize)).tag =
                str2tag(json_obj_key_at(_axis, j as u32));
            base_script_from_json(
                script_val,
                (*axis).entries.offset(jj as isize) as *mut BaseScriptEntry,
            );
            jj = jj.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    (*axis).script_count = jj;
    qsort(
        (*axis).entries as *mut ::core::ffi::c_void,
        (*axis).script_count as usize,
        ::core::mem::size_of::<BaseScriptEntry>() as usize,
        Some(
            by_script_tag
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    return axis;
}
pub unsafe extern "C" fn otfcc_parse_base(
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
unsafe extern "C" fn by_tag(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(a as *mut u32)).wrapping_sub(*(b as *mut u32)) as ::core::ffi::c_int;
}
pub unsafe extern "C" fn axis_to_bk(mut axis: *const BaseAxis) -> *mut BkBlock {
    if axis.is_null() {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let mut taglist: BaseTagList = BaseTagList {
        size: 0,
        items: ::core::ptr::null_mut::<u32>(),
    };
    taglist.size = 0 as TableId;
    taglist.items = ::core::ptr::null_mut::<u32>();
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*axis).script_count as ::core::ffi::c_int {
        let mut entry: *mut BaseScriptEntry =
            (*axis).entries.offset(j as isize) as *mut BaseScriptEntry;
        if (*entry).default_baseline_tag != 0 {
            let mut found: bool = false;
            let mut jk: TableId = 0 as TableId;
            while (jk as ::core::ffi::c_int) < taglist.size as ::core::ffi::c_int {
                if *taglist.items.offset(jk as isize) == (*entry).default_baseline_tag {
                    found = true;
                    break;
                } else {
                    jk = jk.wrapping_add(1);
                }
            }
            if !found {
                taglist.size =
                    (taglist.size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
                taglist.items = __caryll_reallocate(
                    taglist.items as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<u32>() as usize)
                        .wrapping_mul(taglist.size as usize),
                    241 as ::core::ffi::c_ulong,
                ) as *mut u32;
                *taglist.items.offset(
                    (taglist.size as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) = (*entry).default_baseline_tag;
            }
        }
        let mut k: TableId = 0 as TableId;
        while (k as ::core::ffi::c_int) < (*entry).base_values_count as ::core::ffi::c_int {
            let mut tag: u32 = (*(*entry).base_values.offset(k as isize)).tag;
            let mut found_0: bool = false;
            let mut jk_0: TableId = 0 as TableId;
            while (jk_0 as ::core::ffi::c_int) < taglist.size as ::core::ffi::c_int {
                if *taglist.items.offset(jk_0 as isize) == tag {
                    found_0 = true;
                    break;
                } else {
                    jk_0 = jk_0.wrapping_add(1);
                }
            }
            if !found_0 {
                taglist.size =
                    (taglist.size as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as TableId;
                taglist.items = __caryll_reallocate(
                    taglist.items as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<u32>() as usize)
                        .wrapping_mul(taglist.size as usize),
                    256 as ::core::ffi::c_ulong,
                ) as *mut u32;
                *taglist.items.offset(
                    (taglist.size as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ) = tag;
            }
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    qsort(
        taglist.items as *mut ::core::ffi::c_void,
        taglist.size as usize,
        ::core::mem::size_of::<u32>() as usize,
        Some(
            by_tag
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut base_tag_list: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (taglist.size as ::core::ffi::c_int) as u32)]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < taglist.size as ::core::ffi::c_int {
        bk_push(base_tag_list, &[bk_int(BkCellType::B32, (*taglist.items.offset(j_0 as isize)) as u32)]);
        j_0 = j_0.wrapping_add(1);
    }
    let mut base_script_list: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*axis).script_count as ::core::ffi::c_int) as u32)]);
    let mut j_1: TableId = 0 as TableId;
    while (j_1 as ::core::ffi::c_int) < (*axis).script_count as ::core::ffi::c_int {
        let mut entry_0: *mut BaseScriptEntry =
            (*axis).entries.offset(j_1 as isize) as *mut BaseScriptEntry;
        let mut base_values: *mut BkBlock = bk_new_block(&[]);
        let mut default_index: TableId = 0 as TableId;
        let mut m: TableId = 0 as TableId;
        while (m as ::core::ffi::c_int) < taglist.size as ::core::ffi::c_int {
            if *taglist.items.offset(m as isize) == (*entry_0).default_baseline_tag {
                default_index = m;
                break;
            } else {
                m = m.wrapping_add(1);
            }
        }
        bk_push(base_values, &[bk_int(BkCellType::B16, (default_index as ::core::ffi::c_int) as u32)]);
        bk_push(base_values, &[bk_int(BkCellType::B16, (taglist.size as ::core::ffi::c_int) as u32)]);
        let mut m_0: usize = 0 as usize;
        while m_0 < taglist.size as usize {
            let mut found_1: bool = false;
            let mut found_index: TableId = 0 as TableId;
            let mut k_0: TableId = 0 as TableId;
            while (k_0 as ::core::ffi::c_int) < (*entry_0).base_values_count as ::core::ffi::c_int {
                if (*(*entry_0).base_values.offset(k_0 as isize)).tag
                    == *taglist.items.offset(m_0 as isize)
                {
                    found_1 = true;
                    found_index = k_0;
                    break;
                } else {
                    k_0 = k_0.wrapping_add(1);
                }
            }
            if found_1 {
                bk_push(base_values, &[bk_ptr(BkCellType::P16, bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_int(BkCellType::B16, ((*(*entry_0).base_values.offset(found_index as isize)).coordinate as i16
                            as ::core::ffi::c_int) as u32)]))]);
            } else {
                bk_push(base_values, &[bk_ptr(BkCellType::P16, bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_int(BkCellType::B16, 0 as u32)]))]);
            }
            m_0 = m_0.wrapping_add(1);
        }
        let mut script_record: *mut BkBlock = bk_new_block(&[bk_ptr(BkCellType::P16, base_values), bk_ptr(BkCellType::P16, ::core::ptr::null_mut()), bk_int(BkCellType::B16, 0 as u32)]);
        bk_push(base_script_list, &[bk_int(BkCellType::B32, ((*entry_0).tag) as u32), bk_ptr(BkCellType::P16, script_record)]);
        j_1 = j_1.wrapping_add(1);
    }
    free(taglist.items as *mut ::core::ffi::c_void);
    taglist.items = ::core::ptr::null_mut::<u32>();
    return bk_new_block(&[bk_ptr(BkCellType::P16, base_tag_list), bk_ptr(BkCellType::P16, base_script_list)]);
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_base(
    base: Option<&BaseTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let base = match base {
        Some(b) => b as *const BaseTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B32, 0x10000 as u32), bk_ptr(BkCellType::P16, axis_to_bk((*base).horizontal)), bk_ptr(BkCellType::P16, axis_to_bk((*base).vertical))]);
    return bk_build_block(root);
}
#[inline]
unsafe extern "C" fn tag2str(mut tag: u32, mut tags: *mut ::core::ffi::c_char) {
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
unsafe extern "C" fn str2tag(mut tags: *const ::core::ffi::c_char) -> u32 {
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
