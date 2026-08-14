#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md

use crate::support::binio::{read_16u, read_32u, read_32s};
use crate::logger::{ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{F16Dot16, FontFilePointer, GlyphId};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::support::glyph_order::{GlyphOrder};
use crate::support::parsed_json::{ParsedValue, json_obj_get_type, json_obj_getbool, json_obj_getnum};
use crate::support::buffer::{bufnew, bufwrite16b, bufwrite32b, bufwrite8, bufwrite_bytes};
use crate::support::glyph_order::{OTFCC_PKG_GLYPH_ORDER};
use crate::support::primitives::{otfcc_from_fixed, otfcc_to_fixed};
use crate::support::built_json::{BuiltValue, json_boolean_new, json_double_new, json_integer_new, json_object_new, json_object_push};

#[repr(C)]
pub struct PostTable {
    pub version: F16Dot16,
    pub italic_angle: F16Dot16,
    pub underline_position: i16,
    pub underline_thickness: i16,
    pub is_fixed_pitch: u32,
    pub min_mem_type42: u32,
    pub max_mem_type42: u32,
    pub min_mem_type1: u32,
    pub max_mem_type1: u32,
    pub post_name_map: *mut GlyphOrder,
}
// Stage 6-4 "Box化": `post_name_map` (populated only for a version-2.0
// 'post' table, during OTF read) is the only allocation this struct owns --
// left as a raw pointer (its own Box化 depends on how `Font.glyph_order`,
// the same `GlyphOrder` type, is eventually represented), freed the same
// way `dispose_post` always did via the `OTFCC_PKG_GLYPH_ORDER` package's
// `.free`. `Copy`/`Clone` dropped, matching `LtshTable`/`VorgTable`.
//
// `post_name_map` is a standalone allocation, not an alias into
// `Font.glyph_order`: `otf_reader/unconsolidate.rs`'s `create_glyph_order`
// only ever *reads* from it (to backfill glyph names from a version-2.0
// 'post' table) and never stores the pointer anywhere else, so there is no
// other owner to worry about aliasing with.
impl Drop for PostTable {
    fn drop(&mut self) {
        unsafe {
            if !self.post_name_map.is_null() {
                OTFCC_PKG_GLYPH_ORDER.free.expect("non-null function pointer")(self.post_name_map);
            }
        }
    }
}
static STANDARD_MAC_NAMES: [&::core::ffi::CStr; 258] = [
    c".notdef",
    c".null",
    c"nonmarkingreturn",
    c"space",
    c"exclam",
    c"quotedbl",
    c"numbersign",
    c"dollar",
    c"percent",
    c"ampersand",
    c"quotesingle",
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
    c"grave",
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
    c"Adieresis",
    c"Aring",
    c"Ccedilla",
    c"Eacute",
    c"Ntilde",
    c"Odieresis",
    c"Udieresis",
    c"aacute",
    c"agrave",
    c"acircumflex",
    c"adieresis",
    c"atilde",
    c"aring",
    c"ccedilla",
    c"eacute",
    c"egrave",
    c"ecircumflex",
    c"edieresis",
    c"iacute",
    c"igrave",
    c"icircumflex",
    c"idieresis",
    c"ntilde",
    c"oacute",
    c"ograve",
    c"ocircumflex",
    c"odieresis",
    c"otilde",
    c"uacute",
    c"ugrave",
    c"ucircumflex",
    c"udieresis",
    c"dagger",
    c"degree",
    c"cent",
    c"sterling",
    c"section",
    c"bullet",
    c"paragraph",
    c"germandbls",
    c"registered",
    c"copyright",
    c"trademark",
    c"acute",
    c"dieresis",
    c"notequal",
    c"AE",
    c"Oslash",
    c"infinity",
    c"plusminus",
    c"lessequal",
    c"greaterequal",
    c"yen",
    c"mu",
    c"partialdiff",
    c"summation",
    c"product",
    c"pi",
    c"integral",
    c"ordfeminine",
    c"ordmasculine",
    c"Omega",
    c"ae",
    c"oslash",
    c"questiondown",
    c"exclamdown",
    c"logicalnot",
    c"radical",
    c"florin",
    c"approxequal",
    c"Delta",
    c"guillemotleft",
    c"guillemotright",
    c"ellipsis",
    c"nonbreakingspace",
    c"Agrave",
    c"Atilde",
    c"Otilde",
    c"OE",
    c"oe",
    c"endash",
    c"emdash",
    c"quotedblleft",
    c"quotedblright",
    c"quoteleft",
    c"quoteright",
    c"divide",
    c"lozenge",
    c"ydieresis",
    c"Ydieresis",
    c"fraction",
    c"currency",
    c"guilsinglleft",
    c"guilsinglright",
    c"fi",
    c"fl",
    c"daggerdbl",
    c"periodcentered",
    c"quotesinglbase",
    c"quotedblbase",
    c"perthousand",
    c"Acircumflex",
    c"Ecircumflex",
    c"Aacute",
    c"Edieresis",
    c"Egrave",
    c"Iacute",
    c"Icircumflex",
    c"Idieresis",
    c"Igrave",
    c"Oacute",
    c"Ocircumflex",
    c"apple",
    c"Ograve",
    c"Uacute",
    c"Ucircumflex",
    c"Ugrave",
    c"dotlessi",
    c"circumflex",
    c"tilde",
    c"macron",
    c"breve",
    c"dotaccent",
    c"ring",
    c"cedilla",
    c"hungarumlaut",
    c"ogonek",
    c"caron",
    c"Lslash",
    c"lslash",
    c"Scaron",
    c"scaron",
    c"Zcaron",
    c"zcaron",
    c"brokenbar",
    c"Eth",
    c"eth",
    c"Yacute",
    c"yacute",
    c"Thorn",
    c"thorn",
    c"minus",
    c"multiply",
    c"onesuperior",
    c"twosuperior",
    c"threesuperior",
    c"onehalf",
    c"onequarter",
    c"threequarters",
    c"franc",
    c"Gbreve",
    c"gbreve",
    c"Idotaccent",
    c"Scedilla",
    c"scedilla",
    c"Cacute",
    c"cacute",
    c"Ccaron",
    c"ccaron",
    c"dcroat",
];
pub unsafe extern "C" fn otfcc_read_post(
    packet: Packet,
    mut _options: *const Options,
) -> Option<Box<PostTable>> {
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_POST {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    // Every field below is unconditionally overwritten by
                    // this function's own body, so `mem::zeroed()`'s default
                    // is never observed -- unlike `otfcc_parse_post`, where
                    // `.version`'s default does matter.
                    let mut post_val: PostTable = ::core::mem::zeroed();
                    post_val.version = read_32s(data as *const u8) as F16Dot16;
                    post_val.italic_angle =
                        read_32u(data.offset(4 as ::core::ffi::c_int as isize) as *const u8)
                            as F16Dot16;
                    post_val.underline_position =
                        read_16u(data.offset(8 as ::core::ffi::c_int as isize) as *const u8)
                            as i16;
                    post_val.underline_thickness =
                        read_16u(data.offset(10 as ::core::ffi::c_int as isize) as *const u8)
                            as i16;
                    post_val.is_fixed_pitch =
                        read_32u(data.offset(12 as ::core::ffi::c_int as isize) as *const u8);
                    post_val.min_mem_type42 =
                        read_32u(data.offset(16 as ::core::ffi::c_int as isize) as *const u8);
                    post_val.max_mem_type42 =
                        read_32u(data.offset(20 as ::core::ffi::c_int as isize) as *const u8);
                    post_val.min_mem_type1 =
                        read_32u(data.offset(24 as ::core::ffi::c_int as isize) as *const u8);
                    post_val.max_mem_type1 =
                        read_32u(data.offset(28 as ::core::ffi::c_int as isize) as *const u8);
                    post_val.post_name_map = ::core::ptr::null_mut::<GlyphOrder>();
                    if post_val.version == 0x20000 as F16Dot16 {
                        let mut map: *mut GlyphOrder =
                            (
                                OTFCC_PKG_GLYPH_ORDER
                                    .create
                                    .expect("non-null function pointer"))();
                        let mut pending_names: Vec<Vec<u8>> = Vec::new();
                        let mut number_glyphs: u16 = read_16u(
                            data.offset(32 as ::core::ffi::c_int as isize) as *const u8,
                        );
                        let mut offset: u32 = (34 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * number_glyphs as ::core::ffi::c_int)
                            as u32;
                        let mut pending_name_index: u16 = 0 as u16;
                        while pending_name_index as ::core::ffi::c_int <= 0xffff as ::core::ffi::c_int
                            && offset < table.length
                        {
                            let mut len: u8 = *data.offset(offset as isize);
                            let s: Vec<u8> = if len as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                ::core::slice::from_raw_parts(
                                    data.offset(offset as isize)
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                    len as usize,
                                ).to_vec()
                            } else {
                                Vec::new()
                            };
                            offset = offset.wrapping_add(
                                (len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u32,
                            );
                            pending_names.push(s);
                            pending_name_index = (pending_name_index as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as u16;
                        }
                        let mut j: u16 = 0 as u16;
                        while (j as ::core::ffi::c_int) < number_glyphs as ::core::ffi::c_int {
                            let mut name_map: u16 =
                                read_16u(data.offset(34 as ::core::ffi::c_int as isize).offset(
                                    (2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize,
                                ) as *const u8);
                            if name_map as ::core::ffi::c_int >= 258 as ::core::ffi::c_int {
                                OTFCC_PKG_GLYPH_ORDER
                                    .set_by_gid
                                    .expect("non-null function pointer")(
                                    map,
                                    j as GlyphId,
                                    pending_names[(name_map as ::core::ffi::c_int
                                        - 258 as ::core::ffi::c_int)
                                        as usize]
                                        .clone(),
                                );
                            } else {
                                OTFCC_PKG_GLYPH_ORDER
                                    .set_by_gid
                                    .expect("non-null function pointer")(
                                    map,
                                    j as GlyphId,
                                    STANDARD_MAC_NAMES[name_map as usize].to_bytes().to_vec(),
                                );
                            }
                            j = j.wrapping_add(1);
                        }
                        post_val.post_name_map = map;
                    }
                    return Some(Box::new(post_val));
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_dump_post(
    table: Option<&PostTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let table = match table {
        Some(t) => t as *const PostTable,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::bytesbuild!(b"post"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut post: *mut BuiltValue = json_object_new(10 as usize);
        json_object_push(
            post,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_double_new(otfcc_from_fixed((*table).version)),
        );
        json_object_push(
            post,
            b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new(otfcc_from_fixed((*table).italic_angle) as i64),
        );
        json_object_push(
            post,
            b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).underline_position as i64),
        );
        json_object_push(
            post,
            b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).underline_thickness as i64),
        );
        json_object_push(
            post,
            b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            json_boolean_new((*table).is_fixed_pitch as ::core::ffi::c_int),
        );
        json_object_push(
            post,
            b"minMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).min_mem_type42 as i64),
        );
        json_object_push(
            post,
            b"maxMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_mem_type42 as i64),
        );
        json_object_push(
            post,
            b"minMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).min_mem_type1 as i64),
        );
        json_object_push(
            post,
            b"maxMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).max_mem_type1 as i64),
        );
        json_object_push(
            root,
            b"post\0" as *const u8 as *const ::core::ffi::c_char,
            post,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
pub unsafe extern "C" fn otfcc_parse_post(
    mut root: *const ParsedValue,
    mut options: *const Options,
) -> Option<Box<PostTable>> {
    // `.version`'s `0x30000` default carries through if the "post" JSON key
    // is absent (never overwritten below in that case, unlike every other
    // field); `post_name_map` is never touched here regardless, so its
    // zeroed null is already the old `init_post`'s default.
    let mut post_val: PostTable = ::core::mem::zeroed();
    post_val.version = 0x30000 as ::core::ffi::c_int as F16Dot16;
    let mut post_box: Box<PostTable> = Box::new(post_val);
    let post: *mut PostTable = post_box.as_mut() as *mut PostTable;
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get_type(
        root,
        b"post\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if !table.is_null() {
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::bytesbuild!(b"post"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            if (*options).short_post {
                (*post).version = 0x30000 as ::core::ffi::c_int as F16Dot16;
            } else {
                (*post).version = otfcc_to_fixed(json_obj_getnum(
                    table,
                    b"version\0" as *const u8 as *const ::core::ffi::c_char,
                ));
            }
            (*post).italic_angle = otfcc_to_fixed(json_obj_getnum(
                table,
                b"italicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            (*post).underline_position = json_obj_getnum(
                table,
                b"underlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
            ) as i16;
            (*post).underline_thickness = json_obj_getnum(
                table,
                b"underlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            ) as i16;
            (*post).is_fixed_pitch = json_obj_getbool(
                table,
                b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).min_mem_type42 = json_obj_getnum(
                table,
                b"minMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).max_mem_type42 = json_obj_getnum(
                table,
                b"maxMemType42\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).min_mem_type1 = json_obj_getnum(
                table,
                b"minMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            (*post).max_mem_type1 = json_obj_getnum(
                table,
                b"maxMemType1\0" as *const u8 as *const ::core::ffi::c_char,
            ) as u32;
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return Some(post_box);
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_post(
    post: Option<&PostTable>,
    mut glyphorder: *mut GlyphOrder,
    mut _options: *const Options,
) -> *mut Buffer {
    let post = match post {
        Some(p) => p as *const PostTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let mut buf: *mut Buffer = bufnew();
    bufwrite32b(buf, (*post).version as u32);
    bufwrite32b(buf, (*post).italic_angle as u32);
    bufwrite16b(buf, (*post).underline_position as u16);
    bufwrite16b(buf, (*post).underline_thickness as u16);
    bufwrite32b(buf, (*post).is_fixed_pitch);
    bufwrite32b(buf, (*post).min_mem_type42);
    bufwrite32b(buf, (*post).max_mem_type42);
    bufwrite32b(buf, (*post).min_mem_type1);
    bufwrite32b(buf, (*post).max_mem_type1);
    if (*post).version == 0x20000 as F16Dot16 {
        // Walks `by_gid` (ascending gid order), not `by_name`: by the time
        // this runs, `by_name`'s uthash chain had already been sorted by
        // `order_glyphs` (json_reader.rs) into exactly this order and
        // `by_gid` built by walking it -- so this reproduces the original's
        // effective iteration order without depending on `HashMap`'s
        // (unspecified) iteration order the way a literal `by_name` walk
        // would have to.
        bufwrite16b(buf, (*glyphorder).by_gid.len() as u16);
        for (_, &idx) in (*glyphorder).by_gid.iter() {
            let entry = &(&(*glyphorder).entries)[idx];
            bufwrite16b(
                buf,
                (258 as ::core::ffi::c_int + entry.gid as ::core::ffi::c_int) as u16,
            );
        }
        for (_, &idx) in (*glyphorder).by_gid.iter() {
            let entry = &(&(*glyphorder).entries)[idx];
            bufwrite8(buf, entry.name.len() as u8);
            bufwrite_bytes(buf, entry.name.len(), entry.name.as_ptr());
        }
    }
    return buf;
}
