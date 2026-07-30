#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, memcpy};
unsafe extern "C" {
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}


use crate::support::alloc::{__caryll_allocate_clean};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};

use crate::support::options::{Options};
use crate::support::primitives::{Arity};
use crate::vendor::sds::Hex4;
use crate::libcff::{CffEncoding, CffEncodingRangeFormat1, CffEncodingSupplement, CffFile, CffIOutlineBuilder, CffStack, OP_CHAR_STRINGS, OP_ENCODING, OP_FD_ARRAY, OP_FD_SELECT, OP_PRIVATE, OP_SUBRS, OP_ABS, OP_ADD, OP_AND, OP_CALLGSUBR, OP_CALLSUBR, OP_CHARSET, OP_CNTRMASK, OP_DIV, OP_DROP, OP_DUP, OP_EQ, OP_EXCH, OP_FLEX, OP_FLEX1, OP_GET, OP_HFLEX, OP_HFLEX1, OP_HMOVETO, OP_IFELSE, OP_INDEX, OP_MUL, OP_NEG, OP_NOT, OP_OR, OP_PUT, OP_RMOVETO, OP_ROLL, OP_SQRT, OP_SUB, OP_VMOVETO, OP_VSTEM, OP_VSTEMHM, TYPE2_TRANSIENT_ARRAY};
use crate::libcff::cff_charset::CFF_CHARSET_UNSPECED;
use crate::libcff::cff_fdselect::{CffFdSelectType, CffFdSelect};
use crate::libcff::cff_index::CffIndex;
use crate::libcff::cff_value::{CffValueType, CffValue, CffValueBody};
use crate::libcff::cff_charset::{cff_close_Charset, cff_extract_Charset};
use crate::libcff::cff_codecs::{cff_decodeCS2Token};
use crate::libcff::cff_dict::{CFF_I_DICT};
use crate::libcff::cff_fdselect::{cff_close_FDSelect, cff_extract_FDSelect};
use crate::libcff::cff_index::{CFF_I_INDEX};
use crate::vendor::sds::{sdsempty};

