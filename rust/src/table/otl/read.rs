#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};



use crate::support::binio::{read_16u, read_32u};

use crate::support::options::{Options};
use crate::support::primitives::{FontFilePointer, GlyphId, TableId};
use crate::vendor::sds::{Byte, Dec5, Hex2};
use crate::font::caryll_sfnt::{Packet, PacketPiece};

use crate::table::otl::{Feature, FeatureList, FeaturePtr, FeatureRef, LanguageSystem, LanguageSystemPtr, Lookup, LookupPtr, LookupRef, LookupType, Subtable, SubtablePtr, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CONTEXT, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_EXTEND, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GPOS_UNKNOWN, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_CONTEXT, OTL_TYPE_GSUB_EXTEND, OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OTL_TYPE_GSUB_UNKNOWN, OTL_TYPE_UNKNOWN, OtlTable};
use crate::table::otl::{otfcc_delete_lookup, OTL_I_FEATURE_LIST, OTL_I_FEATURE_PTR, OTL_I_FEATURE_REF_LIST, OTL_I_LANG_SYSTEM_LIST, OTL_I_LANGUAGE_SYSTEM, OTL_I_LOOKUP_LIST, OTL_I_LOOKUP_PTR, OTL_I_LOOKUP_REF_LIST, OTL_I_SUBTABLE_LIST, TABLE_I_OTL};
use crate::table::otl::constants::{SCRIPT_LANGUAGE_SEPARATOR};
use crate::table::otl::subtables::chaining::read::{otl_read_chaining, otl_read_contextual};
use crate::table::otl::subtables::extend::{otfcc_readOtl_gpos_extend, otfcc_readOtl_gsub_extend};
use crate::table::otl::subtables::gpos_cursive::{otl_read_gpos_cursive};
use crate::table::otl::subtables::gpos_mark_to_ligature::{otl_read_gpos_markToLigature};
use crate::table::otl::subtables::gpos_mark_to_single::{otl_read_gpos_markToSingle};
use crate::table::otl::subtables::gpos_pair::{otl_read_gpos_pair};
use crate::table::otl::subtables::gpos_single::{otl_read_gpos_single};
use crate::table::otl::subtables::gsub_ligature::{otl_read_gsub_ligature};
use crate::table::otl::subtables::gsub_multi::{otl_read_gsub_multi};
use crate::table::otl::subtables::gsub_reverse::{otl_read_gsub_reverse};
use crate::table::otl::subtables::gsub_single::{otl_read_gsub_single};
use crate::vendor::sds::{sdsempty};
pub unsafe extern "C" fn otfcc_readOtl_subtable(
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
            return otl_read_gpos_markToSingle(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        OTL_TYPE_GPOS_MARK_TO_MARK => {
            return otl_read_gpos_markToSingle(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        OTL_TYPE_GPOS_MARK_TO_LIGATURE => {
            return otl_read_gpos_markToLigature(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        OTL_TYPE_GSUB_EXTEND => {
            return otfcc_readOtl_gsub_extend(
                data,
                table_length,
                subtable_offset,
                max_glyphs,
                options,
            );
        }
        OTL_TYPE_GPOS_EXTEND => {
            return otfcc_readOtl_gpos_extend(
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
unsafe extern "C" fn parseLanguage(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut base: u32,
    mut lang: *mut LanguageSystem,
    mut features: *mut FeatureList,
) {
    let mut rid: TableId = 0;
    let mut feature_count: TableId = 0;
    if table_length < base.wrapping_add(6 as u32) {
        OTL_I_FEATURE_REF_LIST
            .dispose
            .expect("non-null function pointer")(&raw mut (*lang).features);
        (*lang).requiredFeature = ::core::ptr::null::<Feature>();
        return;
    } else {
        rid = read_16u(
            data.offset(base as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        if (rid as usize) < (*features).length {
            (*lang).requiredFeature = *(*features).items.offset(rid as isize) as FeatureRef;
        } else {
            (*lang).requiredFeature = ::core::ptr::null::<Feature>();
        }
        feature_count = read_16u(
            data.offset(base as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as TableId;
        let mut j: TableId = 0 as TableId;
        while (j as ::core::ffi::c_int) < feature_count as ::core::ffi::c_int {
            let mut featureIndex: TableId = read_16u(
                data.offset(base as isize)
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset((2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize)
                    as *const u8,
            ) as TableId;
            if (featureIndex as usize) < (*features).length {
                OTL_I_FEATURE_REF_LIST.push.expect("non-null function pointer")(
                    &raw mut (*lang).features,
                    *(*features).items.offset(featureIndex as isize) as FeatureRef,
                );
            }
            j = j.wrapping_add(1);
        }
        return;
    };
}
unsafe extern "C" fn otfcc_readOtl_common(
    mut data: FontFilePointer,
    mut table_length: u32,
    mut lookup_type_base: LookupType,
    mut options: *const Options,
) -> *mut OtlTable {
    let mut script_list_offset: u32 = 0;
    let mut feature_list_offset: u32 = 0;
    let mut lookup_list_offset: u32 = 0;
    let mut current_block: u64;
    let mut table: *mut OtlTable = (
        TABLE_I_OTL.create.expect("non-null function pointer"))();
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
                                let mut lookup: *mut Lookup =
                                    ::core::ptr::null_mut::<Lookup>();
                                OTL_I_LOOKUP_PTR.init.expect("non-null function pointer")(
                                    &raw mut lookup,
                                );
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
                                OTL_I_LOOKUP_LIST.push.expect("non-null function pointer")(
                                    &raw mut (*table).lookups,
                                    lookup as LookupPtr,
                                );
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
                                            let mut feature: *mut Feature =
                                                ::core::ptr::null_mut::<Feature>();
                                            OTL_I_FEATURE_PTR
                                                .init
                                                .expect("non-null function pointer")(
                                                &raw mut feature,
                                            );
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
                                                (*feature).name = crate::sdsbuild!(
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
                                            } else {
                                                (*feature).name = crate::sdsbuild!(
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
                                                if (lookupid as usize) < (*table).lookups.length {
                                                    let mut lookup_0: *mut Lookup = *(*table)
                                                        .lookups
                                                        .items
                                                        .offset(lookupid as isize)
                                                        as *mut Lookup;
                                                    if (*lookup_0).name.is_null() {
                                                        if !(*options).glyph_name_prefix.is_null() {
                                                            let fresh3 = lnk;
                                                            lnk = lnk.wrapping_add(1);
                                                            (*lookup_0).name = crate::sdsbuild!(
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
                                                        } else {
                                                            let fresh4 = lnk;
                                                            lnk = lnk.wrapping_add(1);
                                                            (*lookup_0).name = crate::sdsbuild!(
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
                                                        }
                                                    }
                                                    OTL_I_LOOKUP_REF_LIST
                                                        .push
                                                        .expect("non-null function pointer")(
                                                        &raw mut (*feature).lookups,
                                                        lookup_0 as LookupRef,
                                                    );
                                                }
                                                k = k.wrapping_add(1);
                                            }
                                            OTL_I_FEATURE_LIST
                                                .push
                                                .expect("non-null function pointer")(
                                                &raw mut (*table).features,
                                                feature as FeaturePtr,
                                            );
                                            j_0 = j_0.wrapping_add(1);
                                        }
                                        match current_block {
                                            2510049428056405458 => {}
                                            _ => {
                                                let mut scriptCount: TableId =
                                                    read_16u(data.offset(script_list_offset as isize)
                                                        as *const u8)
                                                        as TableId;
                                                if !(table_length
                                                    < script_list_offset
                                                        .wrapping_add(2 as u32)
                                                        .wrapping_add(
                                                            (6 as ::core::ffi::c_int
                                                                * scriptCount as ::core::ffi::c_int)
                                                                as u32,
                                                        ))
                                                {
                                                    let mut n_language_combinations: u32 =
                                                        0 as u32;
                                                    let mut j_1: TableId = 0 as TableId;
                                                    loop {
                                                        if !((j_1 as ::core::ffi::c_int)
                                                            < scriptCount as ::core::ffi::c_int)
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
                                                                < scriptCount as ::core::ffi::c_int
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
                                                                    let mut lang: *mut LanguageSystem = ::core::ptr::null_mut::<
                                                                        LanguageSystem,
                                                                    >();
                                                                    OTL_I_LANGUAGE_SYSTEM
                                                                        .init
                                                                        .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        &raw mut lang
                                                                    );
                                                                    (*lang).name = crate::sdsbuild!(
                                                                        sdsempty(),
                                                                        Byte((tag_0 >> 24 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((tag_0 >> 16 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((tag_0 >> 8 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((tag_0 & 0xff as u32) as u8),
                                                                        Byte((SCRIPT_LANGUAGE_SEPARATOR as ::core::ffi::c_int) as u8),
                                                                        b"DFLT",
                                                                    );
                                                                    parseLanguage(
                                                                        data,
                                                                        table_length,
                                                                        script_offset_0
                                                                            .wrapping_add(
                                                                                default_lang_system_0
                                                                                    as u32,
                                                                            ),
                                                                        lang,
                                                                        &raw mut (*table).features,
                                                                    );
                                                                    OTL_I_LANG_SYSTEM_LIST
                                                                        .push
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(
                                                                        &raw mut (*table).languages,
                                                                        lang as LanguageSystemPtr,
                                                                    );
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
                                                                    let mut lang_0: *mut LanguageSystem = ::core::ptr::null_mut::<
                                                                        LanguageSystem,
                                                                    >();
                                                                    OTL_I_LANGUAGE_SYSTEM
                                                                        .init
                                                                        .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        &raw mut lang_0
                                                                    );
                                                                    (*lang_0).name = crate::sdsbuild!(
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
                                                                    parseLanguage(
                                                                        data,
                                                                        table_length,
                                                                        script_offset_0
                                                                            .wrapping_add(
                                                                                lang_sys as u32,
                                                                            ),
                                                                        lang_0,
                                                                        &raw mut (*table).features,
                                                                    );
                                                                    OTL_I_LANG_SYSTEM_LIST
                                                                        .push
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(
                                                                        &raw mut (*table).languages,
                                                                        lang_0 as LanguageSystemPtr,
                                                                    );
                                                                    k_0 = k_0.wrapping_add(1);
                                                                }
                                                                j_2 = j_2.wrapping_add(1);
                                                            }
                                                            let mut j_3: TableId = 0 as TableId;
                                                            while (j_3 as usize)
                                                                < (*table).lookups.length
                                                            {
                                                                if (**(*table)
                                                                    .lookups
                                                                    .items
                                                                    .offset(j_3 as isize))
                                                                .name
                                                                .is_null()
                                                                {
                                                                    if !(*options)
                                                                        .glyph_name_prefix
                                                                        .is_null()
                                                                    {
                                                                        let ref mut fresh5 =
                                                                            (**(*table)
                                                                                .lookups
                                                                                .items
                                                                                .offset(
                                                                                    j_3 as isize,
                                                                                ))
                                                                            .name;
                                                                        *fresh5 = crate::sdsbuild!(
                                                                            sdsempty(),
                                                                            b"lookup_",
                                                                            (*options).glyph_name_prefix,
                                                                            b"_",
                                                                            Hex2((**(*table).lookups.items.offset(j_3 as isize)).type_0.raw()),
                                                                            b"_",
                                                                            j_3 as ::core::ffi::c_int,
                                                                        );
                                                                    } else {
                                                                        let ref mut fresh6 =
                                                                            (**(*table)
                                                                                .lookups
                                                                                .items
                                                                                .offset(
                                                                                    j_3 as isize,
                                                                                ))
                                                                            .name;
                                                                        *fresh6 = crate::sdsbuild!(
                                                                            sdsempty(),
                                                                            b"lookup_",
                                                                            Hex2((**(*table).lookups.items.offset(j_3 as isize)).type_0.raw()),
                                                                            b"_",
                                                                            j_3 as ::core::ffi::c_int,
                                                                        );
                                                                    }
                                                                }
                                                                j_3 = j_3.wrapping_add(1);
                                                            }
                                                            return table;
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
    if !table.is_null() {
        TABLE_I_OTL.free.expect("non-null function pointer")(table);
    }
    return ::core::ptr::null_mut::<OtlTable>();
}
unsafe extern "C" fn otfcc_readOtl_lookup(
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
        let mut subtable: *mut Subtable = otfcc_readOtl_subtable(
            data,
            table_length,
            subtable_offset,
            (*lookup).type_0,
            max_glyphs,
            options,
        );
        OTL_I_SUBTABLE_LIST.push.expect("non-null function pointer")(
            &raw mut (*lookup).subtables,
            subtable as SubtablePtr,
        );
        j = j.wrapping_add(1);
    }
    if (*lookup).type_0 == OTL_TYPE_GSUB_EXTEND
        || (*lookup).type_0 == OTL_TYPE_GPOS_EXTEND
    {
        (*lookup).type_0 = OTL_TYPE_UNKNOWN;
        let mut j_0: TableId = 0 as TableId;
        while (j_0 as usize) < (*lookup).subtables.length {
            if !(*(*lookup).subtables.items.offset(j_0 as isize)).is_null() {
                (*lookup).type_0 = (**(*lookup).subtables.items.offset(j_0 as isize))
                    .extend
                    .type_0;
                break;
            } else {
                j_0 = j_0.wrapping_add(1);
            }
        }
        if (*lookup).type_0 != OTL_TYPE_UNKNOWN {
            let mut j_1: TableId = 0 as TableId;
            while (j_1 as usize) < (*lookup).subtables.length {
                if !(*(*lookup).subtables.items.offset(j_1 as isize)).is_null()
                    && (**(*lookup).subtables.items.offset(j_1 as isize))
                        .extend
                        .type_0
                        == (*lookup).type_0
                {
                    let mut st: *mut Subtable =
                        (**(*lookup).subtables.items.offset(j_1 as isize))
                            .extend
                            .subtable as *mut Subtable;
                    free(
                        *(*lookup).subtables.items.offset(j_1 as isize) as *mut ::core::ffi::c_void
                    );
                    let ref mut fresh0 = *(*lookup).subtables.items.offset(j_1 as isize);
                    *fresh0 = ::core::ptr::null_mut::<Subtable>();
                    let ref mut fresh1 = *(*lookup).subtables.items.offset(j_1 as isize);
                    *fresh1 = st as SubtablePtr;
                } else if !(*(*lookup).subtables.items.offset(j_1 as isize)).is_null() {
                    let mut temp: *mut Lookup = ::core::ptr::null_mut::<Lookup>();
                    OTL_I_LOOKUP_PTR.init.expect("non-null function pointer")(&raw mut temp);
                    (*temp).type_0 = (**(*lookup).subtables.items.offset(j_1 as isize))
                        .extend
                        .type_0;
                    OTL_I_SUBTABLE_LIST.push.expect("non-null function pointer")(
                        &raw mut (*temp).subtables,
                        (**(*lookup).subtables.items.offset(j_1 as isize))
                            .extend
                            .subtable as SubtablePtr,
                    );
                    otfcc_delete_lookup(temp);
                    temp = ::core::ptr::null_mut::<Lookup>();
                    free(
                        *(*lookup).subtables.items.offset(j_1 as isize) as *mut ::core::ffi::c_void
                    );
                    let ref mut fresh2 = *(*lookup).subtables.items.offset(j_1 as isize);
                    *fresh2 = ::core::ptr::null_mut::<Subtable>();
                }
                j_1 = j_1.wrapping_add(1);
            }
        } else {
            OTL_I_SUBTABLE_LIST
                .disposeDependent
                .expect("non-null function pointer")(
                &raw mut (*lookup).subtables, lookup
            );
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
pub unsafe extern "C" fn otfcc_readOtl(
    mut packet: Packet,
    mut options: *const Options,
    mut tag: u32,
    mut max_glyphs: GlyphId,
) -> *mut OtlTable {
    let mut otl: *mut OtlTable = ::core::ptr::null_mut::<OtlTable>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == tag {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: FontFilePointer = table.data as FontFilePointer;
                    let mut length: u32 = table.length;
                    otl = otfcc_readOtl_common(
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
                    if otl.is_null() {
                        if !otl.is_null() {
                            TABLE_I_OTL.free.expect("non-null function pointer")(otl);
                        }
                        otl = ::core::ptr::null_mut::<OtlTable>();
                    } else {
                        let mut j: TableId = 0 as TableId;
                        while (j as usize) < (*otl).lookups.length {
                            otfcc_readOtl_lookup(
                                data,
                                length,
                                *(*otl).lookups.items.offset(j as isize) as *mut Lookup,
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
    return ::core::ptr::null_mut::<OtlTable>();
}
