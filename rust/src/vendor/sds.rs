#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcmp, memcpy, memmove, memset, realloc, strchr, strlen};

use crate::support::ctype_compat::{c_isprint, c_isspace, c_tolower, c_toupper};
pub type SdsRaw = *mut ::core::ffi::c_char;

/// The fixed header stored immediately before an sds's data bytes.
///
/// The original (redis-derived) `sds` this was transpiled from picked from
/// five variable-width header shapes (`SDS_TYPE_5`/`8`/`16`/`32`/`64`,
/// `#[repr(C, packed)]` structs selected by a tag byte) to save a few bytes
/// per string when managing millions of tiny database keys. otfcc handles at
/// most a few thousand strings per font, so that micro-optimization was pure
/// complexity here: one fixed-size header, chosen unconditionally, removes
/// the whole `SDS_TYPE_*` dispatch without changing anything any caller can
/// observe. Nothing outside this file reads `len`/`cap` except through
/// `sdslen`/`sdsalloc` (confirmed by grep once the last of the 20 duplicated
/// `sdslen()` copies elsewhere in the crate were consolidated onto this
/// module's own), and the content bytes -- what every call site actually
/// cares about -- are laid out identically either way. The trade is a
/// slightly larger header for very short strings (16 bytes here on a 64-bit
/// target vs. as little as 1 for the old `SDS_TYPE_5`); irrelevant at this
/// crate's scale.
#[repr(C)]
struct SdsHeader {
    len: usize,
    cap: usize,
}

const SDS_HDR_SIZE: usize = ::core::mem::size_of::<SdsHeader>();

/// The header sits immediately before the data `s` points at -- the same
/// "walk backward from the string pointer" trick the original used, just
/// with one header shape instead of five. This is what lets `SdsRaw` stay a
/// bare `*mut c_char` (so none of the crate's ~670 `sdsXxx()` call sites, or
/// the `#[repr(C)]` struct fields typed `SdsRaw`, have to change) while the
/// allocation backing it stops needing manual bit-packed type dispatch.
#[inline]
unsafe fn sds_header(s: SdsRaw) -> *mut SdsHeader {
    (s as *mut u8).sub(SDS_HDR_SIZE) as *mut SdsHeader
}

pub const SDS_MAX_PREALLOC: ::core::ffi::c_int =
    1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
