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
// Stage L-8: `instrs` is a real borrowed slice now, not a raw pointer --
// every `InstrData` value only ever lives for the duration of one
// `dump_ttinstr` call (never stored, never outlives its caller's own
// buffer), so a lifetime parameter costs nothing at either of its two
// call sites (both already hold a `&[u8]` for exactly as long as this
// struct needs to borrow it). `bts`, in contrast, is allocated, filled,
// and freed entirely within this file (`instr_typify` builds it,
// `dump_ttinstr` reads it and drops it), so it converts cleanly to `Vec`
// with no boundary to preserve.
#[derive(Debug)]
pub struct InstrData<'a> {
    pub instrs: &'a [u8],
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
/// Peeks the byte at `pos`, or `0` past the end -- the same sentinel a
/// NUL-terminated C string gives a `strlen`/pointer-walking reader for
/// "nothing more here", reproduced without needing an actual embedded NUL
/// byte or an out-of-bounds read to get it. An embedded NUL byte inside
/// `text` (reachable: `parse_ttinstr` can copy arbitrary JSON string bytes
/// in) reads back as `0` here exactly like the true end of the buffer
/// does -- the same ambiguity a C string can't distinguish either, so this
/// is a faithful port, not a new behavior.
fn peek(text: &[u8], pos: usize) -> u8 {
    text.get(pos).copied().unwrap_or(0)
}
/// Mirrors `strtol(s, &mut end, 0)`'s base-0 auto-detection on the decimal
/// push-value operands `parse_instrs` reads: an optional leading `-`,
/// then a radix from the digit prefix -- `"0x"`/`"0X"` (followed by at
/// least one hex digit) selects hex, a leading `0` (followed by at least
/// one octal digit) selects octal, anything else is decimal. Returns the
/// value and how many bytes of `s` were consumed. Only ever called on a
/// prefix already confirmed to start with a digit or `-`
/// (`peek(..).is_ascii_digit() || peek(..) == b'-'`), so there is always
/// at least the decimal fallback to consume something.
fn strtol_base0(s: &[u8]) -> (i64, usize) {
    let mut i = 0usize;
    let neg = s.first() == Some(&b'-');
    if neg {
        i += 1;
    }
    if s[i..].len() >= 3 && s[i] == b'0' && matches!(s[i + 1], b'x' | b'X') && s[i + 2].is_ascii_hexdigit() {
        let mut j = i + 2;
        let mut val: i64 = 0;
        while let Some(d) = s.get(j).and_then(|&b| (b as char).to_digit(16)) {
            val = val.saturating_mul(16).saturating_add(d as i64);
            j += 1;
        }
        return (if neg { -val } else { val }, j);
    }
    if s.get(i) == Some(&b'0') && s.get(i + 1).is_some_and(u8::is_ascii_digit) && s[i + 1] <= b'7' {
        let mut j = i + 1;
        let mut val: i64 = 0;
        while s.get(j).is_some_and(|&b| (b'0'..=b'7').contains(&b)) {
            val = val.saturating_mul(8).saturating_add((s[j] - b'0') as i64);
            j += 1;
        }
        return (if neg { -val } else { val }, j);
    }
    let mut j = i;
    let mut val: i64 = 0;
    while s.get(j).is_some_and(u8::is_ascii_digit) {
        val = val.saturating_mul(10).saturating_add((s[j] - b'0') as i64);
        j += 1;
    }
    (if neg { -val } else { val }, j)
}
/// Mirrors `strtol(s, &mut end, 2)` for the fixed-base binary value inside
/// a bracketed command like `MDRP[grey]`'s `grey` -- no prefix detection
/// (an explicit non-zero base never skips one), just an optional `-` and
/// then `0`/`1` digits.
fn strtol_base2(s: &[u8]) -> (i64, usize) {
    let mut i = 0usize;
    let neg = s.first() == Some(&b'-');
    if neg {
        i += 1;
    }
    let mut j = i;
    let mut val: i64 = 0;
    while matches!(s.get(j), Some(b'0') | Some(b'1')) {
        val = val * 2 + (s[j] - b'0') as i64;
        j += 1;
    }
    (if neg { -val } else { val }, j)
}
/// Case-insensitive *exact-length* comparison, replacing `strnmatch(pt,
/// name, end-pt) == 0 && (end-pt) == name.len()`'s combined condition.
/// The original's own `strnmatch` call read up to `end-pt` bytes from
/// `name` regardless of `name`'s own length -- reading past a shorter
/// `FF_TTF_INSTRNAMES` entry's static array bounds whenever `end-pt`
/// exceeded it, before the separate length check downstream ever ran.
/// Comparing real slices (with the length check moved *first*) makes
/// that read structurally impossible instead of merely detecting the
/// mismatch afterward.
fn instr_name_matches(token: &[u8], name: &[u8]) -> bool {
    token.len() == name.len() && token.iter().zip(name).all(|(&a, &b)| c_tolower(a as i32) == c_tolower(b as i32))
}
/// Case-insensitive *prefix* comparison -- `token` may be shorter than
/// `name`, used only for the bracketed-family fallback scan
/// (`MDRP[...]`/`MIRP[...]`-style names). The original's own second
/// `strnmatch` call had no length-equality check at all, only the
/// shared-prefix comparison through the bracket character; the
/// `token.len() <= name.len()` guard here is the same "never read past
/// either slice" fix `instr_name_matches` makes above.
fn instr_name_has_prefix(token: &[u8], name: &[u8]) -> bool {
    token.len() <= name.len() && token.iter().zip(name).all(|(&a, &b)| c_tolower(a as i32) == c_tolower(b as i32))
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
fn parse_instrs(text: &[u8], mut iv_error: impl FnMut(&[u8], i32)) -> Option<Vec<u8>> {
    let mut numberstack: [i16; 256] = [0; 256];
    let mut npos: i32;
    let mut nread: i32;
    let mut i: i32;
    let mut push_left: i32 = 0_i32;
    let mut push_size: i32 = 0_i32;
    let mut end: usize;
    let mut bend: usize;
    let mut brack: Option<usize>;
    let mut val: i32;
    let mut instrs: Vec<u8> = Vec::with_capacity(text.len());
    let mut pt: usize = 0;
    while peek(text, pt) != 0 {
        npos = 0_i32;
        while npos < 256_i32 {
            while peek(text, pt) == b' ' || peek(text, pt) == b'\t' {
                pt += 1;
            }
            let c = peek(text, pt);
            if !(c_isdigit(c as i32) || c == b'-') {
                break;
            }
            let (raw_val, consumed) = strtol_base0(&text[pt..]);
            val = raw_val as i32;
            if !(-32768_i32..=32767_i32).contains(&val) {
                iv_error(b"A value must be between [-32768,32767]", pt as i32);
                return None;
            }
            pt += consumed;
            numberstack[npos as usize] = val as i16;
            npos = npos + 1;
        }
        while peek(text, pt) == b' ' || peek(text, pt) == b'\t' {
            pt += 1;
        }
        let c = peek(text, pt);
        if !(npos == 0_i32 && (c == b'\r' || c == b'\n' || c == 0)) {
            nread = 0_i32;
            if push_left == -1_i32 {
                if npos == 0_i32 {
                    iv_error(b"Expected a number for a push count", pt as i32);
                } else if numberstack[0_i32 as usize] as i32 > 255_i32
                    || numberstack[0_i32 as usize] as i32 <= 0_i32
                {
                    iv_error(b"The push count must be a number between 0 and 255", pt as i32);
                    return None;
                } else {
                    nread = 1_i32;
                    instrs.push(numberstack[0_i32 as usize] as u8);
                    push_left = numberstack[0_i32 as usize] as i32;
                }
            }
            let c = peek(text, pt);
            if push_left != 0_i32 && push_left < npos - nread && (c == b'\r' || c == b'\n' || c == 0) {
                iv_error(b"More pushes specified than needed", pt as i32);
                return None;
            }
            while push_left > 0_i32 && nread < npos {
                if push_size == 2_i32 {
                    instrs.push((numberstack[nread as usize] as i32 >> 8_i32) as u8);
                    instrs.push((numberstack[nread as usize] as i32 & 0xff_i32) as u8);
                    nread = nread + 1;
                } else if numberstack[0_i32 as usize] as i32 > 255_i32
                    || (numberstack[0_i32 as usize] as i32) < 0_i32
                {
                    iv_error(
                        b"A value to be pushed by a byte push must be between 0 and 255",
                        pt as i32,
                    );
                    return None;
                } else {
                    instrs.push(numberstack[nread as usize] as u8);
                    nread = nread + 1;
                }
                push_left -= 1;
            }
            let c = peek(text, pt);
            if nread < npos && push_left == 0_i32 && (c == b'\r' || c == b'\n' || c == 0) {
                iv_error(b"Unexpected number", pt as i32);
                return None;
            }
            let c = peek(text, pt);
            if !(c == b'\r' || c == b'\n' || c == 0) {
                if push_left > 0_i32 {
                    iv_error(b"Missing pushes", pt as i32);
                    return None;
                }
                while nread < npos {
                    i = nread;
                    if numberstack[nread as usize] as i32 >= 0_i32 && numberstack[nread as usize] as i32 <= 255_i32 {
                        while i < npos && numberstack[i as usize] as i32 >= 0_i32 && numberstack[i as usize] as i32 <= 255_i32 {
                            i += 1;
                        }
                        if i - nread <= 8_i32 {
                            instrs.push((TTF_PUSHB as i32 + (i - nread) - 1_i32) as u8);
                        } else {
                            instrs.push(TTF_NPUSHB);
                            instrs.push((i - nread) as u8);
                        }
                        while nread < i {
                            instrs.push(numberstack[nread as usize] as u8);
                            nread = nread + 1;
                        }
                    } else {
                        while i < npos && ((numberstack[i as usize] as i32) < 0_i32 || numberstack[i as usize] as i32 > 255_i32) {
                            i += 1;
                        }
                        if i - nread <= 8_i32 {
                            instrs.push((TTF_PUSHW as i32 + (i - nread) - 1_i32) as u8);
                        } else {
                            instrs.push(TTF_NPUSHW);
                            instrs.push((i - nread) as u8);
                        }
                        while nread < i {
                            instrs.push((numberstack[nread as usize] as i32 >> 8_i32) as u8);
                            instrs.push((numberstack[nread as usize] as i32 & 0xff_i32) as u8);
                            nread = nread + 1;
                        }
                    }
                }
                brack = None;
                end = pt;
                while !matches!(peek(text, end), b'\r' | b'\n' | b' ' | 0) {
                    if matches!(peek(text, end), b'[' | b'_') {
                        brack = Some(end);
                    }
                    end += 1;
                }
                i = 0_i32;
                while i < 256_i32 {
                    if instr_name_matches(&text[pt..end], FF_TTF_INSTRNAMES[i as usize]) {
                        break;
                    }
                    i += 1;
                }
                if i == 256_i32 {
                    if let Some(brack) = brack {
                        i = 0_i32;
                        while i < 256_i32 {
                            if instr_name_has_prefix(&text[pt..=brack], FF_TTF_INSTRNAMES[i as usize]) {
                                break;
                            }
                            i += 1;
                        }
                        let (raw_val, consumed) = strtol_base2(&text[brack + 1..]);
                        val = raw_val as i32;
                        bend = brack + 1 + consumed;
                        while peek(text, bend) == b' ' || peek(text, bend) == b'\t' {
                            bend += 1;
                        }
                        if peek(text, bend) != b']' {
                            iv_error(
                                b"Missing right bracket in command (or bad binary value in bracket)",
                                pt as i32,
                            );
                            return None;
                        }
                        if val >= 32_i32 {
                            iv_error(b"Bracketted value is too large", pt as i32);
                            return None;
                        }
                        i += val;
                    }
                }
                pt = end;
                instrs.push(i as u8);
                if i == TTF_NPUSHB as i32 || i == TTF_NPUSHW as i32 || i >= TTF_PUSHB as i32 && i <= TTF_PUSHW as i32 + 7_i32 {
                    push_size = if i == TTF_NPUSHB as i32 || i >= TTF_PUSHB as i32 && i <= TTF_PUSHB as i32 + 7_i32 {
                        1_i32
                    } else {
                        2_i32
                    };
                    if i == TTF_NPUSHB as i32 || i == TTF_NPUSHW as i32 {
                        push_left = -1_i32;
                    } else if i >= TTF_PUSHB as i32 && i <= TTF_PUSHB as i32 + 7_i32 {
                        push_left = i - TTF_PUSHB as i32 + 1_i32;
                    } else {
                        push_left = i - TTF_PUSHW as i32 + 1_i32;
                    }
                }
                if peek(text, pt) == 0 {
                    break;
                }
            }
        }
        pt += 1;
    }
    Some(instrs)
}
fn instr_typify(id: &mut InstrData) -> i32 {
    let mut i: i32;
    let len: i32 = id.instr_cnt as i32;
    let mut cnt: i32;
    let mut j: i32;
    let mut lh: i32;
    if id.bts.is_empty() {
        id.bts = vec![ByteType::Instr; (len + 1_i32) as usize];
    }
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
        id.bts[i as usize] = ByteType::Instr;
        lh += 1;
        if id.instrs[i as usize] == TTF_NPUSHB {
            i += 1;
            if i >= len {
                break 'outer;
            }
            id.bts[i as usize] = ByteType::Cnt;
            cnt = id.instrs[i as usize] as i32;
            j = 0_i32;
            while j < cnt {
                i += 1;
                if i >= len {
                    break 'outer;
                }
                id.bts[i as usize] = ByteType::Byte;
                j += 1;
            }
            lh += 1_i32 + cnt;
        } else if id.instrs[i as usize] == TTF_NPUSHW {
            i += 1;
            if i >= len {
                break 'outer;
            }
            id.bts[i as usize] = ByteType::Cnt;
            lh += 1;
            cnt = id.instrs[i as usize] as i32;
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
                if i + 1 >= len {
                    id.bts[i as usize] = ByteType::Byte;
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
                id.bts[i as usize] = ByteType::WordHi;
                i += 1;
                id.bts[i as usize] = ByteType::WordLo;
                j += 1;
            }
            lh += 1_i32 + cnt;
        } else if id.instrs[i as usize] as i32 & 0xf8_i32 == 0xb0_i32 {
            cnt = (id.instrs[i as usize] as i32 & 7_i32) + 1_i32;
            j = 0_i32;
            while j < cnt {
                i += 1;
                if i >= len {
                    break 'outer;
                }
                id.bts[i as usize] = ByteType::Byte;
                j += 1;
            }
            lh += cnt;
        } else if id.instrs[i as usize] as i32 & 0xf8_i32 == 0xb8_i32 {
            cnt = (id.instrs[i as usize] as i32 & 7_i32) + 1_i32;
            j = 0_i32;
            while j < cnt {
                i += 1;
                if i >= len {
                    break 'outer;
                }
                // Same "no orphaned WordHi" fix as the NPUSHW branch above.
                if i + 1 >= len {
                    id.bts[i as usize] = ByteType::Byte;
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
                id.bts[i as usize] = ByteType::WordHi;
                i += 1;
                id.bts[i as usize] = ByteType::WordLo;
                j += 1;
            }
            lh += cnt;
        }
        i += 1;
    }
    id.bts[i as usize] = ByteType::ImpliedReturn;
    lh
}
pub fn dump_ttinstr(instructions: &[u8], options: &Options) -> BuiltValue {
    if options.instr_as_bytes {
        BuiltValue::Str(base64_encode(instructions))
    } else {
        let mut id = InstrData {
            instrs: instructions,
            instr_cnt: instructions.len() as u32,
            bts: Vec::new(),
        };
        instr_typify(&mut id);
        let mut ret = BuiltValue::new_array(id.instr_cnt as usize);
        let mut i: u32 = 0_u32;
        while i < id.instr_cnt {
            if id.bts[i as usize] == ByteType::WordHi {
                ret.push_item(BuiltValue::Int(
                    ((id.instrs[i as usize] as i32) << 8_i32 | id.instrs[i.wrapping_add(1_u32) as usize] as i32) as i16 as i64,
                ));
                i = i.wrapping_add(1);
            } else if id.bts[i as usize] == ByteType::Cnt || id.bts[i as usize] == ByteType::Byte {
                ret.push_item(BuiltValue::Int(id.instrs[i as usize] as i64));
            } else {
                ret.push_item(BuiltValue::Str(FF_TTF_INSTRNAMES[id.instrs[i as usize] as usize].to_vec()));
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
pub fn parse_ttinstr(col: Option<&ParsedValue>, mut make: impl FnMut(Vec<u8>), mut wrong: impl FnMut(&[u8], i32)) {
    let Some(col_ref) = col else {
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
    // No pre-computed total length needed any more (that was only ever in
    // service of a single `sdsnewlen`/zero-filled-`Vec` allocation sized
    // up front) -- `Vec::extend_from_slice`/`push` grow the buffer as they
    // go, and there's no `strlen`-terminator byte to leave room for either
    // (`parse_instrs` takes a real `&[u8]` now, with its own length).
    let mut instr_string: Vec<u8> = Vec::new();
    for record in items {
        if let Some(bytes) = record.as_str_bytes() {
            instr_string.extend_from_slice(bytes);
        } else if let Some(n) = record.as_int() {
            // Matches the original's `snprintf(head, 20, "%d", n as i32)`
            // exactly: plain decimal, no padding, and the same `as i32`
            // truncation for a value that doesn't fit ("%d" reads a C
            // `int`).
            instr_string.extend(crate::bytesbuild!(n as i32));
        } else {
            make(Vec::new());
            return;
        }
        instr_string.push(b'\n');
    }
    let instructions_0: Option<Vec<u8>> = parse_instrs(&instr_string, &mut wrong);
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
        let instrs: Vec<u8> = vec![TTF_NPUSHB, 2];
        let mut id = InstrData {
            instrs: &instrs,
            instr_cnt: instrs.len() as u32,
            bts: Vec::new(),
        };
        instr_typify(&mut id);
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
        let instrs: Vec<u8> = vec![0xb8, 0xab];
        let mut id = InstrData {
            instrs: &instrs,
            instr_cnt: instrs.len() as u32,
            bts: Vec::new(),
        };
        instr_typify(&mut id);
        // Must never be `WordHi` with nothing following it in `instrs`.
        assert_eq!(id.bts[1], ByteType::Byte);
    }

    // Stage L-8: `instr_name_matches`'s `token.len() == name.len()` guard
    // replaces the original's separate `(end-pt) == name.len()` check
    // that ran *alongside* `strnmatch`'s own comparison -- both are
    // required for a match, exact length included. "MDRP" (no bracket)
    // is a genuine 4-character *prefix* of "MDRP[grey]" but is not
    // itself a complete entry in `FF_TTF_INSTRNAMES` (every MDRP variant
    // needs a bracketed or numeric-suffixed qualifier) -- a prefix-only
    // comparison would wrongly resolve it to that opcode instead of
    // falling through to "not recognized", exactly the same outcome an
    // unrecognized name with no bracket at all gets (opcode 0, via the
    // `i == 256 as u8` wraparound). This test failed when
    // `instr_name_matches` was temporarily changed to `token.len() <=
    // name.len()` (a prefix comparison) during development, confirming
    // it actually exercises the guard -- `golden.rs`'s real-font fixtures
    // did not catch that same change, since none of them happen to feed
    // a bracket-family base name through without its bracket.
    #[test]
    fn a_bracket_family_base_name_without_its_bracket_is_not_treated_as_a_prefix_match() {
        let mdrp_grey = FF_TTF_INSTRNAMES
            .iter()
            .position(|n| *n == b"MDRP[grey]")
            .expect("MDRP[grey] must be in the name table") as u8;
        let result = parse_instrs(b"MDRP\n", |_, _| {}).unwrap();
        assert_eq!(result, vec![0]);
        assert_ne!(result, vec![mdrp_grey]);
    }

    // Mirrors `strtol(s, &mut end, 0)`'s base auto-detection exactly:
    // `"0x1F"`/hex, a leading `0` followed by another octal digit/octal,
    // anything else/decimal -- including the two corner cases where the
    // prefix alone doesn't have a digit to back it up (`"0x"` with no hex
    // digit after it, a bare `"0"`) and strtol falls back to consuming
    // just the leading `0` as decimal zero.
    #[test]
    fn strtol_base0_matches_libc_strtol_hex_octal_and_decimal_prefixes() {
        assert_eq!(strtol_base0(b"0x1F"), (0x1F, 4));
        assert_eq!(strtol_base0(b"010"), (8, 3));
        assert_eq!(strtol_base0(b"10"), (10, 2));
        assert_eq!(strtol_base0(b"-5"), (-5, 2));
        assert_eq!(strtol_base0(b"0"), (0, 1));
        assert_eq!(strtol_base0(b"0x"), (0, 1));
    }

    // Fuzz-found, pre-existing crash (see RUST_MIGRATION.md Stage M-14 and
    // M-15): a decimal/hex/octal digit run long enough to overflow `i64`
    // during accumulation used to panic with "attempt to multiply with
    // overflow" (plain `val * 10 + digit` arithmetic). `strtol_base0` now
    // saturates instead, mirroring libc `strtol`'s own overflow behavior
    // (clamp to `LONG_MAX`/`LONG_MIN` and keep scanning digits), so a
    // value this large -- however it's spelled -- can never be anything
    // but `i64::MAX` here, which is guaranteed to fail `parse_instrs`'s
    // `[-32768, 32767]` range check the same way libc's clamped
    // `LONG_MAX` would.
    #[test]
    fn strtol_base0_saturates_on_overflow_instead_of_panicking() {
        assert_eq!(strtol_base0(b"99999999999999999999999"), (i64::MAX, 23));
        assert_eq!(strtol_base0(b"-99999999999999999999999"), (i64::MIN + 1, 24));
        assert_eq!(strtol_base0(b"0xffffffffffffffffffff"), (i64::MAX, 22));
        assert_eq!(strtol_base0(b"07777777777777777777777777"), (i64::MAX, 26));
    }
}
