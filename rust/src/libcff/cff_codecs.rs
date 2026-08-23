#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{printf, sprintf, strlen, strtod};

use crate::libcff::CffDictOperator;
use crate::libcff::cff_value::{CS2_FRACTION, CS2_OPERAND, CS2_OPERATOR, CffValue, CffValueType};
use crate::support::NULL;
use crate::support::buffer::bufnew;
use crate::support::buffer::{Buffer, bufninit, bufwrite8};
use crate::support::font_reader::FontReader;
#[inline]
unsafe fn atof(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double {
    return strtod(__nptr, NULL as *mut *mut ::core::ffi::c_char);
}
/// Every caller passes a DICT operator, so the parameter says so. The body
/// still works in `i32` -- unchanged arithmetic, unchanged bytes.
pub unsafe fn cff_encode_cff_operator(mut val: CffDictOperator) -> *mut Buffer {
    let val = val.0 as i32;
    if val > 256 as i32 {
        return bufninit(&[(val / 256 as i32) as u8, (val % 256 as i32) as u8]);
    } else {
        return bufninit(&[val as u8]);
    };
}
pub unsafe fn cff_encode_cff_integer(mut val: i32) -> *mut Buffer {
    if val >= -(107 as i32) && val <= 107 as i32 {
        return bufninit(&[(val + 139 as i32) as u8]);
    } else if val >= 108 as i32 && val <= 1131 as i32 {
        val = (val as ::core::ffi::c_int - 108 as ::core::ffi::c_int) as i32;
        return bufninit(&[
            ((val >> 8 as ::core::ffi::c_int) + 247 as i32) as u8,
            (val & 0xff as i32) as u8,
        ]);
    } else if val >= -(1131 as i32) && val <= -(108 as i32) {
        val = -(108 as i32) - val;
        return bufninit(&[
            ((val >> 8 as ::core::ffi::c_int) + 251 as i32) as u8,
            (val & 0xff as i32) as u8,
        ]);
    } else if val >= -(32768 as i32) && val < 32768 as i32 {
        return bufninit(&[
            28 as u8,
            (val >> 8 as ::core::ffi::c_int) as u8,
            (val & 0xff as i32) as u8,
        ]);
    } else {
        return bufninit(&[
            29 as u8,
            (val >> 24 as ::core::ffi::c_int & 0xff as i32) as u8,
            (val >> 16 as ::core::ffi::c_int & 0xff as i32) as u8,
            (val >> 8 as ::core::ffi::c_int & 0xff as i32) as u8,
            (val & 0xff as i32) as u8,
        ]);
    };
}
pub unsafe fn cff_encode_cff_float(mut val: ::core::ffi::c_double) -> *mut Buffer {
    let mut blob: *mut Buffer = bufnew();
    let mut i: u32 = 0;
    let mut j: u32 = 0 as u32;
    let mut temp: [u8; 32] = [
        0 as ::core::ffi::c_int as u8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if val == 0.0f64 {
        bufwrite8(blob, 30 as u8);
        bufwrite8(blob, 0xf as u8);
    } else {
        let mut niblen: u32 = 0 as u32;
        let mut array: Vec<u8>;
        sprintf(
            &raw mut temp as *mut u8 as *mut ::core::ffi::c_char,
            b"%.13g\0" as *const u8 as *const ::core::ffi::c_char,
            val,
        );
        i = 0 as u32;
        while (i as usize) < strlen(&raw mut temp as *mut u8 as *mut ::core::ffi::c_char) {
            if temp[i as usize] as ::core::ffi::c_int == '.' as i32 {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int >= '0' as i32
                && temp[i as usize] as ::core::ffi::c_int <= '9' as i32
            {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as u32) as usize] as ::core::ffi::c_int == '-' as i32
            {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(2 as u32);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as u32) as usize] as ::core::ffi::c_int == '+' as i32
            {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(2 as u32);
            } else if temp[i as usize] as ::core::ffi::c_int == '-' as i32 {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(1);
            }
        }
        let blob_size: usize = (2 as u32).wrapping_add(niblen.wrapping_div(2 as u32)) as usize;
        bufwrite8(blob, 30 as u8);
        if niblen.wrapping_rem(2 as u32) != 0 as u32 {
            array = vec![0u8; niblen.wrapping_add(1 as u32) as usize];
            array[niblen as usize] = 0xf as u8;
        } else {
            array = vec![0u8; niblen.wrapping_add(2 as u32) as usize];
            array[niblen.wrapping_add(1 as u32) as usize] = 0xf as u8;
            array[niblen as usize] = 0xf as u8;
        }
        i = 0 as u32;
        while (i as usize) < strlen(&raw mut temp as *mut u8 as *mut ::core::ffi::c_char) {
            if temp[i as usize] as ::core::ffi::c_int == '.' as i32 {
                let fresh0 = j;
                j = j.wrapping_add(1);
                array[fresh0 as usize] = 0xa as u8;
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int >= '0' as i32
                && temp[i as usize] as ::core::ffi::c_int <= '9' as i32
            {
                let fresh1 = j;
                j = j.wrapping_add(1);
                array[fresh1 as usize] =
                    (temp[i as usize] as ::core::ffi::c_int - '0' as i32) as u8;
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as u32) as usize] as ::core::ffi::c_int == '-' as i32
            {
                let fresh2 = j;
                j = j.wrapping_add(1);
                array[fresh2 as usize] = 0xc as u8;
                i = i.wrapping_add(2 as u32);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as u32) as usize] as ::core::ffi::c_int == '+' as i32
            {
                let fresh3 = j;
                j = j.wrapping_add(1);
                array[fresh3 as usize] = 0xb as u8;
                i = i.wrapping_add(2 as u32);
            } else if temp[i as usize] as ::core::ffi::c_int == '-' as i32 {
                let fresh4 = j;
                j = j.wrapping_add(1);
                array[fresh4 as usize] = 0xe as u8;
                i = i.wrapping_add(1);
            }
        }
        i = 1 as u32;
        while (i as usize) < blob_size {
            bufwrite8(
                blob,
                (array[i.wrapping_sub(1 as u32).wrapping_mul(2 as u32) as usize]
                    as ::core::ffi::c_int
                    * 16 as ::core::ffi::c_int
                    + array[i
                        .wrapping_sub(1 as u32)
                        .wrapping_mul(2 as u32)
                        .wrapping_add(1 as u32) as usize] as ::core::ffi::c_int)
                    as u8,
            );
            i = i.wrapping_add(1);
        }
    }
    return blob;
}
// Every one of the token decoders in this file (`cff_decode_cs2_token`,
// `cff_dec_i`/`cff_dec_r`/`cff_dec_o`/`cff_dec_e`) used to read up to 5
// bytes from `start` unconditionally, based only on the *first* byte's
// value, with no idea how many bytes actually remained. Both this file's
// callers already bound their own top-level walk against a real length
// (`cff_parse_outline`'s `while start < data.offset(len)`, `cff_dict.rs`'s
// `parse_to_callback`'s equivalent) -- but that only checks *before*
// decoding a token, not that the token *itself* stays within bounds, so a
// token starting near the end of a truncated CharString or DICT could
// still read past it. Every decoder here now takes `remaining` (the byte
// count actually available from `start`) and returns `Option<u32>`,
// `None` on any read that would run past it; both callers stop their walk
// on `None` instead of reading on.
pub unsafe fn cff_decode_cs2_token(
    start: *const u8,
    remaining: usize,
    val: *mut CffValue,
) -> Option<u32> {
    let slice = ::core::slice::from_raw_parts(start, remaining);
    let mut r = FontReader::new(slice);
    let b0 = r.u8().ok()?;
    let advance: u32;
    if b0 <= 27 {
        (*val).t = CS2_OPERATOR;
        if b0 == 12 {
            let b1 = r.u8().ok()?;
            (*val).c2rust_unnamed.i = ((b0 as i32) << 8) | b1 as i32;
            advance = 2;
        } else {
            // 0-11 and 13-27 all take this same one-byte-operator shape
            // in the original.
            (*val).c2rust_unnamed.i = b0 as i32;
            advance = 1;
        }
    } else if b0 == 28 {
        (*val).t = CS2_OPERAND;
        let b1 = r.u8().ok()?;
        let b2 = r.u8().ok()?;
        (*val).c2rust_unnamed.i = ((((b1 as i32) << 8) | b2 as i32) as i16) as i32;
        advance = 3;
    } else if (29..=31).contains(&b0) {
        (*val).t = CS2_OPERATOR;
        (*val).c2rust_unnamed.i = b0 as i32;
        advance = 1;
    } else if (32..=254).contains(&b0) {
        (*val).t = CS2_OPERAND;
        if (32..=246).contains(&b0) {
            (*val).c2rust_unnamed.i = b0 as i32 - 139;
            advance = 1;
        } else if (247..=250).contains(&b0) {
            let b1 = r.u8().ok()?;
            (*val).c2rust_unnamed.i = (b0 as i32 - 247) * 256 + b1 as i32 + 108;
            advance = 2;
        } else {
            // 251-254
            let b1 = r.u8().ok()?;
            (*val).c2rust_unnamed.i = -((b0 as i32 - 251) * 256) - b1 as i32 - 108;
            advance = 2;
        }
    } else {
        // b0 == 255
        (*val).t = CS2_FRACTION;
        let b1 = r.u8().ok()?;
        let b2 = r.u8().ok()?;
        let b3 = r.u8().ok()?;
        let b4 = r.u8().ok()?;
        let integer_part = ((((b1 as i32) << 8) | b2 as i32) as i16) as i32;
        let fraction_part = ((((b3 as i32) << 8) | b4 as i32) as u16) as i32;
        (*val).c2rust_unnamed.d = integer_part as ::core::ffi::c_double
            + fraction_part as ::core::ffi::c_double / 65536.0f64;
        advance = 5;
    }
    if (*val).t as ::core::ffi::c_uint == CS2_OPERAND as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*val).c2rust_unnamed.d = (*val).c2rust_unnamed.i as ::core::ffi::c_double;
        (*val).t = CS2_FRACTION;
    }
    Some(advance)
}
unsafe fn cff_dec_i(start: *const u8, remaining: usize, val: *mut CffValue) -> Option<u32> {
    let slice = ::core::slice::from_raw_parts(start, remaining);
    let mut r = FontReader::new(slice);
    let b0 = r.u8().ok()?;
    let len: u32;
    if (32..=246).contains(&b0) {
        (*val).c2rust_unnamed.i = b0 as i32 - 139;
        len = 1;
    } else if (247..=250).contains(&b0) {
        let b1 = r.u8().ok()?;
        (*val).c2rust_unnamed.i = (b0 as i32 - 247) * 256 + b1 as i32 + 108;
        len = 2;
    } else if (251..=254).contains(&b0) {
        let b1 = r.u8().ok()?;
        (*val).c2rust_unnamed.i = -(b0 as i32 - 251) * 256 - b1 as i32 - 108;
        len = 2;
    } else if b0 == 28 {
        let b1 = r.u8().ok()?;
        let b2 = r.u8().ok()?;
        (*val).c2rust_unnamed.i = ((b1 as i32) << 8) | b2 as i32;
        len = 3;
    } else if b0 == 29 {
        let b1 = r.u8().ok()?;
        let b2 = r.u8().ok()?;
        let b3 = r.u8().ok()?;
        let b4 = r.u8().ok()?;
        (*val).c2rust_unnamed.i =
            ((b1 as i32) << 24) | ((b2 as i32) << 16) | ((b3 as i32) << 8) | b4 as i32;
        len = 5;
    } else {
        len = 0;
    }
    (*val).t = CffValueType::Integer;
    Some(len)
}
static NIBBLE_SYMB: [&::core::ffi::CStr; 15] = [
    c"0", c"1", c"2", c"3", c"4", c"5", c"6", c"7", c"8", c"9", c".", c"E", c"E-", c"", c"-",
];
// The original scanned the nibble string with no bound at all beyond
// finding a `0xF` terminator nibble -- a malformed DICT real number that
// never has one read arbitrarily far past the buffer. It also built the
// decoded text with `strcat` into a fixed 72-byte stack buffer with no
// check that the (attacker-controlled) nibble count actually fit --
// `restr` is exactly the C-string-manipulation shape flagged crate-wide
// as the main outstanding risk here. Both are closed by scanning through
// a bounds-checked slice and building the text into a growable `Vec<u8>`
// instead of a fixed buffer; `atof`/`strtod` is still what actually
// parses it, unchanged, since that's the number-formatting fidelity this
// PR isn't trying to touch.
unsafe fn cff_dec_r(start: *const u8, remaining: usize, val: *mut CffValue) -> Option<u32> {
    let slice = ::core::slice::from_raw_parts(start, remaining);
    let mut text: Vec<u8> = Vec::new();
    let mut nibst: usize = 1;
    loop {
        let &byte = slice.get(nibst)?;
        let a = byte / 16;
        let b = byte % 16;
        if a == 0xf {
            break;
        }
        text.extend_from_slice(NIBBLE_SYMB[a as usize].to_bytes());
        if b == 0xf {
            break;
        }
        text.extend_from_slice(NIBBLE_SYMB[b as usize].to_bytes());
        nibst += 1;
    }
    let len = (nibst + 1) as u32;
    text.push(0); // NUL-terminate for atof/strtod, matching the original's atof(restr) call
    (*val).c2rust_unnamed.d = atof(text.as_ptr() as *const ::core::ffi::c_char);
    (*val).t = CffValueType::Double;
    Some(len)
}
unsafe fn cff_dec_o(start: *const u8, remaining: usize, val: *mut CffValue) -> Option<u32> {
    let slice = ::core::slice::from_raw_parts(start, remaining);
    let mut r = FontReader::new(slice);
    let b0 = r.u8().ok()?;
    let len: u32;
    if b0 <= 21 {
        if b0 != 12 {
            (*val).c2rust_unnamed.i = b0 as i32;
            len = 1;
        } else {
            let b1 = r.u8().ok()?;
            (*val).c2rust_unnamed.i = b0 as i32 * 256 + b1 as i32;
            len = 2;
        }
    } else {
        len = 0;
    }
    (*val).t = CffValueType::Operator;
    Some(len)
}
unsafe fn cff_dec_e(start: *const u8, remaining: usize, val: *mut CffValue) -> Option<u32> {
    if remaining < 1 {
        return None;
    }
    printf(
        b"Undefined Byte in CFF: %d.\n\0" as *const u8 as *const ::core::ffi::c_char,
        *start as ::core::ffi::c_int,
    );
    (*val).c2rust_unnamed.i = *start as i32;
    (*val).t = CffValueType::Integer;
    Some(1)
}
static DE_T2: [Option<unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>>; 256] = {
    [
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_o as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_e as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_e as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_e as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_e as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_e as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_e as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_r as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_e as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_i as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
        Some(cff_dec_e as unsafe fn(*const u8, usize, *mut CffValue) -> Option<u32>),
    ]
};
pub unsafe fn cff_decode_cff_token(
    start: *const u8,
    remaining: usize,
    val: *mut CffValue,
) -> Option<u32> {
    if remaining < 1 {
        return None;
    }
    DE_T2[*start as usize].expect("non-null function pointer")(start, remaining, val)
}