#[inline]
pub(crate) unsafe fn sdslen(s: SdsRaw) -> usize {
    (*sds_header(s)).len
}
#[inline]
unsafe fn sdsavail(s: SdsRaw) -> usize {
    let h = &*sds_header(s);
    h.cap - h.len
}
#[inline]
unsafe fn sdssetlen(s: SdsRaw, newlen: usize) {
    (*sds_header(s)).len = newlen;
}
#[inline]
unsafe fn sdsalloc(s: SdsRaw) -> usize {
    (*sds_header(s)).cap
}
#[inline]
unsafe fn sdssetalloc(s: SdsRaw, newlen: usize) {
    (*sds_header(s)).cap = newlen;
}
pub unsafe fn sdsnewlen(
    init: *const ::core::ffi::c_void,
    initlen: usize,
) -> SdsRaw {
    let block = malloc(SDS_HDR_SIZE.wrapping_add(initlen).wrapping_add(1 as usize));
    if block.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let h = block as *mut SdsHeader;
    (*h).len = initlen;
    (*h).cap = initlen;
    let s = (block as *mut u8).add(SDS_HDR_SIZE) as SdsRaw;
    if init.is_null() {
        memset(
            s as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            initlen.wrapping_add(1 as usize),
        );
    } else {
        if initlen != 0 as usize {
            memcpy(s as *mut ::core::ffi::c_void, init, initlen);
        }
        *s.offset(initlen as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    return s;
}
pub unsafe fn sdsempty() -> SdsRaw {
    return sdsnewlen(
        b"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        0 as usize,
    );
}
pub unsafe fn sdsnew(mut init: *const ::core::ffi::c_char) -> SdsRaw {
    let mut initlen: usize = if init.is_null() {
        0 as usize
    } else {
        strlen(init)
    };
    return sdsnewlen(init as *const ::core::ffi::c_void, initlen);
}
pub unsafe fn sdsdup(s: SdsRaw) -> SdsRaw {
    return sdsnewlen(s as *const ::core::ffi::c_void, sdslen(s));
}
pub unsafe fn sdsfree(s: SdsRaw) {
    if s.is_null() {
        return;
    }
    free(sds_header(s) as *mut ::core::ffi::c_void);
}
pub unsafe fn sdsupdatelen(mut s: SdsRaw) {
    let mut reallen: ::core::ffi::c_int =
        strlen(s as *const ::core::ffi::c_char) as ::core::ffi::c_int;
    sdssetlen(s, reallen as usize);
}
pub unsafe fn sdsclear(mut s: SdsRaw) {
    sdssetlen(s, 0 as usize);
    *s.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
}
pub unsafe fn sds_make_room_for(mut s: SdsRaw, mut addlen: usize) -> SdsRaw {
    let mut avail: usize = sdsavail(s);
    if avail >= addlen {
        return s;
    }
    let mut len: usize = sdslen(s);
    let mut newlen: usize = len.wrapping_add(addlen);
    if newlen < SDS_MAX_PREALLOC as usize {
        newlen = newlen.wrapping_mul(2 as usize);
    } else {
        newlen = newlen.wrapping_add(SDS_MAX_PREALLOC as usize);
    }
    let newsh = realloc(
        sds_header(s) as *mut ::core::ffi::c_void,
        SDS_HDR_SIZE.wrapping_add(newlen).wrapping_add(1 as usize),
    );
    if newsh.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    s = (newsh as *mut u8).add(SDS_HDR_SIZE) as SdsRaw;
    sdssetalloc(s, newlen);
    return s;
}
pub unsafe fn sds_remove_free_space(mut s: SdsRaw) -> SdsRaw {
    let mut len: usize = sdslen(s);
    let newsh = realloc(
        sds_header(s) as *mut ::core::ffi::c_void,
        SDS_HDR_SIZE.wrapping_add(len).wrapping_add(1 as usize),
    );
    if newsh.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    s = (newsh as *mut u8).add(SDS_HDR_SIZE) as SdsRaw;
    sdssetalloc(s, len);
    return s;
}
pub unsafe fn sdsgrowzero(mut s: SdsRaw, mut len: usize) -> SdsRaw {
    let mut curlen: usize = sdslen(s);
    if len <= curlen {
        return s;
    }
    s = sds_make_room_for(s, len.wrapping_sub(curlen));
    if s.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memset(
        s.offset(curlen as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        len.wrapping_sub(curlen).wrapping_add(1 as usize),
    );
    sdssetlen(s, len);
    return s;
}
pub unsafe fn sdscatlen(
    mut s: SdsRaw,
    mut t: *const ::core::ffi::c_void,
    mut len: usize,
) -> SdsRaw {
    let mut curlen: usize = sdslen(s);
    s = sds_make_room_for(s, len);
    if s.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    memcpy(
        s.offset(curlen as isize) as *mut ::core::ffi::c_void,
        t,
        len,
    );
    sdssetlen(s, curlen.wrapping_add(len));
    *s.offset(curlen.wrapping_add(len) as isize) = '\0' as i32 as ::core::ffi::c_char;
    return s;
}
pub unsafe fn sdscat(mut s: SdsRaw, mut t: *const ::core::ffi::c_char) -> SdsRaw {
    return sdscatlen(s, t as *const ::core::ffi::c_void, strlen(t));
}
pub unsafe fn sdscatsds(mut s: SdsRaw, t: SdsRaw) -> SdsRaw {
    return sdscatlen(s, t as *const ::core::ffi::c_void, sdslen(t));
}
pub unsafe fn sdscpylen(
    mut s: SdsRaw,
    mut t: *const ::core::ffi::c_char,
    mut len: usize,
) -> SdsRaw {
    if sdsalloc(s) < len {
        s = sds_make_room_for(s, len.wrapping_sub(sdslen(s)));
        if s.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    memcpy(
        s as *mut ::core::ffi::c_void,
        t as *const ::core::ffi::c_void,
        len,
    );
    *s.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    sdssetlen(s, len);
    return s;
}
pub unsafe fn sdscpy(mut s: SdsRaw, mut t: *const ::core::ffi::c_char) -> SdsRaw {
    return sdscpylen(s, t, strlen(t));
}
pub unsafe fn sdsll2str(
    mut s: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aux: ::core::ffi::c_char = 0;
    let mut v: ::core::ffi::c_ulonglong = 0;
    let mut l: usize = 0;
    v = (if value < 0 as ::core::ffi::c_longlong {
        -value
    } else {
        value
    }) as ::core::ffi::c_ulonglong;
    p = s;
    loop {
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = ('0' as i32 as ::core::ffi::c_ulonglong)
            .wrapping_add(v.wrapping_rem(10 as ::core::ffi::c_ulonglong))
            as ::core::ffi::c_char;
        v = v.wrapping_div(10 as ::core::ffi::c_ulonglong);
        if !(v != 0) {
            break;
        }
    }
    if value < 0 as ::core::ffi::c_longlong {
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = '-' as i32 as ::core::ffi::c_char;
    }
    l = p.offset_from(s) as ::core::ffi::c_long as usize;
    *p = '\0' as i32 as ::core::ffi::c_char;
    p = p.offset(-1);
    while s < p {
        aux = *s;
        *s = *p;
        *p = aux;
        s = s.offset(1);
        p = p.offset(-1);
    }
    return l as ::core::ffi::c_int;
}
pub unsafe fn sdsull2str(
    mut s: *mut ::core::ffi::c_char,
    mut v: ::core::ffi::c_ulonglong,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aux: ::core::ffi::c_char = 0;
    let mut l: usize = 0;
    p = s;
    loop {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = ('0' as i32 as ::core::ffi::c_ulonglong)
            .wrapping_add(v.wrapping_rem(10 as ::core::ffi::c_ulonglong))
            as ::core::ffi::c_char;
        v = v.wrapping_div(10 as ::core::ffi::c_ulonglong);
        if !(v != 0) {
            break;
        }
    }
    l = p.offset_from(s) as ::core::ffi::c_long as usize;
    *p = '\0' as i32 as ::core::ffi::c_char;
    p = p.offset(-1);
    while s < p {
        aux = *s;
        *s = *p;
        *p = aux;
        s = s.offset(1);
        p = p.offset(-1);
    }
    return l as ::core::ffi::c_int;
}
pub unsafe fn sdsfromlonglong(mut value: ::core::ffi::c_longlong) -> SdsRaw {
    let mut buf: [::core::ffi::c_char; 21] = [0; 21];
    let mut len: ::core::ffi::c_int = sdsll2str(&raw mut buf as *mut ::core::ffi::c_char, value);
    return sdsnewlen(
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as usize,
    );
}
// ---------------------------------------------------------------------------
// Building an SdsRaw from typed pieces
// ---------------------------------------------------------------------------

/// One piece of a string being assembled by [`sdsbuild!`].
///
/// This replaces `sdscatprintf`/`sdscatfmt`. Every one of their 252 call sites
/// passed a *literal* format string, so the format never had to be interpreted
/// at run time: the pieces can simply be appended, and the compiler can check
/// that each argument matches the conversion the C code asked for -- which a
/// `printf` cannot.
///
/// Text is appended **as bytes**, deliberately. The `%s` arguments here are C
/// strings that came out of a font file: glyph names, table strings, PostScript
/// names. Routing them through Rust's `format!` would mean a `CStr` -> `str`
/// conversion, and a glyph name that is not valid UTF-8 would then come out
/// different (`to_str` fails; `to_string_lossy` substitutes U+FFFD) -- a change
/// no test payload would catch. Integers *are* formatted with `format!`, where
/// the output is ASCII digits and no such hazard exists.
pub trait SdsPart {
    /// Append this piece to `s`, returning the (possibly reallocated) string.
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw;

    /// [`Self::append_to`]'s `Vec<u8>`-targeting sibling, for
    /// [`crate::bytesbuild!`] -- same per-type rendering rules, just
    /// writing into a growable owned buffer instead of an `SdsRaw`. Kept
    /// on the same trait (rather than a second one) so every `SdsPart`
    /// impl states both renderings side by side.
    unsafe fn append_to_vec(self, v: &mut Vec<u8>);
}

impl SdsPart for &[u8] {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        sdscatlen(s, self.as_ptr() as *const ::core::ffi::c_void, self.len())
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        v.extend_from_slice(self);
    }
}

impl<const N: usize> SdsPart for &[u8; N] {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        (&self[..]).append_to(s)
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        unsafe { (&self[..]).append_to_vec(v) };
    }
}

/// A `Handle`'s `name` (now `Vec<u8>`, previously a null-terminated
/// `sds`/`SdsRaw` fed through the `*const c_char` impl below): appends up
/// to the first embedded NUL, matching that impl's `%s`/`strlen` semantics
/// exactly, since every existing call site passing a `Handle.name` into
/// `sdsbuild!` relied on that truncation. An empty `Vec` (an unset
/// `Handle`) appends nothing -- the `*const c_char` impl's literal
/// `"(null)"` for a null pointer has no equivalent here (there's no
/// "null vs. empty" distinction left once the storage is owned), which
/// only changes warning-log wording for a handle with no name, never any
/// dumped/built output.
impl SdsPart for &Vec<u8> {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        match self.iter().position(|&b| b == 0) {
            Some(nul_pos) => (&self[..nul_pos]).append_to(s),
            None => (&self[..]).append_to(s),
        }
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        let bytes = match self.iter().position(|&b| b == 0) {
            Some(nul_pos) => &self[..nul_pos],
            None => &self[..],
        };
        unsafe { bytes.append_to_vec(v) };
    }
}

/// A C string (`%s`): the bytes up to the terminating NUL.
///
/// A null pointer appends `(null)`, which is what both glibc and Apple's libc
/// print for `%s`. The old code handed the pointer straight to `vsnprintf`, so
/// any call site that can pass null was already relying on that.
impl SdsPart for *const ::core::ffi::c_char {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        if self.is_null() {
            return b"(null)".append_to(s);
        }
        sdscatlen(s, self as *const ::core::ffi::c_void, strlen(self))
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        if self.is_null() {
            return unsafe { b"(null)".append_to_vec(v) };
        }
        let bytes = unsafe {
            ::core::slice::from_raw_parts(self as *const u8, strlen(self))
        };
        v.extend_from_slice(bytes);
    }
}

