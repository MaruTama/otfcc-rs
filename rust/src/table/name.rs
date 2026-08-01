#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};
use crate::support::json_funcs::{json_obj_get_type, json_obj_getint};
use crate::support::binio::{read_16u};
use crate::logger::{LoggerType, LOG_VL_IMPORTANT, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::json::{JsonType, JsonValue};
use crate::font::caryll_sfnt::{Packet, PacketPiece};
use crate::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};
use crate::support::base64::{base64_decode, base64_encode};
use crate::support::buffer::{buffree, bufnew, bufseek, bufwrite16b, bufwrite_buf, bufwrite_bytes};
use crate::support::unicodeconv::{utf16be_to_utf8, utf8toutf16be};
use crate::vendor::json_builder::{json_array_new, json_array_push, json_integer_new, json_object_new, json_object_push, json_string_new_length};
use crate::vendor::sds::{sdsempty, sdsfree, sdsgrowzero, sdslen, sdsnewlen};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameRecord {
    pub platform_id: u16,
    pub encoding_id: u16,
    pub language_id: u16,
    pub name_id: u16,
    pub name_string: SdsRaw,
}
pub type NameTable = Vec<NameRecord>;
// `NameRecord` stays `Copy` (like `MetaEntry` -- an owned `sds` field doesn't
// block it, only a `Handle` embedding does, and even then only because of the
// explicit-dup convention). Safe here specifically because `TABLE_I_NAME.copy`
// (whole-table clone) was dead before this conversion and is deleted outright
// below, not ported -- nothing ever relies on `Vec<NameRecord>: Clone` doing a
// deep copy.
unsafe fn dispose_name_record(r: *mut NameRecord) {
    sdsfree((*r).name_string);
    (*r).name_string = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
pub const COPYRIGHT_LEN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
// `table_name_dispose`'s job: free every record's `name_string` before the
// backing `Vec` itself is dropped/reset (`NameRecord` owns `name_string` but
// isn't wrapped in a `Drop` impl -- same convention as `MetaEntry`/
// `CaretValueRecord`).
unsafe fn table_name_dispose(arr: *mut NameTable) {
    if arr.is_null() {
        return;
    }
    for r in (*arr).iter_mut() {
        dispose_name_record(r as *mut NameRecord);
    }
    (*arr).clear();
}
pub(crate) unsafe fn table_name_free(x: *mut NameTable) {
    if x.is_null() {
        return;
    }
    table_name_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
pub(crate) unsafe fn table_name_create() -> *mut NameTable {
    // `.write()`, not a field assignment: `NameTable` is directly `Vec<T>`
    // (no wrapper struct), so this placement-constructs the whole value and
    // never reads whatever `malloc` left behind -- same reasoning as
    // `ColrTable`/`MaskList`'s `table_*_create`.
    let x: *mut NameTable = malloc(::core::mem::size_of::<NameTable>() as usize) as *mut NameTable;
    x.write(Vec::new());
    x
}
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
pub unsafe extern "C" fn otfcc_read_name(
    packet: Packet,
    mut options: *const Options,
) -> *mut NameTable {
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
            if table.tag == 1851878757i32 as u32 {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut name: *mut NameTable = ::core::ptr::null_mut::<NameTable>();
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
                            name = table_name_create();
                            let mut j: u16 = 0 as u16;
                            while (j as u32) < count {
                                let mut record: NameRecord = NameRecord {
                                    platform_id: 0,
                                    encoding_id: 0,
                                    language_id: 0,
                                    name_id: 0,
                                    name_string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
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
                                record.name_string = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
                                    let mut name_string: SdsRaw = sdsnewlen(
                                        data.offset(string_offset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const ::core::ffi::c_void,
                                        length_0 as usize,
                                    );
                                    record.name_string = name_string;
                                } else if should_decode_as_utf16(&raw mut record) {
                                    let mut name_string_0: SdsRaw = utf16be_to_utf8(
                                        data.offset(string_offset as isize)
                                            .offset(offset as ::core::ffi::c_int as isize)
                                            as *const u8,
                                        length_0 as ::core::ffi::c_int,
                                    );
                                    record.name_string = name_string_0;
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
                                        sdsnewlen(buf as *const ::core::ffi::c_void, len);
                                    free(buf as *mut ::core::ffi::c_void);
                                    buf = ::core::ptr::null_mut::<u8>();
                                }
                                (*name).push(record);
                                j = j.wrapping_add(1);
                            }
                            return name;
                        }
                    }
                    (*(*options).logger)
                        .log_sds
                        .expect("non-null function pointer")(
                        (*options).logger as *mut ILogger,
                        LOG_VL_IMPORTANT,
                        LoggerType::Warning,
                        crate::sdsbuild!(sdsempty(), b"table 'name' corrupted.\n"),
                    );
                    if !name.is_null() {
                        table_name_free(name);
                        name = ::core::ptr::null_mut::<NameTable>();
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
    return ::core::ptr::null_mut::<NameTable>();
}
pub unsafe extern "C" fn otfcc_dump_name(
    mut name: *const NameTable,
    mut root: *mut JsonValue,
    mut options: *const Options,
) {
    if name.is_null() {
        return;
    }
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), b"name"),
    );
    let records: &Vec<NameRecord> = &*name;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _name: *mut JsonValue = json_array_new(records.len());
        let mut j: u16 = 0 as u16;
        while (j as usize) < records.len() {
            let r: *const NameRecord = &records[j as usize];
            let mut record: *mut JsonValue = json_object_new(5 as usize);
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
                    sdslen((*r).name_string) as ::core::ffi::c_uint,
                    (*r).name_string as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn otfcc_parse_name(
    mut root: *const JsonValue,
    mut options: *const Options,
) -> *mut NameTable {
    let mut name: *mut NameTable = table_name_create();
    let mut table: *mut JsonValue = ::core::ptr::null_mut::<JsonValue>();
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
            crate::sdsbuild!(sdsempty(), b"name"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut j: u32 = 0 as u32;
            while j < (*table).u.array.length as u32 {
                if !(*(*table).u.array.values.offset(j as isize)).is_null()
                    && (**(*table).u.array.values.offset(j as isize)).type_0 == JsonType::Object
                {
                    let mut _record: *mut JsonValue =
                        *(*table).u.array.values.offset(j as isize) as *mut JsonValue;
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
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Missing or invalid platformID for name entry ",
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
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Missing or invalid encodingID for name entry ",
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
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Missing or invalid languageID for name entry ",
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
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Missing or invalid nameID for name entry ",
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
                            crate::sdsbuild!(
                                sdsempty(),
                                b"Missing or invalid name string for name entry ",
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
                            name_string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
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
                        let mut str: *mut JsonValue = json_obj_get_type(
                            _record,
                            b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                            JsonType::String,
                        );
                        record.name_string = sdsnewlen(
                            (*str).u.string.ptr as *const ::core::ffi::c_void,
                            (*str).u.string.length as usize,
                        );
                        (*name).push(record);
                    }
                }
                j = j.wrapping_add(1);
            }
            (*name).sort_by(|a, b| {
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
    return name;
}
pub unsafe extern "C" fn otfcc_build_name(
    mut name: *const NameTable,
    mut _options: *const Options,
) -> *mut Buffer {
    if name.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let records: &Vec<NameRecord> = &*name;
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
            let mut words: usize = 0;
            let mut u16: *mut u8 = utf8toutf16be((*record).name_string, &raw mut words);
            bufwrite_bytes(strings, words, u16);
            free(u16 as *mut ::core::ffi::c_void);
            u16 = ::core::ptr::null_mut::<u8>();
        } else if should_decode_as_bytes(record) {
            bufwrite_bytes(
                strings,
                sdslen((*record).name_string),
                (*record).name_string as *mut u8,
            );
        } else {
            let mut length: usize = 0;
            let mut decoded: *mut u8 = base64_decode(
                (*record).name_string as *mut u8,
                sdslen((*record).name_string),
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
    let mut copyright: SdsRaw = crate::sdsbuild!(
        sdsempty(),
        b"-- By OTFCC ",
        MAIN_VER,
        b".",
        SECONDARY_VER,
        b".",
        PATCH_VER,
        b" --",
    );
    // `sdsgrowzero` may reallocate, so its result has to be assigned back.
    // `name.c:188` drops it -- a use-after-free that has never fired only
    // because `sdscatprintf` happened to over-allocate: it grew the buffer to
    // twice the 21-byte version string, and 42 bytes is (just) enough for the
    // 32 this then asks for. Appending the string in pieces allocates 24, so
    // the growth reallocates, and the stale pointer aborts in `sdsfree`.
    copyright = sdsgrowzero(copyright, COPYRIGHT_LEN as usize);
    bufwrite_bytes(strings, COPYRIGHT_LEN as usize, copyright as *mut u8);
    sdsfree(copyright);
    let mut strings_offset: usize = (*buf).cursor;
    bufwrite_buf(buf, strings);
    bufseek(buf, 4 as usize);
    bufwrite16b(buf, strings_offset as u16);
    buffree(strings);
    return buf;
}
