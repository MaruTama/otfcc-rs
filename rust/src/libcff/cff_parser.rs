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
use crate::libcff::cff_index::{empty_index, extract_index, get_index_length, new_empty_cff_index};
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
// `MAX_SUBR_CALL_DEPTH` bounds how deep `callsubr`/`callgsubr` can nest,
// but says nothing about how many calls happen *within* one nesting level
// -- a single charstring can invoke hundreds of different subroutines one
// after another, each of which is itself within the depth limit and can
// invoke hundreds more. That makes total work exponential in depth even
// though nesting itself never exceeds 10: a subroutine graph shaped like a
// K-ary tree of depth 10 does on the order of K^10 total operator
// evaluations, unrelated to the recursion-depth guard entirely -- the same
// "billion laughs" amplification shape XML entity expansion is named for,
// here via CFF subroutine calls instead of entity references. Found by
// `cargo fuzz run otf_parse`: a mutated CFF table hung for 30+ seconds and
// grew past the 2GB fuzzer memory limit on an input with no other
// attacker-reachable slow path (confirmed by zeroing every other table in
// the same file and rerunning -- only the CFF table's presence mattered).
// A shared, whole-glyph call budget (independent of the depth counter)
// closes it the same way real Type 2 Charstring interpreters (e.g.
// FreeType's `cff_decoder_parse_charstrings`) bound total operator/call
// count, not just nesting depth. 10,000 is far beyond what any real
// subroutinized font's single glyph needs (`KRName-Regular-O2.otf`, the
// only `-O2`/subroutinize-exercising payload in `tests/payload/`, needs
// nowhere near it) while stopping the amplification attack at a small
// fraction of a second.
const MAX_TOTAL_SUBR_CALLS: u32 = 10_000;
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
    let mut pos: u32;
    let offset: i32;
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
    pos = 4_u32.wrapping_add(get_index_length(&raw mut (*cff).name));
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
    pos = 4_u32
        .wrapping_add(get_index_length(&raw mut (*cff).name))
        .wrapping_add(get_index_length(&raw mut (*cff).top_dict));
    extract_index(
        (*cff).raw_data,
        (*cff).raw_length,
        pos,
        &raw mut (*cff).string,
    );
    pos = 4_u32
        .wrapping_add(get_index_length(&raw mut (*cff).name))
        .wrapping_add(get_index_length(&raw mut (*cff).top_dict))
        .wrapping_add(get_index_length(&raw mut (*cff).string));
    extract_index(
        (*cff).raw_data,
        (*cff).raw_length,
        pos,
        &raw mut (*cff).global_subr,
    );
    // The Top DICT INDEX's `data` is the concatenation of every entry's
    // dict bytes; entry 0 (the only one a well-formed OpenType CFF table
    // ever has, per `(*cff).name.count != (*cff).top_dict.count`'s warning
    // below) starts at `offset[0] - 1`, which `extract_index`'s validation
    // guarantees is 0 (CFF INDEX offsets are 1-based). Computed once and
    // reused for every key looked up in the Top DICT below -- previously
    // each lookup recomputed the identical `data.as_ptr()` + manual
    // offset-diff pointer pair from scratch. `.get(..len)` (rather than a
    // raw pointer) makes the "entry 0 starts at 0" assumption load-bearing
    // instead of implicit: a `top_dict` INDEX with more than one entry now
    // safely gets just its first entry's bytes instead of silently reading
    // past them.
    let top_dict_bytes: &[u8] = if !(*cff).top_dict.data.is_empty() {
        let top_dict_offset = &(*cff).top_dict.offset;
        let top_dict_len = top_dict_offset[1].wrapping_sub(top_dict_offset[0]) as usize;
        let top_dict_data: &[u8] = &(*cff).top_dict.data;
        top_dict_data.get(..top_dict_len).unwrap_or(&[])
    } else {
        &[]
    };
    if !(*cff).top_dict.data.is_empty() {
        let mut offset_0: i32;
        offset_0 = parse_dict_key_int(top_dict_bytes, OP_CHAR_STRINGS, 0_u32);
        if offset_0 != -1_i32 {
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
        offset_0 = parse_dict_key_int(top_dict_bytes, OP_ENCODING, 0_u32);
        if offset_0 != -1_i32 {
            (*cff).encodings = parse_encoding(cff, offset_0);
        } else {
            (*cff).encodings = CffEncoding::Unspecified;
        }
        offset_0 = parse_dict_key_int(top_dict_bytes, OP_CHARSET, 0_u32);
        if offset_0 != -1_i32 {
            (*cff).charsets = cff_extract_charset(
                (*cff).raw_data,
                (*cff).raw_length,
                offset_0,
                (*cff).char_strings.count as u16,
            );
        } else {
            (*cff).charsets = CffCharset::IsoAdobe;
        }
        offset_0 = parse_dict_key_int(top_dict_bytes, OP_FD_SELECT, 0_u32);
        if (*cff).char_strings.count != 0 && offset_0 != -1_i32 {
            (*cff).fdselect = cff_extract_fd_select(
                (*cff).raw_data,
                (*cff).raw_length,
                offset_0,
                (*cff).char_strings.count as u16,
            );
        } else {
            (*cff).fdselect = CffFdSelect::Unspecified;
        }
        offset_0 = parse_dict_key_int(top_dict_bytes, OP_FD_ARRAY, 0_u32);
        if offset_0 != -1_i32 {
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
    let mut private_len: i32 = -1_i32;
    let mut private_off: i32 = -1_i32;
    if !(*cff).top_dict.data.is_empty() {
        private_len = parse_dict_key_int(top_dict_bytes, OP_PRIVATE, 0_u32);
        private_off = parse_dict_key_int(top_dict_bytes, OP_PRIVATE, 1_u32);
    }
    // `private_off`/`private_len` are the Private DICT's own `offset`/
    // `length` operands -- values taken straight from the font's (attacker-
    // controlled) Top DICT bytes, not yet validated against the actual
    // buffer. The original turned them directly into `raw_data.offset(
    // private_off)` with no check that `private_off + private_len` stays
    // inside `raw_length` at all -- an out-of-bounds read the moment either
    // operand pointed past the real buffer. Building one bounds-checked
    // slice via `.get(start..).and_then(|s| s.get(..len))` and only calling
    // `parse_dict_key_int` when that succeeds closes it; a negative operand
    // or an out-of-range pair now falls through to the same `empty_index`
    // fallback the "no Private key at all" case already used.
    let private_dict_bytes: Option<&[u8]> = if private_off >= 0 && private_len >= 0 {
        let raw_slice = ::core::slice::from_raw_parts((*cff).raw_data, (*cff).raw_length as usize);
        raw_slice
            .get(private_off as usize..)
            .and_then(|s| s.get(..private_len as usize))
    } else {
        None
    };
    if let Some(private_bytes) = private_dict_bytes {
        offset = parse_dict_key_int(private_bytes, OP_SUBRS, 0_u32);
        if offset != -1_i32 {
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
    // `CffFile` owns several `Vec`-backed fields (each `CffIndex`'s
    // `offset`/`data`, and `CffEncoding`/`CffCharset`/`CffFdSelect`'s
    // `Vec`-carrying variants) -- calloc'ing it and then letting
    // `parse_cff_bytecode` fill each field in with a plain `(*file).field
    // = value;` assignment is UB the instant the first such assignment
    // runs: the assignment drops the *old* value first, and an all-zero
    // bit pattern is never a valid `Vec`/enum-with-a-`Vec`-variant to
    // begin with ("constructing invalid value... encountered 0" under
    // Miri) -- see [[otfcc-vec-field-assign-needs-calloc]]. The same bug
    // also fires on `cff_close`ing a malformed font whose empty Top DICT
    // left `char_strings`/`font_dict`/`encodings`/`charsets`/`fdselect`
    // never written by `parse_cff_bytecode` at all: `cff_close` disposes
    // them unconditionally, which is the identical first-write-onto-
    // zeroed-memory pattern. Building the whole value via `Box::new` up
    // front (instead of calloc) closes both: every field starts out as a
    // real, valid (empty) value, so every later plain `=` -- in
    // `parse_cff_bytecode` or in `cff_close` -- safely drops a real prior
    // value instead of an invalid zeroed one.
    let file: *mut CffFile = Box::into_raw(Box::new(CffFile {
        raw_data: ::core::ptr::null_mut(),
        raw_length: 0,
        cnt_glyph: 0,
        head: crate::libcff::CffHeader {
            major: 0,
            minor: 0,
            hdr_size: 0,
            off_size: 0,
        },
        name: new_empty_cff_index(),
        top_dict: new_empty_cff_index(),
        string: new_empty_cff_index(),
        global_subr: new_empty_cff_index(),
        encodings: CffEncoding::Unspecified,
        charsets: CffCharset::IsoAdobe,
        fdselect: CffFdSelect::Unspecified,
        char_strings: new_empty_cff_index(),
        font_dict: new_empty_cff_index(),
        local_subr: new_empty_cff_index(),
    }));
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
    (*file).cnt_glyph = 0_u16;
    parse_cff_bytecode(file, options);
    return file;
}
pub unsafe fn cff_close(file: *mut CffFile) {
    if !file.is_null() {
        if !(*file).raw_data.is_null() {
            free((*file).raw_data as *mut ::core::ffi::c_void);
            (*file).raw_data = ::core::ptr::null_mut::<u8>();
        }
        // `file` is `Box`-allocated now (`cff_open_stream`), not calloc'd
        // -- `Box::from_raw` + `drop` runs every field's own `Drop` (each
        // `CffIndex`'s `Vec`s, the `CffEncoding`/`CffCharset`/
        // `CffFdSelect` variants' `Vec`s) and reclaims the struct's own
        // memory correctly, replacing the manual per-field
        // `cff_index_dispose`/reset calls and the raw `free()` a calloc'd
        // struct used to need (a bare `free()` doesn't run Drop glue, and
        // freeing a `Box` allocation with libc's `free()` would itself be
        // a mismatched-allocator UB).
        drop(Box::from_raw(file));
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
    let mut fd: u8 = 0_u8;
    let off_private: i32;
    let len_private: i32;
    let off_subr: i32;
    match select {
        CffFdSelect::Format0(fds) => {
            fd = fds[idx as usize];
        }
        CffFdSelect::Format3 { range3, sentinel } => {
            let mut i: i32 = 0_i32;
            while i < range3.len() as i32 - 1_i32 {
                if idx as i32 >= range3[i as usize].first as i32
                    && (idx as i32)
                        < range3[(i + 1_i32) as usize].first as i32
                {
                    fd = range3[i as usize].fd;
                }
                i += 1;
            }
            if idx as i32
                >= range3[range3.len() - 1_usize].first as i32
                && (idx as i32) < *sentinel as i32
            {
                fd = range3[range3.len() - 1_usize].fd;
            }
        }
        CffFdSelect::Unspecified => {
            fd = 0_u8;
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
    // `fd < fdarray.count` is already guaranteed by the early return above,
    // and `extract_index` guarantees `fdarray.offset.len() == fdarray.count
    // + 1` and that every entry is a valid, non-decreasing 1-based offset
    // into `fdarray.data` -- so this FD's dict-data slice is always in
    // bounds. `.get(start..).and_then(|s| s.get(..len))` makes that
    // structural guarantee explicit instead of relying on raw pointer
    // arithmetic to happen to land inside the allocation.
    let fd_dict_start = fdarray.offset[fd as usize].wrapping_sub(1) as usize;
    let fd_dict_len =
        fdarray.offset[fd as usize + 1].wrapping_sub(fdarray.offset[fd as usize]) as usize;
    let fd_dict_bytes = fdarray
        .data
        .get(fd_dict_start..)
        .and_then(|s| s.get(..fd_dict_len))
        .unwrap_or(&[]);
    off_private = parse_dict_key_int(fd_dict_bytes, OP_PRIVATE, 1_u32);
    len_private = parse_dict_key_int(fd_dict_bytes, OP_PRIVATE, 0_u32);
    // Same bounds hole as `parse_cff_bytecode`'s Local Subrs lookup above:
    // `off_private`/`len_private` are Private-DICT-controlled operands,
    // unvalidated against `raw_length` until now.
    let private_dict_bytes: Option<&[u8]> = if off_private >= 0 && len_private >= 0 {
        let raw_slice = ::core::slice::from_raw_parts(raw, raw_length as usize);
        raw_slice
            .get(off_private as usize..)
            .and_then(|s| s.get(..len_private as usize))
    } else {
        None
    };
    if let Some(private_bytes) = private_dict_bytes {
        off_subr = parse_dict_key_int(private_bytes, OP_SUBRS, 0_u32);
        if off_subr != -1_i32 {
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
    if (cnt as i32) < 1240_i32 {
        return 107_u16;
    } else if (cnt as i32) < 33900_i32 {
        return 1131_u16;
    } else {
        return 32768_u16;
    };
}
// The original's two-pointer swap-until-cross is a plain reversal of
// `stack[left..=right]` -- but only when `left <= right`: `p1 < p2`
// starts out false (a graceful no-op) whenever `left > right`, while
// indexing with an inverted `a..=b` range panics rather than yielding
// an empty slice, so that guard has to be explicit here where it was
// implicit in the pointer comparison.
unsafe fn reverse_stack(stack: *mut CffStack, left: u8, right: u8) {
    if left <= right {
        (&mut (*stack).stack)[left as usize..=right as usize].reverse();
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
    total_calls: *mut u32,
) {
    if depth > MAX_SUBR_CALL_DEPTH {
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_IMPORTANT,
            LoggerType::Warning,
            crate::bytesbuild!(
                b"[libcff] Subroutine call nesting exceeded ",
                MAX_SUBR_CALL_DEPTH as i32,
                b"; the rest of this outline is ignored.\n",
            ),
        );
        return;
    }
    let gsubr_bias: u16 = compute_subr_bias(gsubr.count as u16);
    let lsubr_bias: u16 = compute_subr_bias(lsubr.count as u16);
    // `data`/`len` reconstructed as a real slice once here rather than
    // walked with `.offset()` at every step -- `pos` (into `data_slice`)
    // replaces `start` (a `*mut u8` cursor), the same "cursor into a
    // safe slice instead of raw pointer arithmetic" shape the rest of
    // this crate's parse-boundary work already uses. `cff_decode_cs2_token`
    // itself is unchanged (still takes a raw pointer + a length), since
    // it does its own `slice::from_raw_parts` reconstruction internally.
    let data_slice: &[u8] = ::core::slice::from_raw_parts(data, len as usize);
    let mut pos: usize = 0;
    let mut advance: u32;
    let mut i: u32;
    let mut cnt_bezier: u32;
    let mut val: CffValue = CffValue::Unset;
    while pos < data_slice.len() {
        // The outer loop already bounds where a token can *start*, but
        // not that the token itself stays within `len` -- a token
        // starting near the end of a truncated CharString used to read
        // past it (see `cff_codecs.rs`'s own conversion). Stop cleanly
        // instead of reading on.
        let remaining = data_slice.len() - pos;
        let Some(adv) = cff_decode_cs2_token(data_slice[pos..].as_ptr(), remaining, &raw mut val)
        else {
            break;
        };
        advance = adv;
        match val {
            CffValue::Operator(op) => {
                let mut hint_base: ::core::ffi::c_double;
                match op {
                    1 | 3 | 18 | 23 => {
                        if (*stack).index.wrapping_rem(2 as Arity) != 0 {
                            callback_draw_setwidth(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[(0_i32 as isize) as usize],
                                ),
                            );
                        }
                        // `saturating_add`, not `wrapping_add`: this counter
                        // sizes the `hintmask`/`cntrmask` bit array below and
                        // must never wrap back down to a small value while
                        // `stem_h`/`stem_v` (unbounded, real counts) keep
                        // growing -- see the `stem` field's doc comment.
                        (*stack).stem = (*stack).stem.saturating_add((*stack).index >> 1_i32);
                        hint_base = 0_i32 as ::core::ffi::c_double;
                        let mut j: u16 = (*stack).index.wrapping_rem(2 as Arity) as u16;
                        while (j as Arity) < (*stack).index {
                            let pos: ::core::ffi::c_double =
                                cffnum((&mut (*stack).stack)[(j as isize) as usize]);
                            let width: ::core::ffi::c_double =
                                cffnum((&mut (*stack).stack)[(
                                    (j as i32 + 1_i32) as isize) as usize]);
                            callback_draw_sethint(
                                outline,
                                op == OP_VSTEM.0 || op == OP_VSTEMHM.0,
                                pos + hint_base,
                                width,
                            );
                            hint_base += pos + width;
                            j = (j as i32 + 2_i32) as u16;
                        }
                        (*stack).index = 0 as Arity;
                    }
                    19 | 20 => {
                        if (*stack).index.wrapping_rem(2 as Arity) != 0 {
                            callback_draw_setwidth(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[(0_i32 as isize) as usize],
                                ),
                            );
                        }
                        let is_vertical: bool =
                            (*stack).stem as i32 > 0_i32;
                        // `saturating_add`, not `wrapping_add`: this counter
                        // sizes the `hintmask`/`cntrmask` bit array below and
                        // must never wrap back down to a small value while
                        // `stem_h`/`stem_v` (unbounded, real counts) keep
                        // growing -- see the `stem` field's doc comment.
                        (*stack).stem = (*stack).stem.saturating_add((*stack).index >> 1_i32);
                        let mut hint_base_0: ::core::ffi::c_double =
                            0_i32 as ::core::ffi::c_double;
                        let mut j_0: u16 = (*stack).index.wrapping_rem(2 as Arity) as u16;
                        while (j_0 as Arity) < (*stack).index {
                            let pos_0: ::core::ffi::c_double =
                                cffnum((&mut (*stack).stack)[(j_0 as isize) as usize]);
                            let width_0: ::core::ffi::c_double =
                                cffnum((&mut (*stack).stack)[(
                                    (j_0 as i32 + 1_i32) as isize) as usize]);
                            callback_draw_sethint(
                                outline,
                                is_vertical,
                                pos_0 + hint_base_0,
                                width_0,
                            );
                            hint_base_0 += pos_0 + width_0;
                            j_0 = (j_0 as i32 + 2_i32) as u16;
                        }
                        let mask_length: u32 =
                            ((*stack).stem as i32 + 7_i32
                                >> 3_i32) as u32;
                        // `hintmask`/`cntrmask`'s mask bytes are raw payload
                        // embedded directly in the charstring right after
                        // the opcode -- unlike every other operand, they
                        // never go through `cff_decode_cs2_token`'s own
                        // bounds checking, so nothing here previously
                        // stopped `mask_length` (driven by `(*stack).stem`,
                        // the accumulated hint count from every `hstem`/
                        // `vstem` operator already seen) from reading past
                        // the actual CharString buffer. A fuzz-found input
                        // pushed enough stem hints to make `mask_length`
                        // exceed what was left of the charstring by a
                        // single byte -- an ASan-confirmed heap-buffer-
                        // overflow. `remaining` (this token's own
                        // already-computed distance to the buffer's end)
                        // is the same bound `cff_decode_cs2_token` itself
                        // is checked against a few lines up; stop cleanly
                        // here too instead of reading past it.
                        if (advance as usize).wrapping_add(mask_length as usize) > remaining {
                            break;
                        }
                        // Sized to exactly `(*stack).stem + 7` bools, same
                        // as the original's `__caryll_allocate_clean` call
                        // -- the largest index any byte in `0..mask_length`
                        // writes is `((mask_length - 1) << 3) + 7`, and
                        // `mask_length == ((*stack).stem + 7) >> 3` keeps
                        // that within `(*stack).stem + 6` (one shy of this
                        // Vec's length) regardless of whether `stem + 7` is
                        // itself a multiple of 8.
                        let mut mask: Vec<bool> =
                            vec![false; ((*stack).stem as i32 + 7_i32) as usize];
                        let mut byte: u32 = 0_u32;
                        while byte < mask_length {
                            let mask_byte: u8 =
                                data_slice[pos + advance.wrapping_add(byte) as usize];
                            mask[(byte << 3_i32).wrapping_add(0_u32) as usize] =
                                mask_byte as i32 >> 7_i32 & 1_i32 != 0;
                            mask[(byte << 3_i32).wrapping_add(1_u32) as usize] =
                                mask_byte as i32 >> 6_i32 & 1_i32 != 0;
                            mask[(byte << 3_i32).wrapping_add(2_u32) as usize] =
                                mask_byte as i32 >> 5_i32 & 1_i32 != 0;
                            mask[(byte << 3_i32).wrapping_add(3_u32) as usize] =
                                mask_byte as i32 >> 4_i32 & 1_i32 != 0;
                            mask[(byte << 3_i32).wrapping_add(4_u32) as usize] =
                                mask_byte as i32 >> 3_i32 & 1_i32 != 0;
                            mask[(byte << 3_i32).wrapping_add(5_u32) as usize] =
                                mask_byte as i32 >> 2_i32 & 1_i32 != 0;
                            mask[(byte << 3_i32).wrapping_add(6_u32) as usize] =
                                mask_byte as i32 >> 1_i32 & 1_i32 != 0;
                            mask[(byte << 3_i32).wrapping_add(7_u32) as usize] =
                                (mask_byte as i32) & 1_i32 != 0;
                            byte = byte.wrapping_add(1);
                        }
                        callback_draw_setmask(outline, op == OP_CNTRMASK.0, &mask);
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
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                                    ),
                                );
                            }
                            callback_draw_next_contour(outline);
                            callback_draw_lineto(
                                outline,
                                0.0f64,
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
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
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(3 as Arity) as isize) as usize],
                                    ),
                                );
                            }
                            callback_draw_next_contour(outline);
                            callback_draw_lineto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
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
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                                    ),
                                );
                            }
                            callback_draw_next_contour(outline);
                            callback_draw_lineto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
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
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                                ),
                            );
                        }
                    }
                    5 => {
                        i = 0_u32;
                        while i < (*stack).index {
                            callback_draw_lineto(
                                outline,
                                cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                cffnum(
                                    (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                ),
                            );
                            i = i.wrapping_add(2_u32);
                        }
                        (*stack).index = 0 as Arity;
                    }
                    7 => {
                        if (*stack).index.wrapping_rem(2 as Arity) == 1 as Arity {
                            callback_draw_lineto(
                                outline,
                                0.0f64,
                                cffnum(
                                    (&mut (*stack).stack)[(0_i32 as isize) as usize],
                                ),
                            );
                            i = 1_u32;
                            while i < (*stack).index {
                                callback_draw_lineto(
                                    outline,
                                    cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                    0.0f64,
                                );
                                callback_draw_lineto(
                                    outline,
                                    0.0f64,
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                    ),
                                );
                                i = i.wrapping_add(2_u32);
                            }
                        } else {
                            i = 0_u32;
                            while i < (*stack).index {
                                callback_draw_lineto(
                                    outline,
                                    0.0f64,
                                    cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                );
                                callback_draw_lineto(
                                    outline,
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                    ),
                                    0.0f64,
                                );
                                i = i.wrapping_add(2_u32);
                            }
                        }
                        (*stack).index = 0 as Arity;
                    }
                    6 => {
                        if (*stack).index.wrapping_rem(2 as Arity) == 1 as Arity {
                            callback_draw_lineto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[(0_i32 as isize) as usize],
                                ),
                                0.0f64,
                            );
                            i = 1_u32;
                            while i < (*stack).index {
                                callback_draw_lineto(
                                    outline,
                                    0.0f64,
                                    cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                );
                                callback_draw_lineto(
                                    outline,
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                    ),
                                    0.0f64,
                                );
                                i = i.wrapping_add(2_u32);
                            }
                        } else {
                            i = 0_u32;
                            while i < (*stack).index {
                                callback_draw_lineto(
                                    outline,
                                    cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                    0.0f64,
                                );
                                callback_draw_lineto(
                                    outline,
                                    0.0f64,
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                    ),
                                );
                                i = i.wrapping_add(2_u32);
                            }
                        }
                        (*stack).index = 0 as Arity;
                    }
                    8 => {
                        i = 0_u32;
                        while i < (*stack).index {
                            callback_draw_curveto(
                                outline,
                                cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                cffnum(
                                    (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(i.wrapping_add(2_u32) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(i.wrapping_add(3_u32) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(i.wrapping_add(4_u32) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(i.wrapping_add(5_u32) as isize) as usize],
                                ),
                            );
                            i = i.wrapping_add(6_u32);
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
                            i = 0_u32;
                            while i < (*stack).index.wrapping_sub(2 as Arity) {
                                callback_draw_curveto(
                                    outline,
                                    cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(2_u32) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(3_u32) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(4_u32) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(5_u32) as isize) as usize],
                                    ),
                                );
                                i = i.wrapping_add(6_u32);
                            }
                            callback_draw_lineto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
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
                            i = 0_u32;
                            while i < (*stack).index.wrapping_sub(6 as Arity) {
                                callback_draw_lineto(
                                    outline,
                                    cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                    ),
                                );
                                i = i.wrapping_add(2_u32);
                            }
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(6 as Arity) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(5 as Arity) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(4 as Arity) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(3 as Arity) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
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
                                    (&mut (*stack).stack)[(0_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(1_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(2_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(3_i32 as isize) as usize],
                                ),
                                0.0f64,
                                cffnum(
                                    (&mut (*stack).stack)[(4_i32 as isize) as usize],
                                ),
                            );
                            i = 5_u32;
                            while i < (*stack).index {
                                callback_draw_curveto(
                                    outline,
                                    0.0f64,
                                    cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(2_u32) as isize) as usize],
                                    ),
                                    0.0f64,
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(3_u32) as isize) as usize],
                                    ),
                                );
                                i = i.wrapping_add(4_u32);
                            }
                        } else {
                            i = 0_u32;
                            while i < (*stack).index {
                                callback_draw_curveto(
                                    outline,
                                    0.0f64,
                                    cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(2_u32) as isize) as usize],
                                    ),
                                    0.0f64,
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(3_u32) as isize) as usize],
                                    ),
                                );
                                i = i.wrapping_add(4_u32);
                            }
                        }
                        (*stack).index = 0 as Arity;
                    }
                    27 => {
                        if (*stack).index.wrapping_rem(4 as Arity) == 1 as Arity {
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[(1_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(0_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(2_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(3_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(4_i32 as isize) as usize],
                                ),
                                0.0f64,
                            );
                            i = 5_u32;
                            while i < (*stack).index {
                                callback_draw_curveto(
                                    outline,
                                    cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                    0.0f64,
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(2_u32) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(3_u32) as isize) as usize],
                                    ),
                                    0.0f64,
                                );
                                i = i.wrapping_add(4_u32);
                            }
                        } else {
                            i = 0_u32;
                            while i < (*stack).index {
                                callback_draw_curveto(
                                    outline,
                                    cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                    0.0f64,
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(2_u32) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(i.wrapping_add(3_u32) as isize) as usize],
                                    ),
                                    0.0f64,
                                );
                                i = i.wrapping_add(4_u32);
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
                                    .wrapping_div(4 as Arity);
                            } else {
                                cnt_bezier = (*stack).index.wrapping_div(4 as Arity);
                            }
                            i = 0_u32;
                            while i < 4_u32.wrapping_mul(cnt_bezier) {
                                if i.wrapping_div(4_u32).wrapping_rem(2_u32) == 0_u32 {
                                    callback_draw_curveto(
                                        outline,
                                        0.0f64,
                                        cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                        ),
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(2_u32) as isize) as usize],
                                        ),
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(3_u32) as isize) as usize],
                                        ),
                                        0.0f64,
                                    );
                                } else {
                                    callback_draw_curveto(
                                        outline,
                                        cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                        0.0f64,
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                        ),
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(2_u32) as isize) as usize],
                                        ),
                                        0.0f64,
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(3_u32) as isize) as usize],
                                        ),
                                    );
                                }
                                i = i.wrapping_add(4_u32);
                            }
                            if (*stack).index.wrapping_rem(8 as Arity) == 5 as Arity {
                                callback_draw_curveto(
                                    outline,
                                    0.0f64,
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(5 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(4 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(3 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                                    ),
                                );
                            }
                            if (*stack).index.wrapping_rem(8 as Arity) == 1 as Arity {
                                callback_draw_curveto(
                                    outline,
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(5 as Arity) as isize) as usize],
                                    ),
                                    0.0f64,
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(4 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(3 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
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
                                    .wrapping_div(4 as Arity);
                            } else {
                                cnt_bezier = (*stack).index.wrapping_div(4 as Arity);
                            }
                            i = 0_u32;
                            while i < 4_u32.wrapping_mul(cnt_bezier) {
                                if i.wrapping_div(4_u32).wrapping_rem(2_u32) == 0_u32 {
                                    callback_draw_curveto(
                                        outline,
                                        cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                        0.0f64,
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                        ),
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(2_u32) as isize) as usize],
                                        ),
                                        0.0f64,
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(3_u32) as isize) as usize],
                                        ),
                                    );
                                } else {
                                    callback_draw_curveto(
                                        outline,
                                        0.0f64,
                                        cffnum((&mut (*stack).stack)[(i as isize) as usize]),
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(1_u32) as isize) as usize],
                                        ),
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(2_u32) as isize) as usize],
                                        ),
                                        cffnum(
                                            (&mut (*stack).stack)[(i.wrapping_add(3_u32) as isize) as usize],
                                        ),
                                        0.0f64,
                                    );
                                }
                                i = i.wrapping_add(4_u32);
                            }
                            if (*stack).index.wrapping_rem(8 as Arity) == 5 as Arity {
                                callback_draw_curveto(
                                    outline,
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(5 as Arity) as isize) as usize],
                                    ),
                                    0.0f64,
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(4 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(3 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                                    ),
                                );
                            }
                            if (*stack).index.wrapping_rem(8 as Arity) == 1 as Arity {
                                callback_draw_curveto(
                                    outline,
                                    0.0f64,
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(5 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(4 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(3 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                                    ),
                                    cffnum(
                                        (&mut (*stack).stack)[(
                                            (*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
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
                                    (&mut (*stack).stack)[(0_i32 as isize) as usize],
                                ),
                                0.0f64,
                                cffnum(
                                    (&mut (*stack).stack)[(1_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(2_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(3_i32 as isize) as usize],
                                ),
                                0.0f64,
                            );
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[(4_i32 as isize) as usize],
                                ),
                                0.0f64,
                                cffnum(
                                    (&mut (*stack).stack)[(5_i32 as isize) as usize],
                                ),
                                -cffnum(
                                    (&mut (*stack).stack)[(2_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(6_i32 as isize) as usize],
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
                                    (&mut (*stack).stack)[(0_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(1_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(2_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(3_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(4_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(5_i32 as isize) as usize],
                                ),
                            );
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[(6_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(7_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(8_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(9_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(10_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(11_i32 as isize) as usize],
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
                                    (&mut (*stack).stack)[(0_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(1_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(2_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(3_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(4_i32 as isize) as usize],
                                ),
                                0.0f64,
                            );
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[(5_i32 as isize) as usize],
                                ),
                                0.0f64,
                                cffnum(
                                    (&mut (*stack).stack)[(6_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(7_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(8_i32 as isize) as usize],
                                ),
                                -(cffnum(
                                    (&mut (*stack).stack)[(1_i32 as isize) as usize],
                                ) + cffnum(
                                    (&mut (*stack).stack)[(3_i32 as isize) as usize],
                                ) + cffnum(
                                    (&mut (*stack).stack)[(7_i32 as isize) as usize],
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
                                (&mut (*stack).stack)[(0_i32 as isize) as usize],
                            ) + cffnum(
                                (&mut (*stack).stack)[(2_i32 as isize) as usize],
                            ) + cffnum(
                                (&mut (*stack).stack)[(4_i32 as isize) as usize],
                            ) + cffnum(
                                (&mut (*stack).stack)[(6_i32 as isize) as usize],
                            ) + cffnum(
                                (&mut (*stack).stack)[(8_i32 as isize) as usize],
                            );
                            let mut dy: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[(1_i32 as isize) as usize],
                            ) + cffnum(
                                (&mut (*stack).stack)[(3_i32 as isize) as usize],
                            ) + cffnum(
                                (&mut (*stack).stack)[(5_i32 as isize) as usize],
                            ) + cffnum(
                                (&mut (*stack).stack)[(7_i32 as isize) as usize],
                            ) + cffnum(
                                (&mut (*stack).stack)[(9_i32 as isize) as usize],
                            );
                            if fabs(dx) > fabs(dy) {
                                dx = cffnum(
                                    (&mut (*stack).stack)[(10_i32 as isize) as usize],
                                );
                                dy = -dy;
                            } else {
                                dx = -dx;
                                dy = cffnum(
                                    (&mut (*stack).stack)[(10_i32 as isize) as usize],
                                );
                            }
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[(0_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(1_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(2_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(3_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(4_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(5_i32 as isize) as usize],
                                ),
                            );
                            callback_draw_curveto(
                                outline,
                                cffnum(
                                    (&mut (*stack).stack)[(6_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(7_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(8_i32 as isize) as usize],
                                ),
                                cffnum(
                                    (&mut (*stack).stack)[(9_i32 as isize) as usize],
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            let num2: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize]) =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            let num2_0: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize]) =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize]) =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize]) =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            let num2_1: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize]) =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            );
                            let num2_2: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize]) =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            );
                            let num2_3: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize]) =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize]) =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            let num2_4: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize]) =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            );
                            let i_0: i32 = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            ) as i32;
                            // `i_0` is a charstring-supplied operand, not a
                            // trusted cursor -- Rust's `%` keeps the
                            // dividend's sign, so a negative `i_0` (e.g.
                            // pushing `-1` before `put`) made this a
                            // negative array index once cast `as usize`
                            // (wrapping to a huge value), panicking. Real
                            // Type 2 charstrings only ever address this
                            // array with small in-range indices, so
                            // `rem_euclid` (always non-negative for a
                            // positive divisor) matches well-formed input
                            // exactly and just gives malformed input a
                            // well-defined slot instead of a crash.
                            (*stack).transient
                                [i_0.rem_euclid(TYPE2_TRANSIENT_ARRAY as i32) as usize] =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            ) as i32;
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize]) =
                                CffValue::Double(cffnum(
                                    // Same fix as `op_put` above: `rem_euclid`
                                    // instead of `%` so a negative `i_1`
                                    // can't turn into an out-of-bounds
                                    // array index.
                                    (*stack).transient
                                        [i_1.rem_euclid(TYPE2_TRANSIENT_ARRAY as i32) as usize],
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            let v1: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            );
                            let s2: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(3 as Arity) as isize) as usize],
                            );
                            let s1: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(4 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(4 as Arity) as isize) as usize]) =
                                CffValue::Double(if v1 <= v2 { s1 } else { s2 });
                            (*stack).index = (*stack).index.wrapping_sub(3 as Arity);
                        }
                    }
                    3095 => {
                        if ((*stack).index as usize) < (*stack).stack.len() {
                            (&mut (*stack).stack)[((*stack).index as isize) as usize] =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            let num2_5: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize]) =
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize]) =
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
                            (&mut (*stack).stack)[((*stack).index as isize) as usize] =
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize];
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            );
                            let num2_6: ::core::ffi::c_double = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            );
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize]) =
                                CffValue::Double(num2_6);
                            ((&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize]) =
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
                            // `n` is `(*stack).index - 1` truncated to `u8`
                            // -- the real value is always >= 1 here (the
                            // `index < 2` guard above already ensures at
                            // least 2 operands are on the stack), but
                            // truncation wraps `n` back to 0 whenever the
                            // real value is a multiple of 256 (a
                            // charstring pushing 257+ operands before
                            // `index`, well within the stack's real
                            // capacity). `n` is used below both as the
                            // modulus and as the stack offset the index
                            // operand itself was read from, so a truncated
                            // 0 divided by zero and panicked. Treat it the
                            // same as "not enough operands": skip the
                            // operation instead.
                            if n == 0 {
                                logger_log_sds(
                                    &mut *options.logger.borrow_mut(),
                                    LOG_VL_IMPORTANT,
                                    LoggerType::Warning,
                                    crate::bytesbuild!(
                                        b"[libcff] op_index",
                                        b" (",
                                        Hex4(OP_INDEX.0 as u32),
                                        b") operand count overflowed a byte; this operation is ignored.\n",
                                    ),
                                );
                            } else {
                                let j_1: u8 = (n as i32
                                    - 1_i32
                                    - cffnum((&mut (*stack).stack)[(n as isize) as usize]) as u8
                                        as i32
                                        % n as i32)
                                    as u8;
                                (&mut (*stack).stack)[(n as isize) as usize] =
                                    (&mut (*stack).stack)[(j_1 as isize) as usize];
                            }
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
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(1 as Arity) as isize) as usize],
                            ) as i32;
                            let n_0: u32 = cffnum(
                                (&mut (*stack).stack)[((*stack).index.wrapping_sub(2 as Arity) as isize) as usize],
                            ) as u32;
                            if (*stack).index < 2_u32.wrapping_add(n_0) {
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
                            } else if n_0 == 0 {
                                // `n_0` (the roll's element count operand)
                                // is charstring-supplied and cast `as u32`
                                // from a float, which saturates any
                                // negative value to 0 -- so pushing `0` or
                                // a negative count for N reaches here.
                                // "roll 0 elements" is a legitimate no-op
                                // (the `j_2 == 0` branch a few lines down
                                // already treats "nothing to rotate" as a
                                // no-op the same way), but the
                                // `wrapping_rem(n_0)` below panics on a
                                // zero divisor -- skip it instead.
                            } else {
                                j_2 = (-j_2 as u32).wrapping_rem(n_0) as i32;
                                if j_2 < 0_i32 {
                                    j_2 = (j_2 as u32).wrapping_add(n_0) as i32;
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
                                        (last as i32 - j_2 + 1_i32) as u8,
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
                                (&mut (*stack).stack)[((*stack).index as isize) as usize],
                            ) as u32;
                            if let Some((sub_data, sub_len)) = locate_subr(lsubr, lsubr_bias, subr)
                            {
                                *total_calls = (*total_calls).wrapping_add(1);
                                if *total_calls > MAX_TOTAL_SUBR_CALLS {
                                    if *total_calls == MAX_TOTAL_SUBR_CALLS + 1 {
                                        logger_log_sds(
                                            &mut *options.logger.borrow_mut(),
                                            LOG_VL_IMPORTANT,
                                            LoggerType::Warning,
                                            crate::bytesbuild!(
                                                b"[libcff] Subroutine call budget (",
                                                MAX_TOTAL_SUBR_CALLS as i32,
                                                b") exceeded; the rest of this outline is ignored.\n",
                                            ),
                                        );
                                    }
                                } else {
                                    cff_parse_outline(
                                        sub_data as *mut u8,
                                        sub_len,
                                        gsubr,
                                        lsubr,
                                        stack,
                                        outline,
                                        options,
                                        depth + 1,
                                        total_calls,
                                    );
                                }
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
                                (&mut (*stack).stack)[((*stack).index as isize) as usize],
                            ) as u32;
                            if let Some((sub_data, sub_len)) =
                                locate_subr(gsubr, gsubr_bias, subr_0)
                            {
                                *total_calls = (*total_calls).wrapping_add(1);
                                if *total_calls > MAX_TOTAL_SUBR_CALLS {
                                    if *total_calls == MAX_TOTAL_SUBR_CALLS + 1 {
                                        logger_log_sds(
                                            &mut *options.logger.borrow_mut(),
                                            LOG_VL_IMPORTANT,
                                            LoggerType::Warning,
                                            crate::bytesbuild!(
                                                b"[libcff] Subroutine call budget (",
                                                MAX_TOTAL_SUBR_CALLS as i32,
                                                b") exceeded; the rest of this outline is ignored.\n",
                                            ),
                                        );
                                    }
                                } else {
                                    cff_parse_outline(
                                        sub_data as *mut u8,
                                        sub_len,
                                        gsubr,
                                        lsubr,
                                        stack,
                                        outline,
                                        options,
                                        depth + 1,
                                        total_calls,
                                    );
                                }
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
                if ((*stack).index as usize) < (*stack).stack.len() {
                    (&mut (*stack).stack)[((*stack).index as isize) as usize] = val;
                    (*stack).index = (*stack).index.wrapping_add(1);
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
        pos += advance as usize;
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

    #[test]
    fn private_dict_offset_past_raw_length_yields_no_local_subrs_instead_of_reading_oob() {
        // A hand-built, otherwise well-formed 28-byte CFF table (header +
        // 1-entry Name INDEX + 1-entry Top DICT INDEX + empty String INDEX +
        // empty Global Subr INDEX). The Top DICT's only entry is `size 20
        // offset 32767 Private` -- a Private DICT operand pair the DICT
        // parser itself never validates (that's `parse_to_callback`'s job:
        // walk exactly `size`/`offset` bytes of *whatever pointer it's
        // given*). The original built that pointer as `raw_data.offset(
        // 32767)` unconditionally, 32739 bytes past this 28-byte buffer's
        // end -- a real out-of-bounds read `cargo miri test` confirms (see
        // the sibling test below, which reverts the fix and checks Miri
        // actually flags it). With the fix, `private_off`/`private_len` are
        // validated against `raw_length` before any pointer is built, so a
        // malformed offset like this now just yields no Local Subrs.
        let data: [u8; 28] = [
            // header: major, minor, hdrSize, offSize
            1, 0, 4, 4, // Name INDEX: count=1, offSize=1, offset=[1,2], data=[0]
            0, 1, 1, 1, 2, 0,
            // Top DICT INDEX: count=1, offSize=1, offset=[1,8],
            // data = size(20) offset(32767) Private(18)
            0, 1, 1, 1, 8, 28, 0, 20, 28, 127, 255, 18, // String INDEX: empty
            0, 0, 0, // Global Subr INDEX: empty
            0, 0, 0,
        ];
        unsafe {
            let mut cff = cff_file_over(&data);
            let cff_ptr = &raw mut cff;
            let options: Options = Options::default();
            parse_cff_bytecode(cff_ptr, &options);
            assert_eq!(cff.top_dict.count, 1, "sanity: Top DICT INDEX parsed");
            assert_eq!(cff.local_subr.count, 0);
            assert!(cff.local_subr.data.is_empty());
        }
    }
}

#[cfg(test)]
mod cff_open_stream_tests {
    use super::*;
    use crate::support::options::Options;

    // A minimal CFF blob whose Top DICT INDEX is empty (`count == 0`):
    // header + 4 empty INDEXes (Name/Top DICT/String/Global Subr). With
    // an empty Top DICT, `parse_cff_bytecode` never writes
    // `char_strings`/`font_dict`/`encodings`/`charsets`/`fdselect` at
    // all -- this exercises `cff_close`'s *unconditional* disposal of
    // those fields on whatever `cff_open_stream` initialized them to.
    //
    // Before this fix, `cff_open_stream` calloc'd the whole `CffFile`
    // and left every field an invalid all-zero bit pattern until first
    // written. `cff_close`'s disposal of the never-written fields was
    // itself the first "write" to them (a plain `=` inside
    // `cff_index_dispose`) -- UB under Miri ("constructing invalid
    // value... encountered 0") the instant that assignment drops the
    // old, invalid value, regardless of whether this test's assertions
    // below ever observe anything wrong at runtime. Building the whole
    // `CffFile` via `Box::new` up front (this fix) makes every field a
    // real, valid (empty) value from construction, so `cff_close`'s
    // disposal is always dropping a real prior value.
    #[test]
    fn open_and_close_on_a_font_with_an_empty_top_dict_does_not_construct_invalid_values() {
        let mut data: [u8; 16] = [
            1, 0, 4, 4, // header: major, minor, hdrSize, offSize
            0, 0, 0, // Name INDEX: empty
            0, 0, 0, // Top DICT INDEX: empty
            0, 0, 0, // String INDEX: empty
            0, 0, 0, // Global Subr INDEX: empty
        ];
        let options = Options::default();
        unsafe {
            let file = cff_open_stream(data.as_mut_ptr(), data.len() as u32, &options);
            assert_eq!((*file).top_dict.count, 0);
            assert_eq!((*file).char_strings.count, 0);
            assert!((*file).char_strings.data.is_empty());
            assert_eq!((*file).font_dict.count, 0);
            assert!(matches!((*file).encodings, CffEncoding::Unspecified));
            assert!(matches!((*file).charsets, CffCharset::IsoAdobe));
            assert!(matches!((*file).fdselect, CffFdSelect::Unspecified));
            assert_eq!((*file).local_subr.count, 0);
            cff_close(file);
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

#[cfg(test)]
mod cff_parse_outline_total_calls_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;
    use crate::support::options::Options;

    fn empty_cff_index() -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        }
    }

    // 108 global subroutines: the first 107 are empty (`offset[i] ==
    // offset[i + 1]`, a valid zero-length INDEX entry -- never invoked by
    // this test), the last is "push operand 0 (byte 139), return (11)".
    // With this index's count (108, `compute_subr_bias`'s bias-107
    // bracket), pushing operand 0 before `callgsubr` resolves to `bias +
    // 0` = index 107, this index's own last entry -- deliberately avoids
    // ever needing a *negative* pushed operand (which real fonts use to
    // reach a low subroutine index): `cffnum(...) as u32`, downstream of
    // this in `callgsubr`'s own handling, is a float-to-int cast, and
    // Rust's saturates a negative float to `0` rather than wrapping the
    // way the C original's cast did, so a negative operand here would
    // resolve to index `bias + 0` = 107 anyway, not to the small index it
    // looks like it should -- a real, pre-existing quirk of this already-
    // migrated cast, unrelated to this test's own purpose, sidestepped
    // instead of exercised.
    //
    // The subroutine's pushed operand is never popped by anything
    // (`return` doesn't touch the stack), so `stack.index` at the end
    // counts exactly how many times this subroutine actually *ran* -- the
    // caller's own operand push for the `callgsubr` index is always
    // popped by `callgsubr` itself before the recursion decision, so a
    // *skipped* call leaves no trace on the stack at all.
    fn one_trivial_gsubr() -> CffIndex {
        let mut offset = vec![1u32; 108];
        offset.push(3);
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 108,
            off_size: 1,
            offset,
            data: vec![139, 11], // push (byte 139 => operand 0), return
        }
    }

    // `n` copies of "push operand 0 (byte 139); callgsubr (29)" -- 2 bytes
    // each, never recursing past nesting depth 1 (see `one_trivial_gsubr`).
    fn charstring_calling_gsubr_n_times(n: u32) -> Vec<u8> {
        let mut b = Vec::with_capacity(n as usize * 2);
        for _ in 0..n {
            b.push(139); // encodes operand 0
            b.push(29); // callgsubr
        }
        b
    }

    #[test]
    // `requested` has to be within one of the real `MAX_TOTAL_SUBR_CALLS`
    // (10,000) to prove the budget lets everything through -- unlike the
    // sibling stack-operator tests in this file, shrinking the backing
    // `CffStack.stack` allocation (still done below, real but minor) barely
    // moved this test's Miri time, because the actual cost is ~10,000
    // *recursive `cff_parse_outline` calls*, not the array size. That's
    // inherent to what this test proves, the same way
    // `total_language_count_across_the_whole_table_is_capped`
    // (`table/otl/read.rs`) can't shrink its own N below the real cap
    // either. `cargo test` (native) stays the real regression guard.
    #[cfg_attr(miri, ignore = "far too slow to run meaningfully under Miri's interpreter; needs ~10,000 recursive cff_parse_outline calls to exercise the real MAX_TOTAL_SUBR_CALLS budget")]
    fn call_count_within_the_budget_all_execute() {
        let gsubr = one_trivial_gsubr();
        let lsubr = empty_cff_index();
        let requested = MAX_TOTAL_SUBR_CALLS - 1;
        let mut data = charstring_calling_gsubr_n_times(requested);
        let len = data.len() as u32;
        // 11_000 (not the real 0x10000/65536): still shrunk from
        // production's generous capacity down to just above `requested`'s
        // ~10,000 pushes, even though (per the ignore above) this isn't
        // what dominates this test's Miri time.
        let mut stack = CffStack {
            stack: vec![CffValue::Unset; 11_000],
            transient: [CffValue::Unset; TYPE2_TRANSIENT_ARRAY],
            index: 0,
            stem: 0,
        };
        let options = Options::default();
        let mut total_calls: u32 = 0;
        unsafe {
            cff_parse_outline(
                data.as_mut_ptr(),
                len,
                &gsubr,
                &lsubr,
                &raw mut stack,
                ::core::ptr::null_mut(),
                &options,
                0,
                &raw mut total_calls,
            );
        }
        assert_eq!(total_calls, requested);
        // Every one of the `requested` calls actually recursed and ran
        // its subroutine's own push.
        assert_eq!(stack.index, requested as Arity);
    }

    // The bug this pins: `MAX_SUBR_CALL_DEPTH` bounds how deep `callsubr`/
    // `callgsubr` can *nest*, but said nothing about how many calls happen
    // *within* one nesting level -- a subroutine graph with wide fan-out
    // at a shallow, spec-legal depth could still do unbounded total work.
    // `total_calls` (threaded through every recursive `cff_parse_outline`
    // call, shared across the whole glyph) is what actually bounds it.
    #[test]
    // Same Miri cost shape as the sibling test above: recursing up to
    // `MAX_TOTAL_SUBR_CALLS` (10,000) before stopping is the point of
    // this test, not something a smaller N could substitute for.
    #[cfg_attr(miri, ignore = "far too slow to run meaningfully under Miri's interpreter; needs ~10,000 recursive cff_parse_outline calls before the real MAX_TOTAL_SUBR_CALLS budget stops it")]
    fn call_count_past_the_budget_stops_recursing() {
        let gsubr = one_trivial_gsubr();
        let lsubr = empty_cff_index();
        let attempted = MAX_TOTAL_SUBR_CALLS + 500;
        let mut data = charstring_calling_gsubr_n_times(attempted);
        let len = data.len() as u32;
        // Same reasoning as the sibling test above: recursion stops at
        // `MAX_TOTAL_SUBR_CALLS` regardless of `attempted`, so 11_000
        // still has headroom above every push this test can actually
        // reach, while avoiding 0x10000's full-capacity initialization
        // cost under Miri.
        let mut stack = CffStack {
            stack: vec![CffValue::Unset; 11_000],
            transient: [CffValue::Unset; TYPE2_TRANSIENT_ARRAY],
            index: 0,
            stem: 0,
        };
        let options = Options::default();
        let mut total_calls: u32 = 0;
        unsafe {
            cff_parse_outline(
                data.as_mut_ptr(),
                len,
                &gsubr,
                &lsubr,
                &raw mut stack,
                ::core::ptr::null_mut(),
                &options,
                0,
                &raw mut total_calls,
            );
        }
        // The counter itself still climbs past the budget (every
        // `callgsubr` byte pair the outer loop walks over is one
        // attempted call, counted before the budget check decides
        // whether to recurse) -- charstring interpretation for the
        // rest of this glyph isn't aborted, only further recursion is.
        assert_eq!(total_calls, attempted);
        // But only the first `MAX_TOTAL_SUBR_CALLS` calls actually
        // recursed and ran their subroutine's own push -- this is the
        // bug's actual fix: without it, `stack.index` would reach
        // `attempted` too (each recursive call's own push landing on
        // the shared stack), the same as the within-budget test above,
        // with no way to tell the two cases apart from this assertion
        // alone.
        assert_eq!(stack.index, MAX_TOTAL_SUBR_CALLS as Arity);
    }
}

#[cfg(test)]
mod cff_parse_outline_hintmask_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;
    use crate::support::options::Options;
    use crate::table::cff::OutlineBuilderContext;
    use crate::table::glyf::otfcc_new_glyf_glyph;

    fn empty_cff_index() -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        }
    }

    // A fuzz-found font found this: `hintmask`/`cntrmask`'s mask bytes are
    // raw payload embedded directly in the charstring right after the
    // opcode -- unlike every other operand, they never go through
    // `cff_decode_cs2_token`'s own bounds checking, so nothing stopped
    // `mask_length` (driven by `(*stack).stem`, the accumulated hint count
    // from every `hstem`/`vstem` operator already seen in this charstring)
    // from reading past the actual CharString buffer -- an ASan-confirmed
    // heap-buffer-overflow.
    //
    // Charstring: push 0, push 0, `hstem` (one hint pair -> stem count
    // becomes 1, needing a 1-byte mask), `hintmask` -- with the mask byte
    // itself missing (the charstring ends right at the opcode). Reaching
    // the end of this function at all, rather than reading one byte past
    // `data`'s 4-byte allocation, is the regression signal.
    #[test]
    fn hintmask_past_the_charstring_end_stops_cleanly_instead_of_reading_oob() {
        let mut data: Vec<u8> = vec![139, 139, 1, 19];
        let len = data.len() as u32;
        let gsubr = empty_cff_index();
        let lsubr = empty_cff_index();
        let mut stack = CffStack {
            stack: vec![CffValue::Unset; 512],
            transient: [CffValue::Unset; TYPE2_TRANSIENT_ARRAY],
            index: 0,
            stem: 0,
        };
        let options = Options::default();
        let mut total_calls: u32 = 0;
        unsafe {
            let g_ptr = Box::into_raw(otfcc_new_glyf_glyph());
            let mut ctx = OutlineBuilderContext {
                g: g_ptr,
                j_contour: 0,
                j_point: 0,
                default_width_x: 0.0,
                nominal_width_x: 0.0,
                defined_h_stems: 0,
                defined_v_stems: 0,
                defined_hint_masks: 0,
                defined_contour_masks: 0,
                randx: 0,
            };
            cff_parse_outline(
                data.as_mut_ptr(),
                len,
                &gsubr,
                &lsubr,
                &raw mut stack,
                &raw mut ctx as *mut ::core::ffi::c_void,
                &options,
                0,
                &raw mut total_calls,
            );
            drop(Box::from_raw(g_ptr));
        }
        // The `hstem` operator ran (and only it -- `hintmask` bailed
        // before doing anything observable) -- `stem` reflects the one
        // hint pair pushed before the truncated `hintmask`.
        assert_eq!(stack.stem, 1);
    }

    // A second, independent fuzz-found crash in this same op family: a
    // charstring chaining enough `hstem` operators to push the *real*
    // cumulative hint count (tracked by `context.g.stem_h`, an unbounded
    // `Vec`) past 255, while `(*stack).stem` -- back when it was a `u8`
    // used to size the `hintmask` bit array -- silently wrapped back down
    // to a small value at the same point. `callback_draw_setmask` then
    // indexed the undersized array using the real (large) `stem_h.len()`,
    // an out-of-bounds panic (`table/cff.rs`, CI-found: "index out of
    // bounds: the len is 74 but the index is 716").
    //
    // 256 single-hint `hstem` calls (push 0, push 0, `hstem`) push exactly
    // 256 real entries into `stem_h` -- old `u8` arithmetic wrapped
    // `255 + 1` back to `0`; `stem` is now `u32` and must read back the
    // true 256.
    #[test]
    fn chained_hstem_operators_past_255_do_not_wrap_the_hint_count() {
        let mut data: Vec<u8> = Vec::new();
        for _ in 0..256 {
            data.extend_from_slice(&[139, 139, 1]); // push 0, push 0, hstem
        }
        data.push(19); // hintmask
        // mask_length = (256 + 7) >> 3 = 32 bytes.
        data.extend_from_slice(&[0u8; 32]);
        let len = data.len() as u32;
        let gsubr = empty_cff_index();
        let lsubr = empty_cff_index();
        let mut stack = CffStack {
            stack: vec![CffValue::Unset; 512],
            transient: [CffValue::Unset; TYPE2_TRANSIENT_ARRAY],
            index: 0,
            stem: 0,
        };
        let options = Options::default();
        let mut total_calls: u32 = 0;
        unsafe {
            let g_ptr = Box::into_raw(otfcc_new_glyf_glyph());
            let mut ctx = OutlineBuilderContext {
                g: g_ptr,
                j_contour: 0,
                j_point: 0,
                default_width_x: 0.0,
                nominal_width_x: 0.0,
                defined_h_stems: 0,
                defined_v_stems: 0,
                defined_hint_masks: 0,
                defined_contour_masks: 0,
                randx: 0,
            };
            cff_parse_outline(
                data.as_mut_ptr(),
                len,
                &gsubr,
                &lsubr,
                &raw mut stack,
                &raw mut ctx as *mut ::core::ffi::c_void,
                &options,
                0,
                &raw mut total_calls,
            );
            assert_eq!((*ctx.g).stem_h.len(), 256);
            drop(Box::from_raw(g_ptr));
        }
        assert_eq!(stack.stem, 256);
    }
}

