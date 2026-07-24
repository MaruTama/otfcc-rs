extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufninit(n: uint32_t, ...) -> *mut caryll_Buffer;
}

use crate::support::stdio::FILE;
use crate::support::alloc::{__caryll_allocate_clean};
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: size_t,
    pub size: size_t,
    pub free: size_t,
    pub data: *mut uint8_t,
}
pub type cff_Value_Type = ::core::ffi::c_uint;
pub const CS2_FRACTION: cff_Value_Type = 3;
pub const cff_DOUBLE: cff_Value_Type = 3;
pub const CS2_OPERAND: cff_Value_Type = 2;
pub const cff_INTEGER: cff_Value_Type = 2;
pub const CS2_OPERATOR: cff_Value_Type = 1;
pub const cff_OPERATOR: cff_Value_Type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Value {
    pub t: cff_Value_Type,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: int32_t,
    pub d: ::core::ffi::c_double,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double {
    return strtod(__nptr, NULL as *mut *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn cff_encodeCffOperator(mut val: int32_t) -> *mut caryll_Buffer {
    if val > 256 as int32_t {
        return bufninit(2 as uint32_t, val / 256 as int32_t, val % 256 as int32_t);
    } else {
        return bufninit(1 as uint32_t, val);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_encodeCffInteger(mut val: int32_t) -> *mut caryll_Buffer {
    if val >= -(107 as int32_t) && val <= 107 as int32_t {
        return bufninit(1 as uint32_t, val + 139 as int32_t);
    } else if val >= 108 as int32_t && val <= 1131 as int32_t {
        val = (val as ::core::ffi::c_int - 108 as ::core::ffi::c_int) as int32_t;
        return bufninit(
            2 as uint32_t,
            (val >> 8 as ::core::ffi::c_int) + 247 as int32_t,
            val & 0xff as int32_t,
        );
    } else if val >= -(1131 as int32_t) && val <= -(108 as int32_t) {
        val = -(108 as int32_t) - val;
        return bufninit(
            2 as uint32_t,
            (val >> 8 as ::core::ffi::c_int) + 251 as int32_t,
            val & 0xff as int32_t,
        );
    } else if val >= -(32768 as int32_t) && val < 32768 as int32_t {
        return bufninit(
            3 as uint32_t,
            28 as ::core::ffi::c_int,
            val >> 8 as ::core::ffi::c_int,
            val & 0xff as int32_t,
        );
    } else {
        return bufninit(
            5 as uint32_t,
            29 as ::core::ffi::c_int,
            val >> 24 as ::core::ffi::c_int & 0xff as int32_t,
            val >> 16 as ::core::ffi::c_int & 0xff as int32_t,
            val >> 8 as ::core::ffi::c_int & 0xff as int32_t,
            val & 0xff as int32_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_encodeCffFloat(mut val: ::core::ffi::c_double) -> *mut caryll_Buffer {
    let mut blob: *mut caryll_Buffer = bufnew();
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0 as uint32_t;
    let mut temp: [uint8_t; 32] = [
        0 as ::core::ffi::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if val == 0.0f64 {
        (*blob).size = 2 as size_t;
        (*blob).data = __caryll_allocate_clean(
            (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*blob).size),
            56 as ::core::ffi::c_ulong,
        ) as *mut uint8_t;
        *(*blob).data.offset(0 as ::core::ffi::c_int as isize) = 30 as uint8_t;
        *(*blob).data.offset(1 as ::core::ffi::c_int as isize) = 0xf as uint8_t;
    } else {
        let mut niblen: uint32_t = 0 as uint32_t;
        let mut array: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        sprintf(
            &raw mut temp as *mut uint8_t as *mut ::core::ffi::c_char,
            b"%.13g\0" as *const u8 as *const ::core::ffi::c_char,
            val,
        );
        i = 0 as uint32_t;
        while (i as size_t) < strlen(&raw mut temp as *mut uint8_t as *mut ::core::ffi::c_char) {
            if temp[i as usize] as ::core::ffi::c_int == '.' as i32 {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int >= '0' as i32
                && temp[i as usize] as ::core::ffi::c_int <= '9' as i32
            {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as uint32_t) as usize] as ::core::ffi::c_int == '-' as i32
            {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(2 as uint32_t);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as uint32_t) as usize] as ::core::ffi::c_int == '+' as i32
            {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(2 as uint32_t);
            } else if temp[i as usize] as ::core::ffi::c_int == '-' as i32 {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(1);
            }
        }
        (*blob).size = (2 as uint32_t).wrapping_add(niblen.wrapping_div(2 as uint32_t)) as size_t;
        (*blob).data = __caryll_allocate_clean(
            (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul((*blob).size),
            78 as ::core::ffi::c_ulong,
        ) as *mut uint8_t;
        *(*blob).data.offset(0 as ::core::ffi::c_int as isize) = 30 as uint8_t;
        if niblen.wrapping_rem(2 as uint32_t) != 0 as uint32_t {
            array = __caryll_allocate_clean(
                (::core::mem::size_of::<uint8_t>() as size_t)
                    .wrapping_mul(niblen.wrapping_add(1 as uint32_t) as size_t),
                82 as ::core::ffi::c_ulong,
            ) as *mut uint8_t;
            *array.offset(niblen as isize) = 0xf as uint8_t;
        } else {
            array = __caryll_allocate_clean(
                (::core::mem::size_of::<uint8_t>() as size_t)
                    .wrapping_mul(niblen.wrapping_add(2 as uint32_t) as size_t),
                85 as ::core::ffi::c_ulong,
            ) as *mut uint8_t;
            *array.offset(niblen.wrapping_add(1 as uint32_t) as isize) = 0xf as uint8_t;
            *array.offset(niblen as isize) = 0xf as uint8_t;
        }
        i = 0 as uint32_t;
        while (i as size_t) < strlen(&raw mut temp as *mut uint8_t as *mut ::core::ffi::c_char) {
            if temp[i as usize] as ::core::ffi::c_int == '.' as i32 {
                let fresh0 = j;
                j = j.wrapping_add(1);
                *array.offset(fresh0 as isize) = 0xa as uint8_t;
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int >= '0' as i32
                && temp[i as usize] as ::core::ffi::c_int <= '9' as i32
            {
                let fresh1 = j;
                j = j.wrapping_add(1);
                *array.offset(fresh1 as isize) =
                    (temp[i as usize] as ::core::ffi::c_int - '0' as i32) as uint8_t;
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as uint32_t) as usize] as ::core::ffi::c_int == '-' as i32
            {
                let fresh2 = j;
                j = j.wrapping_add(1);
                *array.offset(fresh2 as isize) = 0xc as uint8_t;
                i = i.wrapping_add(2 as uint32_t);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as uint32_t) as usize] as ::core::ffi::c_int == '+' as i32
            {
                let fresh3 = j;
                j = j.wrapping_add(1);
                *array.offset(fresh3 as isize) = 0xb as uint8_t;
                i = i.wrapping_add(2 as uint32_t);
            } else if temp[i as usize] as ::core::ffi::c_int == '-' as i32 {
                let fresh4 = j;
                j = j.wrapping_add(1);
                *array.offset(fresh4 as isize) = 0xe as uint8_t;
                i = i.wrapping_add(1);
            }
        }
        i = 1 as uint32_t;
        while (i as size_t) < (*blob).size {
            *(*blob).data.offset(i as isize) = (*array
                .offset(i.wrapping_sub(1 as uint32_t).wrapping_mul(2 as uint32_t) as isize)
                as ::core::ffi::c_int
                * 16 as ::core::ffi::c_int
                + *array.offset(
                    i.wrapping_sub(1 as uint32_t)
                        .wrapping_mul(2 as uint32_t)
                        .wrapping_add(1 as uint32_t) as isize,
                ) as ::core::ffi::c_int) as uint8_t;
            i = i.wrapping_add(1);
        }
        free(array as *mut ::core::ffi::c_void);
        array = ::core::ptr::null_mut::<uint8_t>();
    }
    return blob;
}
#[no_mangle]
pub unsafe extern "C" fn cff_decodeCS2Token(
    mut start: *const uint8_t,
    mut val: *mut cff_Value,
) -> uint32_t {
    let mut advance: uint32_t = 0 as uint32_t;
    if *start as ::core::ffi::c_int <= 27 as ::core::ffi::c_int {
        (*val).t = CS2_OPERATOR;
        if *start as ::core::ffi::c_int <= 11 as ::core::ffi::c_int {
            (*val).c2rust_unnamed.i = *start as int32_t;
            advance = 1 as uint32_t;
        } else if *start as ::core::ffi::c_int == 12 as ::core::ffi::c_int {
            (*val).c2rust_unnamed.i = ((*start as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | *start.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as int32_t;
            advance = 2 as uint32_t;
        } else if *start as ::core::ffi::c_int >= 13 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 18 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i = *start as int32_t;
            advance = 1 as uint32_t;
        } else if *start as ::core::ffi::c_int >= 19 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 20 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i = *start as int32_t;
            advance = 1 as uint32_t;
        } else if *start as ::core::ffi::c_int >= 21 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 27 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i = *start as int32_t;
            advance = 1 as uint32_t;
        }
    } else if *start as ::core::ffi::c_int == 28 as ::core::ffi::c_int {
        (*val).t = CS2_OPERAND;
        (*val).c2rust_unnamed.i = ((*start.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *start.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as int16_t as int32_t;
        advance = 3 as uint32_t;
    } else if *start as ::core::ffi::c_int >= 29 as ::core::ffi::c_int
        && *start as ::core::ffi::c_int <= 31 as ::core::ffi::c_int
    {
        (*val).t = CS2_OPERATOR;
        (*val).c2rust_unnamed.i = *start as int32_t;
        advance = 1 as uint32_t;
    } else if *start as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
        && *start as ::core::ffi::c_int <= 254 as ::core::ffi::c_int
    {
        (*val).t = CS2_OPERAND;
        if *start as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 246 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i =
                (*start as ::core::ffi::c_int - 139 as ::core::ffi::c_int) as int32_t;
            advance = 1 as uint32_t;
        } else if *start as ::core::ffi::c_int >= 247 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 250 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i = ((*start as ::core::ffi::c_int - 247 as ::core::ffi::c_int)
                * 256 as ::core::ffi::c_int
                + *start.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 108 as ::core::ffi::c_int) as int32_t;
            advance = 2 as uint32_t;
        } else if *start as ::core::ffi::c_int >= 251 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 254 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i = (-((*start as ::core::ffi::c_int
                - 251 as ::core::ffi::c_int)
                * 256 as ::core::ffi::c_int)
                - *start.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 108 as ::core::ffi::c_int) as int32_t;
            advance = 2 as uint32_t;
        }
    } else if *start as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
        (*val).t = CS2_FRACTION;
        let mut integerPart: int16_t = ((*start.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *start.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as int16_t;
        let mut fractionPart: uint16_t = ((*start.offset(3 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *start.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint16_t;
        (*val).c2rust_unnamed.d = integerPart as ::core::ffi::c_int as ::core::ffi::c_double
            + fractionPart as ::core::ffi::c_int as ::core::ffi::c_double / 65536.0f64;
        advance = 5 as uint32_t;
    }
    if (*val).t as ::core::ffi::c_uint == CS2_OPERAND as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*val).c2rust_unnamed.d = (*val).c2rust_unnamed.i as ::core::ffi::c_double;
        (*val).t = CS2_FRACTION;
    }
    return advance;
}
unsafe extern "C" fn cff_dec_i(mut start: *const uint8_t, mut val: *mut cff_Value) -> uint32_t {
    let mut b0: uint8_t = *start;
    let mut b1: uint8_t = 0;
    let mut b2: uint8_t = 0;
    let mut b3: uint8_t = 0;
    let mut b4: uint8_t = 0;
    let mut len: uint32_t = 0 as uint32_t;
    if b0 as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 246 as ::core::ffi::c_int
    {
        (*val).c2rust_unnamed.i = (b0 as ::core::ffi::c_int - 139 as ::core::ffi::c_int) as int32_t;
        len = 1 as uint32_t;
    } else if b0 as ::core::ffi::c_int >= 247 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 250 as ::core::ffi::c_int
    {
        b1 = *start.offset(1 as ::core::ffi::c_int as isize);
        (*val).c2rust_unnamed.i = ((b0 as ::core::ffi::c_int - 247 as ::core::ffi::c_int)
            * 256 as ::core::ffi::c_int
            + b1 as ::core::ffi::c_int
            + 108 as ::core::ffi::c_int) as int32_t;
        len = 2 as uint32_t;
    } else if b0 as ::core::ffi::c_int >= 251 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 254 as ::core::ffi::c_int
    {
        b1 = *start.offset(1 as ::core::ffi::c_int as isize);
        (*val).c2rust_unnamed.i = (-(b0 as ::core::ffi::c_int - 251 as ::core::ffi::c_int)
            * 256 as ::core::ffi::c_int
            - b1 as ::core::ffi::c_int
            - 108 as ::core::ffi::c_int) as int32_t;
        len = 2 as uint32_t;
    } else if b0 as ::core::ffi::c_int == 28 as ::core::ffi::c_int {
        b1 = *start.offset(1 as ::core::ffi::c_int as isize);
        b2 = *start.offset(2 as ::core::ffi::c_int as isize);
        (*val).c2rust_unnamed.i = ((b1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | b2 as ::core::ffi::c_int) as int32_t;
        len = 3 as uint32_t;
    } else if b0 as ::core::ffi::c_int == 29 as ::core::ffi::c_int {
        b1 = *start.offset(1 as ::core::ffi::c_int as isize);
        b2 = *start.offset(2 as ::core::ffi::c_int as isize);
        b3 = *start.offset(3 as ::core::ffi::c_int as isize);
        b4 = *start.offset(4 as ::core::ffi::c_int as isize);
        (*val).c2rust_unnamed.i = ((b1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
            | (b2 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | (b3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | b4 as ::core::ffi::c_int) as int32_t;
        len = 5 as uint32_t;
    }
    (*val).t = cff_INTEGER;
    return len;
}
static mut nibble_attr: [::core::ffi::c_int; 15] = [
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
];
static mut nibble_symb: [*const ::core::ffi::c_char; 15] = [
    b"0\0" as *const u8 as *const ::core::ffi::c_char,
    b"1\0" as *const u8 as *const ::core::ffi::c_char,
    b"2\0" as *const u8 as *const ::core::ffi::c_char,
    b"3\0" as *const u8 as *const ::core::ffi::c_char,
    b"4\0" as *const u8 as *const ::core::ffi::c_char,
    b"5\0" as *const u8 as *const ::core::ffi::c_char,
    b"6\0" as *const u8 as *const ::core::ffi::c_char,
    b"7\0" as *const u8 as *const ::core::ffi::c_char,
    b"8\0" as *const u8 as *const ::core::ffi::c_char,
    b"9\0" as *const u8 as *const ::core::ffi::c_char,
    b".\0" as *const u8 as *const ::core::ffi::c_char,
    b"E\0" as *const u8 as *const ::core::ffi::c_char,
    b"E-\0" as *const u8 as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"-\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn cff_dec_r(mut start: *const uint8_t, mut val: *mut cff_Value) -> uint32_t {
    let mut restr: [uint8_t; 72] = [
        0 as ::core::ffi::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut str_len: size_t = 0 as size_t;
    let mut len: uint32_t = 0;
    let mut a: uint8_t = 0;
    let mut b: uint8_t = 0;
    let mut nibst: *const uint8_t = start.offset(1 as ::core::ffi::c_int as isize);
    loop {
        a = (*nibst as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as uint8_t;
        b = (*nibst as ::core::ffi::c_int % 16 as ::core::ffi::c_int) as uint8_t;
        if !(a as ::core::ffi::c_int != 15 as ::core::ffi::c_int) {
            break;
        }
        str_len = str_len.wrapping_add(nibble_attr[a as usize] as size_t);
        if !(b as ::core::ffi::c_int != 15 as ::core::ffi::c_int) {
            break;
        }
        str_len = str_len.wrapping_add(nibble_attr[b as usize] as size_t);
        nibst = nibst.offset(1);
    }
    len = (nibst.offset_from(start) as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as uint32_t;
    nibst = start.offset(1 as ::core::ffi::c_int as isize);
    loop {
        a = (*nibst as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as uint8_t;
        b = (*nibst as ::core::ffi::c_int % 16 as ::core::ffi::c_int) as uint8_t;
        if !(a as ::core::ffi::c_int != 0xf as ::core::ffi::c_int) {
            break;
        }
        strcat(
            &raw mut restr as *mut uint8_t as *mut ::core::ffi::c_char,
            nibble_symb[a as usize],
        );
        if !(b as ::core::ffi::c_int != 0xf as ::core::ffi::c_int) {
            break;
        }
        strcat(
            &raw mut restr as *mut uint8_t as *mut ::core::ffi::c_char,
            nibble_symb[b as usize],
        );
        nibst = nibst.offset(1);
    }
    (*val).c2rust_unnamed.d = atof(&raw mut restr as *mut uint8_t as *mut ::core::ffi::c_char);
    (*val).t = cff_DOUBLE;
    return len;
}
unsafe extern "C" fn cff_dec_o(mut start: *const uint8_t, mut val: *mut cff_Value) -> uint32_t {
    let mut b0: uint8_t = *start;
    let mut b1: uint8_t = 0;
    let mut len: uint32_t = 0 as uint32_t;
    if b0 as ::core::ffi::c_int <= 21 as ::core::ffi::c_int {
        if b0 as ::core::ffi::c_int != 12 as ::core::ffi::c_int {
            (*val).c2rust_unnamed.i = b0 as int32_t;
            len = 1 as uint32_t;
        } else {
            b1 = *start.offset(1 as ::core::ffi::c_int as isize);
            (*val).c2rust_unnamed.i = (b0 as ::core::ffi::c_int * 256 as ::core::ffi::c_int
                + b1 as ::core::ffi::c_int) as int32_t;
            len = 2 as uint32_t;
        }
    }
    (*val).t = cff_OPERATOR;
    return len;
}
unsafe extern "C" fn cff_dec_e(mut start: *const uint8_t, mut val: *mut cff_Value) -> uint32_t {
    printf(
        b"Undefined Byte in CFF: %d.\n\0" as *const u8 as *const ::core::ffi::c_char,
        *start as ::core::ffi::c_int,
    );
    (*val).c2rust_unnamed.i = *start as int32_t;
    (*val).t = cff_INTEGER;
    return 1 as uint32_t;
}
static mut _de_t2: [Option<unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t>; 256] = {
    [
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_o as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_e as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_e as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_e as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_e as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_e as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_e as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_r as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_e as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_i as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
        Some(cff_dec_e as unsafe extern "C" fn(*const uint8_t, *mut cff_Value) -> uint32_t),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn cff_decodeCffToken(
    mut start: *const uint8_t,
    mut val: *mut cff_Value,
) -> uint32_t {
    return _de_t2[*start as usize].expect("non-null function pointer")(start, val);
}
