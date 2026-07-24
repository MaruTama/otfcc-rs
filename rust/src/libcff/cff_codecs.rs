use libc::{free, printf, sprintf, strcat, strlen, strtod};
extern "C" {
    fn bufnew() -> *mut caryll_Buffer;
    fn bufninit(n: u32, ...) -> *mut caryll_Buffer;
}


use crate::support::alloc::{__caryll_allocate_clean};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct caryll_Buffer {
    pub cursor: usize,
    pub size: usize,
    pub free: usize,
    pub data: *mut u8,
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
    pub i: i32,
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
pub unsafe extern "C" fn cff_encodeCffOperator(mut val: i32) -> *mut caryll_Buffer {
    if val > 256 as i32 {
        return bufninit(2 as u32, val / 256 as i32, val % 256 as i32);
    } else {
        return bufninit(1 as u32, val);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_encodeCffInteger(mut val: i32) -> *mut caryll_Buffer {
    if val >= -(107 as i32) && val <= 107 as i32 {
        return bufninit(1 as u32, val + 139 as i32);
    } else if val >= 108 as i32 && val <= 1131 as i32 {
        val = (val as ::core::ffi::c_int - 108 as ::core::ffi::c_int) as i32;
        return bufninit(
            2 as u32,
            (val >> 8 as ::core::ffi::c_int) + 247 as i32,
            val & 0xff as i32,
        );
    } else if val >= -(1131 as i32) && val <= -(108 as i32) {
        val = -(108 as i32) - val;
        return bufninit(
            2 as u32,
            (val >> 8 as ::core::ffi::c_int) + 251 as i32,
            val & 0xff as i32,
        );
    } else if val >= -(32768 as i32) && val < 32768 as i32 {
        return bufninit(
            3 as u32,
            28 as ::core::ffi::c_int,
            val >> 8 as ::core::ffi::c_int,
            val & 0xff as i32,
        );
    } else {
        return bufninit(
            5 as u32,
            29 as ::core::ffi::c_int,
            val >> 24 as ::core::ffi::c_int & 0xff as i32,
            val >> 16 as ::core::ffi::c_int & 0xff as i32,
            val >> 8 as ::core::ffi::c_int & 0xff as i32,
            val & 0xff as i32,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn cff_encodeCffFloat(mut val: ::core::ffi::c_double) -> *mut caryll_Buffer {
    let mut blob: *mut caryll_Buffer = bufnew();
    let mut i: u32 = 0;
    let mut j: u32 = 0 as u32;
    let mut temp: [u8; 32] = [
        0 as ::core::ffi::c_int as u8,
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
        (*blob).size = 2 as usize;
        (*blob).data = __caryll_allocate_clean(
            (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob).size),
            56 as ::core::ffi::c_ulong,
        ) as *mut u8;
        *(*blob).data.offset(0 as ::core::ffi::c_int as isize) = 30 as u8;
        *(*blob).data.offset(1 as ::core::ffi::c_int as isize) = 0xf as u8;
    } else {
        let mut niblen: u32 = 0 as u32;
        let mut array: *mut u8 = ::core::ptr::null_mut::<u8>();
        sprintf(
            &raw mut temp as *mut u8 as *mut ::core::ffi::c_char,
            b"%.13g\0" as *const u8 as *const ::core::ffi::c_char,
            val,
        );
        i = 0 as u32;
        while (i as usize) < strlen(&raw mut temp as *mut u8 as *mut ::core::ffi::c_char) {
            if temp[i as usize] as ::core::ffi::c_int == '.' as i32 {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int >= '0' as i32
                && temp[i as usize] as ::core::ffi::c_int <= '9' as i32
            {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as u32) as usize] as ::core::ffi::c_int == '-' as i32
            {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(2 as u32);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as u32) as usize] as ::core::ffi::c_int == '+' as i32
            {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(2 as u32);
            } else if temp[i as usize] as ::core::ffi::c_int == '-' as i32 {
                niblen = niblen.wrapping_add(1);
                i = i.wrapping_add(1);
            }
        }
        (*blob).size = (2 as u32).wrapping_add(niblen.wrapping_div(2 as u32)) as usize;
        (*blob).data = __caryll_allocate_clean(
            (::core::mem::size_of::<u8>() as usize).wrapping_mul((*blob).size),
            78 as ::core::ffi::c_ulong,
        ) as *mut u8;
        *(*blob).data.offset(0 as ::core::ffi::c_int as isize) = 30 as u8;
        if niblen.wrapping_rem(2 as u32) != 0 as u32 {
            array = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize)
                    .wrapping_mul(niblen.wrapping_add(1 as u32) as usize),
                82 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *array.offset(niblen as isize) = 0xf as u8;
        } else {
            array = __caryll_allocate_clean(
                (::core::mem::size_of::<u8>() as usize)
                    .wrapping_mul(niblen.wrapping_add(2 as u32) as usize),
                85 as ::core::ffi::c_ulong,
            ) as *mut u8;
            *array.offset(niblen.wrapping_add(1 as u32) as isize) = 0xf as u8;
            *array.offset(niblen as isize) = 0xf as u8;
        }
        i = 0 as u32;
        while (i as usize) < strlen(&raw mut temp as *mut u8 as *mut ::core::ffi::c_char) {
            if temp[i as usize] as ::core::ffi::c_int == '.' as i32 {
                let fresh0 = j;
                j = j.wrapping_add(1);
                *array.offset(fresh0 as isize) = 0xa as u8;
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int >= '0' as i32
                && temp[i as usize] as ::core::ffi::c_int <= '9' as i32
            {
                let fresh1 = j;
                j = j.wrapping_add(1);
                *array.offset(fresh1 as isize) =
                    (temp[i as usize] as ::core::ffi::c_int - '0' as i32) as u8;
                i = i.wrapping_add(1);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as u32) as usize] as ::core::ffi::c_int == '-' as i32
            {
                let fresh2 = j;
                j = j.wrapping_add(1);
                *array.offset(fresh2 as isize) = 0xc as u8;
                i = i.wrapping_add(2 as u32);
            } else if temp[i as usize] as ::core::ffi::c_int == 'e' as i32
                && temp[i.wrapping_add(1 as u32) as usize] as ::core::ffi::c_int == '+' as i32
            {
                let fresh3 = j;
                j = j.wrapping_add(1);
                *array.offset(fresh3 as isize) = 0xb as u8;
                i = i.wrapping_add(2 as u32);
            } else if temp[i as usize] as ::core::ffi::c_int == '-' as i32 {
                let fresh4 = j;
                j = j.wrapping_add(1);
                *array.offset(fresh4 as isize) = 0xe as u8;
                i = i.wrapping_add(1);
            }
        }
        i = 1 as u32;
        while (i as usize) < (*blob).size {
            *(*blob).data.offset(i as isize) = (*array
                .offset(i.wrapping_sub(1 as u32).wrapping_mul(2 as u32) as isize)
                as ::core::ffi::c_int
                * 16 as ::core::ffi::c_int
                + *array.offset(
                    i.wrapping_sub(1 as u32)
                        .wrapping_mul(2 as u32)
                        .wrapping_add(1 as u32) as isize,
                ) as ::core::ffi::c_int) as u8;
            i = i.wrapping_add(1);
        }
        free(array as *mut ::core::ffi::c_void);
        array = ::core::ptr::null_mut::<u8>();
    }
    return blob;
}
#[no_mangle]
pub unsafe extern "C" fn cff_decodeCS2Token(
    mut start: *const u8,
    mut val: *mut cff_Value,
) -> u32 {
    let mut advance: u32 = 0 as u32;
    if *start as ::core::ffi::c_int <= 27 as ::core::ffi::c_int {
        (*val).t = CS2_OPERATOR;
        if *start as ::core::ffi::c_int <= 11 as ::core::ffi::c_int {
            (*val).c2rust_unnamed.i = *start as i32;
            advance = 1 as u32;
        } else if *start as ::core::ffi::c_int == 12 as ::core::ffi::c_int {
            (*val).c2rust_unnamed.i = ((*start as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | *start.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as i32;
            advance = 2 as u32;
        } else if *start as ::core::ffi::c_int >= 13 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 18 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i = *start as i32;
            advance = 1 as u32;
        } else if *start as ::core::ffi::c_int >= 19 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 20 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i = *start as i32;
            advance = 1 as u32;
        } else if *start as ::core::ffi::c_int >= 21 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 27 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i = *start as i32;
            advance = 1 as u32;
        }
    } else if *start as ::core::ffi::c_int == 28 as ::core::ffi::c_int {
        (*val).t = CS2_OPERAND;
        (*val).c2rust_unnamed.i = ((*start.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *start.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as i16 as i32;
        advance = 3 as u32;
    } else if *start as ::core::ffi::c_int >= 29 as ::core::ffi::c_int
        && *start as ::core::ffi::c_int <= 31 as ::core::ffi::c_int
    {
        (*val).t = CS2_OPERATOR;
        (*val).c2rust_unnamed.i = *start as i32;
        advance = 1 as u32;
    } else if *start as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
        && *start as ::core::ffi::c_int <= 254 as ::core::ffi::c_int
    {
        (*val).t = CS2_OPERAND;
        if *start as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 246 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i =
                (*start as ::core::ffi::c_int - 139 as ::core::ffi::c_int) as i32;
            advance = 1 as u32;
        } else if *start as ::core::ffi::c_int >= 247 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 250 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i = ((*start as ::core::ffi::c_int - 247 as ::core::ffi::c_int)
                * 256 as ::core::ffi::c_int
                + *start.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 108 as ::core::ffi::c_int) as i32;
            advance = 2 as u32;
        } else if *start as ::core::ffi::c_int >= 251 as ::core::ffi::c_int
            && *start as ::core::ffi::c_int <= 254 as ::core::ffi::c_int
        {
            (*val).c2rust_unnamed.i = (-((*start as ::core::ffi::c_int
                - 251 as ::core::ffi::c_int)
                * 256 as ::core::ffi::c_int)
                - *start.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 108 as ::core::ffi::c_int) as i32;
            advance = 2 as u32;
        }
    } else if *start as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
        (*val).t = CS2_FRACTION;
        let mut integerPart: i16 = ((*start.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *start.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as i16;
        let mut fractionPart: u16 = ((*start.offset(3 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *start.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as u16;
        (*val).c2rust_unnamed.d = integerPart as ::core::ffi::c_int as ::core::ffi::c_double
            + fractionPart as ::core::ffi::c_int as ::core::ffi::c_double / 65536.0f64;
        advance = 5 as u32;
    }
    if (*val).t as ::core::ffi::c_uint == CS2_OPERAND as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*val).c2rust_unnamed.d = (*val).c2rust_unnamed.i as ::core::ffi::c_double;
        (*val).t = CS2_FRACTION;
    }
    return advance;
}
unsafe extern "C" fn cff_dec_i(mut start: *const u8, mut val: *mut cff_Value) -> u32 {
    let mut b0: u8 = *start;
    let mut b1: u8 = 0;
    let mut b2: u8 = 0;
    let mut b3: u8 = 0;
    let mut b4: u8 = 0;
    let mut len: u32 = 0 as u32;
    if b0 as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 246 as ::core::ffi::c_int
    {
        (*val).c2rust_unnamed.i = (b0 as ::core::ffi::c_int - 139 as ::core::ffi::c_int) as i32;
        len = 1 as u32;
    } else if b0 as ::core::ffi::c_int >= 247 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 250 as ::core::ffi::c_int
    {
        b1 = *start.offset(1 as ::core::ffi::c_int as isize);
        (*val).c2rust_unnamed.i = ((b0 as ::core::ffi::c_int - 247 as ::core::ffi::c_int)
            * 256 as ::core::ffi::c_int
            + b1 as ::core::ffi::c_int
            + 108 as ::core::ffi::c_int) as i32;
        len = 2 as u32;
    } else if b0 as ::core::ffi::c_int >= 251 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 254 as ::core::ffi::c_int
    {
        b1 = *start.offset(1 as ::core::ffi::c_int as isize);
        (*val).c2rust_unnamed.i = (-(b0 as ::core::ffi::c_int - 251 as ::core::ffi::c_int)
            * 256 as ::core::ffi::c_int
            - b1 as ::core::ffi::c_int
            - 108 as ::core::ffi::c_int) as i32;
        len = 2 as u32;
    } else if b0 as ::core::ffi::c_int == 28 as ::core::ffi::c_int {
        b1 = *start.offset(1 as ::core::ffi::c_int as isize);
        b2 = *start.offset(2 as ::core::ffi::c_int as isize);
        (*val).c2rust_unnamed.i = ((b1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | b2 as ::core::ffi::c_int) as i32;
        len = 3 as u32;
    } else if b0 as ::core::ffi::c_int == 29 as ::core::ffi::c_int {
        b1 = *start.offset(1 as ::core::ffi::c_int as isize);
        b2 = *start.offset(2 as ::core::ffi::c_int as isize);
        b3 = *start.offset(3 as ::core::ffi::c_int as isize);
        b4 = *start.offset(4 as ::core::ffi::c_int as isize);
        (*val).c2rust_unnamed.i = ((b1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int
            | (b2 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | (b3 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | b4 as ::core::ffi::c_int) as i32;
        len = 5 as u32;
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
unsafe extern "C" fn cff_dec_r(mut start: *const u8, mut val: *mut cff_Value) -> u32 {
    let mut restr: [u8; 72] = [
        0 as ::core::ffi::c_int as u8,
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
    let mut str_len: usize = 0 as usize;
    let mut len: u32 = 0;
    let mut a: u8 = 0;
    let mut b: u8 = 0;
    let mut nibst: *const u8 = start.offset(1 as ::core::ffi::c_int as isize);
    loop {
        a = (*nibst as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as u8;
        b = (*nibst as ::core::ffi::c_int % 16 as ::core::ffi::c_int) as u8;
        if !(a as ::core::ffi::c_int != 15 as ::core::ffi::c_int) {
            break;
        }
        str_len = str_len.wrapping_add(nibble_attr[a as usize] as usize);
        if !(b as ::core::ffi::c_int != 15 as ::core::ffi::c_int) {
            break;
        }
        str_len = str_len.wrapping_add(nibble_attr[b as usize] as usize);
        nibst = nibst.offset(1);
    }
    len = (nibst.offset_from(start) as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as u32;
    nibst = start.offset(1 as ::core::ffi::c_int as isize);
    loop {
        a = (*nibst as ::core::ffi::c_int / 16 as ::core::ffi::c_int) as u8;
        b = (*nibst as ::core::ffi::c_int % 16 as ::core::ffi::c_int) as u8;
        if !(a as ::core::ffi::c_int != 0xf as ::core::ffi::c_int) {
            break;
        }
        strcat(
            &raw mut restr as *mut u8 as *mut ::core::ffi::c_char,
            nibble_symb[a as usize],
        );
        if !(b as ::core::ffi::c_int != 0xf as ::core::ffi::c_int) {
            break;
        }
        strcat(
            &raw mut restr as *mut u8 as *mut ::core::ffi::c_char,
            nibble_symb[b as usize],
        );
        nibst = nibst.offset(1);
    }
    (*val).c2rust_unnamed.d = atof(&raw mut restr as *mut u8 as *mut ::core::ffi::c_char);
    (*val).t = cff_DOUBLE;
    return len;
}
unsafe extern "C" fn cff_dec_o(mut start: *const u8, mut val: *mut cff_Value) -> u32 {
    let mut b0: u8 = *start;
    let mut b1: u8 = 0;
    let mut len: u32 = 0 as u32;
    if b0 as ::core::ffi::c_int <= 21 as ::core::ffi::c_int {
        if b0 as ::core::ffi::c_int != 12 as ::core::ffi::c_int {
            (*val).c2rust_unnamed.i = b0 as i32;
            len = 1 as u32;
        } else {
            b1 = *start.offset(1 as ::core::ffi::c_int as isize);
            (*val).c2rust_unnamed.i = (b0 as ::core::ffi::c_int * 256 as ::core::ffi::c_int
                + b1 as ::core::ffi::c_int) as i32;
            len = 2 as u32;
        }
    }
    (*val).t = cff_OPERATOR;
    return len;
}
unsafe extern "C" fn cff_dec_e(mut start: *const u8, mut val: *mut cff_Value) -> u32 {
    printf(
        b"Undefined Byte in CFF: %d.\n\0" as *const u8 as *const ::core::ffi::c_char,
        *start as ::core::ffi::c_int,
    );
    (*val).c2rust_unnamed.i = *start as i32;
    (*val).t = cff_INTEGER;
    return 1 as u32;
}
static mut _de_t2: [Option<unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32>; 256] = {
    [
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_r as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut cff_Value) -> u32),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn cff_decodeCffToken(
    mut start: *const u8,
    mut val: *mut cff_Value,
) -> u32 {
    return _de_t2[*start as usize].expect("non-null function pointer")(start, val);
}