/// `SdsRaw` is `*mut c_char`, so this covers both a plain C string and an SdsRaw
/// passed to `%s` -- which, like C, measures it with `strlen` and therefore
/// stops at an embedded NUL. Use [`Sds`] for `%S`.
impl SdsPart for *mut ::core::ffi::c_char {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        (self as *const ::core::ffi::c_char).append_to(s)
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        unsafe { (self as *const ::core::ffi::c_char).append_to_vec(v) };
    }
}

/// A static C string (`%s`), for the label tables that reach the log and the
/// JSON output. Identical to the `*const c_char` impl above, minus the `strlen`
/// and the null check: a `CStr` carries its own length and cannot be null.
impl SdsPart for &::core::ffi::CStr {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        self.to_bytes().append_to(s)
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        unsafe { self.to_bytes().append_to_vec(v) };
    }
}

/// An SdsRaw appended by its stored length (`%S`), so unlike `%s` it keeps any
/// embedded NUL bytes.
pub struct Sds(pub SdsRaw);

impl SdsPart for Sds {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        sdscatsds(s, self.0)
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        let bytes = unsafe {
            ::core::slice::from_raw_parts(self.0 as *const u8, sdslen(self.0))
        };
        v.extend_from_slice(bytes);
    }
}

/// A single byte (`%c`).
///
/// C converts the argument to `unsigned char`, so this is one raw byte and
/// *not* a `char`: formatting a `char` would UTF-8 encode anything above 0x7f
/// into two bytes. `otl/read.rs` builds lookup names out of the four bytes of
/// an OpenType tag this way, and those names reach the JSON output.
pub struct Byte(pub u8);

impl SdsPart for Byte {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        sdscatlen(
            s,
            &self.0 as *const u8 as *const ::core::ffi::c_void,
            1 as usize,
        )
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        v.push(self.0);
    }
}

/// `%04x`
pub struct Hex4(pub u32);
/// `%04X`
pub struct Hex4Upper(pub u32);
/// `%02x`
pub struct Hex2(pub u32);
/// `%02X`
pub struct Hex2Upper(pub u32);
/// `%05d`
pub struct Dec5(pub ::core::ffi::c_int);

/// Append an ASCII rendering of an integer.
///
/// Safe to route through `format!` because the result is digits: see the note
/// on [`SdsPart`] for why text may not be.
unsafe fn cat_ascii(s: SdsRaw, digits: &str) -> SdsRaw {
    digits.as_bytes().append_to(s)
}

fn cat_ascii_vec(v: &mut Vec<u8>, digits: &str) {
    v.extend_from_slice(digits.as_bytes());
}

