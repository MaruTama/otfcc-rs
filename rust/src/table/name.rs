#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};
use crate::support::parsed_json::{ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getint, json_str_len, json_str_ptr, json_type_of};
use crate::support::binio::{read_16u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer};
use crate::vendor::json::{JsonType};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};
use crate::support::base64::{base64_decode, base64_encode};
use crate::support::buffer::{buffree, bufnew, bufseek, bufwrite16b, bufwrite_buf, bufwrite_bytes};
use crate::support::unicodeconv::{utf16be_to_utf8, utf8toutf16be};
use crate::support::built_json::{BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new_length};

// `Copy` dropped (`name_string` is now `Vec<u8>`, the `sds` sweep's last
// leaf field) -- `Clone` alone is enough, and nothing relied on
// `Vec<NameRecord>: Clone` doing a deep copy (the whole-table `.copy` slot
// was already dead before this conversion, per the earlier note this
// replaces).
#[derive(Clone)]
#[repr(C)]
pub struct NameRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub name_id: u16,
    pub name_string: Vec<u8>,
}
pub type NameTable = Vec<NameRecord>;
pub const COPYRIGHT_LEN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
// Stage 6-4 "Box化": `Font.name` becomes `Option<Vec<NameRecord>>` (not
// `Option<Box<Vec<...>>>` -- `Vec` already owns its own heap buffer).
// `NameRecord` has no raw pointers (`name_string: Vec<u8>` only), so a plain
// `Vec<NameRecord>`'s own `Drop` already frees everything -- no per-element
// dispose helper needed, unlike `SvgAssignment`/`table/svg.rs`.
//
// `table_name_create` (this file's only other `malloc` site) is deleted,
// not converted: its sole caller was `create_font_table`'s `create_table`
// vtable slot, and grepping every `FontElementInterface` field found
// `.create_table` itself is never read anywhere in the crate --
// `create_font_table` and its other callee `table_otl_create` are dead
// for the same reason, deleted alongside it.
unsafe extern "C" fn should_decode_as_utf16(mut record: *const NameRecord) -> bool {
    return (*record).platform_id as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*record).platform_id as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            && (*record).encoding_id as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        || (*record).platform_id as ::core::ffi::c_int == 3 as ::core::ffi::c_int
            && ((*record).encoding_id as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*record).encoding_id as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                || (*record).encoding_id as ::core::ffi::c_int == 10 as ::core::ffi::c_int);
}
unsafe extern "C" fn should_decode_as_bytes(mut record: *const NameRecord) -> bool {
    return (*record).platform_id as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && (*record).encoding_id as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*record).language_id as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_read_name(
    packet: Packet,
    mut options: *const Options,
) -> Option<NameTable> {
    let mut count: u32 = 0;
    let mut string_offset: u32 = 0;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == crate::tag::TAG_NAME {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    if !(length < 6 as u32) {
                        count = read_16u(
                            data.offset(2 as ::core::ffi::c_int as isize) as *const u8
                        ) as u32;
                        string_offset = read_16u(
                            data.offset(4 as ::core::ffi::c_int as isize) as *const u8
                        ) as u32;
                        if !(length
                            < (6 as u32).wrapping_add((12 as u32).wrapping_mul(count)))
                        {
                            let mut name: NameTable = Vec::new();
                            let mut j: u16 = 0 as u16;
                            while (j as u32) < count {
                                let mut record: NameRecord = NameRecord {
                                    platform_id: 0,
                                    encoding_id: 0,
                                    language_id: 0,
                                    name_id: 0,
                                    name_string: Vec::new(),
                                };
                                record.platform_id = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize).offset(
                                        (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *const u8,
                                );
                                record.encoding_id = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                record.language_id = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(4 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                record.name_id = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(6 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                record.name_string = Vec::new();
                                let mut length_0: u16 = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(8 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                let mut offset: u16 = read_16u(
                                    data.offset(6 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (j as ::core::ffi::c_int * 12 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset(10 as ::core::ffi::c_int as isize)
                                        as *const u8,
                                );
                                if should_decode_as_bytes(&raw mut record) {
                                    record.name_string = ::core::slice::from_raw_parts(
                                        data.offset(string_offset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const u8,
                                        length_0 as usize,
                                    ).to_vec();
                                } else if should_decode_as_utf16(&raw mut record) {
                                    record.name_string = utf16be_to_utf8(
                                        data.offset(string_offset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const u8,
                                        length_0 as ::core::ffi::c_int,
                                    );
                                } else {
                                    let mut len: usize = 0 as usize;
                                    let mut buf: *mut u8 = base64_encode(
                                        data.offset(string_offset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const u8,
                                        length_0 as usize,
                                        &raw mut len,
                                    );
                                    record.name_string =
                                        ::core::slice::from_raw_parts(buf as *const u8, len).to_vec();
                                    free(buf as *mut ::core::ffi::c_void);
                                    buf = ::core::ptr::null_mut::<u8>();
                                }
                                name.push(record);
                                j = j.wrapping_add(1);
                            }
                            return Some(name);
                        }
                    }
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::bytesbuild!(b"table 'name' corrupted.\n"),
                    );
                    // No `name` to free here: every path that constructs
                    // one (deep inside the nested guards above) returns
                    // immediately afterward, so this branch is only ever
                    // reached before any allocation happens.
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
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_dump_name(
    name: Option<&NameTable>,
    mut root: *mut BuiltValue,
    mut options: *const Options,
) {
    let name = match name {
        Some(n) => n,
        None => return,
    };
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::bytesbuild!(b"name"),
    );
    let records: &Vec<NameRecord> = name;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _name: *mut BuiltValue = json_array_new(records.len());
        let mut j: u16 = 0 as u16;
        while (j as usize) < records.len() {
            let r: *const NameRecord = &records[j as usize];
            let mut record: *mut BuiltValue = json_object_new(5 as usize);
            json_object_push(
                record,
                b"platformID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).platform_id as i64),
            );
            json_object_push(
                record,
                b"encodingID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).encoding_id as i64),
            );
            json_object_push(
                record,
                b"languageID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).language_id as i64),
            );
            json_object_push(
                record,
                b"nameID\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new((*r).name_id as i64),
            );
            json_object_push(
                record,
                b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                json_string_new_length(
                    (*r).name_string.len() as ::core::ffi::c_uint,
                    (*r).name_string.as_ptr() as *const ::core::ffi::c_char,
                ),
            );
            json_array_push(_name, record);
            j = j.wrapping_add(1);
        }
        json_object_push(
            root,
            b"name\0" as *const u8 as *const ::core::ffi::c_char,
            _name,
        );
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_parse_name(
    mut root: *const ParsedValue,
    mut options: *const Options,
) -> Option<NameTable> {
    let mut name: NameTable = Vec::new();
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get_type(
        root,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if !table.is_null() {
        (*(*options).logger)
            .start_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            crate::bytesbuild!(b"name"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut j: u32 = 0 as u32;
            while j < json_arr_len(table) {
                let mut _record: *const ParsedValue = json_arr_at(table, j as u32);
                if !_record.is_null()
                    && json_type_of(_record) == JsonType::Object
                {
                    if json_obj_get_type(
                        _record,
                        b"platformID\0" as *const u8 as *const ::core::ffi::c_char,
                        JsonType::Integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"Missing or invalid platformID for name entry ",
                                j,
                                b"\n",
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"encodingID\0" as *const u8 as *const ::core::ffi::c_char,
                        JsonType::Integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"Missing or invalid encodingID for name entry ",
                                j,
                                b"\n",
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"languageID\0" as *const u8 as *const ::core::ffi::c_char,
                        JsonType::Integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"Missing or invalid languageID for name entry ",
                                j,
                                b"\n",
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"nameID\0" as *const u8 as *const ::core::ffi::c_char,
                        JsonType::Integer,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"Missing or invalid nameID for name entry ",
                                j,
                                b"\n",
                            ),
                        );
                    } else if json_obj_get_type(
                        _record,
                        b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                        JsonType::String,
                    )
                    .is_null()
                    {
                        (*(*options).logger)
                            .log_sds
                            .expect("non-null function pointer")(
                            (*options).logger as *mut ILogger,
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(b"Missing or invalid name string for name entry ",
                                j,
                                b"\n",
                            ),
                        );
                    } else {
                        let mut record: NameRecord = NameRecord {
                            platform_id: 0,
                            encoding_id: 0,
                            language_id: 0,
                            name_id: 0,
                            name_string: Vec::new(),
                        };
                        record.platform_id = json_obj_getint(
                            _record,
                            b"platformID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as u16;
                        record.encoding_id = json_obj_getint(
                            _record,
                            b"encodingID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as u16;
                        record.language_id = json_obj_getint(
                            _record,
                            b"languageID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as u16;
                        record.name_id = json_obj_getint(
                            _record,
                            b"nameID\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as u16;
                        let mut str: *const ParsedValue = json_obj_get_type(
                            _record,
                            b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                            JsonType::String,
                        );
                        record.name_string = ::core::slice::from_raw_parts(
                            json_str_ptr(str) as *const u8,
                            json_str_len(str) as usize,
                        ).to_vec();
                        name.push(record);
                    }
                }
                j = j.wrapping_add(1);
            }
            name.sort_by(|a, b| {
                a.platform_id
                    .cmp(&b.platform_id)
                    .then(a.encoding_id.cmp(&b.encoding_id))
                    .then(a.language_id.cmp(&b.language_id))
                    .then(a.name_id.cmp(&b.name_id))
            });
            ___loggedstep_v = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger
            );
        }
    }
    return Some(name);
}
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn otfcc_build_name(
    name: Option<&NameTable>,
    mut _options: *const Options,
) -> *mut Buffer {
    let name = match name {
        Some(n) => n,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let records: &Vec<NameRecord> = name;
    let mut buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, records.len() as u16);
    bufwrite16b(buf, 0 as u16);
    let mut strings: *mut Buffer = bufnew();
    let mut j: u16 = 0 as u16;
    while (j as usize) < records.len() {
        let record: *const NameRecord = &records[j as usize];
        bufwrite16b(buf, (*record).platform_id);
        bufwrite16b(buf, (*record).encoding_id);
        bufwrite16b(buf, (*record).language_id);
        bufwrite16b(buf, (*record).name_id);
        let mut cbefore: usize = (*strings).cursor;
        if should_decode_as_utf16(record) {
            let u16: Vec<u8> = utf8toutf16be(&(*record).name_string);
            bufwrite_bytes(strings, u16.len(), u16.as_ptr() as *mut u8);
        } else if should_decode_as_bytes(record) {
            bufwrite_bytes(
                strings,
                (*record).name_string.len(),
                (*record).name_string.as_ptr() as *mut u8,
            );
        } else {
            let mut length: usize = 0;
            let mut decoded: *mut u8 = base64_decode(
                (*record).name_string.as_ptr() as *mut u8,
                (*record).name_string.len(),
                &raw mut length,
            );
            bufwrite_bytes(strings, length, decoded);
            free(decoded as *mut ::core::ffi::c_void);
            decoded = ::core::ptr::null_mut::<u8>();
        }
        let mut cafter: usize = (*strings).cursor;
        bufwrite16b(buf, cafter.wrapping_sub(cbefore) as u16);
        bufwrite16b(buf, cbefore as u16);
        j = j.wrapping_add(1);
    }
    let mut copyright: Vec<u8> = crate::bytesbuild!(
        b"-- By OTFCC ",
        MAIN_VER,
        b".",
        SECONDARY_VER,
        b".",
        PATCH_VER,
        b" --",
    );
    // The C original's `sdsgrowzero` re-grow-in-place had a use-after-free
    // latent in it (`name.c:188` drops the reallocated result -- see the
    // history of this comment in git blame if curious); `Vec::resize`
    // has no such hazard to begin with, so there is nothing to preserve.
    copyright.resize(COPYRIGHT_LEN as usize, 0);
    bufwrite_bytes(strings, COPYRIGHT_LEN as usize, copyright.as_ptr());
    let mut strings_offset: usize = (*buf).cursor;
    bufwrite_buf(buf, strings);
    bufseek(buf, 4 as usize);
    bufwrite16b(buf, strings_offset as u16);
    buffree(strings);
    return buf;
}
