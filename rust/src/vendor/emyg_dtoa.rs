#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::memmove;
#[derive(Copy, Clone)]
pub struct DiyFp {
    pub f: u64,
    pub e: i32,
}
static K_DIY_SIGNIFICAND_SIZE: i32 = 64_i32;
static K_DP_SIGNIFICAND_SIZE: i32 = 52_i32;
static K_DP_EXPONENT_BIAS: i32 =
    0x3ff_i32 + 52_i32;
static K_DP_MIN_EXPONENT: i32 =
    -0x3ff_i32 - 52_i32;
static K_DP_EXPONENT_MASK: u64 = (0x7ff00000_i32 as u64)
    << 32_i32
    | 0_i32 as u64;
static K_DP_SIGNIFICAND_MASK: u64 = (0xfffff_i32 as u64)
    << 32_i32
    | 0xffffffff as ::core::ffi::c_uint as u64;
static K_DP_HIDDEN_BIT: u64 = (0x100000_i32 as u64) << 32_i32
    | 0_i32 as u64;
#[inline]
unsafe fn diy_fp_from_parts(f: u64, e: i32) -> DiyFp {
    let mut fp: DiyFp = DiyFp { f: 0, e: 0 };
    fp.f = f;
    fp.e = e;
    return fp;
}
pub unsafe fn diy_fp_from_double(d: ::core::ffi::c_double) -> DiyFp {
    // Was a `DoubleBits` union (`d: f64`/`u64_0: u64`, written via `.d`
    // then read via `.u64_0`); `f64::to_bits` is the same bit-for-bit
    // reinterpretation without a union.
    let bits: u64 = d.to_bits();
    let mut res: DiyFp = DiyFp { f: 0, e: 0 };
    let biased_e: i32 =
        ((bits & K_DP_EXPONENT_MASK) >> K_DP_SIGNIFICAND_SIZE) as i32;
    let significand: u64 = bits & K_DP_SIGNIFICAND_MASK;
    if biased_e != 0_i32 {
        res.f = significand.wrapping_add(K_DP_HIDDEN_BIT);
        res.e = biased_e - K_DP_EXPONENT_BIAS;
    } else {
        res.f = significand;
        res.e = K_DP_MIN_EXPONENT + 1_i32;
    }
    return res;
}
#[inline]
unsafe fn diy_fp_subtract(lhs: DiyFp, rhs: DiyFp) -> DiyFp {
    return diy_fp_from_parts(lhs.f.wrapping_sub(rhs.f), lhs.e);
}
#[inline]
unsafe fn diy_fp_multiply(lhs: DiyFp, rhs: DiyFp) -> DiyFp {
    let m32: u64 = 0xffffffff_u64;
    let a: u64 = lhs.f >> 32_i32;
    let b: u64 = lhs.f & m32;
    let c: u64 = rhs.f >> 32_i32;
    let d: u64 = rhs.f & m32;
    let ac: u64 = a.wrapping_mul(c);
    let bc: u64 = b.wrapping_mul(c);
    let ad: u64 = a.wrapping_mul(d);
    let bd: u64 = b.wrapping_mul(d);
    let mut tmp: u64 = (bd >> 32_i32)
        .wrapping_add(ad & m32)
        .wrapping_add(bc & m32);
    tmp = tmp.wrapping_add(((1 as ::core::ffi::c_uint) << 31_i32) as u64);
    return diy_fp_from_parts(
        ac.wrapping_add(ad >> 32_i32)
            .wrapping_add(bc >> 32_i32)
            .wrapping_add(tmp >> 32_i32),
        lhs.e + rhs.e + 64_i32,
    );
}
#[inline]
unsafe fn normalize(lhs: DiyFp) -> DiyFp {
    let s: i32 = (lhs.f as ::core::ffi::c_ulonglong).leading_zeros() as i32;
    return diy_fp_from_parts(lhs.f << s, lhs.e - s);
}
#[inline]
unsafe fn normalize_boundary(lhs: DiyFp) -> DiyFp {
    let mut res: DiyFp = lhs;
    while res.f & K_DP_HIDDEN_BIT << 1_i32 == 0 {
        res.f <<= 1_i32;
        res.e -= 1;
    }
    res.f <<= K_DIY_SIGNIFICAND_SIZE - K_DP_SIGNIFICAND_SIZE - 2_i32;
    res.e = res.e - (K_DIY_SIGNIFICAND_SIZE - K_DP_SIGNIFICAND_SIZE - 2_i32);
    return res;
}
#[inline]
unsafe fn normalized_boundaries(lhs: DiyFp, minus: *mut DiyFp, plus: *mut DiyFp) {
    let pl: DiyFp = normalize_boundary(diy_fp_from_parts(
        (lhs.f << 1_i32).wrapping_add(1_u64),
        lhs.e - 1_i32,
    ));
    let mut mi: DiyFp = if lhs.f == K_DP_HIDDEN_BIT {
        diy_fp_from_parts(
            (lhs.f << 2_i32).wrapping_sub(1_u64),
            lhs.e - 2_i32,
        )
    } else {
        diy_fp_from_parts(
            (lhs.f << 1_i32).wrapping_sub(1_u64),
            lhs.e - 1_i32,
        )
    };
    mi.f <<= mi.e - pl.e;
    mi.e = pl.e;
    *plus = pl;
    *minus = mi;
}
#[inline]
unsafe fn get_cached_power(e: i32, k_out: *mut i32) -> DiyFp {
    static K_CACHED_POWERS_F: [u64; 87] = [
        (0xfa8fd5a0 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x81c0288_i32 as u64,
        (0xbaaee17f as ::core::ffi::c_uint as u64) << 32_i32
            | 0xa23ebf76 as ::core::ffi::c_uint as u64,
        (0x8b16fb20 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x3055ac76_i32 as u64,
        (0xcf42894a as ::core::ffi::c_uint as u64) << 32_i32
            | 0x5dce35ea_i32 as u64,
        (0x9a6bb0aa as ::core::ffi::c_uint as u64) << 32_i32
            | 0x55653b2d_i32 as u64,
        (0xe61acf03 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x3d1a45df_i32 as u64,
        (0xab70fe17 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xc79ac6ca as ::core::ffi::c_uint as u64,
        (0xff77b1fc as ::core::ffi::c_uint as u64) << 32_i32
            | 0xbebcdc4f as ::core::ffi::c_uint as u64,
        (0xbe5691ef as ::core::ffi::c_uint as u64) << 32_i32
            | 0x416bd60c_i32 as u64,
        (0x8dd01fad as ::core::ffi::c_uint as u64) << 32_i32
            | 0x907ffc3c as ::core::ffi::c_uint as u64,
        (0xd3515c28 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x31559a83_i32 as u64,
        (0x9d71ac8f as ::core::ffi::c_uint as u64) << 32_i32
            | 0xada6c9b5 as ::core::ffi::c_uint as u64,
        (0xea9c2277 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x23ee8bcb_i32 as u64,
        (0xaecc4991 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x4078536d_i32 as u64,
        (0x823c1279 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x5db6ce57_i32 as u64,
        (0xc2109436 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x4dfb5637_i32 as u64,
        (0x9096ea6f as ::core::ffi::c_uint as u64) << 32_i32
            | 0x3848984f_i32 as u64,
        (0xd77485cb as ::core::ffi::c_uint as u64) << 32_i32
            | 0x25823ac7_i32 as u64,
        (0xa086cfcd as ::core::ffi::c_uint as u64) << 32_i32
            | 0x97bf97f4 as ::core::ffi::c_uint as u64,
        (0xef340a98 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x172aace5_i32 as u64,
        (0xb23867fb as ::core::ffi::c_uint as u64) << 32_i32
            | 0x2a35b28e_i32 as u64,
        (0x84c8d4df as ::core::ffi::c_uint as u64) << 32_i32
            | 0xd2c63f3b as ::core::ffi::c_uint as u64,
        (0xc5dd4427 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x1ad3cdba_i32 as u64,
        (0x936b9fce as ::core::ffi::c_uint as u64) << 32_i32
            | 0xbb25c996 as ::core::ffi::c_uint as u64,
        (0xdbac6c24 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x7d62a584_i32 as u64,
        (0xa3ab6658 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xd5fdaf6_i32 as u64,
        (0xf3e2f893 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xdec3f126 as ::core::ffi::c_uint as u64,
        (0xb5b5ada8 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xaaff80b8 as ::core::ffi::c_uint as u64,
        (0x87625f05 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x6c7c4a8b_i32 as u64,
        (0xc9bcff60 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x34c13053_i32 as u64,
        (0x964e858c as ::core::ffi::c_uint as u64) << 32_i32
            | 0x91ba2655 as ::core::ffi::c_uint as u64,
        (0xdff97724 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x70297ebd_i32 as u64,
        (0xa6dfbd9f as ::core::ffi::c_uint as u64) << 32_i32
            | 0xb8e5b88f as ::core::ffi::c_uint as u64,
        (0xf8a95fcf as ::core::ffi::c_uint as u64) << 32_i32
            | 0x88747d94 as ::core::ffi::c_uint as u64,
        (0xb9447093 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x8fa89bcf as ::core::ffi::c_uint as u64,
        (0x8a08f0f8 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xbf0f156b as ::core::ffi::c_uint as u64,
        (0xcdb02555 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x653131b6_i32 as u64,
        (0x993fe2c6 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xd07b7fac as ::core::ffi::c_uint as u64,
        (0xe45c10c4 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x2a2b3b06_i32 as u64,
        (0xaa242499 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x697392d3_i32 as u64,
        (0xfd87b5f2 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x8300ca0e as ::core::ffi::c_uint as u64,
        (0xbce50864 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x92111aeb as ::core::ffi::c_uint as u64,
        (0x8cbccc09 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x6f5088cc_i32 as u64,
        (0xd1b71758 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xe219652c as ::core::ffi::c_uint as u64,
        (0x9c400000 as ::core::ffi::c_uint as u64) << 32_i32
            | 0_i32 as u64,
        (0xe8d4a510 as ::core::ffi::c_uint as u64) << 32_i32
            | 0_i32 as u64,
        (0xad78ebc5 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xac620000 as ::core::ffi::c_uint as u64,
        (0x813f3978 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xf8940984 as ::core::ffi::c_uint as u64,
        (0xc097ce7b as ::core::ffi::c_uint as u64) << 32_i32
            | 0xc90715b3 as ::core::ffi::c_uint as u64,
        (0x8f7e32ce as ::core::ffi::c_uint as u64) << 32_i32
            | 0x7bea5c70_i32 as u64,
        (0xd5d238a4 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xabe98068 as ::core::ffi::c_uint as u64,
        (0x9f4f2726 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x179a2245_i32 as u64,
        (0xed63a231 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xd4c4fb27 as ::core::ffi::c_uint as u64,
        (0xb0de6538 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x8cc8ada8 as ::core::ffi::c_uint as u64,
        (0x83c7088e as ::core::ffi::c_uint as u64) << 32_i32
            | 0x1aab65db_i32 as u64,
        (0xc45d1df9 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x42711d9a_i32 as u64,
        (0x924d692c as ::core::ffi::c_uint as u64) << 32_i32
            | 0xa61be758 as ::core::ffi::c_uint as u64,
        (0xda01ee64 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x1a708dea_i32 as u64,
        (0xa26da399 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x9aef774a as ::core::ffi::c_uint as u64,
        (0xf209787b as ::core::ffi::c_uint as u64) << 32_i32
            | 0xb47d6b85 as ::core::ffi::c_uint as u64,
        (0xb454e4a1 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x79dd1877_i32 as u64,
        (0x865b8692 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x5b9bc5c2_i32 as u64,
        (0xc83553c5 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xc8965d3d as ::core::ffi::c_uint as u64,
        (0x952ab45c as ::core::ffi::c_uint as u64) << 32_i32
            | 0xfa97a0b3 as ::core::ffi::c_uint as u64,
        (0xde469fbd as ::core::ffi::c_uint as u64) << 32_i32
            | 0x99a05fe3 as ::core::ffi::c_uint as u64,
        (0xa59bc234 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xdb398c25 as ::core::ffi::c_uint as u64,
        (0xf6c69a72 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xa3989f5c as ::core::ffi::c_uint as u64,
        (0xb7dcbf53 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x54e9bece_i32 as u64,
        (0x88fcf317 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xf22241e2 as ::core::ffi::c_uint as u64,
        (0xcc20ce9b as ::core::ffi::c_uint as u64) << 32_i32
            | 0xd35c78a5 as ::core::ffi::c_uint as u64,
        (0x98165af3 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x7b2153df_i32 as u64,
        (0xe2a0b5dc as ::core::ffi::c_uint as u64) << 32_i32
            | 0x971f303a as ::core::ffi::c_uint as u64,
        (0xa8d9d153 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x5ce3b396_i32 as u64,
        (0xfb9b7cd9 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xa4a7443c as ::core::ffi::c_uint as u64,
        (0xbb764c4c as ::core::ffi::c_uint as u64) << 32_i32
            | 0xa7a44410 as ::core::ffi::c_uint as u64,
        (0x8bab8eef as ::core::ffi::c_uint as u64) << 32_i32
            | 0xb6409c1a as ::core::ffi::c_uint as u64,
        (0xd01fef10 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xa657842c as ::core::ffi::c_uint as u64,
        (0x9b10a4e5 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xe9913129 as ::core::ffi::c_uint as u64,
        (0xe7109bfb as ::core::ffi::c_uint as u64) << 32_i32
            | 0xa19c0c9d as ::core::ffi::c_uint as u64,
        (0xac2820d9 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x623bf429_i32 as u64,
        (0x80444b5e as ::core::ffi::c_uint as u64) << 32_i32
            | 0x7aa7cf85_i32 as u64,
        (0xbf21e440 as ::core::ffi::c_uint as u64) << 32_i32
            | 0x3acdd2d_i32 as u64,
        (0x8e679c2f as ::core::ffi::c_uint as u64) << 32_i32
            | 0x5e44ff8f_i32 as u64,
        (0xd433179d as ::core::ffi::c_uint as u64) << 32_i32
            | 0x9c8cb841 as ::core::ffi::c_uint as u64,
        (0x9e19db92 as ::core::ffi::c_uint as u64) << 32_i32
            | 0xb4e31ba9 as ::core::ffi::c_uint as u64,
        (0xeb96bf6e as ::core::ffi::c_uint as u64) << 32_i32
            | 0xbadf77d9 as ::core::ffi::c_uint as u64,
        (0xaf87023b as ::core::ffi::c_uint as u64) << 32_i32
            | 0x9bf0ee6b as ::core::ffi::c_uint as u64,
    ];
    static K_CACHED_POWERS_E: [i16; 87] = [
        -1220_i32 as i16,
        -1193_i32 as i16,
        -1166_i32 as i16,
        -1140_i32 as i16,
        -1113_i32 as i16,
        -1087_i32 as i16,
        -1060_i32 as i16,
        -1034_i32 as i16,
        -1007_i32 as i16,
        -980_i32 as i16,
        -954_i32 as i16,
        -927_i32 as i16,
        -901_i32 as i16,
        -874_i32 as i16,
        -847_i32 as i16,
        -821_i32 as i16,
        -794_i32 as i16,
        -768_i32 as i16,
        -741_i32 as i16,
        -715_i32 as i16,
        -688_i32 as i16,
        -661_i32 as i16,
        -635_i32 as i16,
        -608_i32 as i16,
        -582_i32 as i16,
        -555_i32 as i16,
        -529_i32 as i16,
        -502_i32 as i16,
        -475_i32 as i16,
        -449_i32 as i16,
        -422_i32 as i16,
        -396_i32 as i16,
        -369_i32 as i16,
        -343_i32 as i16,
        -316_i32 as i16,
        -289_i32 as i16,
        -263_i32 as i16,
        -236_i32 as i16,
        -210_i32 as i16,
        -183_i32 as i16,
        -157_i32 as i16,
        -130_i32 as i16,
        -103_i32 as i16,
        -77_i32 as i16,
        -50_i32 as i16,
        -24_i32 as i16,
        3_i32 as i16,
        30_i32 as i16,
        56_i32 as i16,
        83_i32 as i16,
        109_i32 as i16,
        136_i32 as i16,
        162_i32 as i16,
        189_i32 as i16,
        216_i32 as i16,
        242_i32 as i16,
        269_i32 as i16,
        295_i32 as i16,
        322_i32 as i16,
        348_i32 as i16,
        375_i32 as i16,
        402_i32 as i16,
        428_i32 as i16,
        455_i32 as i16,
        481_i32 as i16,
        508_i32 as i16,
        534_i32 as i16,
        561_i32 as i16,
        588_i32 as i16,
        614_i32 as i16,
        641_i32 as i16,
        667_i32 as i16,
        694_i32 as i16,
        720_i32 as i16,
        747_i32 as i16,
        774_i32 as i16,
        800_i32 as i16,
        827_i32 as i16,
        853_i32 as i16,
        880_i32 as i16,
        907_i32 as i16,
        933_i32 as i16,
        960_i32 as i16,
        986_i32 as i16,
        1013_i32 as i16,
        1039_i32 as i16,
        1066_i32 as i16,
    ];
    let dk: ::core::ffi::c_double = (-61_i32 - e) as ::core::ffi::c_double
        * 0.30102999566398114f64
        + 347_i32 as ::core::ffi::c_double;
    let mut k: i32 = dk as i32;
    if k as ::core::ffi::c_double != dk {
        k += 1;
    }
    let index: ::core::ffi::c_uint =
        ((k >> 3_i32) + 1_i32) as ::core::ffi::c_uint;
    *k_out =
        -(-348_i32 + (index << 3_i32) as i32);
    return diy_fp_from_parts(
        K_CACHED_POWERS_F[index as usize],
        K_CACHED_POWERS_E[index as usize] as i32,
    );
}
#[inline]
unsafe fn grisu_round(
    buffer: *mut ::core::ffi::c_char,
    len: i32,
    delta: u64,
    mut rest: u64,
    ten_kappa: u64,
    wp_w: u64,
) {
    while rest < wp_w
        && delta.wrapping_sub(rest) >= ten_kappa
        && (rest.wrapping_add(ten_kappa) < wp_w
            || wp_w.wrapping_sub(rest) > rest.wrapping_add(ten_kappa).wrapping_sub(wp_w))
    {
        let ref mut fresh10 = *buffer.offset((len - 1_i32) as isize);
        *fresh10 -= 1;
        rest = rest.wrapping_add(ten_kappa);
    }
}
#[inline]
unsafe fn count_decimal_digit32(n: u32) -> ::core::ffi::c_uint {
    if n < 10_u32 {
        return 1 as ::core::ffi::c_uint;
    }
    if n < 100_u32 {
        return 2 as ::core::ffi::c_uint;
    }
    if n < 1000_u32 {
        return 3 as ::core::ffi::c_uint;
    }
    if n < 10000_u32 {
        return 4 as ::core::ffi::c_uint;
    }
    if n < 100000_u32 {
        return 5 as ::core::ffi::c_uint;
    }
    if n < 1000000_u32 {
        return 6 as ::core::ffi::c_uint;
    }
    if n < 10000000_u32 {
        return 7 as ::core::ffi::c_uint;
    }
    if n < 100000000_u32 {
        return 8 as ::core::ffi::c_uint;
    }
    if n < 1000000000_u32 {
        return 9 as ::core::ffi::c_uint;
    }
    return 10 as ::core::ffi::c_uint;
}
#[inline]
unsafe fn digit_gen(
    w: DiyFp,
    mp: DiyFp,
    mut delta: u64,
    buffer: *mut ::core::ffi::c_char,
    len: *mut i32,
    k_out: *mut i32,
) {
    static K_POW10: [u32; 10] = [
        1_i32 as u32,
        10_i32 as u32,
        100_i32 as u32,
        1000_i32 as u32,
        10000_i32 as u32,
        100000_i32 as u32,
        1000000_i32 as u32,
        10000000_i32 as u32,
        100000000_i32 as u32,
        1000000000_i32 as u32,
    ];
    let one: DiyFp = diy_fp_from_parts((1_i32 as u64) << -mp.e, mp.e) as DiyFp;
    let wp_w: DiyFp = diy_fp_subtract(mp, w) as DiyFp;
    let mut p1: u32 = (mp.f >> -one.e) as u32;
    let mut p2: u64 = mp.f & one.f.wrapping_sub(1_u64);
    let mut kappa: i32 = count_decimal_digit32(p1) as i32;
    *len = 0_i32;
    while kappa > 0_i32 {
        let d: u32;
        match kappa {
            10 => {
                d = p1.wrapping_div(1000000000_u32);
                p1 = p1.wrapping_rem(1000000000_u32);
            }
            9 => {
                d = p1.wrapping_div(100000000_u32);
                p1 = p1.wrapping_rem(100000000_u32);
            }
            8 => {
                d = p1.wrapping_div(10000000_u32);
                p1 = p1.wrapping_rem(10000000_u32);
            }
            7 => {
                d = p1.wrapping_div(1000000_u32);
                p1 = p1.wrapping_rem(1000000_u32);
            }
            6 => {
                d = p1.wrapping_div(100000_u32);
                p1 = p1.wrapping_rem(100000_u32);
            }
            5 => {
                d = p1.wrapping_div(10000_u32);
                p1 = p1.wrapping_rem(10000_u32);
            }
            4 => {
                d = p1.wrapping_div(1000_u32);
                p1 = p1.wrapping_rem(1000_u32);
            }
            3 => {
                d = p1.wrapping_div(100_u32);
                p1 = p1.wrapping_rem(100_u32);
            }
            2 => {
                d = p1.wrapping_div(10_u32);
                p1 = p1.wrapping_rem(10_u32);
            }
            1 => {
                d = p1;
                p1 = 0_u32;
            }
            _ => {
                d = 0_u32;
            }
        }
        if d != 0 || *len != 0 {
            let fresh8 = *len;
            *len = *len + 1;
            *buffer.offset(fresh8 as isize) = ('0' as i32
                + d as ::core::ffi::c_char as i32)
                as ::core::ffi::c_char;
        }
        kappa -= 1;
        let tmp: u64 = ((p1 as u64) << -one.e).wrapping_add(p2);
        if tmp <= delta {
            *k_out += kappa;
            grisu_round(
                buffer,
                *len,
                delta,
                tmp,
                (K_POW10[(kappa as usize).min(9)] as u64) << -one.e,
                wp_w.f,
            );
            return;
        }
    }
    loop {
        p2 = p2.wrapping_mul(10_u64);
        delta = delta.wrapping_mul(10_u64);
        let d_0: ::core::ffi::c_char = (p2 >> -one.e) as ::core::ffi::c_char;
        if d_0 as i32 != 0 || *len != 0 {
            let fresh9 = *len;
            *len = *len + 1;
            *buffer.offset(fresh9 as isize) =
                ('0' as i32 + d_0 as i32) as ::core::ffi::c_char;
        }
        p2 &= one.f.wrapping_sub(1_u64);
        kappa -= 1;
        if p2 < delta {
            *k_out += kappa;
            grisu_round(
                buffer,
                *len,
                delta,
                p2,
                one.f,
                wp_w.f
                    .wrapping_mul(K_POW10[(-kappa as usize).min(9)] as u64),
            );
            return;
        }
    }
}
#[inline]
unsafe fn grisu2(
    value: ::core::ffi::c_double,
    buffer: *mut ::core::ffi::c_char,
    length: *mut i32,
    k_out: *mut i32,
) {
    let v: DiyFp = diy_fp_from_double(value) as DiyFp;
    let mut w_m: DiyFp = DiyFp { f: 0, e: 0 };
    let mut w_p: DiyFp = DiyFp { f: 0, e: 0 };
    normalized_boundaries(v, &raw mut w_m, &raw mut w_p);
    let c_mk: DiyFp = get_cached_power(w_p.e, k_out) as DiyFp;
    let w: DiyFp = diy_fp_multiply(normalize(v), c_mk) as DiyFp;
    let mut wp: DiyFp = diy_fp_multiply(w_p, c_mk);
    let mut wm: DiyFp = diy_fp_multiply(w_m, c_mk);
    wm.f = wm.f.wrapping_add(1);
    wp.f = wp.f.wrapping_sub(1);
    digit_gen(w, wp, wp.f.wrapping_sub(wm.f), buffer, length, k_out);
}
#[inline]
unsafe fn get_digits_lut() -> *const ::core::ffi::c_char {
    static C_DIGITS_LUT: [::core::ffi::c_char; 200] = [
        '0' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '0' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
    ];
    return &raw const C_DIGITS_LUT as *const ::core::ffi::c_char;
}
#[inline]
unsafe fn write_exponent(mut k_out: i32, mut buffer: *mut ::core::ffi::c_char) {
    if k_out < 0_i32 {
        let fresh1 = buffer;
        buffer = buffer.offset(1);
        *fresh1 = '-' as i32 as ::core::ffi::c_char;
        k_out = -k_out;
    }
    if k_out >= 100_i32 {
        let fresh2 = buffer;
        buffer = buffer.offset(1);
        *fresh2 = ('0' as i32
            + (k_out / 100_i32) as ::core::ffi::c_char as i32)
            as ::core::ffi::c_char;
        k_out %= 100_i32;
        let d: *const ::core::ffi::c_char =
            get_digits_lut().offset((k_out * 2_i32) as isize);
        let fresh3 = buffer;
        buffer = buffer.offset(1);
        *fresh3 = *d.offset(0_i32 as isize);
        let fresh4 = buffer;
        buffer = buffer.offset(1);
        *fresh4 = *d.offset(1_i32 as isize);
    } else if k_out >= 10_i32 {
        let d_0: *const ::core::ffi::c_char =
            get_digits_lut().offset((k_out * 2_i32) as isize);
        let fresh5 = buffer;
        buffer = buffer.offset(1);
        *fresh5 = *d_0.offset(0_i32 as isize);
        let fresh6 = buffer;
        buffer = buffer.offset(1);
        *fresh6 = *d_0.offset(1_i32 as isize);
    } else {
        let fresh7 = buffer;
        buffer = buffer.offset(1);
        *fresh7 = ('0' as i32 + k_out as ::core::ffi::c_char as i32)
            as ::core::ffi::c_char;
    }
    *buffer = '\0' as i32 as ::core::ffi::c_char;
}
#[inline]
unsafe fn prettify(
    buffer: *mut ::core::ffi::c_char,
    length: i32,
    k: i32,
) {
    let kk: i32 = length + k;
    if length <= kk && kk <= 21_i32 {
        let mut i: i32 = length;
        while i < kk {
            *buffer.offset(i as isize) = '0' as i32 as ::core::ffi::c_char;
            i += 1;
        }
        *buffer.offset(kk as isize) = '.' as i32 as ::core::ffi::c_char;
        *buffer.offset((kk + 1_i32) as isize) = '0' as i32 as ::core::ffi::c_char;
        *buffer.offset((kk + 2_i32) as isize) =
            '\0' as i32 as ::core::ffi::c_char;
    } else if 0_i32 < kk && kk <= 21_i32 {
        memmove(
            buffer.offset((kk + 1_i32) as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            buffer.offset(kk as isize) as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            (length - kk) as usize,
        );
        *buffer.offset(kk as isize) = '.' as i32 as ::core::ffi::c_char;
        *buffer.offset((length + 1_i32) as isize) =
            '\0' as i32 as ::core::ffi::c_char;
    } else if -6_i32 < kk && kk <= 0_i32 {
        let offset: i32 = 2_i32 - kk;
        memmove(
            buffer.offset(offset as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            buffer.offset(0_i32 as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            length as usize,
        );
        *buffer.offset(0_i32 as isize) = '0' as i32 as ::core::ffi::c_char;
        *buffer.offset(1_i32 as isize) = '.' as i32 as ::core::ffi::c_char;
        let mut i_0: i32 = 2_i32;
        while i_0 < offset {
            *buffer.offset(i_0 as isize) = '0' as i32 as ::core::ffi::c_char;
            i_0 += 1;
        }
        *buffer.offset((length + offset) as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else if length == 1_i32 {
        *buffer.offset(1_i32 as isize) = 'e' as i32 as ::core::ffi::c_char;
        write_exponent(
            kk - 1_i32,
            buffer.offset(2_i32 as isize) as *mut ::core::ffi::c_char,
        );
    } else {
        memmove(
            buffer.offset(2_i32 as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            buffer.offset(1_i32 as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            (length - 1_i32) as usize,
        );
        *buffer.offset(1_i32 as isize) = '.' as i32 as ::core::ffi::c_char;
        *buffer.offset((length + 1_i32) as isize) =
            'e' as i32 as ::core::ffi::c_char;
        write_exponent(
            kk - 1_i32,
            buffer.offset((length + 2_i32) as isize)
                as *mut ::core::ffi::c_char,
        );
    };
}
pub unsafe fn emyg_dtoa(mut value: ::core::ffi::c_double, mut buffer: *mut ::core::ffi::c_char) {
    if value == 0_i32 as ::core::ffi::c_double {
        *buffer.offset(0_i32 as isize) = '0' as i32 as ::core::ffi::c_char;
        *buffer.offset(1_i32 as isize) = '.' as i32 as ::core::ffi::c_char;
        *buffer.offset(2_i32 as isize) = '0' as i32 as ::core::ffi::c_char;
        *buffer.offset(3_i32 as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else {
        if value < 0_i32 as ::core::ffi::c_double {
            let fresh0 = buffer;
            buffer = buffer.offset(1);
            *fresh0 = '-' as i32 as ::core::ffi::c_char;
            value = -value;
        }
        let mut length: i32 = 0;
        let mut k_out: i32 = 0;
        grisu2(value, buffer, &raw mut length, &raw mut k_out);
        prettify(buffer, length, k_out);
    };
}
