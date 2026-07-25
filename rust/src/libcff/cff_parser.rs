#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, memcpy};
unsafe extern "C" {
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sdsempty() -> sds;
    static cff_iIndex: __caryll_elementinterface_cff_Index;
    static cff_iDict: __caryll_elementinterface_cff_Dict;
    fn cff_close_Charset(cset: cff_Charset);
    fn cff_extract_Charset(
        data: *mut u8,
        offset: i32,
        nchars: u16,
        charsets: *mut cff_Charset,
    );
    fn cff_close_FDSelect(fds: cff_FDSelect);
    fn cff_extract_FDSelect(
        data: *mut u8,
        offset: i32,
        nchars: u16,
        fdselect: *mut cff_FDSelect,
    );
    fn cff_decodeCS2Token(start: *const u8, val: *mut cff_Value) -> u32;
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};

use crate::support::options::{otfcc_Options};
use crate::support::primitives::{arity_t};
use crate::vendor::sds::{Hex4, sds};
use crate::libcff::{cff_Encoding, cff_EncodingRangeFormat1, cff_EncodingSupplement, cff_File, cff_IOutlineBuilder, cff_Stack, op_CharStrings, op_Encoding, op_FDArray, op_FDSelect, op_Private, op_Subrs, op_abs, op_add, op_and, op_callgsubr, op_callsubr, op_charset, op_cntrmask, op_div, op_drop, op_dup, op_eq, op_exch, op_flex, op_flex1, op_get, op_hflex, op_hflex1, op_hmoveto, op_ifelse, op_index, op_mul, op_neg, op_not, op_or, op_put, op_rmoveto, op_roll, op_sqrt, op_sub, op_vmoveto, op_vstem, op_vstemhm, type2_transient_array};
use crate::libcff::cff_charset::{cff_CHARSET_UNSPECED, cff_Charset};
use crate::libcff::cff_dict::{__caryll_elementinterface_cff_Dict};
use crate::libcff::cff_fdselect::{cff_FDSELECT_FORMAT0, cff_FDSELECT_FORMAT3, cff_FDSELECT_UNSPECED, cff_FDSelect};
use crate::libcff::cff_index::{__caryll_elementinterface_cff_Index, cff_Index};
use crate::libcff::cff_value::{cff_DOUBLE, cff_Value, cff_ValueBody, cff_Value_Type};

