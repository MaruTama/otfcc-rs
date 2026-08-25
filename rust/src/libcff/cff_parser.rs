#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, memcpy};
unsafe extern "C" {
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::alloc::__caryll_allocate_clean;
use crate::support::font_reader::FontReader;

use crate::libcff::cff_charset::CffCharset;
use crate::libcff::cff_charset::cff_extract_charset;
use crate::libcff::cff_codecs::cff_decode_cs2_token;
use crate::libcff::cff_dict::parse_dict_key_int;
use crate::libcff::cff_fdselect::CffFdSelect;
use crate::libcff::cff_fdselect::cff_extract_fd_select;
use crate::libcff::cff_index::CffIndex;
use crate::libcff::cff_index::{cff_index_dispose, empty_index, extract_index, get_index_length};
use crate::libcff::cff_value::{CffValue, cffnum};
use crate::libcff::{
    CffEncoding, CffEncodingRangeFormat1, CffEncodingSupplement, CffFile, CffStack, OP_ABS, OP_ADD,
    OP_AND, OP_CALLGSUBR, OP_CALLSUBR, OP_CHAR_STRINGS, OP_CHARSET, OP_CNTRMASK, OP_DIV, OP_DROP,
    OP_DUP, OP_ENCODING, OP_EQ, OP_EXCH, OP_FD_ARRAY, OP_FD_SELECT, OP_FLEX, OP_FLEX1, OP_GET,
    OP_HFLEX, OP_HFLEX1, OP_HMOVETO, OP_IFELSE, OP_INDEX, OP_MUL, OP_NEG, OP_NOT, OP_OR,
    OP_PRIVATE, OP_PUT, OP_RMOVETO, OP_ROLL, OP_SQRT, OP_SUB, OP_SUBRS, OP_VMOVETO, OP_VSTEM,
    OP_VSTEMHM, TYPE2_TRANSIENT_ARRAY,
};
use crate::support::options::Options;
use crate::support::primitives::Arity;
use crate::table::cff::{
    callback_draw_curveto, callback_draw_getrand, callback_draw_lineto, callback_draw_next_contour,
    callback_draw_sethint, callback_draw_setmask, callback_draw_setwidth,
};
use crate::support::fmt::Hex4;

/// The Top DICT's Encoding offset is overloaded by spec: values 0 and 1
/// select the two predefined (Standard/Expert) encodings outright, and
/// any other value is a real offset into an embedded encoding table.
/// `CffEncoding` (`libcff.rs`) is the crate's own classification of the
/// result; these two constants are just the spec's special-cased offset
/// values `parse_encoding` compares against before treating an offset as
/// real.
const CFF_STANDARD_ENCODING_OFFSET: i32 = 0;
const CFF_EXPERT_ENCODING_OFFSET: i32 = 1;
// The Type 2 Charstring spec (Adobe TN #5177) caps subroutine call nesting
// at 10. Neither this parser nor the C implementation it was transpiled
// from ever enforced that: `callsubr`/`callgsubr` recursed into
// `cff_parse_outline` unconditionally, so a subroutine that (directly or
// through a cycle of other subroutines) calls itself recurses until the
// native stack overflows -- a crash confirmed to reproduce in the C
// toolchain too (`rust/fuzz/README.md`), not a migration regression.
// `FDArrayTest257.otf` in the fuzz seed corpus triggers exactly this.
const MAX_SUBR_CALL_DEPTH: u32 = 10;
// `gu1`/`gu2` (no bounds checking, no length parameter at all) are gone --
// see `libcff/cff_index.rs`'s own conversion for the same move.
//
// Returns `CffEncoding` by value instead of writing through a `*mut
// CffEncoding` out-param -- the same "unwrap_X_table"-adjacent shape as
// every other `parse_*`/`read_*` function elsewhere in this migration
// that used to fill an already-allocated out-param slot.
//
// No longer `extern "C"`: `CffEncoding` is a data-carrying enum with no C
// spelling, so claiming the C ABI would be a lie (`improper_ctypes_definitions`).
// Only called from within this file, not part of the crate's public ABI.
//
// The original had no bounds checking anywhere in this function -- not on
// `offset` (a negative value, reachable from a malformed DICT key, moved
// the read pointer *before* the buffer), not on any of the three formats'
// arrays. Every read now goes through one sequential `FontReader`: all
// three formats lay their count field and array immediately after the
// format byte with no gaps, so one reader walking forward covers the
// whole record (matches `cff_extract_fd_select`'s equivalent conversion).
// On any bounds failure, or a negative `offset`, this falls back to
// `Unspecified` -- the same fallback the original already used at its own
// call site for "no Encoding key in the DICT at all"; this function
// itself drew no such distinction before, since it never had a failure
// path.
unsafe fn parse_encoding(cff: *mut CffFile, offset: i32) -> CffEncoding {
    if offset == CFF_STANDARD_ENCODING_OFFSET {
        return CffEncoding::Standard;
    } else if offset == CFF_EXPERT_ENCODING_OFFSET {
        return CffEncoding::Expert;
    }
    if offset < 0 {
        return CffEncoding::Unspecified;
    }
    let slice = ::core::slice::from_raw_parts((*cff).raw_data, (*cff).raw_length as usize);
    let result: Option<CffEncoding> = 'parse: {
        let Ok(mut r) = FontReader::new(slice).at(offset as usize) else {
            break 'parse None;
        };
        let Ok(format) = r.u8() else {
            break 'parse None;
        };
        match format {
            0 => {
                let Ok(ncodes) = r.u8() else {
                    break 'parse None;
                };
                let mut code: Vec<u8> = Vec::with_capacity(ncodes as usize);
                for _ in 0..ncodes {
                    let Ok(v) = r.u8() else { break 'parse None };
                    code.push(v);
                }
                break 'parse Some(CffEncoding::Format0(code));
            }
            1 => {
                let Ok(nranges) = r.u8() else {
                    break 'parse None;
                };
                let mut range1: Vec<CffEncodingRangeFormat1> = Vec::with_capacity(nranges as usize);
                for _ in 0..nranges {
                    let Ok(first) = r.u8() else { break 'parse None };
                    let Ok(nleft) = r.u8() else { break 'parse None };
                    range1.push(CffEncodingRangeFormat1 { first, nleft });
                }
                break 'parse Some(CffEncoding::Format1(range1));
            }
            _ => {
                // The original re-reads the format byte itself as `nsup`
                // here (both are `data[offset]`) -- preserved verbatim,
                // out of this PR's scope to second-guess.
                let nsup = format;
                let mut supplement: Vec<CffEncodingSupplement> = Vec::with_capacity(nsup as usize);
                for _ in 0..nsup {
                    let Ok(code) = r.u8() else { break 'parse None };
                    let Ok(glyph) = r.u16() else {
                        break 'parse None;
                    };
                    supplement.push(CffEncodingSupplement { code, glyph });
                }
                break 'parse Some(CffEncoding::FormatSupplement(supplement));
            }
        }
    };
    result.unwrap_or(CffEncoding::Unspecified)
}
unsafe fn parse_cff_bytecode(cff: *mut CffFile, options: &Options) {
    let mut pos: u32 = 0;
    let mut offset: i32 = 0;
    // No length check guarded these 4 header-byte reads at all -- a `raw_
    // length` shorter than 4 read straight past the allocation. Every
    // field now defaults to 0 on a bounds failure instead: `extract_index`
    // below is already bounds-checked regardless of what `pos` it's given
    // (a garbage `hdr_size` just makes it fail cleanly too, same as any
    // other malformed offset), so there's nothing to gain from bailing out
    // of this function early on a too-short header.
    let header_slice = ::core::slice::from_raw_parts((*cff).raw_data, (*cff).raw_length as usize);
    let mut header_reader = FontReader::new(header_slice);
    (*cff).head.major = header_reader.u8().unwrap_or(0);
    (*cff).head.minor = header_reader.u8().unwrap_or(0);
    (*cff).head.hdr_size = header_reader.u8().unwrap_or(0);
    (*cff).head.off_size = header_reader.u8().unwrap_or(0);
    pos = (*cff).head.hdr_size as u32;
    extract_index(
        (*cff).raw_data,
        (*cff).raw_length,
        pos,
        &raw mut (*cff).name,
    );
    pos = (4 as u32).wrapping_add(get_index_length(&raw mut (*cff).name));
    extract_index(
        (*cff).raw_data,
        (*cff).raw_length,
        pos,
        &raw mut (*cff).top_dict,
    );
    if (*cff).name.count != (*cff).top_dict.count {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"[libcff] Bad CFF font: (",
                (*cff).name.count,
                b", name), (",
                (*cff).top_dict.count,
                b", top_dict).\n",
            ),
        );
    }
    pos = (4 as u32)
        .wrapping_add(get_index_length(&raw mut (*cff).name))
        .wrapping_add(get_index_length(&raw mut (*cff).top_dict));
    extract_index(
        (*cff).raw_data,
        (*cff).raw_length,
        pos,
        &raw mut (*cff).string,
    );
    pos = (4 as u32)
        .wrapping_add(get_index_length(&raw mut (*cff).name))
        .wrapping_add(get_index_length(&raw mut (*cff).top_dict))
        .wrapping_add(get_index_length(&raw mut (*cff).string));
    extract_index(
        (*cff).raw_data,
        (*cff).raw_length,
        pos,
        &raw mut (*cff).global_subr,
    );
    if !(*cff).top_dict.data.is_empty() {
        let mut offset_0: i32 = 0;
        offset_0 = parse_dict_key_int(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset
                .as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_CHAR_STRINGS,
            0 as u32,
        );
        if offset_0 != -(1 as i32) {
            extract_index(
                (*cff).raw_data,
                (*cff).raw_length,
                offset_0 as u32,
                &raw mut (*cff).char_strings,
            );
            (*cff).cnt_glyph = (*cff).char_strings.count as u16;
        } else {
            empty_index(&raw mut (*cff).char_strings);
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"[libcff] Bad CFF font: no any glyph data.\n"),
            );
        }
        offset_0 = parse_dict_key_int(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset
                .as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_ENCODING,
            0 as u32,
        );
        if offset_0 != -(1 as i32) {
            (*cff).encodings = parse_encoding(cff, offset_0);
        } else {
            (*cff).encodings = CffEncoding::Unspecified;
        }
        offset_0 = parse_dict_key_int(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset
                .as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_CHARSET,
            0 as u32,
        );
        if offset_0 != -(1 as i32) {
            (*cff).charsets = cff_extract_charset(
                (*cff).raw_data,
                (*cff).raw_length,
                offset_0,
                (*cff).char_strings.count as u16,
            );
        } else {
            (*cff).charsets = CffCharset::IsoAdobe;
        }
        offset_0 = parse_dict_key_int(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset
                .as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_FD_SELECT,
            0 as u32,
        );
        if (*cff).char_strings.count != 0 && offset_0 != -(1 as i32) {
            (*cff).fdselect = cff_extract_fd_select(
                (*cff).raw_data,
                (*cff).raw_length,
                offset_0,
                (*cff).char_strings.count as u16,
            );
        } else {
            (*cff).fdselect = CffFdSelect::Unspecified;
        }
        offset_0 = parse_dict_key_int(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset
                .as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_FD_ARRAY,
            0 as u32,
        );
        if offset_0 != -(1 as i32) {
            extract_index(
                (*cff).raw_data,
                (*cff).raw_length,
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
        private_len = parse_dict_key_int(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset
                .as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_PRIVATE,
            0 as u32,
        );
        private_off = parse_dict_key_int(
            (*cff).top_dict.data.as_ptr(),
            (*(*cff)
                .top_dict
                .offset
                .as_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .wrapping_sub(
                *(*cff)
                    .top_dict
                    .offset
                    .as_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
            ),
            OP_PRIVATE,
            1 as u32,
        );
    }
    if private_off != -(1 as i32) && private_len != -(1 as i32) {
        offset = parse_dict_key_int(
            (*cff).raw_data.offset(private_off as isize),
            private_len as u32,
            OP_SUBRS,
            0 as u32,
        );
        if offset != -(1 as i32) {
            extract_index(
                (*cff).raw_data,
                (*cff).raw_length,
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
    data: *mut u8,
    len: u32,
    options: &Options,
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
    raw_length: u32,
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
    // `fd` comes from the FDSelect table -- attacker-controlled bytes from
    // the font file, not bounded against `fdarray`'s actual size by
    // anything above. `fdarray.offset` (an INDEX's `count + 1` offset
    // entries) was then indexed via raw `.offset()` arithmetic with no
    // bounds check at all: an `fd` past `fdarray.count` read arbitrary
    // memory past the `Vec`'s allocation, a real SEGV a local fuzzing run
    // found within two minutes. `locate_subr` (used elsewhere in this
    // file for `callsubr`/`callgsubr`) already validates its INDEX lookup
    // the same way this one now does -- treat an out-of-range `fd` as "no
    // private dict for this glyph" and fall back to `empty_index`, the
    // same fallback already used a few lines down for a well-formed `fd`
    // whose FDArray entry just doesn't declare a Private dict.
    if fd as u32 >= fdarray.count {
        empty_index(subr);
        return fd;
    }
    off_private = parse_dict_key_int(
        fdarray
            .data
            .as_ptr()
            .offset(*fdarray.offset.as_ptr().offset(fd as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*fdarray
            .offset
            .as_ptr()
            .offset((fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*fdarray.offset.as_ptr().offset(fd as isize)),
        OP_PRIVATE,
        1 as u32,
    );
    len_private = parse_dict_key_int(
        fdarray
            .data
            .as_ptr()
            .offset(*fdarray.offset.as_ptr().offset(fd as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*fdarray
            .offset
            .as_ptr()
            .offset((fd as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*fdarray.offset.as_ptr().offset(fd as isize)),
        OP_PRIVATE,
        0 as u32,
    );
    if off_private != -(1 as i32) && len_private != -(1 as i32) {
        off_subr = parse_dict_key_int(
            raw.offset(off_private as isize),
            len_private as u32,
            OP_SUBRS,
            0 as u32,
        );
        if off_subr != -(1 as i32) {
            extract_index(raw, raw_length, (off_private + off_subr) as u32, subr);
        } else {
            empty_index(subr);
        }
    } else {
        empty_index(subr);
    }
    return fd;
}
#[inline]
// The subroutine index a `callsubr`/`callgsubr` operator uses is an
// entirely attacker-controlled Type2 CharString operand (a stack value,
// popped and cast to `u32`). The original indexed `gsubr`/`lsubr`'s own
// offset array with it via raw `.offset()` arithmetic and no bounds
// check at all, then used the (possibly garbage) result to derive a
// *pointer and length* for a *recursive* `cff_parse_outline` call -- a
// malformed subroutine index could recurse into arbitrary memory.
// `extract_index` (`libcff/cff_index.rs`) only validates an INDEX's
// *last* offset entry against the wraparound-to-4GB bug; intermediate
// entries can still be zero or non-monotonic, so this also re-validates
// the specific pair this call needs (both in bounds, and consistent with
// each other and with the INDEX's own data length), not just that
// `offset.get(idx)` succeeds.
unsafe fn locate_subr(subr_index: &CffIndex, bias: u16, subr: u32) -> Option<(*const u8, u32)> {
    let idx = (bias as u32).checked_add(subr)? as usize;
    let start = *subr_index.offset.get(idx)?;
    let end = *subr_index.offset.get(idx.checked_add(1)?)?;
    if start < 1 || end < start {
        return None;
    }
    let data_offset = (start - 1) as usize;
    let data_len = end - start;
    if data_offset.checked_add(data_len as usize)? > subr_index.data.len() {
        return None;
    }
    Some((subr_index.data.as_ptr().add(data_offset), data_len))
}
unsafe fn compute_subr_bias(cnt: u16) -> u16 {
    if (cnt as ::core::ffi::c_int) < 1240 as ::core::ffi::c_int {
        return 107 as u16;
    } else if (cnt as ::core::ffi::c_int) < 33900 as ::core::ffi::c_int {
        return 1131 as u16;
    } else {
        return 32768 as u16;
    };
}
unsafe fn reverse_stack(stack: *mut CffStack, left: u8, right: u8) {
    let mut p1: *mut CffValue = (*stack)
        .stack
        .as_mut_ptr()
        .offset(left as ::core::ffi::c_int as isize);
    let mut p2: *mut CffValue = (*stack)
        .stack
        .as_mut_ptr()
        .offset(right as ::core::ffi::c_int as isize);
    while p1 < p2 {
        let temp: CffValue = *p1;
        *p1 = *p2;
        *p2 = temp;
        p1 = p1.offset(1);
        p2 = p2.offset(-1);
    }
}
// `methods: CffIOutlineBuilder` parameter dropped: this was called from
// exactly one call site (`table/cff.rs`), always passing the single static
// `DRAW_PASS` -- degenerate polymorphism like every other collapsed
// vtable, just structured as a by-value struct argument instead of a
// global static. Every field of `DRAW_PASS` is always `Some`, so the old
// per-field `.is_none()` fallback-to-`callback_nop_*` branches below were
// already unreachable dead code; deleted along with the extraction, not
// just the vtable shell.
pub unsafe fn cff_parse_outline(
    data: *mut u8,
    len: u32,
    gsubr: &CffIndex,
    lsubr: &CffIndex,
    stack: *mut CffStack,
    outline: *mut ::core::ffi::c_void,
    options: &Options,
    depth: u32,
) {
    if depth > MAX_SUBR_CALL_DEPTH {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"[libcff] Subroutine call nesting exceeded ",
                MAX_SUBR_CALL_DEPTH as ::core::ffi::c_int,
                b"; the rest of this outline is ignored.\n",
            ),
        );
        return;
    }
    let gsubr_bias: u16 = compute_subr_bias(gsubr.count as u16);
    let lsubr_bias: u16 = compute_subr_bias(lsubr.count as u16);
    let mut start: *mut u8 = data;
    let mut advance: u32 = 0;
    let mut i: u32 = 0;
    let mut cnt_bezier: u32 = 0;
    let mut val: CffValue = CffValue::Unset;
    while start < data.offset(len as isize) {
        // The outer loop already bounds where a token can *start*, but
        // not that the token itself stays within `len` -- a token
        // starting near the end of a truncated CharString used to read
        // past it (see `cff_codecs.rs`'s own conversion). Stop cleanly
        // instead of reading on.
        let remaining = data.offset(len as isize).offset_from(start) as usize;
        let Some(adv) = cff_decode_cs2_token(start, remaining, &raw mut val) else {
            break;
        };
        advance = adv;
        match val {
            CffValue::Operator(op) => {
                let mut hint_base: ::core::ffi::c_double = 0.;
                match op {
                    1 | 3 | 18 | 23 => {
                        if (*stack).index.wrapping_rem(2 as Arity) != 0 {
                            callback_draw_setwidth(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(0 as ::core::ffi::c_int as isize),
                                ),
                            );
                        }
                        (*stack).stem = ((*stack).stem as Arity)
                            .wrapping_add((*stack).index >> 1 as ::core::ffi::c_int)
                            as u8 as u8;
                        hint_base = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut j: u16 = (*stack).index.wrapping_rem(2 as Arity) as u16;
                        while (j as Arity) < (*stack).index {
                            let pos: ::core::ffi::c_double =
                                cffnum(*(*stack).stack.as_mut_ptr().offset(j as isize));
                            let width: ::core::ffi::c_double =
                                cffnum(*(*stack).stack.as_mut_ptr().offset(
                                    (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ));
                            callback_draw_sethint(
                                outline,
                                op == OP_VSTEM.0 || op == OP_VSTEMHM.0,
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
                            callback_draw_setwidth(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(0 as ::core::ffi::c_int as isize),
                                ),
                            );
                        }
                        let is_vertical: bool =
                            (*stack).stem as ::core::ffi::c_int > 0 as ::core::ffi::c_int;
                        (*stack).stem = ((*stack).stem as Arity)
                            .wrapping_add((*stack).index >> 1 as ::core::ffi::c_int)
                            as u8 as u8;
                        let mut hint_base_0: ::core::ffi::c_double =
                            0 as ::core::ffi::c_int as ::core::ffi::c_double;
                        let mut j_0: u16 = (*stack).index.wrapping_rem(2 as Arity) as u16;
                        while (j_0 as Arity) < (*stack).index {
                            let pos_0: ::core::ffi::c_double =
                                cffnum(*(*stack).stack.as_mut_ptr().offset(j_0 as isize));
                            let width_0: ::core::ffi::c_double =
                                cffnum(*(*stack).stack.as_mut_ptr().offset(
                                    (j_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ));
                            callback_draw_sethint(
                                outline,
                                is_vertical,
                                pos_0 + hint_base_0,
                                width_0,
                            );
                            hint_base_0 += pos_0 + width_0;
                            j_0 = (j_0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as u16;
                        }
                        let mask_length: u32 =
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
                            let mask_byte: u8 =
                                *start.offset(advance.wrapping_add(byte) as isize);
                            *mask
                                .offset((byte << 3 as ::core::ffi::c_int).wrapping_add(0 as u32)
                                    as isize) = mask_byte as ::core::ffi::c_int
                                >> 7 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask
                                .offset((byte << 3 as ::core::ffi::c_int).wrapping_add(1 as u32)
                                    as isize) = mask_byte as ::core::ffi::c_int
                                >> 6 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask
                                .offset((byte << 3 as ::core::ffi::c_int).wrapping_add(2 as u32)
                                    as isize) = mask_byte as ::core::ffi::c_int
                                >> 5 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask
                                .offset((byte << 3 as ::core::ffi::c_int).wrapping_add(3 as u32)
                                    as isize) = mask_byte as ::core::ffi::c_int
                                >> 4 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask
                                .offset((byte << 3 as ::core::ffi::c_int).wrapping_add(4 as u32)
                                    as isize) = mask_byte as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask
                                .offset((byte << 3 as ::core::ffi::c_int).wrapping_add(5 as u32)
                                    as isize) = mask_byte as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask
                                .offset((byte << 3 as ::core::ffi::c_int).wrapping_add(6 as u32)
                                    as isize) = mask_byte as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            *mask
                                .offset((byte << 3 as ::core::ffi::c_int).wrapping_add(7 as u32)
                                    as isize) = mask_byte as ::core::ffi::c_int
                                >> 0 as ::core::ffi::c_int
                                & 1 as ::core::ffi::c_int
                                != 0;
                            byte = byte.wrapping_add(1);
                        }
                        callback_draw_setmask(outline, op == OP_CNTRMASK.0, mask);
                        advance = advance.wrapping_add(mask_length);
                        (*stack).index = 0 as Arity;
                    }
                    4 => {
                        if (*stack).index < 1 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_vmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_VMOVETO.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 1 as Arity {
                                callback_draw_setwidth(
                                    outline,
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize,
                                        ),
                                    ),
                                );
                            }
                            callback_draw_next_contour(outline);
                            callback_draw_lineto(
                                outline,
                                0.0f64,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                                ),
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    21 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_rmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_RMOVETO.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 2 as Arity {
                                callback_draw_setwidth(
                                    outline,
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(3 as Arity) as isize,
                                        ),
                                    ),
                                );
                            }
                            callback_draw_next_contour(outline);
                            callback_draw_lineto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                                ),
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    22 => {
                        if (*stack).index < 1 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hmoveto\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_HMOVETO.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            if (*stack).index > 1 as Arity {
                                callback_draw_setwidth(
                                    outline,
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize,
                                        ),
                                    ),
                                );
                            }
                            callback_draw_next_contour(outline);
                            callback_draw_lineto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                                ),
                                0.0f64,
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    14 => {
                        if (*stack).index > 0 as Arity {
                            callback_draw_setwidth(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                                ),
                            );
                        }
                    }
                    5 => {
                        i = 0 as u32;
                        while i < (*stack).index {
                            callback_draw_lineto(
                                outline,
                                cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize),
                                ),
                            );
                            i = i.wrapping_add(2 as u32);
                        }
                        (*stack).index = 0 as Arity;
                    }
                    7 => {
                        if (*stack).index.wrapping_rem(2 as Arity) == 1 as Arity {
                            callback_draw_lineto(
                                outline,
                                0.0f64,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(0 as ::core::ffi::c_int as isize),
                                ),
                            );
                            i = 1 as u32;
                            while i < (*stack).index {
                                callback_draw_lineto(
                                    outline,
                                    cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                    0.0f64,
                                );
                                callback_draw_lineto(
                                    outline,
                                    0.0f64,
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(1 as u32) as isize),
                                    ),
                                );
                                i = i.wrapping_add(2 as u32);
                            }
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index {
                                callback_draw_lineto(
                                    outline,
                                    0.0f64,
                                    cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                );
                                callback_draw_lineto(
                                    outline,
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(1 as u32) as isize),
                                    ),
                                    0.0f64,
                                );
                                i = i.wrapping_add(2 as u32);
                            }
                        }
                        (*stack).index = 0 as Arity;
                    }
                    6 => {
                        if (*stack).index.wrapping_rem(2 as Arity) == 1 as Arity {
                            callback_draw_lineto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(0 as ::core::ffi::c_int as isize),
                                ),
                                0.0f64,
                            );
                            i = 1 as u32;
                            while i < (*stack).index {
                                callback_draw_lineto(
                                    outline,
                                    0.0f64,
                                    cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                );
                                callback_draw_lineto(
                                    outline,
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(1 as u32) as isize),
                                    ),
                                    0.0f64,
                                );
                                i = i.wrapping_add(2 as u32);
                            }
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index {
                                callback_draw_lineto(
                                    outline,
                                    cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                    0.0f64,
                                );
                                callback_draw_lineto(
                                    outline,
                                    0.0f64,
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(1 as u32) as isize),
                                    ),
                                );
                                i = i.wrapping_add(2 as u32);
                            }
                        }
                        (*stack).index = 0 as Arity;
                    }
                    8 => {
                        i = 0 as u32;
                        while i < (*stack).index {
                            callback_draw_curveto(
                                outline,
                                cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(i.wrapping_add(1 as u32) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(i.wrapping_add(2 as u32) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(i.wrapping_add(3 as u32) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(i.wrapping_add(4 as u32) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(i.wrapping_add(5 as u32) as isize),
                                ),
                            );
                            i = i.wrapping_add(6 as u32);
                        }
                        (*stack).index = 0 as Arity;
                    }
                    24 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_rcurveline (24). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index.wrapping_sub(2 as Arity) {
                                callback_draw_curveto(
                                    outline,
                                    cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(1 as u32) as isize),
                                    ),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(2 as u32) as isize),
                                    ),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(3 as u32) as isize),
                                    ),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(4 as u32) as isize),
                                    ),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(5 as u32) as isize),
                                    ),
                                );
                                i = i.wrapping_add(6 as u32);
                            }
                            callback_draw_lineto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                                ),
                            );
                        }
                        (*stack).index = 0 as Arity;
                    }
                    25 => {
                        if (*stack).index < 6 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_rlinecurve (25). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index.wrapping_sub(6 as Arity) {
                                callback_draw_lineto(
                                    outline,
                                    cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(1 as u32) as isize),
                                    ),
                                );
                                i = i.wrapping_add(2 as u32);
                            }
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(6 as Arity) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(5 as Arity) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(4 as Arity) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(3 as Arity) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                                ),
                            );
                        }
                        (*stack).index = 0 as Arity;
                    }
                    26 => {
                        if (*stack).index.wrapping_rem(4 as Arity) == 1 as Arity {
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(0 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(2 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(3 as ::core::ffi::c_int as isize),
                                ),
                                0.0f64,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(4 as ::core::ffi::c_int as isize),
                                ),
                            );
                            i = 5 as u32;
                            while i < (*stack).index {
                                callback_draw_curveto(
                                    outline,
                                    0.0f64,
                                    cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(1 as u32) as isize),
                                    ),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(2 as u32) as isize),
                                    ),
                                    0.0f64,
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(3 as u32) as isize),
                                    ),
                                );
                                i = i.wrapping_add(4 as u32);
                            }
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index {
                                callback_draw_curveto(
                                    outline,
                                    0.0f64,
                                    cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(1 as u32) as isize),
                                    ),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(2 as u32) as isize),
                                    ),
                                    0.0f64,
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(3 as u32) as isize),
                                    ),
                                );
                                i = i.wrapping_add(4 as u32);
                            }
                        }
                        (*stack).index = 0 as Arity;
                    }
                    27 => {
                        if (*stack).index.wrapping_rem(4 as Arity) == 1 as Arity {
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(0 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(2 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(3 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(4 as ::core::ffi::c_int as isize),
                                ),
                                0.0f64,
                            );
                            i = 5 as u32;
                            while i < (*stack).index {
                                callback_draw_curveto(
                                    outline,
                                    cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                    0.0f64,
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(1 as u32) as isize),
                                    ),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(2 as u32) as isize),
                                    ),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(3 as u32) as isize),
                                    ),
                                    0.0f64,
                                );
                                i = i.wrapping_add(4 as u32);
                            }
                        } else {
                            i = 0 as u32;
                            while i < (*stack).index {
                                callback_draw_curveto(
                                    outline,
                                    cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                    0.0f64,
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(1 as u32) as isize),
                                    ),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(2 as u32) as isize),
                                    ),
                                    cffnum(
                                        *(*stack)
                                            .stack
                                            .as_mut_ptr()
                                            .offset(i.wrapping_add(3 as u32) as isize),
                                    ),
                                    0.0f64,
                                );
                                i = i.wrapping_add(4 as u32);
                            }
                        }
                        (*stack).index = 0 as Arity;
                    }
                    30 => {
                        // `index % 4 == 1` alone doesn't guarantee enough
                        // operands: the only value satisfying it below 5 is 1
                        // itself, a single lone coordinate with no complete
                        // curve to pair it with. Every read below (the
                        // `index - 5` here and the `% 8 == 1` block's own
                        // `index - 5`/`- 4`/`- 3`) assumes a full curve (4)
                        // plus that odd trailing coordinate (1) are both
                        // actually present, i.e. `index >= 5`.
                        if (*stack).index.wrapping_rem(4 as Arity) == 1 as Arity
                            && (*stack).index < 5 as Arity
                        {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_vhcurveto (30). This operation is ignored.\n",
                                ),
                            );
                        } else {
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
                                if i.wrapping_div(4 as u32).wrapping_rem(2 as u32) == 0 as u32 {
                                    callback_draw_curveto(
                                        outline,
                                        0.0f64,
                                        cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(1 as u32) as isize),
                                        ),
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(2 as u32) as isize),
                                        ),
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(3 as u32) as isize),
                                        ),
                                        0.0f64,
                                    );
                                } else {
                                    callback_draw_curveto(
                                        outline,
                                        cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                        0.0f64,
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(1 as u32) as isize),
                                        ),
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(2 as u32) as isize),
                                        ),
                                        0.0f64,
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(3 as u32) as isize),
                                        ),
                                    );
                                }
                                i = i.wrapping_add(4 as u32);
                            }
                            if (*stack).index.wrapping_rem(8 as Arity) == 5 as Arity {
                                callback_draw_curveto(
                                    outline,
                                    0.0f64,
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(5 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(4 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(3 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(1 as Arity) as isize,
                                        ),
                                    ),
                                );
                            }
                            if (*stack).index.wrapping_rem(8 as Arity) == 1 as Arity {
                                callback_draw_curveto(
                                    outline,
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(5 as Arity) as isize,
                                        ),
                                    ),
                                    0.0f64,
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(4 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(3 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(1 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize,
                                        ),
                                    ),
                                );
                            }
                        }
                        (*stack).index = 0 as Arity;
                    }
                    31 => {
                        // Same reasoning as op 30 above: `index % 4 == 1`
                        // with `index < 5` means exactly `index == 1`, a
                        // lone coordinate with no complete curve behind it.
                        if (*stack).index.wrapping_rem(4 as Arity) == 1 as Arity
                            && (*stack).index < 5 as Arity
                        {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hvcurveto (31). This operation is ignored.\n",
                                ),
                            );
                        } else {
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
                                if i.wrapping_div(4 as u32).wrapping_rem(2 as u32) == 0 as u32 {
                                    callback_draw_curveto(
                                        outline,
                                        cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                        0.0f64,
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(1 as u32) as isize),
                                        ),
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(2 as u32) as isize),
                                        ),
                                        0.0f64,
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(3 as u32) as isize),
                                        ),
                                    );
                                } else {
                                    callback_draw_curveto(
                                        outline,
                                        0.0f64,
                                        cffnum(*(*stack).stack.as_mut_ptr().offset(i as isize)),
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(1 as u32) as isize),
                                        ),
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(2 as u32) as isize),
                                        ),
                                        cffnum(
                                            *(*stack)
                                                .stack
                                                .as_mut_ptr()
                                                .offset(i.wrapping_add(3 as u32) as isize),
                                        ),
                                        0.0f64,
                                    );
                                }
                                i = i.wrapping_add(4 as u32);
                            }
                            if (*stack).index.wrapping_rem(8 as Arity) == 5 as Arity {
                                callback_draw_curveto(
                                    outline,
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(5 as Arity) as isize,
                                        ),
                                    ),
                                    0.0f64,
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(4 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(3 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(1 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize,
                                        ),
                                    ),
                                );
                            }
                            if (*stack).index.wrapping_rem(8 as Arity) == 1 as Arity {
                                callback_draw_curveto(
                                    outline,
                                    0.0f64,
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(5 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(4 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(3 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize,
                                        ),
                                    ),
                                    cffnum(
                                        *(*stack).stack.as_mut_ptr().offset(
                                            (*stack).index.wrapping_sub(1 as Arity) as isize,
                                        ),
                                    ),
                                );
                            }
                        }
                        (*stack).index = 0 as Arity;
                    }
                    3106 => {
                        if (*stack).index < 7 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hflex\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_HFLEX.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(0 as ::core::ffi::c_int as isize),
                                ),
                                0.0f64,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(2 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(3 as ::core::ffi::c_int as isize),
                                ),
                                0.0f64,
                            );
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(4 as ::core::ffi::c_int as isize),
                                ),
                                0.0f64,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(5 as ::core::ffi::c_int as isize),
                                ),
                                -cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(2 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(6 as ::core::ffi::c_int as isize),
                                ),
                                0.0f64,
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    3107 => {
                        if (*stack).index < 12 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_flex\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_FLEX.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(0 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(2 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(3 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(4 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(5 as ::core::ffi::c_int as isize),
                                ),
                            );
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(6 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(7 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(8 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(9 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(10 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(11 as ::core::ffi::c_int as isize),
                                ),
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    3108 => {
                        if (*stack).index < 9 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_hflex1\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_HFLEX1.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(0 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(2 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(3 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(4 as ::core::ffi::c_int as isize),
                                ),
                                0.0f64,
                            );
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(5 as ::core::ffi::c_int as isize),
                                ),
                                0.0f64,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(6 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(7 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(8 as ::core::ffi::c_int as isize),
                                ),
                                -(cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ) + cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(3 as ::core::ffi::c_int as isize),
                                ) + cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(7 as ::core::ffi::c_int as isize),
                                )),
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    3109 => {
                        if (*stack).index < 11 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_flex1\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_FLEX1.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut dx: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset(0 as ::core::ffi::c_int as isize),
                            ) + cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset(2 as ::core::ffi::c_int as isize),
                            ) + cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset(4 as ::core::ffi::c_int as isize),
                            ) + cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset(6 as ::core::ffi::c_int as isize),
                            ) + cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset(8 as ::core::ffi::c_int as isize),
                            );
                            let mut dy: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset(1 as ::core::ffi::c_int as isize),
                            ) + cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset(3 as ::core::ffi::c_int as isize),
                            ) + cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset(5 as ::core::ffi::c_int as isize),
                            ) + cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset(7 as ::core::ffi::c_int as isize),
                            ) + cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset(9 as ::core::ffi::c_int as isize),
                            );
                            if fabs(dx) > fabs(dy) {
                                dx = cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(10 as ::core::ffi::c_int as isize),
                                );
                                dy = -dy;
                            } else {
                                dx = -dx;
                                dy = cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(10 as ::core::ffi::c_int as isize),
                                );
                            }
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(0 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(1 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(2 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(3 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(4 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(5 as ::core::ffi::c_int as isize),
                                ),
                            );
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(6 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(7 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(8 as ::core::ffi::c_int as isize),
                                ),
                                cffnum(
                                    *(*stack)
                                        .stack
                                        .as_mut_ptr()
                                        .offset(9 as ::core::ffi::c_int as isize),
                                ),
                                dx,
                                dy,
                            );
                            (*stack).index = 0 as Arity;
                        }
                    }
                    3075 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_and\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_AND.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num1: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            let num2: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize)) =
                                CffValue::Double(if num1 != 0. && num2 != 0. {
                                    1.0f64
                                } else {
                                    0.0f64
                                });
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3076 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_or\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_OR.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num1_0: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            let num2_0: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize)) =
                                CffValue::Double(if num1_0 != 0. || num2_0 != 0. {
                                    1.0f64
                                } else {
                                    0.0f64
                                });
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3077 => {
                        if (*stack).index < 1 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_not\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_NOT.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize)) =
                                CffValue::Double(if num != 0. { 0.0f64 } else { 1.0f64 });
                        }
                    }
                    3081 => {
                        if (*stack).index < 1 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_abs\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_ABS.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num_0: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize)) =
                                CffValue::Double(if num_0 < 0.0f64 { -num_0 } else { num_0 });
                        }
                    }
                    3082 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_add\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_ADD.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num1_1: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            let num2_1: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize)) =
                                CffValue::Double(num1_1 + num2_1);
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3083 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_sub\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_SUB.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num1_2: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            );
                            let num2_2: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize)) =
                                CffValue::Double(num1_2 - num2_2);
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3084 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_div\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_DIV.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num1_3: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            );
                            let num2_3: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize)) =
                                CffValue::Double(num1_3 / num2_3);
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3086 => {
                        if (*stack).index < 1 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_neg\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_NEG.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num_1: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize)) =
                                CffValue::Double(-num_1);
                        }
                    }
                    3087 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_eq\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_EQ.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num1_4: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            let num2_4: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize)) =
                                CffValue::Double(if num1_4 == num2_4 { 1.0f64 } else { 0.0f64 });
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3090 => {
                        if (*stack).index < 1 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
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
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_put\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_PUT.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let val_0: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            );
                            let i_0: i32 = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            ) as i32;
                            (*stack).transient[(i_0 % TYPE2_TRANSIENT_ARRAY as i32) as usize] =
                                CffValue::Double(val_0);
                            (*stack).index = (*stack).index.wrapping_sub(2 as Arity);
                        }
                    }
                    3093 => {
                        if (*stack).index < 1 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_get\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_GET.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let i_1: i32 = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            ) as i32;
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize)) =
                                CffValue::Double(cffnum(
                                    (*stack).transient
                                        [(i_1 % TYPE2_TRANSIENT_ARRAY as i32) as usize],
                                ));
                        }
                    }
                    3094 => {
                        if (*stack).index < 4 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_ifelse\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_IFELSE.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let v2: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            let v1: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            );
                            let s2: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(3 as Arity) as isize),
                            );
                            let s1: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(4 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(4 as Arity) as isize)) =
                                CffValue::Double(if v1 <= v2 { s1 } else { s2 });
                            (*stack).index = (*stack).index.wrapping_sub(3 as Arity);
                        }
                    }
                    3095 => {
                        if ((*stack).index as usize) < (*stack).stack.len() {
                            *(*stack).stack.as_mut_ptr().offset((*stack).index as isize) =
                                CffValue::Double(callback_draw_getrand(outline));
                            (*stack).index = (*stack).index.wrapping_add(1 as Arity);
                        } else {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Operand stack overflow in Type 2 CharString; ",
                                    b"the rest of this outline is ignored.\n",
                                ),
                            );
                            return;
                        }
                    }
                    3096 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_mul\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_MUL.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num1_5: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            let num2_5: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize)) =
                                CffValue::Double(num1_5 * num2_5);
                            (*stack).index = (*stack).index.wrapping_sub(1 as Arity);
                        }
                    }
                    3098 => {
                        if (*stack).index < 1 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_sqrt\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_SQRT.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num_2: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize)) =
                                CffValue::Double(sqrt(num_2));
                        }
                    }
                    3099 => {
                        if (*stack).index < 1 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_dup\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_DUP.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else if ((*stack).index as usize) < (*stack).stack.len() {
                            *(*stack).stack.as_mut_ptr().offset((*stack).index as isize) =
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize);
                            (*stack).index = (*stack).index.wrapping_add(1 as Arity);
                        } else {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Operand stack overflow in Type 2 CharString; ",
                                    b"the rest of this outline is ignored.\n",
                                ),
                            );
                            return;
                        }
                    }
                    3100 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_exch\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_EXCH.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let num1_6: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            );
                            let num2_6: ::core::ffi::c_double = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            );
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(1 as Arity) as isize)) =
                                CffValue::Double(num2_6);
                            (*(*stack)
                                .stack
                                .as_mut_ptr()
                                .offset((*stack).index.wrapping_sub(2 as Arity) as isize)) =
                                CffValue::Double(num1_6);
                        }
                    }
                    3101 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_index\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_INDEX.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let n: u8 = (*stack).index.wrapping_sub(1 as Arity) as u8;
                            let j_1: u8 = (n as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - cffnum(*(*stack).stack.as_mut_ptr().offset(n as isize)) as u8
                                    as ::core::ffi::c_int
                                    % n as ::core::ffi::c_int)
                                as u8;
                            *(*stack).stack.as_mut_ptr().offset(n as isize) =
                                *(*stack).stack.as_mut_ptr().offset(j_1 as isize);
                        }
                    }
                    3102 => {
                        if (*stack).index < 2 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_roll\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_ROLL.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            let mut j_2: i32 = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(1 as Arity) as isize),
                            ) as i32;
                            let n_0: u32 = cffnum(
                                *(*stack)
                                    .stack
                                    .as_mut_ptr()
                                    .offset((*stack).index.wrapping_sub(2 as Arity) as isize),
                            ) as u32;
                            if (*stack).index < (2 as u32).wrapping_add(n_0) {
                                logger_log_sds(
                                    &mut *options.logger.borrow_mut(),
                                    LOG_VL_IMPORTANT,
                                    LoggerType::Warning,
                                    crate::bytesbuild!(
                                        b"[libcff] Stack cannot provide enough parameters for ",
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
                                    let last: u8 =
                                        (*stack).index.wrapping_sub(3 as Arity) as u8;
                                    let first: u8 = (*stack)
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
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_callsubr\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_CALLSUBR.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1);
                            let subr: u32 = cffnum(
                                *(*stack).stack.as_mut_ptr().offset((*stack).index as isize),
                            ) as u32;
                            if let Some((sub_data, sub_len)) = locate_subr(lsubr, lsubr_bias, subr)
                            {
                                cff_parse_outline(
                                    sub_data as *mut u8,
                                    sub_len,
                                    gsubr,
                                    lsubr,
                                    stack,
                                    outline,
                                    options,
                                    depth + 1,
                                );
                            } else {
                                logger_log_sds(
                                    &mut *options.logger.borrow_mut(),
                                    LOG_VL_IMPORTANT,
                                    LoggerType::Warning,
                                    crate::bytesbuild!(
                                        b"[libcff] Invalid local subroutine index for ",
                                        b"op_callsubr\0" as *const u8 as *const ::core::ffi::c_char,
                                        b" (",
                                        Hex4(OP_CALLSUBR.0 as u32),
                                        b"). This call is ignored.\n",
                                    ),
                                );
                            }
                        }
                    }
                    29 => {
                        if (*stack).index < 1 as Arity {
                            logger_log_sds(
                                &mut *options.logger.borrow_mut(),
                                LOG_VL_IMPORTANT,
                                LoggerType::Warning,
                                crate::bytesbuild!(
                                    b"[libcff] Stack cannot provide enough parameters for ",
                                    b"op_callgsubr\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (",
                                    Hex4(OP_CALLGSUBR.0 as u32),
                                    b"). This operation is ignored.\n",
                                ),
                            );
                        } else {
                            (*stack).index = (*stack).index.wrapping_sub(1);
                            let subr_0: u32 = cffnum(
                                *(*stack).stack.as_mut_ptr().offset((*stack).index as isize),
                            ) as u32;
                            if let Some((sub_data, sub_len)) =
                                locate_subr(gsubr, gsubr_bias, subr_0)
                            {
                                cff_parse_outline(
                                    sub_data as *mut u8,
                                    sub_len,
                                    gsubr,
                                    lsubr,
                                    stack,
                                    outline,
                                    options,
                                    depth + 1,
                                );
                            } else {
                                logger_log_sds(
                                    &mut *options.logger.borrow_mut(),
                                    LOG_VL_IMPORTANT,
                                    LoggerType::Warning,
                                    crate::bytesbuild!(
                                        b"[libcff] Invalid global subroutine index for ",
                                        b"op_callgsubr\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        b" (",
                                        Hex4(OP_CALLGSUBR.0 as u32),
                                        b"). This call is ignored.\n",
                                    ),
                                );
                            }
                        }
                    }
                    _ => {
                        logger_log_sds(
                            &mut *options.logger.borrow_mut(),
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(
                                b"Warning: unknown operator ",
                                op,
                                b" occurs in Type 2 CharString. It may caused by file corruption.",
                            ),
                        );
                        return;
                    }
                }
            }
            CffValue::Integer(_) | CffValue::Double(_) => {
                let fresh0 = (*stack).index;
                if (fresh0 as usize) < (*stack).stack.len() {
                    (*stack).index = (*stack).index.wrapping_add(1);
                    *(*stack).stack.as_mut_ptr().offset(fresh0 as isize) = val;
                } else {
                    logger_log_sds(
                        &mut *options.logger.borrow_mut(),
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(
                            b"[libcff] Operand stack overflow in Type 2 CharString; ",
                            b"the rest of this outline is ignored.\n",
                        ),
                    );
                    return;
                }
            }
            CffValue::Unset => {}
        }
        start = start.offset(advance as isize);
    }
}

