//! Bounds-checked reading over a table's raw bytes, replacing
//! `support/binio.rs`'s `read_*` family (a bare `*const u8` with no length,
//! trusted by 465 call sites across 41 files -- see rust/README.md's Phase 5
//! plan, "Stage 7-1"). Every read here is checked against the buffer's
//! actual length before it happens; on failure it returns a `ReadError`
//! instead of reading past the end.
//!
//! This intentionally changes behavior on malformed input: where the old
//! `read_*` functions would read (and the caller would go on to use)
//! whatever bytes happened to be adjacent in memory, a `FontReader` call
//! fails cleanly. Each table reader migrated onto this converts that
//! `Result::Err` into the same "log a warning, skip this table" outcome the
//! table already used for its one or two existing length checks -- so a
//! well-formed font's output is unaffected (checked by the golden-fixture
//! comparison), and a malformed one that used to read/copy garbage now just
//! loses that one table instead.
//!
//! `require()` guards the multiply-then-compare shape a `count`-driven loop
//! needs (`count * stride` bytes available) with `checked_mul`/`checked_add`
//! rather than the wrapping arithmetic several existing hand-written guards
//! use, which is itself an unchecked-overflow bug class this migration
//! closes as it goes (see `table/cmap.rs`'s `n_groups` guard in the plan's
//! own writeup, for the shape being replaced).

/// A read that would go past the end of the buffer. Carries enough to write
/// a useful log line, but table readers migrated onto this generally just
/// match `Err(_)` and log a fixed "table '...' corrupted" message, matching
/// the wording every already-migrated reader already used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadError {
    pub needed: usize,
    pub available: usize,
}

