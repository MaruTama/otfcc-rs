use libc::{free, malloc, memcpy, memset, qsort};
extern "C" {
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    static otl_iCoverage: __otfcc_ICoverage;
    fn bk_new_Block(type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_push(b: *mut bk_Block, type0: ::core::ffi::c_int, ...) -> *mut bk_Block;
    fn bk_newBlockFromBuffer(buf: *mut caryll_Buffer) -> *mut bk_Block;
    fn bk_build_Block(root: *mut bk_Block) -> *mut caryll_Buffer;
    fn position_format_length(format: u16) -> u8;
    fn read_gpos_value(
        data: font_file_pointer,
        tableLength: u32,
        offset: u32,
        format: u16,
    ) -> otl_PositionValue;
    fn required_position_format(v: otl_PositionValue) -> u8;
    fn bk_gpos_value(v: otl_PositionValue, format: u16) -> *mut bk_Block;
    fn gpos_dump_value(value: otl_PositionValue) -> *mut json_value;
    fn gpos_parse_value(pos: *mut json_value) -> otl_PositionValue;
}

use crate::table::otl::coverage::{__otfcc_ICoverage, otl_Coverage, otl_Coverage_create, otl_Coverage_free, pushToCoverage, readCoverage};
use crate::support::handle::{handle_fromName, otfcc_Handle_dispose, otfcc_Handle_dup, otfcc_Handle, otfcc_GlyphHandle, HANDLE_STATE_EMPTY};
use crate::support::binio::{read_16u};

use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_object, json_value};
use crate::support::cvec::{CVecRaw, cvec_grow, cvec_grow_to, cvec_grow_to_n, cvec_init, cvec_move, cvec_pop, cvec_push, cvec_resize_to};
use crate::bk::bkblock::{b16, bk_Block, bkembed, bkover, p16};