#[cfg(test)]
mod cff_parse_outline_stack_operator_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;
    use crate::support::options::Options;

    // A charstring's `put`/`get`/`index`/`roll` operators each take a
    // charstring-supplied stack *value* (not the trusted `(*stack).index`
    // cursor) and use it as an array index or modulus divisor into a
    // small fixed-size structure (`transient[32]`, or the operand stack
    // itself), with no range check. Found by reading the interpreter
    // directly (not fuzzing) while investigating this file as the
    // successor to `cff_dict.rs`'s Private-DICT-offset fix (PR #262):
    // that fix closed an out-of-bounds *read*, these are guaranteed
    // Rust *panics* (array-index or divide-by-zero) reachable with a
    // handful of ordinary charstring bytes -- a different bug class
    // (DoS, not memory corruption), but real and previously unguarded.

    fn empty_cff_index() -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        }
    }

    // Real `CffStack.stack` is 0x10000 entries (matching the operand
    // stack's generous production capacity), but every test in this
    // module pushes at most 257 operands -- allocating and initializing
    // the full 65536-entry Vec added ~10s per test under Miri's
    // per-element provenance tracking (an otherwise-sub-10ms test suite
    // module took over a minute combined) for headroom none of these
    // tests use. 512 comfortably covers the largest case
    // (`op_index_with_operand_count_multiple_of_256_does_not_panic`'s
    // 257 pushes) while cutting the allocation two orders of magnitude;
    // the bugs these tests guard against are all about index-computation
    // correctness (negative wraparound, zero-divisor guards), not
    // anything sensitive to the backing array's total capacity.
    fn fresh_stack() -> CffStack {
        CffStack {
            stack: vec![CffValue::Unset; 512],
            transient: [CffValue::Unset; TYPE2_TRANSIENT_ARRAY],
            index: 0,
            stem: 0,
        }
    }

    unsafe fn run(data: &mut [u8], stack: &mut CffStack) {
        let gsubr = empty_cff_index();
        let lsubr = empty_cff_index();
        let options = Options::default();
        let mut total_calls: u32 = 0;
        unsafe {
            cff_parse_outline(
                data.as_mut_ptr(),
                data.len() as u32,
                &gsubr,
                &lsubr,
                &raw mut *stack,
                ::core::ptr::null_mut(),
                &options,
                0,
                &raw mut total_calls,
            );
        }
    }

    #[test]
    fn op_get_with_negative_index_operand_does_not_panic() {
        // `-1` (byte 138) then `get` (escape `12 21` = OP_GET). The
        // pre-fix `i_1 % TYPE2_TRANSIENT_ARRAY as i32` kept the
        // dividend's sign (Rust's `%`), so `i_1 == -1` produced a
        // negative remainder that panicked once cast `as usize` for the
        // `transient[]` index.
        let mut data: Vec<u8> = vec![138, 12, 21];
        let mut stack = fresh_stack();
        unsafe {
            run(&mut data, &mut stack);
        }
        // `-1` `rem_euclid` 32 == 31, a never-written transient slot --
        // `cffnum` reads that as 0.0. Reaching this assertion at all
        // (rather than panicking mid-parse) is the regression signal.
        assert_eq!(stack.index, 1);
        assert!(matches!(stack.stack[0], CffValue::Double(v) if v == 0.0));
    }

    #[test]
    fn op_put_with_negative_index_operand_does_not_panic() {
        // Push a value (0), push `-1` (byte 138) as the index, then
        // `put` (escape `12 20` = OP_PUT). Same bug/fix as `get` above.
        let mut data: Vec<u8> = vec![139, 138, 12, 20];
        let mut stack = fresh_stack();
        unsafe {
            run(&mut data, &mut stack);
        }
        assert_eq!(stack.index, 0);
        assert!(matches!(stack.transient[31], CffValue::Double(v) if v == 0.0));
    }

    #[test]
    fn op_roll_with_zero_count_operand_does_not_panic() {
        // Push J=0, push N=0, then `roll` (escape `12 30` = OP_ROLL).
        // `n_0` is `cffnum(...) as u32`, a saturating float-to-int cast
        // (a *negative* N reaches the same `n_0 == 0` path this way,
        // not just a literal 0 -- see the analogous comment in
        // `cff_parse_outline_total_calls_tests`). The pre-fix code fell
        // through to `wrapping_rem(n_0)` unconditionally once the
        // "enough operands" guard passed, panicking on the zero
        // divisor -- "roll 0 elements" is a legitimate no-op (the
        // `j_2 == 0` case a few lines below already treats "nothing to
        // rotate" the same way), not a malformed-input case.
        let mut data: Vec<u8> = vec![139, 139, 12, 30];
        let mut stack = fresh_stack();
        unsafe {
            run(&mut data, &mut stack);
        }
        // No-op: both pushed operands (J and N) are still on the stack,
        // untouched, exactly like the pre-existing `j_2 == 0` no-op case.
        assert_eq!(stack.index, 2);
    }

    #[test]
    fn op_index_with_operand_count_multiple_of_256_does_not_panic() {
        // Push 257 zero-operands (each 1 byte: value 0 encodes as byte
        // 139), then `index` (escape `12 29` = OP_INDEX). `(*stack).index
        // - 1 == 256` truncates to `0` once cast `as u8` -- the pre-fix
        // code then used that truncated `0` as both a stack offset and a
        // modulus divisor, panicking on the divide.
        let mut data: Vec<u8> = vec![139u8; 257];
        data.push(12);
        data.push(29);
        let mut stack = fresh_stack();
        unsafe {
            run(&mut data, &mut stack);
        }
        // The operation was skipped (truncated `n == 0`), not executed
        // -- reaching this assertion at all (rather than panicking
        // mid-parse) is the regression signal. All 257 pushed operands
        // are still on the stack, untouched.
        assert_eq!(stack.index, 257);
    }
}
