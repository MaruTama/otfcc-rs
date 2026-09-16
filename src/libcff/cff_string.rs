#![allow(unsafe_op_in_unsafe_fn)]
// Stage 6 removes this; see RUST_MIGRATION.md
// `get_cff_sid` (renamed from `sdsget_cff_sid` once `vendor/sds.rs` was
// removed -- it never built an `sds`, just returned `Option<Vec<u8>>`, so
// the name was a pure holdover) has its only callers as direct Rust call
// sites (never a real FFI boundary) -- goes away with the vtable/
// extern "C" cleanup, same as every other instance of this allow.
#![allow(improper_ctypes_definitions)]
use crate::libcff::cff_index::CffIndex;
use crate::support::primitives::Arity;

static STRING_STANDARD: [&str; 391] = [
    ".notdef",
    "space",
    "exclam",
    "quotedbl",
    "numbersign",
    "dollar",
    "percent",
    "ampersand",
    "quoteright",
    "parenleft",
    "parenright",
    "asterisk",
    "plus",
    "comma",
    "hyphen",
    "period",
    "slash",
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "colon",
    "semicolon",
    "less",
    "equal",
    "greater",
    "question",
    "at",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "bracketleft",
    "backslash",
    "bracketright",
    "asciicircum",
    "underscore",
    "quoteleft",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "braceleft",
    "bar",
    "braceright",
    "asciitilde",
    "exclamdown",
    "cent",
    "sterling",
    "fraction",
    "yen",
    "florin",
    "section",
    "currency",
    "quotesingle",
    "quotedblleft",
    "guillemotleft",
    "guilsinglleft",
    "guilsinglright",
    "fi",
    "fl",
    "endash",
    "dagger",
    "daggerdbl",
    "periodcentered",
    "paragraph",
    "bullet",
    "quotesinglbase",
    "quotedblbase",
    "quotedblright",
    "guillemotright",
    "ellipsis",
    "perthousand",
    "questiondown",
    "grave",
    "acute",
    "circumflex",
    "tilde",
    "macron",
    "breve",
    "dotaccent",
    "dieresis",
    "ring",
    "cedilla",
    "hungarumlaut",
    "ogonek",
    "caron",
    "emdash",
    "AE",
    "ordfeminine",
    "Lslash",
    "Oslash",
    "OE",
    "ordmasculine",
    "ae",
    "dotlessi",
    "lslash",
    "oslash",
    "oe",
    "germandbls",
    "onesuperior",
    "logicalnot",
    "mu",
    "trademark",
    "Eth",
    "onehalf",
    "plusminus",
    "Thorn",
    "onequarter",
    "divide",
    "brokenbar",
    "degree",
    "thorn",
    "threequarters",
    "twosuperior",
    "registered",
    "minus",
    "eth",
    "multiply",
    "threesuperior",
    "copyright",
    "Aacute",
    "Acircumflex",
    "Adieresis",
    "Agrave",
    "Aring",
    "Atilde",
    "Ccedilla",
    "Eacute",
    "Ecircumflex",
    "Edieresis",
    "Egrave",
    "Iacute",
    "Icircumflex",
    "Idieresis",
    "Igrave",
    "Ntilde",
    "Oacute",
    "Ocircumflex",
    "Odieresis",
    "Ograve",
    "Otilde",
    "Scaron",
    "Uacute",
    "Ucircumflex",
    "Udieresis",
    "Ugrave",
    "Yacute",
    "Ydieresis",
    "Zcaron",
    "aacute",
    "acircumflex",
    "adieresis",
    "agrave",
    "aring",
    "atilde",
    "ccedilla",
    "eacute",
    "ecircumflex",
    "edieresis",
    "egrave",
    "iacute",
    "icircumflex",
    "idieresis",
    "igrave",
    "ntilde",
    "oacute",
    "ocircumflex",
    "odieresis",
    "ograve",
    "otilde",
    "scaron",
    "uacute",
    "ucircumflex",
    "udieresis",
    "ugrave",
    "yacute",
    "ydieresis",
    "zcaron",
    "exclamsmall",
    "Hungarumlautsmall",
    "dollaroldstyle",
    "dollarsuperior",
    "ampersandsmall",
    "Acutesmall",
    "parenleftsuperior",
    "parenrightsuperior",
    "twodotenleader",
    "onedotenleader",
    "zerooldstyle",
    "oneoldstyle",
    "twooldstyle",
    "threeoldstyle",
    "fouroldstyle",
    "fiveoldstyle",
    "sixoldstyle",
    "sevenoldstyle",
    "eightoldstyle",
    "nineoldstyle",
    "commasuperior",
    "threequartersemdash",
    "periodsuperior",
    "questionsmall",
    "asuperior",
    "bsuperior",
    "centsuperior",
    "dsuperior",
    "esuperior",
    "isuperior",
    "lsuperior",
    "msuperior",
    "nsuperior",
    "osuperior",
    "rsuperior",
    "ssuperior",
    "tsuperior",
    "ff",
    "ffi",
    "ffl",
    "parenleftinferior",
    "parenrightinferior",
    "Circumflexsmall",
    "hyphensuperior",
    "Gravesmall",
    "Asmall",
    "Bsmall",
    "Csmall",
    "Dsmall",
    "Esmall",
    "Fsmall",
    "Gsmall",
    "Hsmall",
    "Ismall",
    "Jsmall",
    "Ksmall",
    "Lsmall",
    "Msmall",
    "Nsmall",
    "Osmall",
    "Psmall",
    "Qsmall",
    "Rsmall",
    "Ssmall",
    "Tsmall",
    "Usmall",
    "Vsmall",
    "Wsmall",
    "Xsmall",
    "Ysmall",
    "Zsmall",
    "colonmonetary",
    "onefitted",
    "rupiah",
    "Tildesmall",
    "exclamdownsmall",
    "centoldstyle",
    "Lslashsmall",
    "Scaronsmall",
    "Zcaronsmall",
    "Dieresissmall",
    "Brevesmall",
    "Caronsmall",
    "Dotaccentsmall",
    "Macronsmall",
    "figuredash",
    "hypheninferior",
    "Ogoneksmall",
    "Ringsmall",
    "Cedillasmall",
    "questiondownsmall",
    "oneeighth",
    "threeeighths",
    "fiveeighths",
    "seveneighths",
    "onethird",
    "twothirds",
    "zerosuperior",
    "foursuperior",
    "fivesuperior",
    "sixsuperior",
    "sevensuperior",
    "eightsuperior",
    "ninesuperior",
    "zeroinferior",
    "oneinferior",
    "twoinferior",
    "threeinferior",
    "fourinferior",
    "fiveinferior",
    "sixinferior",
    "seveninferior",
    "eightinferior",
    "nineinferior",
    "centinferior",
    "dollarinferior",
    "periodinferior",
    "commainferior",
    "Agravesmall",
    "Aacutesmall",
    "Acircumflexsmall",
    "Atildesmall",
    "Adieresissmall",
    "Aringsmall",
    "AEsmall",
    "Ccedillasmall",
    "Egravesmall",
    "Eacutesmall",
    "Ecircumflexsmall",
    "Edieresissmall",
    "Igravesmall",
    "Iacutesmall",
    "Icircumflexsmall",
    "Idieresissmall",
    "Ethsmall",
    "Ntildesmall",
    "Ogravesmall",
    "Oacutesmall",
    "Ocircumflexsmall",
    "Otildesmall",
    "Odieresissmall",
    "OEsmall",
    "Oslashsmall",
    "Ugravesmall",
    "Uacutesmall",
    "Ucircumflexsmall",
    "Udieresissmall",
    "Yacutesmall",
    "Thornsmall",
    "Ydieresissmall",
    "001.000",
    "001.001",
    "001.002",
    "001.003",
    "Black",
    "Bold",
    "Book",
    "Light",
    "Medium",
    "Regular",
    "Roman",
    "Semibold",
];
/// `str.offset[]` is only ever populated by `extract_index`, which now
/// validates the whole array is non-decreasing and 1-based before this
/// function ever sees it -- but `start < 1 || end < start` (matching
/// `locate_subr`'s own defense-in-depth comment) and the explicit
/// `checked_add`/bounds check below are kept anyway, at negligible cost,
/// so this function stays safe on its own if a `CffIndex` is ever built
/// any other way in the future. Plain slice indexing (`str.data[start..
/// end]`) replaces the original's raw `.offset()`/`from_raw_parts` walk --
/// no unsafe pointer arithmetic left to get wrong here at all.
pub fn get_cff_sid(idx: u16, str: &CffIndex) -> Option<Vec<u8>> {
    if idx as i32 <= 390_i32 {
        return Some(STRING_STANDARD[idx as usize].as_bytes().to_vec());
    }
    if str.count == 0 as Arity {
        return None;
    }
    let sid_index = (idx as i32 - 391_i32) as Arity;
    if sid_index >= str.count {
        return None;
    }
    let start = str.offset[sid_index as usize];
    let end = str.offset[(sid_index + 1) as usize];
    if start < 1 || end < start {
        return None;
    }
    let data_start = (start - 1) as usize;
    let len = (end - start) as usize;
    let data_end = data_start.checked_add(len)?;
    if data_end > str.data.len() {
        return None;
    }
    Some(str.data[data_start..data_end].to_vec())
}