impl SdsPart for ::core::ffi::c_int {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{self}"))
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{self}"));
    }
}

impl SdsPart for ::core::ffi::c_uint {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{self}"))
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{self}"));
    }
}

impl SdsPart for Dec5 {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{:05}", self.0))
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{:05}", self.0));
    }
}

/// The `u32` casts at the call sites are not cosmetic: C's `%x` reads an
/// `unsigned int`, so a negative `int` argument prints as its 32-bit two's
/// complement -- eight digits, not four. `as u32` reproduces exactly that,
/// and widens a `u16` the same way C's default promotion does.
impl SdsPart for Hex4 {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{:04x}", self.0))
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{:04x}", self.0));
    }
}

impl SdsPart for Hex4Upper {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{:04X}", self.0))
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{:04X}", self.0));
    }
}

impl SdsPart for Hex2 {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{:02x}", self.0))
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{:02x}", self.0));
    }
}

impl SdsPart for Hex2Upper {
    unsafe fn append_to(self, s: SdsRaw) -> SdsRaw {
        cat_ascii(s, &format!("{:02X}", self.0))
    }
    unsafe fn append_to_vec(self, v: &mut Vec<u8>) {
        cat_ascii_vec(v, &format!("{:02X}", self.0));
    }
}

/// Append pieces to an SdsRaw, in order, and evaluate to the result.
///
/// ```ignore
/// sdscatprintf(sdsempty(), "lookup_%s_%02x_%d\0", name, kind, index)
/// ```
/// becomes
/// ```ignore
/// sdsbuild!(sdsempty(), b"lookup_", name, b"_", Hex2(kind as u32), b"_", index)
/// ```
///
/// Each piece is appended through [`SdsPart`], so its type decides how it is
/// rendered. Returns null if any reallocation fails, exactly as the `sdscat*`
/// functions do.
#[macro_export]
macro_rules! sdsbuild {
    ($base:expr $(, $part:expr)* $(,)?) => {{
        let mut __sds: $crate::vendor::sds::SdsRaw = $base;
        $(
            __sds = $crate::vendor::sds::SdsPart::append_to($part, __sds);
        )*
        __sds
    }};
}

/// [`sdsbuild!`]'s `Vec<u8>`-targeting sibling, for callers with nothing
/// left to `SdsRaw`-ify a result into (the `ILogger` vtable, once
/// retyped). No base/seed argument, unlike `sdsbuild!` -- `Vec::new()`
/// needs no allocator call the way `sdsempty()` does, so there is nothing
/// to thread through.
///
/// ```ignore
/// sdsbuild!(sdsempty(), b"lookup_", name, b"_", Hex2(kind as u32))
/// ```
/// becomes
/// ```ignore
/// bytesbuild!(b"lookup_", name, b"_", Hex2(kind as u32))
/// ```
#[macro_export]
macro_rules! bytesbuild {
    ($($part:expr),* $(,)?) => {{
        let mut __v: ::std::vec::Vec<u8> = ::std::vec::Vec::new();
        $(
            // Callers in the middle of the `unsafe_op_in_unsafe_fn` burn-down
            // increasingly wrap their whole function body in one `unsafe {}`
            // block rather than fine-grain every raw-pointer operation --
            // this macro's own `unsafe` then nests inside that caller block,
            // which `unused_unsafe` (correctly) flags as redundant. Silenced
            // here, once, rather than forcing every call site (this macro is
            // used at ~50+ of them) to restructure around it.
            #[allow(unused_unsafe)]
            unsafe { $crate::vendor::sds::SdsPart::append_to_vec($part, &mut __v); }
        )*
        __v
    }};
}

