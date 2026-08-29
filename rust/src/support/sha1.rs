#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::memset;
pub type BYTE = ::core::ffi::c_uchar;
pub type WORD = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
pub struct Sha1Ctx {
    pub data: [BYTE; 64],
    pub datalen: WORD,
    pub bitlen: ::core::ffi::c_ulonglong,
    pub state: [WORD; 5],
    pub k: [WORD; 4],
}
pub unsafe fn sha1_transform(ctx: *mut Sha1Ctx, data: *const BYTE) {
    let mut a: WORD;
    let mut b: WORD;
    let mut c: WORD;
    let mut d: WORD;
    let mut e: WORD;
    let mut i: WORD;
    let mut j: WORD;
    let mut t: WORD;
    let mut m: [WORD; 80] = [0; 80];
    i = 0 as WORD;
    j = 0 as WORD;
    while i < 16 as WORD {
        m[i as usize] = (((*data.offset(j as isize) as i32)
            << 24_i32)
            + ((*data.offset(j.wrapping_add(1 as WORD) as isize) as i32)
                << 16_i32)
            + ((*data.offset(j.wrapping_add(2 as WORD) as isize) as i32)
                << 8_i32)
            + *data.offset(j.wrapping_add(3 as WORD) as isize) as i32)
            as WORD;
        i = i.wrapping_add(1);
        j = j.wrapping_add(4 as WORD);
    }
    while i < 80 as WORD {
        m[i as usize] = m[i.wrapping_sub(3 as WORD) as usize]
            ^ m[i.wrapping_sub(8 as WORD) as usize]
            ^ m[i.wrapping_sub(14 as WORD) as usize]
            ^ m[i.wrapping_sub(16 as WORD) as usize];
        m[i as usize] = m[i as usize].rotate_left(1_u32);
        i = i.wrapping_add(1);
    }
    a = (*ctx).state[0_i32 as usize];
    b = (*ctx).state[1_i32 as usize];
    c = (*ctx).state[2_i32 as usize];
    d = (*ctx).state[3_i32 as usize];
    e = (*ctx).state[4_i32 as usize];
    i = 0 as WORD;
    while i < 20 as WORD {
        t = (a.rotate_left(5_u32))
            .wrapping_add(b & c ^ !b & d)
            .wrapping_add(e)
            .wrapping_add((*ctx).k[0_i32 as usize])
            .wrapping_add(m[i as usize]);
        e = d;
        d = c;
        c = b.rotate_right(32_u32 - 30_u32);
        b = a;
        a = t;
        i = i.wrapping_add(1);
    }
    while i < 40 as WORD {
        t = (a.rotate_left(5_u32))
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add((*ctx).k[1_i32 as usize])
            .wrapping_add(m[i as usize]);
        e = d;
        d = c;
        c = b.rotate_right(32_u32 - 30_u32);
        b = a;
        a = t;
        i = i.wrapping_add(1);
    }
    while i < 60 as WORD {
        t = (a.rotate_left(5_u32))
            .wrapping_add(b & c ^ b & d ^ c & d)
            .wrapping_add(e)
            .wrapping_add((*ctx).k[2_i32 as usize])
            .wrapping_add(m[i as usize]);
        e = d;
        d = c;
        c = b.rotate_right(32_u32 - 30_u32);
        b = a;
        a = t;
        i = i.wrapping_add(1);
    }
    while i < 80 as WORD {
        t = (a.rotate_left(5_u32))
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add((*ctx).k[3_i32 as usize])
            .wrapping_add(m[i as usize]);
        e = d;
        d = c;
        c = b.rotate_right(32_u32 - 30_u32);
        b = a;
        a = t;
        i = i.wrapping_add(1);
    }
    (*ctx).state[0_i32 as usize] =
        (*ctx).state[0_i32 as usize].wrapping_add(a);
    (*ctx).state[1_i32 as usize] =
        (*ctx).state[1_i32 as usize].wrapping_add(b);
    (*ctx).state[2_i32 as usize] =
        (*ctx).state[2_i32 as usize].wrapping_add(c);
    (*ctx).state[3_i32 as usize] =
        (*ctx).state[3_i32 as usize].wrapping_add(d);
    (*ctx).state[4_i32 as usize] =
        (*ctx).state[4_i32 as usize].wrapping_add(e);
}
pub unsafe fn sha1_init(ctx: *mut Sha1Ctx) {
    (*ctx).datalen = 0 as WORD;
    (*ctx).bitlen = 0 as ::core::ffi::c_ulonglong;
    (*ctx).state[0_i32 as usize] = 0x67452301_i32 as WORD;
    (*ctx).state[1_i32 as usize] = 0xefcdab89 as ::core::ffi::c_uint as WORD;
    (*ctx).state[2_i32 as usize] = 0x98badcfe as ::core::ffi::c_uint as WORD;
    (*ctx).state[3_i32 as usize] = 0x10325476_i32 as WORD;
    (*ctx).state[4_i32 as usize] = 0xc3d2e1f0 as ::core::ffi::c_uint as WORD;
    (*ctx).k[0_i32 as usize] = 0x5a827999_i32 as WORD;
    (*ctx).k[1_i32 as usize] = 0x6ed9eba1_i32 as WORD;
    (*ctx).k[2_i32 as usize] = 0x8f1bbcdc as ::core::ffi::c_uint as WORD;
    (*ctx).k[3_i32 as usize] = 0xca62c1d6 as ::core::ffi::c_uint as WORD;
}
pub unsafe fn sha1_update(ctx: *mut Sha1Ctx, data: *const BYTE, len: usize) {
    let mut i: usize;
    i = 0_usize;
    while i < len {
        (*ctx).data[(*ctx).datalen as usize] = *data.offset(i as isize);
        (*ctx).datalen = (*ctx).datalen.wrapping_add(1);
        if (*ctx).datalen == 64 as WORD {
            sha1_transform(ctx, &raw mut (*ctx).data as *mut BYTE as *const BYTE);
            (*ctx).bitlen = (*ctx).bitlen.wrapping_add(512 as ::core::ffi::c_ulonglong);
            (*ctx).datalen = 0 as WORD;
        }
        i = i.wrapping_add(1);
    }
}
pub unsafe fn sha1_final(ctx: *mut Sha1Ctx, hash: *mut BYTE) {
    let mut i: WORD;
    i = (*ctx).datalen;
    if (*ctx).datalen < 56 as WORD {
        (*ctx).data[i as usize] = 0x80 as BYTE;
        i = i.wrapping_add(1);
        while i < 56 as WORD {
            (*ctx).data[i as usize] = 0 as BYTE;
            i = i.wrapping_add(1);
        }
    } else {
        (*ctx).data[i as usize] = 0x80 as BYTE;
        i = i.wrapping_add(1);
        while i < 64 as WORD {
            (*ctx).data[i as usize] = 0 as BYTE;
            i = i.wrapping_add(1);
        }
        sha1_transform(ctx, &raw mut (*ctx).data as *mut BYTE as *const BYTE);
        memset(
            &raw mut (*ctx).data as *mut BYTE as *mut ::core::ffi::c_void,
            0_i32,
            56_usize,
        );
    }
    (*ctx).bitlen = (*ctx)
        .bitlen
        .wrapping_add((*ctx).datalen.wrapping_mul(8 as WORD) as ::core::ffi::c_ulonglong);
    (*ctx).data[63_i32 as usize] = (*ctx).bitlen as BYTE;
    (*ctx).data[62_i32 as usize] =
        ((*ctx).bitlen >> 8_i32) as BYTE;
    (*ctx).data[61_i32 as usize] =
        ((*ctx).bitlen >> 16_i32) as BYTE;
    (*ctx).data[60_i32 as usize] =
        ((*ctx).bitlen >> 24_i32) as BYTE;
    (*ctx).data[59_i32 as usize] =
        ((*ctx).bitlen >> 32_i32) as BYTE;
    (*ctx).data[58_i32 as usize] =
        ((*ctx).bitlen >> 40_i32) as BYTE;
    (*ctx).data[57_i32 as usize] =
        ((*ctx).bitlen >> 48_i32) as BYTE;
    (*ctx).data[56_i32 as usize] =
        ((*ctx).bitlen >> 56_i32) as BYTE;
    sha1_transform(ctx, &raw mut (*ctx).data as *mut BYTE as *const BYTE);
    i = 0 as WORD;
    while i < 4 as WORD {
        *hash.offset(i as isize) = ((*ctx).state[0_i32 as usize]
            >> (24 as WORD).wrapping_sub(i.wrapping_mul(8 as WORD))
            & 0xff as WORD) as BYTE;
        *hash.offset(i.wrapping_add(4 as WORD) as isize) = ((*ctx).state
            [1_i32 as usize]
            >> (24 as WORD).wrapping_sub(i.wrapping_mul(8 as WORD))
            & 0xff as WORD) as BYTE;
        *hash.offset(i.wrapping_add(8 as WORD) as isize) = ((*ctx).state
            [2_i32 as usize]
            >> (24 as WORD).wrapping_sub(i.wrapping_mul(8 as WORD))
            & 0xff as WORD) as BYTE;
        *hash.offset(i.wrapping_add(12 as WORD) as isize) = ((*ctx).state
            [3_i32 as usize]
            >> (24 as WORD).wrapping_sub(i.wrapping_mul(8 as WORD))
            & 0xff as WORD) as BYTE;
        *hash.offset(i.wrapping_add(16 as WORD) as isize) = ((*ctx).state
            [4_i32 as usize]
            >> (24 as WORD).wrapping_sub(i.wrapping_mul(8 as WORD))
            & 0xff as WORD) as BYTE;
        i = i.wrapping_add(1);
    }
}