/// Which encoding a CFF font carries: one of the two predefined ones, or the
/// format of an embedded encoding. Again the crate's own classification rather
/// than anything read from the file -- though `cff_extract_Encoding` does lean
/// on the numbering, comparing the *offset* from the Top DICT against
/// `CffEncodingType::Standard`/`CffEncodingType::Expert`, which the spec assigns 0 and 1.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CffEncodingType {
    Standard = 0,
    Expert = 1,
    Format0 = 2,
    Format1 = 3,
    FormatSupplement = 4,
    Unspecified = 5,
}
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
    mut cff: *mut CffFile,
    mut offset: i32,
    mut enc: *mut CffEncoding,
) {
    let mut data: *mut u8 = (*cff).raw_data;
    if offset == CffEncodingType::Standard as ::core::ffi::c_int as i32 {
        (*enc).t = CffEncodingType::Standard;
    } else if offset == CffEncodingType::Expert as ::core::ffi::c_int as i32 {
        (*enc).t = CffEncodingType::Expert;
    } else {
        match *data.offset(offset as isize) as ::core::ffi::c_int {
            0 => {
                (*enc).t = CffEncodingType::Format0;
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
                (*enc).t = CffEncodingType::Format1;
                (*enc).c2rust_unnamed.f1.format = 1 as u8;
                (*enc).c2rust_unnamed.f1.nranges = *data.offset((offset + 1 as i32) as isize);
                (*enc).c2rust_unnamed.f1.range1 = __caryll_allocate_clean(
                    (::core::mem::size_of::<CffEncodingRangeFormat1>() as usize)
                        .wrapping_mul((*enc).c2rust_unnamed.f1.nranges as usize),
                    41 as ::core::ffi::c_ulong,
                )
                    as *mut CffEncodingRangeFormat1;
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
                (*enc).t = CffEncodingType::FormatSupplement;
                (*enc).c2rust_unnamed.ns.nsup = *data.offset(offset as isize);
                (*enc).c2rust_unnamed.ns.supplement = __caryll_allocate_clean(
                    (::core::mem::size_of::<CffEncodingSupplement>() as usize)
                        .wrapping_mul((*enc).c2rust_unnamed.ns.nsup as usize),
                    52 as ::core::ffi::c_ulong,
                )
                    as *mut CffEncodingSupplement;
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
unsafe extern "C" fn parse_cff_bytecode(mut cff: *mut CffFile, mut options: *const Options) {
    let mut pos: u32 = 0;
    let mut offset: i32 = 0;
    (*cff).head.major = gu1((*cff).raw_data, 0 as u32) as u8;
    (*cff).head.minor = gu1((*cff).raw_data, 1 as u32) as u8;
    (*cff).head.hdrSize = gu1((*cff).raw_data, 2 as u32) as u8;
    (*cff).head.offSize = gu1((*cff).raw_data, 3 as u32) as u8;
    pos = (*cff).head.hdrSize as u32;
    CFF_I_INDEX.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).name,
    );
    pos = (4 as u32).wrapping_add(CFF_I_INDEX.getLength.expect("non-null function pointer")(
        &raw mut (*cff).name,
    ));
    CFF_I_INDEX.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).top_dict,
    );
    if (*cff).name.count != (*cff).top_dict.count {
        (*(*options).logger)
            .logSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
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
        .wrapping_add(CFF_I_INDEX.getLength.expect("non-null function pointer")(
            &raw mut (*cff).name,
        ))
        .wrapping_add(CFF_I_INDEX.getLength.expect("non-null function pointer")(
            &raw mut (*cff).top_dict,
        ));
    CFF_I_INDEX.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).string,
    );
    pos = (4 as u32)
        .wrapping_add(CFF_I_INDEX.getLength.expect("non-null function pointer")(
            &raw mut (*cff).name,
        ))
        .wrapping_add(CFF_I_INDEX.getLength.expect("non-null function pointer")(
            &raw mut (*cff).top_dict,
        ))
        .wrapping_add(CFF_I_INDEX.getLength.expect("non-null function pointer")(
            &raw mut (*cff).string,
        ));
    CFF_I_INDEX.parse.expect("non-null function pointer")(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).global_subr,
    );
    if !(*cff).top_dict.data.is_null() {
        let mut offset_0: i32 = 0;
        offset_0 = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
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
            OP_CHAR_STRINGS as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            CFF_I_INDEX.parse.expect("non-null function pointer")(
                (*cff).raw_data,
                offset_0 as u32,
                &raw mut (*cff).char_strings,
            );
            (*cff).cnt_glyph = (*cff).char_strings.count as u16;
        } else {
            CFF_I_INDEX.empty.expect("non-null function pointer")(&raw mut (*cff).char_strings);
            (*(*options).logger)
                .logSDS
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::sdsbuild!(sdsempty(), b"[libcff] Bad CFF font: no any glyph data.\n"),
            );
        }
        offset_0 = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
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
            OP_ENCODING as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            parse_encoding(cff, offset_0, &raw mut (*cff).encodings);
        } else {
            (*cff).encodings.t = CffEncodingType::Unspecified;
        }
        offset_0 = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
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
            OP_CHARSET as u32,
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
            (*cff).charsets.t = CFF_CHARSET_UNSPECED;
        }
        offset_0 = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
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
            OP_FD_SELECT as u32,
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
            (*cff).fdselect.t = CffFdSelectType::Unspecified;
        }
        offset_0 = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
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
            OP_FD_ARRAY as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            CFF_I_INDEX.parse.expect("non-null function pointer")(
                (*cff).raw_data,
                offset_0 as u32,
                &raw mut (*cff).font_dict,
            );
        } else {
            CFF_I_INDEX.empty.expect("non-null function pointer")(&raw mut (*cff).font_dict);
        }
    }
    let mut private_len: i32 = -(1 as i32);
    let mut private_off: i32 = -(1 as i32);
    if !(*cff).top_dict.data.is_null() {
        private_len = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
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
            OP_PRIVATE as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        private_off = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
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
            OP_PRIVATE as u32,
            1 as u32,
        )
        .c2rust_unnamed
        .i;
    }
    if private_off != -(1 as i32) && private_len != -(1 as i32) {
        offset = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
            (*cff).raw_data.offset(private_off as isize),
            private_len as u32,
            OP_SUBRS as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset != -(1 as i32) {
            CFF_I_INDEX.parse.expect("non-null function pointer")(
                (*cff).raw_data,
                (private_off + offset) as u32,
                &raw mut (*cff).local_subr,
            );
        } else {
            CFF_I_INDEX.empty.expect("non-null function pointer")(&raw mut (*cff).local_subr);
        }
    } else {
        CFF_I_INDEX.empty.expect("non-null function pointer")(&raw mut (*cff).local_subr);
    };
}
pub unsafe extern "C" fn cff_openStream(
    mut data: *mut u8,
    mut len: u32,
    mut options: *const Options,
) -> *mut CffFile {
    let mut file: *mut CffFile = ::core::ptr::null_mut::<CffFile>();
    file = __caryll_allocate_clean(
        ::core::mem::size_of::<CffFile>() as usize,
        203 as ::core::ffi::c_ulong,
    ) as *mut CffFile;
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
pub unsafe extern "C" fn cff_close(mut file: *mut CffFile) {
    if !file.is_null() {
        if !(*file).raw_data.is_null() {
            free((*file).raw_data as *mut ::core::ffi::c_void);
            (*file).raw_data = ::core::ptr::null_mut::<u8>();
        }
        CFF_I_INDEX.dispose.expect("non-null function pointer")(&raw mut (*file).name);
        CFF_I_INDEX.dispose.expect("non-null function pointer")(&raw mut (*file).top_dict);
        CFF_I_INDEX.dispose.expect("non-null function pointer")(&raw mut (*file).string);
        CFF_I_INDEX.dispose.expect("non-null function pointer")(&raw mut (*file).global_subr);
        CFF_I_INDEX.dispose.expect("non-null function pointer")(&raw mut (*file).char_strings);
        CFF_I_INDEX.dispose.expect("non-null function pointer")(&raw mut (*file).font_dict);
        CFF_I_INDEX.dispose.expect("non-null function pointer")(&raw mut (*file).local_subr);
        match (*file).encodings.t {
            CffEncodingType::Format0 => {
                if !(*file).encodings.c2rust_unnamed.f0.code.is_null() {
                    free((*file).encodings.c2rust_unnamed.f0.code as *mut ::core::ffi::c_void);
                    (*file).encodings.c2rust_unnamed.f0.code = ::core::ptr::null_mut::<u8>();
                }
            }
            CffEncodingType::Format1 => {
                if !(*file).encodings.c2rust_unnamed.f1.range1.is_null() {
                    free((*file).encodings.c2rust_unnamed.f1.range1 as *mut ::core::ffi::c_void);
                    (*file).encodings.c2rust_unnamed.f1.range1 =
                        ::core::ptr::null_mut::<CffEncodingRangeFormat1>();
                }
            }
            CffEncodingType::FormatSupplement => {
                if !(*file).encodings.c2rust_unnamed.ns.supplement.is_null() {
                    free(
                        (*file).encodings.c2rust_unnamed.ns.supplement as *mut ::core::ffi::c_void,
                    );
                    (*file).encodings.c2rust_unnamed.ns.supplement =
                        ::core::ptr::null_mut::<CffEncodingSupplement>();
                }
            }
            _ => {}
        }
        cff_close_Charset((*file).charsets);
        cff_close_FDSelect((*file).fdselect);
        free(file as *mut ::core::ffi::c_void);
        file = ::core::ptr::null_mut::<CffFile>();
    }
}
pub unsafe extern "C" fn cff_parseSubr(
    mut idx: u16,
    mut raw: *mut u8,
    mut fdarray: CffIndex,
    mut select: CffFdSelect,
    mut subr: *mut CffIndex,
) -> u8 {
    let mut fd: u8 = 0 as u8;
    let mut off_private: i32 = 0;
    let mut len_private: i32 = 0;
    let mut off_subr: i32 = 0;
    match select.t {
        CffFdSelectType::Format0 => {
            fd = *select.c2rust_unnamed.f0.fds.offset(idx as isize);
        }
        CffFdSelectType::Format3 => {
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
        CffFdSelectType::Unspecified => {
            fd = 0 as u8;
        }
    }
    off_private = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
        fdarray
            .data
            .offset(*fdarray.offset.offset(fd as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*fdarray
            .offset
            .offset((fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*fdarray.offset.offset(fd as isize)),
        OP_PRIVATE as u32,
        1 as u32,
    )
    .c2rust_unnamed
    .i;
    len_private = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
        fdarray
            .data
            .offset(*fdarray.offset.offset(fd as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*fdarray
            .offset
            .offset((fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*fdarray.offset.offset(fd as isize)),
        OP_PRIVATE as u32,
        0 as u32,
    )
    .c2rust_unnamed
    .i;
    if off_private != -(1 as i32) && len_private != -(1 as i32) {
        off_subr = CFF_I_DICT.parseDictKey.expect("non-null function pointer")(
            raw.offset(off_private as isize),
            len_private as u32,
            OP_SUBRS as u32,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if off_subr != -(1 as i32) {
            CFF_I_INDEX.parse.expect("non-null function pointer")(
                raw,
                (off_private + off_subr) as u32,
                subr,
            );
        } else {
            CFF_I_INDEX.empty.expect("non-null function pointer")(subr);
        }
    } else {
        CFF_I_INDEX.empty.expect("non-null function pointer")(subr);
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
    mut stack: *mut CffStack,
    mut left: u8,
    mut right: u8,
) {
    let mut p1: *mut CffValue = (*stack).stack.offset(left as ::core::ffi::c_int as isize);
    let mut p2: *mut CffValue = (*stack).stack.offset(right as ::core::ffi::c_int as isize);
    while p1 < p2 {
        let mut temp: CffValue = *p1;
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
pub unsafe extern "C" fn cff_parseOutline(
    mut data: *mut u8,
    mut len: u32,
    mut gsubr: CffIndex,
    mut lsubr: CffIndex,
    mut stack: *mut CffStack,
    mut outline: *mut ::core::ffi::c_void,
    mut methods: CffIOutlineBuilder,
    mut options: *const Options,
) {
    let mut gsubr_bias: u16 = compute_subr_bias(gsubr.count as u16);
    let mut lsubr_bias: u16 = compute_subr_bias(lsubr.count as u16);
    let mut start: *mut u8 = data;
    let mut advance: u32 = 0;
    let mut i: u32 = 0;
    let mut cnt_bezier: u32 = 0;
    let mut val: CffValue = CffValue {
        t: CffValueType::Unset,
        c2rust_unnamed: CffValueBody { i: 0 },
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
        match val.t {
            CffValueType::Operator => {
                let mut hintBase: ::core::ffi::c_double = 0.;
                match val.c2rust_unnamed.i {
                    1 | 3 | 18 | 23 => {
                        if (*stack).index.wrapping_rem(2 as Arity) != 0 {
                            setWidth.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                        }
                        (*stack).stem = ((*stack).stem as Arity)
                            .wrapping_add((*stack).index >> 1 as ::core::ffi::c_int)
                            as u8 as u8;
                        hintBase = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut j: u16 = (*stack).index.wrapping_rem(2 as Arity) as u16;
                        while (j as Arity) < (*stack).index {
                            let mut pos: ::core::ffi::c_double =
                                (*(*stack).stack.offset(j as isize)).c2rust_unnamed.d;
                            let mut width: ::core::ffi::c_double = (*(*stack).stack.offset(
                                (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .c2rust_unnamed
                            .d;
                            setHint.expect("non-null function pointer")(
                                outline,
                                val.c2rust_unnamed.i == OP_VSTEM
                                    || val.c2rust_unnamed.i
                                        == OP_VSTEMHM,
                                pos + hintBase,
                                width,
                            );
                            hintBase += pos + width;
                            j = (j as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u16;
                        }
                        (*stack).index = 0 as Arity;
                    }
                    19 | 20 => {
                        if (*stack).index.wrapping_rem(2 as Arity) != 0 {
                            setWidth.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                        }
                        let mut isVertical: bool =
                            (*stack).stem as ::core::ffi::c_int > 0 as ::core::ffi::c_int;
                        (*stack).stem = ((*stack).stem as Arity)
                            .wrapping_add((*stack).index >> 1 as ::core::ffi::c_int)
                            as u8 as u8;
                        let mut hintBase_0: ::core::ffi::c_double =
                            0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut j_0: u16 =
                            (*stack).index.wrapping_rem(2 as Arity) as u16;
                        while (j_0 as Arity) < (*stack).index {
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
                            val.c2rust_unnamed.i == OP_CNTRMASK,
                            mask,
                        );
                        advance = advance.wrapping_add(maskLength);
                        (*stack).index = 0 as Arity;
                    }
                    4 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_vmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_VMOVETO) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 1 as Arity {
                                setWidth
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
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
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    21 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_rmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_RMOVETO) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 2 as Arity {
                                setWidth
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            newContour.expect("non-null function pointer")(outline);
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    22 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_HMOVETO) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 1 as Arity {
                                setWidth
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack
                                        .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            newContour.expect("non-null function pointer")(outline);
                            lineTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    14 => {
                        if (*stack).index > 0 as Arity {
                            setWidth.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
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
                        (*stack).index = 0 as Arity;
                    }
                    7 => {
                        if (*stack).index.wrapping_rem(2 as Arity) == 1 as Arity {
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
                        (*stack).index = 0 as Arity;
                    }
                    6 => {
                        if (*stack).index.wrapping_rem(2 as Arity) == 1 as Arity {
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
                        (*stack).index = 0 as Arity;
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
                        (*stack).index = 0 as Arity;
                    }
                    24 => {
                        i = 0 as u32;
                        while i < (*stack).index.wrapping_sub(2 as Arity) {
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
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                        );
                        (*stack).index = 0 as Arity;
                    }
                    25 => {
                        i = 0 as u32;
                        while i < (*stack).index.wrapping_sub(6 as Arity) {
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
                                .offset((*stack).index.wrapping_sub(6 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(5 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                        );
                        (*stack).index = 0 as Arity;
                    }
                    26 => {
                        if (*stack).index.wrapping_rem(4 as Arity) == 1 as Arity {
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
                        (*stack).index = 0 as Arity;
                    }
                    27 => {
                        if (*stack).index.wrapping_rem(4 as Arity) == 1 as Arity {
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
                        (*stack).index = 0 as Arity;
                    }
                    30 => {
                        if (*stack).index.wrapping_rem(4 as Arity) == 1 as Arity {
                            cnt_bezier = (*stack)
                                .index
                                .wrapping_sub(5 as Arity)
                                .wrapping_div(4 as Arity)
                                as u32;
                        } else {
                            cnt_bezier = (*stack).index.wrapping_div(4 as Arity) as u32;
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
                        if (*stack).index.wrapping_rem(8 as Arity) == 5 as Arity {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        if (*stack).index.wrapping_rem(8 as Arity) == 1 as Arity {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        (*stack).index = 0 as Arity;
                    }
                    31 => {
                        if (*stack).index.wrapping_rem(4 as Arity) == 1 as Arity {
                            cnt_bezier = (*stack)
                                .index
                                .wrapping_sub(5 as Arity)
                                .wrapping_div(4 as Arity)
                                as u32;
                        } else {
                            cnt_bezier = (*stack).index.wrapping_div(4 as Arity) as u32;
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
                        if (*stack).index.wrapping_rem(8 as Arity) == 5 as Arity {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        if (*stack).index.wrapping_rem(8 as Arity) == 1 as Arity {
                            curveTo.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(5 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        (*stack).index = 0 as Arity;
                    }
                    3106 => {
                        if (*stack).index < 7 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hflex\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_HFLEX) as u32),
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
                            (*stack).index = 0 as Arity;
                        }
                    }
                    3107 => {
                        if (*stack).index < 12 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_flex\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_FLEX) as u32),
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
                            (*stack).index = 0 as Arity;
                        }
                    }
                    3108 => {
                        if (*stack).index < 9 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hflex1\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_HFLEX1) as u32),
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
                            (*stack).index = 0 as Arity;
                        }
                    }
                    3109 => {
                        if (*stack).index < 11 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_flex1\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_FLEX1) as u32),
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
                            (*stack).index = 0 as Arity;
                        }
                    }
                    3075 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_and\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_AND) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = if num1 != 0. && num2 != 0. {
                                1.0f64
                            } else {
                                0.0f64
                            };
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3076 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_or\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_OR) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = if num1_0 != 0. || num2_0 != 0. {
                                1.0f64
                            } else {
                                0.0f64
                            };
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3077 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_not\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_NOT) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = if num != 0. { 0.0f64 } else { 1.0f64 };
                        }
                    }
                    3081 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_abs\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_ABS) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = if num_0 < 0.0f64 { -num_0 } else { num_0 };
                        }
                    }
                    3082 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_add\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_ADD) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num1_1 + num2_1;
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3083 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_sub\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_SUB) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num1_2 - num2_2;
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3084 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_div\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_DIV) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_3: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_3: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num1_3 / num2_3;
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3086 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_neg\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_NEG) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num_1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = -num_1;
                        }
                    }
                    3087 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_eq\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_EQ) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_4: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_4: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = if num1_4 == num2_4 { 1.0f64 } else { 0.0f64 };
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3090 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_drop\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_DROP) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3092 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_put\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_PUT) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut val_0: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut i_0: i32 = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d as i32;
                            (*stack).transient[(i_0
                                % TYPE2_TRANSIENT_ARRAY as i32)
                                as usize]
                                .c2rust_unnamed
                                .d = val_0;
                            (*stack).index = (*stack).index.wrapping_sub(2 as Arity);
                        }
                    }
                    3093 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_get\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_GET) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut i_1: i32 = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d as i32;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = (*stack).transient[(i_1
                                % TYPE2_TRANSIENT_ARRAY as i32)
                                as usize]
                                .c2rust_unnamed
                                .d;
                        }
                    }
                    3094 => {
                        if (*stack).index < 4 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_ifelse\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_IFELSE) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut v2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut v1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut s2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut s1: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                            .c2rust_unnamed
                            .d = if v1 <= v2 { s1 } else { s2 };
                            (*stack).index = (*stack).index.wrapping_sub(3 as Arity);
                        }
                    }
                    3095 => {
                        (*(*stack).stack.offset((*stack).index as isize)).t = CffValueType::Double;
                        (*(*stack).stack.offset((*stack).index as isize))
                            .c2rust_unnamed
                            .d = getrand.expect("non-null function pointer")(outline);
                        (*stack).index = (*stack).index.wrapping_add(1 as Arity);
                    }
                    3096 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_mul\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_MUL) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_5: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_5: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num1_5 * num2_5;
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3098 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_sqrt\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_SQRT) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num_2: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = sqrt(num_2);
                        }
                    }
                    3099 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_dup\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_DUP) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            *(*stack).stack.offset((*stack).index as isize) = *(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize);
                            (*stack).index = (*stack).index.wrapping_add(1 as Arity);
                        }
                    }
                    3100 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_exch\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_EXCH) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_6: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_6: ::core::ffi::c_double = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num2_6;
                            (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num1_6;
                        }
                    }
                    3101 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_index\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_INDEX) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut n: u8 =
                                (*stack).index.wrapping_sub(1 as Arity) as u8;
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
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_roll\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_ROLL) as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut j_2: i32 = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d as i32;
                            let mut n_0: u32 = (*(*stack)
                                .stack
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d as u32;
                            if (*stack).index < (2 as u32).wrapping_add(n_0) {
                                (*(*options).logger)
                                    .logSDS
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*options).logger as *mut ILogger,
                                    LOG_VL_IMPORTANT,
                                    LoggerType::Warning,
                                    crate::sdsbuild!(
                                        sdsempty(),
                                        b"[libcff] Stack cannot provide enough parameters for ",
                                        b"op_roll\0" as *const u8 as *const ::core::ffi::c_char,
                                        b" (",
                                        Hex4((OP_ROLL) as u32),
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
                                        (*stack).index.wrapping_sub(3 as Arity) as u8;
                                    let mut first: u8 = (*stack)
                                        .index
                                        .wrapping_sub(2 as Arity)
                                        .wrapping_sub(n_0 as Arity)
                                        as u8;
                                    reverseStack(stack, first, last);
                                    reverseStack(
                                        stack,
                                        (last as i32 - j_2 + 1 as i32) as u8,
                                        last,
                                    );
                                    reverseStack(stack, first, (last as i32 - j_2) as u8);
                                    (*stack).index = (*stack).index.wrapping_sub(2 as Arity);
                                }
                            }
                        }
                    }
                    11 => return,
                    10 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_callsubr\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_CALLSUBR) as u32),
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
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .logSDS
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::sdsbuild!(
                                    sdsempty(),
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_callgsubr\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4((OP_CALLGSUBR) as u32),
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
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
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
            CffValueType::Integer | CffValueType::Double => {
                let fresh0 = (*stack).index;
                (*stack).index = (*stack).index.wrapping_add(1);
                *(*stack).stack.offset(fresh0 as isize) = val;
            }
            _ => {}
        }
        start = start.offset(advance as isize);
    }
}