#[cfg(test)]
mod cff_header_and_encoding_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;

    fn empty_cff_index() -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        }
    }

    unsafe fn cff_file_over(data: &[u8]) -> CffFile {
        CffFile {
            raw_data: data.as_ptr() as *mut u8,
            raw_length: data.len() as u32,
            cnt_glyph: 0,
            head: crate::libcff::CffHeader {
                major: 0,
                minor: 0,
                hdr_size: 0,
                off_size: 0,
            },
            name: empty_cff_index(),
            top_dict: empty_cff_index(),
            string: empty_cff_index(),
            global_subr: empty_cff_index(),
            encodings: CffEncoding::Unspecified,
            charsets: CffCharset::IsoAdobe,
            fdselect: CffFdSelect::Unspecified,
            char_strings: empty_cff_index(),
            font_dict: empty_cff_index(),
            local_subr: empty_cff_index(),
        }
    }

    #[test]
    fn header_fields_default_to_zero_instead_of_reading_oob() {
        // The original read the 4 fixed header bytes with no check that
        // `raw_length` was even that long.
        let data = [0x01u8]; // only 1 byte, header needs 4
        unsafe {
            let mut cff = cff_file_over(&data);
            let cff_ptr = &raw mut cff;
            // Never dereferenced on this path: `name.count == top_dict.count`
            // (both 0 for a header this short), so the only place this
            // function reads `options` -- the mismatch-count warning log --
            // is never reached. A default `Options` stands in for "never
            // used" now that the parameter is a real reference and can't be
            // null the way the old raw pointer could.
            let options: Options = Options::default();
            parse_cff_bytecode(cff_ptr, &options);
            assert_eq!(cff.head.major, 1);
            assert_eq!(cff.head.minor, 0);
            assert_eq!(cff.head.hdr_size, 0);
            assert_eq!(cff.head.off_size, 0);
        }
    }

    #[test]
    fn parse_encoding_format0_reads_the_code_array() {
        // offset=0/1 are reserved predefined-encoding sentinels, so the
        // real data starts at offset 2.
        let data = [0u8, 0, 0x00, 0x02, 5, 9]; // format=0, codes=[5,9]
        unsafe {
            let mut cff = cff_file_over(&data);
            let CffEncoding::Format0(code) = parse_encoding(&raw mut cff, 2) else {
                panic!("expected Format0");
            };
            assert_eq!(code, vec![5, 9]);
        }
    }

    #[test]
    fn parse_encoding_format0_truncated_falls_back_to_unspecified_instead_of_reading_oob() {
        let data = [0u8, 0, 0x00, 0x02, 5]; // format=0, ncodes=2, only 1 code present
        unsafe {
            let mut cff = cff_file_over(&data);
            let result = parse_encoding(&raw mut cff, 2);
            assert!(matches!(result, CffEncoding::Unspecified));
        }
    }

    #[test]
    fn parse_encoding_negative_offset_falls_back_to_unspecified_instead_of_reading_before_the_buffer()
     {
        let data = [0u8; 8];
        unsafe {
            let mut cff = cff_file_over(&data);
            let result = parse_encoding(&raw mut cff, -5);
            assert!(matches!(result, CffEncoding::Unspecified));
        }
    }
}

