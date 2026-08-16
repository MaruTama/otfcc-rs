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
use crate::libcff::cff_charset::CffCharset;
use crate::libcff::cff_fdselect::{CffFdSelect};
use crate::libcff::cff_index::CffIndex;
use crate::libcff::cff_value::{CffValueType, CffValue, CffValueBody};
use crate::libcff::cff_charset::{cff_extract_charset};
use crate::libcff::cff_codecs::{cff_decode_cs2_token};
use crate::libcff::cff_dict::{parse_dict_key};
use crate::libcff::cff_fdselect::{cff_extract_fd_select};
use crate::libcff::cff_index::{extract_index, get_index_length, empty_index, cff_index_dispose};

/// The Top DICT's Encoding offset is overloaded by spec: values 0 and 1
/// select the two predefined (Standard/Expert) encodings outright, and
/// any other value is a real offset into an embedded encoding table.
/// `CffEncoding` (`libcff.rs`) is the crate's own classification of the
/// result; these two constants are just the spec's special-cased offset
/// values `parse_encoding` compares against before treating an offset as
/// real.
const CFF_STANDARD_ENCODING_OFFSET: i32 = 0;
const CFF_EXPERT_ENCODING_OFFSET: i32 = 1;
#[inline]
unsafe fn gu1(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 = *s.offset(p as isize) as u32;
    return b0;
}
#[inline]
unsafe fn gu2(mut s: *mut u8, mut p: u32) -> u32 {
    let mut b0: u32 =
        ((*s.offset(p as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
    let mut b1: u32 = *s
        .offset(p as isize)
        .offset(1 as ::core::ffi::c_int as isize) as u32;
    return b0 | b1;
}
// Returns `CffEncoding` by value instead of writing through a `*mut
// CffEncoding` out-param -- the same "unwrap_X_table"-adjacent shape as
// every other `parse_*`/`read_*` function elsewhere in this migration
// that used to fill an already-allocated out-param slot.
//
// No longer `extern "C"`: `CffEncoding` is a data-carrying enum with no C
// spelling, so claiming the C ABI would be a lie (`improper_ctypes_definitions`).
// Only called from within this file, not part of the crate's public ABI.
unsafe fn parse_encoding(mut cff: *mut CffFile, mut offset: i32) -> CffEncoding {
    let mut data: *mut u8 = (*cff).raw_data;
    if offset == CFF_STANDARD_ENCODING_OFFSET {
        return CffEncoding::Standard;
    } else if offset == CFF_EXPERT_ENCODING_OFFSET {
        return CffEncoding::Expert;
    }
    match *data.offset(offset as isize) as ::core::ffi::c_int {
        0 => {
            let ncodes = *data.offset((offset + 1 as i32) as isize);
            let mut code: Vec<u8> = Vec::with_capacity(ncodes as usize);
            let mut i: u32 = 0 as u32;
            while i < ncodes as u32 {
                code.push(*data.offset(((offset + 2 as i32) as u32).wrapping_add(i) as isize));
                i = i.wrapping_add(1);
            }
            CffEncoding::Format0(code)
        }
        1 => {
            let nranges = *data.offset((offset + 1 as i32) as isize);
            let mut range1: Vec<CffEncodingRangeFormat1> = Vec::with_capacity(nranges as usize);
            let mut i_0: u32 = 0 as u32;
            while i_0 < nranges as u32 {
                let first = *data.offset(
                    ((offset + 2 as i32) as u32).wrapping_add(i_0.wrapping_mul(2 as u32)) as isize,
                );
                let nleft = *data.offset(
                    ((offset + 3 as i32) as u32).wrapping_add(i_0.wrapping_mul(2 as u32)) as isize,
                );
                range1.push(CffEncodingRangeFormat1 { first, nleft });
                i_0 = i_0.wrapping_add(1);
            }
            CffEncoding::Format1(range1)
        }
        _ => {
            let nsup = *data.offset(offset as isize);
            let mut supplement: Vec<CffEncodingSupplement> = Vec::with_capacity(nsup as usize);
            let mut i_1: u32 = 0 as u32;
            while i_1 < nsup as u32 {
                let code = *data.offset(
                    ((offset + 1 as i32) as u32).wrapping_add(i_1.wrapping_mul(3 as u32)) as isize,
                );
                let glyph = gu2(
                    data,
                    ((offset + 2 as i32) as u32).wrapping_add(i_1.wrapping_mul(3 as u32)),
                ) as u16;
                supplement.push(CffEncodingSupplement { code, glyph });
                i_1 = i_1.wrapping_add(1);
            }
            CffEncoding::FormatSupplement(supplement)
        }
    }
}
unsafe fn parse_cff_bytecode(mut cff: *mut CffFile, mut options: *const Options) {
    let mut pos: u32 = 0;
    let mut offset: i32 = 0;
    (*cff).head.major = gu1((*cff).raw_data, 0 as u32) as u8;
    (*cff).head.minor = gu1((*cff).raw_data, 1 as u32) as u8;
    (*cff).head.hdr_size = gu1((*cff).raw_data, 2 as u32) as u8;
    (*cff).head.off_size = gu1((*cff).raw_data, 3 as u32) as u8;
    pos = (*cff).head.hdr_size as u32;
    extract_index(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).name,
    );
    pos = (4 as u32).wrapping_add(get_index_length(
        &raw mut (*cff).name,
    ));
    extract_index(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).top_dict,
    );
    if (*cff).name.count != (*cff).top_dict.count {
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(b"[libcff] Bad CFF font: (",
                (*cff).name.count,
                b", name), (",
                (*cff).top_dict.count,
                b", top_dict).\n",
            ),
        );
    }
    pos = (4 as u32)
        .wrapping_add(get_index_length(
            &raw mut (*cff).name,
        ))
        .wrapping_add(get_index_length(
            &raw mut (*cff).top_dict,
        ));
    extract_index(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).string,
    );
    pos = (4 as u32)
        .wrapping_add(get_index_length(
            &raw mut (*cff).name,
        ))
        .wrapping_add(get_index_length(
            &raw mut (*cff).top_dict,
        ))
        .wrapping_add(get_index_length(
            &raw mut (*cff).string,
        ));
    extract_index(
        (*cff).raw_data,
        pos,
        &raw mut (*cff).global_subr,
    );
    if !(*cff).top_dict.data.is_empty() {
        let mut offset_0: i32 = 0;
        offset_0 = parse_dict_key(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset.as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset.as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_CHAR_STRINGS,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            extract_index(
                (*cff).raw_data,
                offset_0 as u32,
                &raw mut (*cff).char_strings,
            );
            (*cff).cnt_glyph = (*cff).char_strings.count as u16;
        } else {
            empty_index(&raw mut (*cff).char_strings);
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[libcff] Bad CFF font: no any glyph data.\n"),
            );
        }
        offset_0 = parse_dict_key(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset.as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset.as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_ENCODING,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            (*cff).encodings = parse_encoding(cff, offset_0);
        } else {
            (*cff).encodings = CffEncoding::Unspecified;
        }
        offset_0 = parse_dict_key(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset.as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset.as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_CHARSET,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            (*cff).charsets = cff_extract_charset(
                (*cff).raw_data,
                offset_0,
                (*cff).char_strings.count as u16,
            );
        } else {
            (*cff).charsets = CffCharset::IsoAdobe;
        }
        offset_0 = parse_dict_key(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset.as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset.as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_FD_SELECT,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if (*cff).char_strings.count != 0 && offset_0 != -(1 as i32) {
            (*cff).fdselect = cff_extract_fd_select(
                (*cff).raw_data,
                offset_0,
                (*cff).char_strings.count as u16,
            );
        } else {
            (*cff).fdselect = CffFdSelect::Unspecified;
        }
        offset_0 = parse_dict_key(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset.as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset.as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_FD_ARRAY,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset_0 != -(1 as i32) {
            extract_index(
                (*cff).raw_data,
                offset_0 as u32,
                &raw mut (*cff).font_dict,
            );
        } else {
            empty_index(&raw mut (*cff).font_dict);
        }
    }
    let mut private_len: i32 = -(1 as i32);
    let mut private_off: i32 = -(1 as i32);
    if !(*cff).top_dict.data.is_empty() {
        private_len = parse_dict_key(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset.as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset.as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_PRIVATE,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        private_off = parse_dict_key(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset.as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset.as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_PRIVATE,
            1 as u32,
        )
        .c2rust_unnamed
        .i;
    }
    if private_off != -(1 as i32) && private_len != -(1 as i32) {
        offset = parse_dict_key(
            (*cff).raw_data.offset(private_off as isize),
            private_len as u32,
            OP_SUBRS,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if offset != -(1 as i32) {
            extract_index(
                (*cff).raw_data,
                (private_off + offset) as u32,
                &raw mut (*cff).local_subr,
            );
        } else {
            empty_index(&raw mut (*cff).local_subr);
        }
    } else {
        empty_index(&raw mut (*cff).local_subr);
    };
}
pub unsafe fn cff_open_stream(
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
pub unsafe fn cff_close(mut file: *mut CffFile) {
    if !file.is_null() {
        if !(*file).raw_data.is_null() {
            free((*file).raw_data as *mut ::core::ffi::c_void);
            (*file).raw_data = ::core::ptr::null_mut::<u8>();
        }
        cff_index_dispose(&raw mut (*file).name);
        cff_index_dispose(&raw mut (*file).top_dict);
        cff_index_dispose(&raw mut (*file).string);
        cff_index_dispose(&raw mut (*file).global_subr);
        cff_index_dispose(&raw mut (*file).char_strings);
        cff_index_dispose(&raw mut (*file).font_dict);
        cff_index_dispose(&raw mut (*file).local_subr);
        // Reassigning drops whatever Vec the previous variant owned, before the
        // struct itself is freed via a bare `free()` below (which does not run
        // Drop glue) -- same pattern as `dispose_glyph_order`.
        (*file).encodings = CffEncoding::Unspecified;
        (*file).charsets = CffCharset::IsoAdobe;
        (*file).fdselect = CffFdSelect::Unspecified;
        free(file as *mut ::core::ffi::c_void);
        file = ::core::ptr::null_mut::<CffFile>();
    }
}
// No longer `extern "C"`: `&CffFdSelect` has no C spelling. Only called
// from within `table/cff.rs`, not part of the crate's public ABI -- same
// reasoning as `parse_encoding`. Takes `&CffFdSelect` rather than by value
// since its two callers both read `(*f).fdselect` from a shared `CffFile`
// across repeated per-glyph calls -- moving it out on the first call would
// leave it invalid for the rest.
pub unsafe fn cff_parse_subr(
    idx: u16,
    raw: *mut u8,
    fdarray: &CffIndex,
    select: &CffFdSelect,
    subr: *mut CffIndex,
) -> u8 {
    let mut fd: u8 = 0 as u8;
    let mut off_private: i32 = 0;
    let mut len_private: i32 = 0;
    let mut off_subr: i32 = 0;
    match select {
        CffFdSelect::Format0(fds) => {
            fd = fds[idx as usize];
        }
        CffFdSelect::Format3 { range3, sentinel } => {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < range3.len() as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                if idx as ::core::ffi::c_int >= range3[i as usize].first as ::core::ffi::c_int
                    && (idx as ::core::ffi::c_int)
                        < range3[(i + 1 as ::core::ffi::c_int) as usize].first as ::core::ffi::c_int
                {
                    fd = range3[i as usize].fd;
                }
                i += 1;
            }
            if idx as ::core::ffi::c_int
                >= range3[range3.len() - 1 as usize].first as ::core::ffi::c_int
                && (idx as ::core::ffi::c_int) < *sentinel as ::core::ffi::c_int
            {
                fd = range3[range3.len() - 1 as usize].fd;
            }
        }
        CffFdSelect::Unspecified => {
            fd = 0 as u8;
        }
    }
    off_private = parse_dict_key(
        fdarray
            .data.as_ptr()
            .offset(*fdarray.offset.as_ptr().offset(fd as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*fdarray
            .offset.as_ptr()
            .offset((fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*fdarray.offset.as_ptr().offset(fd as isize)),
        OP_PRIVATE,
        1 as u32,
    )
    .c2rust_unnamed
    .i;
    len_private = parse_dict_key(
        fdarray
            .data.as_ptr()
            .offset(*fdarray.offset.as_ptr().offset(fd as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*fdarray
            .offset.as_ptr()
            .offset((fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*fdarray.offset.as_ptr().offset(fd as isize)),
        OP_PRIVATE,
        0 as u32,
    )
    .c2rust_unnamed
    .i;
    if off_private != -(1 as i32) && len_private != -(1 as i32) {
        off_subr = parse_dict_key(
            raw.offset(off_private as isize),
            len_private as u32,
            OP_SUBRS,
            0 as u32,
        )
        .c2rust_unnamed
        .i;
        if off_subr != -(1 as i32) {
            extract_index(
                raw,
                (off_private + off_subr) as u32,
                subr,
            );
        } else {
            empty_index(subr);
        }
    } else {
        empty_index(subr);
    }
    return fd;
}
#[inline]
unsafe fn compute_subr_bias(mut cnt: u16) -> u16 {
    if (cnt as ::core::ffi::c_int) < 1240 as ::core::ffi::c_int {
        return 107 as u16;
    } else if (cnt as ::core::ffi::c_int) < 33900 as ::core::ffi::c_int {
        return 1131 as u16;
    } else {
        return 32768 as u16;
    };
}
unsafe fn reverse_stack(
    mut stack: *mut CffStack,
    mut left: u8,
    mut right: u8,
) {
    let mut p1: *mut CffValue = (*stack).stack.as_mut_ptr().offset(left as ::core::ffi::c_int as isize);
    let mut p2: *mut CffValue = (*stack).stack.as_mut_ptr().offset(right as ::core::ffi::c_int as isize);
    while p1 < p2 {
        let mut temp: CffValue = *p1;
        *p1 = *p2;
        *p2 = temp;
        p1 = p1.offset(1);
        p2 = p2.offset(-1);
    }
}
unsafe extern "C" fn callback_nop_set_width(
    mut _context: *mut ::core::ffi::c_void,
    mut _width: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nop_new_contour(mut _context: *mut ::core::ffi::c_void) {}
unsafe extern "C" fn callback_nop_line_to(
    mut _context: *mut ::core::ffi::c_void,
    mut _x1: ::core::ffi::c_double,
    mut _y1: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nop_curve_to(
    mut _context: *mut ::core::ffi::c_void,
    mut _x1: ::core::ffi::c_double,
    mut _y1: ::core::ffi::c_double,
    mut _x2: ::core::ffi::c_double,
    mut _y2: ::core::ffi::c_double,
    mut _x3: ::core::ffi::c_double,
    mut _y3: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nopset_hint(
    mut _context: *mut ::core::ffi::c_void,
    mut _is_vertical: bool,
    mut _position: ::core::ffi::c_double,
    mut _width: ::core::ffi::c_double,
) {
}
unsafe extern "C" fn callback_nopset_mask(
    mut _context: *mut ::core::ffi::c_void,
    mut _is_contour_mask: bool,
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
pub unsafe fn cff_parse_outline(
    mut data: *mut u8,
    mut len: u32,
    gsubr: &CffIndex,
    lsubr: &CffIndex,
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
    let mut set_width: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> (),
    > = methods.set_width;
    let mut new_contour: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> =
        methods.new_contour;
    let mut line_to: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    > = methods.line_to;
    let mut curve_to: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    > = methods.curve_to;
    let mut set_hint: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            bool,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    > = methods.set_hint;
    let mut set_mask: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> ()> =
        methods.set_mask;
    let mut getrand: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double,
    > = methods.getrand;
    if set_width.is_none() {
        set_width = Some(
            callback_nop_set_width
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> (),
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> ()>;
    }
    if new_contour.is_none() {
        new_contour =
            Some(callback_nop_new_contour as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    }
    if line_to.is_none() {
        line_to = Some(
            callback_nop_line_to
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
    if curve_to.is_none() {
        curve_to = Some(
            callback_nop_curve_to
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
    if set_hint.is_none() {
        set_hint = Some(
            callback_nopset_hint
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
    if set_mask.is_none() {
        set_mask = Some(
            callback_nopset_mask
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
        advance = cff_decode_cs2_token(start, &raw mut val);
        match val.t {
            CffValueType::Operator => {
                let mut hint_base: ::core::ffi::c_double = 0.;
                match val.c2rust_unnamed.i {
                    1 | 3 | 18 | 23 => {
                        if (*stack).index.wrapping_rem(2 as Arity) != 0 {
                            set_width.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                        }
                        (*stack).stem = ((*stack).stem as Arity)
                            .wrapping_add((*stack).index >> 1 as ::core::ffi::c_int)
                            as u8 as u8;
                        hint_base = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut j: u16 = (*stack).index.wrapping_rem(2 as Arity) as u16;
                        while (j as Arity) < (*stack).index {
                            let mut pos: ::core::ffi::c_double =
                                (*(*stack).stack.as_mut_ptr().offset(j as isize)).c2rust_unnamed.d;
                            let mut width: ::core::ffi::c_double = (*(*stack).stack.as_mut_ptr().offset(
                                (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .c2rust_unnamed
                            .d;
                            set_hint.expect("non-null function pointer")(
                                outline,
                                val.c2rust_unnamed.i == OP_VSTEM.0
                                    || val.c2rust_unnamed.i
                                        == OP_VSTEMHM.0,
                                pos + hint_base,
                                width,
                            );
                            hint_base += pos + width;
                            j = (j as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u16;
                        }
                        (*stack).index = 0 as Arity;
                    }
                    19 | 20 => {
                        if (*stack).index.wrapping_rem(2 as Arity) != 0 {
                            set_width.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                        }
                        let mut is_vertical: bool =
                            (*stack).stem as ::core::ffi::c_int > 0 as ::core::ffi::c_int;
                        (*stack).stem = ((*stack).stem as Arity)
                            .wrapping_add((*stack).index >> 1 as ::core::ffi::c_int)
                            as u8 as u8;
                        let mut hint_base_0: ::core::ffi::c_double =
                            0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut j_0: u16 =
                            (*stack).index.wrapping_rem(2 as Arity) as u16;
                        while (j_0 as Arity) < (*stack).index {
                            let mut pos_0: ::core::ffi::c_double =
                                (*(*stack).stack.as_mut_ptr().offset(j_0 as isize)).c2rust_unnamed.d;
                            let mut width_0: ::core::ffi::c_double = (*(*stack).stack.as_mut_ptr().offset(
                                (j_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .c2rust_unnamed
                            .d;
                            set_hint.expect("non-null function pointer")(
                                outline,
                                is_vertical,
                                pos_0 + hint_base_0,
                                width_0,
                            );
                            hint_base_0 += pos_0 + width_0;
                            j_0 = (j_0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u16;
                        }
                        let mut mask_length: u32 =
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
                        while byte < mask_length {
                            let mut mask_byte: u8 =
                                *start.offset(advance.wrapping_add(byte) as isize);
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(0 as u32)
                                    as isize,
                            ) = mask_byte as ::core::ffi::c_int >> 7 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(1 as u32)
                                    as isize,
                            ) = mask_byte as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(2 as u32)
                                    as isize,
                            ) = mask_byte as ::core::ffi::c_int >> 5 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(3 as u32)
                                    as isize,
                            ) = mask_byte as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(4 as u32)
                                    as isize,
                            ) = mask_byte as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(5 as u32)
                                    as isize,
                            ) = mask_byte as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(6 as u32)
                                    as isize,
                            ) = mask_byte as ::core::ffi::c_int >> 1 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask.offset(
                                (byte << 3 as ::core::ffi::c_int).wrapping_add(7 as u32)
                                    as isize,
                            ) = mask_byte as ::core::ffi::c_int >> 0 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            byte = byte.wrapping_add(1);
                        }
                        set_mask.expect("non-null function pointer")(
                            outline,
                            val.c2rust_unnamed.i == OP_CNTRMASK.0,
                            mask,
                        );
                        advance = advance.wrapping_add(mask_length);
                        (*stack).index = 0 as Arity;
                    }
                    4 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_vmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_VMOVETO.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 1 as Arity {
                                set_width
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            new_contour.expect("non-null function pointer")(outline);
                            line_to.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack.as_mut_ptr()
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_rmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_RMOVETO.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 2 as Arity {
                                set_width
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            new_contour.expect("non-null function pointer")(outline);
                            line_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_HMOVETO.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 1 as Arity {
                                set_width
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    outline,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                        .c2rust_unnamed
                                        .d,
                                );
                            }
                            new_contour.expect("non-null function pointer")(outline);
                            line_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack.as_mut_ptr()
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
                            set_width.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                    }
                    5 => {
                        i = 0 as u32;
                        while i < (*stack).index {
                            line_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
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
                            line_to.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            i = 1 as u32;
                            while i < (*stack).index {
                                line_to.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                );
                                line_to.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                                i = i.wrapping_add(2 as u32);
                            }
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index {
                                line_to.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                );
                                line_to.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
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
                            line_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            i = 1 as u32;
                            while i < (*stack).index {
                                line_to.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                );
                                line_to.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
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
                                line_to.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                );
                                line_to.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
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
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset(i.wrapping_add(1 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset(i.wrapping_add(2 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset(i.wrapping_add(3 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset(i.wrapping_add(4 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
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
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset(i.wrapping_add(1 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset(i.wrapping_add(2 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset(i.wrapping_add(3 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset(i.wrapping_add(4 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset(i.wrapping_add(5 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            i = i.wrapping_add(6 as u32);
                        }
                        line_to.expect("non-null function pointer")(
                            outline,
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                        );
                        (*stack).index = 0 as Arity;
                    }
                    25 => {
                        i = 0 as u32;
                        while i < (*stack).index.wrapping_sub(6 as Arity) {
                            line_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset(i.wrapping_add(1 as u32) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                            i = i.wrapping_add(2 as u32);
                        }
                        curve_to.expect("non-null function pointer")(
                            outline,
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(6 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(5 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d,
                        );
                        (*stack).index = 0 as Arity;
                    }
                    26 => {
                        if (*stack).index.wrapping_rem(4 as Arity) == 1 as Arity {
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.as_mut_ptr().offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            i = 5 as u32;
                            while i < (*stack).index {
                                curve_to.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                                i = i.wrapping_add(4 as u32);
                            }
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index {
                                curve_to.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
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
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            i = 5 as u32;
                            while i < (*stack).index {
                                curve_to.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
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
                                curve_to.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
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
                                curve_to.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                            } else {
                                curve_to.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                            }
                            i = i.wrapping_add(4 as u32);
                        }
                        if (*stack).index.wrapping_rem(8 as Arity) == 5 as Arity {
                            curve_to.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(5 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        if (*stack).index.wrapping_rem(8 as Arity) == 1 as Arity {
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(5 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
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
                                curve_to.expect("non-null function pointer")(
                                    outline,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                );
                            } else {
                                curve_to.expect("non-null function pointer")(
                                    outline,
                                    0.0f64,
                                    (*(*stack).stack.as_mut_ptr().offset(i as isize)).c2rust_unnamed.d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(2 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    (*(*stack)
                                        .stack.as_mut_ptr()
                                        .offset(i.wrapping_add(3 as u32) as isize))
                                    .c2rust_unnamed
                                    .d,
                                    0.0f64,
                                );
                            }
                            i = i.wrapping_add(4 as u32);
                        }
                        if (*stack).index.wrapping_rem(8 as Arity) == 5 as Arity {
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(5 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                0.0f64,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                            );
                        }
                        if (*stack).index.wrapping_rem(8 as Arity) == 1 as Arity {
                            curve_to.expect("non-null function pointer")(
                                outline,
                                0.0f64,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(5 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                                .c2rust_unnamed
                                .d,
                                (*(*stack)
                                    .stack.as_mut_ptr()
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hflex\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_HFLEX.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.as_mut_ptr().offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                -(*(*stack).stack.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(6 as ::core::ffi::c_int as isize))
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_flex\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_FLEX.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(7 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(9 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(10 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(11 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    3108 => {
                        if (*stack).index < 9 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hflex1\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_HFLEX1.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                            );
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                0.0f64,
                                (*(*stack).stack.as_mut_ptr().offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(7 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                -((*(*stack).stack.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d
                                    + (*(*stack).stack.as_mut_ptr().offset(3 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.as_mut_ptr().offset(7 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d),
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    3109 => {
                        if (*stack).index < 11 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_flex1\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_FLEX1.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut dx: ::core::ffi::c_double =
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d
                                    + (*(*stack).stack.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.as_mut_ptr().offset(4 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.as_mut_ptr().offset(6 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d;
                            let mut dy: ::core::ffi::c_double =
                                (*(*stack).stack.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d
                                    + (*(*stack).stack.as_mut_ptr().offset(3 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.as_mut_ptr().offset(5 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.as_mut_ptr().offset(7 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d
                                    + (*(*stack).stack.as_mut_ptr().offset(9 as ::core::ffi::c_int as isize))
                                        .c2rust_unnamed
                                        .d;
                            if fabs(dx) > fabs(dy) {
                                dx = (*(*stack).stack.as_mut_ptr().offset(10 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d;
                                dy = -dy;
                            } else {
                                dx = -dx;
                                dy = (*(*stack).stack.as_mut_ptr().offset(10 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d;
                            }
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(3 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(4 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(5 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                            );
                            curve_to.expect("non-null function pointer")(
                                outline,
                                (*(*stack).stack.as_mut_ptr().offset(6 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(7 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize))
                                    .c2rust_unnamed
                                    .d,
                                (*(*stack).stack.as_mut_ptr().offset(9 as ::core::ffi::c_int as isize))
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_and\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_AND.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_or\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_OR.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_0: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_0: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_not\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_NOT.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = if num != 0. { 0.0f64 } else { 1.0f64 };
                        }
                    }
                    3081 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_abs\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_ABS.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num_0: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = if num_0 < 0.0f64 { -num_0 } else { num_0 };
                        }
                    }
                    3082 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_add\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_ADD.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_1: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_1: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num1_1 + num2_1;
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3083 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_sub\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_SUB.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_2: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_2: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num1_2 - num2_2;
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3084 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_div\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_DIV.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_3: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_3: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num1_3 / num2_3;
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3086 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_neg\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_NEG.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num_1: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = -num_1;
                        }
                    }
                    3087 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_eq\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_EQ.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_4: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_4: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = if num1_4 == num2_4 { 1.0f64 } else { 0.0f64 };
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3090 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_drop\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_DROP.0 as u32),
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_put\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_PUT.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut val_0: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut i_0: i32 = (*(*stack)
                                .stack.as_mut_ptr()
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_get\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_GET.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut i_1: i32 = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d as i32;
                            (*(*stack)
                                .stack.as_mut_ptr()
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_ifelse\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_IFELSE.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut v2: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut v1: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut s2: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(3 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut s1: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(4 as Arity) as isize))
                            .c2rust_unnamed
                            .d = if v1 <= v2 { s1 } else { s2 };
                            (*stack).index = (*stack).index.wrapping_sub(3 as Arity);
                        }
                    }
                    3095 => {
                        (*(*stack).stack.as_mut_ptr().offset((*stack).index as isize)).t = CffValueType::Double;
                        (*(*stack).stack.as_mut_ptr().offset((*stack).index as isize))
                            .c2rust_unnamed
                            .d = getrand.expect("non-null function pointer")(outline);
                        (*stack).index = (*stack).index.wrapping_add(1 as Arity);
                    }
                    3096 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_mul\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_MUL.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_5: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_5: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num1_5 * num2_5;
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3098 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_sqrt\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_SQRT.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num_2: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = sqrt(num_2);
                        }
                    }
                    3099 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_dup\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_DUP.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            *(*stack).stack.as_mut_ptr().offset((*stack).index as isize) = *(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize);
                            (*stack).index = (*stack).index.wrapping_add(1 as Arity);
                        }
                    }
                    3100 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_exch\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_EXCH.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut num1_6: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            let mut num2_6: ::core::ffi::c_double = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num2_6;
                            (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d = num1_6;
                        }
                    }
                    3101 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_index\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_INDEX.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut n: u8 =
                                (*stack).index.wrapping_sub(1 as Arity) as u8;
                            let mut j_1: u8 = (n as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - (*(*stack).stack.as_mut_ptr().offset(n as isize)).c2rust_unnamed.d as u8
                                    as ::core::ffi::c_int
                                    % n as ::core::ffi::c_int)
                                as u8;
                            *(*stack).stack.as_mut_ptr().offset(n as isize) =
                                *(*stack).stack.as_mut_ptr().offset(j_1 as isize);
                        }
                    }
                    3102 => {
                        if (*stack).index < 2 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_roll\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_ROLL.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut j_2: i32 = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize))
                            .c2rust_unnamed
                            .d as i32;
                            let mut n_0: u32 = (*(*stack)
                                .stack.as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize))
                            .c2rust_unnamed
                            .d as u32;
                            if (*stack).index < (2 as u32).wrapping_add(n_0) {
                                (*(*options).logger)
                                    .log_sds
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*options).logger as *mut ILogger,
                                    LOG_VL_IMPORTANT,
                                    LoggerType::Warning,
                                    crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                        b"op_roll\0" as *const u8 as *const ::core::ffi::c_char,
                                        b" (",
                                        Hex4(OP_ROLL.0 as u32),
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
                                    reverse_stack(stack, first, last);
                                    reverse_stack(
                                        stack,
                                        (last as i32 - j_2 + 1 as i32) as u8,
                                        last,
                                    );
                                    reverse_stack(stack, first, (last as i32 - j_2) as u8);
                                    (*stack).index = (*stack).index.wrapping_sub(2 as Arity);
                                }
                            }
                        }
                    }
                    11 => return,
                    10 => {
                        if (*stack).index < 1 as Arity {
                            (*(*options).logger)
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_callsubr\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_CALLSUBR.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1);
                            let mut subr: u32 =
                                (*(*stack).stack.as_mut_ptr().offset((*stack).index as isize))
                                    .c2rust_unnamed
                                    .d as u32;
                            cff_parse_outline(
                                (lsubr
                                    .data.as_ptr() as *mut u8)
                                    .offset(
                                        *lsubr
                                            .offset.as_ptr()
                                            .offset((lsubr_bias as u32).wrapping_add(subr)
                                                as isize)
                                            as isize,
                                    )
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                (*lsubr.offset.as_ptr().offset(
                                    (lsubr_bias as u32)
                                        .wrapping_add(subr)
                                        .wrapping_add(1 as u32)
                                        as isize,
                                ))
                                .wrapping_sub(
                                    *lsubr.offset.as_ptr().offset(
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
                                .log_sds
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*options).logger as *mut ILogger,
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_callgsubr\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_CALLGSUBR.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1);
                            let mut subr_0: u32 =
                                (*(*stack).stack.as_mut_ptr().offset((*stack).index as isize))
                                    .c2rust_unnamed
                                    .d as u32;
                            cff_parse_outline(
                                (gsubr
                                    .data.as_ptr() as *mut u8)
                                    .offset(*gsubr.offset.as_ptr().offset(
                                        (gsubr_bias as u32).wrapping_add(subr_0) as isize,
                                    ) as isize)
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                (*gsubr.offset.as_ptr().offset(
                                    (gsubr_bias as u32)
                                        .wrapping_add(subr_0)
                                        .wrapping_add(1 as u32)
                                        as isize,
                                ))
                                .wrapping_sub(
                                    *gsubr.offset.as_ptr().offset(
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
                            .log_sds
                            .expect(
                                "non-null function pointer",
                            )(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"Warning: unknown operator ",
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
                *(*stack).stack.as_mut_ptr().offset(fresh0 as isize) = val;
            }
            _ => {}
        }
        start = start.offset(advance as isize);
    }
}
