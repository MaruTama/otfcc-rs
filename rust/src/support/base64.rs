#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{malloc, memset};

static BASE64_TABLE: [u8; 64] = unsafe {
    ::core::mem::transmute::<[u8; 64], [u8; 64]>(
        *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    )
};
pub unsafe fn base64_encode(
    src: *const u8,
    len: usize,
    out_len: *mut usize,
) -> *mut u8 {
    let out: *mut u8;
    let mut pos: *mut u8;
    let end: *const u8;
    let mut in_0: *const u8;
    let mut olen: usize;
    olen = len
        .wrapping_add(3_usize)
        .wrapping_sub(1_usize)
        .wrapping_div(3_usize)
        .wrapping_mul(4_usize);
    olen = olen.wrapping_add(1);
    out = malloc((::core::mem::size_of::<u8>() as usize).wrapping_mul(olen)) as *mut u8;
    if out.is_null() {
        return ::core::ptr::null_mut::<u8>();
    }
    end = src.offset(len as isize);
    in_0 = src;
    pos = out;
    while end.offset_from(in_0) as ::core::ffi::c_long >= 3 as ::core::ffi::c_long {
        let fresh0 = pos;
        pos = pos.offset(1);
        *fresh0 = BASE64_TABLE[(*in_0.offset(0_i32 as isize)
            as i32
            >> 2_i32) as usize];
        let fresh1 = pos;
        pos = pos.offset(1);
        *fresh1 = BASE64_TABLE[((*in_0.offset(0_i32 as isize)
            as i32
            & 0x3_i32)
            << 4_i32
            | *in_0.offset(1_i32 as isize) as i32
                >> 4_i32) as usize];
        let fresh2 = pos;
        pos = pos.offset(1);
        *fresh2 = BASE64_TABLE[((*in_0.offset(1_i32 as isize)
            as i32
            & 0xf_i32)
            << 2_i32
            | *in_0.offset(2_i32 as isize) as i32
                >> 6_i32) as usize];
        let fresh3 = pos;
        pos = pos.offset(1);
        *fresh3 = BASE64_TABLE[(*in_0.offset(2_i32 as isize)
            as i32
            & 0x3f_i32) as usize];
        in_0 = in_0.offset(3_i32 as isize);
    }
    if end.offset_from(in_0) as ::core::ffi::c_long != 0 {
        let fresh4 = pos;
        pos = pos.offset(1);
        *fresh4 = BASE64_TABLE[(*in_0.offset(0_i32 as isize)
            as i32
            >> 2_i32) as usize];
        if end.offset_from(in_0) as ::core::ffi::c_long == 1 as ::core::ffi::c_long {
            let fresh5 = pos;
            pos = pos.offset(1);
            *fresh5 = BASE64_TABLE[((*in_0.offset(0_i32 as isize)
                as i32
                & 0x3_i32)
                << 4_i32) as usize];
            let fresh6 = pos;
            pos = pos.offset(1);
            *fresh6 = '=' as i32 as u8;
        } else {
            let fresh7 = pos;
            pos = pos.offset(1);
            *fresh7 = BASE64_TABLE[((*in_0.offset(0_i32 as isize)
                as i32
                & 0x3_i32)
                << 4_i32
                | *in_0.offset(1_i32 as isize) as i32
                    >> 4_i32) as usize];
            let fresh8 = pos;
            pos = pos.offset(1);
            *fresh8 = BASE64_TABLE[((*in_0.offset(1_i32 as isize)
                as i32
                & 0xf_i32)
                << 2_i32) as usize];
        }
        let fresh9 = pos;
        pos = pos.offset(1);
        *fresh9 = '=' as i32 as u8;
    }
    *pos = '\0' as i32 as u8;
    if !out_len.is_null() {
        *out_len = pos.offset_from(out) as ::core::ffi::c_long as usize;
    }
    return out;
}
pub unsafe fn base64_decode(
    src: *const u8,
    len: usize,
    out_len: *mut usize,
) -> *mut u8 {
    let mut dtable: [u8; 256] = [0; 256];
    let out: *mut u8;
    let mut pos: *mut u8;
    let mut in_0: [u8; 4] = [0; 4];
    let mut block: [u8; 4] = [0; 4];
    let mut tmp: u8;
    let mut i: usize;
    let mut count: usize;
    memset(
        &raw mut dtable as *mut u8 as *mut ::core::ffi::c_void,
        0x80_i32,
        256_usize,
    );
    i = 0_usize;
    while i < ::core::mem::size_of::<[u8; 64]>() {
        dtable[BASE64_TABLE[i] as usize] = i as u8;
        i = i.wrapping_add(1);
    }
    dtable['=' as i32 as usize] = 0_u8;
    count = 0_usize;
    i = 0_usize;
    while i < len {
        if dtable[*src.offset(i as isize) as usize] as i32
            != 0x80_i32
        {
            count = count.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if count.wrapping_rem(4_usize) != 0 {
        return ::core::ptr::null_mut::<u8>();
    }
    out = malloc((::core::mem::size_of::<u8>() as usize).wrapping_mul(count)) as *mut u8;
    pos = out;
    if out.is_null() {
        return ::core::ptr::null_mut::<u8>();
    }
    count = 0_usize;
    i = 0_usize;
    while i < len {
        tmp = dtable[*src.offset(i as isize) as usize];
        if !(tmp as i32 == 0x80_i32) {
            in_0[count] = *src.offset(i as isize);
            block[count] = tmp;
            count = count.wrapping_add(1);
            if count == 4_usize {
                let fresh10 = pos;
                pos = pos.offset(1);
                *fresh10 = ((block[0_i32 as usize] as i32)
                    << 2_i32
                    | block[1_i32 as usize] as i32
                        >> 4_i32) as u8;
                let fresh11 = pos;
                pos = pos.offset(1);
                *fresh11 = ((block[1_i32 as usize] as i32)
                    << 4_i32
                    | block[2_i32 as usize] as i32
                        >> 2_i32) as u8;
                let fresh12 = pos;
                pos = pos.offset(1);
                *fresh12 = ((block[2_i32 as usize] as i32)
                    << 6_i32
                    | block[3_i32 as usize] as i32)
                    as u8;
                count = 0_usize;
            }
        }
        i = i.wrapping_add(1);
    }
    if pos > out {
        if in_0[2_i32 as usize] as i32 == '=' as i32 {
            pos = pos.offset(-(2_i32 as isize));
        } else if in_0[3_i32 as usize] as i32 == '=' as i32 {
            pos = pos.offset(-1);
        }
    }
    *out_len = pos.offset_from(out) as ::core::ffi::c_long as usize;
    return out;
}
