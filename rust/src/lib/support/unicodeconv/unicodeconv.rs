extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: size_t) -> sds;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub unsafe extern "C" fn utf16le_to_utf8(
    mut inb: *const uint8_t,
    mut inlenb: ::core::ffi::c_int,
) -> sds {
    let mut in_0: *mut uint16_t = inb as *mut uint16_t;
    let mut inend: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut inlen: uint32_t = 0;
    let mut bits: ::core::ffi::c_int = 0;
    if inlenb % 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        inlenb -= 1;
    }
    inlen = (inlenb / 2 as ::core::ffi::c_int) as uint32_t;
    inend = in_0.offset(inlen as isize);
    let mut bytesNeeded: uint32_t = 0 as uint32_t;
    while in_0 < inend {
        let fresh0 = in_0;
        in_0 = in_0.offset(1);
        c = *fresh0 as uint32_t;
        if c & 0xfc00 as uint32_t == 0xd800 as uint32_t {
            if in_0 >= inend {
                break;
            }
            let fresh1 = in_0;
            in_0 = in_0.offset(1);
            d = *fresh1 as uint32_t;
            if d & 0xfc00 as uint32_t == 0xdc00 as uint32_t {
                c &= 0x3ff as uint32_t;
                c <<= 10 as ::core::ffi::c_int;
                c |= d & 0x3ff as uint32_t;
                c = c.wrapping_add(0x10000 as uint32_t);
            }
        }
        if c < 0x80 as uint32_t {
            bytesNeeded = bytesNeeded.wrapping_add(1 as uint32_t);
        } else if c < 0x800 as uint32_t {
            bytesNeeded = bytesNeeded.wrapping_add(2 as uint32_t);
        } else if c < 0x10000 as uint32_t {
            bytesNeeded = bytesNeeded.wrapping_add(3 as uint32_t);
        } else {
            bytesNeeded = bytesNeeded.wrapping_add(4 as uint32_t);
        }
    }
    in_0 = inb as *mut uint16_t;
    let mut out: sds = sdsnewlen(
        ::core::ptr::null::<::core::ffi::c_void>(),
        bytesNeeded as size_t,
    );
    let mut out0: sds = out;
    while in_0 < inend {
        let fresh2 = in_0;
        in_0 = in_0.offset(1);
        c = *fresh2 as uint32_t;
        if c & 0xfc00 as uint32_t == 0xd800 as uint32_t {
            if in_0 >= inend {
                break;
            }
            let fresh3 = in_0;
            in_0 = in_0.offset(1);
            d = *fresh3 as uint32_t;
            if d & 0xfc00 as uint32_t == 0xdc00 as uint32_t {
                c &= 0x3ff as uint32_t;
                c <<= 10 as ::core::ffi::c_int;
                c |= d & 0x3ff as uint32_t;
                c = c.wrapping_add(0x10000 as uint32_t);
            }
        }
        if c < 0x80 as uint32_t {
            let fresh4 = out;
            out = out.offset(1);
            *fresh4 = c as ::core::ffi::c_char;
            bits = -(6 as ::core::ffi::c_int);
        } else if c < 0x800 as uint32_t {
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 = (c >> 6 as ::core::ffi::c_int & 0x1f as uint32_t | 0xc0 as uint32_t)
                as ::core::ffi::c_char;
            bits = 0 as ::core::ffi::c_int;
        } else if c < 0x10000 as uint32_t {
            let fresh6 = out;
            out = out.offset(1);
            *fresh6 = (c >> 12 as ::core::ffi::c_int & 0xf as uint32_t | 0xe0 as uint32_t)
                as ::core::ffi::c_char;
            bits = 6 as ::core::ffi::c_int;
        } else {
            let fresh7 = out;
            out = out.offset(1);
            *fresh7 = (c >> 18 as ::core::ffi::c_int & 0x7 as uint32_t | 0xf0 as uint32_t)
                as ::core::ffi::c_char;
            bits = 12 as ::core::ffi::c_int;
        }
        while bits >= 0 as ::core::ffi::c_int {
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 = (c >> bits & 0x3f as uint32_t | 0x80 as uint32_t) as ::core::ffi::c_char;
            bits -= 6 as ::core::ffi::c_int;
        }
    }
    return out0;
}
#[no_mangle]
pub unsafe extern "C" fn utf16be_to_utf8(
    mut inb: *const uint8_t,
    mut inlenb: ::core::ffi::c_int,
) -> sds {
    let mut in_0: *mut uint16_t = inb as *mut uint16_t;
    let mut inend: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut inlen: uint32_t = 0;
    let mut tmp: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut bits: ::core::ffi::c_int = 0;
    if inlenb % 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        inlenb -= 1;
    }
    inlen = (inlenb / 2 as ::core::ffi::c_int) as uint32_t;
    inend = in_0.offset(inlen as isize);
    let mut bytesNeeded: uint32_t = 0 as uint32_t;
    while in_0 < inend {
        tmp = in_0 as *mut uint8_t;
        let fresh9 = tmp;
        tmp = tmp.offset(1);
        c = *fresh9 as uint32_t;
        c = c << 8 as ::core::ffi::c_int;
        c = c | *tmp as uint32_t;
        in_0 = in_0.offset(1);
        if c & 0xfc00 as uint32_t == 0xd800 as uint32_t {
            if in_0 >= inend {
                break;
            }
            tmp = in_0 as *mut uint8_t;
            let fresh10 = tmp;
            tmp = tmp.offset(1);
            d = *fresh10 as uint32_t;
            d = d << 8 as ::core::ffi::c_int;
            d = d | *tmp as uint32_t;
            in_0 = in_0.offset(1);
            if d & 0xfc00 as uint32_t == 0xdc00 as uint32_t {
                c &= 0x3ff as uint32_t;
                c <<= 10 as ::core::ffi::c_int;
                c |= d & 0x3ff as uint32_t;
                c = c.wrapping_add(0x10000 as uint32_t);
            }
        }
        if c < 0x80 as uint32_t {
            bytesNeeded = bytesNeeded.wrapping_add(1 as uint32_t);
        } else if c < 0x800 as uint32_t {
            bytesNeeded = bytesNeeded.wrapping_add(2 as uint32_t);
        } else if c < 0x10000 as uint32_t {
            bytesNeeded = bytesNeeded.wrapping_add(3 as uint32_t);
        } else {
            bytesNeeded = bytesNeeded.wrapping_add(4 as uint32_t);
        }
    }
    in_0 = inb as *mut uint16_t;
    let mut out: sds = sdsnewlen(
        ::core::ptr::null::<::core::ffi::c_void>(),
        bytesNeeded as size_t,
    );
    let mut out0: sds = out;
    while in_0 < inend {
        tmp = in_0 as *mut uint8_t;
        let fresh11 = tmp;
        tmp = tmp.offset(1);
        c = *fresh11 as uint32_t;
        c = c << 8 as ::core::ffi::c_int;
        c = c | *tmp as uint32_t;
        in_0 = in_0.offset(1);
        if c & 0xfc00 as uint32_t == 0xd800 as uint32_t {
            if in_0 >= inend {
                break;
            }
            tmp = in_0 as *mut uint8_t;
            let fresh12 = tmp;
            tmp = tmp.offset(1);
            d = *fresh12 as uint32_t;
            d = d << 8 as ::core::ffi::c_int;
            d = d | *tmp as uint32_t;
            in_0 = in_0.offset(1);
            if d & 0xfc00 as uint32_t == 0xdc00 as uint32_t {
                c &= 0x3ff as uint32_t;
                c <<= 10 as ::core::ffi::c_int;
                c |= d & 0x3ff as uint32_t;
                c = c.wrapping_add(0x10000 as uint32_t);
            }
        }
        if c < 0x80 as uint32_t {
            let fresh13 = out;
            out = out.offset(1);
            *fresh13 = c as ::core::ffi::c_char;
            bits = -(6 as ::core::ffi::c_int);
        } else if c < 0x800 as uint32_t {
            let fresh14 = out;
            out = out.offset(1);
            *fresh14 = (c >> 6 as ::core::ffi::c_int & 0x1f as uint32_t | 0xc0 as uint32_t)
                as ::core::ffi::c_char;
            bits = 0 as ::core::ffi::c_int;
        } else if c < 0x10000 as uint32_t {
            let fresh15 = out;
            out = out.offset(1);
            *fresh15 = (c >> 12 as ::core::ffi::c_int & 0xf as uint32_t | 0xe0 as uint32_t)
                as ::core::ffi::c_char;
            bits = 6 as ::core::ffi::c_int;
        } else {
            let fresh16 = out;
            out = out.offset(1);
            *fresh16 = (c >> 18 as ::core::ffi::c_int & 0x7 as uint32_t | 0xf0 as uint32_t)
                as ::core::ffi::c_char;
            bits = 12 as ::core::ffi::c_int;
        }
        while bits >= 0 as ::core::ffi::c_int {
            let fresh17 = out;
            out = out.offset(1);
            *fresh17 = (c >> bits & 0x3f as uint32_t | 0x80 as uint32_t) as ::core::ffi::c_char;
            bits -= 6 as ::core::ffi::c_int;
        }
    }
    return out0;
}
#[no_mangle]
pub unsafe extern "C" fn utf8toutf16be(mut _in: sds, mut out_bytes: *mut size_t) -> *mut uint8_t {
    if _in.is_null() {
        *out_bytes = 0 as size_t;
        return ::core::ptr::null_mut::<uint8_t>();
    }
    let mut in_0: sds = _in;
    let mut inlen: size_t = sdslen(in_0);
    let mut inend: *mut ::core::ffi::c_char = in_0.offset(inlen as isize);
    let mut wordsNeeded: uint32_t = 0 as uint32_t;
    let mut trailing: uint8_t = 0 as uint8_t;
    let mut c: uint32_t = 0 as uint32_t;
    while in_0 < inend {
        let fresh18 = in_0;
        in_0 = in_0.offset(1);
        let mut d: uint8_t = *fresh18 as uint8_t;
        if (d as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
            c = d as uint32_t;
            trailing = 0 as uint8_t;
        } else {
            if (d as ::core::ffi::c_int) < 0xc0 as ::core::ffi::c_int {
                break;
            }
            if (d as ::core::ffi::c_int) < 0xe0 as ::core::ffi::c_int {
                c = (d as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as uint32_t;
                trailing = 1 as uint8_t;
            } else if (d as ::core::ffi::c_int) < 0xf0 as ::core::ffi::c_int {
                c = (d as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as uint32_t;
                trailing = 2 as uint8_t;
            } else {
                if !((d as ::core::ffi::c_int) < 0xf8 as ::core::ffi::c_int) {
                    break;
                }
                c = (d as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as uint32_t;
                trailing = 3 as uint8_t;
            }
        }
        if (inend.offset_from(in_0) as ::core::ffi::c_long) < trailing as ::core::ffi::c_long {
            break;
        }
        while trailing != 0 {
            if in_0 >= inend || {
                let fresh19 = in_0;
                in_0 = in_0.offset(1);
                d = *fresh19 as uint8_t;
                d as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int != 0x80 as ::core::ffi::c_int
            } {
                break;
            }
            c <<= 6 as ::core::ffi::c_int;
            c |= (d as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as uint32_t;
            trailing = trailing.wrapping_sub(1);
        }
        if c < 0x10000 as uint32_t {
            wordsNeeded = wordsNeeded.wrapping_add(1 as uint32_t);
        } else if c < 0x110000 as uint32_t {
            wordsNeeded = wordsNeeded.wrapping_add(2 as uint32_t);
        }
    }
    let mut _out: *mut uint8_t = malloc(
        ((2 as uint32_t).wrapping_mul(wordsNeeded) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    let mut out: *mut uint8_t = _out;
    in_0 = _in;
    while in_0 < inend {
        let fresh20 = in_0;
        in_0 = in_0.offset(1);
        let mut d_0: uint8_t = *fresh20 as uint8_t;
        if (d_0 as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
            c = d_0 as uint32_t;
            trailing = 0 as uint8_t;
        } else {
            if (d_0 as ::core::ffi::c_int) < 0xc0 as ::core::ffi::c_int {
                break;
            }
            if (d_0 as ::core::ffi::c_int) < 0xe0 as ::core::ffi::c_int {
                c = (d_0 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as uint32_t;
                trailing = 1 as uint8_t;
            } else if (d_0 as ::core::ffi::c_int) < 0xf0 as ::core::ffi::c_int {
                c = (d_0 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as uint32_t;
                trailing = 2 as uint8_t;
            } else {
                if !((d_0 as ::core::ffi::c_int) < 0xf8 as ::core::ffi::c_int) {
                    break;
                }
                c = (d_0 as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as uint32_t;
                trailing = 3 as uint8_t;
            }
        }
        if (inend.offset_from(in_0) as ::core::ffi::c_long) < trailing as ::core::ffi::c_long {
            break;
        }
        while trailing != 0 {
            if in_0 >= inend || {
                let fresh21 = in_0;
                in_0 = in_0.offset(1);
                d_0 = *fresh21 as uint8_t;
                d_0 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int != 0x80 as ::core::ffi::c_int
            } {
                break;
            }
            c <<= 6 as ::core::ffi::c_int;
            c |= (d_0 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as uint32_t;
            trailing = trailing.wrapping_sub(1);
        }
        if c < 0x10000 as uint32_t {
            let fresh22 = out;
            out = out.offset(1);
            *fresh22 = (c >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
            let fresh23 = out;
            out = out.offset(1);
            *fresh23 = (c & 0xff as uint32_t) as uint8_t;
        } else if c < 0x110000 as uint32_t {
            let mut tmp1: uint16_t =
                (0xd800 as uint32_t | c >> 10 as ::core::ffi::c_int) as uint16_t;
            let fresh24 = out;
            out = out.offset(1);
            *fresh24 = (tmp1 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as uint8_t;
            let fresh25 = out;
            out = out.offset(1);
            *fresh25 = (tmp1 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
            let mut tmp2: uint16_t = (0xdc00 as uint32_t | c & 0x3ff as uint32_t) as uint16_t;
            let fresh26 = out;
            out = out.offset(1);
            *fresh26 = (tmp2 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as uint8_t;
            let fresh27 = out;
            out = out.offset(1);
            *fresh27 = (tmp2 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        }
    }
    *out_bytes = wordsNeeded.wrapping_mul(2 as uint32_t) as size_t;
    return _out;
}
