#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};



use crate::support::binio::{read_16u, read_32u};
use crate::support::handle::{sds_to_vec};
use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};
use crate::vendor::sds::{Byte, Dec5, Hex2};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::otl::{Feature, FeatureList, FeatureRef, LanguageSystem, Lookup, LookupRef, LookupType, Subtable, SubtablePtr, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CONTEXT, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_EXTEND, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GPOS_UNKNOWN, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_CONTEXT, OTL_TYPE_GSUB_EXTEND, OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OTL_TYPE_GSUB_UNKNOWN, OTL_TYPE_UNKNOWN, OtlTable};
use crate::table::otl::{otl_feature_ref_list_dispose, otl_subtable_list_dispose_dependent, new_feature, new_language, new_lookup};
use crate::table::otl::constants::{SCRIPT_LANGUAGE_SEPARATOR};
use crate::table::otl::subtables::chaining::read::{otl_read_chaining, otl_read_contextual};
use crate::table::otl::subtables::extend::{otfcc_read_otl_gpos_extend, otfcc_read_otl_gsub_extend};
use crate::table::otl::subtables::gpos_cursive::{otl_read_gpos_cursive};
use crate::table::otl::subtables::gpos_mark_to_ligature::{otl_read_gpos_mark_to_ligature};
use crate::table::otl::subtables::gpos_mark_to_single::{otl_read_gpos_mark_to_single};
use crate::table::otl::subtables::gpos_pair::{otl_read_gpos_pair};
use crate::table::otl::subtables::gpos_single::{otl_read_gpos_single};
use crate::table::otl::subtables::gsub_ligature::{otl_read_gsub_ligature};
use crate::table::otl::subtables::gsub_multi::{otl_read_gsub_multi};
use crate::table::otl::subtables::gsub_reverse::{otl_read_gsub_reverse};
use crate::table::otl::subtables::gsub_single::{otl_read_gsub_single};
use crate::vendor::sds::{sdsempty, sdsfree};
pub unsafe extern "C" fn otfcc_read_otl_subtable(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut subtable_offset: u32,
    mut lookup_type: LookupType,
    max_glyphs: GlyphId,
    mut options: *const Options,
) -> *mut Subtable {
    match lookup_type {
        OTL_TYPE_GSUB_SINGLE => {
            return otl_read_gsub_single(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GSUB_MULTIPLE => {
            return otl_read_gsub_multi(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GSUB_ALTERNATE => {
            return otl_read_gsub_multi(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GSUB_LIGATURE => {
            return otl_read_gsub_ligature(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GSUB_CHAINING => {
            return otl_read_chaining(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GSUB_REVERSE => {
            return otl_read_gsub_reverse(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GPOS_CHAINING => {
            return otl_read_chaining(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GSUB_CONTEXT => {
            return otl_read_contextual(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GPOS_CONTEXT => {
            return otl_read_contextual(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GPOS_SINGLE => {
            return otl_read_gpos_single(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GPOS_PAIR => {
            return otl_read_gpos_pair(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GPOS_CURSIVE => {
            return otl_read_gpos_cursive(data, table_length, subtable_offset, max_glyphs, options);
        }
        OTL_TYPE_GPOS_MARK_TO_BASE => {
            return otl_read_gpos_mark_to_single(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        OTL_TYPE_GPOS_MARK_TO_MARK => {
            return otl_read_gpos_mark_to_single(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        OTL_TYPE_GPOS_MARK_TO_LIGATURE => {
            return otl_read_gpos_mark_to_ligature(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        OTL_TYPE_GSUB_EXTEND => {
            return otfcc_read_otl_gsub_extend(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        OTL_TYPE_GPOS_EXTEND => {
            return otfcc_read_otl_gpos_extend(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        _ => return ::core::ptr::null_mut::<Subtable>(),
    };
}
unsafe extern "C" fn parse_language(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut base: u32,
    mut lang: *mut LanguageSystem,
    mut features: *mut FeatureList,
) {
    let mut rid: TableId = 0;
    let mut feature_count: TableId = 0;
    if table_length < base.wrapping_add(6 as u32) {
        otl_feature_ref_list_dispose(&raw mut (*lang).features);
        (*lang).required_feature = ::core::ptr::null::<Feature>();
        return;
    } else {
        rid = read_16u(
            data.offset(base as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if (rid as usize) < (*features).len() {
            (*lang).required_feature = &raw const *(&(*features))[rid as usize] as FeatureRef;
        } else {
            (*lang).required_feature = ::core::ptr::null::<Feature>();
        }
        feature_count = read_16u(
            data.offset(base as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < feature_count as ::core::ffi::c_int {
            let mut feature_index: TableId = read_16u(
                data.offset(base as isize)
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset((2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize)
                    as *const u8,
            ) as TableId;
            if (feature_index as usize) < (*features).len() {
                (*lang).features.push(&raw const *(&(*features))[feature_index as usize] as FeatureRef);
            }
            j = j.wrapping_add(1);
        }
        return;
    };
}
unsafe extern "C" fn otfcc_read_otl_common(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut lookup_type_base: LookupType,
    mut options: *const Options,
) -> Option<Box<OtlTable>> {
    let mut script_list_offset: u32 = 0;
    let mut feature_list_offset: u32 = 0;
    let mut lookup_list_offset: u32 = 0;
    let mut current_block: u64;
    let mut table_box: Box<OtlTable> = Box::new(OtlTable { lookups: Vec::new(), features: Vec::new(), languages: Vec::new() });
    let table: *mut OtlTable = table_box.as_mut() as *mut OtlTable;
    if !table.is_null() {
        if !(table_length < 10 as u32) {
            script_list_offset =
                read_16u(data.offset(4 as ::core::ffi::c_int as isize) as *const u8)
                    as u32;
            if !(table_length < script_list_offset.wrapping_add(2 as u32)) {
                feature_list_offset =
                    read_16u(data.offset(6 as ::core::ffi::c_int as isize) as *const u8)
                        as u32;
                if !(table_length < feature_list_offset.wrapping_add(2 as u32)) {
                    lookup_list_offset =
                        read_16u(data.offset(8 as ::core::ffi::c_int as isize) as *const u8)
                            as u32;
                    if !(table_length < lookup_list_offset.wrapping_add(2 as u32)) {
                        let mut lookup_count: TableId =
                            read_16u(data.offset(lookup_list_offset as isize) as *const u8)
                                as TableId;
                        if !(table_length
                            < lookup_list_offset.wrapping_add(2 as u32).wrapping_add(
                                (lookup_count as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as u32,
                            ))
                        {
                            let mut j: TableId = 0 as TableId;
                            loop {
                                if !((j as ::core::ffi::c_int) < lookup_count as ::core::ffi::c_int)
                                {
                                    current_block = 12147880666119273379;
                                    break;
                                }
                                let mut lookup: Box<Lookup> = new_lookup();
                                (*lookup)._offset = lookup_list_offset.wrapping_add(read_16u(
                                    data.offset(lookup_list_offset as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                if table_length < (*lookup)._offset.wrapping_add(6 as u32) {
                                    current_block = 2510049428056405458;
                                    break;
                                }
                                (*lookup).type_0 = LookupType::from_file(
                                    lookup_type_base,
                                    read_16u(data.offset((*lookup)._offset as isize) as *const u8),
                                );
                                (*table).lookups.push(lookup);
                                j = j.wrapping_add(1);
                            }
                            match current_block {
                                2510049428056405458 => {}
                                _ => {
                                    let mut feature_count: TableId =
                                        read_16u(data.offset(feature_list_offset as isize)
                                            as *const u8)
                                            as TableId;
                                    if !(table_length
                                        < feature_list_offset
                                            .wrapping_add(2 as u32)
                                            .wrapping_add(
                                                (feature_count as ::core::ffi::c_int
                                                    * 6 as ::core::ffi::c_int)
                                                    as u32,
                                            ))
                                    {
                                        let mut lnk: TableId = 0 as TableId;
                                        let mut j_0: TableId = 0 as TableId;
                                        loop {
                                            if !((j_0 as ::core::ffi::c_int)
                                                < feature_count as ::core::ffi::c_int)
                                            {
                                                current_block = 13460095289871124136;
                                                break;
                                            }
                                            let mut feature: Box<Feature> = new_feature();
                                            let mut tag: u32 = read_32u(
                                                data.offset(feature_list_offset as isize)
                                                    .offset(2 as ::core::ffi::c_int as isize)
                                                    .offset(
                                                        (j_0 as ::core::ffi::c_int
                                                            * 6 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                    as *const u8,
                                            );
                                            if !(*options).glyph_name_prefix.is_null() {
                                                let tmp = crate::sdsbuild!(
                                                    sdsempty(),
                                                    Byte((tag >> 24 as ::core::ffi::c_int
                                                        & 0xff as u32) as u8),
                                                    Byte((tag >> 16 as ::core::ffi::c_int
                                                        & 0xff as u32) as u8),
                                                    Byte((tag >> 8 as ::core::ffi::c_int
                                                        & 0xff as u32) as u8),
                                                    Byte((tag & 0xff as u32) as u8),
                                                    b"_",
                                                    (*options).glyph_name_prefix,
                                                    b"_",
                                                    Dec5((j_0 as ::core::ffi::c_int) as ::core::ffi::c_int),
                                                );
                                                (*feature).name = sds_to_vec(tmp);
                                                sdsfree(tmp);
                                            } else {
                                                let tmp = crate::sdsbuild!(
                                                    sdsempty(),
                                                    Byte((tag >> 24 as ::core::ffi::c_int
                                                        & 0xff as u32) as u8),
                                                    Byte((tag >> 16 as ::core::ffi::c_int
                                                        & 0xff as u32) as u8),
                                                    Byte((tag >> 8 as ::core::ffi::c_int
                                                        & 0xff as u32) as u8),
                                                    Byte((tag & 0xff as u32) as u8),
                                                    b"_",
                                                    Dec5((j_0 as ::core::ffi::c_int) as ::core::ffi::c_int),
                                                );
                                                (*feature).name = sds_to_vec(tmp);
                                                sdsfree(tmp);
                                            }
                                            let mut feature_offset: u32 = feature_list_offset
                                                .wrapping_add(read_16u(
                                                    data.offset(feature_list_offset as isize)
                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (j_0 as ::core::ffi::c_int
                                                                * 6 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        .offset(4 as ::core::ffi::c_int as isize)
                                                        as *const u8,
                                                )
                                                    as u32);
                                            if table_length
                                                < feature_offset.wrapping_add(4 as u32)
                                            {
                                                current_block = 2510049428056405458;
                                                break;
                                            }
                                            let mut lookup_count_0: TableId = read_16u(
                                                data.offset(feature_offset as isize)
                                                    .offset(2 as ::core::ffi::c_int as isize)
                                                    as *const u8,
                                            )
                                                as TableId;
                                            if table_length
                                                < feature_offset
                                                    .wrapping_add(4 as u32)
                                                    .wrapping_add(
                                                        (lookup_count_0 as ::core::ffi::c_int
                                                            * 2 as ::core::ffi::c_int)
                                                            as u32,
                                                    )
                                            {
                                                current_block = 2510049428056405458;
                                                break;
                                            }
                                            let mut k: TableId = 0 as TableId;
                                            while (k as ::core::ffi::c_int)
                                                < lookup_count_0 as ::core::ffi::c_int
                                            {
                                                let mut lookupid: TableId = read_16u(
                                                    data.offset(feature_offset as isize)
                                                        .offset(4 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (k as ::core::ffi::c_int
                                                                * 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        as *const u8,
                                                )
                                                    as TableId;
                                                if (lookupid as usize) < (*table).lookups.len() {
                                                    let lookup_0: *mut Lookup =
                                                        &raw mut *(&mut (*table).lookups)[lookupid as usize];
                                                    if (*lookup_0).name.is_empty() {
                                                        if !(*options).glyph_name_prefix.is_null() {
                                                            let fresh3 = lnk;
                                                            lnk = lnk.wrapping_add(1);
                                                            let tmp = crate::sdsbuild!(
                                                                sdsempty(),
                                                                b"lookup_",
                                                                (*options).glyph_name_prefix,
                                                                b"_",
                                                                Byte((tag >> 24 as ::core::ffi::c_int
                                                                    & 0xff as u32) as u8),
                                                                Byte((tag >> 16 as ::core::ffi::c_int
                                                                    & 0xff as u32) as u8),
                                                                Byte((tag >> 8 as ::core::ffi::c_int
                                                                    & 0xff as u32) as u8),
                                                                Byte((tag & 0xff as u32) as u8),
                                                                b"_",
                                                                fresh3 as ::core::ffi::c_int,
                                                            );
                                                            (*lookup_0).name = sds_to_vec(tmp);
                                                            sdsfree(tmp);
                                                        } else {
                                                            let fresh4 = lnk;
                                                            lnk = lnk.wrapping_add(1);
                                                            let tmp = crate::sdsbuild!(
                                                                sdsempty(),
                                                                b"lookup_",
                                                                Byte((tag >> 24 as ::core::ffi::c_int
                                                                    & 0xff as u32) as u8),
                                                                Byte((tag >> 16 as ::core::ffi::c_int
                                                                    & 0xff as u32) as u8),
                                                                Byte((tag >> 8 as ::core::ffi::c_int
                                                                    & 0xff as u32) as u8),
                                                                Byte((tag & 0xff as u32) as u8),
                                                                b"_",
                                                                fresh4 as ::core::ffi::c_int,
                                                            );
                                                            (*lookup_0).name = sds_to_vec(tmp);
                                                            sdsfree(tmp);
                                                        }
                                                    }
                                                    (*feature).lookups.push(lookup_0 as LookupRef);
                                                }
                                                k = k.wrapping_add(1);
                                            }
                                            (*table).features.push(feature);
                                            j_0 = j_0.wrapping_add(1);
                                        }
                                        match current_block {
                                            2510049428056405458 => {}
                                            _ => {
                                                let mut script_count: TableId =
                                                    read_16u(data.offset(script_list_offset as isize)
                                                        as *const u8)
                                                        as TableId;
                                                if !(table_length
                                                    < script_list_offset
                                                        .wrapping_add(2 as u32)
                                                        .wrapping_add(
                                                            (6 as ::core::ffi::c_int
                                                                * script_count as ::core::ffi::c_int)
                                                                as u32,
                                                        ))
                                                {
                                                    let mut n_language_combinations: u32 =
                                                        0 as u32;
                                                    let mut j_1: TableId = 0 as TableId;
                                                    loop {
                                                        if !((j_1 as ::core::ffi::c_int)
                                                            < script_count as ::core::ffi::c_int)
                                                        {
                                                            current_block = 6528285054092551010;
                                                            break;
                                                        }
                                                        let mut script_offset: u32 =
                                                            script_list_offset
                                                                .wrapping_add(read_16u(
                                                                data.offset(
                                                                    script_list_offset as isize,
                                                                )
                                                                .offset(
                                                                    2 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                .offset(
                                                                    (6 as ::core::ffi::c_int
                                                                        * j_1 as ::core::ffi::c_int)
                                                                        as isize,
                                                                )
                                                                .offset(
                                                                    4 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                    as *const u8,
                                                            )
                                                                as u32);
                                                        if table_length
                                                            < script_offset
                                                                .wrapping_add(4 as u32)
                                                        {
                                                            current_block = 2510049428056405458;
                                                            break;
                                                        }
                                                        let mut default_lang_system: TableId =
                                                            read_16u(
                                                                data.offset(script_offset as isize)
                                                                    as *const u8,
                                                            )
                                                                as TableId;
                                                        n_language_combinations =
                                                            n_language_combinations.wrapping_add(
                                                                ((if default_lang_system
                                                                    as ::core::ffi::c_int
                                                                    != 0
                                                                {
                                                                    1 as ::core::ffi::c_int
                                                                } else {
                                                                    0 as ::core::ffi::c_int
                                                                }) + read_16u(
                                                                    data.offset(
                                                                        script_offset as isize,
                                                                    )
                                                                    .offset(
                                                                        2 as ::core::ffi::c_int
                                                                            as isize,
                                                                    )
                                                                        as *const u8,
                                                                )
                                                                    as ::core::ffi::c_int)
                                                                    as u32,
                                                            );
                                                        j_1 = j_1.wrapping_add(1);
                                                    }
                                                    match current_block {
                                                        2510049428056405458 => {}
                                                        _ => {
                                                            let mut j_2: TableId = 0 as TableId;
                                                            while (j_2 as ::core::ffi::c_int)
                                                                < script_count as ::core::ffi::c_int
                                                            {
                                                                let mut tag_0: u32 = read_32u(
                                                                    data
                                                                        .offset(script_list_offset as isize)
                                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                                        .offset(
                                                                            (6 as ::core::ffi::c_int * j_2 as ::core::ffi::c_int)
                                                                                as isize,
                                                                        ) as *const u8,
                                                                );
                                                                let mut script_offset_0: u32 = script_list_offset
                                                                    .wrapping_add(
                                                                        read_16u(
                                                                            data
                                                                                .offset(script_list_offset as isize)
                                                                                .offset(2 as ::core::ffi::c_int as isize)
                                                                                .offset(
                                                                                    (6 as ::core::ffi::c_int * j_2 as ::core::ffi::c_int)
                                                                                        as isize,
                                                                                )
                                                                                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
                                                                        ) as u32,
                                                                    );
                                                                let mut default_lang_system_0: TableId = read_16u(
                                                                    data.offset(script_offset_0 as isize) as *const u8,
                                                                ) as TableId;
                                                                if default_lang_system_0 != 0 {
                                                                    let mut lang: Box<LanguageSystem> = new_language();
                                                                    let tmp = crate::sdsbuild!(
                                                                        sdsempty(),
                                                                        Byte((tag_0 >> 24 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((tag_0 >> 16 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((tag_0 >> 8 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((tag_0 & 0xff as u32) as u8),
                                                                        Byte((SCRIPT_LANGUAGE_SEPARATOR as ::core::ffi::c_int) as u8),
                                                                        b"DFLT",
                                                                    );
                                                                    (*lang).name = sds_to_vec(tmp);
                                                                    sdsfree(tmp);
                                                                    parse_language(
                                                                        data,
                                                                        table_length,
                                                                        script_offset_0
                                                                            .wrapping_add(
                                                                                default_lang_system_0
                                                                                    as u32,
                                                                            ),
                                                                        &raw mut *lang,
                                                                        &raw mut (*table).features,
                                                                    );
                                                                    (*table).languages.push(lang);
                                                                }
                                                                let mut lang_sys_count: TableId =
                                                                    read_16u(
                                                                        data.offset(
                                                                            script_offset_0 as isize,
                                                                        )
                                                                        .offset(
                                                                            2 as ::core::ffi::c_int
                                                                                as isize,
                                                                        )
                                                                            as *const u8,
                                                                    )
                                                                        as TableId;
                                                                let mut k_0: TableId =
                                                                    0 as TableId;
                                                                while (k_0 as ::core::ffi::c_int)
                                                                    < lang_sys_count
                                                                        as ::core::ffi::c_int
                                                                {
                                                                    let mut lang_tag: u32 = read_32u(
                                                                        data
                                                                            .offset(script_offset_0 as isize)
                                                                            .offset(4 as ::core::ffi::c_int as isize)
                                                                            .offset(
                                                                                (6 as ::core::ffi::c_int * k_0 as ::core::ffi::c_int)
                                                                                    as isize,
                                                                            ) as *const u8,
                                                                    );
                                                                    let mut lang_sys: TableId = read_16u(
                                                                        data
                                                                            .offset(script_offset_0 as isize)
                                                                            .offset(4 as ::core::ffi::c_int as isize)
                                                                            .offset(
                                                                                (6 as ::core::ffi::c_int * k_0 as ::core::ffi::c_int)
                                                                                    as isize,
                                                                            )
                                                                            .offset(4 as ::core::ffi::c_int as isize) as *const u8,
                                                                    ) as TableId;
                                                                    let mut lang_0: Box<LanguageSystem> = new_language();
                                                                    let tmp = crate::sdsbuild!(
                                                                        sdsempty(),
                                                                        Byte((tag_0 >> 24 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((tag_0 >> 16 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((tag_0 >> 8 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((tag_0 & 0xff as u32) as u8),
                                                                        Byte((SCRIPT_LANGUAGE_SEPARATOR as ::core::ffi::c_int) as u8),
                                                                        Byte((lang_tag >> 24 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((lang_tag >> 16 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((lang_tag >> 8 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((lang_tag & 0xff as u32) as u8),
                                                                    );
                                                                    (*lang_0).name = sds_to_vec(tmp);
                                                                    sdsfree(tmp);
                                                                    parse_language(
                                                                        data,
                                                                        table_length,
                                                                        script_offset_0
                                                                            .wrapping_add(
                                                                                lang_sys as u32,
                                                                            ),
                                                                        &raw mut *lang_0,
                                                                        &raw mut (*table).features,
                                                                    );
                                                                    (*table).languages.push(lang_0);
                                                                    k_0 = k_0.wrapping_add(1);
                                                                }
                                                                j_2 = j_2.wrapping_add(1);
                                                            }
                                                            let mut j_3: TableId = 0 as TableId;
                                                            while (j_3 as usize)
                                                                < (*table).lookups.len()
                                                            {
                                                                if (*(&(*table)
                                                                    .lookups)[j_3 as usize])
                                                                .name
                                                                .is_empty()
                                                                {
                                                                    if !(*options)
                                                                        .glyph_name_prefix
                                                                        .is_null()
                                                                    {
                                                                        let tmp = crate::sdsbuild!(
                                                                            sdsempty(),
                                                                            b"lookup_",
                                                                            (*options).glyph_name_prefix,
                                                                            b"_",
                                                                            Hex2((*(&(*table).lookups)[j_3 as usize]).type_0.raw()),
                                                                            b"_",
                                                                            j_3 as ::core::ffi::c_int,
                                                                        );
                                                                        (*(&mut (*table).lookups)[j_3 as usize]).name = sds_to_vec(tmp);
                                                                        sdsfree(tmp);
                                                                    } else {
                                                                        let tmp = crate::sdsbuild!(
                                                                            sdsempty(),
                                                                            b"lookup_",
                                                                            Hex2((*(&(*table).lookups)[j_3 as usize]).type_0.raw()),
                                                                            b"_",
                                                                            j_3 as ::core::ffi::c_int,
                                                                        );
                                                                        (*(&mut (*table).lookups)[j_3 as usize]).name = sds_to_vec(tmp);
                                                                        sdsfree(tmp);
                                                                    }
                                                                }
                                                                j_3 = j_3.wrapping_add(1);
                                                            }
                                                            return Some(table_box);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return None;
}
unsafe extern "C" fn otfcc_read_otl_lookup(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut lookup: *mut Lookup,
    mut max_glyphs: GlyphId,
    mut options: *const Options,
) {
    (*lookup).flags = read_16u(
        data.offset((*lookup)._offset as isize)
            .offset(2 as ::core::ffi::c_int as isize) as *const u8,
    );
    let mut subtable_count: TableId = read_16u(
        data.offset((*lookup)._offset as isize)
            .offset(4 as ::core::ffi::c_int as isize) as *const u8,
    ) as TableId;
    if subtable_count == 0
        || table_length
            < (*lookup)._offset.wrapping_add(6 as u32).wrapping_add(
                (2 as ::core::ffi::c_int * subtable_count as ::core::ffi::c_int) as u32,
            )
    {
        (*lookup).type_0 = OTL_TYPE_UNKNOWN;
        return;
    }
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < subtable_count as ::core::ffi::c_int {
        let mut subtable_offset: u32 = (*lookup)._offset.wrapping_add(read_16u(
            data.offset((*lookup)._offset as isize)
                .offset(6 as ::core::ffi::c_int as isize)
                .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                as *const u8,
        ) as u32);
        let mut subtable: *mut Subtable = otfcc_read_otl_subtable(
            data,
            table_length,
            subtable_offset,
            (*lookup).type_0,
            max_glyphs,
            options,
        );
        (*lookup).subtables.push(subtable as SubtablePtr);
        j = j.wrapping_add(1);
    }
    if (*lookup).type_0 == OTL_TYPE_GSUB_EXTEND
        || (*lookup).type_0 == OTL_TYPE_GPOS_EXTEND
    {
        (*lookup).type_0 = OTL_TYPE_UNKNOWN;
        let mut j_0: TableId = 0 as TableId;
        while (j_0 as usize) < (*lookup).subtables.len() {
            if !(&(*lookup).subtables)[j_0 as usize].is_null() {
                (*lookup).type_0 = (*(&(*lookup).subtables)[j_0 as usize])
                    .extend
                    .type_0;
                break;
            } else {
                j_0 = j_0.wrapping_add(1);
            }
        }
        if (*lookup).type_0 != OTL_TYPE_UNKNOWN {
            let mut j_1: TableId = 0 as TableId;
            while (j_1 as usize) < (*lookup).subtables.len() {
                if !(&(*lookup).subtables)[j_1 as usize].is_null()
                    && (*(&(*lookup).subtables)[j_1 as usize])
                        .extend
                        .type_0
                        == (*lookup).type_0
                {
                    let st: *mut Subtable =
                        (*(&(*lookup).subtables)[j_1 as usize])
                            .extend
                            .subtable as *mut Subtable;
                    free(
                        (&(*lookup).subtables)[j_1 as usize] as *mut ::core::ffi::c_void
                    );
                    (&mut (*lookup).subtables)[j_1 as usize] = st as SubtablePtr;
                } else if !(&(*lookup).subtables)[j_1 as usize].is_null() {
                    // A scratch `Lookup` purely to reuse its (now `Drop`-driven)
                    // type-dispatched subtable teardown on this one subtable --
                    // never pushed anywhere, so it's just let go out of scope
                    // instead of the old explicit `otfcc_delete_lookup` call.
                    let mut temp: Box<Lookup> = new_lookup();
                    (*temp).type_0 = (*(&(*lookup).subtables)[j_1 as usize])
                        .extend
                        .type_0;
                    (*temp).subtables.push(
                        (*(&(*lookup).subtables)[j_1 as usize])
                            .extend
                            .subtable as SubtablePtr,
                    );
                    drop(temp);
                    free(
                        (&(*lookup).subtables)[j_1 as usize] as *mut ::core::ffi::c_void
                    );
                    (&mut (*lookup).subtables)[j_1 as usize] = ::core::ptr::null_mut::<Subtable>();
                }
                j_1 = j_1.wrapping_add(1);
            }
        } else {
            otl_subtable_list_dispose_dependent(&raw mut (*lookup).subtables, lookup);
            return;
        }
    }
    if (*lookup).type_0 == OTL_TYPE_GSUB_CONTEXT
    {
        (*lookup).type_0 = OTL_TYPE_GSUB_CHAINING;
    }
    if (*lookup).type_0 == OTL_TYPE_GPOS_CONTEXT
    {
        (*lookup).type_0 = OTL_TYPE_GPOS_CHAINING;
    }
}
pub unsafe extern "C" fn otfcc_read_otl(
    mut packet: Packet,
    mut options: *const Options,
    mut tag: u32,
    mut max_glyphs: GlyphId,
) -> Option<Box<OtlTable>> {
    let mut otl: Option<Box<OtlTable>> = None;
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.num_tables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == tag {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    otl = otfcc_read_otl_common(
                        data,
                        length,
                        if tag == 1196643650i32 as u32 {
                            OTL_TYPE_GSUB_UNKNOWN
                        } else if tag == 1196445523i32 as u32 {
                            OTL_TYPE_GPOS_UNKNOWN
                        } else {
                            OTL_TYPE_UNKNOWN
                        },
                        options,
                    );
                    if let Some(otl_box) = otl.as_mut() {
                        let otl_ptr: *mut OtlTable = otl_box.as_mut() as *mut OtlTable;
                        let mut j: TableId = 0 as TableId;
                        while (j as usize) < (*otl_ptr).lookups.len() {
                            otfcc_read_otl_lookup(
                                data,
                                length,
                                &raw mut *(&mut (*otl_ptr).lookups)[j as usize],
                                max_glyphs,
                                options,
                            );
                            j = j.wrapping_add(1);
                        }
                        return otl;
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
    return None;
}
