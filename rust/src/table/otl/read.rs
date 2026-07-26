#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free};



use crate::support::binio::{read_16u, read_32u};

use crate::support::options::{otfcc_Options};
use crate::support::primitives::{font_file_pointer, glyphid_t, tableid_t};
use crate::vendor::sds::{Byte, Dec5, Hex2};
use crate::font::caryll_sfnt::{otfcc_Packet, otfcc_PacketPiece};

use crate::table::otl::{otl_Feature, otl_FeatureList, otl_FeaturePtr, otl_FeatureRef, otl_LanguageSystem, otl_LanguageSystemPtr, otl_Lookup, otl_LookupPtr, otl_LookupRef, otl_LookupType, otl_Subtable, otl_SubtablePtr, otl_type_gpos_chaining, otl_type_gpos_context, otl_type_gpos_cursive, otl_type_gpos_extend, otl_type_gpos_markToBase, otl_type_gpos_markToLigature, otl_type_gpos_markToMark, otl_type_gpos_pair, otl_type_gpos_single, otl_type_gpos_unknown, otl_type_gsub_alternate, otl_type_gsub_chaining, otl_type_gsub_context, otl_type_gsub_extend, otl_type_gsub_ligature, otl_type_gsub_multiple, otl_type_gsub_reverse, otl_type_gsub_single, otl_type_gsub_unknown, otl_type_unknown, table_OTL};
use crate::table::otl::{otfcc_delete_lookup, otl_iFeatureList, otl_iFeaturePtr, otl_iFeatureRefList, otl_iLangSystemList, otl_iLanguageSystem, otl_iLookupList, otl_iLookupPtr, otl_iLookupRefList, otl_iSubtableList, table_iOTL};
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
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut subtableOffset: u32,
    mut lookupType: otl_LookupType,
    maxGlyphs: glyphid_t,
    mut options: *const otfcc_Options,
) -> *mut otl_Subtable {
    match lookupType {
        otl_type_gsub_single => {
            return otl_read_gsub_single(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gsub_multiple => {
            return otl_read_gsub_multi(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gsub_alternate => {
            return otl_read_gsub_multi(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gsub_ligature => {
            return otl_read_gsub_ligature(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gsub_chaining => {
            return otl_read_chaining(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gsub_reverse => {
            return otl_read_gsub_reverse(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gpos_chaining => {
            return otl_read_chaining(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gsub_context => {
            return otl_read_contextual(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gpos_context => {
            return otl_read_contextual(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gpos_single => {
            return otl_read_gpos_single(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gpos_pair => {
            return otl_read_gpos_pair(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gpos_cursive => {
            return otl_read_gpos_cursive(data, tableLength, subtableOffset, maxGlyphs, options);
        }
        otl_type_gpos_markToBase => {
            return otl_read_gpos_markToSingle(
                data,
                tableLength,
                subtableOffset,
                maxGlyphs,
                options,
            );
        }
        otl_type_gpos_markToMark => {
            return otl_read_gpos_markToSingle(
                data,
                tableLength,
                subtableOffset,
                maxGlyphs,
                options,
            );
        }
        otl_type_gpos_markToLigature => {
            return otl_read_gpos_markToLigature(
                data,
                tableLength,
                subtableOffset,
                maxGlyphs,
                options,
            );
        }
        otl_type_gsub_extend => {
            return otfcc_readOtl_gsub_extend(
                data,
                tableLength,
                subtableOffset,
                maxGlyphs,
                options,
            );
        }
        otl_type_gpos_extend => {
            return otfcc_readOtl_gpos_extend(
                data,
                tableLength,
                subtableOffset,
                maxGlyphs,
                options,
            );
        }
        _ => return ::core::ptr::null_mut::<otl_Subtable>(),
    };
}
unsafe extern "C" fn parseLanguage(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut base: u32,
    mut lang: *mut otl_LanguageSystem,
    mut features: *mut otl_FeatureList,
) {
    let mut rid: tableid_t = 0;
    let mut featureCount: tableid_t = 0;
    if tableLength < base.wrapping_add(6 as u32) {
        otl_iFeatureRefList
            .dispose
            .expect("non-null function pointer")(&raw mut (*lang).features);
        (*lang).requiredFeature = ::core::ptr::null::<otl_Feature>();
        return;
    } else {
        rid = read_16u(
            data.offset(base as isize)
                .offset(2 as ::core::ffi::c_int as isize) as *const u8,
        ) as tableid_t;
        if (rid as usize) < (*features).length {
            (*lang).requiredFeature = *(*features).items.offset(rid as isize) as otl_FeatureRef;
        } else {
            (*lang).requiredFeature = ::core::ptr::null::<otl_Feature>();
        }
        featureCount = read_16u(
            data.offset(base as isize)
                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
        ) as tableid_t;
        let mut j: tableid_t = 0 as tableid_t;
        while (j as ::core::ffi::c_int) < featureCount as ::core::ffi::c_int {
            let mut featureIndex: tableid_t = read_16u(
                data.offset(base as isize)
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset((2 as ::core::ffi::c_int * j as ::core::ffi::c_int) as isize)
                    as *const u8,
            ) as tableid_t;
            if (featureIndex as usize) < (*features).length {
                otl_iFeatureRefList.push.expect("non-null function pointer")(
                    &raw mut (*lang).features,
                    *(*features).items.offset(featureIndex as isize) as otl_FeatureRef,
                );
            }
            j = j.wrapping_add(1);
        }
        return;
    };
}
unsafe extern "C" fn otfcc_readOtl_common(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut lookup_type_base: otl_LookupType,
    mut options: *const otfcc_Options,
) -> *mut table_OTL {
    let mut scriptListOffset: u32 = 0;
    let mut featureListOffset: u32 = 0;
    let mut lookupListOffset: u32 = 0;
    let mut current_block: u64;
    let mut table: *mut table_OTL = (
        table_iOTL.create.expect("non-null function pointer"))();
    if !table.is_null() {
        if !(tableLength < 10 as u32) {
            scriptListOffset =
                read_16u(data.offset(4 as ::core::ffi::c_int as isize) as *const u8)
                    as u32;
            if !(tableLength < scriptListOffset.wrapping_add(2 as u32)) {
                featureListOffset =
                    read_16u(data.offset(6 as ::core::ffi::c_int as isize) as *const u8)
                        as u32;
                if !(tableLength < featureListOffset.wrapping_add(2 as u32)) {
                    lookupListOffset =
                        read_16u(data.offset(8 as ::core::ffi::c_int as isize) as *const u8)
                            as u32;
                    if !(tableLength < lookupListOffset.wrapping_add(2 as u32)) {
                        let mut lookupCount: tableid_t =
                            read_16u(data.offset(lookupListOffset as isize) as *const u8)
                                as tableid_t;
                        if !(tableLength
                            < lookupListOffset.wrapping_add(2 as u32).wrapping_add(
                                (lookupCount as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as u32,
                            ))
                        {
                            let mut j: tableid_t = 0 as tableid_t;
                            loop {
                                if !((j as ::core::ffi::c_int) < lookupCount as ::core::ffi::c_int)
                                {
                                    current_block = 12147880666119273379;
                                    break;
                                }
                                let mut lookup: *mut otl_Lookup =
                                    ::core::ptr::null_mut::<otl_Lookup>();
                                otl_iLookupPtr.init.expect("non-null function pointer")(
                                    &raw mut lookup,
                                );
                                (*lookup)._offset = lookupListOffset.wrapping_add(read_16u(
                                    data.offset(lookupListOffset as isize)
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        .offset(
                                            (2 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                                                as isize,
                                        ) as *const u8,
                                )
                                    as u32);
                                if tableLength < (*lookup)._offset.wrapping_add(6 as u32) {
                                    current_block = 2510049428056405458;
                                    break;
                                }
                                (*lookup).type_0 = otl_LookupType::from_file(
                                    lookup_type_base,
                                    read_16u(data.offset((*lookup)._offset as isize) as *const u8),
                                );
                                otl_iLookupList.push.expect("non-null function pointer")(
                                    &raw mut (*table).lookups,
                                    lookup as otl_LookupPtr,
                                );
                                j = j.wrapping_add(1);
                            }
                            match current_block {
                                2510049428056405458 => {}
                                _ => {
                                    let mut featureCount: tableid_t =
                                        read_16u(data.offset(featureListOffset as isize)
                                            as *const u8)
                                            as tableid_t;
                                    if !(tableLength
                                        < featureListOffset
                                            .wrapping_add(2 as u32)
                                            .wrapping_add(
                                                (featureCount as ::core::ffi::c_int
                                                    * 6 as ::core::ffi::c_int)
                                                    as u32,
                                            ))
                                    {
                                        let mut lnk: tableid_t = 0 as tableid_t;
                                        let mut j_0: tableid_t = 0 as tableid_t;
                                        loop {
                                            if !((j_0 as ::core::ffi::c_int)
                                                < featureCount as ::core::ffi::c_int)
                                            {
                                                current_block = 13460095289871124136;
                                                break;
                                            }
                                            let mut feature: *mut otl_Feature =
                                                ::core::ptr::null_mut::<otl_Feature>();
                                            otl_iFeaturePtr
                                                .init
                                                .expect("non-null function pointer")(
                                                &raw mut feature,
                                            );
                                            let mut tag: u32 = read_32u(
                                                data.offset(featureListOffset as isize)
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
                                            let mut featureOffset: u32 = featureListOffset
                                                .wrapping_add(read_16u(
                                                    data.offset(featureListOffset as isize)
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
                                            if tableLength
                                                < featureOffset.wrapping_add(4 as u32)
                                            {
                                                current_block = 2510049428056405458;
                                                break;
                                            }
                                            let mut lookupCount_0: tableid_t = read_16u(
                                                data.offset(featureOffset as isize)
                                                    .offset(2 as ::core::ffi::c_int as isize)
                                                    as *const u8,
                                            )
                                                as tableid_t;
                                            if tableLength
                                                < featureOffset
                                                    .wrapping_add(4 as u32)
                                                    .wrapping_add(
                                                        (lookupCount_0 as ::core::ffi::c_int
                                                            * 2 as ::core::ffi::c_int)
                                                            as u32,
                                                    )
                                            {
                                                current_block = 2510049428056405458;
                                                break;
                                            }
                                            let mut k: tableid_t = 0 as tableid_t;
                                            while (k as ::core::ffi::c_int)
                                                < lookupCount_0 as ::core::ffi::c_int
                                            {
                                                let mut lookupid: tableid_t = read_16u(
                                                    data.offset(featureOffset as isize)
                                                        .offset(4 as ::core::ffi::c_int as isize)
                                                        .offset(
                                                            (k as ::core::ffi::c_int
                                                                * 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                        as *const u8,
                                                )
                                                    as tableid_t;
                                                if (lookupid as usize) < (*table).lookups.length {
                                                    let mut lookup_0: *mut otl_Lookup = *(*table)
                                                        .lookups
                                                        .items
                                                        .offset(lookupid as isize)
                                                        as *mut otl_Lookup;
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
                                                    otl_iLookupRefList
                                                        .push
                                                        .expect("non-null function pointer")(
                                                        &raw mut (*feature).lookups,
                                                        lookup_0 as otl_LookupRef,
                                                    );
                                                }
                                                k = k.wrapping_add(1);
                                            }
                                            otl_iFeatureList
                                                .push
                                                .expect("non-null function pointer")(
                                                &raw mut (*table).features,
                                                feature as otl_FeaturePtr,
                                            );
                                            j_0 = j_0.wrapping_add(1);
                                        }
                                        match current_block {
                                            2510049428056405458 => {}
                                            _ => {
                                                let mut scriptCount: tableid_t =
                                                    read_16u(data.offset(scriptListOffset as isize)
                                                        as *const u8)
                                                        as tableid_t;
                                                if !(tableLength
                                                    < scriptListOffset
                                                        .wrapping_add(2 as u32)
                                                        .wrapping_add(
                                                            (6 as ::core::ffi::c_int
                                                                * scriptCount as ::core::ffi::c_int)
                                                                as u32,
                                                        ))
                                                {
                                                    let mut nLanguageCombinations: u32 =
                                                        0 as u32;
                                                    let mut j_1: tableid_t = 0 as tableid_t;
                                                    loop {
                                                        if !((j_1 as ::core::ffi::c_int)
                                                            < scriptCount as ::core::ffi::c_int)
                                                        {
                                                            current_block = 6528285054092551010;
                                                            break;
                                                        }
                                                        let mut scriptOffset: u32 =
                                                            scriptListOffset
                                                                .wrapping_add(read_16u(
                                                                data.offset(
                                                                    scriptListOffset as isize,
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
                                                        if tableLength
                                                            < scriptOffset
                                                                .wrapping_add(4 as u32)
                                                        {
                                                            current_block = 2510049428056405458;
                                                            break;
                                                        }
                                                        let mut defaultLangSystem: tableid_t =
                                                            read_16u(
                                                                data.offset(scriptOffset as isize)
                                                                    as *const u8,
                                                            )
                                                                as tableid_t;
                                                        nLanguageCombinations =
                                                            nLanguageCombinations.wrapping_add(
                                                                ((if defaultLangSystem
                                                                    as ::core::ffi::c_int
                                                                    != 0
                                                                {
                                                                    1 as ::core::ffi::c_int
                                                                } else {
                                                                    0 as ::core::ffi::c_int
                                                                }) + read_16u(
                                                                    data.offset(
                                                                        scriptOffset as isize,
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
                                                            let mut j_2: tableid_t = 0 as tableid_t;
                                                            while (j_2 as ::core::ffi::c_int)
                                                                < scriptCount as ::core::ffi::c_int
                                                            {
                                                                let mut tag_0: u32 = read_32u(
                                                                    data
                                                                        .offset(scriptListOffset as isize)
                                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                                        .offset(
                                                                            (6 as ::core::ffi::c_int * j_2 as ::core::ffi::c_int)
                                                                                as isize,
                                                                        ) as *const u8,
                                                                );
                                                                let mut scriptOffset_0: u32 = scriptListOffset
                                                                    .wrapping_add(
                                                                        read_16u(
                                                                            data
                                                                                .offset(scriptListOffset as isize)
                                                                                .offset(2 as ::core::ffi::c_int as isize)
                                                                                .offset(
                                                                                    (6 as ::core::ffi::c_int * j_2 as ::core::ffi::c_int)
                                                                                        as isize,
                                                                                )
                                                                                .offset(4 as ::core::ffi::c_int as isize) as *const u8,
                                                                        ) as u32,
                                                                    );
                                                                let mut defaultLangSystem_0: tableid_t = read_16u(
                                                                    data.offset(scriptOffset_0 as isize) as *const u8,
                                                                ) as tableid_t;
                                                                if defaultLangSystem_0 != 0 {
                                                                    let mut lang: *mut otl_LanguageSystem = ::core::ptr::null_mut::<
                                                                        otl_LanguageSystem,
                                                                    >();
                                                                    otl_iLanguageSystem
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
                                                                        tableLength,
                                                                        scriptOffset_0
                                                                            .wrapping_add(
                                                                                defaultLangSystem_0
                                                                                    as u32,
                                                                            ),
                                                                        lang,
                                                                        &raw mut (*table).features,
                                                                    );
                                                                    otl_iLangSystemList
                                                                        .push
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(
                                                                        &raw mut (*table).languages,
                                                                        lang as otl_LanguageSystemPtr,
                                                                    );
                                                                }
                                                                let mut langSysCount: tableid_t =
                                                                    read_16u(
                                                                        data.offset(
                                                                            scriptOffset_0 as isize,
                                                                        )
                                                                        .offset(
                                                                            2 as ::core::ffi::c_int
                                                                                as isize,
                                                                        )
                                                                            as *const u8,
                                                                    )
                                                                        as tableid_t;
                                                                let mut k_0: tableid_t =
                                                                    0 as tableid_t;
                                                                while (k_0 as ::core::ffi::c_int)
                                                                    < langSysCount
                                                                        as ::core::ffi::c_int
                                                                {
                                                                    let mut langTag: u32 = read_32u(
                                                                        data
                                                                            .offset(scriptOffset_0 as isize)
                                                                            .offset(4 as ::core::ffi::c_int as isize)
                                                                            .offset(
                                                                                (6 as ::core::ffi::c_int * k_0 as ::core::ffi::c_int)
                                                                                    as isize,
                                                                            ) as *const u8,
                                                                    );
                                                                    let mut langSys: tableid_t = read_16u(
                                                                        data
                                                                            .offset(scriptOffset_0 as isize)
                                                                            .offset(4 as ::core::ffi::c_int as isize)
                                                                            .offset(
                                                                                (6 as ::core::ffi::c_int * k_0 as ::core::ffi::c_int)
                                                                                    as isize,
                                                                            )
                                                                            .offset(4 as ::core::ffi::c_int as isize) as *const u8,
                                                                    ) as tableid_t;
                                                                    let mut lang_0: *mut otl_LanguageSystem = ::core::ptr::null_mut::<
                                                                        otl_LanguageSystem,
                                                                    >();
                                                                    otl_iLanguageSystem
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
                                                                        Byte((langTag >> 24 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((langTag >> 16 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((langTag >> 8 as ::core::ffi::c_int & 0xff as u32) as u8),
                                                                        Byte((langTag & 0xff as u32) as u8),
                                                                    );
                                                                    parseLanguage(
                                                                        data,
                                                                        tableLength,
                                                                        scriptOffset_0
                                                                            .wrapping_add(
                                                                                langSys as u32,
                                                                            ),
                                                                        lang_0,
                                                                        &raw mut (*table).features,
                                                                    );
                                                                    otl_iLangSystemList
                                                                        .push
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(
                                                                        &raw mut (*table).languages,
                                                                        lang_0 as otl_LanguageSystemPtr,
                                                                    );
                                                                    k_0 = k_0.wrapping_add(1);
                                                                }
                                                                j_2 = j_2.wrapping_add(1);
                                                            }
                                                            let mut j_3: tableid_t = 0 as tableid_t;
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
        table_iOTL.free.expect("non-null function pointer")(table);
    }
    return ::core::ptr::null_mut::<table_OTL>();
}
unsafe extern "C" fn otfcc_readOtl_lookup(
    mut data: font_file_pointer,
    mut tableLength: u32,
    mut lookup: *mut otl_Lookup,
    mut maxGlyphs: glyphid_t,
    mut options: *const otfcc_Options,
) {
    (*lookup).flags = read_16u(
        data.offset((*lookup)._offset as isize)
            .offset(2 as ::core::ffi::c_int as isize) as *const u8,
    );
    let mut subtableCount: tableid_t = read_16u(
        data.offset((*lookup)._offset as isize)
            .offset(4 as ::core::ffi::c_int as isize) as *const u8,
    ) as tableid_t;
    if subtableCount == 0
        || tableLength
            < (*lookup)._offset.wrapping_add(6 as u32).wrapping_add(
                (2 as ::core::ffi::c_int * subtableCount as ::core::ffi::c_int) as u32,
            )
    {
        (*lookup).type_0 = otl_type_unknown;
        return;
    }
    let mut j: tableid_t = 0 as tableid_t;
    while (j as ::core::ffi::c_int) < subtableCount as ::core::ffi::c_int {
        let mut subtableOffset: u32 = (*lookup)._offset.wrapping_add(read_16u(
            data.offset((*lookup)._offset as isize)
                .offset(6 as ::core::ffi::c_int as isize)
                .offset((j as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                as *const u8,
        ) as u32);
        let mut subtable: *mut otl_Subtable = otfcc_readOtl_subtable(
            data,
            tableLength,
            subtableOffset,
            (*lookup).type_0,
            maxGlyphs,
            options,
        );
        otl_iSubtableList.push.expect("non-null function pointer")(
            &raw mut (*lookup).subtables,
            subtable as otl_SubtablePtr,
        );
        j = j.wrapping_add(1);
    }
    if (*lookup).type_0 == otl_type_gsub_extend
        || (*lookup).type_0 == otl_type_gpos_extend
    {
        (*lookup).type_0 = otl_type_unknown;
        let mut j_0: tableid_t = 0 as tableid_t;
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
        if (*lookup).type_0 != otl_type_unknown {
            let mut j_1: tableid_t = 0 as tableid_t;
            while (j_1 as usize) < (*lookup).subtables.length {
                if !(*(*lookup).subtables.items.offset(j_1 as isize)).is_null()
                    && (**(*lookup).subtables.items.offset(j_1 as isize))
                        .extend
                        .type_0
                        == (*lookup).type_0
                {
                    let mut st: *mut otl_Subtable =
                        (**(*lookup).subtables.items.offset(j_1 as isize))
                            .extend
                            .subtable as *mut otl_Subtable;
                    free(
                        *(*lookup).subtables.items.offset(j_1 as isize) as *mut ::core::ffi::c_void
                    );
                    let ref mut fresh0 = *(*lookup).subtables.items.offset(j_1 as isize);
                    *fresh0 = ::core::ptr::null_mut::<otl_Subtable>();
                    let ref mut fresh1 = *(*lookup).subtables.items.offset(j_1 as isize);
                    *fresh1 = st as otl_SubtablePtr;
                } else if !(*(*lookup).subtables.items.offset(j_1 as isize)).is_null() {
                    let mut temp: *mut otl_Lookup = ::core::ptr::null_mut::<otl_Lookup>();
                    otl_iLookupPtr.init.expect("non-null function pointer")(&raw mut temp);
                    (*temp).type_0 = (**(*lookup).subtables.items.offset(j_1 as isize))
                        .extend
                        .type_0;
                    otl_iSubtableList.push.expect("non-null function pointer")(
                        &raw mut (*temp).subtables,
                        (**(*lookup).subtables.items.offset(j_1 as isize))
                            .extend
                            .subtable as otl_SubtablePtr,
                    );
                    otfcc_delete_lookup(temp);
                    temp = ::core::ptr::null_mut::<otl_Lookup>();
                    free(
                        *(*lookup).subtables.items.offset(j_1 as isize) as *mut ::core::ffi::c_void
                    );
                    let ref mut fresh2 = *(*lookup).subtables.items.offset(j_1 as isize);
                    *fresh2 = ::core::ptr::null_mut::<otl_Subtable>();
                }
                j_1 = j_1.wrapping_add(1);
            }
        } else {
            otl_iSubtableList
                .disposeDependent
                .expect("non-null function pointer")(
                &raw mut (*lookup).subtables, lookup
            );
            return;
        }
    }
    if (*lookup).type_0 == otl_type_gsub_context
    {
        (*lookup).type_0 = otl_type_gsub_chaining;
    }
    if (*lookup).type_0 == otl_type_gpos_context
    {
        (*lookup).type_0 = otl_type_gpos_chaining;
    }
}
pub unsafe extern "C" fn otfcc_readOtl(
    mut packet: otfcc_Packet,
    mut options: *const otfcc_Options,
    mut tag: u32,
    mut maxGlyphs: glyphid_t,
) -> *mut table_OTL {
    let mut otl: *mut table_OTL = ::core::ptr::null_mut::<table_OTL>();
    let mut __fortable_keep: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut __fortable_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut __notfound: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while __notfound != 0
        && __fortable_keep != 0
        && __fortable_count < packet.numTables as ::core::ffi::c_int
    {
        let mut table: otfcc_PacketPiece = *packet.pieces.offset(__fortable_count as isize);
        while __fortable_keep != 0 {
            if table.tag == tag {
                let mut __fortable_k2: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while __fortable_k2 != 0 {
                    let mut data: font_file_pointer = table.data as font_file_pointer;
                    let mut length: u32 = table.length;
                    otl = otfcc_readOtl_common(
                        data,
                        length,
                        if tag == 1196643650i32 as u32 {
                            otl_type_gsub_unknown
                        } else if tag == 1196445523i32 as u32 {
                            otl_type_gpos_unknown
                        } else {
                            otl_type_unknown
                        },
                        options,
                    );
                    if otl.is_null() {
                        if !otl.is_null() {
                            table_iOTL.free.expect("non-null function pointer")(otl);
                        }
                        otl = ::core::ptr::null_mut::<table_OTL>();
                    } else {
                        let mut j: tableid_t = 0 as tableid_t;
                        while (j as usize) < (*otl).lookups.length {
                            otfcc_readOtl_lookup(
                                data,
                                length,
                                *(*otl).lookups.items.offset(j as isize) as *mut otl_Lookup,
                                maxGlyphs,
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
    return ::core::ptr::null_mut::<table_OTL>();
}
