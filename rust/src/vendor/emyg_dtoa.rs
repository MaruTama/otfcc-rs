extern "C" {
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __int16_t = i16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int16_t = __int16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
pub type DiyFp = DiyFp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DiyFp_s {
    pub f: uint64_t,
    pub e: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub d: ::core::ffi::c_double,
    pub u64_0: uint64_t,
}
static mut kDiySignificandSize: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
static mut kDpSignificandSize: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
static mut kDpExponentBias: ::core::ffi::c_int =
    0x3ff as ::core::ffi::c_int + 52 as ::core::ffi::c_int;
static mut kDpMinExponent: ::core::ffi::c_int =
    -(0x3ff as ::core::ffi::c_int) - 52 as ::core::ffi::c_int;
static mut kDpExponentMask: uint64_t = (0x7ff00000 as ::core::ffi::c_int as uint64_t)
    << 32 as ::core::ffi::c_int
    | 0 as ::core::ffi::c_int as uint64_t;
static mut kDpSignificandMask: uint64_t = (0xfffff as ::core::ffi::c_int as uint64_t)
    << 32 as ::core::ffi::c_int
    | 0xffffffff as ::core::ffi::c_uint as uint64_t;
static mut kDpHiddenBit: uint64_t = (0x100000 as ::core::ffi::c_int as uint64_t)
    << 32 as ::core::ffi::c_int
    | 0 as ::core::ffi::c_int as uint64_t;
#[inline]
unsafe extern "C" fn DiyFp_from_parts(mut f: uint64_t, mut e: ::core::ffi::c_int) -> DiyFp {
    let mut fp: DiyFp = DiyFp_s { f: 0, e: 0 };
    fp.f = f;
    fp.e = e;
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn DiyFp_from_double(mut d: ::core::ffi::c_double) -> DiyFp {
    let mut u: C2RustUnnamed = C2RustUnnamed { d: d };
    let mut res: DiyFp = DiyFp_s { f: 0, e: 0 };
    let mut biased_e: ::core::ffi::c_int =
        ((u.u64_0 & kDpExponentMask) >> kDpSignificandSize) as ::core::ffi::c_int;
    let mut significand: uint64_t = u.u64_0 & kDpSignificandMask;
    if biased_e != 0 as ::core::ffi::c_int {
        res.f = significand.wrapping_add(kDpHiddenBit);
        res.e = biased_e - kDpExponentBias;
    } else {
        res.f = significand;
        res.e = kDpMinExponent + 1 as ::core::ffi::c_int;
    }
    return res;
}
#[inline]
unsafe extern "C" fn DiyFp_subtract(lhs: DiyFp, rhs: DiyFp) -> DiyFp {
    return DiyFp_from_parts(lhs.f.wrapping_sub(rhs.f), lhs.e);
}
#[inline]
unsafe extern "C" fn DiyFp_multiply(lhs: DiyFp, rhs: DiyFp) -> DiyFp {
    let M32: uint64_t = 0xffffffff as uint64_t;
    let a: uint64_t = lhs.f >> 32 as ::core::ffi::c_int;
    let b: uint64_t = lhs.f & M32;
    let c: uint64_t = rhs.f >> 32 as ::core::ffi::c_int;
    let d: uint64_t = rhs.f & M32;
    let ac: uint64_t = a.wrapping_mul(c);
    let bc: uint64_t = b.wrapping_mul(c);
    let ad: uint64_t = a.wrapping_mul(d);
    let bd: uint64_t = b.wrapping_mul(d);
    let mut tmp: uint64_t = (bd >> 32 as ::core::ffi::c_int)
        .wrapping_add(ad & M32)
        .wrapping_add(bc & M32);
    tmp = tmp.wrapping_add(((1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int) as uint64_t);
    return DiyFp_from_parts(
        ac.wrapping_add(ad >> 32 as ::core::ffi::c_int)
            .wrapping_add(bc >> 32 as ::core::ffi::c_int)
            .wrapping_add(tmp >> 32 as ::core::ffi::c_int),
        lhs.e + rhs.e + 64 as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn Normalize(lhs: DiyFp) -> DiyFp {
    let mut s: ::core::ffi::c_int = (lhs.f as ::core::ffi::c_ulonglong).leading_zeros() as i32;
    return DiyFp_from_parts(lhs.f << s, lhs.e - s);
}
#[inline]
unsafe extern "C" fn NormalizeBoundary(lhs: DiyFp) -> DiyFp {
    let mut res: DiyFp = lhs;
    while res.f & kDpHiddenBit << 1 as ::core::ffi::c_int == 0 {
        res.f <<= 1 as ::core::ffi::c_int;
        res.e -= 1;
    }
    res.f <<= kDiySignificandSize - kDpSignificandSize - 2 as ::core::ffi::c_int;
    res.e = res.e - (kDiySignificandSize - kDpSignificandSize - 2 as ::core::ffi::c_int);
    return res;
}
#[inline]
unsafe extern "C" fn NormalizedBoundaries(
    mut lhs: DiyFp,
    mut minus: *mut DiyFp,
    mut plus: *mut DiyFp,
) {
    let mut pl: DiyFp = NormalizeBoundary(DiyFp_from_parts(
        (lhs.f << 1 as ::core::ffi::c_int).wrapping_add(1 as uint64_t),
        lhs.e - 1 as ::core::ffi::c_int,
    ));
    let mut mi: DiyFp = if lhs.f == kDpHiddenBit {
        DiyFp_from_parts(
            (lhs.f << 2 as ::core::ffi::c_int).wrapping_sub(1 as uint64_t),
            lhs.e - 2 as ::core::ffi::c_int,
        )
    } else {
        DiyFp_from_parts(
            (lhs.f << 1 as ::core::ffi::c_int).wrapping_sub(1 as uint64_t),
            lhs.e - 1 as ::core::ffi::c_int,
        )
    };
    mi.f <<= mi.e - pl.e;
    mi.e = pl.e;
    *plus = pl;
    *minus = mi;
}
#[inline]
unsafe extern "C" fn GetCachedPower(
    mut e: ::core::ffi::c_int,
    mut K: *mut ::core::ffi::c_int,
) -> DiyFp {
    static mut kCachedPowers_F: [uint64_t; 87] = [
        (0xfa8fd5a0 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x81c0288 as ::core::ffi::c_int as uint64_t,
        (0xbaaee17f as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xa23ebf76 as ::core::ffi::c_uint as uint64_t,
        (0x8b16fb20 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x3055ac76 as ::core::ffi::c_int as uint64_t,
        (0xcf42894a as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x5dce35ea as ::core::ffi::c_int as uint64_t,
        (0x9a6bb0aa as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x55653b2d as ::core::ffi::c_int as uint64_t,
        (0xe61acf03 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x3d1a45df as ::core::ffi::c_int as uint64_t,
        (0xab70fe17 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xc79ac6ca as ::core::ffi::c_uint as uint64_t,
        (0xff77b1fc as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xbebcdc4f as ::core::ffi::c_uint as uint64_t,
        (0xbe5691ef as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x416bd60c as ::core::ffi::c_int as uint64_t,
        (0x8dd01fad as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x907ffc3c as ::core::ffi::c_uint as uint64_t,
        (0xd3515c28 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x31559a83 as ::core::ffi::c_int as uint64_t,
        (0x9d71ac8f as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xada6c9b5 as ::core::ffi::c_uint as uint64_t,
        (0xea9c2277 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x23ee8bcb as ::core::ffi::c_int as uint64_t,
        (0xaecc4991 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x4078536d as ::core::ffi::c_int as uint64_t,
        (0x823c1279 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x5db6ce57 as ::core::ffi::c_int as uint64_t,
        (0xc2109436 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x4dfb5637 as ::core::ffi::c_int as uint64_t,
        (0x9096ea6f as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x3848984f as ::core::ffi::c_int as uint64_t,
        (0xd77485cb as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x25823ac7 as ::core::ffi::c_int as uint64_t,
        (0xa086cfcd as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x97bf97f4 as ::core::ffi::c_uint as uint64_t,
        (0xef340a98 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x172aace5 as ::core::ffi::c_int as uint64_t,
        (0xb23867fb as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x2a35b28e as ::core::ffi::c_int as uint64_t,
        (0x84c8d4df as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xd2c63f3b as ::core::ffi::c_uint as uint64_t,
        (0xc5dd4427 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x1ad3cdba as ::core::ffi::c_int as uint64_t,
        (0x936b9fce as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xbb25c996 as ::core::ffi::c_uint as uint64_t,
        (0xdbac6c24 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x7d62a584 as ::core::ffi::c_int as uint64_t,
        (0xa3ab6658 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xd5fdaf6 as ::core::ffi::c_int as uint64_t,
        (0xf3e2f893 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xdec3f126 as ::core::ffi::c_uint as uint64_t,
        (0xb5b5ada8 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xaaff80b8 as ::core::ffi::c_uint as uint64_t,
        (0x87625f05 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x6c7c4a8b as ::core::ffi::c_int as uint64_t,
        (0xc9bcff60 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x34c13053 as ::core::ffi::c_int as uint64_t,
        (0x964e858c as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x91ba2655 as ::core::ffi::c_uint as uint64_t,
        (0xdff97724 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x70297ebd as ::core::ffi::c_int as uint64_t,
        (0xa6dfbd9f as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xb8e5b88f as ::core::ffi::c_uint as uint64_t,
        (0xf8a95fcf as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x88747d94 as ::core::ffi::c_uint as uint64_t,
        (0xb9447093 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x8fa89bcf as ::core::ffi::c_uint as uint64_t,
        (0x8a08f0f8 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xbf0f156b as ::core::ffi::c_uint as uint64_t,
        (0xcdb02555 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x653131b6 as ::core::ffi::c_int as uint64_t,
        (0x993fe2c6 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xd07b7fac as ::core::ffi::c_uint as uint64_t,
        (0xe45c10c4 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x2a2b3b06 as ::core::ffi::c_int as uint64_t,
        (0xaa242499 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x697392d3 as ::core::ffi::c_int as uint64_t,
        (0xfd87b5f2 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x8300ca0e as ::core::ffi::c_uint as uint64_t,
        (0xbce50864 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x92111aeb as ::core::ffi::c_uint as uint64_t,
        (0x8cbccc09 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x6f5088cc as ::core::ffi::c_int as uint64_t,
        (0xd1b71758 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xe219652c as ::core::ffi::c_uint as uint64_t,
        (0x9c400000 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int as uint64_t,
        (0xe8d4a510 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0 as ::core::ffi::c_int as uint64_t,
        (0xad78ebc5 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xac620000 as ::core::ffi::c_uint as uint64_t,
        (0x813f3978 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xf8940984 as ::core::ffi::c_uint as uint64_t,
        (0xc097ce7b as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xc90715b3 as ::core::ffi::c_uint as uint64_t,
        (0x8f7e32ce as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x7bea5c70 as ::core::ffi::c_int as uint64_t,
        (0xd5d238a4 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xabe98068 as ::core::ffi::c_uint as uint64_t,
        (0x9f4f2726 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x179a2245 as ::core::ffi::c_int as uint64_t,
        (0xed63a231 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xd4c4fb27 as ::core::ffi::c_uint as uint64_t,
        (0xb0de6538 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x8cc8ada8 as ::core::ffi::c_uint as uint64_t,
        (0x83c7088e as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x1aab65db as ::core::ffi::c_int as uint64_t,
        (0xc45d1df9 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x42711d9a as ::core::ffi::c_int as uint64_t,
        (0x924d692c as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xa61be758 as ::core::ffi::c_uint as uint64_t,
        (0xda01ee64 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x1a708dea as ::core::ffi::c_int as uint64_t,
        (0xa26da399 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x9aef774a as ::core::ffi::c_uint as uint64_t,
        (0xf209787b as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xb47d6b85 as ::core::ffi::c_uint as uint64_t,
        (0xb454e4a1 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x79dd1877 as ::core::ffi::c_int as uint64_t,
        (0x865b8692 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x5b9bc5c2 as ::core::ffi::c_int as uint64_t,
        (0xc83553c5 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xc8965d3d as ::core::ffi::c_uint as uint64_t,
        (0x952ab45c as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xfa97a0b3 as ::core::ffi::c_uint as uint64_t,
        (0xde469fbd as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x99a05fe3 as ::core::ffi::c_uint as uint64_t,
        (0xa59bc234 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xdb398c25 as ::core::ffi::c_uint as uint64_t,
        (0xf6c69a72 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xa3989f5c as ::core::ffi::c_uint as uint64_t,
        (0xb7dcbf53 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x54e9bece as ::core::ffi::c_int as uint64_t,
        (0x88fcf317 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xf22241e2 as ::core::ffi::c_uint as uint64_t,
        (0xcc20ce9b as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xd35c78a5 as ::core::ffi::c_uint as uint64_t,
        (0x98165af3 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x7b2153df as ::core::ffi::c_int as uint64_t,
        (0xe2a0b5dc as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x971f303a as ::core::ffi::c_uint as uint64_t,
        (0xa8d9d153 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x5ce3b396 as ::core::ffi::c_int as uint64_t,
        (0xfb9b7cd9 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xa4a7443c as ::core::ffi::c_uint as uint64_t,
        (0xbb764c4c as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xa7a44410 as ::core::ffi::c_uint as uint64_t,
        (0x8bab8eef as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xb6409c1a as ::core::ffi::c_uint as uint64_t,
        (0xd01fef10 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xa657842c as ::core::ffi::c_uint as uint64_t,
        (0x9b10a4e5 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xe9913129 as ::core::ffi::c_uint as uint64_t,
        (0xe7109bfb as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xa19c0c9d as ::core::ffi::c_uint as uint64_t,
        (0xac2820d9 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x623bf429 as ::core::ffi::c_int as uint64_t,
        (0x80444b5e as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x7aa7cf85 as ::core::ffi::c_int as uint64_t,
        (0xbf21e440 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x3acdd2d as ::core::ffi::c_int as uint64_t,
        (0x8e679c2f as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x5e44ff8f as ::core::ffi::c_int as uint64_t,
        (0xd433179d as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x9c8cb841 as ::core::ffi::c_uint as uint64_t,
        (0x9e19db92 as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xb4e31ba9 as ::core::ffi::c_uint as uint64_t,
        (0xeb96bf6e as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0xbadf77d9 as ::core::ffi::c_uint as uint64_t,
        (0xaf87023b as ::core::ffi::c_uint as uint64_t) << 32 as ::core::ffi::c_int
            | 0x9bf0ee6b as ::core::ffi::c_uint as uint64_t,
    ];
    static mut kCachedPowers_E: [int16_t; 87] = [
        -(1220 as ::core::ffi::c_int) as int16_t,
        -(1193 as ::core::ffi::c_int) as int16_t,
        -(1166 as ::core::ffi::c_int) as int16_t,
        -(1140 as ::core::ffi::c_int) as int16_t,
        -(1113 as ::core::ffi::c_int) as int16_t,
        -(1087 as ::core::ffi::c_int) as int16_t,
        -(1060 as ::core::ffi::c_int) as int16_t,
        -(1034 as ::core::ffi::c_int) as int16_t,
        -(1007 as ::core::ffi::c_int) as int16_t,
        -(980 as ::core::ffi::c_int) as int16_t,
        -(954 as ::core::ffi::c_int) as int16_t,
        -(927 as ::core::ffi::c_int) as int16_t,
        -(901 as ::core::ffi::c_int) as int16_t,
        -(874 as ::core::ffi::c_int) as int16_t,
        -(847 as ::core::ffi::c_int) as int16_t,
        -(821 as ::core::ffi::c_int) as int16_t,
        -(794 as ::core::ffi::c_int) as int16_t,
        -(768 as ::core::ffi::c_int) as int16_t,
        -(741 as ::core::ffi::c_int) as int16_t,
        -(715 as ::core::ffi::c_int) as int16_t,
        -(688 as ::core::ffi::c_int) as int16_t,
        -(661 as ::core::ffi::c_int) as int16_t,
        -(635 as ::core::ffi::c_int) as int16_t,
        -(608 as ::core::ffi::c_int) as int16_t,
        -(582 as ::core::ffi::c_int) as int16_t,
        -(555 as ::core::ffi::c_int) as int16_t,
        -(529 as ::core::ffi::c_int) as int16_t,
        -(502 as ::core::ffi::c_int) as int16_t,
        -(475 as ::core::ffi::c_int) as int16_t,
        -(449 as ::core::ffi::c_int) as int16_t,
        -(422 as ::core::ffi::c_int) as int16_t,
        -(396 as ::core::ffi::c_int) as int16_t,
        -(369 as ::core::ffi::c_int) as int16_t,
        -(343 as ::core::ffi::c_int) as int16_t,
        -(316 as ::core::ffi::c_int) as int16_t,
        -(289 as ::core::ffi::c_int) as int16_t,
        -(263 as ::core::ffi::c_int) as int16_t,
        -(236 as ::core::ffi::c_int) as int16_t,
        -(210 as ::core::ffi::c_int) as int16_t,
        -(183 as ::core::ffi::c_int) as int16_t,
        -(157 as ::core::ffi::c_int) as int16_t,
        -(130 as ::core::ffi::c_int) as int16_t,
        -(103 as ::core::ffi::c_int) as int16_t,
        -(77 as ::core::ffi::c_int) as int16_t,
        -(50 as ::core::ffi::c_int) as int16_t,
        -(24 as ::core::ffi::c_int) as int16_t,
        3 as ::core::ffi::c_int as int16_t,
        30 as ::core::ffi::c_int as int16_t,
        56 as ::core::ffi::c_int as int16_t,
        83 as ::core::ffi::c_int as int16_t,
        109 as ::core::ffi::c_int as int16_t,
        136 as ::core::ffi::c_int as int16_t,
        162 as ::core::ffi::c_int as int16_t,
        189 as ::core::ffi::c_int as int16_t,
        216 as ::core::ffi::c_int as int16_t,
        242 as ::core::ffi::c_int as int16_t,
        269 as ::core::ffi::c_int as int16_t,
        295 as ::core::ffi::c_int as int16_t,
        322 as ::core::ffi::c_int as int16_t,
        348 as ::core::ffi::c_int as int16_t,
        375 as ::core::ffi::c_int as int16_t,
        402 as ::core::ffi::c_int as int16_t,
        428 as ::core::ffi::c_int as int16_t,
        455 as ::core::ffi::c_int as int16_t,
        481 as ::core::ffi::c_int as int16_t,
        508 as ::core::ffi::c_int as int16_t,
        534 as ::core::ffi::c_int as int16_t,
        561 as ::core::ffi::c_int as int16_t,
        588 as ::core::ffi::c_int as int16_t,
        614 as ::core::ffi::c_int as int16_t,
        641 as ::core::ffi::c_int as int16_t,
        667 as ::core::ffi::c_int as int16_t,
        694 as ::core::ffi::c_int as int16_t,
        720 as ::core::ffi::c_int as int16_t,
        747 as ::core::ffi::c_int as int16_t,
        774 as ::core::ffi::c_int as int16_t,
        800 as ::core::ffi::c_int as int16_t,
        827 as ::core::ffi::c_int as int16_t,
        853 as ::core::ffi::c_int as int16_t,
        880 as ::core::ffi::c_int as int16_t,
        907 as ::core::ffi::c_int as int16_t,
        933 as ::core::ffi::c_int as int16_t,
        960 as ::core::ffi::c_int as int16_t,
        986 as ::core::ffi::c_int as int16_t,
        1013 as ::core::ffi::c_int as int16_t,
        1039 as ::core::ffi::c_int as int16_t,
        1066 as ::core::ffi::c_int as int16_t,
    ];
    let mut dk: ::core::ffi::c_double = (-(61 as ::core::ffi::c_int) - e) as ::core::ffi::c_double
        * 0.30102999566398114f64
        + 347 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut k: ::core::ffi::c_int = dk as ::core::ffi::c_int;
    if k as ::core::ffi::c_double != dk {
        k += 1;
    }
    let mut index: ::core::ffi::c_uint =
        ((k >> 3 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    *K = -(-(348 as ::core::ffi::c_int) + (index << 3 as ::core::ffi::c_int) as ::core::ffi::c_int);
    return DiyFp_from_parts(
        kCachedPowers_F[index as usize],
        kCachedPowers_E[index as usize] as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn GrisuRound(
    mut buffer: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut delta: uint64_t,
    mut rest: uint64_t,
    mut ten_kappa: uint64_t,
    mut wp_w: uint64_t,
) {
    while rest < wp_w
        && delta.wrapping_sub(rest) >= ten_kappa
        && (rest.wrapping_add(ten_kappa) < wp_w
            || wp_w.wrapping_sub(rest) > rest.wrapping_add(ten_kappa).wrapping_sub(wp_w))
    {
        let ref mut fresh10 = *buffer.offset((len - 1 as ::core::ffi::c_int) as isize);
        *fresh10 -= 1;
        rest = rest.wrapping_add(ten_kappa);
    }
}
#[inline]
unsafe extern "C" fn CountDecimalDigit32(mut n: uint32_t) -> ::core::ffi::c_uint {
    if n < 10 as uint32_t {
        return 1 as ::core::ffi::c_uint;
    }
    if n < 100 as uint32_t {
        return 2 as ::core::ffi::c_uint;
    }
    if n < 1000 as uint32_t {
        return 3 as ::core::ffi::c_uint;
    }
    if n < 10000 as uint32_t {
        return 4 as ::core::ffi::c_uint;
    }
    if n < 100000 as uint32_t {
        return 5 as ::core::ffi::c_uint;
    }
    if n < 1000000 as uint32_t {
        return 6 as ::core::ffi::c_uint;
    }
    if n < 10000000 as uint32_t {
        return 7 as ::core::ffi::c_uint;
    }
    if n < 100000000 as uint32_t {
        return 8 as ::core::ffi::c_uint;
    }
    if n < 1000000000 as uint32_t {
        return 9 as ::core::ffi::c_uint;
    }
    return 10 as ::core::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn DigitGen(
    W: DiyFp,
    Mp: DiyFp,
    mut delta: uint64_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut len: *mut ::core::ffi::c_int,
    mut K: *mut ::core::ffi::c_int,
) {
    static mut kPow10: [uint32_t; 10] = [
        1 as ::core::ffi::c_int as uint32_t,
        10 as ::core::ffi::c_int as uint32_t,
        100 as ::core::ffi::c_int as uint32_t,
        1000 as ::core::ffi::c_int as uint32_t,
        10000 as ::core::ffi::c_int as uint32_t,
        100000 as ::core::ffi::c_int as uint32_t,
        1000000 as ::core::ffi::c_int as uint32_t,
        10000000 as ::core::ffi::c_int as uint32_t,
        100000000 as ::core::ffi::c_int as uint32_t,
        1000000000 as ::core::ffi::c_int as uint32_t,
    ];
    let one: DiyFp =
        DiyFp_from_parts((1 as ::core::ffi::c_int as uint64_t) << -Mp.e, Mp.e) as DiyFp;
    let wp_w: DiyFp = DiyFp_subtract(Mp, W) as DiyFp;
    let mut p1: uint32_t = (Mp.f >> -one.e) as uint32_t;
    let mut p2: uint64_t = Mp.f & one.f.wrapping_sub(1 as uint64_t);
    let mut kappa: ::core::ffi::c_int = CountDecimalDigit32(p1) as ::core::ffi::c_int;
    *len = 0 as ::core::ffi::c_int;
    while kappa > 0 as ::core::ffi::c_int {
        let mut d: uint32_t = 0 as uint32_t;
        match kappa {
            10 => {
                d = p1.wrapping_div(1000000000 as uint32_t);
                p1 = p1.wrapping_rem(1000000000 as uint32_t);
            }
            9 => {
                d = p1.wrapping_div(100000000 as uint32_t);
                p1 = p1.wrapping_rem(100000000 as uint32_t);
            }
            8 => {
                d = p1.wrapping_div(10000000 as uint32_t);
                p1 = p1.wrapping_rem(10000000 as uint32_t);
            }
            7 => {
                d = p1.wrapping_div(1000000 as uint32_t);
                p1 = p1.wrapping_rem(1000000 as uint32_t);
            }
            6 => {
                d = p1.wrapping_div(100000 as uint32_t);
                p1 = p1.wrapping_rem(100000 as uint32_t);
            }
            5 => {
                d = p1.wrapping_div(10000 as uint32_t);
                p1 = p1.wrapping_rem(10000 as uint32_t);
            }
            4 => {
                d = p1.wrapping_div(1000 as uint32_t);
                p1 = p1.wrapping_rem(1000 as uint32_t);
            }
            3 => {
                d = p1.wrapping_div(100 as uint32_t);
                p1 = p1.wrapping_rem(100 as uint32_t);
            }
            2 => {
                d = p1.wrapping_div(10 as uint32_t);
                p1 = p1.wrapping_rem(10 as uint32_t);
            }
            1 => {
                d = p1;
                p1 = 0 as uint32_t;
            }
            _ => {
                d = 0 as uint32_t;
            }
        }
        if d != 0 || *len != 0 {
            let fresh8 = *len;
            *len = *len + 1;
            *buffer.offset(fresh8 as isize) = ('0' as i32
                + d as ::core::ffi::c_char as ::core::ffi::c_int)
                as ::core::ffi::c_char;
        }
        kappa -= 1;
        let mut tmp: uint64_t = ((p1 as uint64_t) << -one.e).wrapping_add(p2);
        if tmp <= delta {
            *K += kappa;
            GrisuRound(
                buffer,
                *len,
                delta,
                tmp,
                (kPow10[(kappa as usize).min(9)] as uint64_t) << -one.e,
                wp_w.f,
            );
            return;
        }
    }
    loop {
        p2 = p2.wrapping_mul(10 as uint64_t);
        delta = delta.wrapping_mul(10 as uint64_t);
        let mut d_0: ::core::ffi::c_char = (p2 >> -one.e) as ::core::ffi::c_char;
        if d_0 as ::core::ffi::c_int != 0 || *len != 0 {
            let fresh9 = *len;
            *len = *len + 1;
            *buffer.offset(fresh9 as isize) =
                ('0' as i32 + d_0 as ::core::ffi::c_int) as ::core::ffi::c_char;
        }
        p2 &= one.f.wrapping_sub(1 as uint64_t);
        kappa -= 1;
        if p2 < delta {
            *K += kappa;
            GrisuRound(
                buffer,
                *len,
                delta,
                p2,
                one.f,
                wp_w.f.wrapping_mul(kPow10[(-kappa as usize).min(9)] as uint64_t),
            );
            return;
        }
    }
}
#[inline]
unsafe extern "C" fn Grisu2(
    mut value: ::core::ffi::c_double,
    mut buffer: *mut ::core::ffi::c_char,
    mut length: *mut ::core::ffi::c_int,
    mut K: *mut ::core::ffi::c_int,
) {
    let v: DiyFp = DiyFp_from_double(value) as DiyFp;
    let mut w_m: DiyFp = DiyFp_s { f: 0, e: 0 };
    let mut w_p: DiyFp = DiyFp_s { f: 0, e: 0 };
    NormalizedBoundaries(v, &raw mut w_m, &raw mut w_p);
    let c_mk: DiyFp = GetCachedPower(w_p.e, K) as DiyFp;
    let W: DiyFp = DiyFp_multiply(Normalize(v), c_mk) as DiyFp;
    let mut Wp: DiyFp = DiyFp_multiply(w_p, c_mk);
    let mut Wm: DiyFp = DiyFp_multiply(w_m, c_mk);
    Wm.f = Wm.f.wrapping_add(1);
    Wp.f = Wp.f.wrapping_sub(1);
    DigitGen(W, Wp, Wp.f.wrapping_sub(Wm.f), buffer, length, K);
}
#[inline]
unsafe extern "C" fn GetDigitsLut() -> *const ::core::ffi::c_char {
    static mut cDigitsLut: [::core::ffi::c_char; 200] = [
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
    return &raw const cDigitsLut as *const ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn WriteExponent(
    mut K: ::core::ffi::c_int,
    mut buffer: *mut ::core::ffi::c_char,
) {
    if K < 0 as ::core::ffi::c_int {
        let fresh1 = buffer;
        buffer = buffer.offset(1);
        *fresh1 = '-' as i32 as ::core::ffi::c_char;
        K = -K;
    }
    if K >= 100 as ::core::ffi::c_int {
        let fresh2 = buffer;
        buffer = buffer.offset(1);
        *fresh2 = ('0' as i32
            + (K / 100 as ::core::ffi::c_int) as ::core::ffi::c_char as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        K %= 100 as ::core::ffi::c_int;
        let mut d: *const ::core::ffi::c_char =
            GetDigitsLut().offset((K * 2 as ::core::ffi::c_int) as isize);
        let fresh3 = buffer;
        buffer = buffer.offset(1);
        *fresh3 = *d.offset(0 as ::core::ffi::c_int as isize);
        let fresh4 = buffer;
        buffer = buffer.offset(1);
        *fresh4 = *d.offset(1 as ::core::ffi::c_int as isize);
    } else if K >= 10 as ::core::ffi::c_int {
        let mut d_0: *const ::core::ffi::c_char =
            GetDigitsLut().offset((K * 2 as ::core::ffi::c_int) as isize);
        let fresh5 = buffer;
        buffer = buffer.offset(1);
        *fresh5 = *d_0.offset(0 as ::core::ffi::c_int as isize);
        let fresh6 = buffer;
        buffer = buffer.offset(1);
        *fresh6 = *d_0.offset(1 as ::core::ffi::c_int as isize);
    } else {
        let fresh7 = buffer;
        buffer = buffer.offset(1);
        *fresh7 =
            ('0' as i32 + K as ::core::ffi::c_char as ::core::ffi::c_int) as ::core::ffi::c_char;
    }
    *buffer = '\0' as i32 as ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn Prettify(
    mut buffer: *mut ::core::ffi::c_char,
    mut length: ::core::ffi::c_int,
    mut k: ::core::ffi::c_int,
) {
    let kk: ::core::ffi::c_int = length + k;
    if length <= kk && kk <= 21 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = length;
        while i < kk {
            *buffer.offset(i as isize) = '0' as i32 as ::core::ffi::c_char;
            i += 1;
        }
        *buffer.offset(kk as isize) = '.' as i32 as ::core::ffi::c_char;
        *buffer.offset((kk + 1 as ::core::ffi::c_int) as isize) = '0' as i32 as ::core::ffi::c_char;
        *buffer.offset((kk + 2 as ::core::ffi::c_int) as isize) =
            '\0' as i32 as ::core::ffi::c_char;
    } else if (0 as ::core::ffi::c_int) < kk && kk <= 21 as ::core::ffi::c_int {
        memmove(
            buffer.offset((kk + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            buffer.offset(kk as isize) as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            (length - kk) as size_t,
        );
        *buffer.offset(kk as isize) = '.' as i32 as ::core::ffi::c_char;
        *buffer.offset((length + 1 as ::core::ffi::c_int) as isize) =
            '\0' as i32 as ::core::ffi::c_char;
    } else if -(6 as ::core::ffi::c_int) < kk && kk <= 0 as ::core::ffi::c_int {
        let offset: ::core::ffi::c_int = 2 as ::core::ffi::c_int - kk;
        memmove(
            buffer.offset(offset as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            buffer.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            length as size_t,
        );
        *buffer.offset(0 as ::core::ffi::c_int as isize) = '0' as i32 as ::core::ffi::c_char;
        *buffer.offset(1 as ::core::ffi::c_int as isize) = '.' as i32 as ::core::ffi::c_char;
        let mut i_0: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
        while i_0 < offset {
            *buffer.offset(i_0 as isize) = '0' as i32 as ::core::ffi::c_char;
            i_0 += 1;
        }
        *buffer.offset((length + offset) as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else if length == 1 as ::core::ffi::c_int {
        *buffer.offset(1 as ::core::ffi::c_int as isize) = 'e' as i32 as ::core::ffi::c_char;
        WriteExponent(
            kk - 1 as ::core::ffi::c_int,
            buffer.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        );
    } else {
        memmove(
            buffer.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            buffer.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            (length - 1 as ::core::ffi::c_int) as size_t,
        );
        *buffer.offset(1 as ::core::ffi::c_int as isize) = '.' as i32 as ::core::ffi::c_char;
        *buffer.offset((length + 1 as ::core::ffi::c_int) as isize) =
            'e' as i32 as ::core::ffi::c_char;
        WriteExponent(
            kk - 1 as ::core::ffi::c_int,
            buffer.offset((0 as ::core::ffi::c_int + length + 2 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn emyg_dtoa(
    mut value: ::core::ffi::c_double,
    mut buffer: *mut ::core::ffi::c_char,
) {
    if value == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        *buffer.offset(0 as ::core::ffi::c_int as isize) = '0' as i32 as ::core::ffi::c_char;
        *buffer.offset(1 as ::core::ffi::c_int as isize) = '.' as i32 as ::core::ffi::c_char;
        *buffer.offset(2 as ::core::ffi::c_int as isize) = '0' as i32 as ::core::ffi::c_char;
        *buffer.offset(3 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else {
        if value < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            let fresh0 = buffer;
            buffer = buffer.offset(1);
            *fresh0 = '-' as i32 as ::core::ffi::c_char;
            value = -value;
        }
        let mut length: ::core::ffi::c_int = 0;
        let mut K: ::core::ffi::c_int = 0;
        Grisu2(value, buffer, &raw mut length, &raw mut K);
        Prettify(buffer, length, K);
    };
}
