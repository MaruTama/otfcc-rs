// c2rust translated the C source's <ctype.h> macro/inline expansions
// literally. On glibc these route through the internal table-pointer
// functions `__ctype_b_loc`/`__ctype_tolower_loc`/`__ctype_toupper_loc`,
// which the transpiled call sites (in sds.rs, json.rs, ttinstr.rs) still
// reference by name via `extern "C" { fn __ctype_b_loc() -> ...; }`. Those
// symbols don't exist on macOS (Darwin libc classifies chars differently
// internally), so a macOS build fails to link.
//
// Rather than rewrite every call site (the reconstructed expansions vary in
// shape: direct bitmask tests, `tolower`/`toupper` reimplementations feeding
// a `__res` local, etc.), this provides real, portable implementations of
// the three table-pointer functions themselves, built from the plain "C"
// locale classification rules (this codebase never calls `setlocale`, so
// that's the only locale ever actually in effect) — every existing call
// site keeps working completely unmodified, on any platform.
//
// glibc's tables are indexable from -128..255 (to classify a byte that was
// sign-extended from a signed `char`, e.g. `byte as c_char as c_int`). Any
// index outside plain 0..=127 ASCII classifies as "no bits set" here, which
// matches glibc's own "C" locale table (only 0..127 has non-zero bits) and
// sidesteps needing to special-case EOF(-1) separately.

use core::ffi::c_ushort;
#[cfg(target_os = "macos")]
use core::ffi::c_int;

const IS_UPPER: c_ushort = 1 << 8;
const IS_LOWER: c_ushort = 1 << 9;
const IS_ALPHA: c_ushort = 1 << 10;
const IS_DIGIT: c_ushort = 1 << 11;
const IS_XDIGIT: c_ushort = 1 << 12;
const IS_SPACE: c_ushort = 1 << 13;
const IS_PRINT: c_ushort = 1 << 14;
const IS_GRAPH: c_ushort = 1 << 15;
const IS_BLANK: c_ushort = 1 << 0;
const IS_CNTRL: c_ushort = 1 << 1;
const IS_PUNCT: c_ushort = 1 << 2;
const IS_ALNUM: c_ushort = 1 << 3;

const fn classify_ascii(c: i32) -> c_ushort {
    if c < 0 || c > 127 {
        return 0;
    }
    let b = c as u8;
    let is_upper = b.is_ascii_uppercase();
    let is_lower = b.is_ascii_lowercase();
    let is_digit = b.is_ascii_digit();
    let is_space = matches!(b, b' ' | 0x09..=0x0D); // space, \t \n \v \f \r
    let is_cntrl = b < 0x20 || b == 0x7F;
    let is_alpha = is_upper || is_lower;
    let is_alnum = is_alpha || is_digit;
    let is_xdigit = b.is_ascii_hexdigit();
    let is_print = b >= 0x20 && b < 0x7F;
    let is_graph = is_print && b != b' ';
    let is_blank = b == b' ' || b == b'\t';
    let is_punct = is_graph && !is_alnum;

    let mut bits: c_ushort = 0;
    if is_upper {
        bits |= IS_UPPER;
    }
    if is_lower {
        bits |= IS_LOWER;
    }
    if is_alpha {
        bits |= IS_ALPHA;
    }
    if is_digit {
        bits |= IS_DIGIT;
    }
    if is_xdigit {
        bits |= IS_XDIGIT;
    }
    if is_space {
        bits |= IS_SPACE;
    }
    if is_print {
        bits |= IS_PRINT;
    }
    if is_graph {
        bits |= IS_GRAPH;
    }
    if is_blank {
        bits |= IS_BLANK;
    }
    if is_cntrl {
        bits |= IS_CNTRL;
    }
    if is_punct {
        bits |= IS_PUNCT;
    }
    if is_alnum {
        bits |= IS_ALNUM;
    }
    bits
}

const fn to_lower_ascii(c: i32) -> i32 {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        c + 32
    } else {
        c
    }
}

const fn to_upper_ascii(c: i32) -> i32 {
    if c >= 'a' as i32 && c <= 'z' as i32 {
        c - 32
    } else {
        c
    }
}

const fn build_class_table() -> [c_ushort; 384] {
    let mut table = [0 as c_ushort; 384];
    let mut i = 0;
    while i < 384 {
        table[i] = classify_ascii(i as i32 - 128);
        i += 1;
    }
    table
}

const fn build_lower_table() -> [i32; 384] {
    let mut table = [0i32; 384];
    let mut i = 0;
    while i < 384 {
        let c = i as i32 - 128;
        table[i] = if c < 0 || c > 127 { c } else { to_lower_ascii(c) };
        i += 1;
    }
    table
}

const fn build_upper_table() -> [i32; 384] {
    let mut table = [0i32; 384];
    let mut i = 0;
    while i < 384 {
        let c = i as i32 - 128;
        table[i] = if c < 0 || c > 127 { c } else { to_upper_ascii(c) };
        i += 1;
    }
    table
}

static CLASS_TABLE: [c_ushort; 384] = build_class_table();
static LOWER_TABLE: [i32; 384] = build_lower_table();
static UPPER_TABLE: [i32; 384] = build_upper_table();

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn __ctype_b_loc() -> *mut *const c_ushort {
    static mut PTR: *const c_ushort = core::ptr::null();
    PTR = CLASS_TABLE.as_ptr().offset(128);
    core::ptr::addr_of_mut!(PTR)
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn __ctype_tolower_loc() -> *mut *const c_int {
    static mut PTR: *const c_int = core::ptr::null();
    PTR = LOWER_TABLE.as_ptr().offset(128);
    core::ptr::addr_of_mut!(PTR)
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn __ctype_toupper_loc() -> *mut *const c_int {
    static mut PTR: *const c_int = core::ptr::null();
    PTR = UPPER_TABLE.as_ptr().offset(128);
    core::ptr::addr_of_mut!(PTR)
}
