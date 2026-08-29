#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

// `utf16be_to_utf8` returns `Vec<u8>` now instead of `SdsRaw`, its only
// caller (`table/name.rs`) a direct Rust call site (never a real FFI
// boundary) -- goes away with the vtable/extern "C" cleanup, same as
// every other instance of this allow in the crate.
#[allow(improper_ctypes_definitions)]
pub unsafe fn utf16be_to_utf8(inb: *const u8, mut inlenb: i32) -> Vec<u8> {
    let mut in_0: *mut u16 = inb as *mut u16;
    let inend: *mut u16;
    let mut c: u32;
    let mut d: u32;
    let inlen: u32;
    let mut tmp: *mut u8;
    let mut bits: i32;
    if inlenb % 2_i32 == 1_i32 {
        inlenb -= 1;
    }
    inlen = (inlenb / 2_i32) as u32;
    inend = in_0.offset(inlen as isize);
    let mut bytes_needed: u32 = 0_u32;
    while in_0 < inend {
        tmp = in_0 as *mut u8;
        c = *tmp as u32;
        c = c << 8_i32;
        tmp = tmp.offset(1);
        c = c | *tmp as u32;
        in_0 = in_0.offset(1);
        if c & 0xfc00_u32 == 0xd800_u32 {
            if in_0 >= inend {
                break;
            }
            tmp = in_0 as *mut u8;
            d = *tmp as u32;
            d = d << 8_i32;
            tmp = tmp.offset(1);
            d = d | *tmp as u32;
            in_0 = in_0.offset(1);
            if d & 0xfc00_u32 == 0xdc00_u32 {
                c &= 0x3ff_u32;
                c <<= 10_i32;
                c |= d & 0x3ff_u32;
                c = c.wrapping_add(0x10000_u32);
            }
        }
        if c < 0x80_u32 {
            bytes_needed = bytes_needed.wrapping_add(1_u32);
        } else if c < 0x800_u32 {
            bytes_needed = bytes_needed.wrapping_add(2_u32);
        } else if c < 0x10000_u32 {
            bytes_needed = bytes_needed.wrapping_add(3_u32);
        } else {
            bytes_needed = bytes_needed.wrapping_add(4_u32);
        }
    }
    in_0 = inb as *mut u16;
    let mut out: Vec<u8> = Vec::with_capacity(bytes_needed as usize);
    while in_0 < inend {
        tmp = in_0 as *mut u8;
        c = *tmp as u32;
        c = c << 8_i32;
        tmp = tmp.offset(1);
        c = c | *tmp as u32;
        in_0 = in_0.offset(1);
        if c & 0xfc00_u32 == 0xd800_u32 {
            if in_0 >= inend {
                break;
            }
            tmp = in_0 as *mut u8;
            d = *tmp as u32;
            d = d << 8_i32;
            tmp = tmp.offset(1);
            d = d | *tmp as u32;
            in_0 = in_0.offset(1);
            if d & 0xfc00_u32 == 0xdc00_u32 {
                c &= 0x3ff_u32;
                c <<= 10_i32;
                c |= d & 0x3ff_u32;
                c = c.wrapping_add(0x10000_u32);
            }
        }
        if c < 0x80_u32 {
            out.push(c as u8);
            bits = -6_i32;
        } else if c < 0x800_u32 {
            out.push((c >> 6_i32 & 0x1f_u32 | 0xc0_u32) as u8);
            bits = 0_i32;
        } else if c < 0x10000_u32 {
            out.push((c >> 12_i32 & 0xf_u32 | 0xe0_u32) as u8);
            bits = 6_i32;
        } else {
            out.push((c >> 18_i32 & 0x7_u32 | 0xf0_u32) as u8);
            bits = 12_i32;
        }
        while bits >= 0_i32 {
            out.push((c >> bits & 0x3f_u32 | 0x80_u32) as u8);
            bits -= 6_i32;
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
    let inend: *const ::core::ffi::c_char = in_0.offset(inlen as isize);
    let mut words_needed: u32 = 0_u32;
    let mut trailing: u8;
    let mut c: u32;
    while in_0 < inend {
        let mut d: u8 = *in_0 as u8;
        in_0 = in_0.offset(1);
        if (d as i32) < 0x80_i32 {
            c = d as u32;
            trailing = 0_u8;
        } else {
            if (d as i32) < 0xc0_i32 {
                break;
            }
            if (d as i32) < 0xe0_i32 {
                c = (d as i32 & 0x1f_i32) as u32;
                trailing = 1_u8;
            } else if (d as i32) < 0xf0_i32 {
                c = (d as i32 & 0xf_i32) as u32;
                trailing = 2_u8;
            } else {
                if !((d as i32) < 0xf8_i32) {
                    break;
                }
                c = (d as i32 & 0x7_i32) as u32;
                trailing = 3_u8;
            }
        }
        if (inend.offset_from(in_0) as ::core::ffi::c_long) < trailing as ::core::ffi::c_long {
            break;
        }
        while trailing != 0 {
            if in_0 >= inend || {
                d = *in_0 as u8;
                in_0 = in_0.offset(1);
                d as i32 & 0xc0_i32 != 0x80_i32
            } {
                break;
            }
            c <<= 6_i32;
            c |= (d as i32 & 0x3f_i32) as u32;
            trailing = trailing.wrapping_sub(1);
        }
        if c < 0x10000_u32 {
            words_needed = words_needed.wrapping_add(1_u32);
        } else if c < 0x110000_u32 {
            words_needed = words_needed.wrapping_add(2_u32);
        }
    }
    let mut out: Vec<u8> = Vec::with_capacity(2_u32.wrapping_mul(words_needed) as usize);
    in_0 = _in.as_ptr() as *const ::core::ffi::c_char;
    while in_0 < inend {
        let mut d_0: u8 = *in_0 as u8;
        in_0 = in_0.offset(1);
        if (d_0 as i32) < 0x80_i32 {
            c = d_0 as u32;
            trailing = 0_u8;
        } else {
            if (d_0 as i32) < 0xc0_i32 {
                break;
            }
            if (d_0 as i32) < 0xe0_i32 {
                c = (d_0 as i32 & 0x1f_i32) as u32;
                trailing = 1_u8;
            } else if (d_0 as i32) < 0xf0_i32 {
                c = (d_0 as i32 & 0xf_i32) as u32;
                trailing = 2_u8;
            } else {
                if !((d_0 as i32) < 0xf8_i32) {
                    break;
                }
                c = (d_0 as i32 & 0x7_i32) as u32;
                trailing = 3_u8;
            }
        }
        if (inend.offset_from(in_0) as ::core::ffi::c_long) < trailing as ::core::ffi::c_long {
            break;
        }
        while trailing != 0 {
            if in_0 >= inend || {
                d_0 = *in_0 as u8;
                in_0 = in_0.offset(1);
                d_0 as i32 & 0xc0_i32 != 0x80_i32
            } {
                break;
            }
            c <<= 6_i32;
            c |= (d_0 as i32 & 0x3f_i32) as u32;
            trailing = trailing.wrapping_sub(1);
        }
        if c < 0x10000_u32 {
            out.push((c >> 8_i32 & 0xff_u32) as u8);
            out.push((c & 0xff_u32) as u8);
        } else if c < 0x110000_u32 {
            let tmp1: u16 = (0xd800_u32 | c >> 10_i32) as u16;
            out.push(
                (tmp1 as i32 >> 8_i32 & 0xff_i32)
                    as u8,
            );
            out.push((tmp1 as i32 & 0xff_i32) as u8);
            let tmp2: u16 = (0xdc00_u32 | c & 0x3ff_u32) as u16;
            out.push(
                (tmp2 as i32 >> 8_i32 & 0xff_i32)
                    as u8,
            );
            out.push((tmp2 as i32 & 0xff_i32) as u8);
        }
    }
    return out;
}
