use libc::{malloc};
extern "C" {
    fn json_array_new(length: usize) -> *mut json_value;
    fn json_array_push(array: *mut json_value, _: *mut json_value) -> *mut json_value;
    fn json_object_new(length: usize) -> *mut json_value;
    fn json_object_push(
        object: *mut json_value,
        name: *const ::core::ffi::c_char,
        _: *mut json_value,
    ) -> *mut json_value;
    fn json_string_new(_: *const ::core::ffi::c_char) -> *mut json_value;
    fn json_string_new_nocopy(
        length: ::core::ffi::c_uint,
        _: *mut ::core::ffi::c_char,
    ) -> *mut json_value;
    fn json_integer_new(_: i64) -> *mut json_value;
    fn json_boolean_new(_: ::core::ffi::c_int) -> *mut json_value;
    fn json_measure_ex(_: *mut json_value, _: json_serialize_opts) -> usize;
    fn json_serialize_ex(buf: *mut ::core::ffi::c_char, _: *mut json_value, _: json_serialize_opts);
    fn json_builder_free(_: *mut json_value);
    fn sdsempty() -> sds;
    fn sdscatprintf(s: sds, fmt: *const ::core::ffi::c_char, ...) -> sds;
    fn otl_gsub_dump_single(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gsub_dump_multi(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gsub_dump_ligature(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gsub_dump_reverse(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gpos_dump_single(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gpos_dump_cursive(_subtable: *const otl_Subtable) -> *mut json_value;
    fn otl_gpos_dump_markToSingle(st: *const otl_Subtable) -> *mut json_value;
    fn otl_gpos_dump_markToLigature(st: *const otl_Subtable) -> *mut json_value;
    fn otl_dump_chaining(_subtable: *const otl_Subtable) -> *mut json_value;
    static mut lookupFlagsLabels: [*const ::core::ffi::c_char; 0];
    static mut tableNames: [*const ::core::ffi::c_char; 0];
    fn otl_gpos_dump_pair(_subtable: *const otl_Subtable) -> *mut json_value;
}



use crate::logger::{otfcc_ILogger};
use crate::support::options::{otfcc_Options};
use crate::support::primitives::{tableid_t};
use crate::vendor::sds::{sds};
use crate::vendor::json::{json_pre_serialized, json_value};
use crate::support::{true_0};
use crate::table::otl::{otl_Feature, otl_LanguageSystem, otl_Lookup, otl_LookupType, otl_Subtable, otl_type_gpos_chaining, otl_type_gpos_cursive, otl_type_gpos_markToBase, otl_type_gpos_markToLigature, otl_type_gpos_markToMark, otl_type_gpos_pair, otl_type_gpos_single, otl_type_gsub_alternate, otl_type_gsub_chaining, otl_type_gsub_ligature, otl_type_gsub_multiple, otl_type_gsub_reverse, otl_type_gsub_single, table_OTL};
use crate::vendor::json_builder::{json_serialize_mode_packed, json_serialize_opts};
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
unsafe extern "C" fn preserialize(mut x: *mut json_value) -> *mut json_value {
    let mut opts: json_serialize_opts = json_serialize_opts {
        mode: json_serialize_mode_packed,
        opts: 0,
        indent_size: 0,
    };
    let mut preserialize_len: usize = json_measure_ex(x, opts);
    let mut buf: *mut ::core::ffi::c_char = malloc(preserialize_len) as *mut ::core::ffi::c_char;
    json_serialize_ex(buf, x, opts);
    json_builder_free(x);
    let mut xx: *mut json_value = json_string_new_nocopy(
        preserialize_len.wrapping_sub(1 as usize) as ::core::ffi::c_uint,
        buf,
    );
    (*xx).type_0 = json_pre_serialized;
    return xx;
}
unsafe extern "C" fn _declare_lookup_dumper(
    mut llt: otl_LookupType,
    mut lt: *const ::core::ffi::c_char,
    mut dumper: Option<unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value>,
    mut lookup: *mut otl_Lookup,
    mut dump: *mut json_value,
) {
    if (*lookup).type_0 as ::core::ffi::c_uint == llt as ::core::ffi::c_uint {
        json_object_push(
            dump,
            b"type\0" as *const u8 as *const ::core::ffi::c_char,
            json_string_new(lt),
        );
        json_object_push(
            dump,
            b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            otfcc_dump_flags(
                (*lookup).flags as ::core::ffi::c_int,
                &raw mut lookupFlagsLabels as *mut *const ::core::ffi::c_char,
            ),
        );
        if (*lookup).flags as ::core::ffi::c_int >> 8 as ::core::ffi::c_int != 0 {
            json_object_push(
                dump,
                b"markAttachmentType\0" as *const u8 as *const ::core::ffi::c_char,
                json_integer_new(
                    ((*lookup).flags as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as i64,
                ),
            );
        }
        let mut subtables: *mut json_value = json_array_new((*lookup).subtables.length);
        let mut j: tableid_t = 0 as tableid_t;
        while (j as usize) < (*lookup).subtables.length {
            if !(*(*lookup).subtables.items.offset(j as isize)).is_null() {
                json_array_push(
                    subtables,
                    dumper.expect("non-null function pointer")(
                        *(*lookup).subtables.items.offset(j as isize) as *const otl_Subtable,
                    ),
                );
            }
            j = j.wrapping_add(1);
        }
        json_object_push(
            dump,
            b"subtables\0" as *const u8 as *const ::core::ffi::c_char,
            subtables,
        );
    }
}
unsafe extern "C" fn _dump_lookup(mut lookup: *mut otl_Lookup, mut dump: *mut json_value) {
    _declare_lookup_dumper(
        otl_type_gsub_single,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_single as ::core::ffi::c_int as isize),
        Some(otl_gsub_dump_single as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gsub_multiple,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_multiple as ::core::ffi::c_int as isize),
        Some(otl_gsub_dump_multi as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gsub_alternate,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_alternate as ::core::ffi::c_int as isize),
        Some(otl_gsub_dump_multi as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gsub_ligature,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_ligature as ::core::ffi::c_int as isize),
        Some(
            otl_gsub_dump_ligature as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gsub_chaining,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_chaining as ::core::ffi::c_int as isize),
        Some(otl_dump_chaining as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gsub_reverse,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gsub_reverse as ::core::ffi::c_int as isize),
        Some(otl_gsub_dump_reverse as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_chaining,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_chaining as ::core::ffi::c_int as isize),
        Some(otl_dump_chaining as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_single,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_single as ::core::ffi::c_int as isize),
        Some(otl_gpos_dump_single as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_pair,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_pair as ::core::ffi::c_int as isize),
        Some(otl_gpos_dump_pair as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_cursive,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_cursive as ::core::ffi::c_int as isize),
        Some(otl_gpos_dump_cursive as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_markToBase,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_markToBase as ::core::ffi::c_int as isize),
        Some(
            otl_gpos_dump_markToSingle
                as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_markToMark,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_markToMark as ::core::ffi::c_int as isize),
        Some(
            otl_gpos_dump_markToSingle
                as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value,
        ),
        lookup,
        dump,
    );
    _declare_lookup_dumper(
        otl_type_gpos_markToLigature,
        *(&raw mut tableNames as *mut *const ::core::ffi::c_char)
            .offset(otl_type_gpos_markToLigature as ::core::ffi::c_int as isize),
        Some(
            otl_gpos_dump_markToLigature
                as unsafe extern "C" fn(*const otl_Subtable) -> *mut json_value,
        ),
        lookup,
        dump,
    );
}
#[no_mangle]
pub unsafe extern "C" fn otfcc_dumpOtl(
    mut table: *const table_OTL,
    mut root: *mut json_value,
    mut options: *const otfcc_Options,
    mut tag: *const ::core::ffi::c_char,
) {
    if table.is_null()
        || (*table).languages.length == 0
        || (*table).lookups.length == 0
        || (*table).features.length == 0
    {
        return;
    }
    (*(*options).logger)
        .startSDS
        .expect("non-null function pointer")(
        (*options).logger as *mut otfcc_ILogger,
        sdscatprintf(
            sdsempty(),
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            tag,
        ),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut otl: *mut json_value = json_object_new(3 as usize);
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"Languages"),
        );
        let mut ___loggedstep_v_0: bool = true;
        while ___loggedstep_v_0 {
            let mut languages: *mut json_value = json_object_new((*table).languages.length);
            let mut j: tableid_t = 0 as tableid_t;
            while (j as usize) < (*table).languages.length {
                let mut _lang: *mut json_value = json_object_new(5 as usize);
                let mut lang: *mut otl_LanguageSystem =
                    *(*table).languages.items.offset(j as isize) as *mut otl_LanguageSystem;
                if !(*lang).requiredFeature.is_null() {
                    json_object_push(
                        _lang,
                        b"requiredFeature\0" as *const u8 as *const ::core::ffi::c_char,
                        json_string_new(
                            (*(*lang).requiredFeature).name as *const ::core::ffi::c_char,
                        ),
                    );
                }
                let mut features: *mut json_value = json_array_new((*lang).features.length);
                let mut k: tableid_t = 0 as tableid_t;
                while (k as usize) < (*lang).features.length {
                    if !(*(*lang).features.items.offset(k as isize)).is_null() {
                        json_array_push(
                            features,
                            json_string_new(
                                (**(*lang).features.items.offset(k as isize)).name
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                    }
                    k = k.wrapping_add(1);
                }
                json_object_push(
                    _lang,
                    b"features\0" as *const u8 as *const ::core::ffi::c_char,
                    preserialize(features),
                );
                json_object_push(languages, (*lang).name as *const ::core::ffi::c_char, _lang);
                j = j.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"languages\0" as *const u8 as *const ::core::ffi::c_char,
                languages,
            );
            ___loggedstep_v_0 = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"Features"),
        );
        let mut ___loggedstep_v_1: bool = true;
        while ___loggedstep_v_1 {
            let mut features_0: *mut json_value = json_object_new((*table).features.length);
            let mut j_0: tableid_t = 0 as tableid_t;
            while (j_0 as usize) < (*table).features.length {
                let mut feature: *mut otl_Feature =
                    *(*table).features.items.offset(j_0 as isize) as *mut otl_Feature;
                let mut _feature: *mut json_value = json_array_new((*feature).lookups.length);
                let mut k_0: tableid_t = 0 as tableid_t;
                while (k_0 as usize) < (*feature).lookups.length {
                    if !(*(*feature).lookups.items.offset(k_0 as isize)).is_null() {
                        json_array_push(
                            _feature,
                            json_string_new(
                                (**(*feature).lookups.items.offset(k_0 as isize)).name
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                    }
                    k_0 = k_0.wrapping_add(1);
                }
                json_object_push(
                    features_0,
                    (*feature).name as *const ::core::ffi::c_char,
                    preserialize(_feature),
                );
                j_0 = j_0.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"features\0" as *const u8 as *const ::core::ffi::c_char,
                features_0,
            );
            ___loggedstep_v_1 = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        (*(*options).logger)
            .startSDS
            .expect("non-null function pointer")(
            (*options).logger as *mut otfcc_ILogger,
            crate::sdsbuild!(sdsempty(), b"Lookups"),
        );
        let mut ___loggedstep_v_2: bool = true;
        while ___loggedstep_v_2 {
            let mut lookups: *mut json_value = json_object_new((*table).lookups.length);
            let mut lookupOrder: *mut json_value = json_array_new((*table).lookups.length);
            let mut j_1: tableid_t = 0 as tableid_t;
            while (j_1 as usize) < (*table).lookups.length {
                let mut _lookup: *mut json_value = json_object_new(5 as usize);
                let mut lookup: *mut otl_Lookup =
                    *(*table).lookups.items.offset(j_1 as isize) as *mut otl_Lookup;
                _dump_lookup(lookup, _lookup);
                json_object_push(
                    lookups,
                    (*lookup).name as *const ::core::ffi::c_char,
                    _lookup,
                );
                json_array_push(
                    lookupOrder,
                    json_string_new((*lookup).name as *const ::core::ffi::c_char),
                );
                j_1 = j_1.wrapping_add(1);
            }
            json_object_push(
                otl,
                b"lookups\0" as *const u8 as *const ::core::ffi::c_char,
                lookups,
            );
            json_object_push(
                otl,
                b"lookupOrder\0" as *const u8 as *const ::core::ffi::c_char,
                lookupOrder,
            );
            ___loggedstep_v_2 = false;
            (*(*options).logger)
                .finish
                .expect("non-null function pointer")(
                (*options).logger as *mut otfcc_ILogger
            );
        }
        json_object_push(root, tag, otl);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut otfcc_ILogger);
    }
}