#[derive(Clone, Copy)]
pub struct FontReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> FontReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        FontReader { data, pos: 0 }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    fn err(&self, needed: usize) -> ReadError {
        ReadError {
            needed,
            available: self.remaining(),
        }
    }

    fn require(&self, n: usize) -> Result<(), ReadError> {
        match self.pos.checked_add(n) {
            Some(end) if end <= self.data.len() => Ok(()),
            _ => Err(self.err(n)),
        }
    }

    /// `count * stride` bytes available from the current position, checked
    /// against overflow in the multiplication itself (an attacker-supplied
    /// `count` can be large enough that `count * stride` overflows `usize`
    /// on its own, before the length comparison ever runs).
    pub fn require_room(&self, count: usize, stride: usize) -> Result<(), ReadError> {
        let need = count
            .checked_mul(stride)
            .ok_or_else(|| self.err(usize::MAX))?;
        self.require(need)
    }

    pub fn skip(&mut self, n: usize) -> Result<(), ReadError> {
        self.require(n)?;
        self.pos += n;
        Ok(())
    }

    /// The next `n` bytes, without advancing -- for a caller that wants to
    /// validate a whole span before starting to interpret it piecewise.
    pub fn peek_bytes(&self, n: usize) -> Result<&'a [u8], ReadError> {
        self.require(n)?;
        Ok(&self.data[self.pos..self.pos + n])
    }

    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8], ReadError> {
        let s = self.peek_bytes(n)?;
        self.pos += n;
        Ok(s)
    }

    pub fn u8(&mut self) -> Result<u8, ReadError> {
        let b = self.bytes(1)?;
        Ok(b[0])
    }

    pub fn i8(&mut self) -> Result<i8, ReadError> {
        Ok(self.u8()? as i8)
    }

    pub fn u16(&mut self) -> Result<u16, ReadError> {
        let b = self.bytes(2)?;
        Ok(u16::from_be_bytes([b[0], b[1]]))
    }

    pub fn i16(&mut self) -> Result<i16, ReadError> {
        Ok(self.u16()? as i16)
    }

    pub fn u24(&mut self) -> Result<u32, ReadError> {
        let b = self.bytes(3)?;
        Ok(u32::from_be_bytes([0, b[0], b[1], b[2]]))
    }

    pub fn u32(&mut self) -> Result<u32, ReadError> {
        let b = self.bytes(4)?;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub fn i32(&mut self) -> Result<i32, ReadError> {
        Ok(self.u32()? as i32)
    }

    pub fn u64(&mut self) -> Result<u64, ReadError> {
        let b = self.bytes(8)?;
        Ok(u64::from_be_bytes([
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
        ]))
    }

    /// A fresh reader positioned at an absolute offset from the start of
    /// this reader's own buffer -- for following an in-table offset field
    /// (a subtable offset, a string-heap offset) without losing the
    /// original buffer's bounds.
    pub fn at(&self, offset: usize) -> Result<Self, ReadError> {
        if offset > self.data.len() {
            return Err(ReadError {
                needed: offset,
                available: self.data.len(),
            });
        }
        Ok(FontReader {
            data: self.data,
            pos: offset,
        })
    }

    /// A fresh reader over exactly `[offset, offset + len)` of this
    /// reader's own buffer -- for a subtable that has both an offset and an
    /// explicit length, so the returned reader can't run past the
    /// subtable's own end even if the outer buffer has more data after it.
    pub fn sub(&self, offset: usize, len: usize) -> Result<FontReader<'a>, ReadError> {
        let end = offset.checked_add(len).ok_or(ReadError {
            needed: len,
            available: 0,
        })?;
        if end > self.data.len() {
            return Err(ReadError {
                needed: end,
                available: self.data.len(),
            });
        }
        Ok(FontReader {
            data: &self.data[offset..end],
            pos: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_big_endian_and_advances() {
        let mut r = FontReader::new(&[0x00, 0x01, 0xFF, 0xFE, 0x00, 0x00, 0x00, 0x2A]);
        assert_eq!(r.u16().unwrap(), 1);
        assert_eq!(r.u16().unwrap(), 0xFFFE);
        assert_eq!(r.u32().unwrap(), 42);
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn signed_reads_match_twos_complement() {
        let mut r = FontReader::new(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(r.i16().unwrap(), -1);
        assert_eq!(r.i32().unwrap(), -1);
    }

    #[test]
    fn u24_zero_extends_into_u32() {
        let mut r = FontReader::new(&[0x01, 0x02, 0x03]);
        assert_eq!(r.u24().unwrap(), 0x010203);
    }

    #[test]
    fn read_past_the_end_errs_instead_of_reading_oob() {
        let mut r = FontReader::new(&[0x00, 0x01]);
        assert!(r.u32().is_err());
        // A failed read must not have moved the cursor -- otherwise a
        // caller that ignores one error and keeps reading would desync.
        assert_eq!(r.pos(), 0);
    }

    #[test]
    fn skip_and_bytes_bounds_check() {
        let mut r = FontReader::new(&[1, 2, 3, 4, 5]);
        r.skip(2).unwrap();
        assert_eq!(r.bytes(2).unwrap(), &[3, 4]);
        assert!(r.bytes(2).is_err()); // only 1 byte left
        assert!(r.skip(100).is_err());
    }

    #[test]
    fn require_room_rejects_overflowing_multiplication() {
        let r = FontReader::new(&[0; 16]);
        // A count large enough that count * stride overflows usize on its
        // own, before any comparison against the real (tiny) buffer length
        // -- this is exactly the `n_groups` class of bug the plan's Stage
        // 7-1 writeup documents in table/cmap.rs's existing hand-written
        // guard (`length_limit < 16 + 12 * n_groups`, computed with
        // `wrapping_add`/`wrapping_mul` so the overflow wraps silently
        // instead of failing the check).
        assert!(r.require_room(usize::MAX / 2, 100).is_err());
        assert!(r.require_room(2, 8).is_ok()); // 16 bytes, exactly fits
        assert!(r.require_room(3, 8).is_err()); // 24 bytes, doesn't
    }

    #[test]
    fn at_seeks_to_an_absolute_offset_within_bounds() {
        let r = FontReader::new(&[10, 20, 30, 40, 50]);
        let mut r2 = r.at(3).unwrap();
        assert_eq!(r2.u8().unwrap(), 40);
        assert!(r.at(6).is_err()); // one past the end
        assert!(r.at(5).is_ok()); // exactly at the end: zero bytes left, valid
    }

    #[test]
    fn sub_bounds_a_subtable_independent_of_the_outer_buffer() {
        let r = FontReader::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut inner = r.sub(2, 3).unwrap();
        assert_eq!(inner.bytes(3).unwrap(), &[2, 3, 4]);
        // The subtable's own bounds stop at its declared length, not the
        // outer buffer's -- reading a 4th byte fails even though the outer
        // buffer has plenty more after offset 5.
        assert!(inner.u8().is_err());
        assert!(r.sub(8, 3).is_err()); // offset + len runs past the end
        assert!(r.sub(usize::MAX, 1).is_err()); // overflow in offset + len
    }

    #[test]
    fn peek_does_not_advance() {
        let mut r = FontReader::new(&[1, 2, 3]);
        assert_eq!(r.peek_bytes(2).unwrap(), &[1, 2]);
        assert_eq!(r.pos(), 0);
        assert_eq!(r.bytes(2).unwrap(), &[1, 2]);
        assert_eq!(r.pos(), 2);
    }
}
