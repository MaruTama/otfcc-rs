use otfcc_binary::FontReader;

use crate::libcff::charset::CffCharset;
use crate::libcff::charset::cff_extract_charset;
use crate::libcff::dict::parse_dict_key_int;
use crate::libcff::fdselect::CffFdSelect;
use crate::libcff::fdselect::cff_extract_fd_select;
use crate::libcff::index::CffIndex;
use crate::libcff::index::{empty_index, extract_index, get_index_length, new_empty_cff_index};
use crate::libcff::{
    CffEncoding, CffEncodingRangeFormat1, CffEncodingSupplement, CffFile, OP_CHAR_STRINGS, OP_CHARSET, OP_ENCODING, OP_FD_ARRAY, OP_FD_SELECT,
    OP_PRIVATE, OP_SUBRS,
};

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
// toolchain too (`fuzz/README.md`), not a migration regression.
// `FDArrayTest257.otf` in the fuzz seed corpus triggers exactly this.
pub(crate) const MAX_SUBR_CALL_DEPTH: u32 = 10;
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
pub(crate) const MAX_TOTAL_SUBR_CALLS: u32 = 10_000;
// `gu1`/`gu2` (no bounds checking, no length parameter at all) are gone --
// see `libcff/index.rs`'s own conversion for the same move.
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
fn parse_encoding(cff: &CffFile, offset: i32) -> CffEncoding {
    if offset == CFF_STANDARD_ENCODING_OFFSET {
        return CffEncoding::Standard;
    } else if offset == CFF_EXPERT_ENCODING_OFFSET {
        return CffEncoding::Expert;
    }
    if offset < 0 {
        return CffEncoding::Unspecified;
    }
    // `raw_data` is a plain `Vec<u8>` now (Stage M-10) -- no raw pointer to
    // bridge into a slice, just borrow it.
    let slice = cff.raw_data.as_slice();
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
fn parse_cff_bytecode(cff: &mut CffFile) {
    let mut pos: u32;
    let offset: i32;
    // No length check guarded these 4 header-byte reads at all -- a `raw_
    // length` shorter than 4 read straight past the allocation. Every
    // field now defaults to 0 on a bounds failure instead: `extract_index`
    // below is already bounds-checked regardless of what `pos` it's given
    // (a garbage `hdr_size` just makes it fail cleanly too, same as any
    // other malformed offset), so there's nothing to gain from bailing out
    // of this function early on a too-short header.
    //
    // `raw_data` is a plain `Vec<u8>` now (Stage M-10); every other
    // operation below only reads/writes `cff`'s already-safe fields or
    // reuses this one bounds-checked `header_slice`.
    let header_slice = cff.raw_data.as_slice();
    let mut header_reader = FontReader::new(header_slice);
    cff.head.major = header_reader.u8().unwrap_or(0);
    cff.head.minor = header_reader.u8().unwrap_or(0);
    cff.head.hdr_size = header_reader.u8().unwrap_or(0);
    cff.head.off_size = header_reader.u8().unwrap_or(0);
    pos = cff.head.hdr_size as u32;
    extract_index(header_slice, pos, &mut cff.name);
    pos = 4_u32.wrapping_add(get_index_length(&cff.name));
    extract_index(header_slice, pos, &mut cff.top_dict);
    if cff.name.count != cff.top_dict.count {
        tracing::warn!("[libcff] Bad CFF font: ({}, name), ({}, top_dict).\n", cff.name.count, cff.top_dict.count);
    }
    pos = 4_u32
        .wrapping_add(get_index_length(&cff.name))
        .wrapping_add(get_index_length(&cff.top_dict));
    extract_index(header_slice, pos, &mut cff.string);
    pos = 4_u32
        .wrapping_add(get_index_length(&cff.name))
        .wrapping_add(get_index_length(&cff.top_dict))
        .wrapping_add(get_index_length(&cff.string));
    extract_index(header_slice, pos, &mut cff.global_subr);
    // The Top DICT INDEX's `data` is the concatenation of every entry's
    // dict bytes; entry 0 (the only one a well-formed OpenType CFF table
    // ever has, per `cff.name.count != cff.top_dict.count`'s warning
    // below) starts at `offset[0] - 1`, which `extract_index`'s validation
    // guarantees is 0 (CFF INDEX offsets are 1-based). Computed once and
    // reused for every key looked up in the Top DICT below -- previously
    // each lookup recomputed the identical `data.as_ptr()` + manual
    // offset-diff pointer pair from scratch. `.get(..len)` (rather than a
    // raw pointer) makes the "entry 0 starts at 0" assumption load-bearing
    // instead of implicit: a `top_dict` INDEX with more than one entry now
    // safely gets just its first entry's bytes instead of silently reading
    // past them.
    let top_dict_bytes: &[u8] = if !cff.top_dict.data.is_empty() {
        let top_dict_offset = &cff.top_dict.offset;
        let top_dict_len = top_dict_offset[1].wrapping_sub(top_dict_offset[0]) as usize;
        let top_dict_data: &[u8] = &cff.top_dict.data;
        top_dict_data.get(..top_dict_len).unwrap_or(&[])
    } else {
        &[]
    };
    if !cff.top_dict.data.is_empty() {
        let mut offset_0: i32;
        offset_0 = parse_dict_key_int(top_dict_bytes, OP_CHAR_STRINGS, 0_u32);
        if offset_0 != -1_i32 {
            extract_index(header_slice, offset_0 as u32, &mut cff.char_strings);
            cff.cnt_glyph = cff.char_strings.count as u16;
        } else {
            empty_index(&mut cff.char_strings);
            tracing::warn!("[libcff] Bad CFF font: no any glyph data.\n");
        }
        offset_0 = parse_dict_key_int(top_dict_bytes, OP_ENCODING, 0_u32);
        if offset_0 != -1_i32 {
            cff.encodings = parse_encoding(cff, offset_0);
        } else {
            cff.encodings = CffEncoding::Unspecified;
        }
        offset_0 = parse_dict_key_int(top_dict_bytes, OP_CHARSET, 0_u32);
        if offset_0 != -1_i32 {
            cff.charsets =
                cff_extract_charset(header_slice, offset_0, cff.char_strings.count as u16);
        } else {
            cff.charsets = CffCharset::IsoAdobe;
        }
        offset_0 = parse_dict_key_int(top_dict_bytes, OP_FD_SELECT, 0_u32);
        if cff.char_strings.count != 0 && offset_0 != -1_i32 {
            cff.fdselect =
                cff_extract_fd_select(header_slice, offset_0, cff.char_strings.count as u16);
        } else {
            cff.fdselect = CffFdSelect::Unspecified;
        }
        offset_0 = parse_dict_key_int(top_dict_bytes, OP_FD_ARRAY, 0_u32);
        if offset_0 != -1_i32 {
            extract_index(header_slice, offset_0 as u32, &mut cff.font_dict);
        } else {
            empty_index(&mut cff.font_dict);
        }
    }
    let mut private_len: i32 = -1_i32;
    let mut private_off: i32 = -1_i32;
    if !cff.top_dict.data.is_empty() {
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
        header_slice
            .get(private_off as usize..)
            .and_then(|s| s.get(..private_len as usize))
    } else {
        None
    };
    if let Some(private_bytes) = private_dict_bytes {
        offset = parse_dict_key_int(private_bytes, OP_SUBRS, 0_u32);
        if offset != -1_i32 {
            extract_index(
                header_slice,
                (private_off + offset) as u32,
                &mut cff.local_subr,
            );
        } else {
            empty_index(&mut cff.local_subr);
        }
    } else {
        empty_index(&mut cff.local_subr);
    };
}
pub fn cff_open_stream(data: &[u8]) -> Box<CffFile> {
    // `CffFile` owns several `Vec`-backed fields (each `CffIndex`'s
    // `offset`/`data`, and `CffEncoding`/`CffCharset`/`CffFdSelect`'s
    // `Vec`-carrying variants) -- calloc'ing it and then letting
    // `parse_cff_bytecode` fill each field in with a plain `(*file).field
    // = value;` assignment is UB the instant the first such assignment
    // runs: the assignment drops the *old* value first, and an all-zero
    // bit pattern is never a valid `Vec`/enum-with-a-`Vec`-variant to
    // begin with ("constructing invalid value... encountered 0" under
    // Miri) -- see [[otfcc-vec-field-assign-needs-calloc]]. The same bug
    // also fires on disposing a malformed font whose empty Top DICT left
    // `char_strings`/`font_dict`/`encodings`/`charsets`/`fdselect` never
    // written by `parse_cff_bytecode` at all: dropping them unconditionally
    // (as this struct's own `Drop` glue already does) is the identical
    // first-write-onto-zeroed-memory pattern. Building the whole value via
    // `Box::new` up front (instead of calloc) closes both: every field
    // starts out as a real, valid (empty) value, so every later plain `=`
    // -- in `parse_cff_bytecode` or on drop -- safely drops a real prior
    // value instead of an invalid zeroed one.
    //
    // Stage M-14: returns the `Box<CffFile>` itself now, instead of
    // `Box::into_raw`-ing it purely to hand back a pointer. This function
    // already built the value as an owned local (the `Box::new(...)`
    // above) -- boxing it into a raw pointer only to have its one caller
    // immediately `Box::from_raw` it back at the end of that caller's
    // scope was pure ABI-shaped roundtrip, the same pattern this
    // migration already removed from `cff_dict_create`/`new_index_by_
    // callback` and friends at Stage M-10. Nothing aliases `file` between
    // construction and return, so ownership transfers cleanly through the
    // `Box` itself.
    let mut file: Box<CffFile> = Box::new(CffFile {
        raw_data: Vec::new(),
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
    });
    // Stage M-19: `data`/`len` used to be a raw mutable byte pointer plus
    // a `u32` length, requiring an unsafe call to `core::slice::from_raw_parts(data,
    // len as usize)` here and an `unsafe fn` signature purely to carry
    // that contract -- but this function's one production caller
    // (`table/cff.rs`'s `read_cff_and_glyf_tables`) already had a
    // real `&[u8]` in hand (`PacketPiece.data`) and was only decomposing
    // it into a pointer+length to satisfy this signature, the same
    // "raw pointer purely dodging the borrow checker" shape M-3/M-9/
    // M-14/M-16/M-17 removed elsewhere. Taking `&[u8]` directly removes
    // the decompose-then-reconstruct round trip and the unsafe wrapping
    // it required at both call sites (this file's one test included) --
    // `to_vec()` needs no unsafe at all once `data` is already a real
    // slice.
    file.raw_data = data.to_vec();
    file.cnt_glyph = 0_u16;
    parse_cff_bytecode(&mut file);
    return file;
}
// No longer `extern "C"`: `&CffFdSelect` has no C spelling. Only called
// from within `table/cff.rs`, not part of the crate's public ABI -- same
// reasoning as `parse_encoding`. Takes `&CffFdSelect` rather than by value
// since its two callers both read `(*f).fdselect` from a shared `CffFile`
// across repeated per-glyph calls -- moving it out on the first call would
// leave it invalid for the rest.
pub fn cff_parse_subr(
    idx: u16,
    raw: &[u8],
    fdarray: &CffIndex,
    select: &CffFdSelect,
    subr: &mut CffIndex,
) -> u8 {
    let mut fd: u8 = 0_u8;
    let off_subr: i32;
    match select {
        CffFdSelect::Format0(fds) => {
            fd = fds[idx as usize];
        }
        CffFdSelect::Format3 { range3, sentinel } => {
            for pair in range3.windows(2) {
                if idx as i32 >= pair[0].first as i32 && (idx as i32) < pair[1].first as i32 {
                    fd = pair[0].fd;
                }
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
    let off_private: i32 = parse_dict_key_int(fd_dict_bytes, OP_PRIVATE, 1_u32);
    let len_private: i32 = parse_dict_key_int(fd_dict_bytes, OP_PRIVATE, 0_u32);
    // Same bounds hole as `parse_cff_bytecode`'s Local Subrs lookup above:
    // `off_private`/`len_private` are Private-DICT-controlled operands,
    // unvalidated against `raw_length` until now.
    let private_dict_bytes: Option<&[u8]> = if off_private >= 0 && len_private >= 0 {
        raw.get(off_private as usize..)
            .and_then(|s| s.get(..len_private as usize))
    } else {
        None
    };
    if let Some(private_bytes) = private_dict_bytes {
        off_subr = parse_dict_key_int(private_bytes, OP_SUBRS, 0_u32);
        if off_subr != -1_i32 {
            extract_index(raw, (off_private + off_subr) as u32, subr);
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
// `extract_index` (`libcff/index.rs`) only validates an INDEX's
// *last* offset entry against the wraparound-to-4GB bug; intermediate
// entries can still be zero or non-monotonic, so this also re-validates
// the specific pair this call needs (both in bounds, and consistent with
// each other and with the INDEX's own data length), not just that
// `offset.get(idx)` succeeds.
// Returns the subroutine's own bytes directly instead of a `(*const u8,
// u32)` pair -- the `.add(data_offset)` pointer arithmetic that used to
// follow the bounds check above was itself provably in-bounds (the check
// just above it guarantees `data_offset + data_len <= subr_index.data.
// len()`), so it was pure c2rust residue on top of an already-safe
// design: `.get(data_offset..)?.get(..data_len)` expresses the exact
// same guarantee as a bounds-checked slice instead of a raw offset.
// `subr` is the signed operand of `callsubr`/`callgsubr`: subroutine
// numbers run from `-bias` upward, so a font with more than 107
// subroutines calls the lower ones with negative numbers.
pub(crate) fn locate_subr(subr_index: &CffIndex, bias: u16, subr: i32) -> Option<&[u8]> {
    let idx = usize::try_from(i64::from(bias) + i64::from(subr)).ok()?;
    let start = *subr_index.offset.get(idx)?;
    let end = *subr_index.offset.get(idx.checked_add(1)?)?;
    if start < 1 || end < start {
        return None;
    }
    let data_offset = (start - 1) as usize;
    let data_len = (end - start) as usize;
    subr_index.data.get(data_offset..)?.get(..data_len)
}
pub(crate) fn compute_subr_bias(cnt: u16) -> u16 {
    if (cnt as i32) < 1240_i32 {
        return 107_u16;
    } else if (cnt as i32) < 33900_i32 {
        return 1131_u16;
    } else {
        return 32768_u16;
    };
}
pub use crate::libcff::charstring_interp::cff_parse_outline;

#[cfg(test)]
mod cff_header_and_encoding_tests {
    use super::*;
    use crate::libcff::index::CffIndexCountType;

    fn empty_cff_index() -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: 0,
            off_size: 0,
            offset: Vec::new(),
            data: Vec::new(),
        }
    }

    fn cff_file_over(data: &[u8]) -> CffFile {
        CffFile {
            raw_data: data.to_vec(),
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
        let mut cff = cff_file_over(&data);
        // Never dereferenced on this path: `name.count == top_dict.count`
        // (both 0 for a header this short), so the only place this
        // function reads `options` -- the mismatch-count warning log --
        // is never reached. A default `Options` stands in for "never
        // used" now that the parameter is a real reference and can't be
        // null the way the old raw pointer could.
        parse_cff_bytecode(&mut cff);
        assert_eq!(cff.head.major, 1);
        assert_eq!(cff.head.minor, 0);
        assert_eq!(cff.head.hdr_size, 0);
        assert_eq!(cff.head.off_size, 0);
    }

    #[test]
    fn parse_encoding_format0_reads_the_code_array() {
        // offset=0/1 are reserved predefined-encoding sentinels, so the
        // real data starts at offset 2.
        let data = [0u8, 0, 0x00, 0x02, 5, 9]; // format=0, codes=[5,9]
        let cff = cff_file_over(&data);
        let CffEncoding::Format0(code) = parse_encoding(&cff, 2) else {
            panic!("expected Format0");
        };
        assert_eq!(code, vec![5, 9]);
    }

    #[test]
    fn parse_encoding_format0_truncated_falls_back_to_unspecified_instead_of_reading_oob() {
        let data = [0u8, 0, 0x00, 0x02, 5]; // format=0, ncodes=2, only 1 code present
        let cff = cff_file_over(&data);
        let result = parse_encoding(&cff, 2);
        assert!(matches!(result, CffEncoding::Unspecified));
    }

    #[test]
    fn parse_encoding_negative_offset_falls_back_to_unspecified_instead_of_reading_before_the_buffer()
     {
        let data = [0u8; 8];
        let cff = cff_file_over(&data);
        let result = parse_encoding(&cff, -5);
        assert!(matches!(result, CffEncoding::Unspecified));
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
        let mut cff = cff_file_over(&data);
        parse_cff_bytecode(&mut cff);
        assert_eq!(cff.top_dict.count, 1, "sanity: Top DICT INDEX parsed");
        assert_eq!(cff.local_subr.count, 0);
        assert!(cff.local_subr.data.is_empty());
    }
}

#[cfg(test)]
mod cff_open_stream_tests {
    use super::*;


    // A minimal CFF blob whose Top DICT INDEX is empty (`count == 0`):
    // header + 4 empty INDEXes (Name/Top DICT/String/Global Subr). With
    // an empty Top DICT, `parse_cff_bytecode` never writes
    // `char_strings`/`font_dict`/`encodings`/`charsets`/`fdselect` at
    // all -- this exercises `Box::from_raw`'s (formerly `cff_close`'s)
    // *unconditional* disposal of those fields on whatever
    // `cff_open_stream` initialized them to.
    //
    // Before this fix, `cff_open_stream` calloc'd the whole `CffFile`
    // and left every field an invalid all-zero bit pattern until first
    // written. Disposing of the never-written fields (originally inside
    // `cff_close`, now `CffFile`'s own field-by-field `Drop` glue) was
    // itself the first "write" to them (a plain `=` inside
    // `cff_index_dispose`) -- UB under Miri ("constructing invalid
    // value... encountered 0") the instant that assignment drops the
    // old, invalid value, regardless of whether this test's assertions
    // below ever observe anything wrong at runtime. Building the whole
    // `CffFile` via `Box::new` up front (this fix) makes every field a
    // real, valid (empty) value from construction, so disposing of it is
    // always dropping a real prior value.
    //
    // Stage M-14: `cff_open_stream` now returns `Box<CffFile>` directly,
    // so this test drops the plain `Box` at scope end instead of a
    // separate `Box::from_raw` call.
    //
    // Stage M-19: `cff_open_stream` takes `&[u8]` directly now, so this
    // call needs no unsafe wrapper any more either.
    #[test]
    fn open_and_close_on_a_font_with_an_empty_top_dict_does_not_construct_invalid_values() {
        let data: [u8; 16] = [
            1, 0, 4, 4, // header: major, minor, hdrSize, offSize
            0, 0, 0, // Name INDEX: empty
            0, 0, 0, // Top DICT INDEX: empty
            0, 0, 0, // String INDEX: empty
            0, 0, 0, // Global Subr INDEX: empty
        ];
        let file = cff_open_stream(&data);
        assert_eq!(file.top_dict.count, 0);
        assert_eq!(file.char_strings.count, 0);
        assert!(file.char_strings.data.is_empty());
        assert_eq!(file.font_dict.count, 0);
        assert!(matches!(file.encodings, CffEncoding::Unspecified));
        assert!(matches!(file.charsets, CffCharset::IsoAdobe));
        assert!(matches!(file.fdselect, CffFdSelect::Unspecified));
        assert_eq!(file.local_subr.count, 0);
        drop(file);
    }
}

#[cfg(test)]
mod locate_subr_tests {
    use super::*;
    use crate::support::primitives::Arity;
    use crate::libcff::index::CffIndexCountType;

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
        assert_eq!(locate_subr(&idx, 0, 0).unwrap(), &[0xAA, 0xBB]);
        assert_eq!(locate_subr(&idx, 0, 1).unwrap(), &[0xCC, 0xDD]);
    }

    #[test]
    fn subroutine_index_past_the_end_is_rejected_instead_of_reading_oob() {
        // The original indexed the offset array with a raw, unchecked
        // `.offset()` -- a `callsubr`/`callgsubr` operand large enough to
        // run past it read (and then recursed into) arbitrary memory.
        let idx = subr_index(vec![1, 3, 5], vec![0xAA, 0xBB, 0xCC, 0xDD]);
        assert!(locate_subr(&idx, 0, 5).is_none());
    }

    #[test]
    fn negative_subroutine_numbers_count_up_from_minus_the_bias() {
        let idx = subr_index(vec![1, 3, 5], vec![0xAA, 0xBB, 0xCC, 0xDD]);
        assert_eq!(locate_subr(&idx, 2, -2).unwrap(), &[0xAA, 0xBB]);
        assert_eq!(locate_subr(&idx, 2, -1).unwrap(), &[0xCC, 0xDD]);
    }

    #[test]
    fn subroutine_number_below_minus_the_bias_is_rejected() {
        let idx = subr_index(vec![1, 3], vec![0xAA, 0xBB]);
        assert!(locate_subr(&idx, 107, -108).is_none());
        assert!(locate_subr(&idx, u16::MAX, i32::MIN).is_none());
        assert!(locate_subr(&idx, u16::MAX, i32::MAX).is_none());
    }

    #[test]
    fn a_zero_intermediate_offset_is_rejected() {
        // `extract_index` only validates the INDEX's *last* offset entry
        // against the wraparound bug -- an intermediate entry of 0 (not
        // a valid 1-based offset) was never checked here at all.
        let idx = subr_index(vec![1, 0, 5], vec![0xAA, 0xBB, 0xCC, 0xDD]);
        assert!(locate_subr(&idx, 0, 0).is_none());
    }

    #[test]
    fn a_non_monotonic_offset_pair_is_rejected() {
        let idx = subr_index(vec![5, 1], vec![0xAA, 0xBB, 0xCC, 0xDD]);
        assert!(locate_subr(&idx, 0, 0).is_none());
    }

    #[test]
    fn a_range_past_the_actual_data_length_is_rejected_instead_of_reading_oob() {
        // The offsets are internally consistent (monotonic, both >= 1)
        // but claim more data than `subr_index.data` actually holds.
        let idx = subr_index(vec![1, 100], vec![0xAA, 0xBB]);
        assert!(locate_subr(&idx, 0, 0).is_none());
    }
}

#[cfg(test)]
mod cff_parse_subr_tests {
    use super::*;
    use crate::libcff::fdselect::CffFdSelect;
    use crate::libcff::index::CffIndexCountType;

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
        // `raw` is never touched on this path -- `fd` (99) is already past
        // `fdarray.count` (1), so `cff_parse_subr` takes its early-return
        // branch before reading any font bytes.
        let fd = cff_parse_subr(0, &[], &fdarray, &select, &mut subr);
        assert_eq!(fd, 99);
        assert_eq!(subr.count, 0);
    }
}
