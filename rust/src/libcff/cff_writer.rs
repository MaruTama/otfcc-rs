use crate::libcff::CffCharstringOperator;
use crate::support::buffer::bufwrite8;
use crate::support::buffer::{Buffer, bufninit, bufnwrite8};
unsafe extern "C" {
    fn modf(
        __x: ::core::ffi::c_double,
        __iptr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub unsafe fn cff_build_header() -> *mut Buffer {
    return unsafe { bufninit(&[1_u8, 0_u8, 4_u8, 4_u8]) };
}
pub unsafe fn cff_merge_cs2_operator(blob: *mut Buffer, val: CffCharstringOperator) {
    let val = val.0;
    unsafe {
        if val >= 0x100_i32 {
            bufnwrite8(
                blob,
                &[
                    (val >> 8 as ::core::ffi::c_int) as u8,
                    (val & 0xff_i32) as u8,
                ],
            );
        } else {
            bufnwrite8(blob, &[(val & 0xff_i32) as u8]);
        };
    }
}
pub unsafe fn cff_merge_cs2_int(blob: *mut Buffer, val: i32) {
    unsafe {
        if (-1131_i32..=-108_i32).contains(&val) {
            bufnwrite8(
                blob,
                &[
                    (((-108_i32 - val) / 256_i32 + 251_i32) as u8 as ::core::ffi::c_int)
                        as u8,
                    (((-108_i32 - val) % 256_i32) as u8 as ::core::ffi::c_int) as u8,
                ],
            );
        } else if (-107_i32..=107_i32).contains(&val) {
            bufnwrite8(
                blob,
                &[((val + 139_i32) as u8 as ::core::ffi::c_int) as u8],
            );
        } else if (108_i32..=1131_i32).contains(&val) {
            bufnwrite8(
                blob,
                &[
                    (((val - 108_i32) / 256_i32 + 247_i32) as u8 as ::core::ffi::c_int)
                        as u8,
                    (((val - 108_i32) % 256_i32) as u8 as ::core::ffi::c_int) as u8,
                ],
            );
        } else if (-32768_i32..=32767_i32).contains(&val) {
            bufnwrite8(
                blob,
                &[
                    28_u8,
                    ((val >> 8 as ::core::ffi::c_int) as u8 as ::core::ffi::c_int) as u8,
                    ((val << 8 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8
                        as ::core::ffi::c_int) as u8,
                ],
            );
        } else {
            cff_merge_cs2_int(blob, 0_i32);
        };
    }
}
unsafe fn merge_cs2_real(blob: *mut Buffer, val: ::core::ffi::c_double) {
    let integer_part: i16 = unsafe { floor(val) } as i16;
    let fraction_part: u16 =
        ((val - integer_part as ::core::ffi::c_int as ::core::ffi::c_double) * 65536.0f64) as u16;
    unsafe {
        bufnwrite8(
            blob,
            &[
                0xff_u8,
                (integer_part as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8,
                (integer_part as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u8,
                (fraction_part as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as u8,
                (fraction_part as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as u8,
            ],
        );
    }
}
pub unsafe fn cff_merge_cs2_operand(blob: *mut Buffer, val: ::core::ffi::c_double) {
    let mut intpart: ::core::ffi::c_double = 0.;
    unsafe {
        if modf(val, &raw mut intpart) == 0.0f64 {
            cff_merge_cs2_int(blob, intpart as i32);
        } else {
            merge_cs2_real(blob, val);
        };
    }
}
pub unsafe fn cff_merge_cs2_special(blob: *mut Buffer, val: u8) {
    unsafe { bufwrite8(blob, val) };
}
pub unsafe fn cff_build_offset(val: i32) -> *mut Buffer {
    return unsafe {
        bufninit(&[
            29_u8,
            (val >> 24 as ::core::ffi::c_int & 0xff_i32) as u8,
            (val >> 16 as ::core::ffi::c_int & 0xff_i32) as u8,
            (val >> 8 as ::core::ffi::c_int & 0xff_i32) as u8,
            (val & 0xff_i32) as u8,
        ])
    };
}
