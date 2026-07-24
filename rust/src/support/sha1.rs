extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type BYTE = ::core::ffi::c_uchar;
pub type WORD = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1_CTX {
    pub data: [BYTE; 64],
    pub datalen: WORD,
    pub bitlen: ::core::ffi::c_ulonglong,
    pub state: [WORD; 5],
    pub k: [WORD; 4],
}
#[no_mangle]
pub unsafe extern "C" fn sha1_transform(mut ctx: *mut SHA1_CTX, mut data: *const BYTE) {
    let mut a: WORD = 0;
    let mut b: WORD = 0;
    let mut c: WORD = 0;
    let mut d: WORD = 0;
    let mut e: WORD = 0;
    let mut i: WORD = 0;
    let mut j: WORD = 0;
    let mut t: WORD = 0;
    let mut m: [WORD; 80] = [0; 80];
    i = 0 as WORD;
    j = 0 as WORD;
    while i < 16 as WORD {
        m[i as usize] = (((*data.offset(j as isize) as ::core::ffi::c_int)
            << 24 as ::core::ffi::c_int)
            + ((*data.offset(j.wrapping_add(1 as WORD) as isize) as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int)
            + ((*data.offset(j.wrapping_add(2 as WORD) as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int)
            + *data.offset(j.wrapping_add(3 as WORD) as isize) as ::core::ffi::c_int)
            as WORD;
        i = i.wrapping_add(1);
        j = j.wrapping_add(4 as WORD);
    }
    while i < 80 as WORD {
        m[i as usize] = m[i.wrapping_sub(3 as WORD) as usize]
            ^ m[i.wrapping_sub(8 as WORD) as usize]
            ^ m[i.wrapping_sub(14 as WORD) as usize]
            ^ m[i.wrapping_sub(16 as WORD) as usize];
        m[i as usize] =
            m[i as usize] << 1 as ::core::ffi::c_int | m[i as usize] >> 31 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    a = (*ctx).state[0 as ::core::ffi::c_int as usize];
    b = (*ctx).state[1 as ::core::ffi::c_int as usize];
    c = (*ctx).state[2 as ::core::ffi::c_int as usize];
    d = (*ctx).state[3 as ::core::ffi::c_int as usize];
    e = (*ctx).state[4 as ::core::ffi::c_int as usize];
    i = 0 as WORD;
    while i < 20 as WORD {
        t = (a << 5 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(b & c ^ !b & d)
            .wrapping_add(e)
            .wrapping_add((*ctx).k[0 as ::core::ffi::c_int as usize])
            .wrapping_add(m[i as usize]);
        e = d;
        d = c;
        c = b << 30 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
        b = a;
        a = t;
        i = i.wrapping_add(1);
    }
    while i < 40 as WORD {
        t = (a << 5 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add((*ctx).k[1 as ::core::ffi::c_int as usize])
            .wrapping_add(m[i as usize]);
        e = d;
        d = c;
        c = b << 30 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
        b = a;
        a = t;
        i = i.wrapping_add(1);
    }
    while i < 60 as WORD {
        t = (a << 5 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(b & c ^ b & d ^ c & d)
            .wrapping_add(e)
            .wrapping_add((*ctx).k[2 as ::core::ffi::c_int as usize])
            .wrapping_add(m[i as usize]);
        e = d;
        d = c;
        c = b << 30 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
        b = a;
        a = t;
        i = i.wrapping_add(1);
    }
    while i < 80 as WORD {
        t = (a << 5 as ::core::ffi::c_int
            | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add((*ctx).k[3 as ::core::ffi::c_int as usize])
            .wrapping_add(m[i as usize]);
        e = d;
        d = c;
        c = b << 30 as ::core::ffi::c_int
            | b >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
        b = a;
        a = t;
        i = i.wrapping_add(1);
    }
    (*ctx).state[0 as ::core::ffi::c_int as usize] =
        (*ctx).state[0 as ::core::ffi::c_int as usize].wrapping_add(a);
    (*ctx).state[1 as ::core::ffi::c_int as usize] =
        (*ctx).state[1 as ::core::ffi::c_int as usize].wrapping_add(b);
    (*ctx).state[2 as ::core::ffi::c_int as usize] =
        (*ctx).state[2 as ::core::ffi::c_int as usize].wrapping_add(c);
    (*ctx).state[3 as ::core::ffi::c_int as usize] =
        (*ctx).state[3 as ::core::ffi::c_int as usize].wrapping_add(d);
    (*ctx).state[4 as ::core::ffi::c_int as usize] =
        (*ctx).state[4 as ::core::ffi::c_int as usize].wrapping_add(e);
}
#[no_mangle]
pub unsafe extern "C" fn sha1_init(mut ctx: *mut SHA1_CTX) {
    (*ctx).datalen = 0 as WORD;
    (*ctx).bitlen = 0 as ::core::ffi::c_ulonglong;
    (*ctx).state[0 as ::core::ffi::c_int as usize] = 0x67452301 as ::core::ffi::c_int as WORD;
    (*ctx).state[1 as ::core::ffi::c_int as usize] = 0xefcdab89 as ::core::ffi::c_uint as WORD;
    (*ctx).state[2 as ::core::ffi::c_int as usize] = 0x98badcfe as ::core::ffi::c_uint as WORD;
    (*ctx).state[3 as ::core::ffi::c_int as usize] = 0x10325476 as ::core::ffi::c_int as WORD;
    (*ctx).state[4 as ::core::ffi::c_int as usize] = 0xc3d2e1f0 as ::core::ffi::c_uint as WORD;
    (*ctx).k[0 as ::core::ffi::c_int as usize] = 0x5a827999 as ::core::ffi::c_int as WORD;
    (*ctx).k[1 as ::core::ffi::c_int as usize] = 0x6ed9eba1 as ::core::ffi::c_int as WORD;
    (*ctx).k[2 as ::core::ffi::c_int as usize] = 0x8f1bbcdc as ::core::ffi::c_uint as WORD;
    (*ctx).k[3 as ::core::ffi::c_int as usize] = 0xca62c1d6 as ::core::ffi::c_uint as WORD;
}
#[no_mangle]
pub unsafe extern "C" fn sha1_update(
    mut ctx: *mut SHA1_CTX,
    mut data: *const BYTE,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
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
#[no_mangle]
pub unsafe extern "C" fn sha1_final(mut ctx: *mut SHA1_CTX, mut hash: *mut BYTE) {
    let mut i: WORD = 0;
    i = (*ctx).datalen;
    if (*ctx).datalen < 56 as WORD {
        let fresh0 = i;
        i = i.wrapping_add(1);
        (*ctx).data[fresh0 as usize] = 0x80 as BYTE;
        while i < 56 as WORD {
            let fresh1 = i;
            i = i.wrapping_add(1);
            (*ctx).data[fresh1 as usize] = 0 as BYTE;
        }
    } else {
        let fresh2 = i;
        i = i.wrapping_add(1);
        (*ctx).data[fresh2 as usize] = 0x80 as BYTE;
        while i < 64 as WORD {
            let fresh3 = i;
            i = i.wrapping_add(1);
            (*ctx).data[fresh3 as usize] = 0 as BYTE;
        }
        sha1_transform(ctx, &raw mut (*ctx).data as *mut BYTE as *const BYTE);
        memset(
            &raw mut (*ctx).data as *mut BYTE as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            56 as size_t,
        );
    }
    (*ctx).bitlen = (*ctx)
        .bitlen
        .wrapping_add((*ctx).datalen.wrapping_mul(8 as WORD) as ::core::ffi::c_ulonglong);
    (*ctx).data[63 as ::core::ffi::c_int as usize] = (*ctx).bitlen as BYTE;
    (*ctx).data[62 as ::core::ffi::c_int as usize] =
        ((*ctx).bitlen >> 8 as ::core::ffi::c_int) as BYTE;
    (*ctx).data[61 as ::core::ffi::c_int as usize] =
        ((*ctx).bitlen >> 16 as ::core::ffi::c_int) as BYTE;
    (*ctx).data[60 as ::core::ffi::c_int as usize] =
        ((*ctx).bitlen >> 24 as ::core::ffi::c_int) as BYTE;
    (*ctx).data[59 as ::core::ffi::c_int as usize] =
        ((*ctx).bitlen >> 32 as ::core::ffi::c_int) as BYTE;
    (*ctx).data[58 as ::core::ffi::c_int as usize] =
        ((*ctx).bitlen >> 40 as ::core::ffi::c_int) as BYTE;
    (*ctx).data[57 as ::core::ffi::c_int as usize] =
        ((*ctx).bitlen >> 48 as ::core::ffi::c_int) as BYTE;
    (*ctx).data[56 as ::core::ffi::c_int as usize] =
        ((*ctx).bitlen >> 56 as ::core::ffi::c_int) as BYTE;
    sha1_transform(ctx, &raw mut (*ctx).data as *mut BYTE as *const BYTE);
    i = 0 as WORD;
    while i < 4 as WORD {
        *hash.offset(i as isize) = ((*ctx).state[0 as ::core::ffi::c_int as usize]
            >> (24 as WORD).wrapping_sub(i.wrapping_mul(8 as WORD))
            & 0xff as WORD) as BYTE;
        *hash.offset(i.wrapping_add(4 as WORD) as isize) = ((*ctx).state
            [1 as ::core::ffi::c_int as usize]
            >> (24 as WORD).wrapping_sub(i.wrapping_mul(8 as WORD))
            & 0xff as WORD) as BYTE;
        *hash.offset(i.wrapping_add(8 as WORD) as isize) = ((*ctx).state
            [2 as ::core::ffi::c_int as usize]
            >> (24 as WORD).wrapping_sub(i.wrapping_mul(8 as WORD))
            & 0xff as WORD) as BYTE;
        *hash.offset(i.wrapping_add(12 as WORD) as isize) = ((*ctx).state
            [3 as ::core::ffi::c_int as usize]
            >> (24 as WORD).wrapping_sub(i.wrapping_mul(8 as WORD))
            & 0xff as WORD) as BYTE;
        *hash.offset(i.wrapping_add(16 as WORD) as isize) = ((*ctx).state
            [4 as ::core::ffi::c_int as usize]
            >> (24 as WORD).wrapping_sub(i.wrapping_mul(8 as WORD))
            & 0xff as WORD) as BYTE;
        i = i.wrapping_add(1);
    }
}
