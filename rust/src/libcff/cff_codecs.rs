#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, printf, sprintf, strcat, strlen, strtod};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::buffer::{bufninit, Buffer};
use crate::libcff::cff_value::{CS2_FRACTION, CS2_OPERAND, CS2_OPERATOR, CffValueType, CffValue};
use crate::support::{NULL};
use crate::support::buffer::{bufnew};
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double {
    return strtod(__nptr, NULL as *mut *mut ::core::ffi::c_char);
}
pub unsafe extern "C" fn cff_encode_cff_operator(mut val: i32) -> *mut Buffer {
    if val > 256 as i32 {
        return bufninit(&[(val / 256 as i32) as u8, (val % 256 as i32) as u8]);
    } else {
        return bufninit(&[val as u8]);
    };
}
pub unsafe extern "C" fn cff_encode_cff_integer(mut val: i32) -> *mut Buffer {
    if val >= -(107 as i32) && val <= 107 as i32 {
        return bufninit(&[(val + 139 as i32) as u8]);
    } else if val >= 108 as i32 && val <= 1131 as i32 {
        val = (val as ::core::ffi::c_int - 108 as ::core::ffi::c_int) as i32;
        return bufninit(&[((val >> 8 as ::core::ffi::c_int) + 247 as i32) as u8, (val & 0xff as i32) as u8]);
    } else if val >= -(1131 as i32) && val <= -(108 as i32) {
        val = -(108 as i32) - val;
        return bufninit(&[((val >> 8 as ::core::ffi::c_int) + 251 as i32) as u8, (val & 0xff as i32) as u8]);
    } else if val >= -(32768 as i32) && val < 32768 as i32 {
        return bufninit(&[28 as u8, (val >> 8 as ::core::ffi::c_int) as u8, (val & 0xff as i32) as u8]);
    } else {
        return bufninit(&[29 as u8, (val >> 24 as ::core::ffi::c_int & 0xff as i32) as u8, (val >> 16 as ::core::ffi::c_int & 0xff as i32) as u8, (val >> 8 as ::core::ffi::c_int & 0xff as i32) as u8, (val & 0xff as i32) as u8]);
    };
}
pub unsafe extern "C" fn cff_encode_cff_float(mut val: ::core::ffi::c_double) -> *mut Buffer {
    let mut blob: *mut Buffer = bufnew();
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
pub unsafe extern "C" fn cff_decode_cs2_token(
    mut start: *const u8,
    mut val: *mut CffValue,
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
        let mut integer_part: i16 = ((*start.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *start.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as i16;
        let mut fraction_part: u16 = ((*start.offset(3 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *start.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as u16;
        (*val).c2rust_unnamed.d = integer_part as ::core::ffi::c_int as ::core::ffi::c_double
            + fraction_part as ::core::ffi::c_int as ::core::ffi::c_double / 65536.0f64;
        advance = 5 as u32;
    }
    if (*val).t as ::core::ffi::c_uint == CS2_OPERAND as ::core::ffi::c_int as ::core::ffi::c_uint {
        (*val).c2rust_unnamed.d = (*val).c2rust_unnamed.i as ::core::ffi::c_double;
        (*val).t = CS2_FRACTION;
    }
    return advance;
}
unsafe extern "C" fn cff_dec_i(mut start: *const u8, mut val: *mut CffValue) -> u32 {
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
    (*val).t = CffValueType::Integer;
    return len;
}
static NIBBLE_ATTR: [::core::ffi::c_int; 15] = [
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
static NIBBLE_SYMB: [&::core::ffi::CStr; 15] = [
    c"0",
    c"1",
    c"2",
    c"3",
    c"4",
    c"5",
    c"6",
    c"7",
    c"8",
    c"9",
    c".",
    c"E",
    c"E-",
    c"",
    c"-",
];
unsafe extern "C" fn cff_dec_r(mut start: *const u8, mut val: *mut CffValue) -> u32 {
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
        str_len = str_len.wrapping_add(NIBBLE_ATTR[a as usize] as usize);
        if !(b as ::core::ffi::c_int != 15 as ::core::ffi::c_int) {
            break;
        }
        str_len = str_len.wrapping_add(NIBBLE_ATTR[b as usize] as usize);
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
            NIBBLE_SYMB[a as usize].as_ptr(),
        );
        if !(b as ::core::ffi::c_int != 0xf as ::core::ffi::c_int) {
            break;
        }
        strcat(
            &raw mut restr as *mut u8 as *mut ::core::ffi::c_char,
            NIBBLE_SYMB[b as usize].as_ptr(),
        );
        nibst = nibst.offset(1);
    }
    (*val).c2rust_unnamed.d = atof(&raw mut restr as *mut u8 as *mut ::core::ffi::c_char);
    (*val).t = CffValueType::Double;
    return len;
}
unsafe extern "C" fn cff_dec_o(mut start: *const u8, mut val: *mut CffValue) -> u32 {
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
    (*val).t = CffValueType::Operator;
    return len;
}
unsafe extern "C" fn cff_dec_e(mut start: *const u8, mut val: *mut CffValue) -> u32 {
    printf(
        b"Undefined Byte in CFF: %d.\n\0" as *const u8 as *const ::core::ffi::c_char,
        *start as ::core::ffi::c_int,
    );
    (*val).c2rust_unnamed.i = *start as i32;
    (*val).t = CffValueType::Integer;
    return 1 as u32;
}
static DE_T2: [Option<unsafe extern "C" fn(*const u8, *mut CffValue) -> u32>; 256] = {
    [
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_o as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_r as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_i as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
        Some(cff_dec_e as unsafe extern "C" fn(*const u8, *mut CffValue) -> u32),
    ]
};
pub unsafe extern "C" fn cff_decode_cff_token(
    mut start: *const u8,
    mut val: *mut CffValue,
) -> u32 {
    return DE_T2[*start as usize].expect("non-null function pointer")(start, val);
}