/// Which encoding a CFF font carries: one of the two predefined ones, or the
/// format of an embedded encoding. Again the crate's own classification rather
/// than anything read from the file -- though `cff_extract_Encoding` does lean
/// on the numbering, comparing the *offset* from the Top DICT against
/// `cff_ENC_STANDARD`/`cff_ENC_EXPERT`, which the spec assigns 0 and 1.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum cff_EncodingType {
    cff_ENC_STANDARD = 0,
    cff_ENC_EXPERT = 1,
    cff_ENC_FORMAT0 = 2,
    cff_ENC_FORMAT1 = 3,
    cff_ENC_FORMAT_SUPPLEMENT = 4,
    cff_ENC_UNSPECED = 5,
}
pub use cff_EncodingType::*;
#[inline]
unsafe extern "C" fn gu1(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 = *s.offset(p as isize) as u32;
    return b0;
}
#[inline]
unsafe extern "C" fn gu2(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
    let mut b1: u32 = *s
        .offset(p as isize)
        .offset(1 as ::core::ffi::c_int as isize) as u32;
    return b0 | b1;
}
unsafe extern "C" fn parse_encoding(
    mut cff: *mut cff_File,
    mut offset: i32,
    mut enc: *mut cff_Encoding,
) {
    let mut data: *mut u8 = (*cff).raw_data;
    if offset == cff_ENC_STANDARD as ::core::ffi::c_int as i32 {
        (*enc).t = cff_ENC_STANDARD;
    } else if offset == cff_ENC_EXPERT as ::core::ffi::c_int as i32 {
        (*enc).t = cff_ENC_EXPERT;
    } else {
        match *data.offset(offset as isize) as ::core::ffi::c_int {
            0 => {
                (*enc).t = cff_ENC_FORMAT0;
                (*enc).c2rust_unnamed.f0.format = 0 as u8;
                (*enc).c2rust_unnamed.f0.ncodes = *data.offset((offset + 1 as i32) as isize);
                (*enc).c2rust_unnamed.f0.code = __caryll_allocate_clean(
                    (::core::mem::size_of::<u8>() as usize)
                        .wrapping_mul((*enc).c2rust_unnamed.f0.ncodes as usize),
                    30 as ::core::ffi::c_ulong,
                ) as *mut u8;
                let mut i: u32 = 0 as u32;
                while i < (*enc).c2rust_unnamed.f0.ncodes as u32 {
                    *(*enc).c2rust_unnamed.f0.code.offset(i as isize) = *data
                        .offset(((offset + 2 as i32) as u32).wrapping_add(i) as isize);
                    i = i.wrapping_add(1);
                }
            }
            1 => {
                (*enc).t = cff_ENC_FORMAT1;
                (*enc).c2rust_unnamed.f1.format = 1 as u8;
                (*enc).c2rust_unnamed.f1.nranges = *data.offset((offset + 1 as i32) as isize);
                (*enc).c2rust_unnamed.f1.range1 = __caryll_allocate_clean(
                    (::core::mem::size_of::<cff_EncodingRangeFormat1>() as usize)
                        .wrapping_mul((*enc).c2rust_unnamed.f1.nranges as usize),
                    41 as ::core::ffi::c_ulong,
                )
                    as *mut cff_EncodingRangeFormat1;
                let mut i_0: u32 = 0 as u32;
                while i_0 < (*enc).c2rust_unnamed.f1.nranges as u32 {
                    (*(*enc).c2rust_unnamed.f1.range1.offset(i_0 as isize)).first = *data.offset(
                        ((offset + 2 as i32) as u32)
                            .wrapping_add(i_0.wrapping_mul(2 as u32))
                            as isize,
                    );
                    (*(*enc).c2rust_unnamed.f1.range1.offset(i_0 as isize)).nleft = *data.offset(
                        ((offset + 3 as i32) as u32)
                            .wrapping_add(i_0.wrapping_mul(2 as u32))
                            as isize,
                    );
                    i_0 = i_0.wrapping_add(1);
                }
            }
            _ => {
                (*enc).t = cff_ENC_FORMAT_SUPPLEMENT;
                (*enc).c2rust_unnamed.ns.nsup = *data.offset(offset as isize);
                (*enc).c2rust_unnamed.ns.supplement = __caryll_allocate_clean(
                    (::core::mem::size_of::<cff_EncodingSupplement>() as usize)
                        .wrapping_mul((*enc).c2rust_unnamed.ns.nsup as usize),
                    52 as ::core::ffi::c_ulong,
                )
                    as *mut cff_EncodingSupplement;
                let mut i_1: u32 = 0 as u32;
                while i_1 < (*enc).c2rust_unnamed.ns.nsup as u32 {
                    (*(*enc).c2rust_unnamed.ns.supplement.offset(i_1 as isize)).code = *data
                        .offset(
                            ((offset + 1 as i32) as u32)
                                .wrapping_add(i_1.wrapping_mul(3 as u32))
                                as isize,
                        );
                    (*(*enc).c2rust_unnamed.ns.supplement.offset(i_1 as isize)).glyph = gu2(
                        data,
                        ((offset + 2 as i32) as u32)
                            .wrapping_add(i_1.wrapping_mul(3 as u32)),
                    )
                        as u16;
                    i_1 = i_1.wrapping_add(1);
                }
            }
        }
    };
}
unsafe extern "C" fn parse_cff_bytecode(mut cff: *mut cff_File, mut options: *const otfcc_Options) {
    let mut pos: u32 = 0;
    let mut offset: i32 = 0;
    (*cff).head.major = gu1((*cff).raw_data, 0 as u32) as u8;
    (*cff).head.minor = gu1((*cff).raw_data, 1 as u32) as u8;
    (*cff).head.hdrSize = gu1((*cff).raw_data, 2 as u32) as u8;
    (*cff).head.offSize = gu1((*cff).raw_data, 3 as u32) as u8;
    pos = (*cff).head.hdrSize as u32;
    cff_iIndex.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).name,
    );
    pos = (4 as u32).wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
        &raw mut (*cff).name,
    ));
    cff_iIndex.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).top_dict,
    );
    if (*cff).name.count != (*cff).top_dict.count {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            log_vl_important as ::core::ffi::c_int as u8,
            log_type_warning,
            crate::sdsbuild!(
                sdsempty(),
                b"[libcff] Bad CFF font: (",
                (*cff).name.count,
                b", name), (",
                (*cff).top_dict.count,
                b", top_dict).\n",
            ),
        );
    }
    pos = (4 as u32)
        .wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
            &raw mut (*cff).name,
        ))
        .wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
            &raw mut (*cff).top_dict,
        ));
    cff_iIndex.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).string,
    );
    pos = (4 as u32)
        .wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
            &raw mut (*cff).name,
        ))
        .wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
            &raw mut (*cff).top_dict,
        ))
        .wrapping_add(cff_iIndex.getLength.expect("non-null function pointer")(
            &raw mut (*cff).string,
        ));
    cff_iIndex.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).global_subr,
    );
    if !(*cff).top_dict.data.is_null() {
        let mut offset_0: i32 = 0;
        offset_0 = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_CharStrings as ::core::ffi::c_int as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            cff_iIndex.parse.expect("non-null function pointer")(
                (*cff).raw_data,
                offset_0 as u32,
                &raw mut (*cff).char_strings,
            );
            (*cff).cnt_glyph = (*cff).char_strings.count as u16;
        } else {
            cff_iIndex.empty.expect("non-null function pointer")(&raw mut (*cff).char_strings);
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger,
                log_vl_important as ::core::ffi::c_int as u8,
                log_type_warning,
                crate::sdsbuild!(sdsempty(), b"[libcff] Bad CFF font: no any glyph data.\n"),
            );
        }
        offset_0 = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_Encoding as ::core::ffi::c_int as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            parse_encoding(cff, offset_0, &raw mut (*cff).encodings);
        } else {
            (*cff).encodings.t = cff_ENC_UNSPECED;
        }
        offset_0 = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_charset as ::core::ffi::c_int as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            cff_extract_Charset(
                (*cff).raw_data,
                offset_0,
                (*cff).char_strings.count as u16,
                &raw mut (*cff).charsets,
            );
        } else {
            (*cff).charsets.t = cff_CHARSET_UNSPECED;
        }
        offset_0 = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_FDSelect as ::core::ffi::c_int as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if (*cff).char_strings.count != 0 && offset_0 != -(1 as i32) {
            cff_extract_FDSelect(
                (*cff).raw_data,
                offset_0,
                (*cff).char_strings.count as u16,
                &raw mut (*cff).fdselect,
            );
        } else {
            (*cff).fdselect.t = cff_FDSELECT_UNSPECED;
        }
        offset_0 = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_FDArray as ::core::ffi::c_int as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            cff_iIndex.parse.expect("non-null function pointer")(
                (*cff).raw_data,
                offset_0 as u32,
                &raw mut (*cff).font_dict,
            );
        } else {
            cff_iIndex.empty.expect("non-null function pointer")(&raw mut (*cff).font_dict);
        }
    }
    let mut private_len: i32 = -(1 as i32);
    let mut private_off: i32 = -(1 as i32);
    if !(*cff).top_dict.data.is_null() {
        private_len = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_Private as ::core::ffi::c_int as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        private_off = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).top_dict.data,
            (*(*cff)
                .top_dict
                .offset
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            op_Private as ::core::ffi::c_int as u32,
            1 as u32,
        )
        .c2rust_unnamed
        .i;
    }
    if private_off != -(1 as i32) && private_len != -(1 as i32) {
        offset = cff_iDict.parseDictKey.expect("non-null function pointer")(
            (*cff).raw_data.offset(private_off as isize),
            private_len as u32,
            op_Subrs as ::core::ffi::c_int as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset != -(1 as i32) {
            cff_iIndex.parse.expect("non-null function pointer")(
                (*cff).raw_data,
                (private_off + offset) as u32,
                &raw mut (*cff).local_subr,
            );
        } else {
            cff_iIndex.empty.expect("non-null function pointer")(&raw mut (*cff).local_subr);
        }
    } else {
        cff_iIndex.empty.expect("non-null function pointer")(&raw mut (*cff).local_subr);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cff_openStream(
    mut data: *mut u8,
    mut len: u32,
    mut options: *const otfcc_Options,
) -> *mut cff_File {
    let mut file: *mut cff_File = ::core::ptr::null_mut::<cff_File>();
    file = __caryll_allocate_clean(
        ::core::mem::size_of::<cff_File>() as usize,
        203 as ::core::ffi::c_ulong,
    ) as *mut cff_File;
    (*file).raw_data = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul(len as usize),
        205 as ::core::ffi::c_ulong,
    ) as *mut u8;
    memcpy(
        (*file).raw_data as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        len as usize,
    );
    (*file).raw_length = len;
    (*file).cnt_glyph = 0 as u16;
    parse_cff_bytecode(file, options);
    return file;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cff_close(mut file: *mut cff_File) {
    if !file.is_null() {
        if !(*file).raw_data.is_null() {
            free((*file).raw_data as *mut ::core::ffi::c_void);
            (*file).raw_data = ::core::ptr::null_mut::<u8>();
        }
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).name);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).top_dict);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).string);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).global_subr);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).char_strings);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).font_dict);
        cff_iIndex.dispose.expect("non-null function pointer")(&raw mut (*file).local_subr);
        match (*file).encodings.t {
            cff_ENC_FORMAT0 => {
                if !(*file).encodings.c2rust_unnamed.f0.code.is_null() {
                    free((*file).encodings.c2rust_unnamed.f0.code as *mut ::core::ffi::c_void);
                    (*file).encodings.c2rust_unnamed.f0.code = ::core::ptr::null_mut::<u8>();
                }
            }
            cff_ENC_FORMAT1 => {
                if !(*file).encodings.c2rust_unnamed.f1.range1.is_null() {
                    free((*file).encodings.c2rust_unnamed.f1.range1 as *mut ::core::ffi::c_void);
                    (*file).encodings.c2rust_unnamed.f1.range1 =
                        ::core::ptr::null_mut::<cff_EncodingRangeFormat1>();
                }
            }
            cff_ENC_FORMAT_SUPPLEMENT => {
                if !(*file).encodings.c2rust_unnamed.ns.supplement.is_null() {
                    free(
                        (*file).encodings.c2rust_unnamed.ns.supplement as *mut ::core::ffi::c_void,
                    );
                    (*file).encodings.c2rust_unnamed.ns.supplement =
                        ::core::ptr::null_mut::<cff_EncodingSupplement>();
                }
            }
            _ => {}
        }
        cff_close_Charset((*file).charsets);
        cff_close_FDSelect((*file).fdselect);
        free(file as *mut ::core::ffi::c_void);
        file = ::core::ptr::null_mut::<cff_File>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cff_parseSubr(
    mut idx: u16,
    mut raw: *mut u8,
    mut fdarray: cff_Index,
    mut select: cff_FDSelect,
    mut subr: *mut cff_Index,
) -> u8 {
    let mut fd: u8 = 0 as u8;
    let mut off_private: i32 = 0;
    let mut len_private: i32 = 0;
    let mut off_subr: i32 = 0;
    match select.t {
        cff_FDSELECT_FORMAT0 => {
            fd = *select.c2rust_unnamed.f0.fds.offset(idx as isize);
        }
        cff_FDSELECT_FORMAT3 => {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < select.c2rust_unnamed.f3.nranges as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
            {
                if idx as ::core::ffi::c_int
                    >= (*select.c2rust_unnamed.f3.range3.offset(i as isize)).first
                        as ::core::ffi::c_int
                    && (idx as ::core::ffi::c_int)
                        < (*select
                            .c2rust_unnamed
                            .f3
                            .range3
                            .offset((i + 1 as ::core::ffi::c_int) as isize))
                        .first as ::core::ffi::c_int
                {
                    fd = (*select.c2rust_unnamed.f3.range3.offset(i as isize)).fd;
                }
                i += 1;
            }
            if idx as ::core::ffi::c_int
                >= (*select.c2rust_unnamed.f3.range3.offset(
                    (select.c2rust_unnamed.f3.nranges as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as isize,
                ))
                .first as ::core::ffi::c_int
                && (idx as ::core::ffi::c_int)
                    < select.c2rust_unnamed.f3.sentinel as ::core::ffi::c_int
            {
                fd = (*select.c2rust_unnamed.f3.range3.offset(
                    (select.c2rust_unnamed.f3.nranges as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as isize,
                ))
                .fd;
            }
        }
        cff_FDSELECT_UNSPECED => {
            fd = 0 as u8;
        }
    }
    off_private = cff_iDict.parseDictKey.expect("non-null function pointer")(
        fdarray
            .data
            .offset(*fdarray.offset.offset(fd as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*fdarray
            .offset
            .offset((fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*fdarray.offset.offset(fd as isize)),
        op_Private as ::core::ffi::c_int as u32,
        1 as u32,
    )
    .c2rust_unnamed
    .i;
    len_private = cff_iDict.parseDictKey.expect("non-null function pointer")(
        fdarray
            .data
            .offset(*fdarray.offset.offset(fd as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*fdarray
            .offset
            .offset((fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*fdarray.offset.offset(fd as isize)),
        op_Private as ::core::ffi::c_int as u32,
        0 as u32,
    )
    .c2rust_unnamed
    .i;
    if off_private != -(1 as i32) && len_private != -(1 as i32) {
        off_subr = cff_iDict.parseDictKey.expect("non-null function pointer")(
            raw.offset(off_private as isize),
            len_private as u32,
            op_Subrs as ::core::ffi::c_int as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if off_subr != -(1 as i32) {
            cff_iIndex.parse.expect("non-null function pointer")(
                raw,
                (off_private + off_subr) as u32,
                subr,
            );
        } else {
            cff_iIndex.empty.expect("non-null function pointer")(subr);
        }
    } else {
        cff_iIndex.empty.expect("non-null function pointer")(subr);
    }
    return fd;
}
#[inline]
unsafe extern "C" fn compute_subr_bias(mut cnt: u16) -> u16 {
    if (cnt as ::core::ffi::c_int) < 1240 as ::core::ffi::c_int {
        return 107 as u16;
    } else if (cnt as ::core::ffi::c_int) < 33900 as ::core::ffi::c_int {
        return 1131 as u16;
    } else {
        return 32768 as u16;
    };
}
unsafe extern "C" fn reverseStack(
    mut stack: *mut cff_Stack,
    mut left: u8,
    mut right: u8,
) {
    let mut p1: *mut cff_Value = (*stack).stack.offset(left as ::core::ffi::c_int as isize);
    let mut p2: *mut cff_Value = (*stack).stack.offset(right as ::core::ffi::c_int as isize);
    while p1 < p2 {
        let mut temp: cff_Value = *p1;
        *p1 = *p2;
        *p2 = temp;
        p1 = p1.offset(1);
        p2 = p2.offset(-1);
    }
}
unsafe extern "C" fn callback_nopSetWidth(
    mut _context: *mut ::core::ffi::c_void,
    mut _width: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nopNewContour(mut _context: *mut ::core::ffi::c_void) {}
unsafe extern "C" fn callback_nopLineTo(
    mut _context: *mut ::core::ffi::c_void,
    mut _x1: ::core::ffi::c_double,
    mut _y1: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nopCurveTo(
    mut _context: *mut ::core::ffi::c_void,
    mut _x1: ::core::ffi::c_double,
    mut _y1: ::core::ffi::c_double,
    mut _x2: ::core::ffi::c_double,
    mut _y2: ::core::ffi::c_double,
    mut _x3: ::core::ffi::c_double,
    mut _y3: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nopsetHint(
    mut _context: *mut ::core::ffi::c_void,
    mut _isVertical: bool,
    mut _position: ::core::ffi::c_double,
    mut _width: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nopsetMask(
    mut _context: *mut ::core::ffi::c_void,
    mut _isContourMask: bool,
    mut mask: *mut bool,
) {
    free(mask as *mut ::core::ffi::c_void);
    mask = ::core::ptr::null_mut::<bool>();
}
unsafe extern "C" fn callback_nopgetrand(
    mut _context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_double {
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cff_parseOutline(
    mut data: *mut u8,
    mut len: u32,
    mut gsubr: cff_Index,
    mut lsubr: cff_Index,
    mut stack: *mut cff_Stack,
    mut outline: *mut ::core::ffi::c_void,
    mut methods: cff_IOutlineBuilder,
    mut options: *const otfcc_Options,
) {
    let mut gsubr_bias: u16 = compute_subr_bias(gsubr.count as u16);
    let mut lsubr_bias: u16 = compute_subr_bias(lsubr.count as u16);
    let mut start: *mut u8 = data;
    let mut advance: u32 = 0;
    let mut i: u32 = 0;
    let mut cnt_bezier: u32 = 0;
    let mut val: cff_Value = cff_Value {
        t: 0 as cff_Value_Type,
        c2rust_unnamed: cff_ValueBody { i: 0 },
    };
    let mut setWidth: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> (),
    > = methods.setWidth;
    let mut newContour: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> =
        methods.newContour;
    let mut lineTo: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    > = methods.lineTo;
    let mut curveTo: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    > = methods.curveTo;
    let mut setHint: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            bool,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    > = methods.setHint;
    let mut setMask: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> ()> =
        methods.setMask;
    let mut getrand: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double,
    > = methods.getrand;
    if setWidth.is_none() {
        setWidth = Some(
            callback_nopSetWidth
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> (),
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> ()>;
    }
    if newContour.is_none() {
        newContour =
            Some(callback_nopNewContour as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    }
    if lineTo.is_none() {
        lineTo = Some(
            callback_nopLineTo
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
            >;
    }
    if curveTo.is_none() {
        curveTo = Some(
            callback_nopCurveTo
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
            >;
    }
    if setHint.is_none() {
        setHint = Some(
            callback_nopsetHint
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    bool,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    bool,
                    ::core::ffi::c_double,
                    ::core::ffi::c_double,
                ) -> (),
            >;
    }
    if setMask.is_none() {
        setMask = Some(
            callback_nopsetMask
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> (),
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> ()>;
    }
    if getrand.is_none() {
        getrand = Some(
            callback_nopgetrand
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double,
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double>;
    }
    while start < data.offset(len as isize) {
        advance = cff_decodeCS2Token(start, &raw mut val);
        match val.t as ::core::ffi::c_uint {
            1 => {
                let mut hintBase: ::core::ffi::c_double = 0.;
                match val.c2rust_unnamed.i {
                    1 | 3 | 18 | 23 => {
                        if (*stack).index.wrapping_rem(2 as arity_t) != 0 {
                            setWidth.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                        }
                        (*stack).stem = ((*stack).stem as arity_t)
                            .wrapping_add((*stack).index >> 1 as ::core::ffi::c_int)
                            as u8 as u8;
                        hintBase = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut j: u16 = (*stack).index.wrapping_rem(2 as arity_t) as u16;
                        while (j as arity_t) < (*stack).index {
                            let mut pos: ::core::ffi::c_double =
                                (*(*stack).stack.offset(j as isize)).c2rust_unnamed.d;
                            let mut width: ::core::ffi::c_double = (*(*stack).stack.offset(
                                (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .c2rust_unnamed
                            .d;
                            setHint.expect("non-null function pointer")(
                                outline,
                                val.c2rust_unnamed.i == op_vstem as ::core::ffi::c_int as i32
                                    || val.c2rust_unnamed.i
                                        == op_vstemhm as ::core::ffi::c_int as i32,
                                pos + hintBase,
                                width,
                            );
                            hintBase += pos + width;
                            j = (j as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u16;
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    19 | 20 => {
                        if (*stack).index.wrapping_rem(2 as arity_t) != 0 {
                            setWidth.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                        }
                        let mut isVertical: bool =
                            (*stack).stem as ::core::ffi::c_int > 0 as ::core::ffi::c_int;
                        (*stack).stem = ((*stack).stem as arity_t)
                            .wrapping_add((*stack).index >> 1 as ::core::ffi::c_int)
                            as u8 as u8;
                        let mut hintBase_0: ::core::ffi::c_double =
                            0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut j_0: u16 =
                            (*stack).index.wrapping_rem(2 as arity_t) as u16;
                        while (j_0 as arity_t) < (*stack).index {
                            let mut pos_0: ::core::ffi::c_double =
                                (*(*stack).stack.offset(j_0 as isize)).c2rust_unnamed.d;
                            let mut width_0: ::core::ffi::c_double = (*(*stack).stack.offset(
                                (j_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .c2rust_unnamed
                            .d;
                            setHint.expect("non-null function pointer")(
                                outline,
                                isVertical,
                                pos_0 + hintBase_0,
                                width_0,
                            );
                            hintBase_0 += pos_0 + width_0;
                            j_0 = (j_0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u16;
                        }
                        let mut maskLength: u32 =
                            ((*stack).stem as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int) as u32;
                        let mut mask: *mut bool = ::core::ptr::null_mut::<bool>();
                        mask = __caryll_allocate_clean(
                            (::core::mem::size_of::<bool>() as usize).wrapping_mul(
                                ((*stack).stem as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
                                    as usize,
                            ),
                            405 as ::core::ffi::c_ulong,
                        ) as *mut bool;
                        let mut byte: u32 = 0 as u32;
                        while byte < maskLength {
                            let mut maskByte: u8 =
                                *start.offset(advance.wrapping_add(byte) as isize);
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(0 as u32)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 7 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(1 as u32)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(2 as u32)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 5 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(3 as u32)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(4 as u32)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(5 as u32)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(6 as u32)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(7 as u32)
                                    as isize,
                            ) = maskByte as ::core::ffi::c_int >> 0 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            byte = byte.wrapping_add(1);
                        }
                        setMask.expect("non-null function pointer")(
                            outline,
                            val.c2rust_unnamed.i == op_cntrmask as ::core::ffi::c_int as i32,
                            mask,
                        );
                        advance = advance.wrapping_add(maskLength);
                        (*stack).index = 0 as arity_t;
                    }
                    4 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_vmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_vmoveto as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 1 as arity_t {
                                setWidth
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            newContour.expect("non-null function pointer")(outline);
                            lineTo.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    21 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_rmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_rmoveto as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 2 as arity_t {
                                setWidth
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            newContour.expect("non-null function pointer")(outline);
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    22 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_hmoveto as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 1 as arity_t {
                                setWidth
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            newContour.expect("non-null function pointer")(outline);
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    14 => {
                        if (*stack).index > 0 as arity_t {
                            setWidth.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                    }
                    5 => {
                        i = 0 as u32;
                        while i < (*stack).index {
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(1 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            i = i.wrapping_add(2 as u32);
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    7 => {
                        if (*stack).index.wrapping_rem(2 as arity_t) == 1 as arity_t {
                            lineTo.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            i = 1 as u32;
                            while i < (*stack).index {
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                );
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                                i = i.wrapping_add(2 as u32);
                            }
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index {
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                );
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                                i = i.wrapping_add(2 as u32);
                            }
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    6 => {
                        if (*stack).index.wrapping_rem(2 as arity_t) == 1 as arity_t {
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            i = 1 as u32;
                            while i < (*stack).index {
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                );
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                                i = i.wrapping_add(2 as u32);
                            }
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index {
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                );
                                lineTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                                i = i.wrapping_add(2 as u32);
                            }
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    8 => {
                        i = 0 as u32;
                        while i < (*stack).index {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(1 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(2 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(3 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(4 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(5 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            i = i.wrapping_add(6 as u32);
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    24 => {
                        i = 0 as u32;
                        while i < (*stack).index.wrapping_sub(2 as arity_t) {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(1 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(2 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(3 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(4 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(5 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            i = i.wrapping_add(6 as u32);
                        }
                        lineTo.expect("non-null function pointer")(
                            outline,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                        );
                        (*stack).index = 0 as arity_t;
                    }
                    25 => {
                        i = 0 as u32;
                        while i < (*stack).index.wrapping_sub(6 as arity_t) {
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack
                                    .offset(i.wrapping_add(1 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            i = i.wrapping_add(2 as u32);
                        }
                        curveTo.expect("non-null function pointer")(
                            outline,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(6 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(5 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d,
                        );
                        (*stack).index = 0 as arity_t;
                    }
                    26 => {
                        if (*stack).index.wrapping_rem(4 as arity_t) == 1 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            i = 5 as u32;
                            while i < (*stack).index {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                                i = i.wrapping_add(4 as u32);
                            }
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                                i = i.wrapping_add(4 as u32);
                            }
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    27 => {
                        if (*stack).index.wrapping_rem(4 as arity_t) == 1 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            i = 5 as u32;
                            while i < (*stack).index {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                                i = i.wrapping_add(4 as u32);
                            }
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                                i = i.wrapping_add(4 as u32);
                            }
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    30 => {
                        if (*stack).index.wrapping_rem(4 as arity_t) == 1 as arity_t {
                            cnt_bezier = (*stack)
                                .index
                                .wrapping_sub(5 as arity_t)
                                .wrapping_div(4 as arity_t)
                                as u32;
                        } else {
                            cnt_bezier = (*stack).index.wrapping_div(4 as arity_t) as u32;
                        }
                        i = 0 as u32;
                        while i < (4 as u32).wrapping_mul(cnt_bezier) {
                            if i.wrapping_div(4 as u32).wrapping_rem(2 as u32)
                                == 0 as u32
                            {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                            } else {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                            }
                            i = i.wrapping_add(4 as u32);
                        }
                        if (*stack).index.wrapping_rem(8 as arity_t) == 5 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        if (*stack).index.wrapping_rem(8 as arity_t) == 1 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    31 => {
                        if (*stack).index.wrapping_rem(4 as arity_t) == 1 as arity_t {
                            cnt_bezier = (*stack)
                                .index
                                .wrapping_sub(5 as arity_t)
                                .wrapping_div(4 as arity_t)
                                as u32;
                        } else {
                            cnt_bezier = (*stack).index.wrapping_div(4 as arity_t) as u32;
                        }
                        i = 0 as u32;
                        while i < (4 as u32).wrapping_mul(cnt_bezier) {
                            if i.wrapping_div(4 as u32).wrapping_rem(2 as u32)
                                == 0 as u32
                            {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                            } else {
                                curveTo.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                            }
                            i = i.wrapping_add(4 as u32);
                        }
                        if (*stack).index.wrapping_rem(8 as arity_t) == 5 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        if (*stack).index.wrapping_rem(8 as arity_t) == 1 as arity_t {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        (*stack).index = 0 as arity_t;
                    }
                    3106 => {
                        if (*stack).index < 7 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hflex\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_hflex as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                -(*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    3107 => {
                        if (*stack).index < 12 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_flex\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_flex as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(7 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(8 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(9 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(10 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(11 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    3108 => {
                        if (*stack).index < 9 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hflex1\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_hflex1 as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(7 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(8 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                -((*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d
                                    + (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(7 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d),
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    3109 => {
                        if (*stack).index < 11 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_flex1\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_flex1 as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut dx: ::core::ffi::c_double =
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d
                                    + (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(6 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(8 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d;
                            let mut dy: ::core::ffi::c_double =
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d
                                    + (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(5 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(7 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.offset(9 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d;
                            if fabs(dx) > fabs(dy) {
                                dx = (*(*stack).stack.offset(10 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d;
                                dy = -dy;
                            } else {
                                dx = -dx;
                                dy = (*(*stack).stack.offset(10 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d;
                            }
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(7 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(8 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.offset(9 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                dx,
                                dy,
                            );
                            (*stack).index = 0 as arity_t;
                        }
                    }
                    3075 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_and\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_and as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if num1 != 0. && num2 != 0. {
                                1.0f64
                            } else {
                                0.0f64
                            };
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3076 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_or\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_or as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if num1_0 != 0. || num2_0 != 0. {
                                1.0f64
                            } else {
                                0.0f64
                            };
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3077 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_not\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_not as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if num != 0. { 0.0f64 } else { 1.0f64 };
                        }
                    }
                    3081 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_abs\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_abs as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if num_0 < 0.0f64 { -num_0 } else { num_0 };
                        }
                    }
                    3082 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_add\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_add as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num1_1 + num2_1;
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3083 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_sub\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_sub as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num1_2 - num2_2;
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3084 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_div\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_div as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_3: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_3: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num1_3 / num2_3;
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3086 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_neg\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_neg as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num_1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = -num_1;
                        }
                    }
                    3087 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_eq\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_eq as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_4: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_4: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if num1_4 == num2_4 { 1.0f64 } else { 0.0f64 };
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3090 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_drop\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_drop as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3092 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_put\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_put as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut val_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut i_0: i32 = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d as i32;
                            (*stack).transient[(i_0
                                % type2_transient_array as ::core::ffi::c_int as i32)
                                as usize]
                                .c2rust_unnamed
                                .d = val_0;
                            (*stack).index = (*stack).index.wrapping_sub(2 as arity_t);
                        }
                    }
                    3093 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_get\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_get as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut i_1: i32 = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d as i32;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = (*stack).transient[(i_1
                                % type2_transient_array as ::core::ffi::c_int as i32)
                                as usize]
                                .c2rust_unnamed
                                .d;
                        }
                    }
                    3094 => {
                        if (*stack).index < 4 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_ifelse\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_ifelse as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut v2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut v1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut s2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(3 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut s1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(4 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = if v1 <= v2 { s1 } else { s2 };
                            (*stack).index = (*stack).index.wrapping_sub(3 as arity_t);
                        }
                    }
                    3095 => {
                        (*(*stack).stack.offset((*stack).index as isize)).t = cff_DOUBLE;
                        (*(*stack).stack.offset((*stack).index as isize))
                            .c2rust_unnamed
                            .d = getrand.expect("non-null function pointer")(outline);
                        (*stack).index = (*stack).index.wrapping_add(1 as arity_t);
                    }
                    3096 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_mul\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_mul as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_5: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_5: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num1_5 * num2_5;
                            (*stack).index = (*stack).index.wrapping_sub(1 as arity_t);
                        }
                    }
                    3098 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_sqrt\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_sqrt as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num_2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = sqrt(num_2);
                        }
                    }
                    3099 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_dup\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_dup as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            *(*stack).stack.offset((*stack).index as isize) = *(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize);
                            (*stack).index = (*stack).index.wrapping_add(1 as arity_t);
                        }
                    }
                    3100 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_exch\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_exch as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_6: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_6: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num2_6;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d = num1_6;
                        }
                    }
                    3101 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_index\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_index as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut n: u8 =
                                (*stack).index.wrapping_sub(1 as arity_t) as u8;
                            let mut j_1: u8 = (n as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - (*(*stack).stack.offset(n as isize)).c2rust_unnamed.d as u8
                                    as ::core::ffi::c_int
                                    % n as ::core::ffi::c_int)
                                as u8;
                            *(*stack).stack.offset(n as isize) =
                                *(*stack).stack.offset(j_1 as isize);
                        }
                    }
                    3102 => {
                        if (*stack).index < 2 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_roll\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_roll as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut j_2: i32 = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as arity_t) as isize))
                            .c2rust_unnamed
                            .d as i32;
                            let mut n_0: u32 = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as arity_t) as isize))
                            .c2rust_unnamed
                            .d as u32;
                            if (*stack).index < (2 as u32).wrapping_add(n_0) {
                                (*(*options).logger)
                                    .logSDS
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*options).logger as *mut otfcc_ILogger,
                                    log_vl_important as ::core::ffi::c_int as u8,
                                    log_type_warning,
                                    crate::sdsbuild!(
                                        sdsempty(),
                                        b"[libcff] Stack cannot provide enough parameters for ",
                                        b"op_roll\0" as *const u8 as *const ::core::ffi::c_char,
                                        b" (",
                                        Hex4((op_roll as ::core::ffi::c_int) as u32),
                                        b"). This operation is ignored.\n",
                                    ),
                                );
                            } else {
                                j_2 = (-j_2 as u32).wrapping_rem(n_0) as i32;
                                if j_2 < 0 as i32 {
                                    j_2 = (j_2 as u32).wrapping_add(n_0) as i32 as i32;
                                }
                                if !(j_2 == 0) {
                                    let mut last: u8 =
                                        (*stack).index.wrapping_sub(3 as arity_t) as u8;
                                    let mut first: u8 = (*stack)
                                        .index
                                        .wrapping_sub(2 as arity_t)
                                        .wrapping_sub(n_0 as arity_t)
                                        as u8;
                                    reverseStack(stack, first, last);
                                    reverseStack(
                                        stack,
                                        (last as i32 - j_2 + 1 as i32) as u8,
                                        last,
                                    );
                                    reverseStack(stack, first, (last as i32 - j_2) as u8);
                                    (*stack).index = (*stack).index.wrapping_sub(2 as arity_t);
                                }
                            }
                        }
                    }
                    11 => return,
                    10 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_callsubr\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_callsubr as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1);
                            let mut subr: u32 =
                                (*(*stack).stack.offset((*stack).index as isize))
                                    .c2rust_unnamed
                                    .d as u32;
                            cff_parseOutline(
                                lsubr
                                    .data
                                    .offset(
                                        *lsubr
                                            .offset
                                            .offset((lsubr_bias as u32).wrapping_add(subr)
                                                as isize)
                                            as isize,
                                    )
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                (*lsubr.offset.offset(
                                    (lsubr_bias as u32)
                                        .wrapping_add(subr)
                                        .wrapping_add(1 as u32)
                                        as isize,
                                ))
                                .wrapping_sub(
                                    *lsubr.offset.offset(
                                        (lsubr_bias as u32).wrapping_add(subr) as isize,
                                    ),
                                ),
                                gsubr,
                                lsubr,
                                stack,
                                outline,
                                methods,
                                options,
                            );
                        }
                    }
                    29 => {
                        if (*stack).index < 1 as arity_t {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut otfcc_ILogger,
                                log_vl_important as ::core::ffi::c_int as u8,
                                log_type_warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_callgsubr\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((op_callgsubr as ::core::ffi::c_int) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1);
                            let mut subr_0: u32 =
                                (*(*stack).stack.offset((*stack).index as isize))
                                    .c2rust_unnamed
                                    .d as u32;
                            cff_parseOutline(
                                gsubr
                                    .data
                                    .offset(*gsubr.offset.offset(
                                        (gsubr_bias as u32).wrapping_add(subr_0) as isize,
                                    ) as isize)
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                (*gsubr.offset.offset(
                                    (gsubr_bias as u32)
                                        .wrapping_add(subr_0)
                                        .wrapping_add(1 as u32)
                                        as isize,
                                ))
                                .wrapping_sub(
                                    *gsubr.offset.offset(
                                        (gsubr_bias as u32).wrapping_add(subr_0) as isize,
                                    ),
                                ),
                                gsubr,
                                lsubr,
                                stack,
                                outline,
                                methods,
                                options,
                            );
                        }
                    }
                    _ => {
                        (*(*options).logger)
                            .logSDS
                            .expect(
                                "non-null function pointer",
                            )(
                            (*options).logger as *mut otfcc_ILogger,
                            log_vl_important as ::core::ffi::c_int as u8,
                            log_type_warning,
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Warning: unknown operator ",
                                val.c2rust_unnamed.i,
                                b" occurs in Type 2 CharString. It may caused by file corruption.",
                            ),
                        );
                        return;
                    }
                }
            }
            2 | 3 => {
                let fresh0 = (*stack).index;
                (*stack).index = (*stack).index.wrapping_add(1);
                *(*stack).stack.offset(fresh0 as isize) = val;
            }
            _ => {}
        }
        start = start.offset(advance as isize);
    }
}