#[cfg(test)]
mod locate_subr_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;

    fn subr_index(offset: Vec<u32>, data: Vec<u8>) -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: (offset.len().saturating_sub(1)) as Arity,
            off_size: 1,
            offset,
            data,
        }
    }

    #[test]
    fn finds_the_first_and_second_subroutine() {
        let idx = subr_index(vec![1, 3, 5], vec![0xAA, 0xBB, 0xCC, 0xDD]);
        unsafe {
            let (p0, len0) = locate_subr(&idx, 0, 0).unwrap();
            assert_eq!(len0, 2);
            assert_eq!(
                ::core::slice::from_raw_parts(p0, len0 as usize),
                &[0xAA, 0xBB]
            );

            let (p1, len1) = locate_subr(&idx, 0, 1).unwrap();
            assert_eq!(len1, 2);
            assert_eq!(
                ::core::slice::from_raw_parts(p1, len1 as usize),
                &[0xCC, 0xDD]
            );
        }
    }

    #[test]
    fn subroutine_index_past_the_end_is_rejected_instead_of_reading_oob() {
        // The original indexed the offset array with a raw, unchecked
        // `.offset()` -- a `callsubr`/`callgsubr` operand large enough to
        // run past it read (and then recursed into) arbitrary memory.
        let idx = subr_index(vec![1, 3, 5], vec![0xAA, 0xBB, 0xCC, 0xDD]);
        unsafe {
            assert!(locate_subr(&idx, 0, 5).is_none());
        }
    }

    #[test]
    fn bias_plus_subr_overflow_is_rejected() {
        let idx = subr_index(vec![1, 3], vec![0xAA, 0xBB]);
        unsafe {
            assert!(locate_subr(&idx, u16::MAX, u32::MAX).is_none());
        }
    }

    #[test]
    fn a_zero_intermediate_offset_is_rejected() {
        // `extract_index` only validates the INDEX's *last* offset entry
        // against the wraparound bug -- an intermediate entry of 0 (not
        // a valid 1-based offset) was never checked here at all.
        let idx = subr_index(vec![1, 0, 5], vec![0xAA, 0xBB, 0xCC, 0xDD]);
        unsafe {
            assert!(locate_subr(&idx, 0, 0).is_none());
        }
    }

    #[test]
    fn a_non_monotonic_offset_pair_is_rejected() {
        let idx = subr_index(vec![5, 1], vec![0xAA, 0xBB, 0xCC, 0xDD]);
        unsafe {
            assert!(locate_subr(&idx, 0, 0).is_none());
        }
    }

    #[test]
    fn a_range_past_the_actual_data_length_is_rejected_instead_of_reading_oob() {
        // The offsets are internally consistent (monotonic, both >= 1)
        // but claim more data than `subr_index.data` actually holds.
        let idx = subr_index(vec![1, 100], vec![0xAA, 0xBB]);
        unsafe {
            assert!(locate_subr(&idx, 0, 0).is_none());
        }
    }
}

