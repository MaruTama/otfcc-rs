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

// Stage 9: `Buffer`'s data (`{cursor, data: Vec<u8>}`) has been fully safe
// since 7-2-e -- the unsafety crate-wide was entirely in a free-function
// shell (`bufnew`/`bufwrite*(buf, ...)`/etc.), kept raw-pointer-shaped on
// purpose so `table/*/build.rs` call sites could stay textually identical
// to the old C idiom during the mechanical c2rust port. Every one of the
// ~775 original call sites has since migrated to this safe `impl` directly
// and the free-function shell itself has been deleted (Stage 9, Phase 16).
impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            cursor: 0,
            data: Vec::new(),
        }
    }

    /// A fresh buffer holding `bytes`. Replaces the old `bufninit`'s body.
    pub fn from_bytes(bytes: &[u8]) -> Buffer {
        let mut b = Buffer::new();
        b.write_bytes(bytes);
        b
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn pos(&self) -> usize {
        self.cursor
    }
    pub fn seek(&mut self, pos: usize) {
        self.cursor = pos;
    }
    pub fn clear(&mut self) {
        self.cursor = 0;
        // `.clear()`, not `= Vec::new()`: drops every element but keeps the
        // backing allocation, the same "reset length, keep the allocation"
        // contract `size = 0` + `free = size + free` used to give by hand.
        self.data.clear();
    }

    // Pushes `bytes` at the cursor, growing the buffer first if needed, and
    // advances the cursor. Every fixed-width `write_*` method below is
    // exactly this plus an endian-ordered byte array (to_le_bytes/
    // to_be_bytes), which replaces c2rust's manual per-byte shift-mask-store
    // expansion.
    //
    // A write can seek backward and overwrite already-written bytes in
    // place (the hand-rolled offset-backpatching idiom real call sites use,
    // e.g. `table/cmap.rs`'s format4 segment-count backpatch) -- so this is
    // not a plain `Vec::extend`. If the write fits entirely within the
    // already-written region (`cursor + bytes.len() <= data.len()`), it's a
    // pure in-place overwrite; otherwise `resize` grows the `Vec` first
    // (zero-filling any gap between the old length and `cursor`, matching
    // what a fresh `realloc` over calloc'd memory used to leave there)
    // before the same slice-copy runs either way.
    fn push_bytes(&mut self, bytes: &[u8]) {
        let cursor = self.cursor;
        let end = cursor.wrapping_add(bytes.len());
        if self.data.len() < end {
            self.data.resize(end, 0);
        }
        self.data[cursor..end].copy_from_slice(bytes);
        self.cursor = end;
    }

    pub fn write_u8(&mut self, byte: u8) {
        self.push_bytes(&[byte]);
    }
    pub fn write_u16le(&mut self, x: u16) {
        self.push_bytes(&x.to_le_bytes());
    }
    pub fn write_u16be(&mut self, x: u16) {
        self.push_bytes(&x.to_be_bytes());
    }
    pub fn write_u24le(&mut self, x: u32) {
        // Low 3 bytes only, matching the original's shift-mask expansion,
        // which never touched bits 24-31 either.
        self.push_bytes(&x.to_le_bytes()[..3]);
    }
    pub fn write_u24be(&mut self, x: u32) {
        self.push_bytes(&x.to_be_bytes()[1..]);
    }
    pub fn write_u32le(&mut self, x: u32) {
        self.push_bytes(&x.to_le_bytes());
    }
    pub fn write_u32be(&mut self, x: u32) {
        self.push_bytes(&x.to_be_bytes());
    }
    pub fn write_u64le(&mut self, x: u64) {
        self.push_bytes(&x.to_le_bytes());
    }
    pub fn write_u64be(&mut self, x: u64) {
        self.push_bytes(&x.to_be_bytes());
    }
    /// Append `bytes`, growing the buffer first. Replaces `bufnwrite8`/
    /// `bufwrite_bytes`'s bodies.
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.push_bytes(bytes);
    }

    /// Appends `that`'s contents, without consuming it.
    ///
    /// Takes `&that.data` directly rather than cloning it: `self` and
    /// `that` can no longer alias now that every call site goes through
    /// this safe `&mut self`/`&Buffer` signature -- the borrow checker
    /// itself guarantees they're distinct objects, the same guarantee the
    /// old raw-pointer free-function shell (`bufwrite_buf`, deleted once
    /// this method's last raw-pointer bridge went away in Stage 9) could
    /// only get from a by-hand audit of its ~55 call sites.
    pub fn write_buffer(&mut self, that: &Buffer) {
        self.push_bytes(&that.data);
    }
    /// [`write_buffer`], consuming `that`.
    pub fn write_buffer_owned(&mut self, that: Buffer) {
        self.write_buffer(&that);
    }

    /// Pads the buffer's length up to a multiple of 4 bytes, restoring the
    /// cursor afterward.
    pub fn long_align(&mut self) {
        let cp = self.cursor;
        self.seek(self.len());
        let padding = self.len().wrapping_rem(4);
        if (1..4).contains(&padding) {
            for _ in padding..4 {
                self.write_u8(0);
            }
        }
        self.seek(cp);
    }

    // The pair below is the only place a `Buffer` still crosses a raw
    // pointer: the real ABI boundary in `ffi/dll.rs` (`otfccbuild_json_otf`
    // returns `*mut Buffer`) and, during Stage 9's migration, the bridge
    // back into not-yet-converted call sites still using the free-function
    // API below. Not for use anywhere else.
    pub fn into_raw(self) -> *mut Buffer {
        Box::into_raw(Box::new(self))
    }
    /// # Safety
    /// `ptr` must either be null or have come from [`Buffer::into_raw`]
    /// and not have been freed already.
    pub unsafe fn from_raw(ptr: *mut Buffer) -> Option<Buffer> {
        if ptr.is_null() {
            None
        } else {
            Some(*unsafe { Box::from_raw(ptr) })
        }
    }
}

