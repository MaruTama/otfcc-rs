//! Named constants for the SFNT/OpenType four-byte tags this crate matches
//! on. c2rust transpiled every tag comparison and table-directory entry as
//! a raw decimal `u32` literal (e.g. `1835365473` for `'meta'`), which is
//! opaque to read and easy to mistype when hand-editing. These constants
//! are derived from the tag's byte string with `u32::from_be_bytes`, so the
//! value is guaranteed identical to the literal it replaces while staying
//! legible.
//!
//! A few tags have a second, legacy `_ALT` form: otfcc accepts an
//! alternate spelling (`_` in place of `/` or a trailing space) for tags
//! that aren't valid as bare filesystem/JSON-key characters.

pub const TAG_BASE: u32 = u32::from_be_bytes(*b"BASE");
pub const TAG_CFF: u32 = u32::from_be_bytes(*b"CFF ");
pub const TAG_CFF_ALT: u32 = u32::from_be_bytes(*b"CFF_");
pub const TAG_COLR: u32 = u32::from_be_bytes(*b"COLR");
pub const TAG_CPAL: u32 = u32::from_be_bytes(*b"CPAL");
pub const TAG_DSIG: u32 = u32::from_be_bytes(*b"DSIG");
pub const TAG_GDEF: u32 = u32::from_be_bytes(*b"GDEF");
pub const TAG_GPOS: u32 = u32::from_be_bytes(*b"GPOS");
pub const TAG_GSUB: u32 = u32::from_be_bytes(*b"GSUB");
pub const TAG_LTSH: u32 = u32::from_be_bytes(*b"LTSH");
pub const TAG_OS_2: u32 = u32::from_be_bytes(*b"OS/2");
pub const TAG_OS_2_ALT: u32 = u32::from_be_bytes(*b"OS_2");
pub const TAG_SVG: u32 = u32::from_be_bytes(*b"SVG ");
pub const TAG_SVG_ALT: u32 = u32::from_be_bytes(*b"SVG_");
pub const TAG_TSI0: u32 = u32::from_be_bytes(*b"TSI0");
pub const TAG_TSI1: u32 = u32::from_be_bytes(*b"TSI1");
pub const TAG_TSI2: u32 = u32::from_be_bytes(*b"TSI2");
pub const TAG_TSI3: u32 = u32::from_be_bytes(*b"TSI3");
pub const TAG_TSI5: u32 = u32::from_be_bytes(*b"TSI5");
pub const TAG_VDMX: u32 = u32::from_be_bytes(*b"VDMX");
pub const TAG_VORG: u32 = u32::from_be_bytes(*b"VORG");
pub const TAG_CMAP: u32 = u32::from_be_bytes(*b"cmap");
pub const TAG_CVT: u32 = u32::from_be_bytes(*b"cvt ");
pub const TAG_CVT_ALT: u32 = u32::from_be_bytes(*b"cvt_");
pub const TAG_DLNG: u32 = u32::from_be_bytes(*b"dlng");
pub const TAG_FPGM: u32 = u32::from_be_bytes(*b"fpgm");
pub const TAG_FVAR: u32 = u32::from_be_bytes(*b"fvar");
pub const TAG_GASP: u32 = u32::from_be_bytes(*b"gasp");
pub const TAG_GLYF: u32 = u32::from_be_bytes(*b"glyf");
pub const TAG_GVAR: u32 = u32::from_be_bytes(*b"gvar");
pub const TAG_HDMX: u32 = u32::from_be_bytes(*b"hdmx");
pub const TAG_HEAD: u32 = u32::from_be_bytes(*b"head");
pub const TAG_HHEA: u32 = u32::from_be_bytes(*b"hhea");
pub const TAG_HMTX: u32 = u32::from_be_bytes(*b"hmtx");
pub const TAG_LOCA: u32 = u32::from_be_bytes(*b"loca");
pub const TAG_MAXP: u32 = u32::from_be_bytes(*b"maxp");
pub const TAG_META: u32 = u32::from_be_bytes(*b"meta");
pub const TAG_NAME: u32 = u32::from_be_bytes(*b"name");
pub const TAG_POST: u32 = u32::from_be_bytes(*b"post");
pub const TAG_PREP: u32 = u32::from_be_bytes(*b"prep");
pub const TAG_SLNG: u32 = u32::from_be_bytes(*b"slng");
pub const TAG_VERT: u32 = u32::from_be_bytes(*b"vert");
pub const TAG_VHEA: u32 = u32::from_be_bytes(*b"vhea");
pub const TAG_VMTX: u32 = u32::from_be_bytes(*b"vmtx");
