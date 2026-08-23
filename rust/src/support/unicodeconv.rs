#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

// `utf16be_to_utf8` returns `Vec<u8>` now instead of `SdsRaw`, its only
// caller (`table/name.rs`) a direct Rust call site (never a real FFI
// boundary) -- goes away with the vtable/extern "C" cleanup, same as
// every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
pub unsafe fn utf16be_to_utf8(mut inb: *const u8, mut inlenb: ::core::ffi::c_int) -> Vec<u8> {
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
    let mut bytes_needed: u32 = 0 as u32;
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
            bytes_needed = bytes_needed.wrapping_add(1 as u32);
        } else if c < 0x800 as u32 {
            bytes_needed = bytes_needed.wrapping_add(2 as u32);
        } else if c < 0x10000 as u32 {
            bytes_needed = bytes_needed.wrapping_add(3 as u32);
        } else {
            bytes_needed = bytes_needed.wrapping_add(4 as u32);
        }
    }
    in_0 = inb as *mut u16;
    let mut out: Vec<u8> = Vec::with_capacity(bytes_needed as usize);
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
            out.push(c as u8);
            bits = -(6 as ::core::ffi::c_int);
        } else if c < 0x800 as u32 {
            out.push((c >> 6 as ::core::ffi::c_int & 0x1f as u32 | 0xc0 as u32) as u8);
            bits = 0 as ::core::ffi::c_int;
        } else if c < 0x10000 as u32 {
            out.push((c >> 12 as ::core::ffi::c_int & 0xf as u32 | 0xe0 as u32) as u8);
            bits = 6 as ::core::ffi::c_int;
        } else {
            out.push((c >> 18 as ::core::ffi::c_int & 0x7 as u32 | 0xf0 as u32) as u8);
            bits = 12 as ::core::ffi::c_int;
        }
        while bits >= 0 as ::core::ffi::c_int {
            out.push((c >> bits & 0x3f as u32 | 0x80 as u32) as u8);
            bits -= 6 as ::core::ffi::c_int;
        }
    }
    return out;
}
// `_in` is `&[u8]` now instead of `SdsRaw` -- this function only ever
// reads it (`inlen` from the slice's own length instead of `sdslen`, the
// raw-pointer walk over `[in_0, inend)` otherwise unchanged), never
// stores or frees it, so there was no ownership to plumb through in the
// first place. The null check this replaces was only ever reachable via
// a null `SdsRaw`; a slice reference can't be null, so it's gone rather
// than translated.
//
// Never a real FFI boundary -- internal call site only, same rationale
// as every other instance of this allow in the crate.
//
// Returns `Vec<u8>` now instead of a `malloc`'d `*mut u8` plus an
// `out_bytes` out-param -- matches `utf16be_to_utf8`'s sibling shape,
// its size no longer duplicated in a separate counter the caller had to
// remember to `free`.
pub unsafe fn utf8toutf16be(_in: &[u8]) -> Vec<u8> {
    let mut in_0: *const ::core::ffi::c_char = _in.as_ptr() as *const ::core::ffi::c_char;
    let inlen: usize = _in.len();
    let mut inend: *const ::core::ffi::c_char = in_0.offset(inlen as isize);
    let mut words_needed: u32 = 0 as u32;
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
            words_needed = words_needed.wrapping_add(1 as u32);
        } else if c < 0x110000 as u32 {
            words_needed = words_needed.wrapping_add(2 as u32);
        }
    }
    let mut out: Vec<u8> = Vec::with_capacity((2 as u32).wrapping_mul(words_needed) as usize);
    in_0 = _in.as_ptr() as *const ::core::ffi::c_char;
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
            out.push((c >> 8 as ::core::ffi::c_int & 0xff as u32) as u8);
            out.push((c & 0xff as u32) as u8);
        } else if c < 0x110000 as u32 {
            let mut tmp1: u16 = (0xd800 as u32 | c >> 10 as ::core::ffi::c_int) as u16;
            out.push(
                (tmp1 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                    as u8,
            );
            out.push((tmp1 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u8);
            let mut tmp2: u16 = (0xdc00 as u32 | c & 0x3ff as u32) as u16;
            out.push(
                (tmp2 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                    as u8,
            );
            out.push((tmp2 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u8);
        }
    }
    return out;
}
