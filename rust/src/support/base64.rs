extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut base64_table: [uint8_t; 64] = unsafe {
    ::core::mem::transmute::<[u8; 64], [uint8_t; 64]>(
        *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    )
};
#[no_mangle]
pub unsafe extern "C" fn base64_encode(
    mut src: *const uint8_t,
    mut len: size_t,
    mut out_len: *mut size_t,
) -> *mut uint8_t {
    let mut out: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut pos: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut end: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut in_0: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut olen: size_t = 0;
    olen = len
        .wrapping_add(3 as size_t)
        .wrapping_sub(1 as size_t)
        .wrapping_div(3 as size_t)
        .wrapping_mul(4 as size_t);
    olen = olen.wrapping_add(1);
    out = malloc((::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(olen)) as *mut uint8_t;
    if out.is_null() {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    end = src.offset(len as isize);
    in_0 = src;
    pos = out;
    while end.offset_from(in_0) as ::core::ffi::c_long >= 3 as ::core::ffi::c_long {
        let fresh0 = pos;
        pos = pos.offset(1);
        *fresh0 = base64_table[(*in_0.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as usize];
        let fresh1 = pos;
        pos = pos.offset(1);
        *fresh1 = base64_table[((*in_0.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & 0x3 as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int
            | *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as usize];
        let fresh2 = pos;
        pos = pos.offset(1);
        *fresh2 = base64_table[((*in_0.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int
            | *in_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as usize];
        let fresh3 = pos;
        pos = pos.offset(1);
        *fresh3 = base64_table[(*in_0.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & 0x3f as ::core::ffi::c_int) as usize];
        in_0 = in_0.offset(3 as ::core::ffi::c_int as isize);
    }
    if end.offset_from(in_0) as ::core::ffi::c_long != 0 {
        let fresh4 = pos;
        pos = pos.offset(1);
        *fresh4 = base64_table[(*in_0.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as usize];
        if end.offset_from(in_0) as ::core::ffi::c_long == 1 as ::core::ffi::c_long {
            let fresh5 = pos;
            pos = pos.offset(1);
            *fresh5 = base64_table[((*in_0.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & 0x3 as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int) as usize];
            let fresh6 = pos;
            pos = pos.offset(1);
            *fresh6 = '=' as i32 as uint8_t;
        } else {
            let fresh7 = pos;
            pos = pos.offset(1);
            *fresh7 = base64_table[((*in_0.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & 0x3 as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int
                | *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int) as usize];
            let fresh8 = pos;
            pos = pos.offset(1);
            *fresh8 = base64_table[((*in_0.offset(1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) as usize];
        }
        let fresh9 = pos;
        pos = pos.offset(1);
        *fresh9 = '=' as i32 as uint8_t;
    }
    *pos = '\0' as i32 as uint8_t;
    if !out_len.is_null() {
        *out_len = pos.offset_from(out) as ::core::ffi::c_long as size_t;
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn base64_decode(
    mut src: *const uint8_t,
    mut len: size_t,
    mut out_len: *mut size_t,
) -> *mut uint8_t {
    let mut dtable: [uint8_t; 256] = [0; 256];
    let mut out: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut pos: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut in_0: [uint8_t; 4] = [0; 4];
    let mut block: [uint8_t; 4] = [0; 4];
    let mut tmp: uint8_t = 0;
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    memset(
        &raw mut dtable as *mut uint8_t as *mut ::core::ffi::c_void,
        0x80 as ::core::ffi::c_int,
        256 as size_t,
    );
    i = 0 as size_t;
    while i < ::core::mem::size_of::<[uint8_t; 64]>() {
        dtable[base64_table[i] as usize] = i as uint8_t;
        i = i.wrapping_add(1);
    }
    dtable['=' as i32 as usize] = 0 as uint8_t;
    count = 0 as size_t;
    i = 0 as size_t;
    while i < len {
        if dtable[*src.offset(i as isize) as usize] as ::core::ffi::c_int
            != 0x80 as ::core::ffi::c_int
        {
            count = count.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if count.wrapping_rem(4 as size_t) != 0 {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    out = malloc((::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(count)) as *mut uint8_t;
    pos = out;
    if out.is_null() {
        return ::core::ptr::null_mut::<uint8_t>();
    }
    count = 0 as size_t;
    i = 0 as size_t;
    while i < len {
        tmp = dtable[*src.offset(i as isize) as usize];
        if !(tmp as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int) {
            in_0[count] = *src.offset(i as isize);
            block[count] = tmp;
            count = count.wrapping_add(1);
            if count == 4 as size_t {
                let fresh10 = pos;
                pos = pos.offset(1);
                *fresh10 = ((block[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int
                    | block[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        >> 4 as ::core::ffi::c_int) as uint8_t;
                let fresh11 = pos;
                pos = pos.offset(1);
                *fresh11 = ((block[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int
                    | block[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int) as uint8_t;
                let fresh12 = pos;
                pos = pos.offset(1);
                *fresh12 = ((block[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int
                    | block[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                    as uint8_t;
                count = 0 as size_t;
            }
        }
        i = i.wrapping_add(1);
    }
    if pos > out {
        if in_0[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '=' as i32 {
            pos = pos.offset(-(2 as ::core::ffi::c_int as isize));
        } else if in_0[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '=' as i32 {
            pos = pos.offset(-1);
        }
    }
    *out_len = pos.offset_from(out) as ::core::ffi::c_long as size_t;
    return out;
}
