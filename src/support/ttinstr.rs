use otfcc_json::ParsedValue;

use crate::support::options::Options;

use crate::support::base64::{base64_decode, base64_encode};
use otfcc_json::BuiltValue;
/// The four opcodes `parse_instrs`/`instr_typify` have to recognise, because
/// their operands are part of the instruction stream rather than separate
/// instructions. `u8`, since that is what `InstrData.instrs` holds. Every
/// other opcode is looked up by name in `FF_TTF_INSTRNAMES` below.
pub const TTF_NPUSHB: u8 = 64;
pub const TTF_NPUSHW: u8 = 65;
pub const TTF_PUSHB: u8 = 176;
pub const TTF_PUSHW: u8 = 184;
// `instrs` borrows the instruction bytes for one `dump_ttinstr` call; `bts`
// (built by `instr_typify`) records each byte's role.
#[derive(Debug)]
pub struct InstrData<'a> {
    pub instrs: &'a [u8],
    pub instr_cnt: u32,
    /// What each byte of `instrs` *is*, one entry per byte, filled in by
    /// `instr_typify`. Not part of the instruction stream: the two arrays run
    /// in parallel, which is why this one is typed and `instrs` stays `u8`.
    pub bts: Vec<ByteType>,
}

/// The role of one byte in a TrueType instruction stream: the opcode itself, or
/// one of the operand bytes that follow a push. `Instr` is 0, the role every
/// byte starts with.
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
    if s[i..].len() >= 3
        && s[i] == b'0'
        && matches!(s[i + 1], b'x' | b'X')
        && s[i + 2].is_ascii_hexdigit()
    {
        let mut j = i + 2;
        let mut val: i64 = 0;
        while let Some(d) = s.get(j).and_then(|&b| (b as char).to_digit(16)) {
            val = val.saturating_mul(16).saturating_add(i64::from(d));
            j += 1;
        }
        return (if neg { -val } else { val }, j);
    }
    if s.get(i) == Some(&b'0') && s.get(i + 1).is_some_and(u8::is_ascii_digit) && s[i + 1] <= b'7' {
        let mut j = i + 1;
        let mut val: i64 = 0;
        while s.get(j).is_some_and(|&b| (b'0'..=b'7').contains(&b)) {
            val = val.saturating_mul(8).saturating_add(i64::from(s[j] - b'0'));
            j += 1;
        }
        return (if neg { -val } else { val }, j);
    }
    let mut j = i;
    let mut val: i64 = 0;
    while s.get(j).is_some_and(u8::is_ascii_digit) {
        val = val
            .saturating_mul(10)
            .saturating_add(i64::from(s[j] - b'0'));
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
        val = val * 2 + i64::from(s[j] - b'0');
        j += 1;
    }
    (if neg { -val } else { val }, j)
}
/// Case-insensitive *exact-length* comparison. The length check runs first,
/// so a token longer than `name` is rejected without reading past either
/// slice.
fn instr_name_matches(token: &[u8], name: &[u8]) -> bool {
    token.eq_ignore_ascii_case(name)
}
/// Case-insensitive *prefix* comparison -- `token` may be shorter than
/// `name`. Used only for the bracketed-family fallback scan
/// (`MDRP[...]`/`MIRP[...]`-style names); the `token.len() <= name.len()`
/// guard keeps the comparison inside both slices.
fn instr_name_has_prefix(token: &[u8], name: &[u8]) -> bool {
    token.len() <= name.len() && token.eq_ignore_ascii_case(&name[..token.len()])
}
/// How many values the most recent `PUSH*` instruction still expects, and
/// whether they are bytes or words. `left` is `-1` right after `NPUSHB` or
/// `NPUSHW`, when the count byte has not been read yet.
struct PushState {
    left: i32,
    size: i32,
}
fn skip_blanks(text: &[u8], pt: &mut usize) {
    while matches!(peek(text, *pt), b' ' | b'\t') {
        *pt += 1;
    }
}
fn at_line_end(text: &[u8], pt: usize) -> bool {
    matches!(peek(text, pt), b'\r' | b'\n' | 0)
}
/// Reads up to 256 numbers from the current line into `stack`, stopping at
/// the first token that is not a number. Returns how many were read, or
/// `None` (after reporting) if one does not fit in an `i16`.
fn read_number_run(
    text: &[u8],
    pt: &mut usize,
    stack: &mut [i16; 256],
    iv_error: &mut impl FnMut(&[u8], i32),
) -> Option<usize> {
    let mut npos = 0usize;
    while npos < 256 {
        skip_blanks(text, pt);
        let c = peek(text, *pt);
        if !(c.is_ascii_digit() || c == b'-') {
            break;
        }
        let (raw_val, consumed) = strtol_base0(&text[*pt..]);
        let val = raw_val as i32;
        if !(-32768..=32767).contains(&val) {
            iv_error(b"A value must be between [-32768,32767]", *pt as i32);
            return None;
        }
        *pt += consumed;
        stack[npos] = val as i16;
        npos += 1;
    }
    Some(npos)
}
/// Matches the numbers on a line against the push the previous instruction
/// opened (`state`): reads the `NPUSH*` count byte if it is due, then writes
/// as many values as the push still expects. Returns how many of the first
/// `npos` numbers were used, or `None` (after reporting) when the line
/// contradicts the push.
fn consume_pushed_values(
    text: &[u8],
    pt: usize,
    state: &mut PushState,
    stack: &[i16; 256],
    npos: usize,
    instrs: &mut Vec<u8>,
    iv_error: &mut impl FnMut(&[u8], i32),
) -> Option<usize> {
    let npos = npos as i32;
    let mut nread: i32 = 0;
    if state.left == -1 {
        if npos == 0 {
            iv_error(b"Expected a number for a push count", pt as i32);
        } else if i32::from(stack[0]) > 255 || i32::from(stack[0]) <= 0 {
            iv_error(
                b"The push count must be a number between 0 and 255",
                pt as i32,
            );
            return None;
        } else {
            nread = 1;
            instrs.push(stack[0] as u8);
            state.left = i32::from(stack[0]);
        }
    }
    if state.left != 0 && state.left < npos - nread && at_line_end(text, pt) {
        iv_error(b"More pushes specified than needed", pt as i32);
        return None;
    }
    while state.left > 0 && nread < npos {
        if state.size == 2 {
            instrs.push((i32::from(stack[nread as usize]) >> 8) as u8);
            instrs.push((i32::from(stack[nread as usize]) & 0xff) as u8);
            nread += 1;
        } else if i32::from(stack[0]) > 255 || i32::from(stack[0]) < 0 {
            iv_error(
                b"A value to be pushed by a byte push must be between 0 and 255",
                pt as i32,
            );
            return None;
        } else {
            instrs.push(stack[nread as usize] as u8);
            nread += 1;
        }
        state.left -= 1;
    }
    if nread < npos && state.left == 0 && at_line_end(text, pt) {
        iv_error(b"Unexpected number", pt as i32);
        return None;
    }
    Some(nread as usize)
}
/// Writes `stack[nread..npos]` as the shortest run of `PUSHB`/`NPUSHB` and
/// `PUSHW`/`NPUSHW` instructions: each run is a maximal stretch of values that
/// all fit in a byte (0..=255) or all need a word.
fn emit_implicit_pushes(stack: &[i16; 256], nread: usize, npos: usize, instrs: &mut Vec<u8>) {
    let (mut nread, npos) = (nread as i32, npos as i32);
    while nread < npos {
        let mut i = nread;
        let fits_byte = |v: i16| (0..=255).contains(&i32::from(v));
        if fits_byte(stack[nread as usize]) {
            while i < npos && fits_byte(stack[i as usize]) {
                i += 1;
            }
            if i - nread <= 8 {
                instrs.push((i32::from(TTF_PUSHB) + (i - nread) - 1) as u8);
            } else {
                instrs.push(TTF_NPUSHB);
                instrs.push((i - nread) as u8);
            }
            while nread < i {
                instrs.push(stack[nread as usize] as u8);
                nread += 1;
            }
        } else {
            while i < npos && !fits_byte(stack[i as usize]) {
                i += 1;
            }
            if i - nread <= 8 {
                instrs.push((i32::from(TTF_PUSHW) + (i - nread) - 1) as u8);
            } else {
                instrs.push(TTF_NPUSHW);
                instrs.push((i - nread) as u8);
            }
            while nread < i {
                instrs.push((i32::from(stack[nread as usize]) >> 8) as u8);
                instrs.push((i32::from(stack[nread as usize]) & 0xff) as u8);
                nread += 1;
            }
        }
    }
}
/// Reads the instruction name starting at `pt`, up to a space, line end or
/// NUL, and returns its opcode and where the name ends. A name that is not
/// in the table is retried as a bracketed family such as `MDRP[01101]`, whose
/// bracket digits are added to the family's base opcode. Returns `None`
/// (after reporting) if the bracket is not closed or its value is 32 or more.
/// A name that matches nothing gives opcode 256, which the caller truncates
/// to a byte.
fn read_instruction_name(
    text: &[u8],
    pt: usize,
    iv_error: &mut impl FnMut(&[u8], i32),
) -> Option<(i32, usize)> {
    let mut brack: Option<usize> = None;
    let mut end = pt;
    while !matches!(peek(text, end), b'\r' | b'\n' | b' ' | 0) {
        if matches!(peek(text, end), b'[' | b'_') {
            brack = Some(end);
        }
        end += 1;
    }
    let mut i: i32 = 0;
    while i < 256 {
        if instr_name_matches(&text[pt..end], FF_TTF_INSTRNAMES[i as usize]) {
            break;
        }
        i += 1;
    }
    if i == 256
        && let Some(brack) = brack
    {
        i = 0;
        while i < 256 {
            if instr_name_has_prefix(&text[pt..=brack], FF_TTF_INSTRNAMES[i as usize]) {
                break;
            }
            i += 1;
        }
        let (raw_val, consumed) = strtol_base2(&text[brack + 1..]);
        let val = raw_val as i32;
        let mut bend = brack + 1 + consumed;
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
        if val >= 32 {
            iv_error(b"Bracketted value is too large", pt as i32);
            return None;
        }
        i += val;
    }
    Some((i, end))
}
impl PushState {
    /// Records what a just-written opcode `i` expects next: `PUSHB`/`PUSHW`
    /// name their count, `NPUSHB`/`NPUSHW` read it from the next number.
    fn after_opcode(&mut self, i: i32) {
        let pushb = i32::from(TTF_PUSHB);
        let pushw = i32::from(TTF_PUSHW);
        let is_npush = i == i32::from(TTF_NPUSHB) || i == i32::from(TTF_NPUSHW);
        if is_npush || (i >= pushb && i <= pushw + 7) {
            self.size = if i == i32::from(TTF_NPUSHB) || (i >= pushb && i <= pushb + 7) {
                1
            } else {
                2
            };
            if is_npush {
                self.left = -1;
            } else if i >= pushb && i <= pushb + 7 {
                self.left = i - pushb + 1;
            } else {
                self.left = i - pushw + 1;
            }
        }
    }
}
// `iv_error` is called with the offending token and its position for every
// word that is neither a known instruction nor a number in range; each
// caller (`table/fpgm_prep.rs`, `table/glyf.rs`) supplies its own warning.
fn parse_instrs(text: &[u8], mut iv_error: impl FnMut(&[u8], i32)) -> Option<Vec<u8>> {
    let mut numberstack: [i16; 256] = [0; 256];
    let mut push = PushState { left: 0, size: 0 };
    let mut instrs: Vec<u8> = Vec::with_capacity(text.len());
    let mut pt: usize = 0;
    while peek(text, pt) != 0 {
        let npos = read_number_run(text, &mut pt, &mut numberstack, &mut iv_error)?;
        skip_blanks(text, &mut pt);
        if !(npos == 0 && at_line_end(text, pt)) {
            let nread = consume_pushed_values(
                text,
                pt,
                &mut push,
                &numberstack,
                npos,
                &mut instrs,
                &mut iv_error,
            )?;
            if !at_line_end(text, pt) {
                if push.left > 0 {
                    iv_error(b"Missing pushes", pt as i32);
                    return None;
                }
                emit_implicit_pushes(&numberstack, nread, npos, &mut instrs);
                let (opcode, end) = read_instruction_name(text, pt, &mut iv_error)?;
                pt = end;
                instrs.push(opcode as u8);
                push.after_opcode(opcode);
                if peek(text, pt) == 0 {
                    break;
                }
            }
        }
        pt += 1;
    }
    Some(instrs)
}
/// Marks the `cnt` operands that follow the push instruction at `*i`, as
/// bytes or as high/low halves of words, leaving `*i` on the last operand
/// marked. Returns `false` when the operands run off the end of the
/// instruction block, with `*i == len`; the stream is then left untypified
/// from there on instead of being read or written past `len`.
///
/// `bts` is sized to exactly `len + 1` slots, so no write may land past
/// `len`. A `WordHi` marker promises `dump_ttinstr` a paired `WordLo` right
/// after it, so a word operand with only one byte left is marked `Byte`
/// instead (and `*i` advanced to `len` so the caller's `ImpliedReturn` write
/// does not clobber it). Both cases were heap overflows found by fuzzing.
fn mark_push_operands(id: &mut InstrData, i: &mut i32, cnt: i32, words: bool) -> bool {
    let len: i32 = id.instr_cnt as i32;
    for _ in 0..cnt {
        *i += 1;
        if *i >= len {
            return false;
        }
        if !words {
            id.bts[*i as usize] = ByteType::Byte;
        } else if *i + 1 >= len {
            id.bts[*i as usize] = ByteType::Byte;
            *i += 1;
            return false;
        } else {
            id.bts[*i as usize] = ByteType::WordHi;
            *i += 1;
            id.bts[*i as usize] = ByteType::WordLo;
        }
    }
    true
}
fn instr_typify(id: &mut InstrData) -> i32 {
    let len: i32 = id.instr_cnt as i32;
    if id.bts.is_empty() {
        id.bts = vec![ByteType::Instr; (len + 1) as usize];
    }
    let mut lh: i32 = 0;
    let mut i: i32 = 0;
    while i < len {
        id.bts[i as usize] = ByteType::Instr;
        lh += 1;
        let opcode = id.instrs[i as usize];
        if opcode == TTF_NPUSHB || opcode == TTF_NPUSHW {
            // The count byte follows the opcode; the operands follow the count.
            i += 1;
            if i >= len {
                break;
            }
            id.bts[i as usize] = ByteType::Cnt;
            let words = opcode == TTF_NPUSHW;
            if words {
                lh += 1;
            }
            let cnt = i32::from(id.instrs[i as usize]);
            if !mark_push_operands(id, &mut i, cnt, words) {
                break;
            }
            lh += 1 + cnt;
        } else if i32::from(opcode) & 0xf8 == 0xb0 || i32::from(opcode) & 0xf8 == 0xb8 {
            // `PUSHB[n]` / `PUSHW[n]`: the count is in the opcode's low bits.
            let words = i32::from(opcode) & 0xf8 == 0xb8;
            let cnt = (i32::from(opcode) & 7) + 1;
            if !mark_push_operands(id, &mut i, cnt, words) {
                break;
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
        let mut i: u32 = 0;
        while i < id.instr_cnt {
            if id.bts[i as usize] == ByteType::WordHi {
                ret.push_item(BuiltValue::Int(i64::from(
                    (i32::from(id.instrs[i as usize]) << 8
                        | i32::from(id.instrs[i.wrapping_add(1) as usize]))
                        as i16,
                )));
                i = i.wrapping_add(1);
            } else if id.bts[i as usize] == ByteType::Cnt || id.bts[i as usize] == ByteType::Byte {
                ret.push_item(BuiltValue::Int(i64::from(id.instrs[i as usize])));
            } else {
                ret.push_item(BuiltValue::Str(
                    FF_TTF_INSTRNAMES[id.instrs[i as usize] as usize].to_vec(),
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
// `parse_fpgm_prep` and `table/glyf.rs`'s `glyf_parse_glyph`,
// each with a different concrete target for `make` to write into).
pub fn parse_ttinstr(
    col: Option<&ParsedValue>,
    mut make: impl FnMut(Vec<u8>),
    mut wrong: impl FnMut(&[u8], i32),
) {
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
    let mut instr_string: Vec<u8> = Vec::new();
    for record in items {
        if let Some(bytes) = record.as_str_bytes() {
            instr_string.extend_from_slice(bytes);
        } else if let Some(n) = record.as_int() {
            // Plain decimal of `n as i32`, matching the `%d` this output
            // has always used.
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

    // `instr_typify` starts from a zero-filled array, one byte per
    // instruction byte, so `ByteType::Instr` has to be the zero variant and
    // the type has to stay one byte wide.
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

    // `instr_name_matches` requires an exact length match. "MDRP" (no
    // bracket) is a 4-character *prefix* of "MDRP[grey]" but is not itself
    // an entry in `FF_TTF_INSTRNAMES` (every MDRP variant needs a bracketed
    // or numeric-suffixed qualifier), so it must fall through to "not
    // recognized" (opcode 0) rather than resolve to that opcode. This test
    // fails if the guard is loosened to `token.len() <= name.len()`; none of
    // `golden.rs`'s real-font fixtures feed a bracket-family base name
    // through without its bracket.
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

    // Fuzz-found: a digit run long enough to overflow `i64` during
    // accumulation used to panic with "attempt to multiply with overflow".
    // `strtol_base0` saturates instead, like libc `strtol` (clamp to
    // `LONG_MAX`/`LONG_MIN` and keep scanning digits), so such a value is
    // always `i64::MAX` here and always fails `parse_instrs`'s
    // `[-32768, 32767]` range check.
    #[test]
    fn strtol_base0_saturates_on_overflow_instead_of_panicking() {
        assert_eq!(strtol_base0(b"99999999999999999999999"), (i64::MAX, 23));
        assert_eq!(
            strtol_base0(b"-99999999999999999999999"),
            (i64::MIN + 1, 24)
        );
        assert_eq!(strtol_base0(b"0xffffffffffffffffffff"), (i64::MAX, 22));
        assert_eq!(strtol_base0(b"07777777777777777777777777"), (i64::MAX, 26));
    }
}
