#![allow(unsafe_op_in_unsafe_fn)]
// Stage 6 removes this; see rust/README.md
// `get_cff_sid` (renamed from `sdsget_cff_sid` once `vendor/sds.rs` was
// removed -- it never built an `sds`, just returned `Option<Vec<u8>>`, so
// the name was a pure holdover) has its only callers as direct Rust call
// sites (never a real FFI boundary) -- goes away with the vtable/
// extern "C" cleanup, same as every other instance of this allow.
#![allow(improper_ctypes_definitions)]
use crate::libcff::cff_index::CffIndex;
use crate::support::primitives::Arity;

static STRING_STANDARD: [&::core::ffi::CStr; 391] = [
    c".notdef",
    c"space",
    c"exclam",
    c"quotedbl",
    c"numbersign",
    c"dollar",
    c"percent",
    c"ampersand",
    c"quoteright",
    c"parenleft",
    c"parenright",
    c"asterisk",
    c"plus",
    c"comma",
    c"hyphen",
    c"period",
    c"slash",
    c"zero",
    c"one",
    c"two",
    c"three",
    c"four",
    c"five",
    c"six",
    c"seven",
    c"eight",
    c"nine",
    c"colon",
    c"semicolon",
    c"less",
    c"equal",
    c"greater",
    c"question",
    c"at",
    c"A",
    c"B",
    c"C",
    c"D",
    c"E",
    c"F",
    c"G",
    c"H",
    c"I",
    c"J",
    c"K",
    c"L",
    c"M",
    c"N",
    c"O",
    c"P",
    c"Q",
    c"R",
    c"S",
    c"T",
    c"U",
    c"V",
    c"W",
    c"X",
    c"Y",
    c"Z",
    c"bracketleft",
    c"backslash",
    c"bracketright",
    c"asciicircum",
    c"underscore",
    c"quoteleft",
    c"a",
    c"b",
    c"c",
    c"d",
    c"e",
    c"f",
    c"g",
    c"h",
    c"i",
    c"j",
    c"k",
    c"l",
    c"m",
    c"n",
    c"o",
    c"p",
    c"q",
    c"r",
    c"s",
    c"t",
    c"u",
    c"v",
    c"w",
    c"x",
    c"y",
    c"z",
    c"braceleft",
    c"bar",
    c"braceright",
    c"asciitilde",
    c"exclamdown",
    c"cent",
    c"sterling",
    c"fraction",
    c"yen",
    c"florin",
    c"section",
    c"currency",
    c"quotesingle",
    c"quotedblleft",
    c"guillemotleft",
    c"guilsinglleft",
    c"guilsinglright",
    c"fi",
    c"fl",
    c"endash",
    c"dagger",
    c"daggerdbl",
    c"periodcentered",
    c"paragraph",
    c"bullet",
    c"quotesinglbase",
    c"quotedblbase",
    c"quotedblright",
    c"guillemotright",
    c"ellipsis",
    c"perthousand",
    c"questiondown",
    c"grave",
    c"acute",
    c"circumflex",
    c"tilde",
    c"macron",
    c"breve",
    c"dotaccent",
    c"dieresis",
    c"ring",
    c"cedilla",
    c"hungarumlaut",
    c"ogonek",
    c"caron",
    c"emdash",
    c"AE",
    c"ordfeminine",
    c"Lslash",
    c"Oslash",
    c"OE",
    c"ordmasculine",
    c"ae",
    c"dotlessi",
    c"lslash",
    c"oslash",
    c"oe",
    c"germandbls",
    c"onesuperior",
    c"logicalnot",
    c"mu",
    c"trademark",
    c"Eth",
    c"onehalf",
    c"plusminus",
    c"Thorn",
    c"onequarter",
    c"divide",
    c"brokenbar",
    c"degree",
    c"thorn",
    c"threequarters",
    c"twosuperior",
    c"registered",
    c"minus",
    c"eth",
    c"multiply",
    c"threesuperior",
    c"copyright",
    c"Aacute",
    c"Acircumflex",
    c"Adieresis",
    c"Agrave",
    c"Aring",
    c"Atilde",
    c"Ccedilla",
    c"Eacute",
    c"Ecircumflex",
    c"Edieresis",
    c"Egrave",
    c"Iacute",
    c"Icircumflex",
    c"Idieresis",
    c"Igrave",
    c"Ntilde",
    c"Oacute",
    c"Ocircumflex",
    c"Odieresis",
    c"Ograve",
    c"Otilde",
    c"Scaron",
    c"Uacute",
    c"Ucircumflex",
    c"Udieresis",
    c"Ugrave",
    c"Yacute",
    c"Ydieresis",
    c"Zcaron",
    c"aacute",
    c"acircumflex",
    c"adieresis",
    c"agrave",
    c"aring",
    c"atilde",
    c"ccedilla",
    c"eacute",
    c"ecircumflex",
    c"edieresis",
    c"egrave",
    c"iacute",
    c"icircumflex",
    c"idieresis",
    c"igrave",
    c"ntilde",
    c"oacute",
    c"ocircumflex",
    c"odieresis",
    c"ograve",
    c"otilde",
    c"scaron",
    c"uacute",
    c"ucircumflex",
    c"udieresis",
    c"ugrave",
    c"yacute",
    c"ydieresis",
    c"zcaron",
    c"exclamsmall",
    c"Hungarumlautsmall",
    c"dollaroldstyle",
    c"dollarsuperior",
    c"ampersandsmall",
    c"Acutesmall",
    c"parenleftsuperior",
    c"parenrightsuperior",
    c"twodotenleader",
    c"onedotenleader",
    c"zerooldstyle",
    c"oneoldstyle",
    c"twooldstyle",
    c"threeoldstyle",
    c"fouroldstyle",
    c"fiveoldstyle",
    c"sixoldstyle",
    c"sevenoldstyle",
    c"eightoldstyle",
    c"nineoldstyle",
    c"commasuperior",
    c"threequartersemdash",
    c"periodsuperior",
    c"questionsmall",
    c"asuperior",
    c"bsuperior",
    c"centsuperior",
    c"dsuperior",
    c"esuperior",
    c"isuperior",
    c"lsuperior",
    c"msuperior",
    c"nsuperior",
    c"osuperior",
    c"rsuperior",
    c"ssuperior",
    c"tsuperior",
    c"ff",
    c"ffi",
    c"ffl",
    c"parenleftinferior",
    c"parenrightinferior",
    c"Circumflexsmall",
    c"hyphensuperior",
    c"Gravesmall",
    c"Asmall",
    c"Bsmall",
    c"Csmall",
    c"Dsmall",
    c"Esmall",
    c"Fsmall",
    c"Gsmall",
    c"Hsmall",
    c"Ismall",
    c"Jsmall",
    c"Ksmall",
    c"Lsmall",
    c"Msmall",
    c"Nsmall",
    c"Osmall",
    c"Psmall",
    c"Qsmall",
    c"Rsmall",
    c"Ssmall",
    c"Tsmall",
    c"Usmall",
    c"Vsmall",
    c"Wsmall",
    c"Xsmall",
    c"Ysmall",
    c"Zsmall",
    c"colonmonetary",
    c"onefitted",
    c"rupiah",
    c"Tildesmall",
    c"exclamdownsmall",
    c"centoldstyle",
    c"Lslashsmall",
    c"Scaronsmall",
    c"Zcaronsmall",
    c"Dieresissmall",
    c"Brevesmall",
    c"Caronsmall",
    c"Dotaccentsmall",
    c"Macronsmall",
    c"figuredash",
    c"hypheninferior",
    c"Ogoneksmall",
    c"Ringsmall",
    c"Cedillasmall",
    c"questiondownsmall",
    c"oneeighth",
    c"threeeighths",
    c"fiveeighths",
    c"seveneighths",
    c"onethird",
    c"twothirds",
    c"zerosuperior",
    c"foursuperior",
    c"fivesuperior",
    c"sixsuperior",
    c"sevensuperior",
    c"eightsuperior",
    c"ninesuperior",
    c"zeroinferior",
    c"oneinferior",
    c"twoinferior",
    c"threeinferior",
    c"fourinferior",
    c"fiveinferior",
    c"sixinferior",
    c"seveninferior",
    c"eightinferior",
    c"nineinferior",
    c"centinferior",
    c"dollarinferior",
    c"periodinferior",
    c"commainferior",
    c"Agravesmall",
    c"Aacutesmall",
    c"Acircumflexsmall",
    c"Atildesmall",
    c"Adieresissmall",
    c"Aringsmall",
    c"AEsmall",
    c"Ccedillasmall",
    c"Egravesmall",
    c"Eacutesmall",
    c"Ecircumflexsmall",
    c"Edieresissmall",
    c"Igravesmall",
    c"Iacutesmall",
    c"Icircumflexsmall",
    c"Idieresissmall",
    c"Ethsmall",
    c"Ntildesmall",
    c"Ogravesmall",
    c"Oacutesmall",
    c"Ocircumflexsmall",
    c"Otildesmall",
    c"Odieresissmall",
    c"OEsmall",
    c"Oslashsmall",
    c"Ugravesmall",
    c"Uacutesmall",
    c"Ucircumflexsmall",
    c"Udieresissmall",
    c"Yacutesmall",
    c"Thornsmall",
    c"Ydieresissmall",
    c"001.000",
    c"001.001",
    c"001.002",
    c"001.003",
    c"Black",
    c"Bold",
    c"Book",
    c"Light",
    c"Medium",
    c"Regular",
    c"Roman",
    c"Semibold",
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
pub unsafe fn get_cff_sid(idx: u16, str: &CffIndex) -> Option<Vec<u8>> {
    if idx as i32 <= 390_i32 {
        return Some(STRING_STANDARD[idx as usize].to_bytes().to_vec());
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
        unsafe {
            assert_eq!(get_cff_sid(0, &str).unwrap(), b".notdef");
        }
    }

    #[test]
    fn custom_sid_reads_the_right_slice() {
        let str = string_index(vec![1, 3, 6], b"ABCDE".to_vec());
        unsafe {
            assert_eq!(get_cff_sid(391, &str).unwrap(), b"AB");
            assert_eq!(get_cff_sid(392, &str).unwrap(), b"CDE");
        }
    }

    #[test]
    fn sid_past_the_index_count_is_rejected() {
        let str = string_index(vec![1, 3], b"AB".to_vec());
        unsafe {
            assert!(get_cff_sid(392, &str).is_none());
        }
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
        unsafe {
            assert!(get_cff_sid(391, &str).is_none());
        }
    }

    #[test]
    fn zero_offset_is_rejected() {
        let str = string_index(vec![0, 2], b"AB".to_vec());
        unsafe {
            assert!(get_cff_sid(391, &str).is_none());
        }
    }

    #[test]
    fn range_past_the_actual_data_length_is_rejected_instead_of_reading_oob() {
        // The offsets are internally consistent (non-decreasing, both
        // >= 1) but claim more data than `str.data` actually holds.
        let str = string_index(vec![1, 100], b"AB".to_vec());
        unsafe {
            assert!(get_cff_sid(391, &str).is_none());
        }
    }
}