#[cfg(test)]
mod get_cff_sid_tests {
    use super::*;
    use crate::libcff::cff_index::CffIndexCountType;

    fn string_index(offset: Vec<u32>, data: Vec<u8>) -> CffIndex {
        CffIndex {
            count_type: CffIndexCountType::U16,
            count: (offset.len().saturating_sub(1)) as Arity,
            off_size: 1,
            offset,
            data,
        }
    }

    #[test]
    fn standard_sid_never_touches_the_custom_index() {
        // idx <= 390 is always one of the predefined strings, regardless
        // of what (or whether) a custom String INDEX exists.
        let str = string_index(Vec::new(), Vec::new());
        assert_eq!(get_cff_sid(0, &str).unwrap(), b".notdef");
    }

    #[test]
    fn custom_sid_reads_the_right_slice() {
        let str = string_index(vec![1, 3, 6], b"ABCDE".to_vec());
        assert_eq!(get_cff_sid(391, &str).unwrap(), b"AB");
        assert_eq!(get_cff_sid(392, &str).unwrap(), b"CDE");
    }

    #[test]
    fn sid_past_the_index_count_is_rejected() {
        let str = string_index(vec![1, 3], b"AB".to_vec());
        assert!(get_cff_sid(392, &str).is_none());
    }

    #[test]
    fn non_decreasing_offset_pair_is_rejected_not_wrapped() {
        // `extract_index` now refuses to build a `CffIndex` with a
        // decreasing offset pair at all (see its own test), so this
        // constructs one by hand to confirm `get_cff_sid` doesn't rely
        // solely on that -- the exact bug `cargo fuzz run otf_parse`
        // found as a heap-buffer-overflow: `end.wrapping_sub(start)` with
        // `end < start` wraps to a length near `u32::MAX`.
        let str = string_index(vec![5, 1], b"AB".to_vec());
        assert!(get_cff_sid(391, &str).is_none());
    }

    #[test]
    fn zero_offset_is_rejected() {
        let str = string_index(vec![0, 2], b"AB".to_vec());
        assert!(get_cff_sid(391, &str).is_none());
    }

    #[test]
    fn range_past_the_actual_data_length_is_rejected_instead_of_reading_oob() {
        // The offsets are internally consistent (non-decreasing, both
        // >= 1) but claim more data than `str.data` actually holds.
        let str = string_index(vec![1, 100], b"AB".to_vec());
        assert!(get_cff_sid(391, &str).is_none());
    }
}
