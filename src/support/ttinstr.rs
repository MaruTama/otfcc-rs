#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see RUST_MIGRATION.md
use libc::{memcpy, snprintf, strlen, strtol};

use crate::support::parsed_json::ParsedValue;

use crate::support::options::Options;

use crate::support::base64::{base64_decode, base64_encode};
use crate::support::built_json::BuiltValue;
use crate::support::ctype_compat::{c_isdigit, c_tolower};
/// The four opcodes `parse_instrs`/`instr_typify` have to recognise, because
/// their operands are part of the instruction stream rather than separate
/// instructions. `u8`, since that is what `InstrData.instrs` holds.
///
/// c2rust emitted all 123 of `ttf_instructions`' names, of which these were the
/// only ones any code referenced. The rest restated `FF_TTF_INSTRNAMES` below,
/// which the dumper and parser actually use and which covers all 256 opcodes --
/// checked name by name against it before removing them (121 matched exactly;
/// `TTF_PUSHB`/`TTF_PUSHW` name the base of the eight `PUSHB_1`..`PUSHB_8`
/// variants the table spells out).
pub const TTF_NPUSHB: u8 = 64;
pub const TTF_NPUSHW: u8 = 65;
pub const TTF_PUSHB: u8 = 176;
pub const TTF_PUSHW: u8 = 184;
// `instrs` stays a borrowed raw pointer -- every `InstrData.instrs` value is
// an alias into a caller-owned buffer (`Glyph.instructions`/`FpgmPrepTable.
// bytes`), never allocated here, and those two fields are themselves a
// deliberate Stage 6-4 "outer struct Box'd, inner array stays a manually
// freed raw pointer" case, per RUST_MIGRATION.md -- left untouched this round.
// `bts`, in contrast, is allocated, filled, and freed entirely within this
// file (`instr_typify` builds it, `dump_ttinstr` reads it and drops it), so
// it converts cleanly to `Vec` with no boundary to preserve.
pub struct InstrData {
    pub instrs: *mut u8,
    pub instr_cnt: u32,
    /// What each byte of `instrs` *is*, one entry per byte, filled in by
    /// [`instr_typify`]. Not part of the instruction stream: the two arrays run
    /// in parallel, which is why this one is typed and `instrs` stays `u8`.
    pub bts: Vec<ByteType>,
}

