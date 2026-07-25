#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{malloc};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, sds, sdshdr16, sdshdr32, sdshdr64, sdshdr8};

unsafe extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr8>() as isize))
                as *mut sdshdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr16>() as isize))
                as *mut sdshdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr32>() as isize))
                as *mut sdshdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<sdshdr64>() as isize))
                as *mut sdshdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn utf16le_to_utf8(
    mut inb: *const u8,
    mut inlenb: ::core::ffi::c_int,
) -> sds {
    let mut in_0: *mut u16 = inb as *mut u16;
    let mut inend: *mut u16 = ::core::ptr::null_mut::<u16>();
    let mut c: u32 = 0;
    let mut d: u32 = 0;
    let mut inlen: u32 = 0;
    let mut bits: ::core::ffi::c_int = 0;
    if inlenb % 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        inlenb -= 1;
    }
    inlen = (inlenb / 2 as ::core::ffi::c_int) as u32;
    inend = in_0.offset(inlen as isize);
    let mut bytesNeeded: u32 = 0 as u32;
    while in_0 < inend {
        let fresh0 = in_0;
        in_0 = in_0.offset(1);
        c = *fresh0 as u32;
        if c & 0xfc00 as u32 == 0xd800 as u32 {
            if in_0 >= inend {
                break;
            }
            let fresh1 = in_0;
            in_0 = in_0.offset(1);
            d = *fresh1 as u32;
            if d & 0xfc00 as u32 == 0xdc00 as u32 {
                c &= 0x3ff as u32;
                c <<= 10 as ::core::ffi::c_int;
                c |= d & 0x3ff as u32;
                c = c.wrapping_add(0x10000 as u32);
            }
        }
        if c < 0x80 as u32 {
            bytesNeeded = bytesNeeded.wrapping_add(1 as u32);
        } else if c < 0x800 as u32 {
            bytesNeeded = bytesNeeded.wrapping_add(2 as u32);
        } else if c < 0x10000 as u32 {
            bytesNeeded = bytesNeeded.wrapping_add(3 as u32);
        } else {
            bytesNeeded = bytesNeeded.wrapping_add(4 as u32);
        }
    }
    in_0 = inb as *mut u16;
    let mut out: sds = sdsnewlen(
        ::core::ptr::null::<::core::ffi::c_void>(),
        bytesNeeded as usize,
    );
    let mut out0: sds = out;
    while in_0 < inend {
        let fresh2 = in_0;
        in_0 = in_0.offset(1);
        c = *fresh2 as u32;
        if c & 0xfc00 as u32 == 0xd800 as u32 {
            if in_0 >= inend {
                break;
            }
            let fresh3 = in_0;
            in_0 = in_0.offset(1);
            d = *fresh3 as u32;
            if d & 0xfc00 as u32 == 0xdc00 as u32 {
                c &= 0x3ff as u32;
                c <<= 10 as ::core::ffi::c_int;
                c |= d & 0x3ff as u32;
                c = c.wrapping_add(0x10000 as u32);
            }
        }
        if c < 0x80 as u32 {
            let fresh4 = out;
            out = out.offset(1);
            *fresh4 = c as ::core::ffi::c_char;
            bits = -(6 as ::core::ffi::c_int);
        } else if c < 0x800 as u32 {
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 = (c >> 6 as ::core::ffi::c_int & 0x1f as u32 | 0xc0 as u32)
                as ::core::ffi::c_char;
            bits = 0 as ::core::ffi::c_int;
        } else if c < 0x10000 as u32 {
            let fresh6 = out;
            out = out.offset(1);
            *fresh6 = (c >> 12 as ::core::ffi::c_int & 0xf as u32 | 0xe0 as u32)
                as ::core::ffi::c_char;
            bits = 6 as ::core::ffi::c_int;
        } else {
            let fresh7 = out;
            out = out.offset(1);
            *fresh7 = (c >> 18 as ::core::ffi::c_int & 0x7 as u32 | 0xf0 as u32)
                as ::core::ffi::c_char;
            bits = 12 as ::core::ffi::c_int;
        }
        while bits >= 0 as ::core::ffi::c_int {
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 = (c >> bits & 0x3f as u32 | 0x80 as u32) as ::core::ffi::c_char;
            bits -= 6 as ::core::ffi::c_int;
        }
    }
    return out0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn utf16be_to_utf8(
    mut inb: *const u8,
    mut inlenb: ::core::ffi::c_int,
) -> sds {
    let mut in_0: *mut u16 = inb as *mut u16;
    let mut inend: *mut u16 = ::core::ptr::null_mut::<u16>();
    let mut c: u32 = 0;
    let mut d: u32 = 0;
    let mut inlen: u32 = 0;
    let mut tmp: *mut u8 = ::core::ptr::null_mut::<u8>();
    let mut bits: ::core::ffi::c_int = 0;
    if inlenb % 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        inlenb -= 1;
    }
    inlen = (inlenb / 2 as ::core::ffi::c_int) as u32;
    inend = in_0.offset(inlen as isize);
    let mut bytesNeeded: u32 = 0 as u32;
    while in_0 < inend {
        tmp = in_0 as *mut u8;
        let fresh9 = tmp;
        tmp = tmp.offset(1);
        c = *fresh9 as u32;
        c = c << 8 as ::core::ffi::c_int;
        c = c | *tmp as u32;
        in_0 = in_0.offset(1);
        if c & 0xfc00 as u32 == 0xd800 as u32 {
            if in_0 >= inend {
                break;
            }
            tmp = in_0 as *mut u8;
            let fresh10 = tmp;
            tmp = tmp.offset(1);
            d = *fresh10 as u32;
            d = d << 8 as ::core::ffi::c_int;
            d = d | *tmp as u32;
            in_0 = in_0.offset(1);
            if d & 0xfc00 as u32 == 0xdc00 as u32 {
                c &= 0x3ff as u32;
                c <<= 10 as ::core::ffi::c_int;
                c |= d & 0x3ff as u32;
                c = c.wrapping_add(0x10000 as u32);
            }
        }
        if c < 0x80 as u32 {
            bytesNeeded = bytesNeeded.wrapping_add(1 as u32);
        } else if c < 0x800 as u32 {
            bytesNeeded = bytesNeeded.wrapping_add(2 as u32);
        } else if c < 0x10000 as u32 {
            bytesNeeded = bytesNeeded.wrapping_add(3 as u32);
        } else {
            bytesNeeded = bytesNeeded.wrapping_add(4 as u32);
        }
    }
    in_0 = inb as *mut u16;
    let mut out: sds = sdsnewlen(
        ::core::ptr::null::<::core::ffi::c_void>(),
        bytesNeeded as usize,
    );
    let mut out0: sds = out;
    while in_0 < inend {
        tmp = in_0 as *mut u8;
        let fresh11 = tmp;
        tmp = tmp.offset(1);
        c = *fresh11 as u32;
        c = c << 8 as ::core::ffi::c_int;
        c = c | *tmp as u32;
        in_0 = in_0.offset(1);
        if c & 0xfc00 as u32 == 0xd800 as u32 {
            if in_0 >= inend {
                break;
            }
            tmp = in_0 as *mut u8;
            let fresh12 = tmp;
            tmp = tmp.offset(1);
            d = *fresh12 as u32;
            d = d << 8 as ::core::ffi::c_int;
            d = d | *tmp as u32;
            in_0 = in_0.offset(1);
            if d & 0xfc00 as u32 == 0xdc00 as u32 {
                c &= 0x3ff as u32;
                c <<= 10 as ::core::ffi::c_int;
                c |= d & 0x3ff as u32;
                c = c.wrapping_add(0x10000 as u32);
            }
        }
        if c < 0x80 as u32 {
            let fresh13 = out;
            out = out.offset(1);
            *fresh13 = c as ::core::ffi::c_char;
            bits = -(6 as ::core::ffi::c_int);
        } else if c < 0x800 as u32 {
            let fresh14 = out;
            out = out.offset(1);
            *fresh14 = (c >> 6 as ::core::ffi::c_int & 0x1f as u32 | 0xc0 as u32)
                as ::core::ffi::c_char;
            bits = 0 as ::core::ffi::c_int;
        } else if c < 0x10000 as u32 {
            let fresh15 = out;
            out = out.offset(1);
            *fresh15 = (c >> 12 as ::core::ffi::c_int & 0xf as u32 | 0xe0 as u32)
                as ::core::ffi::c_char;
            bits = 6 as ::core::ffi::c_int;
        } else {
            let fresh16 = out;
            out = out.offset(1);
            *fresh16 = (c >> 18 as ::core::ffi::c_int & 0x7 as u32 | 0xf0 as u32)
                as ::core::ffi::c_char;
            bits = 12 as ::core::ffi::c_int;
        }
        while bits >= 0 as ::core::ffi::c_int {
            let fresh17 = out;
            out = out.offset(1);
            *fresh17 = (c >> bits & 0x3f as u32 | 0x80 as u32) as ::core::ffi::c_char;
            bits -= 6 as ::core::ffi::c_int;
        }
    }
    return out0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn utf8toutf16be(mut _in: sds, mut out_bytes: *mut usize) -> *mut u8 {
    if _in.is_null() {
        *out_bytes = 0 as usize;
        return ::core::ptr::null_mut::<u8>();
    }
    let mut in_0: sds = _in;
    let mut inlen: usize = sdslen(in_0);
    let mut inend: *mut ::core::ffi::c_char = in_0.offset(inlen as isize);
    let mut wordsNeeded: u32 = 0 as u32;
    let mut trailing: u8 = 0 as u8;
    let mut c: u32 = 0 as u32;
    while in_0 < inend {
        let fresh18 = in_0;
        in_0 = in_0.offset(1);
        let mut d: u8 = *fresh18 as u8;
        if (d as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
            c = d as u32;
            trailing = 0 as u8;
        } else {
            if (d as ::core::ffi::c_int) < 0xc0 as ::core::ffi::c_int {
                break;
            }
            if (d as ::core::ffi::c_int) < 0xe0 as ::core::ffi::c_int {
                c = (d as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as u32;
                trailing = 1 as u8;
            } else if (d as ::core::ffi::c_int) < 0xf0 as ::core::ffi::c_int {
                c = (d as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as u32;
                trailing = 2 as u8;
            } else {
                if !((d as ::core::ffi::c_int) < 0xf8 as ::core::ffi::c_int) {
                    break;
                }
                c = (d as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as u32;
                trailing = 3 as u8;
            }
        }
        if (inend.offset_from(in_0) as ::core::ffi::c_long) < trailing as ::core::ffi::c_long {
            break;
        }
        while trailing != 0 {
            if in_0 >= inend || {
                let fresh19 = in_0;
                in_0 = in_0.offset(1);
                d = *fresh19 as u8;
                d as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int != 0x80 as ::core::ffi::c_int
            } {
                break;
            }
            c <<= 6 as ::core::ffi::c_int;
            c |= (d as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as u32;
            trailing = trailing.wrapping_sub(1);
        }
        if c < 0x10000 as u32 {
            wordsNeeded = wordsNeeded.wrapping_add(1 as u32);
        } else if c < 0x110000 as u32 {
            wordsNeeded = wordsNeeded.wrapping_add(2 as u32);
        }
    }
    let mut _out: *mut u8 = malloc(
        ((2 as u32).wrapping_mul(wordsNeeded) as usize)
            .wrapping_mul(::core::mem::size_of::<u8>() as usize),
    ) as *mut u8;
    let mut out: *mut u8 = _out;
    in_0 = _in;
    while in_0 < inend {
        let fresh20 = in_0;
        in_0 = in_0.offset(1);
        let mut d_0: u8 = *fresh20 as u8;
        if (d_0 as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
            c = d_0 as u32;
            trailing = 0 as u8;
        } else {
            if (d_0 as ::core::ffi::c_int) < 0xc0 as ::core::ffi::c_int {
                break;
            }
            if (d_0 as ::core::ffi::c_int) < 0xe0 as ::core::ffi::c_int {
                c = (d_0 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as u32;
                trailing = 1 as u8;
            } else if (d_0 as ::core::ffi::c_int) < 0xf0 as ::core::ffi::c_int {
                c = (d_0 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as u32;
                trailing = 2 as u8;
            } else {
                if !((d_0 as ::core::ffi::c_int) < 0xf8 as ::core::ffi::c_int) {
                    break;
                }
                c = (d_0 as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as u32;
                trailing = 3 as u8;
            }
        }
        if (inend.offset_from(in_0) as ::core::ffi::c_long) < trailing as ::core::ffi::c_long {
            break;
        }
        while trailing != 0 {
            if in_0 >= inend || {
                let fresh21 = in_0;
                in_0 = in_0.offset(1);
                d_0 = *fresh21 as u8;
                d_0 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int != 0x80 as ::core::ffi::c_int
            } {
                break;
            }
            c <<= 6 as ::core::ffi::c_int;
            c |= (d_0 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as u32;
            trailing = trailing.wrapping_sub(1);
        }
        if c < 0x10000 as u32 {
            let fresh22 = out;
            out = out.offset(1);
            *fresh22 = (c >> 8 as ::core::ffi::c_int & 0xff as u32) as u8;
            let fresh23 = out;
            out = out.offset(1);
            *fresh23 = (c & 0xff as u32) as u8;
        } else if c < 0x110000 as u32 {
            let mut tmp1: u16 =
                (0xd800 as u32 | c >> 10 as ::core::ffi::c_int) as u16;
            let fresh24 = out;
            out = out.offset(1);
            *fresh24 = (tmp1 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as u8;
            let fresh25 = out;
            out = out.offset(1);
            *fresh25 = (tmp1 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u8;
            let mut tmp2: u16 = (0xdc00 as u32 | c & 0x3ff as u32) as u16;
            let fresh26 = out;
            out = out.offset(1);
            *fresh26 = (tmp2 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as u8;
            let fresh27 = out;
            out = out.offset(1);
            *fresh27 = (tmp2 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u8;
        }
    }
    *out_bytes = wordsNeeded.wrapping_mul(2 as u32) as usize;
    return _out;
}