pub unsafe fn sdstrim(mut s: SdsRaw, mut cset: *const ::core::ffi::c_char) -> SdsRaw {
    let mut start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: usize = 0;
    start = s as *mut ::core::ffi::c_char;
    sp = start;
    end = s
        .offset(sdslen(s) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_char;
    ep = end;
    while sp <= end && !strchr(cset, *sp as ::core::ffi::c_int).is_null() {
        sp = sp.offset(1);
    }
    while ep > sp && !strchr(cset, *ep as ::core::ffi::c_int).is_null() {
        ep = ep.offset(-1);
    }
    len = (if sp > ep {
        0 as ::core::ffi::c_long
    } else {
        ep.offset_from(sp) as ::core::ffi::c_long + 1 as ::core::ffi::c_long
    }) as usize;
    if s != sp {
        memmove(
            s as *mut ::core::ffi::c_void,
            sp as *const ::core::ffi::c_void,
            len,
        );
    }
    *s.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    sdssetlen(s, len);
    return s;
}
pub unsafe fn sdsrange(
    mut s: SdsRaw,
    mut start: ::core::ffi::c_int,
    mut end: ::core::ffi::c_int,
) {
    let mut newlen: usize = 0;
    let mut len: usize = sdslen(s);
    if len == 0 as usize {
        return;
    }
    if start < 0 as ::core::ffi::c_int {
        start = len.wrapping_add(start as usize) as ::core::ffi::c_int;
        if start < 0 as ::core::ffi::c_int {
            start = 0 as ::core::ffi::c_int;
        }
    }
    if end < 0 as ::core::ffi::c_int {
        end = len.wrapping_add(end as usize) as ::core::ffi::c_int;
        if end < 0 as ::core::ffi::c_int {
            end = 0 as ::core::ffi::c_int;
        }
    }
    newlen = (if start > end {
        0 as ::core::ffi::c_int
    } else {
        end - start + 1 as ::core::ffi::c_int
    }) as usize;
    if newlen != 0 as usize {
        if start >= len as ::core::ffi::c_int {
            newlen = 0 as usize;
        } else if end >= len as ::core::ffi::c_int {
            end = len.wrapping_sub(1 as usize) as ::core::ffi::c_int;
            newlen = (if start > end {
                0 as ::core::ffi::c_int
            } else {
                end - start + 1 as ::core::ffi::c_int
            }) as usize;
        }
    } else {
        start = 0 as ::core::ffi::c_int;
    }
    if start != 0 && newlen != 0 {
        memmove(
            s as *mut ::core::ffi::c_void,
            s.offset(start as isize) as *const ::core::ffi::c_void,
            newlen,
        );
    }
    *s.offset(newlen as isize) = 0 as ::core::ffi::c_char;
    sdssetlen(s, newlen);
}
pub unsafe fn sdstolower(mut s: SdsRaw) {
    let mut len: ::core::ffi::c_int = sdslen(s) as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < len {
        *s.offset(j as isize) = (c_tolower(*s.offset(j as isize) as ::core::ffi::c_int)) as ::core::ffi::c_char;
        j += 1;
    }
}
pub unsafe fn sdstoupper(mut s: SdsRaw) {
    let mut len: ::core::ffi::c_int = sdslen(s) as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < len {
        *s.offset(j as isize) = (c_toupper(*s.offset(j as isize) as ::core::ffi::c_int)) as ::core::ffi::c_char;
        j += 1;
    }
}
pub unsafe fn sdscmp(s1: SdsRaw, s2: SdsRaw) -> ::core::ffi::c_int {
    let mut l1: usize = 0;
    let mut l2: usize = 0;
    let mut minlen: usize = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    l1 = sdslen(s1);
    l2 = sdslen(s2);
    minlen = if l1 < l2 { l1 } else { l2 };
    cmp = memcmp(
        s1 as *const ::core::ffi::c_void,
        s2 as *const ::core::ffi::c_void,
        minlen,
    );
    if cmp == 0 as ::core::ffi::c_int {
        return l1.wrapping_sub(l2) as ::core::ffi::c_int;
    }
    return cmp;
}
pub unsafe fn sdssplitlen(
    mut s: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut sep: *const ::core::ffi::c_char,
    mut seplen: ::core::ffi::c_int,
    mut count: *mut ::core::ffi::c_int,
) -> *mut SdsRaw {
    let mut current_block: u64;
    let mut elements: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut slots: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    let mut start: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0;
    let mut tokens: *mut SdsRaw = ::core::ptr::null_mut::<SdsRaw>();
    if seplen < 1 as ::core::ffi::c_int || len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<SdsRaw>();
    }
    tokens =
        malloc((::core::mem::size_of::<SdsRaw>() as usize).wrapping_mul(slots as usize)) as *mut SdsRaw;
    if tokens.is_null() {
        return ::core::ptr::null_mut::<SdsRaw>();
    }
    if len == 0 as ::core::ffi::c_int {
        *count = 0 as ::core::ffi::c_int;
        return tokens;
    }
    j = 0 as ::core::ffi::c_int;
    loop {
        if !(j < len - (seplen - 1 as ::core::ffi::c_int)) {
            current_block = 15904375183555213903;
            break;
        }
        if slots < elements + 2 as ::core::ffi::c_int {
            let mut newtokens: *mut SdsRaw = ::core::ptr::null_mut::<SdsRaw>();
            slots *= 2 as ::core::ffi::c_int;
            newtokens = realloc(
                tokens as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<SdsRaw>() as usize).wrapping_mul(slots as usize),
            ) as *mut SdsRaw;
            if newtokens.is_null() {
                current_block = 2896259319996730917;
                break;
            }
            tokens = newtokens;
        }
        if seplen == 1 as ::core::ffi::c_int
            && *s.offset(j as isize) as ::core::ffi::c_int
                == *sep.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            || memcmp(
                s.offset(j as isize) as *const ::core::ffi::c_void,
                sep as *const ::core::ffi::c_void,
                seplen as usize,
            ) == 0 as ::core::ffi::c_int
        {
            let fresh9 = &raw mut *tokens.offset(elements as isize);
            *fresh9 = sdsnewlen(
                s.offset(start as isize) as *const ::core::ffi::c_void,
                (j - start) as usize,
            );
            if (*tokens.offset(elements as isize)).is_null() {
                current_block = 2896259319996730917;
                break;
            }
            elements += 1;
            start = j + seplen;
            j = j + seplen - 1 as ::core::ffi::c_int;
        }
        j += 1;
    }
    match current_block {
        15904375183555213903 => {
            let fresh10 = &raw mut *tokens.offset(elements as isize);
            *fresh10 = sdsnewlen(
                s.offset(start as isize) as *const ::core::ffi::c_void,
                (len - start) as usize,
            );
            if !(*tokens.offset(elements as isize)).is_null() {
                elements += 1;
                *count = elements;
                return tokens;
            }
        }
        _ => {}
    }
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < elements {
        sdsfree(*tokens.offset(i as isize));
        i += 1;
    }
    free(tokens as *mut ::core::ffi::c_void);
    *count = 0 as ::core::ffi::c_int;
    return ::core::ptr::null_mut::<SdsRaw>();
}
pub unsafe fn sdsfreesplitres(mut tokens: *mut SdsRaw, mut count: ::core::ffi::c_int) {
    if tokens.is_null() {
        return;
    }
    loop {
        let fresh11 = count;
        count = count - 1;
        if !(fresh11 != 0) {
            break;
        }
        sdsfree(*tokens.offset(count as isize));
    }
    free(tokens as *mut ::core::ffi::c_void);
}
pub unsafe fn sdscatrepr(
    mut s: SdsRaw,
    mut p: *const ::core::ffi::c_char,
    mut len: usize,
) -> SdsRaw {
    s = sdscatlen(
        s,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as usize,
    );
    loop {
        let fresh12 = len;
        len = len.wrapping_sub(1);
        if !(fresh12 != 0) {
            break;
        }
        match *p as ::core::ffi::c_int {
            92 | 34 => {
                s = crate::sdsbuild!(s, b"\\", Byte((*p as ::core::ffi::c_int) as u8));
            }
            10 => {
                s = sdscatlen(
                    s,
                    b"\\n\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as usize,
                );
            }
            13 => {
                s = sdscatlen(
                    s,
                    b"\\r\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as usize,
                );
            }
            9 => {
                s = sdscatlen(
                    s,
                    b"\\t\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as usize,
                );
            }
            7 => {
                s = sdscatlen(
                    s,
                    b"\\a\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as usize,
                );
            }
            8 => {
                s = sdscatlen(
                    s,
                    b"\\b\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    2 as usize,
                );
            }
            _ => {
                if c_isprint(*p as ::core::ffi::c_int)
                {
                    s = crate::sdsbuild!(s, Byte((*p as ::core::ffi::c_int) as u8));
                } else {
                    s = crate::sdsbuild!(
                        s,
                        b"\\x",
                        Hex2((*p as ::core::ffi::c_uchar as ::core::ffi::c_int) as u32),
                    );
                }
            }
        }
        p = p.offset(1);
    }
    return sdscatlen(
        s,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as usize,
    );
}
pub unsafe fn is_hex_digit(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32
        || c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'f' as i32
        || c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32)
        as ::core::ffi::c_int;
}
pub unsafe fn hex_digit_to_int(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    match c as ::core::ffi::c_int {
        48 => return 0 as ::core::ffi::c_int,
        49 => return 1 as ::core::ffi::c_int,
        50 => return 2 as ::core::ffi::c_int,
        51 => return 3 as ::core::ffi::c_int,
        52 => return 4 as ::core::ffi::c_int,
        53 => return 5 as ::core::ffi::c_int,
        54 => return 6 as ::core::ffi::c_int,
        55 => return 7 as ::core::ffi::c_int,
        56 => return 8 as ::core::ffi::c_int,
        57 => return 9 as ::core::ffi::c_int,
        97 | 65 => return 10 as ::core::ffi::c_int,
        98 | 66 => return 11 as ::core::ffi::c_int,
        99 | 67 => return 12 as ::core::ffi::c_int,
        100 | 68 => return 13 as ::core::ffi::c_int,
        101 | 69 => return 14 as ::core::ffi::c_int,
        102 | 70 => return 15 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
pub unsafe fn sdssplitargs(
    mut line: *const ::core::ffi::c_char,
    mut argc: *mut ::core::ffi::c_int,
) -> *mut SdsRaw {
    let mut p: *const ::core::ffi::c_char = line;
    let mut current: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut vector: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    *argc = 0 as ::core::ffi::c_int;
    's_13: loop {
        while *p as ::core::ffi::c_int != 0
            && c_isspace(*p as ::core::ffi::c_int)
        {
            p = p.offset(1);
        }
        if *p != 0 {
            let mut inq: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut insq: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut done: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if current.is_null() {
                current = sdsempty() as *mut ::core::ffi::c_char;
            }
            while done == 0 {
                if inq != 0 {
                    if *p as ::core::ffi::c_int == '\\' as i32
                        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 'x' as i32
                        && is_hex_digit(*p.offset(2 as ::core::ffi::c_int as isize)) != 0
                        && is_hex_digit(*p.offset(3 as ::core::ffi::c_int as isize)) != 0
                    {
                        let mut byte: ::core::ffi::c_uchar = 0;
                        byte = (hex_digit_to_int(*p.offset(2 as ::core::ffi::c_int as isize))
                            * 16 as ::core::ffi::c_int
                            + hex_digit_to_int(*p.offset(3 as ::core::ffi::c_int as isize)))
                            as ::core::ffi::c_uchar;
                        current = sdscatlen(
                            current as SdsRaw,
                            &raw mut byte as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                            1 as usize,
                        ) as *mut ::core::ffi::c_char;
                        p = p.offset(3 as ::core::ffi::c_int as isize);
                    } else if *p as ::core::ffi::c_int == '\\' as i32
                        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
                    {
                        let mut c: ::core::ffi::c_char = 0;
                        p = p.offset(1);
                        match *p as ::core::ffi::c_int {
                            110 => {
                                c = '\n' as i32 as ::core::ffi::c_char;
                            }
                            114 => {
                                c = '\r' as i32 as ::core::ffi::c_char;
                            }
                            116 => {
                                c = '\t' as i32 as ::core::ffi::c_char;
                            }
                            98 => {
                                c = '\u{8}' as i32 as ::core::ffi::c_char;
                            }
                            97 => {
                                c = '\u{7}' as i32 as ::core::ffi::c_char;
                            }
                            _ => {
                                c = *p;
                            }
                        }
                        current = sdscatlen(
                            current as SdsRaw,
                            &raw mut c as *const ::core::ffi::c_void,
                            1 as usize,
                        ) as *mut ::core::ffi::c_char;
                    } else if *p as ::core::ffi::c_int == '"' as i32 {
                        if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
                            && !c_isspace(*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        {
                            break 's_13;
                        }
                        done = 1 as ::core::ffi::c_int;
                    } else if *p == 0 {
                        break 's_13;
                    } else {
                        current =
                            sdscatlen(current as SdsRaw, p as *const ::core::ffi::c_void, 1 as usize)
                                as *mut ::core::ffi::c_char;
                    }
                } else if insq != 0 {
                    if *p as ::core::ffi::c_int == '\\' as i32
                        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == '\'' as i32
                    {
                        p = p.offset(1);
                        current = sdscatlen(
                            current as SdsRaw,
                            b"'\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            1 as usize,
                        ) as *mut ::core::ffi::c_char;
                    } else if *p as ::core::ffi::c_int == '\'' as i32 {
                        if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
                            && !c_isspace(*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        {
                            break 's_13;
                        }
                        done = 1 as ::core::ffi::c_int;
                    } else {
                        if *p == 0 {
                            break 's_13;
                        }
                        current =
                            sdscatlen(current as SdsRaw, p as *const ::core::ffi::c_void, 1 as usize)
                                as *mut ::core::ffi::c_char;
                    }
                } else {
                    match *p as ::core::ffi::c_int {
                        32 | 10 | 13 | 9 | 0 => {
                            done = 1 as ::core::ffi::c_int;
                        }
                        34 => {
                            inq = 1 as ::core::ffi::c_int;
                        }
                        39 => {
                            insq = 1 as ::core::ffi::c_int;
                        }
                        _ => {
                            current = sdscatlen(
                                current as SdsRaw,
                                p as *const ::core::ffi::c_void,
                                1 as usize,
                            ) as *mut ::core::ffi::c_char;
                        }
                    }
                }
                if *p != 0 {
                    p = p.offset(1);
                }
            }
            vector = realloc(
                vector as *mut ::core::ffi::c_void,
                ((*argc + 1 as ::core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize),
            ) as *mut *mut ::core::ffi::c_char;
            let fresh13 = &raw mut *vector.offset(*argc as isize);
            *fresh13 = current;
            *argc += 1;
            current = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            if vector.is_null() {
                vector = malloc(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize)
                    as *mut *mut ::core::ffi::c_char;
            }
            return vector as *mut SdsRaw;
        }
    }
    loop {
        let fresh14 = *argc;
        *argc = *argc - 1;
        if !(fresh14 != 0) {
            break;
        }
        sdsfree(*vector.offset(*argc as isize) as SdsRaw);
    }
    free(vector as *mut ::core::ffi::c_void);
    if !current.is_null() {
        sdsfree(current as SdsRaw);
    }
    *argc = 0 as ::core::ffi::c_int;
    return ::core::ptr::null_mut::<SdsRaw>();
}
pub unsafe fn sdsmapchars(
    mut s: SdsRaw,
    mut from: *const ::core::ffi::c_char,
    mut to: *const ::core::ffi::c_char,
    mut setlen: usize,
) -> SdsRaw {
    let mut j: usize = 0;
    let mut i: usize = 0;
    let mut l: usize = sdslen(s);
    j = 0 as usize;
    while j < l {
        i = 0 as usize;
        while i < setlen {
            if *s.offset(j as isize) as ::core::ffi::c_int
                == *from.offset(i as isize) as ::core::ffi::c_int
            {
                *s.offset(j as isize) = *to.offset(i as isize);
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        j = j.wrapping_add(1);
    }
    return s;
}
pub unsafe fn sdsjoin(
    mut argv: *mut *mut ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut sep: *mut ::core::ffi::c_char,
) -> SdsRaw {
    let mut join: SdsRaw = sdsempty();
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < argc {
        join = sdscat(join, *argv.offset(j as isize));
        if j != argc - 1 as ::core::ffi::c_int {
            join = sdscat(join, sep);
        }
        j += 1;
    }
    return join;
}
pub unsafe fn sdsjoinsds(
    mut argv: *mut SdsRaw,
    mut argc: ::core::ffi::c_int,
    mut sep: *const ::core::ffi::c_char,
    mut seplen: usize,
) -> SdsRaw {
    let mut join: SdsRaw = sdsempty();
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    while j < argc {
        join = sdscatsds(join, *argv.offset(j as isize));
        if j != argc - 1 as ::core::ffi::c_int {
            join = sdscatlen(join, sep as *const ::core::ffi::c_void, seplen);
        }
        j += 1;
    }
    return join;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Covers the header-mechanics rewrite directly: growth, shrink, embedded
    // NUL survival, and repeated realloc all have to keep `len` and content
    // correct across the single-fixed-header representation.
    #[test]
    fn newlen_from_null_init_is_zero_filled() {
        unsafe {
            let s = sdsnewlen(::core::ptr::null(), 5);
            assert_eq!(sdslen(s), 5);
            let bytes = ::core::slice::from_raw_parts(s as *const u8, 5);
            assert_eq!(bytes, &[0u8; 5]);
            assert_eq!(*s.offset(5), 0); // null terminator
            sdsfree(s);
        }
    }

    #[test]
    fn cat_grows_and_keeps_embedded_nul() {
        unsafe {
            let mut s = sdsnewlen(b"ab\0cd".as_ptr() as *const _, 5);
            for _ in 0..20 {
                s = sdscatlen(s, b"xyz\0!!".as_ptr() as *const _, 6);
            }
            assert_eq!(sdslen(s), 5 + 20 * 6);
            let bytes = ::core::slice::from_raw_parts(s as *const u8, sdslen(s));
            assert_eq!(&bytes[0..5], b"ab\0cd");
            assert_eq!(&bytes[5..11], b"xyz\0!!");
            assert_eq!(*s.offset(sdslen(s) as isize), 0); // still null-terminated
            sdsfree(s);
        }
    }

    #[test]
    fn dup_is_independent_of_the_original() {
        unsafe {
            let s = sdsnewlen(b"hello".as_ptr() as *const _, 5);
            let d = sdsdup(s);
            assert_ne!(s, d);
            assert_eq!(sdslen(d), 5);
            let cat = sdscat(d, b"!\0".as_ptr() as *const ::core::ffi::c_char);
            assert_eq!(sdslen(s), 5); // original untouched by growing the dup
            sdsfree(s);
            sdsfree(cat);
        }
    }

    /// What C's `printf` makes of the same conversion and argument.
    ///
    /// The helpers are checked against the C library rather than against
    /// hand-written expectations: the whole point of them is to reproduce
    /// `sdscatprintf` byte for byte, and only libc can settle what that was.
    macro_rules! assert_matches_printf {
        ($fmt:expr, $c_arg:expr, $built:expr) => {{
            let mut expect = [0 as ::core::ffi::c_char; 64];
            let n = libc::snprintf(
                expect.as_mut_ptr(),
                expect.len(),
                concat!($fmt, "\0").as_ptr() as *const ::core::ffi::c_char,
                $c_arg,
            );
            assert!(n >= 0 && (n as usize) < expect.len(), "snprintf overflowed");
            let expect =
                ::core::slice::from_raw_parts(expect.as_ptr() as *const u8, n as usize).to_vec();
            let got = $built;
            let got_bytes = ::core::slice::from_raw_parts(got as *const u8, sdslen(got)).to_vec();
            sdsfree(got);
            assert_eq!(
                String::from_utf8_lossy(&got_bytes),
                String::from_utf8_lossy(&expect),
                "conversion {} disagrees with libc",
                $fmt
            );
        }};
    }

    #[test]
    #[cfg_attr(miri, ignore = "calls libc::snprintf via assert_matches_printf!, unsupported under Miri")]
    fn decimal_matches_printf() {
        unsafe {
            for v in [0, 1, -1, 42, -42, i32::MAX, i32::MIN] {
                assert_matches_printf!("%d", v, sdsbuild!(sdsempty(), v));
                assert_matches_printf!("%05d", v, sdsbuild!(sdsempty(), Dec5(v)));
            }
            for v in [0u32, 1, 65535, u32::MAX] {
                assert_matches_printf!("%u", v, sdsbuild!(sdsempty(), v));
            }
        }
    }

    // A negative `int` reaches `%x` as an `unsigned int`, so it prints eight
    // digits and not four. Casting to `u16` at the call site -- the obvious
    // reading of "%04x" -- would silently drop the top half.
    #[test]
    #[cfg_attr(miri, ignore = "calls libc::snprintf via assert_matches_printf!, unsupported under Miri")]
    fn hex_matches_printf_including_negatives() {
        unsafe {
            for v in [0i32, 1, 0x0a, 0xabcd, 0xfffff, -1, -32768] {
                assert_matches_printf!("%04x", v, sdsbuild!(sdsempty(), Hex4(v as u32)));
                assert_matches_printf!("%04X", v, sdsbuild!(sdsempty(), Hex4Upper(v as u32)));
                assert_matches_printf!("%02x", v, sdsbuild!(sdsempty(), Hex2(v as u32)));
                assert_matches_printf!("%02X", v, sdsbuild!(sdsempty(), Hex2Upper(v as u32)));
            }
        }
    }

    // `%c` is a byte, not a character: 0xe9 is one byte for C, and would be the
    // two bytes of U+00E9 if it went through Rust's `char` formatting.
    #[test]
    #[cfg_attr(miri, ignore = "calls libc::snprintf via assert_matches_printf!, unsupported under Miri")]
    fn byte_is_one_byte_not_a_char() {
        unsafe {
            for v in [b'A' as i32, 0, 0x7f, 0x80, 0xe9, 0xff] {
                assert_matches_printf!("%c", v, sdsbuild!(sdsempty(), Byte(v as u8)));
            }
            let got = sdsbuild!(sdsempty(), Byte(0xe9));
            assert_eq!(sdslen(got), 1);
            sdsfree(got);
            assert_eq!('\u{e9}'.to_string().len(), 2); // ...which this is not
        }
    }

    // The reason these helpers exist instead of `format!`: a glyph name that is
    // not valid UTF-8 has to survive unchanged. `to_string_lossy` would replace
    // the 0xe9 with U+FFFD and the font would come out with a different name.
    #[test]
    fn c_string_is_copied_as_bytes_even_when_not_utf8() {
        unsafe {
            let name = b"caf\xe9\0";
            let got = sdsbuild!(sdsempty(), name.as_ptr() as *const ::core::ffi::c_char);
            let bytes = ::core::slice::from_raw_parts(got as *const u8, sdslen(got));
            assert_eq!(bytes, b"caf\xe9");
            sdsfree(got);
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "calls libc::snprintf via assert_matches_printf!, unsupported under Miri")]
    fn null_c_string_prints_like_libc() {
        unsafe {
            assert_matches_printf!(
                "%s",
                ::core::ptr::null::<::core::ffi::c_char>(),
                sdsbuild!(sdsempty(), ::core::ptr::null::<::core::ffi::c_char>())
            );
        }
    }

    // `%s` stops at the NUL and `%S` does not -- the distinction sdscatfmt drew
    // by calling `strlen` for one and `sdslen` for the other.
    #[test]
    fn sds_part_keeps_embedded_nul_but_c_string_does_not() {
        unsafe {
            let s = sdsnewlen(b"ab\0cd".as_ptr() as *const ::core::ffi::c_void, 5 as usize);
            let by_len = sdsbuild!(sdsempty(), Sds(s));
            assert_eq!(sdslen(by_len), 5);
            let by_nul = sdsbuild!(sdsempty(), s);
            assert_eq!(sdslen(by_nul), 2);
            sdsfree(by_len);
            sdsfree(by_nul);
            sdsfree(s);
        }
    }

    #[test]
    fn pieces_are_appended_in_order() {
        unsafe {
            let got = sdsbuild!(
                sdsempty(),
                b"lookup_",
                b"ccmp\0".as_ptr() as *const ::core::ffi::c_char,
                b"_",
                Hex2(0x1f),
                b"_",
                7 as ::core::ffi::c_int,
            );
            let bytes = ::core::slice::from_raw_parts(got as *const u8, sdslen(got));
            assert_eq!(bytes, b"lookup_ccmp_1f_7");
            sdsfree(got);
        }
    }
}

