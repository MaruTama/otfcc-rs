#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::fprintf;

// Stage 7-2-e "Buffer to Vec": `data` was `*mut u8`, manually grown via
// `__caryll_reallocate`/freed via `libc::free`, with `size`/`free` as
// separate hand-tracked bookkeeping fields (`size` = written length,
// `free` = spare allocated-but-unwritten capacity, capped at growing by at
// most 16 MiB per reallocation). `Vec<u8>` now owns the allocation and
// tracks its own length/capacity, so `size`/`free` are gone -- every former
// read of `.size` is `.data.len()`; there is no external equivalent of
// `.free` any more (nothing outside this file ever read it, confirmed by
// grep before this conversion; `Vec`'s own growth strategy replaces the
// hand-rolled one, including the 16 MiB growth cap, which only existed to
// bound a single `realloc` call's size and has no externally observable
// effect on buffer *contents*).
//
// `Copy` dropped (a `Vec` can't be): the one place that relied on it,
// `libcff/subr.rs`'s `vec![zero_buffer; n]` scratch arrays, keeps working
// unchanged under `Clone` instead -- `vec![x; n]` only ever required
// `Clone`, and cloning an empty `Vec::new()` is cheap.
#[derive(Clone)]
pub struct Buffer {
    pub cursor: usize,
    pub data: Vec<u8>,
}
use crate::support::stdio::stderr;

// `Box`-allocated now, not `__caryll_allocate_clean`'d: a calloc'd, all-zero
// `Buffer` is not a valid value the instant it contains a `Vec` (see
// [[otfcc-vec-field-assign-needs-calloc]] throughout this migration), so
// construction has to go through a real Rust value from the start.
pub unsafe fn bufnew() -> *mut Buffer {
    Box::into_raw(Box::new(Buffer {
        cursor: 0,
        data: Vec::new(),
    }))
}
pub unsafe fn buffree(buf: *mut Buffer) {
    if buf.is_null() {
        return;
    }
    drop(Box::from_raw(buf));
}
pub unsafe fn buflen(buf: *mut Buffer) -> usize {
    (*buf).data.len()
}
pub unsafe fn bufpos(buf: *mut Buffer) -> usize {
    (*buf).cursor
}
pub unsafe fn bufseek(buf: *mut Buffer, pos: usize) {
    (*buf).cursor = pos;
}
pub unsafe fn bufclear(buf: *mut Buffer) {
    (*buf).cursor = 0;
    // `.clear()`, not `= Vec::new()`: drops every element but keeps the
    // backing allocation, the same "reset length, keep the allocation"
    // contract `size = 0` + `free = size + free` used to give by hand.
    (*buf).data.clear();
}
// Pushes `bytes` at the cursor, growing the buffer first if needed, and
// advances the cursor. All the fixed-width bufwriteNN{l,b} functions below
// are exactly this plus an endian-ordered byte array
// (to_le_bytes/to_be_bytes), which replaces c2rust's manual per-byte
// shift-mask-store expansion.
//
// A write can seek backward and overwrite already-written bytes in place
// (the offset-backpatching idiom `bufping16b`/`bufpong` below build on) --
// so this is not a plain `Vec::extend`. If the write fits entirely within
// the already-written region (`cursor + bytes.len() <= data.len()`), it's
// a pure in-place overwrite; otherwise `resize` grows the `Vec` first
// (zero-filling any gap between the old length and `cursor`, matching what
// a fresh `realloc` over calloc'd memory used to leave there) before the
// same slice-copy runs either way.
#[inline]
unsafe fn buf_push_bytes(buf: *mut Buffer, bytes: &[u8]) {
    let cursor = (*buf).cursor;
    let end = cursor.wrapping_add(bytes.len());
    let data: &mut Vec<u8> = &mut (*buf).data;
    if data.len() < end {
        data.resize(end, 0);
    }
    data[cursor..end].copy_from_slice(bytes);
    (*buf).cursor = end;
}
pub unsafe fn bufwrite8(buf: *mut Buffer, byte: u8) {
    buf_push_bytes(buf, &[byte]);
}
pub unsafe fn bufwrite16l(buf: *mut Buffer, x: u16) {
    buf_push_bytes(buf, &x.to_le_bytes());
}
pub unsafe fn bufwrite16b(buf: *mut Buffer, x: u16) {
    buf_push_bytes(buf, &x.to_be_bytes());
}
pub unsafe fn bufwrite24l(buf: *mut Buffer, x: u32) {
    // Low 3 bytes only, matching the original's shift-mask expansion, which
    // never touched bits 24-31 either.
    buf_push_bytes(buf, &x.to_le_bytes()[..3]);
}
pub unsafe fn bufwrite24b(buf: *mut Buffer, x: u32) {
    buf_push_bytes(buf, &x.to_be_bytes()[1..]);
}
pub unsafe fn bufwrite32l(buf: *mut Buffer, x: u32) {
    buf_push_bytes(buf, &x.to_le_bytes());
}
pub unsafe fn bufwrite32b(buf: *mut Buffer, x: u32) {
    buf_push_bytes(buf, &x.to_be_bytes());
}
pub unsafe fn bufwrite64l(buf: *mut Buffer, x: u64) {
    buf_push_bytes(buf, &x.to_le_bytes());
}
pub unsafe fn bufwrite64b(buf: *mut Buffer, x: u64) {
    buf_push_bytes(buf, &x.to_be_bytes());
}
/// A fresh buffer holding `bytes`.
///
/// The C signature took a count followed by that many varargs, and trusted the
/// caller to keep the two in agreement. A slice carries its own length, so the
/// count is gone -- and with it the last use of the `c_variadic` nightly
/// feature in this module.
///
/// No longer `extern "C"`/`#[no_mangle]`: a Rust slice is a fat pointer and has
/// no C spelling, so claiming the C ABI would have been a lie (rustc says so via
/// `improper_ctypes_definitions`). Nothing outside the crate called it -- the
/// public ABI is the four `otfccbuild_*`/`otfcc_get_buf_*` symbols -- so the two
/// names simply leave `scripts/abi-exports.txt`.
pub unsafe fn bufninit(bytes: &[u8]) -> *mut Buffer {
    let buf: *mut Buffer = bufnew();
    buf_push_bytes(buf, bytes);
    return buf;
}

