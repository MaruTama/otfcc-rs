#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::free;

use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::logger::{logger_finish, logger_start_sds};
use crate::support::alloc::__caryll_allocate_clean;
use crate::support::binio::{read_8u, read_16u, read_32u};
use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getint,
    json_obj_getint_fallback, json_type_of,
};
use crate::support::primitives::{ColorId, FontFilePointer, TableId};
use crate::vendor::json::JsonType;

use crate::bk::bkgraph::bk_build_block;
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new,
    json_object_push, preserialize,
};
#[derive(Copy, Clone)]
pub struct CpalColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    pub label: u16,
}
#[derive(Clone)]
pub struct CpalPalette {
    pub colorset: Vec<CpalColor>,
    pub type_0: u32,
    pub label: u32,
}
// Stage 6-4 "Box化": every field this struct owns is already a
// `Vec`/scalar, so no `Drop` impl is needed -- `Box::new` construction
// plus the standard drop glue is sufficient. `init_cpal`/`dispose_cpal`/
// `table_cpal_{init,dispose,create,copy,free}` all deleted: grepping
// confirmed `table_cpal_copy` was never called anywhere (not even
// self-referentially), and `table_cpal_free` was the only one of these
// ever called from outside this file (from `caryll_font.rs`'s table
// disposal).
#[derive(Clone)]
pub struct CpalTable {
    pub version: u16,
    pub palettes: Vec<CpalPalette>,
}
pub static WHITE: CpalColor = CpalColor {
    red: 0xff as u8,
    green: 0xff as u8,
    blue: 0xff as u8,
    alpha: 0xff as u8,
    label: 0xffff as u16,
};
pub unsafe fn otfcc_read_cpal(packet: &Packet) -> Option<Box<CpalTable>> {
    let mut version: u16;
    let mut table_header_length: u32;
    let mut num_palettes_entries: u16;
    let mut num_palettes: u16;
    let mut num_color_records: u16;
    let mut offset_first_color_record: u32;
    let mut color_list: *mut CpalColor;
    let mut t: Option<Box<CpalTable>>;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let table: &PacketPiece = &packet.pieces[__fortable_count as usize];
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_CPAL {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let data: FontFilePointer = table.data.as_ptr() as FontFilePointer;
                    let length: u32 = table.length;
                    if !(length < 2 as u32) {
                        t = Some(Box::new(CpalTable {
                            version: 0,
                            palettes: Vec::new(),
                        }));
                        version = read_16u(data as *const u8);
                        table_header_length =
                            (if version as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                14 as ::core::ffi::c_int
                            } else {
                                26 as ::core::ffi::c_int
                            }) as u32;
                        if !(length < table_header_length) {
                            t.as_mut().unwrap().version = version;
                            num_palettes_entries = read_16u(
                                data.offset(2 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            num_palettes = read_16u(
                                data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                            );
                            num_color_records = read_16u(
                                data.offset(6 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            offset_first_color_record = read_32u(
                                data.offset(8 as ::core::ffi::c_int as isize) as *const u8,
                            );
                            if !(length
                                < offset_first_color_record.wrapping_add(
                                    (num_color_records as ::core::ffi::c_int
                                        * 4 as ::core::ffi::c_int)
                                        as u32,
                                ))
                            {
                                if !(length
                                    < table_header_length.wrapping_add(
                                        (2 as ::core::ffi::c_int
                                            * num_palettes as ::core::ffi::c_int)
                                            as u32,
                                    ))
                                {
                                    color_list = ::core::ptr::null_mut::<CpalColor>();
                                    color_list = __caryll_allocate_clean(
                                        (::core::mem::size_of::<CpalColor>() as usize)
                                            .wrapping_mul(num_color_records as usize),
                                        55 as ::core::ffi::c_ulong,
                                    )
                                        as *mut CpalColor;
                                    let mut j: u16 = 0 as u16;
                                    while (j as ::core::ffi::c_int)
                                        < num_color_records as ::core::ffi::c_int
                                    {
                                        *color_list.offset(j as isize) = CpalColor {
                                            red: read_8u(
                                                data.offset(offset_first_color_record as isize)
                                                    .offset(
                                                        (j as ::core::ffi::c_int
                                                            * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    .offset(2 as ::core::ffi::c_int as isize)
                                                    as *const u8,
                                            ),
                                            green: read_8u(
                                                data.offset(offset_first_color_record as isize)
                                                    .offset(
                                                        (j as ::core::ffi::c_int
                                                            * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    .offset(1 as ::core::ffi::c_int as isize)
                                                    as *const u8,
                                            ),
                                            blue: read_8u(
                                                data.offset(offset_first_color_record as isize)
                                                    .offset(
                                                        (j as ::core::ffi::c_int
                                                            * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    as *const u8,
                                            ),
                                            alpha: read_8u(
                                                data.offset(offset_first_color_record as isize)
                                                    .offset(
                                                        (j as ::core::ffi::c_int
                                                            * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    .offset(3 as ::core::ffi::c_int as isize)
                                                    as *const u8,
                                            ),
                                            label: 0xffff as u16,
                                        };
                                        j = j.wrapping_add(1);
                                    }
                                    let mut j_0: TableId = 0 as TableId;
                                    while (j_0 as ::core::ffi::c_int)
                                        < num_palettes as ::core::ffi::c_int
                                    {
                                        // `label: 0xffff`, not `0` -- matches
                                        // what the deleted `CPAL_I_PALETTE.init`
                                        // call used to leave here (nothing
                                        // overwrites `.label` afterward in
                                        // this function, unlike `.type_0`
                                        // and `.colorset`, which init also
                                        // touched but every caller re-sets).
                                        let mut palette: CpalPalette = CpalPalette {
                                            colorset: Vec::new(),
                                            type_0: 0,
                                            label: 0xffff,
                                        };
                                        let palette_start_index: TableId = read_16u(
                                            data.offset(12 as ::core::ffi::c_int as isize).offset(
                                                (j_0 as ::core::ffi::c_int
                                                    * 2 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as *const u8,
                                        )
                                            as TableId;
                                        let mut j_1: ColorId = 0 as ColorId;
                                        while (j_1 as ::core::ffi::c_int)
                                            < num_palettes_entries as ::core::ffi::c_int
                                        {
                                            if (palette_start_index as ::core::ffi::c_int
                                                + j_1 as ::core::ffi::c_int)
                                                < num_color_records as ::core::ffi::c_int
                                            {
                                                palette.colorset.push(*color_list.offset(
                                                    (j_1 as ::core::ffi::c_int
                                                        + palette_start_index as ::core::ffi::c_int)
                                                        as isize,
                                                ));
                                            } else {
                                                palette.colorset.push(WHITE);
                                            }
                                            j_1 = j_1.wrapping_add(1);
                                        }
                                        t.as_mut().unwrap().palettes.push(palette);
                                        j_0 = j_0.wrapping_add(1);
                                    }
                                    if version as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                        let palettes: &mut Vec<CpalPalette> =
                                            &mut t.as_mut().unwrap().palettes;
                                        let offset_palette_type_array: u32 = read_32u(
                                            data.offset(16 as ::core::ffi::c_int as isize).offset(
                                                (2 as ::core::ffi::c_int
                                                    * num_palettes as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as *const u8,
                                        );
                                        if offset_palette_type_array != 0
                                            && length
                                                >= offset_palette_type_array.wrapping_add(
                                                    (4 as ::core::ffi::c_int
                                                        * num_palettes as ::core::ffi::c_int)
                                                        as u32,
                                                )
                                        {
                                            let mut j_2: TableId = 0 as TableId;
                                            while (j_2 as ::core::ffi::c_int)
                                                < num_palettes as ::core::ffi::c_int
                                            {
                                                let type_0: u32 = read_32u(
                                                    data.offset(
                                                        (j_2 as ::core::ffi::c_int
                                                            * 4 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    .offset(offset_palette_type_array as isize)
                                                        as *const u8,
                                                );
                                                palettes[j_2 as usize].type_0 = type_0;
                                                j_2 = j_2.wrapping_add(1);
                                            }
                                        }
                                        let offset_palette_label_array: u32 = read_32u(
                                            data.offset(20 as ::core::ffi::c_int as isize).offset(
                                                (2 as ::core::ffi::c_int
                                                    * num_palettes as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as *const u8,
                                        );
                                        if offset_palette_label_array != 0
                                            && length
                                                >= offset_palette_label_array.wrapping_add(
                                                    (2 as ::core::ffi::c_int
                                                        * num_palettes as ::core::ffi::c_int)
                                                        as u32,
                                                )
                                        {
                                            let mut j_3: TableId = 0 as TableId;
                                            while (j_3 as ::core::ffi::c_int)
                                                < num_palettes as ::core::ffi::c_int
                                            {
                                                let label: u16 = read_16u(
                                                    data.offset(
                                                        (j_3 as ::core::ffi::c_int
                                                            * 2 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    .offset(offset_palette_label_array as isize)
                                                        as *const u8,
                                                );
                                                palettes[j_3 as usize].label = label as u32;
                                                j_3 = j_3.wrapping_add(1);
                                            }
                                        }
                                        if version as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                            let offset_palette_entry_label_array: u32 =
                                                read_32u(
                                                    data.offset(24 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (2 as ::core::ffi::c_int
                                                                * num_palettes
                                                                    as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        as *const u8,
                                                );
                                            if offset_palette_entry_label_array != 0
                                                && length
                                                    >= offset_palette_entry_label_array
                                                        .wrapping_add(
                                                            (4 as ::core::ffi::c_int
                                                                * num_palettes_entries
                                                                    as ::core::ffi::c_int)
                                                                as u32,
                                                        )
                                            {
                                                let mut j_4: ColorId = 0 as ColorId;
                                                while (j_4 as ::core::ffi::c_int)
                                                    < num_palettes_entries as ::core::ffi::c_int
                                                {
                                                    let label_0: u16 = read_16u(
                                                        data.offset(
                                                            (j_4 as ::core::ffi::c_int
                                                                * 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        .offset(
                                                            offset_palette_entry_label_array
                                                                as isize,
                                                        )
                                                            as *const u8,
                                                    );
                                                    let mut k: TableId = 0 as TableId;
                                                    while (k as ::core::ffi::c_int)
                                                        < num_palettes as ::core::ffi::c_int
                                                    {
                                                        palettes[k as usize].colorset
                                                            [j_4 as usize]
                                                            .label = label_0;
                                                        k = k.wrapping_add(1);
                                                    }
                                                    j_4 = j_4.wrapping_add(1);
                                                }
                                            }
                                        }
                                    }
                                    free(color_list as *mut ::core::ffi::c_void);
                                    color_list = ::core::ptr::null_mut::<CpalColor>();
                                    return t;
                                }
                            }
                        }
                    }
                    t = None;
                    __fortable_k2 = 0 as ::core::ffi::c_int;
                    __notfound = 0 as ::core::ffi::c_int;
                }
            }
            __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        }
        __fortable_keep = (__fortable_keep == 0) as ::core::ffi::c_int;
        __fortable_count += 1;
    }
    return None;
}
#[inline]
unsafe fn dump_color(color: *const CpalColor) -> *mut BuiltValue {
    let mut _color: *mut BuiltValue = json_object_new(5 as usize);
    json_object_push(
        _color,
        b"red\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*color).red as i64),
    );
    json_object_push(
        _color,
        b"green\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*color).green as i64),
    );
    json_object_push(
        _color,
        b"blue\0" as *const u8 as *const ::core::ffi::c_char,
        json_integer_new((*color).blue as i64),
    );
    if (*color).alpha as ::core::ffi::c_int != 0xff as ::core::ffi::c_int {
        json_object_push(
            _color,
            b"alpha\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*color).alpha as i64),
        );
    }
    if (*color).label as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
        json_object_push(
            _color,
            b"label\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*color).label as i64),
        );
    }
    return preserialize(_color);
}
#[inline]
unsafe fn dump_palette(palette: *const CpalPalette) -> *mut BuiltValue {
    let mut _palette: *mut BuiltValue = json_object_new(3 as usize);
    if (*palette).type_0 != 0 {
        json_object_push(
            _palette,
            b"type\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*palette).type_0 as i64),
        );
    }
    if (*palette).label != 0xffff as u32 {
        json_object_push(
            _palette,
            b"label\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*palette).label as i64),
        );
    }
    let colorset: &Vec<CpalColor> = &(*palette).colorset;
    let a: *mut BuiltValue = json_array_new(colorset.len());
    let mut j: ColorId = 0 as ColorId;
    while (j as usize) < colorset.len() {
        json_array_push(a, dump_color(&colorset[j as usize] as *const CpalColor));
        j = j.wrapping_add(1);
    }
    json_object_push(
        _palette,
        b"colors\0" as *const u8 as *const ::core::ffi::c_char,
        a,
    );
    return _palette;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_cpal(
    table: Option<&CpalTable>,
    root: *mut BuiltValue,
    options: &Options,
) {
    let table = match table {
        Some(t) => t,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"CPAL"),
    );
    let palettes: &Vec<CpalPalette> = &(*table).palettes;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _t: *mut BuiltValue = json_object_new(2 as usize);
        json_object_push(
            _t,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
            json_integer_new((*table).version as i64),
        );
        let mut _a: *mut BuiltValue = json_array_new(palettes.len());
        let mut j: TableId = 0 as TableId;
        while (j as usize) < palettes.len() {
            json_array_push(
                _a,
                dump_palette(&palettes[j as usize] as *const CpalPalette),
            );
            j = j.wrapping_add(1);
        }
        json_object_push(
            _t,
            b"palettes\0" as *const u8 as *const ::core::ffi::c_char,
            _a,
        );
        json_object_push(
            root,
            b"CPAL\0" as *const u8 as *const ::core::ffi::c_char,
            _t,
        );
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
#[inline]
unsafe fn parse_color(mut _color: *const ParsedValue) -> CpalColor {
    let mut color: CpalColor = WHITE;
    if _color.is_null() || json_type_of(_color) != JsonType::Object {
        return color;
    }
    color.red = json_obj_getint_fallback(
        _color,
        b"red\0" as *const u8 as *const ::core::ffi::c_char,
        0 as i32,
    ) as u8;
    color.green = json_obj_getint_fallback(
        _color,
        b"green\0" as *const u8 as *const ::core::ffi::c_char,
        0 as i32,
    ) as u8;
    color.blue = json_obj_getint_fallback(
        _color,
        b"blue\0" as *const u8 as *const ::core::ffi::c_char,
        0 as i32,
    ) as u8;
    color.alpha = json_obj_getint_fallback(
        _color,
        b"alpha\0" as *const u8 as *const ::core::ffi::c_char,
        0xff as i32,
    ) as u8;
    color.label = json_obj_getint_fallback(
        _color,
        b"label\0" as *const u8 as *const ::core::ffi::c_char,
        0xffff as i32,
    ) as u16;
    return color;
}
pub unsafe fn otfcc_parse_cpal(
    root: *const ParsedValue,
    options: &Options,
) -> Option<Box<CpalTable>> {
    let table: *const ParsedValue;
    table = json_obj_get_type(
        root,
        b"CPAL\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Object,
    );
    if table.is_null() {
        return None;
    }
    let mut cpal: Option<Box<CpalTable>> = None;
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"CPAL"),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _palettes: *const ParsedValue = json_obj_get_type(
            table,
            b"palettes\0" as *const u8 as *const ::core::ffi::c_char,
            JsonType::Array,
        );
        if _palettes.is_null() || json_arr_len(_palettes) == 0 {
            return None;
        }
        let version = json_obj_getint(
            table,
            b"version\0" as *const u8 as *const ::core::ffi::c_char,
        ) as u16;
        cpal = Some(Box::new(CpalTable {
            version,
            palettes: Vec::new(),
        }));
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_uint) < json_arr_len(_palettes) {
            let mut _palette: *const ParsedValue = json_arr_at(_palettes, j as u32);
            if !(_palette.is_null() || json_type_of(_palette) != JsonType::Object) {
                let mut _colors: *const ParsedValue = json_obj_get_type(
                    _palette,
                    b"colors\0" as *const u8 as *const ::core::ffi::c_char,
                    JsonType::Array,
                );
                if !_colors.is_null() {
                    let mut palette: CpalPalette = CpalPalette {
                        colorset: Vec::new(),
                        type_0: 0,
                        label: 0,
                    };
                    palette.type_0 = json_obj_getint(
                        _palette,
                        b"type\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as u32;
                    palette.label = json_obj_getint_fallback(
                        _palette,
                        b"type\0" as *const u8 as *const ::core::ffi::c_char,
                        0xffff as i32,
                    ) as u32;
                    let mut k: ColorId = 0 as ColorId;
                    while (k as ::core::ffi::c_uint) < json_arr_len(_colors) {
                        palette
                            .colorset
                            .push(parse_color(json_arr_at(_colors, k as u32)));
                        k = k.wrapping_add(1);
                    }
                    cpal.as_mut().unwrap().palettes.push(palette);
                }
            }
            j = j.wrapping_add(1);
        }
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    return cpal;
}
#[inline]
unsafe fn build_palette_type(cpal: *const CpalTable) -> *mut BkBlock {
    let palettes: &Vec<CpalPalette> = &(*cpal).palettes;
    let mut needs_palette_type: bool = false;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < palettes.len() {
        if palettes[j as usize].type_0 != 0 {
            needs_palette_type = true;
        }
        j = j.wrapping_add(1);
    }
    if !needs_palette_type {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let block: *mut BkBlock = bk_new_block(&[]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < palettes.len() {
        bk_push(
            block,
            &[bk_int(
                BkCellType::B32,
                (palettes[j_0 as usize].type_0) as u32,
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[inline]
unsafe fn build_palette_label(cpal: *const CpalTable) -> *mut BkBlock {
    let palettes: &Vec<CpalPalette> = &(*cpal).palettes;
    let mut needs_palette_label: bool = false;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < palettes.len() {
        if palettes[j as usize].label != 0xffff as u32 {
            needs_palette_label = true;
        }
        j = j.wrapping_add(1);
    }
    if !needs_palette_label {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let block: *mut BkBlock = bk_new_block(&[]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < palettes.len() {
        bk_push(
            block,
            &[bk_int(
                BkCellType::B16,
                (palettes[j_0 as usize].label) as u32,
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[inline]
unsafe fn build_palette_entry_label(cpal: *const CpalTable) -> *mut BkBlock {
    let palettes: &Vec<CpalPalette> = &(*cpal).palettes;
    let mut needs_palette_entry_label: bool = false;
    let palette: &CpalPalette = &palettes[0 as usize];
    let mut j: ColorId = 0 as ColorId;
    while (j as usize) < palette.colorset.len() {
        if palette.colorset[j as usize].label as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int
        {
            needs_palette_entry_label = true;
        }
        j = j.wrapping_add(1);
    }
    if !needs_palette_entry_label {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let block: *mut BkBlock = bk_new_block(&[]);
    let mut j_0: ColorId = 0 as ColorId;
    while (j_0 as usize) < palette.colorset.len() {
        bk_push(
            block,
            &[bk_int(
                BkCellType::B16,
                (palette.colorset[j_0 as usize].label as ::core::ffi::c_int) as u32,
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    return block;
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_cpal(cpal: Option<&CpalTable>) -> *mut Buffer {
    let cpal = match cpal {
        Some(c) => c as *const CpalTable,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let palettes: &Vec<CpalPalette> = &(*cpal).palettes;
    if palettes.is_empty() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let num_palettes: u16 = palettes.len() as u16;
    let num_palettes_entries: u16 = palettes[0 as usize].colorset.len() as u16;
    let num_color_records: u16 =
        (num_palettes as ::core::ffi::c_int * num_palettes_entries as ::core::ffi::c_int) as u16;
    let color_records: *mut BkBlock = bk_new_block(&[]);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < num_palettes as ::core::ffi::c_int {
        let palette: &CpalPalette = &palettes[j as usize];
        let total_colors: ColorId = palette.colorset.len() as ColorId;
        let mut k: ColorId = 0 as ColorId;
        while (k as ::core::ffi::c_int) < num_palettes_entries as ::core::ffi::c_int {
            let color: *const CpalColor;
            if (k as ::core::ffi::c_int) < total_colors as ::core::ffi::c_int {
                color = &palette.colorset[k as usize] as *const CpalColor;
            } else {
                color = &raw const WHITE;
            }
            bk_push(
                color_records,
                &[
                    bk_int(BkCellType::B8, ((*color).blue as ::core::ffi::c_int) as u32),
                    bk_int(
                        BkCellType::B8,
                        ((*color).green as ::core::ffi::c_int) as u32,
                    ),
                    bk_int(BkCellType::B8, ((*color).red as ::core::ffi::c_int) as u32),
                    bk_int(
                        BkCellType::B8,
                        ((*color).alpha as ::core::ffi::c_int) as u32,
                    ),
                ],
            );
            k = k.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_int(
            BkCellType::B16,
            ((*cpal).version as ::core::ffi::c_int) as u32,
        ),
        bk_int(
            BkCellType::B16,
            (num_palettes_entries as ::core::ffi::c_int) as u32,
        ),
        bk_int(BkCellType::B16, (num_palettes as ::core::ffi::c_int) as u32),
        bk_int(
            BkCellType::B16,
            (num_color_records as ::core::ffi::c_int) as u32,
        ),
        bk_ptr(BkCellType::P32, color_records),
    ]);
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as ::core::ffi::c_int) < num_palettes as ::core::ffi::c_int {
        bk_push(
            root,
            &[bk_int(
                BkCellType::B16,
                (num_palettes_entries as ::core::ffi::c_int * j_0 as ::core::ffi::c_int) as u32,
            )],
        );
        j_0 = j_0.wrapping_add(1);
    }
    if (*cpal).version as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        bk_push(
            root,
            &[
                bk_ptr(BkCellType::P32, build_palette_type(cpal)),
                bk_ptr(BkCellType::P32, build_palette_label(cpal)),
                bk_ptr(BkCellType::P32, build_palette_entry_label(cpal)),
            ],
        );
    }
    return bk_build_block(root);
}
