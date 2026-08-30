#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::logger::{LOG_VL_IMPORTANT, LoggerType, logger_log_sds};
use crate::support::font_reader::{FontReader, ReadError};

use crate::font::caryll_sfnt::Packet;
use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::primitives::GlyphId;

// Stage 6-4 pilot for `Font`'s `*mut X`-typed table fields Box-ified the
// outer `LtshTable` itself; Stage 7-2-c "inner Vec化" finishes the job here:
// `y_pels` was the only allocation this struct owned, so `Vec<u8>` plus its
// own drop glue replaces the manual `free`-based `impl Drop` that used to
// live here (which itself had replaced the entire
// `LtshTableElementInterface` vtable). `num_glyphs` is kept as a real field
// (not collapsed into `y_pels.len()`, unlike `CvtTable.length`): besides
// sizing `y_pels`, it is independently compared against `Glyf`'s glyph
// count at `otf_reader/unconsolidate.rs`'s `merge_ltsh` (a `.min()` clamp),
// so it carries information beyond a plain redundant length.
pub struct LtshTable {
    pub version: u16,
    pub num_glyphs: GlyphId,
    pub y_pels: Vec<u8>,
}
// Parses into owned values only -- no allocation happens until every read
// has already succeeded, so an `Err` here never leaves a partial `y_pels`
// buffer to free. Same shape as `table/post.rs::parse_post`.
fn parse_ltsh(data: &[u8]) -> Result<(u16, GlyphId, &[u8]), ReadError> {
    let mut r = FontReader::new(data);
    let version = r.u16()?;
    let num_glyphs = r.u16()? as GlyphId;
    let pels = r.bytes(num_glyphs as usize)?;
    Ok((version, num_glyphs, pels))
}

pub fn otfcc_read_ltsh(packet: &Packet, options: &Options) -> Option<Box<LtshTable>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_LTSH)?;
    let (version, num_glyphs, pels) = match parse_ltsh(&table.data) {
        Ok(parsed) => parsed,
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'LTSH' corrupted.\n"),
            );
            return None;
        }
    };
    let y_pels = pels.to_vec();
    Some(Box::new(LtshTable {
        version,
        num_glyphs,
        y_pels,
    }))
}
// `Option<&LtshTable>`, not `*const LtshTable`: internal-only call (never
// crosses the real FFI boundary, see `rust/README.md`), and the crate's
// only caller now hands `(*font).ltsh.as_deref()` from `Font.ltsh:
// Option<Box<LtshTable>>`.
pub fn otfcc_build_ltsh(ltsh: Option<&LtshTable>) -> Option<Buffer> {
    let ltsh = ltsh?;
    let mut buf = Buffer::new();
    buf.write_u16be(0_u16);
    buf.write_u16be(ltsh.num_glyphs);
    let mut j: u16 = 0_u16;
    while (j as i32) < ltsh.num_glyphs as i32 {
        buf.write_u8(ltsh.y_pels[j as usize]);
        j = j.wrapping_add(1);
    }
    Some(buf)
}

#[cfg(test)]
mod parse_ltsh_tests {
    use super::*;

    #[test]
    fn valid_header_resolves_the_pel_array() {
        let data: &[u8] = &[0x00, 0x01, 0x00, 0x03, 10, 20, 30];
        let (version, num_glyphs, pels) = parse_ltsh(data).unwrap();
        assert_eq!(version, 1);
        assert_eq!(num_glyphs, 3);
        assert_eq!(pels, &[10, 20, 30]);
    }

    #[test]
    fn truncated_header_errs() {
        // No committed payload has an LTSH table (checked by hand via
        // otfccdump on every tests/payload/*.ttf), so this direct test is
        // this table's only coverage. otfcc_read_ltsh used to read this
        // unconditionally regardless of the table's real length.
        assert!(parse_ltsh(&[0x00, 0x01]).is_err());
    }

    #[test]
    fn pel_array_shorter_than_num_glyphs_errs() {
        // num_glyphs says 5 but only 1 pel byte actually follows the
        // 4-byte header -- this used to memcpy 5 bytes regardless.
        let data: &[u8] = &[0x00, 0x01, 0x00, 0x05, 10];
        assert!(parse_ltsh(data).is_err());
    }
}