/// The role of one byte in a TrueType instruction stream: the opcode itself, or
/// one of the operand bytes that follow a push.
///
/// `#[repr(u8)]` deliberately -- the array is `calloc`ed one byte per
/// instruction byte, and `ByteType::Instr` being 0 is what makes that zeroing valid.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ByteType {
    Instr = 0,
    Cnt = 1,
    Byte = 2,
    WordHi = 3,
    WordLo = 4,
    ImpliedReturn = 5,
}
pub static FF_TTF_INSTRNAMES: [&[u8]; 256] = [
    b"SVTCA[y-axis]",
    b"SVTCA[x-axis]",
    b"SPVTCA[y-axis]",
    b"SPVTCA[x-axis]",
    b"SFVTCA[y-axis]",
    b"SFVTCA[x-axis]",
    b"SPVTL[parallel]",
    b"SPVTL[orthog]",
    b"SFVTL[parallel]",
    b"SFVTL[orthog]",
    b"SPVFS",
    b"SFVFS",
    b"GPV",
    b"GFV",
    b"SFVTPV",
    b"ISECT",
    b"SRP0",
    b"SRP1",
    b"SRP2",
    b"SZP0",
    b"SZP1",
    b"SZP2",
    b"SZPS",
    b"SLOOP",
    b"RTG",
    b"RTHG",
    b"SMD",
    b"ELSE",
    b"JMPR",
    b"SCVTCI",
    b"SSWCI",
    b"SSW",
    b"DUP",
    b"POP",
    b"CLEAR",
    b"SWAP",
    b"DEPTH",
    b"CINDEX",
    b"MINDEX",
    b"ALIGNPTS",
    b"Unknown28",
    b"UTP",
    b"LOOPCALL",
    b"CALL",
    b"FDEF",
    b"ENDF",
    b"MDAP[no-rnd]",
    b"MDAP[rnd]",
    b"IUP[y]",
    b"IUP[x]",
    b"SHP[rp2]",
    b"SHP[rp1]",
    b"SHC[rp2]",
    b"SHC[rp1]",
    b"SHZ[rp2]",
    b"SHZ[rp1]",
    b"SHPIX",
    b"IP",
    b"MSIRP[no-rp0]",
    b"MSIRP[rp0]",
    b"ALIGNRP",
    b"RTDG",
    b"MIAP[no-rnd]",
    b"MIAP[rnd]",
    b"NPUSHB",
    b"NPUSHW",
    b"WS",
    b"RS",
    b"WCVTP",
    b"RCVT",
    b"GC[cur]",
    b"GC[orig]",
    b"SCFS",
    b"MD[grid]",
    b"MD[orig]",
    b"MPPEM",
    b"MPS",
    b"FLIPON",
    b"FLIPOFF",
    b"DEBUG",
    b"LT",
    b"LTEQ",
    b"GT",
    b"GTEQ",
    b"EQ",
    b"NEQ",
    b"ODD",
    b"EVEN",
    b"IF",
    b"EIF",
    b"AND",
    b"OR",
    b"NOT",
    b"DELTAP1",
    b"SDB",
    b"SDS",
    b"ADD",
    b"SUB",
    b"DIV",
    b"MUL",
    b"ABS",
    b"NEG",
    b"FLOOR",
    b"CEILING",
    b"ROUND[Grey]",
    b"ROUND[Black]",
    b"ROUND[White]",
    b"ROUND[Undef4]",
    b"NROUND[Grey]",
    b"NROUND[Black]",
    b"NROUND[White]",
    b"NROUND[Undef4]",
    b"WCVTF",
    b"DELTAP2",
    b"DELTAP3",
    b"DELTAC1",
    b"DELTAC2",
    b"DELTAC3",
    b"SROUND",
    b"S45ROUND",
    b"JROT",
    b"JROF",
    b"ROFF",
    b"Unknown7B",
    b"RUTG",
    b"RDTG",
    b"SANGW",
    b"AA",
    b"FLIPPT",
    b"FLIPRGON",
    b"FLIPRGOFF",
    b"Unknown83",
    b"Unknown84",
    b"SCANCTRL",
    b"SDPVTL[parallel]",
    b"SDPVTL[orthog]",
    b"GETINFO",
    b"IDEF",
    b"ROLL",
    b"MAX",
    b"MIN",
    b"SCANTYPE",
    b"INSTCTRL",
    b"Unknown8F",
    b"Unknown90",
    b"GETVARIATION",
    b"Unknown92",
    b"Unknown93",
    b"Unknown94",
    b"Unknown95",
    b"Unknown96",
    b"Unknown97",
    b"Unknown98",
    b"Unknown99",
    b"Unknown9A",
    b"Unknown9B",
    b"Unknown9C",
    b"Unknown9D",
    b"Unknown9E",
    b"Unknown9F",
    b"UnknownA0",
    b"UnknownA1",
    b"UnknownA2",
    b"UnknownA3",
    b"UnknownA4",
    b"UnknownA5",
    b"UnknownA6",
    b"UnknownA7",
    b"UnknownA8",
    b"UnknownA9",
    b"UnknownAA",
    b"UnknownAB",
    b"UnknownAC",
    b"UnknownAD",
    b"UnknownAE",
    b"UnknownAF",
    b"PUSHB_1",
    b"PUSHB_2",
    b"PUSHB_3",
    b"PUSHB_4",
    b"PUSHB_5",
    b"PUSHB_6",
    b"PUSHB_7",
    b"PUSHB_8",
    b"PUSHW_1",
    b"PUSHW_2",
    b"PUSHW_3",
    b"PUSHW_4",
    b"PUSHW_5",
    b"PUSHW_6",
    b"PUSHW_7",
    b"PUSHW_8",
    b"MDRP[grey]",
    b"MDRP[black]",
    b"MDRP[white]",
    b"MDRP03",
    b"MDRP[rnd,grey]",
    b"MDRP[rnd,black]",
    b"MDRP[rnd,white]",
    b"MDRP07",
    b"MDRP[min,grey]",
    b"MDRP[min,black]",
    b"MDRP[min,white]",
    b"MDRP0b",
    b"MDRP[min,rnd,grey]",
    b"MDRP[min,rnd,black]",
    b"MDRP[min,rnd,white]",
    b"MDRP0f",
    b"MDRP[rp0,grey]",
    b"MDRP[rp0,black]",
    b"MDRP[rp0,white]",
    b"MDRP13",
    b"MDRP[rp0,rnd,grey]",
    b"MDRP[rp0,rnd,black]",
    b"MDRP[rp0,rnd,white]",
    b"MDRP17",
    b"MDRP[rp0,min,grey]",
    b"MDRP[rp0,min,black]",
    b"MDRP[rp0,min,white]",
    b"MDRP1b",
    b"MDRP[rp0,min,rnd,grey]",
    b"MDRP[rp0,min,rnd,black]",
    b"MDRP[rp0,min,rnd,white]",
    b"MDRP1f",
    b"MIRP[grey]",
    b"MIRP[black]",
    b"MIRP[white]",
    b"MIRP03",
    b"MIRP[rnd,grey]",
    b"MIRP[rnd,black]",
    b"MIRP[rnd,white]",
    b"MIRP07",
    b"MIRP[min,grey]",
    b"MIRP[min,black]",
    b"MIRP[min,white]",
    b"MIRP0b",
    b"MIRP[min,rnd,grey]",
    b"MIRP[min,rnd,black]",
    b"MIRP[min,rnd,white]",
    b"MIRP0f",
    b"MIRP[rp0,grey]",
    b"MIRP[rp0,black]",
    b"MIRP[rp0,white]",
    b"MIRP13",
    b"MIRP[rp0,rnd,grey]",
    b"MIRP[rp0,rnd,black]",
    b"MIRP[rp0,rnd,white]",
    b"MIRP17",
    b"MIRP[rp0,min,grey]",
    b"MIRP[rp0,min,black]",
    b"MIRP[rp0,min,white]",
    b"MIRP1b",
    b"MIRP[rp0,min,rnd,grey]",
    b"MIRP[rp0,min,rnd,black]",
    b"MIRP[rp0,min,rnd,white]",
    b"MIRP1f",
];
unsafe fn strnmatch(
    mut str1: *const ::core::ffi::c_char,
    mut str2: *const ::core::ffi::c_char,
    mut n: i32,
) -> i32 {
    let mut ch1: i32;
    let mut ch2: i32;
    loop {
        if !(n > 0_i32) {
            break;
        }
        n = n - 1;
        ch1 = *str1 as i32;
        str1 = str1.offset(1);
        ch2 = *str2 as i32;
        str2 = str2.offset(1);
        ch1 = c_tolower(ch1);
        ch2 = c_tolower(ch2);
        if ch1 != ch2 || ch1 == '\0' as i32 {
            return ch1 - ch2;
        }
    }
    return 0_i32;
}
// Was a `*mut c_void` context pointer + `Option<unsafe fn(*mut c_void,
// ...)>` callback, type-erasing this function's two callers' distinct
// concrete error handlers (`table/fpgm_prep.rs`'s `wrong_fpgm_prep_instr`,
// `table/glyf.rs`'s `wrong_instrs_for_glyph`) behind a shared shape purely
// so one function pointer type could stand in for both -- the same "type
// erasure that was never actually needed" pattern already resolved for
// `libcff/cff_index.rs`'s `new_index_by_callback`, `libcff/cff_dict.rs`'s
// `parse_to_callback`, and `table/otl.rs`'s `otl_*_filter_env` family.
// `context` is never dereferenced here, only threaded through to
// `iv_error` -- a generic `impl FnMut` closure carries the same
// information with no context pointer to thread at all, since each
// concrete error handler can just capture what it needs directly.
unsafe fn parse_instrs(
    text: *mut ::core::ffi::c_char,
    mut iv_error: impl FnMut(*mut ::core::ffi::c_char, i32),
) -> Option<Vec<u8>> {
    let mut numberstack: [::core::ffi::c_short; 256] = [0; 256];
    let mut npos: i32;
    let mut nread: i32;
    let mut i: i32;
    let mut push_left: i32 = 0_i32;
    let mut push_size: i32 = 0_i32;
    let mut pt: *mut ::core::ffi::c_char;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut brack: *mut ::core::ffi::c_char;
    let imax: i32 = strlen(text) as i32;
    let mut val: i32;
    let mut instrs: Vec<u8> = Vec::with_capacity(imax as usize);
    pt = text;
    while *pt != 0 {
        npos = 0_i32;
        while npos < 256_i32 {
            while *pt as i32 == ' ' as i32
                || *pt as i32 == '\t' as i32
            {
                pt = pt.offset(1);
            }
            if !(c_isdigit(*pt as i32) || *pt as i32 == '-' as i32) {
                break;
            }
            val = strtol(pt, &raw mut end, 0_i32) as i32;
            if !(-32768_i32..=32767_i32).contains(&val) {
                iv_error(
                    b"A value must be between [-32768,32767]\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    pt.offset_from(text) as ::core::ffi::c_long as i32,
                );
                return None;
            }
            pt = end;
            numberstack[npos as usize] = val as ::core::ffi::c_short;
            npos = npos + 1;
        }
        while *pt as i32 == ' ' as i32 || *pt as i32 == '\t' as i32 {
            pt = pt.offset(1);
        }
        if !(npos == 0_i32
            && (*pt as i32 == '\r' as i32
                || *pt as i32 == '\n' as i32
                || *pt as i32 == '\0' as i32))
        {
            nread = 0_i32;
            if push_left == -1_i32 {
                if npos == 0_i32 {
                    iv_error(
                        b"Expected a number for a push count\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as i32,
                    );
                } else if numberstack[0_i32 as usize] as i32
                    > 255_i32
                    || numberstack[0_i32 as usize] as i32
                        <= 0_i32
                {
                    iv_error(
                        b"The push count must be a number between 0 and 255\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as i32,
                    );
                    return None;
                } else {
                    nread = 1_i32;
                    instrs.push(numberstack[0_i32 as usize] as u8);
                    push_left = numberstack[0_i32 as usize] as i32;
                }
            }
            if push_left != 0_i32
                && push_left < npos - nread
                && (*pt as i32 == '\r' as i32
                    || *pt as i32 == '\n' as i32
                    || *pt as i32 == '\0' as i32)
            {
                iv_error(
                    b"More pushes specified than needed\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    pt.offset_from(text) as ::core::ffi::c_long as i32,
                );
                return None;
            }
            while push_left > 0_i32 && nread < npos {
                if push_size == 2_i32 {
                    instrs.push(
                        (numberstack[nread as usize] as i32
                            >> 8_i32) as u8,
                    );
                    instrs.push(
                        (numberstack[nread as usize] as i32
                            & 0xff_i32) as u8,
                    );
                    nread = nread + 1;
                } else if numberstack[0_i32 as usize] as i32
                    > 255_i32
                    || (numberstack[0_i32 as usize] as i32)
                        < 0_i32
                {
                    iv_error(
                        b"A value to be pushed by a byte push must be between 0 and 255\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as i32,
                    );
                    return None;
                } else {
                    instrs.push(numberstack[nread as usize] as u8);
                    nread = nread + 1;
                }
                push_left -= 1;
            }
            if nread < npos
                && push_left == 0_i32
                && (*pt as i32 == '\r' as i32
                    || *pt as i32 == '\n' as i32
                    || *pt as i32 == '\0' as i32)
            {
                iv_error(
                    b"Unexpected number\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    pt.offset_from(text) as ::core::ffi::c_long as i32,
                );
                return None;
            }
            if !(*pt as i32 == '\r' as i32
                || *pt as i32 == '\n' as i32
                || *pt as i32 == '\0' as i32)
            {
                if push_left > 0_i32 {
                    iv_error(
                        b"Missing pushes\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as i32,
                    );
                    return None;
                }
                while nread < npos {
                    i = nread;
                    if numberstack[nread as usize] as i32 >= 0_i32
                        && numberstack[nread as usize] as i32
                            <= 255_i32
                    {
                        while i < npos
                            && numberstack[i as usize] as i32
                                >= 0_i32
                            && numberstack[i as usize] as i32
                                <= 255_i32
                        {
                            i += 1;
                        }
                        if i - nread <= 8_i32 {
                            instrs.push(
                                (TTF_PUSHB as i32 + (i - nread)
                                    - 1_i32)
                                    as u8,
                            );
                        } else {
                            instrs.push(TTF_NPUSHB);
                            instrs.push((i - nread) as u8);
                        }
                        while nread < i {
                            instrs.push(numberstack[nread as usize] as u8);
                            nread = nread + 1;
                        }
                    } else {
                        while i < npos
                            && ((numberstack[i as usize] as i32)
                                < 0_i32
                                || numberstack[i as usize] as i32
                                    > 255_i32)
                        {
                            i += 1;
                        }
                        if i - nread <= 8_i32 {
                            instrs.push(
                                (TTF_PUSHW as i32 + (i - nread)
                                    - 1_i32)
                                    as u8,
                            );
                        } else {
                            instrs.push(TTF_NPUSHW);
                            instrs.push((i - nread) as u8);
                        }
                        while nread < i {
                            instrs.push(
                                (numberstack[nread as usize] as i32
                                    >> 8_i32)
                                    as u8,
                            );
                            instrs.push(
                                (numberstack[nread as usize] as i32
                                    & 0xff_i32)
                                    as u8,
                            );
                            nread = nread + 1;
                        }
                    }
                }
                brack = ::core::ptr::null_mut::<::core::ffi::c_char>();
                end = pt;
                while *end as i32 != '\r' as i32
                    && *end as i32 != '\n' as i32
                    && *end as i32 != ' ' as i32
                    && *end as i32 != '\0' as i32
                {
                    if *end as i32 == '[' as i32
                        || *end as i32 == '_' as i32
                    {
                        brack = end;
                    }
                    end = end.offset(1);
                }
                i = 0_i32;
                while i < 256_i32 {
                    let name_len = ::core::mem::size_of::<::core::ffi::c_char>()
                        .wrapping_mul(end.offset_from(pt) as ::core::ffi::c_long as usize);
                    // Check the length match before calling strnmatch: strnmatch
                    // never looks at str2's own end, so a full-length compare here
                    // (n == FF_TTF_INSTRNAMES[i].len()) is what keeps it from
                    // reading past that entry's actual bytes.
                    if name_len == FF_TTF_INSTRNAMES[i as usize].len()
                        && strnmatch(
                            pt,
                            FF_TTF_INSTRNAMES[i as usize].as_ptr() as *const ::core::ffi::c_char,
                            end.offset_from(pt) as ::core::ffi::c_long as i32,
                        ) == 0_i32
                    {
                        break;
                    }
                    i += 1;
                }
                if i == 256_i32 && !brack.is_null() {
                    i = 0_i32;
                    while i < 256_i32 {
                        let n = (brack.offset_from(pt) as ::core::ffi::c_long
                            + 1 as ::core::ffi::c_long) as usize;
                        // Same guard as above: n comes only from the user input
                        // token's bracket position, so it can exceed a short
                        // table entry's length -- skip those rather than let
                        // strnmatch read past the entry.
                        if n <= FF_TTF_INSTRNAMES[i as usize].len()
                            && strnmatch(
                                pt,
                                FF_TTF_INSTRNAMES[i as usize].as_ptr()
                                    as *const ::core::ffi::c_char,
                                n as i32,
                            ) == 0_i32
                        {
                            break;
                        }
                        i += 1;
                    }
                    val = strtol(
                        brack.offset(1_i32 as isize),
                        &raw mut bend,
                        2_i32,
                    ) as i32;
                    while *bend as i32 == ' ' as i32
                        || *bend as i32 == '\t' as i32
                    {
                        bend = bend.offset(1);
                    }
                    if *bend as i32 != ']' as i32 {
                        iv_error(
                            b"Missing right bracket in command (or bad binary value in bracket)\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            pt.offset_from(text) as ::core::ffi::c_long as i32,
                        );
                        return None;
                    }
                    if val >= 32_i32 {
                        iv_error(
                            b"Bracketted value is too large\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            pt.offset_from(text) as ::core::ffi::c_long as i32,
                        );
                        return None;
                    }
                    i += val;
                }
                pt = end;
                instrs.push(i as u8);
                if i == TTF_NPUSHB as i32
                    || i == TTF_NPUSHW as i32
                    || i >= TTF_PUSHB as i32
                        && i <= TTF_PUSHW as i32 + 7_i32
                {
                    push_size = if i == TTF_NPUSHB as i32
                        || i >= TTF_PUSHB as i32
                            && i <= TTF_PUSHB as i32 + 7_i32
                    {
                        1_i32
                    } else {
                        2_i32
                    };
                    if i == TTF_NPUSHB as i32
                        || i == TTF_NPUSHW as i32
                    {
                        push_left = -1_i32;
                    } else if i >= TTF_PUSHB as i32
                        && i <= TTF_PUSHB as i32 + 7_i32
                    {
                        push_left = i - TTF_PUSHB as i32 + 1_i32;
                    } else {
                        push_left = i - TTF_PUSHW as i32 + 1_i32;
                    }
                }
                if *pt as i32 == '\0' as i32 {
                    break;
                }
            }
        }
        pt = pt.offset(1);
    }
    Some(instrs)
}
unsafe fn instr_typify(id: *mut InstrData) -> i32 {
    let mut i: i32;
    let len: i32 = (*id).instr_cnt as i32;
    let mut cnt: i32;
    let mut j: i32;
    let mut lh: i32;
    let instrs: *mut u8 = (*id).instrs;
    if (*id).bts.is_empty() {
        (*id).bts = vec![ByteType::Instr; (len + 1_i32) as usize];
    }
    let bts: *mut ByteType = (*id).bts.as_mut_ptr();
    lh = 0_i32;
    i = lh;
    // `NPUSHB`/`NPUSHW`/`PUSHB[n]`/`PUSHW[n]` each carry their own
    // attacker-controlled operand count, skipped by advancing `i` several
    // steps within a single outer-loop iteration -- nothing previously
    // stopped that skip from running `i` past `len` (this instruction
    // block's own declared length). `bts` is sized to exactly `len + 1`
    // slots (0..=len, the last for the trailing `ImpliedReturn` marker
    // below), so any write at an `i` past that -- reachable via a
    // `NPUSHB`/`NPUSHW`/`PUSHB[n]`/`PUSHW[n]` whose declared count runs
    // off the end -- is a heap-buffer-overflow (ASan-confirmed: a
    // fuzz-found font's hinting bytecode did exactly this). `'outer`
    // breaks the moment `i` would go out of range, leaving the rest of a
    // truncated/malformed instruction stream untypified instead of
    // reading or writing past `bts` or `instrs` (whose own declared
    // length is the same `len`) -- breaking exactly when `i` reaches
    // `len` lands on the same position the loop's own normal exit
    // condition would anyway, so the trailing `ImpliedReturn` write below
    // needs no separate guard.
    'outer: while i < len {
        *bts.offset(i as isize) = ByteType::Instr;
        lh += 1;
        if *instrs.offset(i as isize) == TTF_NPUSHB {
            i += 1;
            if i >= len {
                break 'outer;
            }
            *bts.offset(i as isize) = ByteType::Cnt;
            cnt = *instrs.offset(i as isize) as i32;
            j = 0_i32;
            while j < cnt {
                i += 1;
                if i >= len {
                    break 'outer;
                }
                *bts.offset(i as isize) = ByteType::Byte;
                j += 1;
            }
            lh += 1_i32 + cnt;
        } else if *instrs.offset(i as isize) == TTF_NPUSHW {
            i += 1;
            if i >= len {
                break 'outer;
            }
            *bts.offset(i as isize) = ByteType::Cnt;
            lh += 1;
            cnt = *instrs.offset(i as isize) as i32;
            j = 0_i32;
            while j < cnt {
                i += 1;
                if i >= len {
                    break 'outer;
                }
                // A `WordHi` marker promises `dump_ttinstr` a paired
                // `WordLo` right after it (it reads `instrs[i+1]`
                // unconditionally whenever it sees `WordHi`) -- if this
                // operand's declared count runs off the end with only one
                // byte of a two-byte word actually present, mark that
                // trailing byte `Byte` instead so nothing later reads past
                // `instrs`'s own `len` bytes (ASan-confirmed: a fuzz-found
                // font's truncated `NPUSHW`/`PUSHW[n]` operand did exactly
                // this, a 1-byte heap-buffer-overflow read in
                // `dump_ttinstr`).
                if i.wrapping_add(1) >= len {
                    *bts.offset(i as isize) = ByteType::Byte;
                    // Advance `i` to `len` before breaking -- every other
                    // break path in this loop leaves `i == len`, which the
                    // unconditional `ImpliedReturn` write right after this
                    // loop relies on (it writes at the *current* `i`, on
                    // the assumption that's the slot past everything this
                    // loop already wrote). Breaking with `i` still at this
                    // byte's own index would let that write silently
                    // clobber the `Byte` marker just set above.
                    i += 1;
                    break 'outer;
                }
                *bts.offset(i as isize) = ByteType::WordHi;
                i += 1;
                *bts.offset(i as isize) = ByteType::WordLo;
                j += 1;
            }
            lh += 1_i32 + cnt;
        } else if *instrs.offset(i as isize) as i32 & 0xf8_i32
            == 0xb0_i32
        {
            cnt = (*instrs.offset(i as isize) as i32 & 7_i32)
                + 1_i32;
            j = 0_i32;
            while j < cnt {
                i += 1;
                if i >= len {
                    break 'outer;
                }
                *bts.offset(i as isize) = ByteType::Byte;
                j += 1;
            }
            lh += cnt;
        } else if *instrs.offset(i as isize) as i32 & 0xf8_i32
            == 0xb8_i32
        {
            cnt = (*instrs.offset(i as isize) as i32 & 7_i32)
                + 1_i32;
            j = 0_i32;
            while j < cnt {
                i += 1;
                if i >= len {
                    break 'outer;
                }
                // Same "no orphaned WordHi" fix as the NPUSHW branch above.
                if i.wrapping_add(1) >= len {
                    *bts.offset(i as isize) = ByteType::Byte;
                    // Advance `i` to `len` before breaking -- every other
                    // break path in this loop leaves `i == len`, which the
                    // unconditional `ImpliedReturn` write right after this
                    // loop relies on (it writes at the *current* `i`, on
                    // the assumption that's the slot past everything this
                    // loop already wrote). Breaking with `i` still at this
                    // byte's own index would let that write silently
                    // clobber the `Byte` marker just set above.
                    i += 1;
                    break 'outer;
                }
                *bts.offset(i as isize) = ByteType::WordHi;
                i += 1;
                *bts.offset(i as isize) = ByteType::WordLo;
                j += 1;
            }
            lh += cnt;
        }
        i += 1;
    }
    *bts.offset(i as isize) = ByteType::ImpliedReturn;
    return lh;
}
pub unsafe fn dump_ttinstr(instructions: *mut u8, length: u32, options: &Options) -> BuiltValue {
    if options.instr_as_bytes {
        let encoded = base64_encode(::core::slice::from_raw_parts(
            instructions,
            length as usize,
        ));
        BuiltValue::Str(encoded)
    } else {
        let mut id: InstrData = InstrData {
            instrs: ::core::ptr::null_mut::<u8>(),
            instr_cnt: 0,
            bts: Vec::new(),
        };
        id.instr_cnt = length;
        id.instrs = instructions;
        instr_typify(&raw mut id);
        let mut ret = BuiltValue::new_array(id.instr_cnt as usize);
        let mut i: u32 = 0_u32;
        while i < id.instr_cnt {
            if id.bts[i as usize] == ByteType::WordHi {
                ret.push_item(BuiltValue::Int(
                    ((*id.instrs.offset(i as isize) as i32) << 8_i32
                        | *id.instrs.offset(i.wrapping_add(1_u32) as isize) as i32)
                        as i16 as i64,
                ));
                i = i.wrapping_add(1);
            } else if id.bts[i as usize] == ByteType::Cnt || id.bts[i as usize] == ByteType::Byte {
                ret.push_item(BuiltValue::Int(*id.instrs.offset(i as isize) as i64));
            } else {
                ret.push_item(BuiltValue::Str(
                    FF_TTF_INSTRNAMES[*id.instrs.offset(i as isize) as usize].to_vec(),
                ));
            }
            i = i.wrapping_add(1);
        }
        ret.preserialize()
    }
}
// Same closure-based de-type-erasure as `parse_instrs` above: `context`
// is never dereferenced here either, only threaded through to `make`/
// `wrong` (this function's own two callers, `table/fpgm_prep.rs`'s
// `otfcc_parse_fpgm_prep` and `table/glyf.rs`'s `otfcc_glyf_parse_glyph`,
// each with a different concrete target for `make` to write into).
pub unsafe fn parse_ttinstr(
    col: *const ParsedValue,
    mut make: impl FnMut(Vec<u8>),
    mut wrong: impl FnMut(*mut ::core::ffi::c_char, i32),
) {
    let Some(col_ref) = col.as_ref() else {
        make(Vec::new());
        return;
    };
    if let Some(bytes) = col_ref.as_str_bytes() {
        let instructions_vec = base64_decode(bytes).unwrap_or_default();
        make(instructions_vec);
        return;
    }
    let Some(items) = col_ref.as_array() else {
        make(Vec::new());
        return;
    };
    let mut istrlen: usize = 0_usize;
    for record in items {
        if let Some(bytes) = record.as_str_bytes() {
            istrlen = istrlen.wrapping_add(bytes.len().wrapping_add(1_usize));
        } else if record.as_int().is_some() {
            istrlen = istrlen.wrapping_add(1_usize + 20_usize);
        } else {
            make(Vec::new());
            return;
        }
    }
    // Zero-filled, `istrlen + 1` bytes: the fill loop below writes
    // exactly `istrlen` bytes, leaving the last one at its zero-
    // initialized value as `parse_instrs`'s NUL terminator (it reads
    // this buffer with `strlen`) -- same size and same guarantee
    // `sdsnewlen(NULL, istrlen + 1)` gave, without needing `sds` at
    // all.
    let mut instr_string: Vec<u8> = vec![0u8; istrlen.wrapping_add(1_usize)];
    let mut head: *mut ::core::ffi::c_char = instr_string.as_mut_ptr() as *mut ::core::ffi::c_char;
    for record in items {
        if let Some(bytes) = record.as_str_bytes() {
            memcpy(
                head as *mut ::core::ffi::c_void,
                bytes.as_ptr() as *const ::core::ffi::c_void,
                bytes.len(),
            );
            head = head.offset(bytes.len() as isize);
        } else if let Some(n) = record.as_int() {
            let written: i32 = snprintf(
                head,
                20_usize,
                b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                n as i32,
            );
            head = head.offset(written as isize);
        }
        *head = '\n' as i32 as ::core::ffi::c_char;
        head = head.offset(1);
    }
    let instructions_0: Option<Vec<u8>> = parse_instrs(
        instr_string.as_mut_ptr() as *mut ::core::ffi::c_char,
        &mut wrong,
    );
    match instructions_0 {
        Some(v) if !v.is_empty() => {
            make(v);
        }
        _ => {
            make(Vec::new());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `instr_typify` allocates the `bts` array with `__caryll_allocate_clean`,
    // one byte per instruction byte, and then fills it in -- so `ByteType::Instr` has
    // to be the zero variant (a calloc'ed enum with no zero variant is instantly
    // invalid) and the type has to stay one byte wide or the allocation is short.
    #[test]
    fn byte_types_is_a_calloc_safe_byte() {
        assert_eq!(::core::mem::size_of::<ByteType>(), 1);
        assert_eq!(ByteType::Instr as u8, 0);
        assert_eq!(
            [
                ByteType::Cnt as u8,
                ByteType::Byte as u8,
                ByteType::WordHi as u8,
                ByteType::WordLo as u8,
                ByteType::ImpliedReturn as u8
            ],
            [1, 2, 3, 4, 5]
        );
    }

    // The four opcodes whose operands sit inside the instruction stream. These
    // are the values the TrueType spec assigns, and `FF_TTF_INSTRNAMES` -- which
    // is what the dumper writes and the parser matches -- has to agree with them,
    // since the two are the only remaining record of the opcode numbering.
    #[test]
    fn push_opcodes_agree_with_the_name_table() {
        assert_eq!(
            [TTF_NPUSHB, TTF_NPUSHW, TTF_PUSHB, TTF_PUSHW],
            [64, 65, 176, 184]
        );
        assert_eq!(FF_TTF_INSTRNAMES[TTF_NPUSHB as usize], b"NPUSHB");
        assert_eq!(FF_TTF_INSTRNAMES[TTF_NPUSHW as usize], b"NPUSHW");
        // `PUSHB`/`PUSHW` are eight opcodes each, pushing 1..=8 values; the
        // constant is the first of the run, which is why the code adds an offset
        // to it rather than comparing for equality.
        for n in 0..8u8 {
            assert_eq!(
                FF_TTF_INSTRNAMES[(TTF_PUSHB + n) as usize],
                format!("PUSHB_{}", n + 1).as_bytes()
            );
            assert_eq!(
                FF_TTF_INSTRNAMES[(TTF_PUSHW + n) as usize],
                format!("PUSHW_{}", n + 1).as_bytes()
            );
        }
    }

    // A fuzz-found font found this: `NPUSHB`'s declared count is
    // attacker-controlled and skipped by advancing `i` once per byte --
    // nothing stopped that skip from running past `instr_cnt` (this
    // instruction stream's own declared length). `bts` is sized to
    // exactly `instr_cnt + 1` slots; without the fix, walking past the
    // declared length here writes `bts[3]`, one past its 3-slot
    // allocation (ASan-confirmed heap-buffer-overflow, found via
    // `dump_ttinstr`, this function's only caller).
    //
    // `NPUSHB`, declared count 2 -- but the instruction stream is only 2
    // bytes long (the opcode plus the count byte itself), so the 2
    // "pushed" operand bytes the count promises don't exist.
    #[test]
    fn npushb_count_running_past_the_declared_length_stops_cleanly_instead_of_overflowing_bts() {
        let mut instrs: Vec<u8> = vec![TTF_NPUSHB, 2];
        let mut id = InstrData {
            instrs: instrs.as_mut_ptr(),
            instr_cnt: instrs.len() as u32,
            bts: Vec::new(),
        };
        unsafe {
            instr_typify(&raw mut id);
        }
        // Reaching here at all -- rather than writing past `bts`'s
        // 3-slot allocation -- is the regression signal.
        assert_eq!(id.bts.len(), 3);
    }

    /// A `cargo fuzz run otf_dump` CI job found a second, independent bug
    /// in this same file: a `NPUSHW`/`PUSHW[n]` operand whose declared
    /// count runs off the end with only *one* of its two operand bytes
    /// actually present left that trailing byte marked `WordHi` with no
    /// following `WordLo` -- `dump_ttinstr`'s own read of a `WordHi` byte
    /// unconditionally reads `instrs[i+1]` to get the paired low byte,
    /// which read exactly one byte past `instrs`'s own allocation (ASan:
    /// heap-buffer-overflow, 1-byte READ, "0 bytes after" the buffer).
    /// `PUSHW[0]` (`0xb8`, declares one 2-byte word operand) with only one
    /// trailing byte reproduces the exact shape.
    #[test]
    fn pushw_with_only_one_trailing_byte_marks_it_plain_instead_of_an_orphaned_wordhi() {
        let mut instrs: Vec<u8> = vec![0xb8, 0xab];
        let mut id = InstrData {
            instrs: instrs.as_mut_ptr(),
            instr_cnt: instrs.len() as u32,
            bts: Vec::new(),
        };
        unsafe {
            instr_typify(&raw mut id);
        }
        // Must never be `WordHi` with nothing following it in `instrs`.
        assert_eq!(id.bts[1], ByteType::Byte);
    }

    /// A pre-existing latent bug, flagged (but not fixed, per "one PR one
    /// theme") when `FF_TTF_INSTRNAMES` became `&[u8]`: the exact-match
    /// lookup sized `strnmatch`'s comparison length only from the user
    /// input token's own length, never clamped to (or checked against)
    /// the candidate table entry's actual length. `strnmatch` never
    /// looks at its `str2` argument's own end, so a token that starts
    /// with a short entry's exact bytes but keeps going -- `"GPV"`, 3
    /// bytes, index 12 -- made it walk `FF_TTF_INSTRNAMES[12]`'s pointer
    /// past that entry's 3-byte static allocation. Undetectable at
    /// runtime (it's a read of otherwise-valid rodata) but Miri
    /// (`cargo +nightly miri test --lib`) flags it as out-of-bounds
    /// pointer arithmetic. No bracket needed for this one -- it hits the
    /// first lookup loop, so it stays runnable under Miri (the bracketed
    /// second loop's identical bug is covered by the test below, but
    /// finishing that path needs `libc::strtol`, unsupported there).
    #[test]
    fn exact_match_lookup_does_not_read_past_a_short_table_entry() {
        assert_eq!(FF_TTF_INSTRNAMES[12], b"GPV");
        let mut text: Vec<u8> = b"GPVXXXX\0".to_vec();
        let result =
            unsafe { parse_instrs(text.as_mut_ptr() as *mut ::core::ffi::c_char, |_, _| {}) };
        // Not a real opcode, so it falls through to the "unknown" byte
        // (256 truncated to 0) rather than erroring -- the regression
        // signal is reaching this point at all (under Miri, with no
        // out-of-bounds diagnostic along the way), not this value.
        assert_eq!(result, Some(vec![0u8]));
    }

    /// Same bug, same table entry, but through the *bracketed*-operand
    /// lookup (e.g. matching `"MIRP[rp0,white]"`) a few lines below the
    /// test above -- see that test's doc comment for the mechanism. This
    /// one needs a `[` in the token to reach that second loop, which
    /// means the code always goes on to parse the bracket's contents
    /// with `libc::strtol` afterwards (regardless of whether the loop
    /// found a match) -- ignored under Miri for the same reason
    /// `cff_codecs.rs`'s `atof`/`strtod` test and `parsed_json.rs`'s
    /// `strcmp` test are: unsupported libc FFI call, not a bug. Run
    /// manually with `cargo +nightly-2026-08-17 miri test --lib
    /// bracket_lookup -- --test-threads=1` on a checkout *without* the
    /// fix to confirm it still catches the out-of-bounds read.
    #[test]
    #[cfg_attr(
        miri,
        ignore = "reaches libc::strtol to parse the bracket's binary value after the fix stops the OOB read, unsupported under Miri"
    )]
    fn bracket_lookup_does_not_read_past_a_short_table_entry() {
        assert_eq!(FF_TTF_INSTRNAMES[12], b"GPV");
        let mut text: Vec<u8> = b"GPVXXXX[ab]\0".to_vec();
        let mut saw_error = false;
        let result = unsafe {
            parse_instrs(
                text.as_mut_ptr() as *mut ::core::ffi::c_char,
                |_msg, _pos| {
                    saw_error = true;
                },
            )
        };
        // Not a real opcode past the bracket, so `iv_error` fires and
        // parsing fails -- the regression signal is reaching this point
        // at all (under Miri, with no out-of-bounds diagnostic along the
        // way), not this particular error path.
        assert!(result.is_none());
        assert!(saw_error);
    }
}
