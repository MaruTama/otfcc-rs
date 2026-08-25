#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::font::caryll_sfnt::Packet;
use crate::logger::{
    LOG_VL_IMPORTANT, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::base64::{base64_decode, base64_encode};
use crate::support::buffer::Buffer;
use crate::support::buffer::{buffree, bufnew, bufseek, bufwrite_buf, bufwrite_bytes, bufwrite16b};
use crate::support::built_json::{
    BuiltValue, json_array_new, json_array_push, json_integer_new, json_object_new,
    json_object_push, json_string_new_length,
};
use crate::support::font_reader::{FontReader, ReadError};
use crate::support::options::Options;
use crate::support::parsed_json::{
    ParsedValue, json_arr_at, json_arr_len, json_obj_get_type, json_obj_getint, json_str_len,
    json_str_ptr, json_type_of,
};
use crate::support::unicodeconv::{utf8toutf16be, utf16be_to_utf8};
use crate::vendor::json::JsonType;
use crate::version::{MAIN_VER, PATCH_VER, SECONDARY_VER};
use libc::free;

// `Copy` dropped (`name_string` is now `Vec<u8>`, the `sds` sweep's last
// leaf field) -- `Clone` alone is enough, and nothing relied on
// `Vec<NameRecord>: Clone` doing a deep copy (the whole-table `.copy` slot
// was already dead before this conversion, per the earlier note this
// replaces).
#[derive(Clone)]
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
unsafe fn should_decode_as_utf16(record: *const NameRecord) -> bool {
    return (*record).platform_id as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*record).platform_id as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            && (*record).encoding_id as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        || (*record).platform_id as ::core::ffi::c_int == 3 as ::core::ffi::c_int
            && ((*record).encoding_id as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*record).encoding_id as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                || (*record).encoding_id as ::core::ffi::c_int == 10 as ::core::ffi::c_int);
}
unsafe fn should_decode_as_bytes(record: *const NameRecord) -> bool {
    return (*record).platform_id as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && (*record).encoding_id as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*record).language_id as ::core::ffi::c_int == 0 as ::core::ffi::c_int;
}
// The record *array* (12 bytes/record starting at offset 6) was already
// guarded (`length < 6 + 12 * count`) -- but each record's *string*, read
// from `string_offset + offset` for `length_0` bytes, was not: nothing
// checked that span against the table's real length before handing it to
// `slice::from_raw_parts`/`utf16be_to_utf8`/`base64_encode`, any of which
// could read arbitrarily far past the table depending on the record's
// declared platform/encoding. Resolved through `FontReader::at` +
// `peek_bytes` before any of those three read the bytes; a record whose
// string span doesn't fit keeps its `platform_id`/`encoding_id`/
// `language_id`/`name_id` (still meaningful metadata) but gets an empty
// `name_string` instead of the out-of-bounds read -- the same "keep the
// record, drop only what doesn't fit" choice `table/post.rs::parse_post`
// made for an out-of-range `glyphNameIndex`.
unsafe fn parse_name(data: &[u8]) -> Result<NameTable, ReadError> {
    let mut header = FontReader::new(data);
    header.skip(2)?; // format, unused
    let count = header.u16()? as u32;
    let string_offset = header.u16()? as u32;
    // The record array itself must fit -- corresponds to the original's
    // `length < 6 + 12 * count` guard, now via `checked_mul`/`checked_add`
    // rather than `wrapping_add`/`wrapping_mul` (so an overflowing `count`
    // fails the check instead of wrapping past it).
    FontReader::new(data)
        .at(6)?
        .require_room(count as usize, 12)?;

    let mut name: NameTable = Vec::with_capacity(count as usize);
    for j in 0..count {
        let mut r = FontReader::new(data).at(6usize + (j as usize) * 12)?;
        let mut record = NameRecord {
            platform_id: r.u16()?,
            encoding_id: r.u16()?,
            language_id: r.u16()?,
            name_id: r.u16()?,
            name_string: Vec::new(),
        };
        let length_0 = r.u16()?;
        let offset = r.u16()?;

        let string_bytes = (string_offset as usize)
            .checked_add(offset as usize)
            .and_then(|start| FontReader::new(data).at(start).ok())
            .and_then(|sr| sr.peek_bytes(length_0 as usize).ok());
        if let Some(bytes) = string_bytes {
            if should_decode_as_bytes(&raw const record) {
                record.name_string = bytes.to_vec();
            } else if should_decode_as_utf16(&raw const record) {
                record.name_string =
                    utf16be_to_utf8(bytes.as_ptr(), bytes.len() as ::core::ffi::c_int);
            } else {
                let mut len: usize = 0 as usize;
                let mut buf: *mut u8 = base64_encode(bytes.as_ptr(), bytes.len(), &raw mut len);
                record.name_string = ::core::slice::from_raw_parts(buf as *const u8, len).to_vec();
                free(buf as *mut ::core::ffi::c_void);
                buf = ::core::ptr::null_mut::<u8>();
            }
        }
        name.push(record);
    }
    Ok(name)
}

