extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}

pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut ::core::ffi::c_void,
    pub __gr_top: *mut ::core::ffi::c_void,
    pub __vr_top: *mut ::core::ffi::c_void,
    pub __gr_offs: ::core::ffi::c_int,
    pub __vr_offs: ::core::ffi::c_int,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type va_list = __gnuc_va_list;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sds = *mut ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: ::core::ffi::c_uchar,
    pub buf: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
use crate::src::lib::support::stdio::{FILE, stderr};
use crate::src::lib::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SDS_TYPE_5: ::core::ffi::c_int = 0;
pub const SDS_TYPE_8: ::core::ffi::c_int = 1;
pub const SDS_TYPE_16: ::core::ffi::c_int = 2;
pub const SDS_TYPE_32: ::core::ffi::c_int = 3;
pub const SDS_TYPE_64: ::core::ffi::c_int = 4;
pub const SDS_TYPE_MASK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SDS_TYPE_BITS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as size_t,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as size_t;
        }
        _ => {}
    }
    return 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn bufnew() -> *mut caryll_Buffer {
    let mut buf: *mut caryll_Buffer = ::core::ptr::null_mut::<caryll_Buffer>();
    buf = __caryll_allocate_clean(
        ::core::mem::size_of::<caryll_Buffer>() as size_t,
        6 as ::core::ffi::c_ulong,
    ) as *mut caryll_Buffer;
    (*buf).free = 0 as size_t;
    (*buf).size = (*buf).free;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn buffree(mut buf: *mut caryll_Buffer) {
    if buf.is_null() {
        return;
    }
    if !(*buf).data.is_null() {
        free((*buf).data as *mut ::core::ffi::c_void);
        (*buf).data = ::core::ptr::null_mut::<uint8_t>();
    }
    free(buf as *mut ::core::ffi::c_void);
    buf = ::core::ptr::null_mut::<caryll_Buffer>();
}
#[no_mangle]
pub unsafe extern "C" fn buflen(mut buf: *mut caryll_Buffer) -> size_t {
    return (*buf).size;
}
#[no_mangle]
pub unsafe extern "C" fn bufpos(mut buf: *mut caryll_Buffer) -> size_t {
    return (*buf).cursor;
}
#[no_mangle]
pub unsafe extern "C" fn bufseek(mut buf: *mut caryll_Buffer, mut pos: size_t) {
    (*buf).cursor = pos;
}
#[no_mangle]
pub unsafe extern "C" fn bufclear(mut buf: *mut caryll_Buffer) {
    (*buf).cursor = 0 as size_t;
    (*buf).free = (*buf).size.wrapping_add((*buf).free);
    (*buf).size = 0 as size_t;
}
unsafe extern "C" fn bufbeforewrite(mut buf: *mut caryll_Buffer, mut towrite: size_t) {
    let mut currentSize: size_t = (*buf).size;
    let mut allocated: size_t = (*buf).size.wrapping_add((*buf).free);
    let mut required: size_t = (*buf).cursor.wrapping_add(towrite);
    if required < currentSize {
        return;
    } else if required <= allocated {
        (*buf).size = required;
        (*buf).free = allocated.wrapping_sub((*buf).size);
    } else {
        (*buf).size = required;
        (*buf).free = required;
        if (*buf).free > 0x1000000 as ::core::ffi::c_int as size_t {
            (*buf).free = 0x1000000 as ::core::ffi::c_int as size_t;
        }
        (*buf).data = __caryll_reallocate(
            (*buf).data as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<uint8_t>() as size_t)
                .wrapping_mul((*buf).size.wrapping_add((*buf).free)),
            46 as ::core::ffi::c_ulong,
        ) as *mut uint8_t;
    };
}
// Pushes `bytes` at the cursor, growing the buffer first, and advances the
// cursor. All the fixed-width bufwriteNN{l,b} functions below are exactly
// this plus an endian-ordered byte array (to_le_bytes/to_be_bytes), which
// replaces c2rust's manual per-byte shift-mask-store expansion.
#[inline]
unsafe fn buf_push_bytes(buf: *mut caryll_Buffer, bytes: &[u8]) {
    bufbeforewrite(buf, bytes.len());
    let dst = (*buf).data.add((*buf).cursor);
    ::core::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
    (*buf).cursor = (*buf).cursor.wrapping_add(bytes.len());
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite8(buf: *mut caryll_Buffer, byte: uint8_t) {
    buf_push_bytes(buf, &[byte]);
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite16l(buf: *mut caryll_Buffer, x: uint16_t) {
    buf_push_bytes(buf, &x.to_le_bytes());
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite16b(buf: *mut caryll_Buffer, x: uint16_t) {
    buf_push_bytes(buf, &x.to_be_bytes());
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite24l(buf: *mut caryll_Buffer, x: uint32_t) {
    // Low 3 bytes only, matching the original's shift-mask expansion, which
    // never touched bits 24-31 either.
    buf_push_bytes(buf, &x.to_le_bytes()[..3]);
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite24b(buf: *mut caryll_Buffer, x: uint32_t) {
    buf_push_bytes(buf, &x.to_be_bytes()[1..]);
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite32l(buf: *mut caryll_Buffer, x: uint32_t) {
    buf_push_bytes(buf, &x.to_le_bytes());
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite32b(buf: *mut caryll_Buffer, x: uint32_t) {
    buf_push_bytes(buf, &x.to_be_bytes());
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite64l(buf: *mut caryll_Buffer, x: uint64_t) {
    buf_push_bytes(buf, &x.to_le_bytes());
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite64b(buf: *mut caryll_Buffer, x: uint64_t) {
    buf_push_bytes(buf, &x.to_be_bytes());
}
#[no_mangle]
pub unsafe extern "C" fn bufninit(n: uint32_t, mut args: ...) -> *mut caryll_Buffer {
    let buf: *mut caryll_Buffer = bufnew();
    bufbeforewrite(buf, n as size_t);
    let mut ap: ::core::ffi::VaListImpl = args.clone();
    for _ in 0..n {
        bufwrite8(buf, ap.arg::<::core::ffi::c_int>() as uint8_t);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn bufnwrite8(buf: *mut caryll_Buffer, n: uint32_t, mut args: ...) {
    bufbeforewrite(buf, n as size_t);
    let mut ap: ::core::ffi::VaListImpl = args.clone();
    for _ in 0..n {
        bufwrite8(buf, ap.arg::<::core::ffi::c_int>() as uint8_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite_sds(buf: *mut caryll_Buffer, str: sds) {
    if str.is_null() {
        return;
    }
    let len: size_t = sdslen(str);
    if len == 0 {
        return;
    }
    buf_push_bytes(buf, ::core::slice::from_raw_parts(str as *const u8, len));
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite_str(buf: *mut caryll_Buffer, str: *const ::core::ffi::c_char) {
    if str.is_null() {
        return;
    }
    let len: size_t = strlen(str);
    if len == 0 {
        return;
    }
    buf_push_bytes(buf, ::core::slice::from_raw_parts(str as *const u8, len));
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite_bytes(buf: *mut caryll_Buffer, len: size_t, str: *const uint8_t) {
    if str.is_null() || len == 0 {
        return;
    }
    buf_push_bytes(buf, ::core::slice::from_raw_parts(str, len));
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite_buf(buf: *mut caryll_Buffer, that: *mut caryll_Buffer) {
    if that.is_null() || (*that).data.is_null() {
        return;
    }
    buf_push_bytes(buf, ::core::slice::from_raw_parts((*that).data, buflen(that)));
}
#[no_mangle]
pub unsafe extern "C" fn bufwrite_bufdel(buf: *mut caryll_Buffer, that: *mut caryll_Buffer) {
    if that.is_null() {
        return;
    }
    if (*that).data.is_null() {
        buffree(that);
        return;
    }
    buf_push_bytes(buf, ::core::slice::from_raw_parts((*that).data, buflen(that)));
    buffree(that);
}
#[no_mangle]
pub unsafe extern "C" fn buflongalign(buf: *mut caryll_Buffer) {
    let cp: size_t = (*buf).cursor;
    bufseek(buf, buflen(buf));
    let padding = buflen(buf).wrapping_rem(4);
    if (1..4).contains(&padding) {
        for _ in padding..4 {
            bufwrite8(buf, 0);
        }
    }
    bufseek(buf, cp);
}
#[no_mangle]
pub unsafe extern "C" fn bufping16b(buf: *mut caryll_Buffer, offset: *mut size_t, cp: *mut size_t) {
    bufwrite16b(buf, *offset as uint16_t);
    *cp = (*buf).cursor;
    bufseek(buf, *offset);
}
#[no_mangle]
pub unsafe extern "C" fn bufping16bd(
    buf: *mut caryll_Buffer,
    offset: *mut size_t,
    shift: *mut size_t,
    cp: *mut size_t,
) {
    bufwrite16b(buf, (*offset).wrapping_sub(*shift) as uint16_t);
    *cp = (*buf).cursor;
    bufseek(buf, *offset);
}
#[no_mangle]
pub unsafe extern "C" fn bufpong(buf: *mut caryll_Buffer, offset: *mut size_t, cp: *mut size_t) {
    *offset = (*buf).cursor;
    bufseek(buf, *cp);
}
#[no_mangle]
pub unsafe extern "C" fn bufpingpong16b(
    buf: *mut caryll_Buffer,
    that: *mut caryll_Buffer,
    offset: *mut size_t,
    cp: *mut size_t,
) {
    bufwrite16b(buf, *offset as uint16_t);
    *cp = (*buf).cursor;
    bufseek(buf, *offset);
    bufwrite_bufdel(buf, that);
    *offset = (*buf).cursor;
    bufseek(buf, *cp);
}
#[no_mangle]
pub unsafe extern "C" fn bufprint(buf: *mut caryll_Buffer) {
    for j in 0..(*buf).size {
        if j % 16 != 0 {
            fprintf(stderr, b" \0" as *const u8 as *const ::core::ffi::c_char);
        }
        fprintf(
            stderr,
            b"%02X\0" as *const u8 as *const ::core::ffi::c_char,
            *(*buf).data.add(j) as ::core::ffi::c_int,
        );
        if j % 16 == 15 {
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
}
