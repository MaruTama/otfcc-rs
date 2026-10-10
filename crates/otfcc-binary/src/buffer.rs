/// Bytes being written, with a cursor: writes go at the cursor, which can be
/// moved back to patch earlier bytes.
#[derive(Clone, Debug)]
pub struct Buffer {
    pub cursor: usize,
    pub data: Vec<u8>,
}

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

    /// A fresh buffer holding `bytes`.
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
        // `.clear()` keeps the allocation.
        self.data.clear();
    }

    // Writes `bytes` at the cursor and advances it. A write may overwrite
    // bytes already written after a backward seek (offsets are often patched
    // in later); past the end, the buffer grows first, zero-filling any gap
    // between its old length and the cursor.
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
    /// A signed 16-bit value, such as an FWORD (a position or a metric).
    ///
    /// Positions are `f64`, so callers write `buf.write_i16be(x as i16)`.
    /// Never `write_u16be(x as u16)` instead: Rust's float-to-unsigned
    /// conversion saturates, so every negative side bearing or offset would
    /// silently become 0. Going through `i16` gives the two's-complement
    /// bits the font needs (`-41.0` is written as `0xFFD7`).
    pub fn write_i16be(&mut self, x: i16) {
        self.push_bytes(&x.to_be_bytes());
    }
    pub fn write_u24le(&mut self, x: u32) {
        // Only the low 3 bytes are written.
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
        // The high byte of the u32 argument is dropped.
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

    #[test]
    fn write_i16be_writes_negative_positions_as_twos_complement() {
        let mut b = Buffer::new();
        for x in [-41.0f64, -1.0, -32768.0, 41.9, -41.9, 0.0, 65535.0] {
            b.write_i16be(x as i16);
        }
        assert_eq!(
            b.data,
            [0xff, 0xd7, 0xff, 0xff, 0x80, 0x00, 0x00, 41, 0xff, 0xd7, 0x00, 0x00, 0x7f, 0xff]
        );
        // ...where a direct unsigned cast would have written 0.
        assert_eq!(-41.0f64 as u16, 0);
    }

}