#[cfg(test)]
mod cff_parse_subr_tests {
    use super::*;
    use crate::libcff::cff_fdselect::CffFdSelect;
    use crate::libcff::cff_index::CffIndexCountType;

    fn empty_cff_index() -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        }
    }

    // The bug this pins: `fd` comes straight from the FDSelect table (a
    // glyph's declared font-dict index) with nothing above validating it
    // against `fdarray`'s actual size. `fdarray.offset` (an INDEX's
    // `count + 1` offsets) was then indexed with a raw, unchecked
    // `.offset()` -- an `fd` past `fdarray.count` read arbitrary memory
    // past the `Vec`'s allocation, a real SEGV a local fuzzing run found.
    #[test]
    fn fd_select_index_past_fdarray_count_is_rejected_instead_of_reading_oob() {
        // One font dict (`count: 1`), but the FDSelect claims glyph 0
        // belongs to font dict 99.
        let fdarray = CffIndex {
            count_type: CffIndexCountType::U16,
            count: 1,
            off_size: 1,
            offset: vec![1, 1],
            data: Vec::new(),
        };
        let select = CffFdSelect::Format0(vec![99]);
        let mut subr = empty_cff_index();
        unsafe {
            let fd = cff_parse_subr(
                0,
                ::core::ptr::null_mut(),
                0,
                &fdarray,
                &select,
                &raw mut subr,
            );
            assert_eq!(fd, 99);
            assert_eq!(subr.count, 0);
        }
    }
}
