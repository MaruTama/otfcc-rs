#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};


use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::font_reader::{FontReader, ReadError};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, logger_log_sds};

use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{GlyphId};
use crate::font::caryll_sfnt::{Packet};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite8};


#[repr(C)]
pub struct LtshTable {
    pub version: u16,
    pub num_glyphs: GlyphId,
    pub y_pels: *mut u8,
}
// Stage 6-4 pilot for `Font`'s `*mut X`-typed table fields: `y_pels` is the
// only allocation this struct owns, so `Box<LtshTable>` (via `Font.ltsh:
// Option<Box<LtshTable>>`) plus this `Drop` impl replaces the entire
// `LtshTableElementInterface` vtable that used to exist here -- grepping
// confirmed only `.free` was ever called from outside this file (from
// `font/caryll_font.rs`'s table disposal), and `.init`/`.copy`/`.create`/
// `.dispose` were never called at all (`otfcc_read_ltsh`/`stat_ltsh`
// already built via `__caryll_allocate_clean` directly, not through the
// vtable's `.create`). `Copy`/`Clone` dropped: a `Drop` impl and `Copy`
// are mutually exclusive, and `y_pels` needing single ownership means
// `Copy` was already semantically wrong before this PR, just unenforced.
impl Drop for LtshTable {
    fn drop(&mut self) {
        unsafe {
            if !self.y_pels.is_null() {
                free(self.y_pels as *mut ::core::ffi::c_void);
                self.y_pels = ::core::ptr::null_mut::<u8>();
            }
        }
    }
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

pub unsafe fn otfcc_read_ltsh(
    packet: &Packet,
    options: &Options,
) -> Option<Box<LtshTable>> {
    let table = packet.pieces.iter().find(|p| p.tag == crate::tag::TAG_LTSH)?;
    let (version, num_glyphs, pels) = match parse_ltsh(&table.data) {
        Ok(parsed) => parsed,
        Err(_) => {
            logger_log_sds(
                options.logger,
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'LTSH' corrupted.\n"),
            );
            return None;
        }
    };
    let y_pels = __caryll_allocate_clean(
        (::core::mem::size_of::<u8>() as usize).wrapping_mul(num_glyphs as usize),
        18 as ::core::ffi::c_ulong,
    ) as *mut u8;
    ::core::ptr::copy_nonoverlapping(pels.as_ptr(), y_pels, num_glyphs as usize);
    Some(Box::new(LtshTable { version, num_glyphs, y_pels }))
}
// `Option<&LtshTable>`, not `*const LtshTable`: internal-only call (never
// crosses the real FFI boundary, see `rust/README.md`), and the crate's
// only caller now hands `(*font).ltsh.as_deref()` from `Font.ltsh:
// Option<Box<LtshTable>>`.
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_ltsh(
    ltsh: Option<&LtshTable>,
) -> *mut Buffer {
    let ltsh = match ltsh {
        Some(l) => l,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, ltsh.num_glyphs as u16);
    let mut j: u16 = 0 as u16;
    while (j as ::core::ffi::c_int) < ltsh.num_glyphs as ::core::ffi::c_int {
        bufwrite8(buf, *ltsh.y_pels.offset(j as isize));
        j = j.wrapping_add(1);
    }
    return buf;
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