#[cfg(test)]
mod token_decoder_tests {
    use super::*;
    use crate::libcff::cff_value::CffValueBody;

    fn zeroed_val() -> CffValue {
        CffValue {
            t: CffValueType::Unset,
            c2rust_unnamed: CffValueBody { i: 0 },
        }
    }

    #[test]
    fn cs2_token_reads_a_single_byte_operand() {
        let data = [100u8]; // 32..=246 -> operand = 100-139 = -39
        let mut val = zeroed_val();
        unsafe {
            let advance = cff_decode_cs2_token(data.as_ptr(), data.len(), &raw mut val).unwrap();
            assert_eq!(advance, 1);
            assert!(matches!(val.t, CffValueType::Double));
            assert_eq!(val.c2rust_unnamed.d, -39.0);
        }
    }

    #[test]
    fn cs2_token_reads_a_five_byte_fraction() {
        let data = [255u8, 0, 2, 0x80, 0x00]; // integer=2, fraction=32768/65536=0.5
        let mut val = zeroed_val();
        unsafe {
            let advance = cff_decode_cs2_token(data.as_ptr(), data.len(), &raw mut val).unwrap();
            assert_eq!(advance, 5);
            assert_eq!(val.c2rust_unnamed.d, 2.5);
        }
    }

    #[test]
    fn cs2_token_truncated_fraction_is_rejected_instead_of_reading_oob() {
        let data = [255u8, 0, 2]; // needs 5 bytes, only 3 present
        let mut val = zeroed_val();
        unsafe {
            assert!(cff_decode_cs2_token(data.as_ptr(), data.len(), &raw mut val).is_none());
        }
    }