// Every byte of an OpenType file leaves the program through these methods,
// so their endianness and cursor bookkeeping are the crate's most
// consequential low-level contract. The byte-for-byte comparison against the
// C build covers them only indirectly (and only for the byte sequences the
// test payloads happen to produce); these tests state the contract directly.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_width_writes_are_big_endian() {
        let mut buf = Buffer::new();
        buf.write_u16be(0x1234);
        buf.write_u32be(0xdeadbeef);
        buf.write_u64be(0x0102030405060708);
        assert_eq!(
            buf.data,
            vec![
                0x12, 0x34, // 16b
                0xde, 0xad, 0xbe, 0xef, // 32b
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // 64b
            ]
        );
    }

    #[test]
    fn fixed_width_writes_are_little_endian() {
        let mut buf = Buffer::new();
        buf.write_u16le(0x1234);
        buf.write_u32le(0xdeadbeef);
        buf.write_u64le(0x0102030405060708);
        assert_eq!(
            buf.data,
            vec![
                0x34, 0x12, // 16l
                0xef, 0xbe, 0xad, 0xde, // 32l
                0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, // 64l
            ]
        );
    }

    #[test]
    fn write24_keeps_only_the_low_three_bytes() {
        // The high byte of the u32 argument is dropped, matching the original
        // shift-mask expansion which never touched bits 24-31.
        let mut buf = Buffer::new();
        buf.write_u24be(0xaabbccdd);
        buf.write_u24le(0xaabbccdd);
        assert_eq!(buf.data, vec![0xbb, 0xcc, 0xdd, 0xdd, 0xcc, 0xbb]);
    }

    #[test]
    fn writes_advance_both_cursor_and_length() {
        let mut buf = Buffer::new();
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.pos(), 0);
        buf.write_u8(0xff);
        buf.write_u16be(0);
        assert_eq!(buf.pos(), 3);
        assert_eq!(buf.len(), 3);
    }

    #[test]
    fn seeking_back_overwrites_in_place_without_shrinking() {
        let mut buf = Buffer::new();
        buf.write_u32be(0);
        buf.seek(1);
        buf.write_u8(0xab);
        assert_eq!(buf.data, vec![0x00, 0xab, 0x00, 0x00]);
        assert_eq!(buf.len(), 4, "length must not shrink to the cursor");
    }

    #[test]
    fn longalign_pads_to_a_multiple_of_four_and_restores_the_cursor() {
        let mut buf = Buffer::new();
        for _ in 0..5 {
            buf.write_u8(0x11);
        }
        buf.seek(2);
        buf.long_align();
        assert_eq!(buf.len(), 8, "5 bytes padded up to 8");
        assert_eq!(buf.pos(), 2, "cursor restored");
        assert_eq!(buf.data[5..], [0, 0, 0]);

        // Already aligned: nothing added.
        buf.seek(buf.len());
        buf.long_align();
        assert_eq!(buf.len(), 8);
    }

    #[test]
    fn clear_resets_length_but_keeps_the_capacity() {
        let mut buf = Buffer::new();
        buf.write_u32be(0xdeadbeef);
        let capacity_before = buf.data.capacity();
        buf.clear();
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.pos(), 0);
        assert_eq!(
            buf.data.capacity(),
            capacity_before,
            "Vec::clear keeps the backing allocation, same as the old size=0/free=size+free bookkeeping"
        );
    }

    #[test]
    fn write_buffer_appends_the_source_contents() {
        let mut dst = Buffer::new();
        let mut src = Buffer::new();
        dst.write_u8(0x01);
        src.write_u16be(0x0203);
        dst.write_buffer(&src);
        assert_eq!(dst.data, vec![0x01, 0x02, 0x03]);
        assert_eq!(src.len(), 2, "write_buffer must not consume the source");
    }

    #[test]
    fn seek_and_rewrite_backpatches_a_16bit_offset() {
        // The hand-rolled offset-backpatching idiom real call sites use
        // (e.g. `table/cmap.rs`'s format4 segment-count backpatch): reserve
        // a slot, write the data whose position it names, then seek back
        // and overwrite the placeholder with the now-known offset.
        let mut buf = Buffer::new();
        buf.write_u16be(0xffff); // placeholder we'll overwrite
        let data_start = buf.pos();
        buf.write_u32be(0xcafebabe);
        let end = buf.pos();
        buf.seek(0);
        buf.write_u16be(data_start as u16);
        buf.seek(end);
        assert_eq!(buf.data, vec![0x00, 0x02, 0xca, 0xfe, 0xba, 0xbe]);
        assert_eq!(buf.pos(), end, "cursor left at the end, not the patched slot");
    }
}