use crate::table::otl::{__caryll_vectorinterface_subtable_gpos_single, otl_GposSingleEntry, otl_PositionValue, otl_Subtable, subtable_gpos_single};
use crate::table::otl::subtables::{otl_BuildHeuristics};
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __caryll_elementinterface_otl_GposSingleEntry {
    pub init: Option<unsafe extern "C" fn(*mut otl_GposSingleEntry) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut otl_GposSingleEntry, *const otl_GposSingleEntry) -> ()>,
    pub move_0:
        Option<unsafe extern "C" fn(*mut otl_GposSingleEntry, *mut otl_GposSingleEntry) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otl_GposSingleEntry) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otl_GposSingleEntry, otl_GposSingleEntry) -> ()>,
    pub copyReplace:
        Option<unsafe extern "C" fn(*mut otl_GposSingleEntry, otl_GposSingleEntry) -> ()>,
}
unsafe extern "C" fn deleteGposSingleEntry(mut entry: *mut otl_GposSingleEntry) {
    otfcc_Handle_dispose(&raw mut (*entry).target);
}
static mut gss_typeinfo: __caryll_elementinterface_otl_GposSingleEntry = {
    __caryll_elementinterface_otl_GposSingleEntry {
        init: None,
        copy: None,
        move_0: None,
        dispose: Some(
            deleteGposSingleEntry as unsafe extern "C" fn(*mut otl_GposSingleEntry) -> (),
        ),
        replace: None,
        copyReplace: None,
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_single_move(dst: *mut subtable_gpos_single, src: *mut subtable_gpos_single) {
    cvec_move(as_cvec(dst), as_cvec(src));
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_resizeTo(arr: *mut subtable_gpos_single, target: usize) {
    cvec_resize_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_filterEnv(
    mut arr: *mut subtable_gpos_single,
    mut fn_0: Option<
        unsafe extern "C" fn(*const otl_GposSingleEntry, *mut ::core::ffi::c_void) -> bool,
    >,
    mut env: *mut ::core::ffi::c_void,
) {
    let mut j: usize = 0 as usize;
    let mut k: usize = 0 as usize;
    while k < (*arr).length {
        if fn_0.expect("non-null function pointer")(
            (*arr).items.offset(k as isize) as *mut otl_GposSingleEntry,
            env,
        ) {
            if j != k {
                *(*arr).items.offset(j as isize) = *(*arr).items.offset(k as isize);
            }
            j = j.wrapping_add(1);
        } else {
            if gss_typeinfo.dispose.is_some() {
                gss_typeinfo.dispose.expect("non-null function pointer")(
                    (*arr).items.offset(k as isize) as *mut otl_GposSingleEntry,
                );
            } else {
            };
        }
        k = k.wrapping_add(1);
    }
    (*arr).length = j;
}
#[inline]
unsafe fn as_cvec(arr: *mut subtable_gpos_single) -> *mut CVecRaw<otl_GposSingleEntry> {
    arr as *mut CVecRaw<otl_GposSingleEntry>
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_init(arr: *mut subtable_gpos_single) {
    cvec_init(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_disposeItem(
    mut arr: *mut subtable_gpos_single,
    mut n: usize,
) {
    if gss_typeinfo.dispose.is_some() {
        gss_typeinfo.dispose.expect("non-null function pointer")(
            (*arr).items.offset(n as isize) as *mut otl_GposSingleEntry
        );
    } else {
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_sort(
    mut arr: *mut subtable_gpos_single,
    mut fn_0: Option<
        unsafe extern "C" fn(
            *const otl_GposSingleEntry,
            *const otl_GposSingleEntry,
        ) -> ::core::ffi::c_int,
    >,
) {
    qsort(
        (*arr).items as *mut ::core::ffi::c_void,
        (*arr).length,
        ::core::mem::size_of::<otl_GposSingleEntry>() as usize,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const otl_GposSingleEntry,
                    *const otl_GposSingleEntry,
                ) -> ::core::ffi::c_int,
            >,
            __compar_fn_t,
        >(fn_0),
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_fill(mut arr: *mut subtable_gpos_single, mut n: usize) {
    while (*arr).length < n {
        let mut x: otl_GposSingleEntry = otl_GposSingleEntry {
            target: otfcc_Handle {
                state: HANDLE_STATE_EMPTY,
                index: 0,
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
            value: otl_PositionValue {
                dx: 0.,
                dy: 0.,
                dWidth: 0.,
                dHeight: 0.,
            },
        };
        if gss_typeinfo.init.is_some() {
            gss_typeinfo.init.expect("non-null function pointer")(&raw mut x);
        } else {
            memset(
                &raw mut x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<otl_GposSingleEntry>() as usize,
            );
        }
        subtable_gpos_single_push(arr, x);
    }
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_push(arr: *mut subtable_gpos_single, elem: otl_GposSingleEntry) {
    cvec_push(as_cvec(arr), elem);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_grow(arr: *mut subtable_gpos_single) {
    cvec_grow(as_cvec(arr));
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_growTo(arr: *mut subtable_gpos_single, target: usize) {
    cvec_grow_to(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_pop(arr: *mut subtable_gpos_single) -> otl_GposSingleEntry {
    cvec_pop(as_cvec(arr))
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_copyReplace(
    mut dst: *mut subtable_gpos_single,
    src: subtable_gpos_single,
) {
    subtable_gpos_single_dispose(dst);
    subtable_gpos_single_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_copy(
    mut dst: *mut subtable_gpos_single,
    mut src: *const subtable_gpos_single,
) {
    subtable_gpos_single_init(dst);
    subtable_gpos_single_growTo(dst, (*src).length);
    (*dst).length = (*src).length;
    if gss_typeinfo.copy.is_some() {
        let mut j: usize = 0 as usize;
        while j < (*src).length {
            gss_typeinfo.copy.expect("non-null function pointer")(
                (*dst).items.offset(j as isize) as *mut otl_GposSingleEntry,
                (*src).items.offset(j as isize) as *mut otl_GposSingleEntry
                    as *const otl_GposSingleEntry,
            );
            j = j.wrapping_add(1);
        }
    } else {
        let mut j_0: usize = 0 as usize;
        while j_0 < (*src).length {
            *(*dst).items.offset(j_0 as isize) = *(*src).items.offset(j_0 as isize);
            j_0 = j_0.wrapping_add(1);
        }
    };
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_dispose(mut arr: *mut subtable_gpos_single) {
    if arr.is_null() {
        return;
    }
    if gss_typeinfo.dispose.is_some() {
        let mut j: usize = (*arr).length;
        loop {
            let fresh1 = j;
            j = j.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            gss_typeinfo.dispose.expect("non-null function pointer")(
                (*arr).items.offset(j as isize) as *mut otl_GposSingleEntry,
            );
        }
    }
    free((*arr).items as *mut ::core::ffi::c_void);
    (*arr).items = ::core::ptr::null_mut::<otl_GposSingleEntry>();
    (*arr).length = 0 as usize;
    (*arr).capacity = 0 as usize;
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_replace(
    mut dst: *mut subtable_gpos_single,
    src: subtable_gpos_single,
) {
    subtable_gpos_single_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<subtable_gpos_single>() as usize,
    );
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_initCapN(
    mut arr: *mut subtable_gpos_single,
    mut n: usize,
) {
    subtable_gpos_single_init(arr);
    subtable_gpos_single_growToN(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_growToN(arr: *mut subtable_gpos_single, target: usize) {
    cvec_grow_to_n(as_cvec(arr), target);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_initN(mut arr: *mut subtable_gpos_single, mut n: usize) {
    subtable_gpos_single_init(arr);
    subtable_gpos_single_growToN(arr, n);
    subtable_gpos_single_fill(arr, n);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_free(mut x: *mut subtable_gpos_single) {
    if x.is_null() {
        return;
    }
    subtable_gpos_single_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_createN(mut n: usize) -> *mut subtable_gpos_single {
    let mut t: *mut subtable_gpos_single =
        malloc(::core::mem::size_of::<subtable_gpos_single>() as usize)
            as *mut subtable_gpos_single;
    subtable_gpos_single_initN(t, n);
    return t;
}
#[inline]
unsafe extern "C" fn subtable_gpos_single_create() -> *mut subtable_gpos_single {
    let mut x: *mut subtable_gpos_single =
        malloc(::core::mem::size_of::<subtable_gpos_single>() as usize)
            as *mut subtable_gpos_single;
    subtable_gpos_single_init(x);
    return x;
}
#[no_mangle]
pub static mut iSubtable_gpos_single: __caryll_vectorinterface_subtable_gpos_single = {
    __caryll_vectorinterface_subtable_gpos_single {
        init: Some(
            subtable_gpos_single_init as unsafe extern "C" fn(*mut subtable_gpos_single) -> (),
        ),
        copy: Some(
            subtable_gpos_single_copy
                as unsafe extern "C" fn(
                    *mut subtable_gpos_single,
                    *const subtable_gpos_single,
                ) -> (),
        ),
        move_0: Some(
            subtable_gpos_single_move
                as unsafe extern "C" fn(*mut subtable_gpos_single, *mut subtable_gpos_single) -> (),
        ),
        dispose: Some(
            subtable_gpos_single_dispose as unsafe extern "C" fn(*mut subtable_gpos_single) -> (),
        ),
        replace: Some(
            subtable_gpos_single_replace
                as unsafe extern "C" fn(*mut subtable_gpos_single, subtable_gpos_single) -> (),
        ),
        copyReplace: Some(
            subtable_gpos_single_copyReplace
                as unsafe extern "C" fn(*mut subtable_gpos_single, subtable_gpos_single) -> (),
        ),
        create: Some(subtable_gpos_single_create),
        free: Some(
            subtable_gpos_single_free as unsafe extern "C" fn(*mut subtable_gpos_single) -> (),
        ),
        initN: Some(
            subtable_gpos_single_initN
                as unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> (),
        ),
        initCapN: Some(
            subtable_gpos_single_initCapN
                as unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> (),
        ),
        createN: Some(
            subtable_gpos_single_createN
                as unsafe extern "C" fn(usize) -> *mut subtable_gpos_single,
        ),
        fill: Some(
            subtable_gpos_single_fill
                as unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> (),
        ),
        clear: Some(
            subtable_gpos_single_dispose as unsafe extern "C" fn(*mut subtable_gpos_single) -> (),
        ),
        push: Some(
            subtable_gpos_single_push
                as unsafe extern "C" fn(*mut subtable_gpos_single, otl_GposSingleEntry) -> (),
        ),
        shrinkToFit: Some(
            subtable_gpos_single_shrinkToFit
                as unsafe extern "C" fn(*mut subtable_gpos_single) -> (),
        ),
        pop: Some(
            subtable_gpos_single_pop
                as unsafe extern "C" fn(*mut subtable_gpos_single) -> otl_GposSingleEntry,
        ),
        disposeItem: Some(
            subtable_gpos_single_disposeItem
                as unsafe extern "C" fn(*mut subtable_gpos_single, usize) -> (),
        ),
        filterEnv: Some(
            subtable_gpos_single_filterEnv
                as unsafe extern "C" fn(
                    *mut subtable_gpos_single,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GposSingleEntry,
                            *mut ::core::ffi::c_void,
                        ) -> bool,
                    >,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        sort: Some(
            subtable_gpos_single_sort
                as unsafe extern "C" fn(
                    *mut subtable_gpos_single,
                    Option<
                        unsafe extern "C" fn(
                            *const otl_GposSingleEntry,
                            *const otl_GposSingleEntry,
                        ) -> ::core::ffi::c_int,
                    >,
                ) -> (),
        ),
    }
};
#[inline]
unsafe extern "C" fn subtable_gpos_single_shrinkToFit(mut arr: *mut subtable_gpos_single) {
    subtable_gpos_single_resizeTo(arr, (*arr).length);
}
#[no_mangle]
pub unsafe extern "C" fn otl_read_gpos_single(
    data: font_file_pointer,
    mut tableLength: u32,
    mut offset: u32,
    _maxGlyphs: glyphid_t,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut subtableFormat: u16 = 0;
    let mut current_block: u64;
    let mut subtable: *mut subtable_gpos_single =
        (
            iSubtable_gpos_single
                .create
                .expect("non-null function pointer"))();
    let mut targets: *mut otl_Coverage = ::core::ptr::null_mut::<otl_Coverage>();
    if !(tableLength < offset.wrapping_add(6 as u32)) {
        subtableFormat = read_16u(data.offset(offset as isize) as *const u8);
        targets = readCoverage(
            data as *const u8,
            tableLength,
            offset.wrapping_add(read_16u(
                data.offset(offset as isize)
                    .offset(2 as ::core::ffi::c_int as isize) as *const u8,
            ) as u32),
        );
        if !(targets.is_null()
            || (*targets).numGlyphs as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            if subtableFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                let mut v: otl_PositionValue = read_gpos_value(
                    data,
                    tableLength,
                    offset.wrapping_add(6 as u32),
                    read_16u(
                        data.offset(offset as isize)
                            .offset(4 as ::core::ffi::c_int as isize)
                            as *const u8,
                    ),
                );
                let mut j: glyphid_t = 0 as glyphid_t;
                while (j as ::core::ffi::c_int) < (*targets).numGlyphs as ::core::ffi::c_int {
                    iSubtable_gpos_single
                        .push
                        .expect("non-null function pointer")(
                        subtable,
                        otl_GposSingleEntry {
                            target: otfcc_Handle_dup(
                                *(*targets).glyphs.offset(j as isize) as otfcc_Handle,
                            ) as otfcc_GlyphHandle,
                            value: v,
                        },
                    );
                    j = j.wrapping_add(1);
                }
                current_block = 6009453772311597924;
            } else {
                let mut valueFormat: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *const u8,
                );
                let mut valueCount: u16 = read_16u(
                    data.offset(offset as isize)
                        .offset(6 as ::core::ffi::c_int as isize)
                        as *const u8,
                );
                if tableLength
                    < offset.wrapping_add(8 as u32).wrapping_add(
                        (position_format_length(valueFormat) as ::core::ffi::c_int
                            * valueCount as ::core::ffi::c_int) as u32,
                    )
                {
                    current_block = 18154618883129817269;
                } else if valueCount as ::core::ffi::c_int
                    != (*targets).numGlyphs as ::core::ffi::c_int
                {
                    current_block = 18154618883129817269;
                } else {
                    let mut j_0: glyphid_t = 0 as glyphid_t;
                    while (j_0 as ::core::ffi::c_int) < (*targets).numGlyphs as ::core::ffi::c_int {
                        iSubtable_gpos_single
                            .push
                            .expect("non-null function pointer")(
                            subtable,
                            otl_GposSingleEntry {
                                target: otfcc_Handle_dup(
                                    *(*targets).glyphs.offset(j_0 as isize) as otfcc_Handle,
                                ) as otfcc_GlyphHandle,
                                value: read_gpos_value(
                                    data,
                                    tableLength,
                                    offset.wrapping_add(8 as u32).wrapping_add(
                                        (j_0 as ::core::ffi::c_int
                                            * position_format_length(valueFormat)
                                                as ::core::ffi::c_int)
                                            as u32,
                                    ),
                                    valueFormat,
                                ),
                            },
                        );
                        j_0 = j_0.wrapping_add(1);
                    }
                    current_block = 6009453772311597924;
                }
            }
            match current_block {
                18154618883129817269 => {}
                _ => {
                    if !targets.is_null() {
                        otl_Coverage_free(targets);
                    }
                    return subtable as *mut otl_Subtable;
                }
            }
        }
    }
    if !targets.is_null() {
        otl_Coverage_free(targets);
    }
    iSubtable_gpos_single
        .free
        .expect("non-null function pointer")(subtable);
    return ::core::ptr::null_mut::<otl_Subtable>();
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_dump_single(
    mut _subtable: *const otl_Subtable,
) -> *mut json_value {
    let mut subtable: *const subtable_gpos_single = &raw const (*_subtable).gpos_single;
    let mut st: *mut json_value = json_object_new((*subtable).length);
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as usize) < (*subtable).length {
        json_object_push(
            st,
            (*(*subtable).items.offset(j as isize)).target.name as *const ::core::ffi::c_char,
            gpos_dump_value((*(*subtable).items.offset(j as isize)).value),
        );
        j = j.wrapping_add(1);
    }
    return st;
}
#[no_mangle]
pub unsafe extern "C" fn otl_gpos_parse_single(
    mut _subtable: *const json_value,
    mut _options: *const otfcc_Options,
) -> *mut otl_Subtable {
    let mut subtable: *mut subtable_gpos_single =
        (
            iSubtable_gpos_single
                .create
                .expect("non-null function pointer"))();
    let mut j: glyphid_t = 0 as glyphid_t;
    while (j as ::core::ffi::c_uint) < (*_subtable).u.object.length {
        if !(*(*_subtable).u.object.values.offset(j as isize))
            .value
            .is_null()
            && (*(*(*_subtable).u.object.values.offset(j as isize)).value).type_0
                as ::core::ffi::c_uint
                == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut gname: sds = sdsnewlen(
                (*(*_subtable).u.object.values.offset(j as isize)).name
                    as *const ::core::ffi::c_void,
                (*(*_subtable).u.object.values.offset(j as isize)).name_length as usize,
            );
            iSubtable_gpos_single
                .push
                .expect("non-null function pointer")(
                subtable,
                otl_GposSingleEntry {
                    target: handle_fromName(gname)
                        as otfcc_GlyphHandle,
                    value: gpos_parse_value(
                        (*(*_subtable).u.object.values.offset(j as isize)).value as *mut json_value,
                    ),
                },
            );
        }
        j = j.wrapping_add(1);
    }
    return subtable as *mut otl_Subtable;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_build_gpos_single(
    mut _subtable: *const otl_Subtable,
    mut _heuristics: otl_BuildHeuristics,
) -> *mut caryll_Buffer {
    let mut subtable: *const subtable_gpos_single = &raw const (*_subtable).gpos_single;
    let mut isConst: bool = (*subtable).length > 0 as usize;
    let mut format: u16 = 0 as u16;
    if (*subtable).length > 0 as usize {
        let mut j: glyphid_t = 0 as glyphid_t;
        while (j as usize) < (*subtable).length {
            isConst = isConst as ::core::ffi::c_int != 0
                && (*(*subtable).items.offset(j as isize)).value.dx
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .dx
                && (*(*subtable).items.offset(j as isize)).value.dy
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .dy
                && (*(*subtable).items.offset(j as isize)).value.dWidth
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .dWidth
                && (*(*subtable).items.offset(j as isize)).value.dHeight
                    == (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize))
                        .value
                        .dHeight;
            format = (format as ::core::ffi::c_int
                | required_position_format((*(*subtable).items.offset(j as isize)).value)
                    as ::core::ffi::c_int) as u16;
            j = j.wrapping_add(1);
        }
    }
    let mut cov: *mut otl_Coverage = otl_Coverage_create();
    let mut j_0: glyphid_t = 0 as glyphid_t;
    while (j_0 as usize) < (*subtable).length {
        pushToCoverage(
            cov,
            otfcc_Handle_dup(
                (*(*subtable).items.offset(j_0 as isize)).target as otfcc_Handle,
            ) as otfcc_GlyphHandle,
        );
        j_0 = j_0.wrapping_add(1);
    }
    let mut coverageBuf: *mut caryll_Buffer =
        otl_iCoverage.build.expect("non-null function pointer")(cov);
    if isConst {
        let mut b: *mut bk_Block = bk_new_Block(
            b16 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            p16 as ::core::ffi::c_int,
            bk_newBlockFromBuffer(coverageBuf),
            b16 as ::core::ffi::c_int,
            format as ::core::ffi::c_int,
            bkembed as ::core::ffi::c_int,
            bk_gpos_value(
                (*(*subtable).items.offset(0 as ::core::ffi::c_int as isize)).value,
                format,
            ),
            bkover as ::core::ffi::c_int,
        );
        otl_Coverage_free(cov);
        return bk_build_Block(b);
    } else {
        let mut b_0: *mut bk_Block = bk_new_Block(
            b16 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            p16 as ::core::ffi::c_int,
            bk_newBlockFromBuffer(coverageBuf),
            b16 as ::core::ffi::c_int,
            format as ::core::ffi::c_int,
            b16 as ::core::ffi::c_int,
            (*subtable).length,
            bkover as ::core::ffi::c_int,
        );
        let mut k: glyphid_t = 0 as glyphid_t;
        while (k as usize) < (*subtable).length {
            bk_push(
                b_0,
                bkembed as ::core::ffi::c_int,
                bk_gpos_value((*(*subtable).items.offset(k as isize)).value, format),
                bkover as ::core::ffi::c_int,
            );
            k = k.wrapping_add(1);
        }
        otl_Coverage_free(cov);
        return bk_build_Block(b_0);
    };
}
