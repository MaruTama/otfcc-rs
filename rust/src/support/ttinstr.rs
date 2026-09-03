#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
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
// freed raw pointer" case, per rust/README.md -- left untouched this round.
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
pub static FF_TTF_INSTRNAMES: [&::core::ffi::CStr; 256] = [
    c"SVTCA[y-axis]",
    c"SVTCA[x-axis]",
    c"SPVTCA[y-axis]",
    c"SPVTCA[x-axis]",
    c"SFVTCA[y-axis]",
    c"SFVTCA[x-axis]",
    c"SPVTL[parallel]",
    c"SPVTL[orthog]",
    c"SFVTL[parallel]",
    c"SFVTL[orthog]",
    c"SPVFS",
    c"SFVFS",
    c"GPV",
    c"GFV",
    c"SFVTPV",
    c"ISECT",
    c"SRP0",
    c"SRP1",
    c"SRP2",
    c"SZP0",
    c"SZP1",
    c"SZP2",
    c"SZPS",
    c"SLOOP",
    c"RTG",
    c"RTHG",
    c"SMD",
    c"ELSE",
    c"JMPR",
    c"SCVTCI",
    c"SSWCI",
    c"SSW",
    c"DUP",
    c"POP",
    c"CLEAR",
    c"SWAP",
    c"DEPTH",
    c"CINDEX",
    c"MINDEX",
    c"ALIGNPTS",
    c"Unknown28",
    c"UTP",
    c"LOOPCALL",
    c"CALL",
    c"FDEF",
    c"ENDF",
    c"MDAP[no-rnd]",
    c"MDAP[rnd]",
    c"IUP[y]",
    c"IUP[x]",
    c"SHP[rp2]",
    c"SHP[rp1]",
    c"SHC[rp2]",
    c"SHC[rp1]",
    c"SHZ[rp2]",
    c"SHZ[rp1]",
    c"SHPIX",
    c"IP",
    c"MSIRP[no-rp0]",
    c"MSIRP[rp0]",
    c"ALIGNRP",
    c"RTDG",
    c"MIAP[no-rnd]",
    c"MIAP[rnd]",
    c"NPUSHB",
    c"NPUSHW",
    c"WS",
    c"RS",
    c"WCVTP",
    c"RCVT",
    c"GC[cur]",
    c"GC[orig]",
    c"SCFS",
    c"MD[grid]",
    c"MD[orig]",
    c"MPPEM",
    c"MPS",
    c"FLIPON",
    c"FLIPOFF",
    c"DEBUG",
    c"LT",
    c"LTEQ",
    c"GT",
    c"GTEQ",
    c"EQ",
    c"NEQ",
    c"ODD",
    c"EVEN",
    c"IF",
    c"EIF",
    c"AND",
    c"OR",
    c"NOT",
    c"DELTAP1",
    c"SDB",
    c"SDS",
    c"ADD",
    c"SUB",
    c"DIV",
    c"MUL",
    c"ABS",
    c"NEG",
    c"FLOOR",
    c"CEILING",
    c"ROUND[Grey]",
    c"ROUND[Black]",
    c"ROUND[White]",
    c"ROUND[Undef4]",
    c"NROUND[Grey]",
    c"NROUND[Black]",
    c"NROUND[White]",
    c"NROUND[Undef4]",
    c"WCVTF",
    c"DELTAP2",
    c"DELTAP3",
    c"DELTAC1",
    c"DELTAC2",
    c"DELTAC3",
    c"SROUND",
    c"S45ROUND",
    c"JROT",
    c"JROF",
    c"ROFF",
    c"Unknown7B",
    c"RUTG",
    c"RDTG",
    c"SANGW",
    c"AA",
    c"FLIPPT",
    c"FLIPRGON",
    c"FLIPRGOFF",
    c"Unknown83",
    c"Unknown84",
    c"SCANCTRL",
    c"SDPVTL[parallel]",
    c"SDPVTL[orthog]",
    c"GETINFO",
    c"IDEF",
    c"ROLL",
    c"MAX",
    c"MIN",
    c"SCANTYPE",
    c"INSTCTRL",
    c"Unknown8F",
    c"Unknown90",
    c"GETVARIATION",
    c"Unknown92",
    c"Unknown93",
    c"Unknown94",
    c"Unknown95",
    c"Unknown96",
    c"Unknown97",
    c"Unknown98",
    c"Unknown99",
    c"Unknown9A",
    c"Unknown9B",
    c"Unknown9C",
    c"Unknown9D",
    c"Unknown9E",
    c"Unknown9F",
    c"UnknownA0",
    c"UnknownA1",
    c"UnknownA2",
    c"UnknownA3",
    c"UnknownA4",
    c"UnknownA5",
    c"UnknownA6",
    c"UnknownA7",
    c"UnknownA8",
    c"UnknownA9",
    c"UnknownAA",
    c"UnknownAB",
    c"UnknownAC",
    c"UnknownAD",
    c"UnknownAE",
    c"UnknownAF",
    c"PUSHB_1",
    c"PUSHB_2",
    c"PUSHB_3",
    c"PUSHB_4",
    c"PUSHB_5",
    c"PUSHB_6",
    c"PUSHB_7",
    c"PUSHB_8",
    c"PUSHW_1",
    c"PUSHW_2",
    c"PUSHW_3",
    c"PUSHW_4",
    c"PUSHW_5",
    c"PUSHW_6",
    c"PUSHW_7",
    c"PUSHW_8",
    c"MDRP[grey]",
    c"MDRP[black]",
    c"MDRP[white]",
    c"MDRP03",
    c"MDRP[rnd,grey]",
    c"MDRP[rnd,black]",
    c"MDRP[rnd,white]",
    c"MDRP07",
    c"MDRP[min,grey]",
    c"MDRP[min,black]",
    c"MDRP[min,white]",
    c"MDRP0b",
    c"MDRP[min,rnd,grey]",
    c"MDRP[min,rnd,black]",
    c"MDRP[min,rnd,white]",
    c"MDRP0f",
    c"MDRP[rp0,grey]",
    c"MDRP[rp0,black]",
    c"MDRP[rp0,white]",
    c"MDRP13",
    c"MDRP[rp0,rnd,grey]",
    c"MDRP[rp0,rnd,black]",
    c"MDRP[rp0,rnd,white]",
    c"MDRP17",
    c"MDRP[rp0,min,grey]",
    c"MDRP[rp0,min,black]",
    c"MDRP[rp0,min,white]",
    c"MDRP1b",
    c"MDRP[rp0,min,rnd,grey]",
    c"MDRP[rp0,min,rnd,black]",
    c"MDRP[rp0,min,rnd,white]",
    c"MDRP1f",
    c"MIRP[grey]",
    c"MIRP[black]",
    c"MIRP[white]",
    c"MIRP03",
    c"MIRP[rnd,grey]",
    c"MIRP[rnd,black]",
    c"MIRP[rnd,white]",
    c"MIRP07",
    c"MIRP[min,grey]",
    c"MIRP[min,black]",
    c"MIRP[min,white]",
    c"MIRP0b",
    c"MIRP[min,rnd,grey]",
    c"MIRP[min,rnd,black]",
    c"MIRP[min,rnd,white]",
    c"MIRP0f",
    c"MIRP[rp0,grey]",
    c"MIRP[rp0,black]",
    c"MIRP[rp0,white]",
    c"MIRP13",
    c"MIRP[rp0,rnd,grey]",
    c"MIRP[rp0,rnd,black]",
    c"MIRP[rp0,rnd,white]",
    c"MIRP17",
    c"MIRP[rp0,min,grey]",
    c"MIRP[rp0,min,black]",
    c"MIRP[rp0,min,white]",
    c"MIRP1b",
    c"MIRP[rp0,min,rnd,grey]",
    c"MIRP[rp0,min,rnd,black]",
    c"MIRP[rp0,min,rnd,white]",
    c"MIRP1f",
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
unsafe fn parse_instrs(
    text: *mut ::core::ffi::c_char,
    context: *mut ::core::ffi::c_void,
    // No longer `extern "C"`: like `make` above, this callback varies at
    // each call site (`table/fpgm_prep.rs`'s `wrong_fpgm_prep_instr`,
    // `table/glyf.rs`'s `wrong_instrs_for_glyph`), but neither crosses the
    // crate's real FFI boundary (`ffi/dll.rs`) -- purely internal
    // Rust-to-Rust indirect calls.
    iv_error: Option<
        unsafe fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_char, i32) -> (),
    >,
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
                iv_error.expect("non-null function pointer")(
                    context,
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
                    iv_error.expect("non-null function pointer")(
                        context,
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
                    iv_error.expect("non-null function pointer")(
                        context,
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
                iv_error.expect("non-null function pointer")(
                    context,
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
                    iv_error.expect("non-null function pointer")(
                        context,
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
                iv_error.expect("non-null function pointer")(
                    context,
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
                    iv_error.expect("non-null function pointer")(
                        context,
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
                    if strnmatch(
                        pt,
                        FF_TTF_INSTRNAMES[i as usize].as_ptr(),
                        end.offset_from(pt) as ::core::ffi::c_long as i32,
                    ) == 0_i32
                        && ::core::mem::size_of::<::core::ffi::c_char>()
                            .wrapping_mul(end.offset_from(pt) as ::core::ffi::c_long as usize)
                            == FF_TTF_INSTRNAMES[i as usize].count_bytes()
                    {
                        break;
                    }
                    i += 1;
                }
                if i == 256_i32 && !brack.is_null() {
                    i = 0_i32;
                    while i < 256_i32 {
                        if strnmatch(
                            pt,
                            FF_TTF_INSTRNAMES[i as usize].as_ptr(),
                            (brack.offset_from(pt) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long)
                                as i32,
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
                        iv_error.expect("non-null function pointer")(
                            context,
                            b"Missing right bracket in command (or bad binary value in bracket)\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            pt.offset_from(text) as ::core::ffi::c_long as i32,
                        );
                        return None;
                    }
                    if val >= 32_i32 {
                        iv_error.expect("non-null function pointer")(
                            context,
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
                    FF_TTF_INSTRNAMES[*id.instrs.offset(i as isize) as usize]
                        .to_bytes()
                        .to_vec(),
                ));
            }
            i = i.wrapping_add(1);
        }
        ret.preserialize()
    }
}
pub unsafe fn parse_ttinstr(
    col: *const ParsedValue,
    context: *mut ::core::ffi::c_void,
    make: Option<unsafe fn(*mut ::core::ffi::c_void, Vec<u8>) -> ()>,
    wrong: Option<
        unsafe fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_char, i32) -> (),
    >,
) {
    let Some(col_ref) = col.as_ref() else {
        make.expect("non-null function pointer")(context, Vec::new());
        return;
    };
    if let Some(bytes) = col_ref.as_str_bytes() {
        let instructions_vec = base64_decode(bytes).unwrap_or_default();
        make.expect("non-null function pointer")(context, instructions_vec);
        return;
    }
    let Some(items) = col_ref.as_array() else {
        make.expect("non-null function pointer")(context, Vec::new());
        return;
    };
    let mut istrlen: usize = 0_usize;
    for record in items {
        if let Some(bytes) = record.as_str_bytes() {
            istrlen = istrlen.wrapping_add(bytes.len().wrapping_add(1_usize));
        } else if record.as_int().is_some() {
            istrlen = istrlen.wrapping_add(1_usize + 20_usize);
        } else {
            make.expect("non-null function pointer")(context, Vec::new());
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
        context,
        wrong,
    );
    match instructions_0 {
        Some(v) if !v.is_empty() => {
            make.expect("non-null function pointer")(context, v);
        }
        _ => {
            make.expect("non-null function pointer")(context, Vec::new());
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
        assert_eq!(FF_TTF_INSTRNAMES[TTF_NPUSHB as usize], c"NPUSHB");
        assert_eq!(FF_TTF_INSTRNAMES[TTF_NPUSHW as usize], c"NPUSHW");
        // `PUSHB`/`PUSHW` are eight opcodes each, pushing 1..=8 values; the
        // constant is the first of the run, which is why the code adds an offset
        // to it rather than comparing for equality.
        for n in 0..8u8 {
            assert_eq!(
                FF_TTF_INSTRNAMES[(TTF_PUSHB + n) as usize].to_bytes(),
                format!("PUSHB_{}", n + 1).as_bytes()
            );
            assert_eq!(
                FF_TTF_INSTRNAMES[(TTF_PUSHW + n) as usize].to_bytes(),
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
}
