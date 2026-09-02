#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::built_json::BuiltValue;
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_dbl_val, json_int_val, json_obj_get,
    json_obj_get_type, json_obj_getnum_fallback, json_str_len, json_str_ptr, json_type_of,
    otfcc_parse_flags,
};
use crate::vendor::json::JsonType;
use libc::memcpy;
#[derive(Copy, Clone)]
pub struct Os2Table {
    pub version: u16,
    pub x_avg_char_width: i16,
    pub us_weight_class: u16,
    pub us_width_class: u16,
    pub fs_type: u16,
    pub y_subscript_x_size: i16,
    pub y_subscript_y_size: i16,
    pub y_subscript_x_offset: i16,
    pub y_subscript_y_offset: i16,
    pub y_supscript_x_size: i16,
    pub y_supscript_y_size: i16,
    pub y_supscript_x_offset: i16,
    pub y_supscript_y_offset: i16,
    pub y_strikeout_size: i16,
    pub y_strikeout_position: i16,
    pub s_family_class: i16,
    pub panose: [u8; 10],
    pub ul_unicode_range1: u32,
    pub ul_unicode_range2: u32,
    pub ul_unicode_range3: u32,
    pub ul_unicode_range4: u32,
    pub ach_vend_id: [u8; 4],
    pub fs_selection: u16,
    pub us_first_char_index: u16,
    pub us_last_char_index: u16,
    pub s_typo_ascender: i16,
    pub s_typo_descender: i16,
    pub s_typo_line_gap: i16,
    pub us_win_ascent: u16,
    pub us_win_descent: u16,
    pub ul_code_page_range1: u32,
    pub ul_code_page_range2: u32,
    pub sx_height: i16,
    pub s_cap_height: i16,
    pub us_default_char: u16,
    pub us_break_char: u16,
    pub us_max_context: u16,
    pub us_lower_optical_point_size: u16,
    pub us_upper_optical_point_size: u16,
}
// Stage 6-4 "Box化": every field is a scalar/fixed-size array, so no
// `Drop` impl is needed -- `Box::new` construction is sufficient. The
// entire vtable is deleted: grepping (for the bare `TABLE_I_OS_2`
// identifier, not an anchored `\.` pattern -- see the `CmapTable` PR's
// note on why that matters) confirmed only `.create`/`.free` were ever
// called, both from within this crate (this file's own read/parse entry
// points, and `caryll_font.rs`'s table disposal).
// `length` here means "the whole declared version tier's fields must fit,
// or the whole table is rejected" -- not merely "read as much as fits".
// The three `version >= N && length < M` gates are the original's, kept
// verbatim: a table that *claims* a higher version than its actual length
// supports is dropped entirely rather than silently truncated, exactly as
// before. Because each threshold (68 < 78 < 86 < 96 < 100) is strictly
// increasing and every version-gated read only happens once the
// corresponding threshold has already passed, reading sequentially through
// one `FontReader` lands on the same fixed byte offsets the original's
// `data.offset(N)` calls used explicitly -- confirmed field-by-field below.
fn parse_os_2(data: &[u8]) -> Result<Os2Table, ReadError> {
    if data.len() < 2 {
        return Err(ReadError {
            needed: 2,
            available: data.len(),
        });
    }
    // All-zero is a valid bit pattern for every field (integers and
    // fixed-size byte arrays only), matching the old `memset`-then-
    // `.version = 4` construction.
    let mut os2 = Os2Table {
        version: 4,
        x_avg_char_width: 0,
        us_weight_class: 0,
        us_width_class: 0,
        fs_type: 0,
        y_subscript_x_size: 0,
        y_subscript_y_size: 0,
        y_subscript_x_offset: 0,
        y_subscript_y_offset: 0,
        y_supscript_x_size: 0,
        y_supscript_y_size: 0,
        y_supscript_x_offset: 0,
        y_supscript_y_offset: 0,
        y_strikeout_size: 0,
        y_strikeout_position: 0,
        s_family_class: 0,
        panose: [0; 10],
        ul_unicode_range1: 0,
        ul_unicode_range2: 0,
        ul_unicode_range3: 0,
        ul_unicode_range4: 0,
        ach_vend_id: [0; 4],
        fs_selection: 0,
        us_first_char_index: 0,
        us_last_char_index: 0,
        s_typo_ascender: 0,
        s_typo_descender: 0,
        s_typo_line_gap: 0,
        us_win_ascent: 0,
        us_win_descent: 0,
        ul_code_page_range1: 0,
        ul_code_page_range2: 0,
        sx_height: 0,
        s_cap_height: 0,
        us_default_char: 0,
        us_break_char: 0,
        us_max_context: 0,
        us_lower_optical_point_size: 0,
        us_upper_optical_point_size: 0,
    };
    let mut r = FontReader::new(data);
    os2.version = r.u16()?;
    if data.len() < 68 {
        return Err(ReadError {
            needed: 68,
            available: data.len(),
        });
    }
    os2.x_avg_char_width = r.i16()?;
    os2.us_weight_class = r.u16()?;
    os2.us_width_class = r.u16()?;
    os2.fs_type = r.u16()?;
    os2.y_subscript_x_size = r.i16()?;
    os2.y_subscript_y_size = r.i16()?;
    os2.y_subscript_x_offset = r.i16()?;
    os2.y_subscript_y_offset = r.i16()?;
    os2.y_supscript_x_size = r.i16()?;
    os2.y_supscript_y_size = r.i16()?;
    os2.y_supscript_x_offset = r.i16()?;
    os2.y_supscript_y_offset = r.i16()?;
    os2.y_strikeout_size = r.i16()?;
    os2.y_strikeout_position = r.i16()?;
    os2.s_family_class = r.i16()?;
    os2.panose.copy_from_slice(r.bytes(10)?);
    os2.ul_unicode_range1 = r.u32()?;
    os2.ul_unicode_range2 = r.u32()?;
    os2.ul_unicode_range3 = r.u32()?;
    os2.ul_unicode_range4 = r.u32()?;
    os2.ach_vend_id.copy_from_slice(r.bytes(4)?);
    os2.fs_selection = r.u16()?;
    os2.us_first_char_index = r.u16()?;
    os2.us_last_char_index = r.u16()?;
    if data.len() >= 78 {
        os2.s_typo_ascender = r.i16()?;
        os2.s_typo_descender = r.i16()?;
        os2.s_typo_line_gap = r.i16()?;
        os2.us_win_ascent = r.u16()?;
        os2.us_win_descent = r.u16()?;
    }
    if os2.version >= 1 && data.len() < 86 {
        return Err(ReadError {
            needed: 86,
            available: data.len(),
        });
    }
    if os2.version >= 1 {
        os2.ul_code_page_range1 = r.u32()?;
        os2.ul_code_page_range2 = r.u32()?;
    }
    if os2.version >= 2 && data.len() < 96 {
        return Err(ReadError {
            needed: 96,
            available: data.len(),
        });
    }
    if os2.version >= 2 {
        os2.sx_height = r.i16()?;
        os2.s_cap_height = r.i16()?;
        os2.us_default_char = r.u16()?;
        os2.us_break_char = r.u16()?;
        os2.us_max_context = r.u16()?;
    }
    if os2.version >= 5 && data.len() < 100 {
        return Err(ReadError {
            needed: 100,
            available: data.len(),
        });
    }
    if os2.version >= 5 {
        // Preserving the original's bug verbatim: both reads assign to
        // `us_lower_optical_point_size`, so `us_upper_optical_point_size`
        // is never actually populated from the file (stays 0). Fixing this
        // is a correctness change outside this migration's parse-bounds-
        // safety scope; see rust/README.md.
        os2.us_lower_optical_point_size = r.u16()?;
        os2.us_lower_optical_point_size = r.u16()?;
    }
    Ok(os2)
}
pub fn otfcc_read_os_2(packet: &Packet, options: &Options) -> Option<Box<Os2Table>> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_OS_2)?;
    match parse_os_2(&table.data) {
        Ok(os2) => Some(Box::new(os2)),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'OS/2' corrupted.\n"),
            );
            None
        }
    }
}
pub static FS_TYPE_LABELS: [&::core::ffi::CStr; 10] = [
    c"_reserved1",
    c"restrictedLicense",
    c"previewPrintLicense",
    c"editableEmbedding",
    c"_reserved2",
    c"_reserved3",
    c"_reserved4",
    c"_reserved5",
    c"noSubsetting",
    c"bitmapEmbeddingOnly",
];
pub static FS_SELECTION_LABELS: [&::core::ffi::CStr; 10] = [
    c"italic",
    c"underscore",
    c"negative",
    c"outlined",
    c"strikeout",
    c"bold",
    c"regular",
    c"useTypoMetrics",
    c"wws",
    c"oblique",
];
pub static CODE_PAGE_LABELS1: [&::core::ffi::CStr; 32] = [
    c"latin1",
    c"latin2",
    c"cyrillic",
    c"greek",
    c"turkish",
    c"hebrew",
    c"arabic",
    c"windowsBaltic",
    c"vietnamese",
    c"ansi1",
    c"ansi2",
    c"ansi3",
    c"ansi4",
    c"ansi5",
    c"ansi6",
    c"ansi7",
    c"thai",
    c"jis",
    c"gbk",
    c"korean",
    c"big5",
    c"koreanJohab",
    c"oem1",
    c"oem2",
    c"oem3",
    c"oem4",
    c"oem5",
    c"oem6",
    c"oem7",
    c"macRoman",
    c"oem",
    c"symbol",
];
pub static CODE_PAGE_LABELS2: [&::core::ffi::CStr; 32] = [
    c"oem8", c"oem9", c"oem10", c"oem11", c"oem12", c"oem13", c"oem14", c"oem15", c"oem16",
    c"oem17", c"oem18", c"oem19", c"oem20", c"oem21", c"oem22", c"oem23", c"cp869", c"cp866",
    c"cp865", c"cp864", c"cp863", c"cp862", c"cp861", c"cp860", c"cp857", c"cp855", c"cp852",
    c"cp775", c"cp737", c"cp708", c"cp850", c"ascii",
];
pub static UNICODE_RANGE_LABELS1: [&::core::ffi::CStr; 32] = [
    c"Basic_Latin",
    c"Latin_1_Supplement",
    c"Latin_Extended_A",
    c"Latin_Extended_B",
    c"Phonetics",
    c"Spacing_Modifiers",
    c"Combining_Diacritical_Marks",
    c"Greek_and_Coptic",
    c"Coptic",
    c"Cyrillic",
    c"Armenian",
    c"Hebrew",
    c"Vai",
    c"Arabic",
    c"NKo",
    c"Devanagari",
    c"Bengali",
    c"Gurmukhi",
    c"Gujarati",
    c"Oriya",
    c"Tamil",
    c"Telugu",
    c"Kannada",
    c"Malayalam",
    c"Thai",
    c"Lao",
    c"Georgian",
    c"Balinese",
    c"Hangul_Jamo",
    c"Latin_Extended_Additional",
    c"Greek_Extended",
    c"Punctuations",
];
pub static UNICODE_RANGE_LABELS2: [&::core::ffi::CStr; 32] = [
    c"Superscripts_And_Subscripts",
    c"Currency_Symbols",
    c"Combining_Diacritical_Marks_For_Symbols",
    c"Letterlike_Symbols",
    c"Number_Forms",
    c"Arrows",
    c"Mathematical_Operators",
    c"Miscellaneous_Technical",
    c"Control_Pictures",
    c"Optical_Character_Recognition",
    c"Enclosed_Alphanumerics",
    c"Box_Drawing",
    c"Block_Elements",
    c"Geometric_Shapes",
    c"Miscellaneous_Symbols",
    c"Dingbats",
    c"CJK_Symbols_And_Punctuation",
    c"Hiragana",
    c"Katakana",
    c"Bopomofo",
    c"Hangul_Compatibility_Jamo",
    c"Phags_pa",
    c"Enclosed_CJK_Letters_And_Months",
    c"CJK_Compatibility",
    c"Hangul_Syllables",
    c"Non_Plane_0",
    c"Phoenician",
    c"CJK_Unified_Ideographs",
    c"Private_Use_Area_p0",
    c"CJK_Strokes",
    c"Alphabetic_Presentation_Forms",
    c"Arabic_Presentation_Forms_A",
];
pub static UNICODE_RANGE_LABELS3: [&::core::ffi::CStr; 32] = [
    c"Combining_Half_Marks",
    c"Vertical_Forms_and_CJK_Compatibility_Forms",
    c"Small_Form_Variants",
    c"Arabic_Presentation_Forms_B",
    c"Halfwidth_And_Fullwidth_Forms",
    c"Specials",
    c"Tibetan",
    c"Syriac",
    c"Thaana",
    c"Sinhala",
    c"Myanmar",
    c"Ethiopic",
    c"Cherokee",
    c"Unified_Canadian_Aboriginal_Syllabics",
    c"Ogham",
    c"Runic",
    c"Khmer",
    c"Mongolian",
    c"Braille_Patterns",
    c"Yi_Syllables",
    c"Tagalog",
    c"Old_Italic",
    c"Gothic",
    c"Deseret",
    c"Musical_Symbols",
    c"Mathematical_Alphanumeric_Symbols",
    c"Private_Use_p15_and_p16",
    c"Variation_Selectors",
    c"Tags",
    c"Limbu",
    c"Tai_Le",
    c"New_Tai_Lue",
];
pub static UNICODE_RANGE_LABELS4: [&::core::ffi::CStr; 27] = [
    c"Buginese",
    c"Glagolitic",
    c"Tifinagh",
    c"Yijing_Hexagram_Symbols",
    c"Syloti_Nagri",
    c"Linear_B_Syllabary_Ideograms_and_Aegean_Numbers",
    c"Ancient_Greek_Numbers",
    c"Ugaritic",
    c"Old_Persian",
    c"Shavian",
    c"Osmanya",
    c"Cypriot_Syllabary",
    c"Kharoshthi",
    c"Tai_Xuan_Jing_Symbols",
    c"Cuneiform",
    c"Counting_Rod_Numerals",
    c"Sundanese",
    c"Lepcha",
    c"Ol_Chiki",
    c"Saurashtra",
    c"Kayah_Li",
    c"Rejang",
    c"Cham",
    c"Ancient_Symbols",
    c"Phaistos_Disc",
    c"Carian_and_Lycian",
    c"Domino_and_Mahjong_Tiles",
];
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_os_2(
    table: Option<&Os2Table>,
    root: &mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t as *const Os2Table,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"OS/2"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut os_2 = BuiltValue::new_object(30);
        os_2.push_field(b"version", BuiltValue::Int((*table).version as i64));
        os_2.push_field(
            b"xAvgCharWidth",
            BuiltValue::Int((*table).x_avg_char_width as i64),
        );
        os_2.push_field(
            b"usWeightClass",
            BuiltValue::Int((*table).us_weight_class as i64),
        );
        os_2.push_field(
            b"usWidthClass",
            BuiltValue::Int((*table).us_width_class as i64),
        );
        os_2.push_field(
            b"fsType",
            BuiltValue::dump_flags((*table).fs_type as i32, &FS_TYPE_LABELS),
        );
        os_2.push_field(
            b"ySubscriptXSize",
            BuiltValue::Int((*table).y_subscript_x_size as i64),
        );
        os_2.push_field(
            b"ySubscriptYSize",
            BuiltValue::Int((*table).y_subscript_y_size as i64),
        );
        os_2.push_field(
            b"ySubscriptXOffset",
            BuiltValue::Int((*table).y_subscript_x_offset as i64),
        );
        os_2.push_field(
            b"ySubscriptYOffset",
            BuiltValue::Int((*table).y_subscript_y_offset as i64),
        );
        os_2.push_field(
            b"ySupscriptXSize",
            BuiltValue::Int((*table).y_supscript_x_size as i64),
        );
        os_2.push_field(
            b"ySupscriptYSize",
            BuiltValue::Int((*table).y_supscript_y_size as i64),
        );
        os_2.push_field(
            b"ySupscriptXOffset",
            BuiltValue::Int((*table).y_supscript_x_offset as i64),
        );
        os_2.push_field(
            b"ySupscriptYOffset",
            BuiltValue::Int((*table).y_supscript_y_offset as i64),
        );
        os_2.push_field(
            b"yStrikeoutSize",
            BuiltValue::Int((*table).y_strikeout_size as i64),
        );
        os_2.push_field(
            b"yStrikeoutPosition",
            BuiltValue::Int((*table).y_strikeout_position as i64),
        );
        os_2.push_field(
            b"sFamilyClass",
            BuiltValue::Int((*table).s_family_class as i64),
        );
        let mut panose = BuiltValue::new_array(10);
        let mut j: u8 = 0_u8;
        while (j as i32) < 10_i32 {
            panose.push_item(BuiltValue::Int((*table).panose[j as usize] as i64));
            j = j.wrapping_add(1);
        }
        os_2.push_field(b"panose", panose);
        os_2.push_field(
            b"ulUnicodeRange1",
            BuiltValue::dump_flags((*table).ul_unicode_range1 as i32, &UNICODE_RANGE_LABELS1),
        );
        os_2.push_field(
            b"ulUnicodeRange2",
            BuiltValue::dump_flags((*table).ul_unicode_range2 as i32, &UNICODE_RANGE_LABELS2),
        );
        os_2.push_field(
            b"ulUnicodeRange3",
            BuiltValue::dump_flags((*table).ul_unicode_range3 as i32, &UNICODE_RANGE_LABELS3),
        );
        os_2.push_field(
            b"ulUnicodeRange4",
            BuiltValue::dump_flags((*table).ul_unicode_range4 as i32, &UNICODE_RANGE_LABELS4),
        );
        os_2.push_field(
            b"achVendID",
            BuiltValue::str_truncated_at_nul(&(*table).ach_vend_id),
        );
        os_2.push_field(
            b"fsSelection",
            BuiltValue::dump_flags((*table).fs_selection as i32, &FS_SELECTION_LABELS),
        );
        os_2.push_field(
            b"usFirstCharIndex",
            BuiltValue::Int((*table).us_first_char_index as i64),
        );
        os_2.push_field(
            b"usLastCharIndex",
            BuiltValue::Int((*table).us_last_char_index as i64),
        );
        os_2.push_field(
            b"sTypoAscender",
            BuiltValue::Int((*table).s_typo_ascender as i64),
        );
        os_2.push_field(
            b"sTypoDescender",
            BuiltValue::Int((*table).s_typo_descender as i64),
        );
        os_2.push_field(
            b"sTypoLineGap",
            BuiltValue::Int((*table).s_typo_line_gap as i64),
        );
        os_2.push_field(
            b"usWinAscent",
            BuiltValue::Int((*table).us_win_ascent as i64),
        );
        os_2.push_field(
            b"usWinDescent",
            BuiltValue::Int((*table).us_win_descent as i64),
        );
        os_2.push_field(
            b"ulCodePageRange1",
            BuiltValue::dump_flags((*table).ul_code_page_range1 as i32, &CODE_PAGE_LABELS1),
        );
        os_2.push_field(
            b"ulCodePageRange2",
            BuiltValue::dump_flags((*table).ul_code_page_range2 as i32, &CODE_PAGE_LABELS2),
        );
        os_2.push_field(b"sxHeight", BuiltValue::Int((*table).sx_height as i64));
        os_2.push_field(
            b"sCapHeight",
            BuiltValue::Int((*table).s_cap_height as i64),
        );
        os_2.push_field(
            b"usDefaultChar",
            BuiltValue::Int((*table).us_default_char as i64),
        );
        os_2.push_field(
            b"usBreakChar",
            BuiltValue::Int((*table).us_break_char as i64),
        );
        os_2.push_field(
            b"usMaxContext",
            BuiltValue::Int((*table).us_max_context as i64),
        );
        os_2.push_field(
            b"usLowerOpticalPointSize",
            BuiltValue::Int((*table).us_lower_optical_point_size as i64),
        );
        os_2.push_field(
            b"usUpperOpticalPointSize",
            BuiltValue::Int((*table).us_upper_optical_point_size as i64),
        );
        root.push_field(b"OS_2", os_2);
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
pub unsafe fn otfcc_parse_os_2(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<Os2Table>> {
    // `Box::new` cannot return null (it aborts on allocation failure), so
    // the old `TABLE_I_OS_2.create()`-returned-null defensive check --
    // guarding a `malloc` that could in principle fail -- has no
    // equivalent here; there is nothing left to check.
    let mut os2_val: Os2Table = ::core::mem::zeroed();
    os2_val.version = 4;
    let mut os_2_box: Box<Os2Table> = Box::new(os2_val);
    let os_2: *mut Os2Table = os_2_box.as_mut() as *mut Os2Table;
    let table: *const ParsedValue;
    table = json_obj_get_type(
        root,
        b"OS_2\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"OS/2"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*os_2).version = json_obj_getnum_fallback(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).x_avg_char_width = json_obj_getnum_fallback(
                table,
                b"xAvgCharWidth\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).us_weight_class = json_obj_getnum_fallback(
                table,
                b"usWeightClass\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).us_width_class = json_obj_getnum_fallback(
                table,
                b"usWidthClass\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).fs_type = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"fsType\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &FS_TYPE_LABELS,
            ) as u16;
            (*os_2).y_subscript_x_size = json_obj_getnum_fallback(
                table,
                b"ySubscriptXSize\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).y_subscript_y_size = json_obj_getnum_fallback(
                table,
                b"ySubscriptYSize\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).y_subscript_x_offset = json_obj_getnum_fallback(
                table,
                b"ySubscriptXOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).y_subscript_y_offset = json_obj_getnum_fallback(
                table,
                b"ySubscriptYOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).y_supscript_x_size = json_obj_getnum_fallback(
                table,
                b"ySupscriptXSize\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).y_supscript_y_size = json_obj_getnum_fallback(
                table,
                b"ySupscriptYSize\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).y_supscript_x_offset = json_obj_getnum_fallback(
                table,
                b"ySupscriptXOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).y_supscript_y_offset = json_obj_getnum_fallback(
                table,
                b"ySupscriptYOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).y_strikeout_size = json_obj_getnum_fallback(
                table,
                b"yStrikeoutSize\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).y_strikeout_position = json_obj_getnum_fallback(
                table,
                b"yStrikeoutPosition\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).s_family_class = json_obj_getnum_fallback(
                table,
                b"sFamilyClass\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).fs_selection = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"fsSelection\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &FS_SELECTION_LABELS,
            ) as u16;
            (*os_2).us_first_char_index = json_obj_getnum_fallback(
                table,
                b"usFirstCharIndex\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).us_last_char_index = json_obj_getnum_fallback(
                table,
                b"usLastCharIndex\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).s_typo_ascender = json_obj_getnum_fallback(
                table,
                b"sTypoAscender\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).s_typo_descender = json_obj_getnum_fallback(
                table,
                b"sTypoDescender\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).s_typo_line_gap = json_obj_getnum_fallback(
                table,
                b"sTypoLineGap\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).us_win_ascent = json_obj_getnum_fallback(
                table,
                b"usWinAscent\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).us_win_descent = json_obj_getnum_fallback(
                table,
                b"usWinDescent\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).ul_code_page_range1 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulCodePageRange1\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &CODE_PAGE_LABELS1,
            );
            (*os_2).ul_code_page_range2 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulCodePageRange2\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &CODE_PAGE_LABELS2,
            );
            (*os_2).ul_unicode_range1 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange1\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &UNICODE_RANGE_LABELS1,
            );
            (*os_2).ul_unicode_range2 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange2\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &UNICODE_RANGE_LABELS2,
            );
            (*os_2).ul_unicode_range3 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange3\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &UNICODE_RANGE_LABELS3,
            );
            (*os_2).ul_unicode_range4 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange4\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &UNICODE_RANGE_LABELS4,
            );
            (*os_2).sx_height = json_obj_getnum_fallback(
                table,
                b"sxHeight\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).s_cap_height = json_obj_getnum_fallback(
                table,
                b"sCapHeight\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as i16;
            (*os_2).us_default_char = json_obj_getnum_fallback(
                table,
                b"usDefaultChar\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).us_break_char = json_obj_getnum_fallback(
                table,
                b"usBreakChar\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).us_max_context = json_obj_getnum_fallback(
                table,
                b"usMaxContext\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).us_lower_optical_point_size = json_obj_getnum_fallback(
                table,
                b"usLowerOpticalPointSize\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            (*os_2).us_upper_optical_point_size = json_obj_getnum_fallback(
                table,
                b"usUpperOpticalPointSize\0" as *const u8 as *const ::core::ffi::c_char,
                0_i32 as ::core::ffi::c_double,
            ) as u16;
            let panose: *const ParsedValue;
            panose = json_obj_get_type(
                table,
                b"panose\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            );
            if !panose.is_null() {
                let mut j: u32 = 0_u32;
                while j < json_arr_len(panose) && j < 10_u32 {
                    let term: *const ParsedValue = json_arr_at(panose, j);
                    if json_type_of(term) == JsonType::Integer {
                        (*os_2).panose[j as usize] = json_int_val(term) as u8;
                    } else if json_type_of(term) == JsonType::Double {
                        (*os_2).panose[j as usize] = json_dbl_val(term) as u8;
                    }
                    j = j.wrapping_add(1);
                }
            }
            let vendorid: *const ParsedValue;
            vendorid = json_obj_get_type(
                table,
                b"achVendID\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !vendorid.is_null() {
                (*os_2).ach_vend_id[0_i32 as usize] = ' ' as i32 as u8;
                (*os_2).ach_vend_id[1_i32 as usize] = ' ' as i32 as u8;
                (*os_2).ach_vend_id[2_i32 as usize] = ' ' as i32 as u8;
                (*os_2).ach_vend_id[3_i32 as usize] = ' ' as i32 as u8;
                if json_str_len(vendorid) >= 4 as ::core::ffi::c_uint {
                    memcpy(
                        &raw mut (*os_2).ach_vend_id as *mut u8 as *mut ::core::ffi::c_void,
                        json_str_ptr(vendorid) as *const ::core::ffi::c_void,
                        4_usize,
                    );
                } else {
                    memcpy(
                        &raw mut (*os_2).ach_vend_id as *mut u8 as *mut ::core::ffi::c_void,
                        json_str_ptr(vendorid) as *const ::core::ffi::c_void,
                        json_str_len(vendorid) as usize,
                    );
                }
            }
            ___loggedstep_v = false;
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    if ((*os_2).version as i32) < 1_i32 {
        (*os_2).version = 1_u16;
    }
    return Some(os_2_box);
}
#[allow(improper_ctypes_definitions)]
pub fn otfcc_build_os_2(os_2: Option<&Os2Table>) -> Option<Buffer> {
    let os_2 = os_2?;
    let mut buf = Buffer::new();
    buf.write_u16be(os_2.version);
    buf.write_u16be(os_2.x_avg_char_width as u16);
    buf.write_u16be(os_2.us_weight_class);
    buf.write_u16be(os_2.us_width_class);
    buf.write_u16be(os_2.fs_type);
    buf.write_u16be(os_2.y_subscript_x_size as u16);
    buf.write_u16be(os_2.y_subscript_y_size as u16);
    buf.write_u16be(os_2.y_subscript_x_offset as u16);
    buf.write_u16be(os_2.y_subscript_y_offset as u16);
    buf.write_u16be(os_2.y_supscript_x_size as u16);
    buf.write_u16be(os_2.y_supscript_y_size as u16);
    buf.write_u16be(os_2.y_supscript_x_offset as u16);
    buf.write_u16be(os_2.y_supscript_y_offset as u16);
    buf.write_u16be(os_2.y_strikeout_size as u16);
    buf.write_u16be(os_2.y_strikeout_position as u16);
    buf.write_u16be(os_2.s_family_class as u16);
    buf.write_bytes(&os_2.panose);
    buf.write_u32be(os_2.ul_unicode_range1);
    buf.write_u32be(os_2.ul_unicode_range2);
    buf.write_u32be(os_2.ul_unicode_range3);
    buf.write_u32be(os_2.ul_unicode_range4);
    buf.write_bytes(&os_2.ach_vend_id);
    buf.write_u16be(os_2.fs_selection);
    buf.write_u16be(os_2.us_first_char_index);
    buf.write_u16be(os_2.us_last_char_index);
    buf.write_u16be(os_2.s_typo_ascender as u16);
    buf.write_u16be(os_2.s_typo_descender as u16);
    buf.write_u16be(os_2.s_typo_line_gap as u16);
    buf.write_u16be(os_2.us_win_ascent);
    buf.write_u16be(os_2.us_win_descent);
    buf.write_u32be(os_2.ul_code_page_range1);
    buf.write_u32be(os_2.ul_code_page_range2);
    if (os_2.version as i32) < 2_i32 {
        return Some(buf);
    }
    buf.write_u16be(os_2.sx_height as u16);
    buf.write_u16be(os_2.s_cap_height as u16);
    buf.write_u16be(os_2.us_default_char);
    buf.write_u16be(os_2.us_break_char);
    buf.write_u16be(os_2.us_max_context);
    if (os_2.version as i32) < 5_i32 {
        return Some(buf);
    }
    buf.write_u16be(os_2.us_lower_optical_point_size);
    buf.write_u16be(os_2.us_upper_optical_point_size);
    Some(buf)
}

#[cfg(test)]
mod parse_os_2_tests {
    use super::*;

    fn version_0_base(us_weight_class: u16) -> Vec<u8> {
        let mut data = vec![0u8; 68];
        data[0..2].copy_from_slice(&0u16.to_be_bytes()); // version 0
        data[4..6].copy_from_slice(&us_weight_class.to_be_bytes());
        data
    }

    #[test]
    fn version_0_table_needs_only_68_bytes() {
        let os2 = parse_os_2(&version_0_base(700)).unwrap();
        assert_eq!(os2.version, 0);
        assert_eq!(os2.us_weight_class, 700);
    }

    #[test]
    fn table_one_byte_short_of_68_is_rejected() {
        let mut data = version_0_base(700);
        data.truncate(67);
        assert!(parse_os_2(&data).is_err());
    }

    #[test]
    fn version_1_table_shorter_than_86_is_rejected_even_though_base_fields_parsed() {
        // The base (0..68) and typo/win (68..78) fields are all in bounds
        // here -- only the version-1-specific code-page-range fields
        // (78..86) are missing. The original drops the *whole* table in
        // this case rather than returning what it already read.
        let mut data = version_0_base(0);
        data[0..2].copy_from_slice(&1u16.to_be_bytes()); // version 1
        data.resize(85, 0);
        assert!(parse_os_2(&data).is_err());
    }

    #[test]
    fn version_1_table_with_86_bytes_reads_code_page_ranges() {
        let mut data = version_0_base(0);
        data[0..2].copy_from_slice(&1u16.to_be_bytes());
        data.resize(86, 0);
        data[78..82].copy_from_slice(&0x0000_0001u32.to_be_bytes());
        let os2 = parse_os_2(&data).unwrap();
        assert_eq!(os2.ul_code_page_range1, 1);
    }

    #[test]
    fn version_5_table_leaves_upper_optical_point_size_at_zero() {
        // Preserving the original's field-name bug: both reads at this
        // version tier assign to `us_lower_optical_point_size`, so
        // `us_upper_optical_point_size` is never actually populated.
        let mut data = version_0_base(0);
        data[0..2].copy_from_slice(&5u16.to_be_bytes());
        data.resize(100, 0);
        data[96..98].copy_from_slice(&12u16.to_be_bytes());
        data[98..100].copy_from_slice(&34u16.to_be_bytes());
        let os2 = parse_os_2(&data).unwrap();
        assert_eq!(os2.us_lower_optical_point_size, 34);
        assert_eq!(os2.us_upper_optical_point_size, 0);
    }

    #[test]
    fn one_byte_table_is_rejected_before_reading_the_version_field() {
        assert!(parse_os_2(&[0x00]).is_err());
    }
}
