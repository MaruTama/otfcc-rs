use crate::libcff::CffCharstringOperator;
use crate::support::buffer::Buffer;
unsafe extern "C" {
    fn modf(
        __x: ::core::ffi::c_double,
        __iptr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn floor(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
}
pub fn cff_build_header() -> Buffer {
    Buffer::from_bytes(&[1_u8, 0_u8, 4_u8, 4_u8])
}
pub fn cff_merge_cs2_operator(blob: &mut Buffer, val: CffCharstringOperator) {
    let val = val.0;
    if val >= 0x100_i32 {
        blob.write_bytes(&[(val >> 8_i32) as u8, (val & 0xff_i32) as u8]);
    } else {
        blob.write_bytes(&[(val & 0xff_i32) as u8]);
    };
}
pub fn cff_merge_cs2_int(blob: &mut Buffer, val: i32) {
    if (-1131_i32..=-108_i32).contains(&val) {
        blob.write_bytes(&[
            (((-108_i32 - val) / 256_i32 + 251_i32) as u8 as i32) as u8,
            (((-108_i32 - val) % 256_i32) as u8 as i32) as u8,
        ]);
    } else if (-107_i32..=107_i32).contains(&val) {
        blob.write_bytes(&[((val + 139_i32) as u8 as i32) as u8]);
    } else if (108_i32..=1131_i32).contains(&val) {
        blob.write_bytes(&[
            (((val - 108_i32) / 256_i32 + 247_i32) as u8 as i32) as u8,
            (((val - 108_i32) % 256_i32) as u8 as i32) as u8,
        ]);
    } else if (-32768_i32..=32767_i32).contains(&val) {
        blob.write_bytes(&[
            28_u8,
            ((val >> 8_i32) as u8 as i32) as u8,
            ((val << 8_i32 >> 8_i32) as u8 as i32) as u8,
        ]);
    } else {
        cff_merge_cs2_int(blob, 0_i32);
    };
}
fn merge_cs2_real(blob: &mut Buffer, val: ::core::ffi::c_double) {
    let integer_part: i16 = unsafe { floor(val) } as i16;
    let fraction_part: u16 =
        ((val - integer_part as i32 as ::core::ffi::c_double) * 65536.0f64) as u16;
    blob.write_bytes(&[
        0xff_u8,
        (integer_part as i32 >> 8_i32) as u8,
        (integer_part as i32 & 0xff_i32) as u8,
        (fraction_part as i32 >> 8_i32) as u8,
        (fraction_part as i32 & 0xff_i32) as u8,
    ]);
}
pub fn cff_merge_cs2_operand(blob: &mut Buffer, val: ::core::ffi::c_double) {
    let mut intpart: ::core::ffi::c_double = 0.;
    if unsafe { modf(val, &raw mut intpart) } == 0.0f64 {
        cff_merge_cs2_int(blob, intpart as i32);
    } else {
        merge_cs2_real(blob, val);
    };
}
pub fn cff_merge_cs2_special(blob: &mut Buffer, val: u8) {
    blob.write_u8(val);
}
pub fn cff_build_offset(val: i32) -> Buffer {
    Buffer::from_bytes(&[
        29_u8,
        (val >> 24_i32 & 0xff_i32) as u8,
        (val >> 16_i32 & 0xff_i32) as u8,
        (val >> 8_i32 & 0xff_i32) as u8,
        (val & 0xff_i32) as u8,
    ])
}