    #[test]
    fn cs2_token_truncated_three_byte_operand_is_rejected_instead_of_reading_oob() {
        let data = [28u8, 0x00]; // needs 3 bytes, only 2 present
        let mut val = zeroed_val();
        unsafe {
            assert!(cff_decode_cs2_token(data.as_ptr(), data.len(), &raw mut val).is_none());
        }
    }

    #[test]
    fn cff_token_reads_a_dict_integer() {
        let data = [200u8]; // 32..=246 -> 200-139 = 61
        let mut val = zeroed_val();
        unsafe {
            let advance = cff_decode_cff_token(data.as_ptr(), data.len(), &raw mut val).unwrap();
            assert_eq!(advance, 1);
            assert!(matches!(val.t, CffValueType::Integer));
            assert_eq!(val.c2rust_unnamed.i, 61);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "calls libc::strtod via atof, unsupported under Miri")]
    fn cff_token_reads_a_dict_real_number() {
        // format=30 (dispatches to cff_dec_r), nibbles 1,'.',5,terminator
        // packed as 0x1A, 0x5F -> "1.5".
        let data = [30u8, 0x1A, 0x5F];
        let mut val = zeroed_val();
        unsafe {
            let advance = cff_decode_cff_token(data.as_ptr(), data.len(), &raw mut val).unwrap();
            assert_eq!(advance, 3);
            assert!(matches!(val.t, CffValueType::Double));
            assert_eq!(val.c2rust_unnamed.d, 1.5);
        }
    }

    #[test]
    fn cff_token_real_number_with_no_terminator_nibble_is_rejected_instead_of_reading_oob() {
        // The original scanned forward with no bound at all until it
        // found a 0xF terminator nibble -- a malformed real number that
        // never has one used to read arbitrarily far past the buffer.
        let data = [30u8, 0x12]; // neither nibble is 0xf, and there's no more data
        let mut val = zeroed_val();
        unsafe {
            assert!(cff_decode_cff_token(data.as_ptr(), data.len(), &raw mut val).is_none());
        }
    }

    #[test]
    fn cff_token_zero_remaining_is_rejected() {
        let data = [42u8];
        let mut val = zeroed_val();
        unsafe {
            assert!(cff_decode_cff_token(data.as_ptr(), 0, &raw mut val).is_none());
        }
    }
}
