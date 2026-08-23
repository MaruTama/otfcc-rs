#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, memcpy, snprintf, strlen, strtol};

use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_int_val, json_str_len, json_str_ptr, json_type_of,
};

use crate::support::options::Options;
use crate::vendor::json::JsonType;

use crate::support::base64::{base64_decode, base64_encode};
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_string_new,
    json_string_new_length, preserialize,
};
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
#[repr(C)]
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
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ch1: ::core::ffi::c_int = 0;
    let mut ch2: ::core::ffi::c_int = 0;
    loop {
        let fresh19 = n;
        n = n - 1;
        if !(fresh19 > 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh20 = str1;
        str1 = str1.offset(1);
        ch1 = *fresh20 as ::core::ffi::c_int;
        let fresh21 = str2;
        str2 = str2.offset(1);
        ch2 = *fresh21 as ::core::ffi::c_int;
        ch1 = c_tolower(ch1);
        ch2 = c_tolower(ch2);
        if ch1 != ch2 || ch1 == '\0' as i32 {
            return ch1 - ch2;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe fn parse_instrs(
    mut text: *mut ::core::ffi::c_char,
    mut context: *mut ::core::ffi::c_void,
    mut iv_error: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> (),
    >,
) -> Option<Vec<u8>> {
    let mut numberstack: [::core::ffi::c_short; 256] = [0; 256];
    let mut npos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nread: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut push_left: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut push_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pt: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut brack: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut imax: ::core::ffi::c_int = strlen(text) as ::core::ffi::c_int;
    let mut val: ::core::ffi::c_int = 0;
    let mut instrs: Vec<u8> = Vec::with_capacity(imax as usize);
    pt = text;
    while *pt != 0 {
        npos = 0 as ::core::ffi::c_int;
        while npos < 256 as ::core::ffi::c_int {
            while *pt as ::core::ffi::c_int == ' ' as i32
                || *pt as ::core::ffi::c_int == '\t' as i32
            {
                pt = pt.offset(1);
            }
            if !(c_isdigit(*pt as ::core::ffi::c_int) || *pt as ::core::ffi::c_int == '-' as i32) {
                break;
            }
            val = strtol(pt, &raw mut end, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if val > 32767 as ::core::ffi::c_int || val < -(32768 as ::core::ffi::c_int) {
                iv_error.expect("non-null function pointer")(
                    context,
                    b"A value must be between [-32768,32767]\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
                return None;
            }
            pt = end;
            let fresh0 = npos;
            npos = npos + 1;
            numberstack[fresh0 as usize] = val as ::core::ffi::c_short;
        }
        while *pt as ::core::ffi::c_int == ' ' as i32 || *pt as ::core::ffi::c_int == '\t' as i32 {
            pt = pt.offset(1);
        }
        if !(npos == 0 as ::core::ffi::c_int
            && (*pt as ::core::ffi::c_int == '\r' as i32
                || *pt as ::core::ffi::c_int == '\n' as i32
                || *pt as ::core::ffi::c_int == '\0' as i32))
        {
            nread = 0 as ::core::ffi::c_int;
            if push_left == -(1 as ::core::ffi::c_int) {
                if npos == 0 as ::core::ffi::c_int {
                    iv_error.expect("non-null function pointer")(
                        context,
                        b"Expected a number for a push count\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                    );
                } else if numberstack[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    > 255 as ::core::ffi::c_int
                    || numberstack[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        <= 0 as ::core::ffi::c_int
                {
                    iv_error.expect("non-null function pointer")(
                        context,
                        b"The push count must be a number between 0 and 255\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                    );
                    return None;
                } else {
                    nread = 1 as ::core::ffi::c_int;
                    instrs.push(numberstack[0 as ::core::ffi::c_int as usize] as u8);
                    push_left = numberstack[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                }
            }
            if push_left != 0 as ::core::ffi::c_int
                && push_left < npos - nread
                && (*pt as ::core::ffi::c_int == '\r' as i32
                    || *pt as ::core::ffi::c_int == '\n' as i32
                    || *pt as ::core::ffi::c_int == '\0' as i32)
            {
                iv_error.expect("non-null function pointer")(
                    context,
                    b"More pushes specified than needed\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
                return None;
            }
            while push_left > 0 as ::core::ffi::c_int && nread < npos {
                if push_size == 2 as ::core::ffi::c_int {
                    instrs.push(
                        (numberstack[nread as usize] as ::core::ffi::c_int
                            >> 8 as ::core::ffi::c_int) as u8,
                    );
                    let fresh3 = nread;
                    nread = nread + 1;
                    instrs.push(
                        (numberstack[fresh3 as usize] as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int) as u8,
                    );
                } else if numberstack[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    > 255 as ::core::ffi::c_int
                    || (numberstack[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                {
                    iv_error.expect("non-null function pointer")(
                        context,
                        b"A value to be pushed by a byte push must be between 0 and 255\0"
                            as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                    );
                    return None;
                } else {
                    let fresh5 = nread;
                    nread = nread + 1;
                    instrs.push(numberstack[fresh5 as usize] as u8);
                }
                push_left -= 1;
            }
            if nread < npos
                && push_left == 0 as ::core::ffi::c_int
                && (*pt as ::core::ffi::c_int == '\r' as i32
                    || *pt as ::core::ffi::c_int == '\n' as i32
                    || *pt as ::core::ffi::c_int == '\0' as i32)
            {
                iv_error.expect("non-null function pointer")(
                    context,
                    b"Unexpected number\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
                return None;
            }
            if !(*pt as ::core::ffi::c_int == '\r' as i32
                || *pt as ::core::ffi::c_int == '\n' as i32
                || *pt as ::core::ffi::c_int == '\0' as i32)
            {
                if push_left > 0 as ::core::ffi::c_int {
                    iv_error.expect("non-null function pointer")(
                        context,
                        b"Missing pushes\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                        pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                    );
                    return None;
                }
                while nread < npos {
                    i = nread;
                    if numberstack[nread as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                        && numberstack[nread as usize] as ::core::ffi::c_int
                            <= 255 as ::core::ffi::c_int
                    {
                        while i < npos
                            && numberstack[i as usize] as ::core::ffi::c_int
                                >= 0 as ::core::ffi::c_int
                            && numberstack[i as usize] as ::core::ffi::c_int
                                <= 255 as ::core::ffi::c_int
                        {
                            i += 1;
                        }
                        if i - nread <= 8 as ::core::ffi::c_int {
                            instrs.push(
                                (TTF_PUSHB as ::core::ffi::c_int + (i - nread)
                                    - 1 as ::core::ffi::c_int)
                                    as u8,
                            );
                        } else {
                            instrs.push(TTF_NPUSHB);
                            instrs.push((i - nread) as u8);
                        }
                        while nread < i {
                            let fresh10 = nread;
                            nread = nread + 1;
                            instrs.push(numberstack[fresh10 as usize] as u8);
                        }
                    } else {
                        while i < npos
                            && ((numberstack[i as usize] as ::core::ffi::c_int)
                                < 0 as ::core::ffi::c_int
                                || numberstack[i as usize] as ::core::ffi::c_int
                                    > 255 as ::core::ffi::c_int)
                        {
                            i += 1;
                        }
                        if i - nread <= 8 as ::core::ffi::c_int {
                            instrs.push(
                                (TTF_PUSHW as ::core::ffi::c_int + (i - nread)
                                    - 1 as ::core::ffi::c_int)
                                    as u8,
                            );
                        } else {
                            instrs.push(TTF_NPUSHW);
                            instrs.push((i - nread) as u8);
                        }
                        while nread < i {
                            instrs.push(
                                (numberstack[nread as usize] as ::core::ffi::c_int
                                    >> 8 as ::core::ffi::c_int)
                                    as u8,
                            );
                            let fresh16 = nread;
                            nread = nread + 1;
                            instrs.push(
                                (numberstack[fresh16 as usize] as ::core::ffi::c_int
                                    & 0xff as ::core::ffi::c_int)
                                    as u8,
                            );
                        }
                    }
                }
                brack = ::core::ptr::null_mut::<::core::ffi::c_char>();
                end = pt;
                while *end as ::core::ffi::c_int != '\r' as i32
                    && *end as ::core::ffi::c_int != '\n' as i32
                    && *end as ::core::ffi::c_int != ' ' as i32
                    && *end as ::core::ffi::c_int != '\0' as i32
                {
                    if *end as ::core::ffi::c_int == '[' as i32
                        || *end as ::core::ffi::c_int == '_' as i32
                    {
                        brack = end;
                    }
                    end = end.offset(1);
                }
                i = 0 as ::core::ffi::c_int;
                while i < 256 as ::core::ffi::c_int {
                    if strnmatch(
                        pt,
                        FF_TTF_INSTRNAMES[i as usize].as_ptr(),
                        end.offset_from(pt) as ::core::ffi::c_long as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int
                        && ::core::mem::size_of::<::core::ffi::c_char>()
                            .wrapping_mul(end.offset_from(pt) as ::core::ffi::c_long as usize)
                            == FF_TTF_INSTRNAMES[i as usize].count_bytes()
                    {
                        break;
                    }
                    i += 1;
                }
                if i == 256 as ::core::ffi::c_int && !brack.is_null() {
                    i = 0 as ::core::ffi::c_int;
                    while i < 256 as ::core::ffi::c_int {
                        if strnmatch(
                            pt,
                            FF_TTF_INSTRNAMES[i as usize].as_ptr(),
                            (brack.offset_from(pt) as ::core::ffi::c_long
                                + 1 as ::core::ffi::c_long)
                                as ::core::ffi::c_int,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        i += 1;
                    }
                    val = strtol(
                        brack.offset(1 as ::core::ffi::c_int as isize),
                        &raw mut bend,
                        2 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    while *bend as ::core::ffi::c_int == ' ' as i32
                        || *bend as ::core::ffi::c_int == '\t' as i32
                    {
                        bend = bend.offset(1);
                    }
                    if *bend as ::core::ffi::c_int != ']' as i32 {
                        iv_error.expect("non-null function pointer")(
                            context,
                            b"Missing right bracket in command (or bad binary value in bracket)\0"
                                as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                        );
                        return None;
                    }
                    if val >= 32 as ::core::ffi::c_int {
                        iv_error.expect("non-null function pointer")(
                            context,
                            b"Bracketted value is too large\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                            pt.offset_from(text) as ::core::ffi::c_long as ::core::ffi::c_int,
                        );
                        return None;
                    }
                    i += val;
                }
                pt = end;
                instrs.push(i as u8);
                if i == TTF_NPUSHB as ::core::ffi::c_int
                    || i == TTF_NPUSHW as ::core::ffi::c_int
                    || i >= TTF_PUSHB as ::core::ffi::c_int
                        && i <= TTF_PUSHW as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                {
                    push_size = if i == TTF_NPUSHB as ::core::ffi::c_int
                        || i >= TTF_PUSHB as ::core::ffi::c_int
                            && i <= TTF_PUSHB as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    };
                    if i == TTF_NPUSHB as ::core::ffi::c_int
                        || i == TTF_NPUSHW as ::core::ffi::c_int
                    {
                        push_left = -(1 as ::core::ffi::c_int);
                    } else if i >= TTF_PUSHB as ::core::ffi::c_int
                        && i <= TTF_PUSHB as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                    {
                        push_left = i - TTF_PUSHB as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                    } else {
                        push_left = i - TTF_PUSHW as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                    }
                }
                if *pt as ::core::ffi::c_int == '\0' as i32 {
                    break;
                }
            }
        }
        pt = pt.offset(1);
    }
    Some(instrs)
}
unsafe fn instr_typify(mut id: *mut InstrData) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = (*id).instr_cnt as ::core::ffi::c_int;
    let mut cnt: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut lh: ::core::ffi::c_int = 0;
    let mut instrs: *mut u8 = (*id).instrs;
    if (*id).bts.is_empty() {
        (*id).bts = vec![ByteType::Instr; (len + 1 as ::core::ffi::c_int) as usize];
    }
    let bts: *mut ByteType = (*id).bts.as_mut_ptr();
    lh = 0 as ::core::ffi::c_int;
    i = lh;
    while i < len {
        *bts.offset(i as isize) = ByteType::Instr;
        lh += 1;
        if *instrs.offset(i as isize) == TTF_NPUSHB {
            i += 1;
            *bts.offset(i as isize) = ByteType::Cnt;
            cnt = *instrs.offset(i as isize) as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < cnt {
                i += 1;
                *bts.offset(i as isize) = ByteType::Byte;
                j += 1;
            }
            lh += 1 as ::core::ffi::c_int + cnt;
        } else if *instrs.offset(i as isize) == TTF_NPUSHW {
            i += 1;
            *bts.offset(i as isize) = ByteType::Cnt;
            lh += 1;
            cnt = *instrs.offset(i as isize) as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < cnt {
                i += 1;
                *bts.offset(i as isize) = ByteType::WordHi;
                i += 1;
                *bts.offset(i as isize) = ByteType::WordLo;
                j += 1;
            }
            lh += 1 as ::core::ffi::c_int + cnt;
        } else if *instrs.offset(i as isize) as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
            == 0xb0 as ::core::ffi::c_int
        {
            cnt = (*instrs.offset(i as isize) as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < cnt {
                i += 1;
                *bts.offset(i as isize) = ByteType::Byte;
                j += 1;
            }
            lh += cnt;
        } else if *instrs.offset(i as isize) as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
            == 0xb8 as ::core::ffi::c_int
        {
            cnt = (*instrs.offset(i as isize) as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int;
            j = 0 as ::core::ffi::c_int;
            while j < cnt {
                i += 1;
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
pub unsafe fn dump_ttinstr(
    mut instructions: *mut u8,
    mut length: u32,
    mut options: &Options,
) -> *mut BuiltValue {
    if options.instr_as_bytes {
        let mut len: usize = 0 as usize;
        let buf: *mut u8 = base64_encode(instructions, length as usize, &raw mut len);
        // `json_string_new_length` copies `buf`'s bytes into a fresh `Vec`
        // rather than taking ownership of it (see its own definition) --
        // `buf` itself was never freed on this path, unlike `table/name.rs`'s
        // three sibling `base64_encode`/`base64_decode` call sites, which do.
        let result =
            json_string_new_length(len as ::core::ffi::c_uint, buf as *mut ::core::ffi::c_char);
        free(buf as *mut ::core::ffi::c_void);
        return result;
    } else {
        let mut id: InstrData = InstrData {
            instrs: ::core::ptr::null_mut::<u8>(),
            instr_cnt: 0,
            bts: Vec::new(),
        };
        id.instr_cnt = length;
        id.instrs = instructions;
        instr_typify(&raw mut id);
        let mut ret: *mut BuiltValue = json_array_new(id.instr_cnt as usize);
        let mut i: u32 = 0 as u32;
        while i < id.instr_cnt {
            if id.bts[i as usize] == ByteType::WordHi {
                json_array_push(
                    ret,
                    json_integer_new(
                        ((*id.instrs.offset(i as isize) as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *id.instrs.offset(i.wrapping_add(1 as u32) as isize)
                                as ::core::ffi::c_int) as i16 as i64,
                    ),
                );
                i = i.wrapping_add(1);
            } else if id.bts[i as usize] == ByteType::Cnt || id.bts[i as usize] == ByteType::Byte {
                json_array_push(ret, json_integer_new(*id.instrs.offset(i as isize) as i64));
            } else {
                json_array_push(
                    ret,
                    json_string_new(
                        FF_TTF_INSTRNAMES[*id.instrs.offset(i as isize) as usize].as_ptr(),
                    ),
                );
            }
            i = i.wrapping_add(1);
        }
        return preserialize(ret);
    };
}
pub unsafe fn parse_ttinstr(
    mut col: *const ParsedValue,
    mut context: *mut ::core::ffi::c_void,
    mut make: Option<unsafe fn(*mut ::core::ffi::c_void, Vec<u8>) -> ()>,
    mut wrong: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> (),
    >,
) {
    if col.is_null() {
        make.expect("non-null function pointer")(context, Vec::new());
    } else if json_type_of(col) == JsonType::String {
        let mut instrlen: usize = 0;
        let instructions: *mut u8 = base64_decode(
            json_str_ptr(col) as *mut u8,
            json_str_len(col) as usize,
            &raw mut instrlen,
        );
        // `base64_decode` hands back a `malloc`ed buffer (or null on a
        // decode failure/empty input) -- copy it into an owned `Vec` and
        // free the original, rather than growing `base64_decode` itself
        // into a `Vec`-returning function (out of scope for this
        // conversion; it has its own unrelated call sites).
        let instructions_vec: Vec<u8> = if instructions.is_null() || instrlen == 0 {
            Vec::new()
        } else {
            ::core::slice::from_raw_parts(instructions, instrlen).to_vec()
        };
        if !instructions.is_null() {
            free(instructions as *mut ::core::ffi::c_void);
        }
        make.expect("non-null function pointer")(context, instructions_vec);
    } else if json_type_of(col) == JsonType::Array {
        let mut istrlen: usize = 0 as usize;
        let mut j: u32 = 0 as u32;
        while j < json_arr_len(col) {
            let mut record: *const ParsedValue = json_arr_at(col, j as u32);
            if json_type_of(record) == JsonType::String {
                istrlen = istrlen.wrapping_add(
                    json_str_len(record).wrapping_add(1 as ::core::ffi::c_uint) as usize,
                );
            } else if json_type_of(record) == JsonType::Integer {
                istrlen = istrlen
                    .wrapping_add((1 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as usize);
            } else {
                make.expect("non-null function pointer")(context, Vec::new());
                return;
            }
            j = j.wrapping_add(1);
        }
        // Zero-filled, `istrlen + 1` bytes: the fill loop below writes
        // exactly `istrlen` bytes, leaving the last one at its zero-
        // initialized value as `parse_instrs`'s NUL terminator (it reads
        // this buffer with `strlen`) -- same size and same guarantee
        // `sdsnewlen(NULL, istrlen + 1)` gave, without needing `sds` at
        // all.
        let mut instr_string: Vec<u8> = vec![0u8; istrlen.wrapping_add(1 as usize)];
        let mut head: *mut ::core::ffi::c_char =
            instr_string.as_mut_ptr() as *mut ::core::ffi::c_char;
        let mut j_0: u32 = 0 as u32;
        while j_0 < json_arr_len(col) {
            let mut record_0: *const ParsedValue = json_arr_at(col, j_0 as u32);
            if json_type_of(record_0) == JsonType::String {
                memcpy(
                    head as *mut ::core::ffi::c_void,
                    json_str_ptr(record_0) as *const ::core::ffi::c_void,
                    (::core::mem::size_of::<::core::ffi::c_char>() as usize)
                        .wrapping_mul(json_str_len(record_0) as usize),
                );
                head = head.offset(json_str_len(record_0) as isize);
            } else if json_type_of(record_0) == JsonType::Integer {
                let mut n: ::core::ffi::c_int = snprintf(
                    head,
                    20 as usize,
                    b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                    json_int_val(record_0) as ::core::ffi::c_int,
                );
                head = head.offset(n as isize);
            }
            *head = '\n' as i32 as ::core::ffi::c_char;
            head = head.offset(1);
            j_0 = j_0.wrapping_add(1);
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
    } else {
        make.expect("non-null function pointer")(context, Vec::new());
    };
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
}
