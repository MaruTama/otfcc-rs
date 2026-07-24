// Shared big-endian byte readers, factored out of the ~40 per-file private
// copies c2rust emitted (one per translation unit that #included
// c/lib/support/bin-io.h's `static inline` helpers). Never externally linked
// (no #[no_mangle]) even in their per-file form, so consolidating them
// changes no ABI. Bodies rewritten to u*::from_be_bytes (matching the
// idiom already used on the write side in support/buffer/buffer.rs), but
// every signature is byte-for-byte identical to the original so callers
// need no changes beyond `use`.
pub type uint8_t = u8;
pub type int8_t = i8;
pub type uint16_t = u16;
pub type int16_t = i16;
pub type uint32_t = u32;
pub type int32_t = i32;
pub type uint64_t = u64;

#[inline]
pub(crate) unsafe fn read_8u(src: *const uint8_t) -> uint8_t {
    *src
}

#[inline]
pub(crate) unsafe fn read_8s(src: *const uint8_t) -> int8_t {
    read_8u(src) as int8_t
}

#[inline]
pub(crate) unsafe fn read_16u(src: *const uint8_t) -> uint16_t {
    uint16_t::from_be_bytes([*src, *src.offset(1)])
}

#[inline]
pub(crate) unsafe fn read_16s(src: *const uint8_t) -> int16_t {
    read_16u(src) as int16_t
}

#[inline]
pub(crate) unsafe fn read_24u(src: *const uint8_t) -> uint32_t {
    uint32_t::from_be_bytes([0, *src, *src.offset(1), *src.offset(2)])
}

#[inline]
pub(crate) unsafe fn read_32u(src: *const uint8_t) -> uint32_t {
    uint32_t::from_be_bytes([*src, *src.offset(1), *src.offset(2), *src.offset(3)])
}

#[inline]
pub(crate) unsafe fn read_32s(src: *const uint8_t) -> int32_t {
    read_32u(src) as int32_t
}

#[inline]
pub(crate) unsafe fn read_64u(src: *const uint8_t) -> uint64_t {
    uint64_t::from_be_bytes([
        *src,
        *src.offset(1),
        *src.offset(2),
        *src.offset(3),
        *src.offset(4),
        *src.offset(5),
        *src.offset(6),
        *src.offset(7),
    ])
}