/// Append `bytes`, growing the buffer first. See [`bufninit`] for why there is
/// no separate count.
pub unsafe fn bufnwrite8(buf: *mut Buffer, bytes: &[u8]) {
    buf_push_bytes(buf, bytes);
}
pub unsafe fn bufwrite_str(buf: *mut Buffer, str: *const ::core::ffi::c_char) {
    if str.is_null() {
        return;
    }
    let cstr = ::core::ffi::CStr::from_ptr(str);
    let bytes = cstr.to_bytes();
    if bytes.is_empty() {
        return;
    }
    buf_push_bytes(buf, bytes);
}
pub unsafe fn bufwrite_bytes(buf: *mut Buffer, len: usize, str: *const u8) {
    if str.is_null() || len == 0 {
        return;
    }
    buf_push_bytes(buf, ::core::slice::from_raw_parts(str, len));
}
pub unsafe fn bufwrite_buf(buf: *mut Buffer, that: *mut Buffer) {
    if that.is_null() {
        return;
    }
    // `.to_vec()` (not a borrow held across `buf_push_bytes`): `buf` and
    // `that` could in principle be the same allocation, and a borrow of
    // `(*that).data` can't coexist with the `&mut` `buf_push_bytes` takes
    // on `(*buf).data` if they alias -- an owned copy sidesteps the
    // question entirely, at the cost of one extra allocation for what was
    // already a copy either way.
    let bytes = (*that).data.clone();
    buf_push_bytes(buf, &bytes);
}
pub unsafe fn bufwrite_bufdel(buf: *mut Buffer, that: *mut Buffer) {
    if that.is_null() {
        return;
    }
    bufwrite_buf(buf, that);
    buffree(that);
}
pub unsafe fn buflongalign(buf: *mut Buffer) {
    let cp: usize = (*buf).cursor;
    bufseek(buf, buflen(buf));
    let padding = buflen(buf).wrapping_rem(4);
    if (1..4).contains(&padding) {
        for _ in padding..4 {
            bufwrite8(buf, 0);
        }
    }
    bufseek(buf, cp);
}
pub unsafe fn bufping16b(buf: *mut Buffer, offset: *mut usize, cp: *mut usize) {
    bufwrite16b(buf, *offset as u16);
    *cp = (*buf).cursor;
    bufseek(buf, *offset);
}
pub unsafe fn bufping16bd(buf: *mut Buffer, offset: *mut usize, shift: *mut usize, cp: *mut usize) {
    bufwrite16b(buf, (*offset).wrapping_sub(*shift) as u16);
    *cp = (*buf).cursor;
    bufseek(buf, *offset);
}
pub unsafe fn bufpong(buf: *mut Buffer, offset: *mut usize, cp: *mut usize) {
    *offset = (*buf).cursor;
    bufseek(buf, *cp);
}
pub unsafe fn bufpingpong16b(
    buf: *mut Buffer,
    that: *mut Buffer,
    offset: *mut usize,
    cp: *mut usize,
) {
    bufwrite16b(buf, *offset as u16);
    *cp = (*buf).cursor;
    bufseek(buf, *offset);
    bufwrite_bufdel(buf, that);
    *offset = (*buf).cursor;
    bufseek(buf, *cp);
}
// Every byte of an OpenType file leaves the program through these functions,
// so their endianness and cursor bookkeeping are the crate's most
// consequential low-level contract. The byte-for-byte comparison against the C
// build covers them only indirectly (and only for the byte sequences the test
// payloads happen to produce); these tests state the contract directly.
#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn contents(buf: *mut Buffer) -> Vec<u8> {
        (*buf).data.clone()
    }

    #[test]
    fn fixed_width_writes_are_big_endian() {
        unsafe {
            let buf = bufnew();
            bufwrite16b(buf, 0x1234);
            bufwrite32b(buf, 0xdeadbeef);
            bufwrite64b(buf, 0x0102030405060708);
            assert_eq!(
                contents(buf),
                vec![
                    0x12, 0x34, // 16b
                    0xde, 0xad, 0xbe, 0xef, // 32b
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // 64b
                ]
            );
            buffree(buf);
        }
    }

    #[test]
    fn fixed_width_writes_are_little_endian() {
        unsafe {
            let buf = bufnew();
            bufwrite16l(buf, 0x1234);
            bufwrite32l(buf, 0xdeadbeef);
            bufwrite64l(buf, 0x0102030405060708);
            assert_eq!(
                contents(buf),
                vec![
                    0x34, 0x12, // 16l
                    0xef, 0xbe, 0xad, 0xde, // 32l
                    0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, // 64l
                ]
            );
            buffree(buf);
        }
    }

    #[test]
    fn write24_keeps_only_the_low_three_bytes() {
        // The high byte of the u32 argument is dropped, matching the original
        // shift-mask expansion which never touched bits 24-31.
        unsafe {
            let buf = bufnew();
            bufwrite24b(buf, 0xaabbccdd);
            bufwrite24l(buf, 0xaabbccdd);
            assert_eq!(contents(buf), vec![0xbb, 0xcc, 0xdd, 0xdd, 0xcc, 0xbb]);
            buffree(buf);
        }
    }

    #[test]
    fn writes_advance_both_cursor_and_length() {
        unsafe {
            let buf = bufnew();
            assert_eq!(buflen(buf), 0);
            assert_eq!(bufpos(buf), 0);
            bufwrite8(buf, 0xff);
            bufwrite16b(buf, 0);
            assert_eq!(bufpos(buf), 3);
            assert_eq!(buflen(buf), 3);
            buffree(buf);
        }
    }

    #[test]
    fn seeking_back_overwrites_in_place_without_shrinking() {
        unsafe {
            let buf = bufnew();
            bufwrite32b(buf, 0);
            bufseek(buf, 1);
            bufwrite8(buf, 0xab);
            assert_eq!(contents(buf), vec![0x00, 0xab, 0x00, 0x00]);
            assert_eq!(buflen(buf), 4, "length must not shrink to the cursor");
            buffree(buf);
        }
    }

    #[test]
    fn longalign_pads_to_a_multiple_of_four_and_restores_the_cursor() {
        unsafe {
            let buf = bufnew();
            for _ in 0..5 {
                bufwrite8(buf, 0x11);
            }
            bufseek(buf, 2);
            buflongalign(buf);
            assert_eq!(buflen(buf), 8, "5 bytes padded up to 8");
            assert_eq!(bufpos(buf), 2, "cursor restored");
            assert_eq!(contents(buf)[5..], [0, 0, 0]);

            // Already aligned: nothing added.
            bufseek(buf, buflen(buf));
            buflongalign(buf);
            assert_eq!(buflen(buf), 8);
            buffree(buf);
        }
    }

    #[test]
    fn clear_resets_length_but_keeps_the_capacity() {
        unsafe {
            let buf = bufnew();
            bufwrite32b(buf, 0xdeadbeef);
            let capacity_before = (*buf).data.capacity();
            bufclear(buf);
            assert_eq!(buflen(buf), 0);
            assert_eq!(bufpos(buf), 0);
            assert_eq!(
                (*buf).data.capacity(),
                capacity_before,
                "Vec::clear keeps the backing allocation, same as the old size=0/free=size+free bookkeeping"
            );
            buffree(buf);
        }
    }

    #[test]
    fn write_buf_appends_the_source_contents() {
        unsafe {
            let dst = bufnew();
            let src = bufnew();
            bufwrite8(dst, 0x01);
            bufwrite16b(src, 0x0203);
            bufwrite_buf(dst, src);
            assert_eq!(contents(dst), vec![0x01, 0x02, 0x03]);
            assert_eq!(buflen(src), 2, "bufwrite_buf must not consume the source");
            buffree(src);
            buffree(dst);
        }
    }

    #[test]
    fn ping_pong_backpatches_a_16bit_offset() {
        // The offset-backpatching idiom used throughout the table writers:
        // reserve a 16-bit offset slot, jump to where the pointed-at data
        // goes, write it, then come back.
        unsafe {
            let buf = bufnew();
            let mut offset: usize = 4; // data will start at byte 4
            let mut cp: usize = 0;
            bufwrite16b(buf, 0xffff); // placeholder we'll leave alone
            bufping16b(buf, &mut offset, &mut cp);
            assert_eq!(bufpos(buf), 4, "jumped to the data position");
            assert_eq!(cp, 4, "saved the return position (just after the slot)");
            bufwrite32b(buf, 0xcafebabe);
            bufpong(buf, &mut offset, &mut cp);
            assert_eq!(offset, 8, "offset advanced past the data just written");
            assert_eq!(bufpos(buf), 4, "returned to the saved position");
            assert_eq!(
                contents(buf),
                vec![0xff, 0xff, 0x00, 0x04, 0xca, 0xfe, 0xba, 0xbe]
            );
            buffree(buf);
        }
    }
}

pub unsafe fn bufprint(buf: *mut Buffer) {
    for (j, &byte) in (*buf).data.iter().enumerate() {
        if j % 16 != 0 {
            fprintf(stderr, b" \0" as *const u8 as *const ::core::ffi::c_char);
        }
        fprintf(
            stderr,
            b"%02X\0" as *const u8 as *const ::core::ffi::c_char,
            byte as i32,
        );
        if j % 16 == 15 {
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
}
