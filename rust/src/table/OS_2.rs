#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc, memcpy, memset};
use crate::support::binio::{read_16u, read_16s, read_32u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::json_funcs::{json_obj_get, json_obj_get_type, json_obj_getnum_fallback, otfcc_dump_flags, otfcc_parse_flags};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b, bufwrite_bytes};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new};
use crate::vendor::sds::{sdsempty, sdsfree, sdsnewlen};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Os2Table {
    pub version: u16,
    pub xAvgCharWidth: i16,
    pub usWeightClass: u16,
    pub usWidthClass: u16,
    pub fsType: u16,
    pub ySubscriptXSize: i16,
    pub ySubscriptYSize: i16,
    pub ySubscriptXOffset: i16,
    pub ySubscriptYOffset: i16,
    pub ySupscriptXSize: i16,
    pub ySupscriptYSize: i16,
    pub ySupscriptXOffset: i16,
    pub ySupscriptYOffset: i16,
    pub yStrikeoutSize: i16,
    pub yStrikeoutPosition: i16,
    pub sFamilyClass: i16,
    pub panose: [u8; 10],
    pub ulUnicodeRange1: u32,
    pub ulUnicodeRange2: u32,
    pub ulUnicodeRange3: u32,
    pub ulUnicodeRange4: u32,
    pub achVendID: [u8; 4],
    pub fsSelection: u16,
    pub usFirstCharIndex: u16,
    pub usLastCharIndex: u16,
    pub sTypoAscender: i16,
    pub sTypoDescender: i16,
    pub sTypoLineGap: i16,
    pub usWinAscent: u16,
    pub usWinDescent: u16,
    pub ulCodePageRange1: u32,
    pub ulCodePageRange2: u32,
    pub sxHeight: i16,
    pub sCapHeight: i16,
    pub usDefaultChar: u16,
    pub usBreakChar: u16,
    pub usMaxContext: u16,
    pub usLowerOpticalPointSize: u16,
    pub usUpperOpticalPointSize: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Os2TableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut Os2Table) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut Os2Table, *const Os2Table) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut Os2Table, *mut Os2Table) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut Os2Table) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut Os2Table, Os2Table) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut Os2Table, Os2Table) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut Os2Table>,
    pub free: Option<unsafe extern "C" fn(*mut Os2Table) -> ()>,
}
#[inline]
unsafe extern "C" fn init_os2(mut table: *mut Os2Table) {
    memset(
        table as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Os2Table>() as usize,
    );
    (*table).version = 4 as u16;
}
#[inline]
unsafe extern "C" fn dispose_os2(mut _table: *mut Os2Table) {}
#[inline]
unsafe extern "C" fn table_os_2_dispose(mut x: *mut Os2Table) {
    dispose_os2(x);
}
#[inline]
unsafe extern "C" fn table_os_2_create() -> *mut Os2Table {
    let mut x: *mut Os2Table =
        malloc(::core::mem::size_of::<Os2Table>() as usize) as *mut Os2Table;
    table_os_2_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_os_2_copy_replace(mut dst: *mut Os2Table, src: Os2Table) {
    table_os_2_dispose(dst);
    table_os_2_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_os_2_init(mut x: *mut Os2Table) {
    init_os2(x);
}
pub static TABLE_I_OS_2: Os2TableElementInterface = {
    Os2TableElementInterface {
        init: Some(table_os_2_init as unsafe extern "C" fn(*mut Os2Table) -> ()),
        copy: Some(
            table_os_2_copy as unsafe extern "C" fn(*mut Os2Table, *const Os2Table) -> (),
        ),
        move_0: Some(
            table_os_2_move as unsafe extern "C" fn(*mut Os2Table, *mut Os2Table) -> (),
        ),
        dispose: Some(table_os_2_dispose as unsafe extern "C" fn(*mut Os2Table) -> ()),
        replace: Some(
            table_os_2_replace as unsafe extern "C" fn(*mut Os2Table, Os2Table) -> (),
        ),
        copyReplace: Some(
            table_os_2_copy_replace as unsafe extern "C" fn(*mut Os2Table, Os2Table) -> (),
        ),
        create: Some(table_os_2_create),
        free: Some(table_os_2_free as unsafe extern "C" fn(*mut Os2Table) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_os_2_copy(mut dst: *mut Os2Table, mut src: *const Os2Table) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Os2Table>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_os_2_replace(mut dst: *mut Os2Table, src: Os2Table) {
    table_os_2_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Os2Table>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_os_2_move(mut dst: *mut Os2Table, mut src: *mut Os2Table) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Os2Table>() as usize,
    );
    table_os_2_init(src);
}
#[inline]
unsafe extern "C" fn table_os_2_free(mut x: *mut Os2Table) {
    if x.is_null() {
        return;
    }
    table_os_2_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub unsafe extern "C" fn otfcc_read_os_2(
    packet: Packet,
    mut options: *const Options,
) -> *mut Os2Table {
    let mut os_2: *mut Os2Table = ::core::ptr::null_mut::<Os2Table>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1330851634i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if !(length < 2 as u32) {
                        os_2 = (
                            TABLE_I_OS_2.create.expect("non-null function pointer"))();
                        (*os_2).version = read_16u(data as *const u8);
                        if !(length < 68 as u32) {
                            (*os_2).xAvgCharWidth = read_16u(
                                data.offset(2 as ::core::ffi::c_int as isize) as *const u8,
                            ) as i16;
                            (*os_2).usWeightClass = read_16u(
                                data.offset(4 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            (*os_2).usWidthClass = read_16u(
                                data.offset(6 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            (*os_2).fsType = read_16u(
                                data.offset(8 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            (*os_2).ySubscriptXSize =
                                read_16u(data.offset(10 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            (*os_2).ySubscriptYSize =
                                read_16u(data.offset(12 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            (*os_2).ySubscriptXOffset =
                                read_16u(data.offset(14 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            (*os_2).ySubscriptYOffset =
                                read_16u(data.offset(16 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            (*os_2).ySupscriptXSize =
                                read_16u(data.offset(18 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            (*os_2).ySupscriptYSize =
                                read_16u(data.offset(20 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            (*os_2).ySupscriptXOffset =
                                read_16u(data.offset(22 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            (*os_2).ySupscriptYOffset =
                                read_16u(data.offset(24 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            (*os_2).yStrikeoutSize =
                                read_16u(data.offset(26 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            (*os_2).yStrikeoutPosition =
                                read_16u(data.offset(28 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            (*os_2).sFamilyClass =
                                read_16u(data.offset(30 as ::core::ffi::c_int as isize)
                                    as *const u8) as i16;
                            memcpy(
                                &raw mut (*os_2).panose as *mut u8 as *mut ::core::ffi::c_void,
                                data.offset(32 as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                10 as usize,
                            );
                            (*os_2).ulUnicodeRange1 =
                                read_32u(data.offset(42 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*os_2).ulUnicodeRange2 =
                                read_32u(data.offset(46 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*os_2).ulUnicodeRange3 =
                                read_32u(data.offset(50 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*os_2).ulUnicodeRange4 =
                                read_32u(data.offset(54 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            memcpy(
                                &raw mut (*os_2).achVendID as *mut u8
                                    as *mut ::core::ffi::c_void,
                                data.offset(58 as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                4 as usize,
                            );
                            (*os_2).fsSelection =
                                read_16u(data.offset(62 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*os_2).usFirstCharIndex =
                                read_16u(data.offset(64 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            (*os_2).usLastCharIndex =
                                read_16u(data.offset(66 as ::core::ffi::c_int as isize)
                                    as *const u8);
                            if length >= 78 as u32 {
                                (*os_2).sTypoAscender =
                                    read_16s(data.offset(68 as ::core::ffi::c_int as isize)
                                        as *const u8);
                                (*os_2).sTypoDescender =
                                    read_16s(data.offset(70 as ::core::ffi::c_int as isize)
                                        as *const u8);
                                (*os_2).sTypoLineGap =
                                    read_16s(data.offset(72 as ::core::ffi::c_int as isize)
                                        as *const u8);
                                (*os_2).usWinAscent =
                                    read_16u(data.offset(74 as ::core::ffi::c_int as isize)
                                        as *const u8);
                                (*os_2).usWinDescent =
                                    read_16u(data.offset(76 as ::core::ffi::c_int as isize)
                                        as *const u8);
                            }
                            if !((*os_2).version as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                                && length < 86 as u32)
                            {
                                if (*os_2).version as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                                {
                                    (*os_2).ulCodePageRange1 =
                                        read_32u(data.offset(78 as ::core::ffi::c_int as isize)
                                            as *const u8);
                                    (*os_2).ulCodePageRange2 =
                                        read_32u(data.offset(82 as ::core::ffi::c_int as isize)
                                            as *const u8);
                                }
                                if !((*os_2).version as ::core::ffi::c_int
                                    >= 2 as ::core::ffi::c_int
                                    && length < 96 as u32)
                                {
                                    if (*os_2).version as ::core::ffi::c_int
                                        >= 2 as ::core::ffi::c_int
                                    {
                                        (*os_2).sxHeight = read_16s(
                                            data.offset(86 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        );
                                        (*os_2).sCapHeight = read_16s(
                                            data.offset(88 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        );
                                        (*os_2).usDefaultChar = read_16u(
                                            data.offset(90 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        );
                                        (*os_2).usBreakChar = read_16u(
                                            data.offset(92 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        );
                                        (*os_2).usMaxContext = read_16u(
                                            data.offset(94 as ::core::ffi::c_int as isize)
                                                as *const u8,
                                        );
                                    }
                                    if !((*os_2).version as ::core::ffi::c_int
                                        >= 5 as ::core::ffi::c_int
                                        && length < 100 as u32)
                                    {
                                        if (*os_2).version as ::core::ffi::c_int
                                            >= 5 as ::core::ffi::c_int
                                        {
                                            (*os_2).usLowerOpticalPointSize = read_16u(
                                                data.offset(96 as ::core::ffi::c_int as isize)
                                                    as *const u8,
                                            );
                                            (*os_2).usLowerOpticalPointSize = read_16u(
                                                data.offset(98 as ::core::ffi::c_int as isize)
                                                    as *const u8,
                                            );
                                        }
                                        return os_2;
                                    }
                                }
                            }
                        }
                    }
                    (*(*options).logger)
                        .logSDS
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(sdsempty(), b"table 'OS/2' corrupted.\n"),
                    );
                    if !os_2.is_null() {
                        free(os_2 as *mut ::core::ffi::c_void);
                        os_2 = ::core::ptr::null_mut::<Os2Table>();
                    }
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return ::core::ptr::null_mut::<Os2Table>();
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
    c"oem8",
    c"oem9",
    c"oem10",
    c"oem11",
    c"oem12",
    c"oem13",
    c"oem14",
    c"oem15",
    c"oem16",
    c"oem17",
    c"oem18",
    c"oem19",
    c"oem20",
    c"oem21",
    c"oem22",
    c"oem23",
    c"cp869",
    c"cp866",
    c"cp865",
    c"cp864",
    c"cp863",
    c"cp862",
    c"cp861",
    c"cp860",
    c"cp857",
    c"cp855",
    c"cp852",
    c"cp775",
    c"cp737",
    c"cp708",
    c"cp850",
    c"ascii",
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
pub unsafe extern "C" fn otfcc_dump_os_2(
    mut table: *const Os2Table,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"OS/2"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut os_2: *mut JsonValue = json_object_new(30 as usize);
        json_object_push(
            os_2,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).version as i64),
        );
        json_object_push(
            os_2,
            b"xAvgCharWidth\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).xAvgCharWidth as i64),
        );
        json_object_push(
            os_2,
            b"usWeightClass\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usWeightClass as i64),
        );
        json_object_push(
            os_2,
            b"usWidthClass\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usWidthClass as i64),
        );
        json_object_push(
            os_2,
            b"fsType\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).fsType as ::core::ffi::c_int,
                &FS_TYPE_LABELS,
            ),
        );
        json_object_push(
            os_2,
            b"ySubscriptXSize\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ySubscriptXSize as i64),
        );
        json_object_push(
            os_2,
            b"ySubscriptYSize\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ySubscriptYSize as i64),
        );
        json_object_push(
            os_2,
            b"ySubscriptXOffset\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ySubscriptXOffset as i64),
        );
        json_object_push(
            os_2,
            b"ySubscriptYOffset\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ySubscriptYOffset as i64),
        );
        json_object_push(
            os_2,
            b"ySupscriptXSize\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ySupscriptXSize as i64),
        );
        json_object_push(
            os_2,
            b"ySupscriptYSize\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ySupscriptYSize as i64),
        );
        json_object_push(
            os_2,
            b"ySupscriptXOffset\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ySupscriptXOffset as i64),
        );
        json_object_push(
            os_2,
            b"ySupscriptYOffset\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).ySupscriptYOffset as i64),
        );
        json_object_push(
            os_2,
            b"yStrikeoutSize\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).yStrikeoutSize as i64),
        );
        json_object_push(
            os_2,
            b"yStrikeoutPosition\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).yStrikeoutPosition as i64),
        );
        json_object_push(
            os_2,
            b"sFamilyClass\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).sFamilyClass as i64),
        );
        let mut panose: *mut JsonValue = json_array_new(10 as usize);
        let mut j: u8 = 0 as u8;
        while (j as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
            json_array_push(
                panose,
                json_integer_new((*table).panose[j as usize] as i64),
            );
            j = j.wrapping_add(1);
        }
        json_object_push(
            os_2,
            b"panose\0" as *const u8 as *const ::core::ffi::c_char,
            panose,
        );
        json_object_push(
            os_2,
            b"ulUnicodeRange1\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).ulUnicodeRange1 as ::core::ffi::c_int,
                &UNICODE_RANGE_LABELS1,
            ),
        );
        json_object_push(
            os_2,
            b"ulUnicodeRange2\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).ulUnicodeRange2 as ::core::ffi::c_int,
                &UNICODE_RANGE_LABELS2,
            ),
        );
        json_object_push(
            os_2,
            b"ulUnicodeRange3\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).ulUnicodeRange3 as ::core::ffi::c_int,
                &UNICODE_RANGE_LABELS3,
            ),
        );
        json_object_push(
            os_2,
            b"ulUnicodeRange4\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).ulUnicodeRange4 as ::core::ffi::c_int,
                &UNICODE_RANGE_LABELS4,
            ),
        );
        let mut vendorid: SdsRaw = sdsnewlen(
            &raw const (*table).achVendID as *const u8 as *const ::core::ffi::c_void,
            4 as usize,
        );
        json_object_push(
            os_2,
            b"achVendID\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new(vendorid as *const ::core::ffi::c_char),
        );
        sdsfree(vendorid);
        json_object_push(
            os_2,
            b"fsSelection\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).fsSelection as ::core::ffi::c_int,
                &FS_SELECTION_LABELS,
            ),
        );
        json_object_push(
            os_2,
            b"usFirstCharIndex\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usFirstCharIndex as i64),
        );
        json_object_push(
            os_2,
            b"usLastCharIndex\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usLastCharIndex as i64),
        );
        json_object_push(
            os_2,
            b"sTypoAscender\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).sTypoAscender as i64),
        );
        json_object_push(
            os_2,
            b"sTypoDescender\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).sTypoDescender as i64),
        );
        json_object_push(
            os_2,
            b"sTypoLineGap\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).sTypoLineGap as i64),
        );
        json_object_push(
            os_2,
            b"usWinAscent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usWinAscent as i64),
        );
        json_object_push(
            os_2,
            b"usWinDescent\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usWinDescent as i64),
        );
        json_object_push(
            os_2,
            b"ulCodePageRange1\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).ulCodePageRange1 as ::core::ffi::c_int,
                &CODE_PAGE_LABELS1,
            ),
        );
        json_object_push(
            os_2,
            b"ulCodePageRange2\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).ulCodePageRange2 as ::core::ffi::c_int,
                &CODE_PAGE_LABELS2,
            ),
        );
        json_object_push(
            os_2,
            b"sxHeight\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).sxHeight as i64),
        );
        json_object_push(
            os_2,
            b"sCapHeight\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).sCapHeight as i64),
        );
        json_object_push(
            os_2,
            b"usDefaultChar\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usDefaultChar as i64),
        );
        json_object_push(
            os_2,
            b"usBreakChar\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usBreakChar as i64),
        );
        json_object_push(
            os_2,
            b"usMaxContext\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usMaxContext as i64),
        );
        json_object_push(
            os_2,
            b"usLowerOpticalPointSize\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usLowerOpticalPointSize as i64),
        );
        json_object_push(
            os_2,
            b"usUpperOpticalPointSize\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).usUpperOpticalPointSize as i64),
        );
        json_object_push(
            root,
            b"OS_2\0" as *const u8 as *const ::core::ffi::c_char,
            os_2,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_os_2(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut Os2Table {
    let mut os_2: *mut Os2Table = (
        TABLE_I_OS_2.create.expect("non-null function pointer"))();
    if os_2.is_null() {
        return ::core::ptr::null_mut::<Os2Table>();
    }
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
    table = json_obj_get_type(
        root,
        b"OS_2\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::sdsbuild!(sdsempty(), b"OS/2"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            (*os_2).version = json_obj_getnum_fallback(
                table,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).xAvgCharWidth = json_obj_getnum_fallback(
                table,
                b"xAvgCharWidth\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).usWeightClass = json_obj_getnum_fallback(
                table,
                b"usWeightClass\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).usWidthClass = json_obj_getnum_fallback(
                table,
                b"usWidthClass\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).fsType = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"fsType\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &FS_TYPE_LABELS,
            ) as u16;
            (*os_2).ySubscriptXSize = json_obj_getnum_fallback(
                table,
                b"ySubscriptXSize\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).ySubscriptYSize = json_obj_getnum_fallback(
                table,
                b"ySubscriptYSize\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).ySubscriptXOffset = json_obj_getnum_fallback(
                table,
                b"ySubscriptXOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).ySubscriptYOffset = json_obj_getnum_fallback(
                table,
                b"ySubscriptYOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).ySupscriptXSize = json_obj_getnum_fallback(
                table,
                b"ySupscriptXSize\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).ySupscriptYSize = json_obj_getnum_fallback(
                table,
                b"ySupscriptYSize\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).ySupscriptXOffset = json_obj_getnum_fallback(
                table,
                b"ySupscriptXOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).ySupscriptYOffset = json_obj_getnum_fallback(
                table,
                b"ySupscriptYOffset\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).yStrikeoutSize = json_obj_getnum_fallback(
                table,
                b"yStrikeoutSize\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).yStrikeoutPosition = json_obj_getnum_fallback(
                table,
                b"yStrikeoutPosition\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).sFamilyClass = json_obj_getnum_fallback(
                table,
                b"sFamilyClass\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).fsSelection = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"fsSelection\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &FS_SELECTION_LABELS,
            ) as u16;
            (*os_2).usFirstCharIndex = json_obj_getnum_fallback(
                table,
                b"usFirstCharIndex\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).usLastCharIndex = json_obj_getnum_fallback(
                table,
                b"usLastCharIndex\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).sTypoAscender = json_obj_getnum_fallback(
                table,
                b"sTypoAscender\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).sTypoDescender = json_obj_getnum_fallback(
                table,
                b"sTypoDescender\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).sTypoLineGap = json_obj_getnum_fallback(
                table,
                b"sTypoLineGap\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).usWinAscent = json_obj_getnum_fallback(
                table,
                b"usWinAscent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).usWinDescent = json_obj_getnum_fallback(
                table,
                b"usWinDescent\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).ulCodePageRange1 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulCodePageRange1\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &CODE_PAGE_LABELS1,
            );
            (*os_2).ulCodePageRange2 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulCodePageRange2\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &CODE_PAGE_LABELS2,
            );
            (*os_2).ulUnicodeRange1 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange1\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &UNICODE_RANGE_LABELS1,
            );
            (*os_2).ulUnicodeRange2 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange2\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &UNICODE_RANGE_LABELS2,
            );
            (*os_2).ulUnicodeRange3 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange3\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &UNICODE_RANGE_LABELS3,
            );
            (*os_2).ulUnicodeRange4 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange4\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &UNICODE_RANGE_LABELS4,
            );
            (*os_2).sxHeight = json_obj_getnum_fallback(
                table,
                b"sxHeight\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).sCapHeight = json_obj_getnum_fallback(
                table,
                b"sCapHeight\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as i16;
            (*os_2).usDefaultChar = json_obj_getnum_fallback(
                table,
                b"usDefaultChar\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).usBreakChar = json_obj_getnum_fallback(
                table,
                b"usBreakChar\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).usMaxContext = json_obj_getnum_fallback(
                table,
                b"usMaxContext\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).usLowerOpticalPointSize = json_obj_getnum_fallback(
                table,
                b"usLowerOpticalPointSize\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            (*os_2).usUpperOpticalPointSize = json_obj_getnum_fallback(
                table,
                b"usUpperOpticalPointSize\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
            ) as u16;
            let mut panose: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
            panose = json_obj_get_type(
                table,
                b"panose\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::Array,
            );
            if !panose.is_null() {
                let mut j: u32 = 0 as u32;
                while j < (*panose).u.array.length as u32 && j < 10 as u32 {
                    let mut term: *mut JsonValue =
                        *(*panose).u.array.values.offset(j as isize) as *mut JsonValue;
                    if (*term).type_0 == JsonType::Integer
                    {
                        (*os_2).panose[j as usize] = (*term).u.integer as u8;
                    } else if (*term).type_0 == JsonType::Double
                    {
                        (*os_2).panose[j as usize] = (*term).u.dbl as u8;
                    }
                    j = j.wrapping_add(1);
                }
            }
            let mut vendorid: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
            vendorid = json_obj_get_type(
                table,
                b"achVendID\0" as *const u8 as *const ::core::ffi::c_char,
                JsonType::String,
            );
            if !vendorid.is_null() {
                (*os_2).achVendID[0 as ::core::ffi::c_int as usize] = ' ' as i32 as u8;
                (*os_2).achVendID[1 as ::core::ffi::c_int as usize] = ' ' as i32 as u8;
                (*os_2).achVendID[2 as ::core::ffi::c_int as usize] = ' ' as i32 as u8;
                (*os_2).achVendID[3 as ::core::ffi::c_int as usize] = ' ' as i32 as u8;
                if (*vendorid).u.string.length >= 4 as ::core::ffi::c_uint {
                    memcpy(
                        &raw mut (*os_2).achVendID as *mut u8 as *mut ::core::ffi::c_void,
                        (*vendorid).u.string.ptr as *const ::core::ffi::c_void,
                        4 as usize,
                    );
                } else {
                    memcpy(
                        &raw mut (*os_2).achVendID as *mut u8 as *mut ::core::ffi::c_void,
                        (*vendorid).u.string.ptr as *const ::core::ffi::c_void,
                        (*vendorid).u.string.length as usize,
                    );
                }
            }
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    if ((*os_2).version as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        (*os_2).version = 1 as u16;
    }
    return os_2;
}
pub unsafe extern "C" fn otfcc_build_os_2(
    mut os_2: *const Os2Table,
    mut _options: *const Options,
) -> *mut Buffer {
    if os_2.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, (*os_2).version);
    bufwrite16b(buf, (*os_2).xAvgCharWidth as u16);
    bufwrite16b(buf, (*os_2).usWeightClass);
    bufwrite16b(buf, (*os_2).usWidthClass);
    bufwrite16b(buf, (*os_2).fsType);
    bufwrite16b(buf, (*os_2).ySubscriptXSize as u16);
    bufwrite16b(buf, (*os_2).ySubscriptYSize as u16);
    bufwrite16b(buf, (*os_2).ySubscriptXOffset as u16);
    bufwrite16b(buf, (*os_2).ySubscriptYOffset as u16);
    bufwrite16b(buf, (*os_2).ySupscriptXSize as u16);
    bufwrite16b(buf, (*os_2).ySupscriptYSize as u16);
    bufwrite16b(buf, (*os_2).ySupscriptXOffset as u16);
    bufwrite16b(buf, (*os_2).ySupscriptYOffset as u16);
    bufwrite16b(buf, (*os_2).yStrikeoutSize as u16);
    bufwrite16b(buf, (*os_2).yStrikeoutPosition as u16);
    bufwrite16b(buf, (*os_2).sFamilyClass as u16);
    bufwrite_bytes(
        buf,
        10 as usize,
        &raw const (*os_2).panose as *const u8,
    );
    bufwrite32b(buf, (*os_2).ulUnicodeRange1);
    bufwrite32b(buf, (*os_2).ulUnicodeRange2);
    bufwrite32b(buf, (*os_2).ulUnicodeRange3);
    bufwrite32b(buf, (*os_2).ulUnicodeRange4);
    bufwrite_bytes(
        buf,
        4 as usize,
        &raw const (*os_2).achVendID as *const u8,
    );
    bufwrite16b(buf, (*os_2).fsSelection);
    bufwrite16b(buf, (*os_2).usFirstCharIndex);
    bufwrite16b(buf, (*os_2).usLastCharIndex);
    bufwrite16b(buf, (*os_2).sTypoAscender as u16);
    bufwrite16b(buf, (*os_2).sTypoDescender as u16);
    bufwrite16b(buf, (*os_2).sTypoLineGap as u16);
    bufwrite16b(buf, (*os_2).usWinAscent);
    bufwrite16b(buf, (*os_2).usWinDescent);
    bufwrite32b(buf, (*os_2).ulCodePageRange1);
    bufwrite32b(buf, (*os_2).ulCodePageRange2);
    if ((*os_2).version as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        return buf;
    }
    bufwrite16b(buf, (*os_2).sxHeight as u16);
    bufwrite16b(buf, (*os_2).sCapHeight as u16);
    bufwrite16b(buf, (*os_2).usDefaultChar);
    bufwrite16b(buf, (*os_2).usBreakChar);
    bufwrite16b(buf, (*os_2).usMaxContext);
    if ((*os_2).version as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
        return buf;
    }
    bufwrite16b(buf, (*os_2).usLowerOpticalPointSize);
    bufwrite16b(buf, (*os_2).usUpperOpticalPointSize);
    return buf;
}