#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_read_name(packet: &Packet, options: &Options) -> Option<NameTable> {
    let table = packet
        .pieces
        .iter()
        .find(|p| p.tag == crate::tag::TAG_NAME)?;
    match parse_name(&table.data) {
        Ok(name) => Some(name),
        Err(_) => {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_IMPORTANT,
                LoggerType::Warning,
                crate::bytesbuild!(b"table 'name' corrupted.\n"),
            );
            None
        }
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_dump_name(
    name: Option<&NameTable>,
    root: *mut BuiltValue,
    options: &Options,
) {
    let name = match name {
        Some(n) => n,
        None => return,
    };
    logger_start_sds(
        &mut *options.logger.borrow_mut(),
        crate::bytesbuild!(b"name"),
    );
    let records: &Vec<NameRecord> = name;
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut _name: *mut BuiltValue = json_array_new(records.len());
        let mut j: u16 = 0 as u16;
        while (j as usize) < records.len() {
            let r: *const NameRecord = &records[j as usize];
            let record: *mut BuiltValue = json_object_new(5 as usize);
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
        logger_finish(&mut *options.logger.borrow_mut());
    }
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_parse_name(
    root: *const ParsedValue,
    options: &Options,
) -> Option<NameTable> {
    let mut name: NameTable = Vec::new();
    let mut table: *const ParsedValue = ::core::ptr::null::<ParsedValue>();
    table = json_obj_get_type(
        root,
        b"name\0" as *const u8 as *const ::core::ffi::c_char,
        JsonType::Array,
    );
    if !table.is_null() {
        logger_start_sds(
            &mut *options.logger.borrow_mut(),
            crate::bytesbuild!(b"name"),
        );
        let mut ___loggedstep_v: bool = true;
        while ___loggedstep_v {
            let mut j: u32 = 0 as u32;
            while j < json_arr_len(table) {
                let mut _record: *const ParsedValue = json_arr_at(table, j as u32);
                if !_record.is_null() && json_type_of(_record) == JsonType::Object {
                    if json_obj_get_type(
                        _record,
                        b"platformID\0" as *const u8 as *const ::core::ffi::c_char,
                        JsonType::Integer,
                    )
                    .is_null()
                    {
                        logger_log_sds(
                            &mut *options.logger.borrow_mut(),
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(
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
                        logger_log_sds(
                            &mut *options.logger.borrow_mut(),
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(
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
                        logger_log_sds(
                            &mut *options.logger.borrow_mut(),
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(
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
                        logger_log_sds(
                            &mut *options.logger.borrow_mut(),
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(
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
                        logger_log_sds(
                            &mut *options.logger.borrow_mut(),
                            LOG_VL_IMPORTANT,
                            LoggerType::Warning,
                            crate::bytesbuild!(
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
                        let str: *const ParsedValue = json_obj_get_type(
                            _record,
                            b"nameString\0" as *const u8 as *const ::core::ffi::c_char,
                            JsonType::String,
                        );
                        record.name_string = ::core::slice::from_raw_parts(
                            json_str_ptr(str) as *const u8,
                            json_str_len(str) as usize,
                        )
                        .to_vec();
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
            logger_finish(&mut *options.logger.borrow_mut());
        }
    }
    return Some(name);
}
#[allow(improper_ctypes_definitions)]
pub unsafe fn otfcc_build_name(name: Option<&NameTable>) -> *mut Buffer {
    let name = match name {
        Some(n) => n,
        None => return ::core::ptr::null_mut::<Buffer>(),
    };
    let records: &Vec<NameRecord> = name;
    let buf: *mut Buffer = bufnew();
    bufwrite16b(buf, 0 as u16);
    bufwrite16b(buf, records.len() as u16);
    bufwrite16b(buf, 0 as u16);
    let strings: *mut Buffer = bufnew();
    let mut j: u16 = 0 as u16;
    while (j as usize) < records.len() {
        let record: *const NameRecord = &records[j as usize];
        bufwrite16b(buf, (*record).platform_id);
        bufwrite16b(buf, (*record).encoding_id);
        bufwrite16b(buf, (*record).language_id);
        bufwrite16b(buf, (*record).name_id);
        let cbefore: usize = (*strings).cursor;
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
        let cafter: usize = (*strings).cursor;
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
    let strings_offset: usize = (*buf).cursor;
    bufwrite_buf(buf, strings);
    bufseek(buf, 4 as usize);
    bufwrite16b(buf, strings_offset as u16);
    buffree(strings);
    return buf;
}

#[cfg(test)]
mod parse_name_tests {
    use super::*;

    // header: format(2, unused) + count(u16) + string_offset(u16)
    fn header(count: u16, string_offset: u16) -> Vec<u8> {
        let mut b = vec![0u8, 0u8];
        b.extend_from_slice(&count.to_be_bytes());
        b.extend_from_slice(&string_offset.to_be_bytes());
        b
    }

    // record: platform(2) + encoding(2) + language(2) + name_id(2) + length(2) + offset(2)
    fn record(
        platform_id: u16,
        encoding_id: u16,
        language_id: u16,
        name_id: u16,
        length: u16,
        offset: u16,
    ) -> Vec<u8> {
        let mut b = Vec::new();
        for v in [
            platform_id,
            encoding_id,
            language_id,
            name_id,
            length,
            offset,
        ] {
            b.extend_from_slice(&v.to_be_bytes());
        }
        b
    }

    #[test]
    fn mac_roman_bytes_decode_directly() {
        // platform 1, encoding 0, language 0 -> should_decode_as_bytes.
        let mut data = header(1, 6 + 12);
        data.extend(record(1, 0, 0, 0, 5, 0));
        data.extend_from_slice(b"Hello");
        unsafe {
            let name = parse_name(&data).unwrap();
            assert_eq!(name.len(), 1);
            assert_eq!(name[0].name_string, b"Hello");
        }
    }

    #[test]
    fn windows_unicode_decodes_utf16be() {
        // platform 3, encoding 1 -> should_decode_as_utf16. "Hi" in UTF-16BE.
        let mut data = header(1, 6 + 12);
        data.extend(record(3, 1, 0x0409, 0, 4, 0));
        data.extend_from_slice(&[0x00, b'H', 0x00, b'i']);
        unsafe {
            let name = parse_name(&data).unwrap();
            assert_eq!(name[0].name_string, b"Hi");
        }
    }

    #[test]
    fn truncated_header_errs() {
        assert!(unsafe { parse_name(&[0, 0, 0, 1]) }.is_err());
    }

    #[test]
    fn record_array_shorter_than_declared_count_errs() {
        // count says 2 records (24 bytes) but only one (12 bytes) is present.
        let mut data = header(2, 6 + 24);
        data.extend(record(1, 0, 0, 0, 0, 0));
        unsafe {
            assert!(parse_name(&data).is_err());
        }
    }

    #[test]
    fn count_large_enough_to_overflow_the_multiplication_errs() {
        let data = header(0xFFFF, 0);
        unsafe {
            assert!(parse_name(&data).is_err());
        }
    }

    #[test]
    fn string_span_past_the_table_end_keeps_the_record_with_an_empty_name() {
        // This is the actual overread otfcc_read_name used to have: the
        // record array bound was checked, but a record's *string* span
        // (string_offset + offset, for `length` bytes) never was. Declares
        // a 100-byte Mac-Roman string where only 5 bytes of table remain.
        let mut data = header(1, 6 + 12);
        data.extend(record(1, 0, 0, 7, 100, 0));
        data.extend_from_slice(b"Hello"); // only 5 bytes actually present
        unsafe {
            let name = parse_name(&data).unwrap();
            assert_eq!(name.len(), 1); // record kept
            assert_eq!(name[0].name_id, 7); // metadata preserved
            assert!(name[0].name_string.is_empty()); // string dropped, not read OOB
        }
    }

    #[test]
    fn string_offset_itself_past_the_table_end_keeps_the_record_with_an_empty_name() {
        let mut data = header(1, 0xFFFF); // string_offset far past the table
        data.extend(record(1, 0, 0, 0, 1, 0));
        unsafe {
            let name = parse_name(&data).unwrap();
            assert_eq!(name.len(), 1);
            assert!(name[0].name_string.is_empty());
        }
    }

    #[test]
    fn zero_length_string_is_empty_not_an_error() {
        let mut data = header(1, 6 + 12);
        data.extend(record(1, 0, 0, 0, 0, 0));
        unsafe {
            let name = parse_name(&data).unwrap();
            assert_eq!(name[0].name_string, Vec::<u8>::new());
        }
    }
}
