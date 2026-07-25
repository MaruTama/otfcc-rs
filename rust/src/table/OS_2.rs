use libc::{free, malloc, memcpy, memset, strcmp};
extern "C" {
    fn sdsnewlen(init: *const ::core::ffi::c_void, initlen: usize) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn bufnew() -> *mut caryll_Buffer;
    fn bufwrite16b(buf: *mut caryll_Buffer, x: u16);
    fn bufwrite32b(buf: *mut caryll_Buffer, x: u32);
    fn bufwrite_bytes(buf: *mut caryll_Buffer, size: usize, str: *const u8);
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new(_: *const ::core::ffi::c_char) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
}
use crate::support::binio::{read_16u, read_16s, read_32u};
use crate::logger::{log_type_warning, log_vl_important, otfcc_ILogger};
use crate::support::buffer::{caryll_Buffer};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_array, json_boolean, json_double, json_integer, json_object, json_string, json_type, json_value};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};
use crate::support::{true_0};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table_OS_2 {
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
pub struct __caryll_elementinterface_table_OS_2 {
    pub init: Option<unsafe extern "C" fn(*mut table_OS_2) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut table_OS_2, *const table_OS_2) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut table_OS_2, *mut table_OS_2) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut table_OS_2) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut table_OS_2, table_OS_2) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut table_OS_2, table_OS_2) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut table_OS_2>,
    pub free: Option<unsafe extern "C" fn(*mut table_OS_2) -> ()>,
}
#[inline]
unsafe extern "C" fn initOS2(mut table: *mut table_OS_2) {
    memset(
        table as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<table_OS_2>() as usize,
    );
    (*table).version = 4 as u16;
}
#[inline]
unsafe extern "C" fn disposeOS2(mut _table: *mut table_OS_2) {}
#[inline]
unsafe extern "C" fn table_OS_2_dispose(mut x: *mut table_OS_2) {
    disposeOS2(x);
}
#[inline]
unsafe extern "C" fn table_OS_2_create() -> *mut table_OS_2 {
    let mut x: *mut table_OS_2 =
        malloc(::core::mem::size_of::<table_OS_2>() as usize) as *mut table_OS_2;
    table_OS_2_init(x);
    return x;
}
#[inline]
unsafe extern "C" fn table_OS_2_copyReplace(mut dst: *mut table_OS_2, src: table_OS_2) {
    table_OS_2_dispose(dst);
    table_OS_2_copy(dst, &raw const src);
}
#[inline]
unsafe extern "C" fn table_OS_2_init(mut x: *mut table_OS_2) {
    initOS2(x);
}
#[no_mangle]
pub static mut table_iOS_2: __caryll_elementinterface_table_OS_2 = {
    __caryll_elementinterface_table_OS_2 {
        init: Some(table_OS_2_init as unsafe extern "C" fn(*mut table_OS_2) -> ()),
        copy: Some(
            table_OS_2_copy as unsafe extern "C" fn(*mut table_OS_2, *const table_OS_2) -> (),
        ),
        move_0: Some(
            table_OS_2_move as unsafe extern "C" fn(*mut table_OS_2, *mut table_OS_2) -> (),
        ),
        dispose: Some(table_OS_2_dispose as unsafe extern "C" fn(*mut table_OS_2) -> ()),
        replace: Some(
            table_OS_2_replace as unsafe extern "C" fn(*mut table_OS_2, table_OS_2) -> (),
        ),
        copyReplace: Some(
            table_OS_2_copyReplace as unsafe extern "C" fn(*mut table_OS_2, table_OS_2) -> (),
        ),
        create: Some(table_OS_2_create),
        free: Some(table_OS_2_free as unsafe extern "C" fn(*mut table_OS_2) -> ()),
    }
};
#[inline]
unsafe extern "C" fn table_OS_2_copy(mut dst: *mut table_OS_2, mut src: *const table_OS_2) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_OS_2>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_OS_2_replace(mut dst: *mut table_OS_2, src: table_OS_2) {
    table_OS_2_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_OS_2>() as usize,
    );
}
#[inline]
unsafe extern "C" fn table_OS_2_move(mut dst: *mut table_OS_2, mut src: *mut table_OS_2) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<table_OS_2>() as usize,
    );
    table_OS_2_init(src);
}
#[inline]
unsafe extern "C" fn table_OS_2_free(mut x: *mut table_OS_2) {
    if x.is_null() {
        return;
    }
    table_OS_2_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_readOS_2(
    packet: otfcc_Packet,
    mut options: *const otfcc_Options,
) -> *mut table_OS_2 {
    let mut os_2: *mut table_OS_2 = ::core::ptr::null_mut::<table_OS_2>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == 1330851634i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    if !(length < 2 as u32) {
                        os_2 = (
                            table_iOS_2.create.expect("non-null function pointer"))();
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
                        (*options).logger as *mut otfcc_ILogger,
                        log_vl_important as ::core::ffi::c_int as u8,
                        log_type_warning,
                        sdscatprintf(
                            sdsempty(),
                            b"table 'OS/2' corrupted.\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        ),
                    );
                    if !os_2.is_null() {
                        free(os_2 as *mut ::core::ffi::c_void);
                        os_2 = ::core::ptr::null_mut::<table_OS_2>();
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
    return ::core::ptr::null_mut::<table_OS_2>();
}
#[no_mangle]
pub static mut fsTypeLabels: [*const ::core::ffi::c_char; 11] = [
    b"_reserved1\0" as *const u8 as *const ::core::ffi::c_char,
    b"restrictedLicense\0" as *const u8 as *const ::core::ffi::c_char,
    b"previewPrintLicense\0" as *const u8 as *const ::core::ffi::c_char,
    b"editableEmbedding\0" as *const u8 as *const ::core::ffi::c_char,
    b"_reserved2\0" as *const u8 as *const ::core::ffi::c_char,
    b"_reserved3\0" as *const u8 as *const ::core::ffi::c_char,
    b"_reserved4\0" as *const u8 as *const ::core::ffi::c_char,
    b"_reserved5\0" as *const u8 as *const ::core::ffi::c_char,
    b"noSubsetting\0" as *const u8 as *const ::core::ffi::c_char,
    b"bitmapEmbeddingOnly\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub static mut fsSelectionLabels: [*const ::core::ffi::c_char; 11] = [
    b"italic\0" as *const u8 as *const ::core::ffi::c_char,
    b"underscore\0" as *const u8 as *const ::core::ffi::c_char,
    b"negative\0" as *const u8 as *const ::core::ffi::c_char,
    b"outlined\0" as *const u8 as *const ::core::ffi::c_char,
    b"strikeout\0" as *const u8 as *const ::core::ffi::c_char,
    b"bold\0" as *const u8 as *const ::core::ffi::c_char,
    b"regular\0" as *const u8 as *const ::core::ffi::c_char,
    b"useTypoMetrics\0" as *const u8 as *const ::core::ffi::c_char,
    b"wws\0" as *const u8 as *const ::core::ffi::c_char,
    b"oblique\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub static mut codePageLabels1: [*const ::core::ffi::c_char; 33] = [
    b"latin1\0" as *const u8 as *const ::core::ffi::c_char,
    b"latin2\0" as *const u8 as *const ::core::ffi::c_char,
    b"cyrillic\0" as *const u8 as *const ::core::ffi::c_char,
    b"greek\0" as *const u8 as *const ::core::ffi::c_char,
    b"turkish\0" as *const u8 as *const ::core::ffi::c_char,
    b"hebrew\0" as *const u8 as *const ::core::ffi::c_char,
    b"arabic\0" as *const u8 as *const ::core::ffi::c_char,
    b"windowsBaltic\0" as *const u8 as *const ::core::ffi::c_char,
    b"vietnamese\0" as *const u8 as *const ::core::ffi::c_char,
    b"ansi1\0" as *const u8 as *const ::core::ffi::c_char,
    b"ansi2\0" as *const u8 as *const ::core::ffi::c_char,
    b"ansi3\0" as *const u8 as *const ::core::ffi::c_char,
    b"ansi4\0" as *const u8 as *const ::core::ffi::c_char,
    b"ansi5\0" as *const u8 as *const ::core::ffi::c_char,
    b"ansi6\0" as *const u8 as *const ::core::ffi::c_char,
    b"ansi7\0" as *const u8 as *const ::core::ffi::c_char,
    b"thai\0" as *const u8 as *const ::core::ffi::c_char,
    b"jis\0" as *const u8 as *const ::core::ffi::c_char,
    b"gbk\0" as *const u8 as *const ::core::ffi::c_char,
    b"korean\0" as *const u8 as *const ::core::ffi::c_char,
    b"big5\0" as *const u8 as *const ::core::ffi::c_char,
    b"koreanJohab\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem1\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem2\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem3\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem4\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem5\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem6\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem7\0" as *const u8 as *const ::core::ffi::c_char,
    b"macRoman\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem\0" as *const u8 as *const ::core::ffi::c_char,
    b"symbol\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub static mut codePageLabels2: [*const ::core::ffi::c_char; 33] = [
    b"oem8\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem9\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem10\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem11\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem12\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem13\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem14\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem15\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem16\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem17\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem18\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem19\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem20\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem21\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem22\0" as *const u8 as *const ::core::ffi::c_char,
    b"oem23\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp869\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp866\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp865\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp864\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp863\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp862\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp861\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp860\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp857\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp855\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp852\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp775\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp737\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp708\0" as *const u8 as *const ::core::ffi::c_char,
    b"cp850\0" as *const u8 as *const ::core::ffi::c_char,
    b"ascii\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub static mut unicodeRangeLabels1: [*const ::core::ffi::c_char; 33] = [
    b"Basic_Latin\0" as *const u8 as *const ::core::ffi::c_char,
    b"Latin_1_Supplement\0" as *const u8 as *const ::core::ffi::c_char,
    b"Latin_Extended_A\0" as *const u8 as *const ::core::ffi::c_char,
    b"Latin_Extended_B\0" as *const u8 as *const ::core::ffi::c_char,
    b"Phonetics\0" as *const u8 as *const ::core::ffi::c_char,
    b"Spacing_Modifiers\0" as *const u8 as *const ::core::ffi::c_char,
    b"Combining_Diacritical_Marks\0" as *const u8 as *const ::core::ffi::c_char,
    b"Greek_and_Coptic\0" as *const u8 as *const ::core::ffi::c_char,
    b"Coptic\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cyrillic\0" as *const u8 as *const ::core::ffi::c_char,
    b"Armenian\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hebrew\0" as *const u8 as *const ::core::ffi::c_char,
    b"Vai\0" as *const u8 as *const ::core::ffi::c_char,
    b"Arabic\0" as *const u8 as *const ::core::ffi::c_char,
    b"NKo\0" as *const u8 as *const ::core::ffi::c_char,
    b"Devanagari\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bengali\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gurmukhi\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gujarati\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oriya\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tamil\0" as *const u8 as *const ::core::ffi::c_char,
    b"Telugu\0" as *const u8 as *const ::core::ffi::c_char,
    b"Kannada\0" as *const u8 as *const ::core::ffi::c_char,
    b"Malayalam\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thai\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lao\0" as *const u8 as *const ::core::ffi::c_char,
    b"Georgian\0" as *const u8 as *const ::core::ffi::c_char,
    b"Balinese\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hangul_Jamo\0" as *const u8 as *const ::core::ffi::c_char,
    b"Latin_Extended_Additional\0" as *const u8 as *const ::core::ffi::c_char,
    b"Greek_Extended\0" as *const u8 as *const ::core::ffi::c_char,
    b"Punctuations\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub static mut unicodeRangeLabels2: [*const ::core::ffi::c_char; 33] = [
    b"Superscripts_And_Subscripts\0" as *const u8 as *const ::core::ffi::c_char,
    b"Currency_Symbols\0" as *const u8 as *const ::core::ffi::c_char,
    b"Combining_Diacritical_Marks_For_Symbols\0" as *const u8 as *const ::core::ffi::c_char,
    b"Letterlike_Symbols\0" as *const u8 as *const ::core::ffi::c_char,
    b"Number_Forms\0" as *const u8 as *const ::core::ffi::c_char,
    b"Arrows\0" as *const u8 as *const ::core::ffi::c_char,
    b"Mathematical_Operators\0" as *const u8 as *const ::core::ffi::c_char,
    b"Miscellaneous_Technical\0" as *const u8 as *const ::core::ffi::c_char,
    b"Control_Pictures\0" as *const u8 as *const ::core::ffi::c_char,
    b"Optical_Character_Recognition\0" as *const u8 as *const ::core::ffi::c_char,
    b"Enclosed_Alphanumerics\0" as *const u8 as *const ::core::ffi::c_char,
    b"Box_Drawing\0" as *const u8 as *const ::core::ffi::c_char,
    b"Block_Elements\0" as *const u8 as *const ::core::ffi::c_char,
    b"Geometric_Shapes\0" as *const u8 as *const ::core::ffi::c_char,
    b"Miscellaneous_Symbols\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dingbats\0" as *const u8 as *const ::core::ffi::c_char,
    b"CJK_Symbols_And_Punctuation\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hiragana\0" as *const u8 as *const ::core::ffi::c_char,
    b"Katakana\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bopomofo\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hangul_Compatibility_Jamo\0" as *const u8 as *const ::core::ffi::c_char,
    b"Phags_pa\0" as *const u8 as *const ::core::ffi::c_char,
    b"Enclosed_CJK_Letters_And_Months\0" as *const u8 as *const ::core::ffi::c_char,
    b"CJK_Compatibility\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hangul_Syllables\0" as *const u8 as *const ::core::ffi::c_char,
    b"Non_Plane_0\0" as *const u8 as *const ::core::ffi::c_char,
    b"Phoenician\0" as *const u8 as *const ::core::ffi::c_char,
    b"CJK_Unified_Ideographs\0" as *const u8 as *const ::core::ffi::c_char,
    b"Private_Use_Area_p0\0" as *const u8 as *const ::core::ffi::c_char,
    b"CJK_Strokes\0" as *const u8 as *const ::core::ffi::c_char,
    b"Alphabetic_Presentation_Forms\0" as *const u8 as *const ::core::ffi::c_char,
    b"Arabic_Presentation_Forms_A\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub static mut unicodeRangeLabels3: [*const ::core::ffi::c_char; 33] = [
    b"Combining_Half_Marks\0" as *const u8 as *const ::core::ffi::c_char,
    b"Vertical_Forms_and_CJK_Compatibility_Forms\0" as *const u8 as *const ::core::ffi::c_char,
    b"Small_Form_Variants\0" as *const u8 as *const ::core::ffi::c_char,
    b"Arabic_Presentation_Forms_B\0" as *const u8 as *const ::core::ffi::c_char,
    b"Halfwidth_And_Fullwidth_Forms\0" as *const u8 as *const ::core::ffi::c_char,
    b"Specials\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tibetan\0" as *const u8 as *const ::core::ffi::c_char,
    b"Syriac\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thaana\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sinhala\0" as *const u8 as *const ::core::ffi::c_char,
    b"Myanmar\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ethiopic\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cherokee\0" as *const u8 as *const ::core::ffi::c_char,
    b"Unified_Canadian_Aboriginal_Syllabics\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ogham\0" as *const u8 as *const ::core::ffi::c_char,
    b"Runic\0" as *const u8 as *const ::core::ffi::c_char,
    b"Khmer\0" as *const u8 as *const ::core::ffi::c_char,
    b"Mongolian\0" as *const u8 as *const ::core::ffi::c_char,
    b"Braille_Patterns\0" as *const u8 as *const ::core::ffi::c_char,
    b"Yi_Syllables\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tagalog\0" as *const u8 as *const ::core::ffi::c_char,
    b"Old_Italic\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gothic\0" as *const u8 as *const ::core::ffi::c_char,
    b"Deseret\0" as *const u8 as *const ::core::ffi::c_char,
    b"Musical_Symbols\0" as *const u8 as *const ::core::ffi::c_char,
    b"Mathematical_Alphanumeric_Symbols\0" as *const u8 as *const ::core::ffi::c_char,
    b"Private_Use_p15_and_p16\0" as *const u8 as *const ::core::ffi::c_char,
    b"Variation_Selectors\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tags\0" as *const u8 as *const ::core::ffi::c_char,
    b"Limbu\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tai_Le\0" as *const u8 as *const ::core::ffi::c_char,
    b"New_Tai_Lue\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub static mut unicodeRangeLabels4: [*const ::core::ffi::c_char; 28] = [
    b"Buginese\0" as *const u8 as *const ::core::ffi::c_char,
    b"Glagolitic\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tifinagh\0" as *const u8 as *const ::core::ffi::c_char,
    b"Yijing_Hexagram_Symbols\0" as *const u8 as *const ::core::ffi::c_char,
    b"Syloti_Nagri\0" as *const u8 as *const ::core::ffi::c_char,
    b"Linear_B_Syllabary_Ideograms_and_Aegean_Numbers\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ancient_Greek_Numbers\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ugaritic\0" as *const u8 as *const ::core::ffi::c_char,
    b"Old_Persian\0" as *const u8 as *const ::core::ffi::c_char,
    b"Shavian\0" as *const u8 as *const ::core::ffi::c_char,
    b"Osmanya\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cypriot_Syllabary\0" as *const u8 as *const ::core::ffi::c_char,
    b"Kharoshthi\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tai_Xuan_Jing_Symbols\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cuneiform\0" as *const u8 as *const ::core::ffi::c_char,
    b"Counting_Rod_Numerals\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sundanese\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lepcha\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ol_Chiki\0" as *const u8 as *const ::core::ffi::c_char,
    b"Saurashtra\0" as *const u8 as *const ::core::ffi::c_char,
    b"Kayah_Li\0" as *const u8 as *const ::core::ffi::c_char,
    b"Rejang\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cham\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ancient_Symbols\0" as *const u8 as *const ::core::ffi::c_char,
    b"Phaistos_Disc\0" as *const u8 as *const ::core::ffi::c_char,
    b"Carian_and_Lycian\0" as *const u8 as *const ::core::ffi::c_char,
    b"Domino_and_Mahjong_Tiles\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpOS_2(
    mut table: *const table_OS_2,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
) {
    if table.is_null() {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"OS/2\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut os_2: *mut json_value = json_object_new(30 as usize);
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
                &raw mut fsTypeLabels as *mut *const ::core::ffi::c_char,
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
        let mut panose: *mut json_value = json_array_new(10 as usize);
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
                &raw mut unicodeRangeLabels1 as *mut *const ::core::ffi::c_char,
            ),
        );
        json_object_push(
            os_2,
            b"ulUnicodeRange2\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).ulUnicodeRange2 as ::core::ffi::c_int,
                &raw mut unicodeRangeLabels2 as *mut *const ::core::ffi::c_char,
            ),
        );
        json_object_push(
            os_2,
            b"ulUnicodeRange3\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).ulUnicodeRange3 as ::core::ffi::c_int,
                &raw mut unicodeRangeLabels3 as *mut *const ::core::ffi::c_char,
            ),
        );
        json_object_push(
            os_2,
            b"ulUnicodeRange4\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).ulUnicodeRange4 as ::core::ffi::c_int,
                &raw mut unicodeRangeLabels4 as *mut *const ::core::ffi::c_char,
            ),
        );
        let mut vendorid: sds = sdsnewlen(
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
                &raw mut fsSelectionLabels as *mut *const ::core::ffi::c_char,
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
                &raw mut codePageLabels1 as *mut *const ::core::ffi::c_char,
            ),
        );
        json_object_push(
            os_2,
            b"ulCodePageRange2\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*table).ulCodePageRange2 as ::core::ffi::c_int,
                &raw mut codePageLabels2 as *mut *const ::core::ffi::c_char,
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
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_parseOS_2(
    mut root: *const json_value,
    mut options: *const otfcc_Options,
) -> *mut table_OS_2 {
    let mut os_2: *mut table_OS_2 = (
        table_iOS_2.create.expect("non-null function pointer"))();
    if os_2.is_null() {
        return ::core::ptr::null_mut::<table_OS_2>();
    }
    let mut table: *mut json_value = ::core::ptr::null_mut::<json_value>();
    table = json_obj_get_type(
        root,
        b"OS_2\0" as *const u8 as *const ::core::ffi::c_char,
        json_object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            sdscatprintf(
                sdsempty(),
                b"OS/2\0" as *const u8 as *const ::core::ffi::c_char,
            ),
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
                &raw mut fsTypeLabels as *mut *const ::core::ffi::c_char,
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
                &raw mut fsSelectionLabels as *mut *const ::core::ffi::c_char,
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
                &raw mut codePageLabels1 as *mut *const ::core::ffi::c_char,
            );
            (*os_2).ulCodePageRange2 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulCodePageRange2\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &raw mut codePageLabels2 as *mut *const ::core::ffi::c_char,
            );
            (*os_2).ulUnicodeRange1 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange1\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &raw mut unicodeRangeLabels1 as *mut *const ::core::ffi::c_char,
            );
            (*os_2).ulUnicodeRange2 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange2\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &raw mut unicodeRangeLabels2 as *mut *const ::core::ffi::c_char,
            );
            (*os_2).ulUnicodeRange3 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange3\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &raw mut unicodeRangeLabels3 as *mut *const ::core::ffi::c_char,
            );
            (*os_2).ulUnicodeRange4 = otfcc_parse_flags(
                json_obj_get(
                    table,
                    b"ulUnicodeRange4\0" as *const u8 as *const ::core::ffi::c_char,
                ),
                &raw mut unicodeRangeLabels4 as *mut *const ::core::ffi::c_char,
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
            let mut panose: *mut json_value = ::core::ptr::null_mut::<json_value>();
            panose = json_obj_get_type(
                table,
                b"panose\0" as *const u8 as *const ::core::ffi::c_char,
                json_array,
            );
            if !panose.is_null() {
                let mut j: u32 = 0 as u32;
                while j < (*panose).u.array.length as u32 && j < 10 as u32 {
                    let mut term: *mut json_value =
                        *(*panose).u.array.values.offset(j as isize) as *mut json_value;
                    if (*term).type_0 as ::core::ffi::c_uint
                        == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*os_2).panose[j as usize] = (*term).u.integer as u8;
                    } else if (*term).type_0 as ::core::ffi::c_uint
                        == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*os_2).panose[j as usize] = (*term).u.dbl as u8;
                    }
                    j = j.wrapping_add(1);
                }
            }
            let mut vendorid: *mut json_value = ::core::ptr::null_mut::<json_value>();
            vendorid = json_obj_get_type(
                table,
                b"achVendID\0" as *const u8 as *const ::core::ffi::c_char,
                json_string,
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
                (*options).logger as *mut otfcc_ILogger
            );
        }
    }
    if ((*os_2).version as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        (*os_2).version = 1 as u16;
    }
    return os_2;
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_buildOS_2(
    mut os_2: *const table_OS_2,
    mut _options: *const otfcc_Options,
) -> *mut caryll_Buffer {
    if os_2.is_null() {
        return ::core::ptr::null_mut::<caryll_Buffer>();
    }
    let mut buf: *mut caryll_Buffer = bufnew();
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
#[inline]
unsafe extern "C" fn json_obj_get(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> *mut json_value {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<json_value>();
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            return (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        }
        _k = _k.wrapping_add(1);
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_get_type(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    type_0: json_type,
) -> *mut json_value {
    let mut v: *mut json_value = json_obj_get(obj, key);
    if !v.is_null() && (*v).type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
        return v;
    }
    return ::core::ptr::null_mut::<json_value>();
}
#[inline]
unsafe extern "C" fn json_obj_getnum_fallback(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
    mut fallback: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return fallback;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.integer as ::core::ffi::c_double;
            }
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.dbl;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return fallback;
}
#[inline]
unsafe extern "C" fn json_obj_getbool(
    mut obj: *const json_value,
    mut key: *const ::core::ffi::c_char,
) -> bool {
    if obj.is_null()
        || (*obj).type_0 as ::core::ffi::c_uint
            != json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return false;
    }
    let mut _k: u32 = 0 as u32;
    while _k < (*obj).u.object.length as u32 {
        let mut ck: *mut ::core::ffi::c_char = (*(*obj).u.object.values.offset(_k as isize)).name;
        let mut cv: *mut json_value =
            (*(*obj).u.object.values.offset(_k as isize)).value as *mut json_value;
        if strcmp(ck, key) == 0 as ::core::ffi::c_int {
            if !cv.is_null()
                && (*cv).type_0 as ::core::ffi::c_uint
                    == json_boolean as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return (*cv).u.boolean != 0;
            }
        }
        _k = _k.wrapping_add(1);
    }
    return false;
}
#[inline]
unsafe extern "C" fn otfcc_dump_flags(
    mut flags: ::core::ffi::c_int,
    mut labels: *mut *const ::core::ffi::c_char,
) -> *mut json_value {
    let mut v: *mut json_value = json_object_new(0 as usize);
    let mut j: u16 = 0 as u16;
    while !(*labels.offset(j as isize)).is_null() {
        if flags & (1 as ::core::ffi::c_int) << j as ::core::ffi::c_int != 0 {
            json_object_push(v, *labels.offset(j as isize), json_boolean_new(true_0));
        }
        j = j.wrapping_add(1);
    }
    return v;
}
#[inline]
unsafe extern "C" fn otfcc_parse_flags(
    mut v: *const json_value,
    mut labels: *mut *const ::core::ffi::c_char,
) -> u32 {
    if v.is_null() {
        return 0 as u32;
    }
    if (*v).type_0 as ::core::ffi::c_uint
        == json_integer as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*v).u.integer as u32;
    } else if (*v).type_0 as ::core::ffi::c_uint
        == json_double as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*v).u.dbl as u32;
    } else if (*v).type_0 as ::core::ffi::c_uint
        == json_object as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut flags: u32 = 0 as u32;
        let mut j: u16 = 0 as u16;
        while !(*labels.offset(j as isize)).is_null() {
            if json_obj_getbool(v, *labels.offset(j as isize)) {
                flags |= ((1 as ::core::ffi::c_int) << j as ::core::ffi::c_int) as u32;
            }
            j = j.wrapping_add(1);
        }
        return flags;
    } else {
        return 0 as u32;
    };
}
