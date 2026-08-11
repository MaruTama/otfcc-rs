#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, strncmp};





use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::logger::{LoggerType, LOG_VL_NOTICE, LOG_VL_PROGRESS, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{TableId};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::table::otl::{Feature, LanguageSystem, Lookup, LookupType, Subtable, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_EXTEND, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GPOS_UNKNOWN, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_EXTEND, OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OTL_TYPE_GSUB_UNKNOWN, OtlTable};
use crate::table::otl::subtables::BuildHeuristics;
use crate::bk::bkblock::{bk_new_block_from_buffer};
use crate::bk::bkgraph::{bk_build_block};
use crate::table::otl::subtables::chaining::build::{otfcc_chaining_lookup_is_contextual_lookup};
use crate::table::otl::subtables::chaining::classifier::{otfcc_classified_build_chaining};
use crate::table::otl::subtables::gpos_cursive::{otfcc_build_gpos_cursive};
use crate::table::otl::subtables::gpos_mark_to_ligature::{otfcc_build_gpos_mark_to_ligature};
use crate::table::otl::subtables::gpos_mark_to_single::{otfcc_build_gpos_mark_to_single};
use crate::table::otl::subtables::gpos_pair::{otfcc_build_gpos_pair};
use crate::table::otl::subtables::gpos_single::{otfcc_build_gpos_single};
use crate::table::otl::subtables::gsub_ligature::{otfcc_build_gsub_ligature_subtable};
use crate::table::otl::subtables::gsub_multi::{otfcc_build_gsub_multi_subtable_split};
use crate::table::otl::subtables::gsub_reverse::{otfcc_build_gsub_reverse};
use crate::table::otl::subtables::gsub_single::{otfcc_build_gsub_single_subtable};
use crate::vendor::sds::{sdsempty};
pub type OtlBuilder =
    Option<unsafe extern "C" fn(*const Subtable, BuildHeuristics) -> *mut Buffer>;
pub type OtlSplitBuilder = Option<
    unsafe extern "C" fn(
        *const Subtable,
        BuildHeuristics,
        *mut TableId,
    ) -> *mut *mut Buffer,
>;
pub const LARGE_SUBTABLE_LIMIT: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
fn feature_name_to_tag(name: &[u8]) -> u32 {
    let mut tag: u32 = 0 as u32;
    if name.len() > 0 as usize {
        tag |= ((name[0 as usize] as u8 as ::core::ffi::c_int)
            << 24 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as u32;
    }
    if name.len() > 1 as usize {
        tag |= ((name[1 as usize] as u8 as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32;
    }
    if name.len() > 2 as usize {
        tag |= ((name[2 as usize] as u8 as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
    }
    if name.len() > 3 as usize {
        tag |= ((name[3 as usize] as u8 as ::core::ffi::c_int)
            << 0 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as u32;
    }
    return tag;
}
unsafe extern "C" fn _declare_lookup_writer(
    mut type_0: LookupType,
    mut fn_0: OtlBuilder,
    mut lookup: *const Lookup,
    mut subtables: *mut *mut *mut Buffer,
    mut last_offset: *mut usize,
    mut prefer_extension_for_this_lut: *mut bool,
    mut heuristics: BuildHeuristics,
) -> TableId {
    if (*lookup).type_0 == type_0 {
        *subtables = __caryll_allocate_clean(
            (::core::mem::size_of::<*mut Buffer>() as usize)
                .wrapping_mul((*lookup).subtables.len()),
            38 as ::core::ffi::c_ulong,
        ) as *mut *mut Buffer;
        let mut total_buf_size_short: usize = 0 as usize;
        let mut total_buf_size_ext: usize = 0 as usize;
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*lookup).subtables.len() {
            let mut buf: *mut Buffer = fn_0.expect("non-null function pointer")(
                (&(*lookup).subtables)[j as usize] as *const Subtable,
                heuristics,
            );
            let ref mut fresh1 = *(*subtables).offset(j as isize);
            *fresh1 = buf;
            total_buf_size_short = total_buf_size_short.wrapping_add((*buf).size);
            total_buf_size_ext = total_buf_size_ext.wrapping_add(8 as usize);
            j = j.wrapping_add(1);
        }
        if total_buf_size_short > LARGE_SUBTABLE_LIMIT as usize {
            *last_offset = (*last_offset).wrapping_add(total_buf_size_ext);
            *prefer_extension_for_this_lut = true;
        } else {
            *last_offset = (*last_offset).wrapping_add(total_buf_size_short);
            *prefer_extension_for_this_lut = false;
        }
        return (*lookup).subtables.len() as TableId;
    }
    return 0 as TableId;
}
unsafe extern "C" fn _declare_lookup_writer_split(
    mut type_0: LookupType,
    mut fn_0: OtlSplitBuilder,
    mut lookup: *const Lookup,
    mut subtables: *mut *mut *mut Buffer,
    mut last_offset: *mut usize,
    mut prefer_extension_for_this_lut: *mut bool,
    mut heuristics: BuildHeuristics,
) -> TableId {
    if (*lookup).type_0 == type_0 {
        let mut buffers: *mut *mut Buffer = ::core::ptr::null_mut::<*mut Buffer>();
        let mut total: TableId = 0 as TableId;
        let mut total_buf_size_short: usize = 0 as usize;
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*lookup).subtables.len() {
            let mut n_part: TableId = 0 as TableId;
            let mut part: *mut *mut Buffer = fn_0.expect("non-null function pointer")(
                (&(*lookup).subtables)[j as usize] as *const Subtable,
                heuristics,
                &raw mut n_part,
            );
            let mut k: TableId = 0 as TableId;
            while (k as ::core::ffi::c_int) < n_part as ::core::ffi::c_int {
                buffers = __caryll_reallocate(
                    buffers as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<*mut Buffer>() as usize).wrapping_mul(
                        (total as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
                    ),
                    81 as ::core::ffi::c_ulong,
                ) as *mut *mut Buffer;
                let ref mut fresh2 = *buffers.offset(total as isize);
                *fresh2 = *part.offset(k as isize);
                total_buf_size_short =
                    total_buf_size_short.wrapping_add((**part.offset(k as isize)).size);
                total = total.wrapping_add(1);
                k = k.wrapping_add(1);
            }
            free(part as *mut ::core::ffi::c_void);
            part = ::core::ptr::null_mut::<*mut Buffer>();
            j = j.wrapping_add(1);
        }
        *subtables = buffers;
        if total_buf_size_short > LARGE_SUBTABLE_LIMIT as usize {
            *last_offset = (*last_offset)
                .wrapping_add((8 as ::core::ffi::c_int * total as ::core::ffi::c_int) as usize);
            *prefer_extension_for_this_lut = true;
        } else {
            *last_offset = (*last_offset).wrapping_add(total_buf_size_short);
            *prefer_extension_for_this_lut = false;
        }
        return total;
    }
    return 0 as TableId;
}
unsafe extern "C" fn _build_lookup(
    mut lookup: *const Lookup,
    mut subtables: *mut *mut *mut Buffer,
    mut last_offset: *mut usize,
    mut prefer_extension_for_this_lut: *mut bool,
    mut heuristics: BuildHeuristics,
) -> TableId {
    if (*lookup).type_0 == OTL_TYPE_GPOS_CHAINING
        || (*lookup).type_0 == OTL_TYPE_GSUB_CHAINING
    {
        return otfcc_classified_build_chaining(lookup, subtables, last_offset);
    }
    let mut written: TableId = 0 as TableId;
    if written == 0 {
        written = _declare_lookup_writer(
            OTL_TYPE_GSUB_SINGLE,
            Some(
                otfcc_build_gsub_single_subtable
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                    ) -> *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer_split(
            OTL_TYPE_GSUB_MULTIPLE,
            Some(
                otfcc_build_gsub_multi_subtable_split
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                        *mut TableId,
                    ) -> *mut *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer_split(
            OTL_TYPE_GSUB_ALTERNATE,
            Some(
                otfcc_build_gsub_multi_subtable_split
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                        *mut TableId,
                    ) -> *mut *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            OTL_TYPE_GSUB_LIGATURE,
            Some(
                otfcc_build_gsub_ligature_subtable
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                    ) -> *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            OTL_TYPE_GSUB_REVERSE,
            Some(
                otfcc_build_gsub_reverse
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                    ) -> *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            OTL_TYPE_GPOS_SINGLE,
            Some(
                otfcc_build_gpos_single
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                    ) -> *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            OTL_TYPE_GPOS_PAIR,
            Some(
                otfcc_build_gpos_pair
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                    ) -> *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            OTL_TYPE_GPOS_CURSIVE,
            Some(
                otfcc_build_gpos_cursive
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                    ) -> *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            OTL_TYPE_GPOS_MARK_TO_BASE,
            Some(
                otfcc_build_gpos_mark_to_single
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                    ) -> *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            OTL_TYPE_GPOS_MARK_TO_MARK,
            Some(
                otfcc_build_gpos_mark_to_single
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                    ) -> *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    if written == 0 {
        written = _declare_lookup_writer(
            OTL_TYPE_GPOS_MARK_TO_LIGATURE,
            Some(
                otfcc_build_gpos_mark_to_ligature
                    as unsafe extern "C" fn(
                        *const Subtable,
                        BuildHeuristics,
                    ) -> *mut Buffer,
            ),
            lookup,
            subtables,
            last_offset,
            prefer_extension_for_this_lut,
            heuristics,
        );
    }
    return written;
}
unsafe extern "C" fn get_lookup_heuristics(
    mut table: *const OtlTable,
    mut lut: *const Lookup,
) -> BuildHeuristics {
    let mut heu: BuildHeuristics = BuildHeuristics::empty();
    if (*lut).type_0 == OTL_TYPE_GSUB_SINGLE
    {
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*table).features.len() {
            let fea: *const Feature = &raw const *(&(*table).features)[j as usize];
            if !(feature_name_to_tag(&(*fea).name) != crate::tag::TAG_VERT) {
                let mut k: TableId = 0 as TableId;
                while (k as usize) < (*fea).lookups.len() {
                    if (&(*fea).lookups)[k as usize] == lut {
                        heu.insert(BuildHeuristics::GSUB_VERT);
                    }
                    k = k.wrapping_add(1);
                }
            }
            j = j.wrapping_add(1);
        }
    }
    return heu;
}
unsafe extern "C" fn write_otl_lookups(
    mut table: *const OtlTable,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut BkBlock {
    let mut subtables: *mut *mut *mut Buffer =
        ::core::ptr::null_mut::<*mut *mut Buffer>();
    subtables = __caryll_allocate_clean(
        (::core::mem::size_of::<*mut *mut Buffer>() as usize)
            .wrapping_mul((*table).lookups.len()),
        150 as ::core::ffi::c_ulong,
    ) as *mut *mut *mut Buffer;
    let mut prefer_ext_for_this_lut: *mut bool = ::core::ptr::null_mut::<bool>();
    let mut subtable_quantity: *mut TableId = ::core::ptr::null_mut::<TableId>();
    subtable_quantity = __caryll_allocate_clean(
        (::core::mem::size_of::<TableId>() as usize).wrapping_mul((*table).lookups.len()),
        153 as ::core::ffi::c_ulong,
    ) as *mut TableId;
    prefer_ext_for_this_lut = __caryll_allocate_clean(
        (::core::mem::size_of::<bool>() as usize).wrapping_mul((*table).lookups.len()),
        154 as ::core::ffi::c_ulong,
    ) as *mut bool;
    let mut last_offset: usize = 0 as usize;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*table).lookups.len() {
        let lookup: *const Lookup = &raw const *(&(*table).lookups)[j as usize];
        let mut heu: BuildHeuristics = get_lookup_heuristics(table, lookup);
        (*(*options).logger)
            .log_sds
            .expect("non-null function pointer")(
            (*options).logger as *mut ILogger,
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            crate::sdsbuild!(
                sdsempty(),
                b"Building lookup ",
                &(*lookup).name,
                b" (",
                j as ::core::ffi::c_int,
                b"/",
                (*table).lookups.len() as u32,
                b")\n",
            ),
        );
        *subtable_quantity.offset(j as isize) = _build_lookup(
            lookup,
            subtables.offset(j as isize) as *mut *mut *mut Buffer,
            &raw mut last_offset,
            prefer_ext_for_this_lut.offset(j as isize) as *mut bool,
            heu,
        );
        j = j.wrapping_add(1);
    }
    let mut header_size: usize =
        (2 as usize).wrapping_add((2 as usize).wrapping_mul((*table).lookups.len()));
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < (*table).lookups.len() {
        if *subtable_quantity.offset(j_0 as isize) != 0 {
            header_size = header_size.wrapping_add(
                (6 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *subtable_quantity.offset(j_0 as isize) as ::core::ffi::c_int)
                    as usize,
            );
        }
        j_0 = j_0.wrapping_add(1);
    }
    let mut use_extended: bool = last_offset >= (0xff00 as usize).wrapping_sub(header_size);
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*table).lookups.len()) as u32)]);
    let mut j_1: TableId = 0 as TableId;
    while (j_1 as usize) < (*table).lookups.len() {
        if *subtable_quantity.offset(j_1 as isize) == 0 {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_NOTICE,
                LoggerType::Info,
                crate::sdsbuild!(
                    sdsempty(),
                    b"Lookup ",
                    &(*(&(*table).lookups)[j_1 as usize]).name,
                    b" is empty.\n",
                ),
            );
        }
        let lookup_0: *const Lookup = &raw const *(&(*table).lookups)[j_1 as usize];
        let can_be_contextual: bool = otfcc_chaining_lookup_is_contextual_lookup(lookup_0);
        let use_extended_for_it: bool = use_extended as ::core::ffi::c_int != 0
            || *prefer_ext_for_this_lut.offset(j_1 as isize) as ::core::ffi::c_int != 0;
        if use_extended_for_it {
            (*(*options).logger)
                .log_sds
                .expect("non-null function pointer")(
                (*options).logger as *mut ILogger,
                LOG_VL_NOTICE,
                LoggerType::Info,
                crate::sdsbuild!(
                    sdsempty(),
                    b"[OTFCC-fea] Using extended OpenType table layout for ",
                    tag,
                    b"/",
                    &(*lookup_0).name,
                    b".\n",
                ),
            );
        }
        // The format number the file wants, which is the lookup type with its
        // table's base taken back off -- `LookupType::file_format`, the
        // same nested comparison C spelled out here and again below.
        let mut lookup_type: u16 = (if use_extended_for_it {
            if (*lookup_0).type_0 > OTL_TYPE_GPOS_UNKNOWN {
                OTL_TYPE_GPOS_EXTEND.file_format()
            } else if (*lookup_0).type_0 > OTL_TYPE_GSUB_UNKNOWN {
                OTL_TYPE_GSUB_EXTEND.file_format()
            } else {
                0
            }
        } else {
            (*lookup_0)
                .type_0
                .file_format()
                .wrapping_sub(can_be_contextual as u32)
        }) as u16;
        let mut blk: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (lookup_type as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*lookup_0).flags as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, (*subtable_quantity.offset(j_1 as isize) as ::core::ffi::c_int) as u32)]);
        let mut k: TableId = 0 as TableId;
        while (k as ::core::ffi::c_int)
            < *subtable_quantity.offset(j_1 as isize) as ::core::ffi::c_int
        {
            if use_extended_for_it {
                let mut extension_lookup_type: u16 = (*lookup_0)
                    .type_0
                    .file_format()
                    .wrapping_sub(can_be_contextual as u32) as u16;
                let mut stub: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, 1 as u32), bk_int(BkCellType::B16, (extension_lookup_type as ::core::ffi::c_int) as u32), bk_ptr(BkCellType::P32, bk_new_block_from_buffer(*(*subtables.offset(j_1 as isize)).offset(k as isize)))]);
                bk_push(blk, &[bk_ptr(BkCellType::P16, stub)]);
            } else {
                bk_push(blk, &[bk_ptr(BkCellType::P16, bk_new_block_from_buffer(*(*subtables.offset(j_1 as isize)).offset(k as isize)))]);
            }
            k = k.wrapping_add(1);
        }
        bk_push(blk, &[bk_int(BkCellType::B16, 0 as u32)]);
        bk_push(root, &[bk_ptr(BkCellType::P16, blk)]);
        free(*subtables.offset(j_1 as isize) as *mut ::core::ffi::c_void);
        let ref mut fresh0 = *subtables.offset(j_1 as isize);
        *fresh0 = ::core::ptr::null_mut::<*mut Buffer>();
        j_1 = j_1.wrapping_add(1);
    }
    free(subtables as *mut ::core::ffi::c_void);
    subtables = ::core::ptr::null_mut::<*mut *mut Buffer>();
    free(subtable_quantity as *mut ::core::ffi::c_void);
    subtable_quantity = ::core::ptr::null_mut::<TableId>();
    free(prefer_ext_for_this_lut as *mut ::core::ffi::c_void);
    prefer_ext_for_this_lut = ::core::ptr::null_mut::<bool>();
    return root;
}
unsafe extern "C" fn write_otl_features(
    mut table: *const OtlTable,
    mut _options: *const Options,
) -> *mut BkBlock {
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, ((*table).features.len()) as u32)]);
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*table).features.len() {
        let mut fea: *mut BkBlock = bk_new_block(&[bk_ptr(BkCellType::P16, ::core::ptr::null_mut()), bk_int(BkCellType::B16, ((*(&(*table).features)[j as usize])
                .lookups
                .len()) as u32)]);
        let mut k: TableId = 0 as TableId;
        while (k as usize)
            < (*(&(*table).features)[j as usize])
                .lookups
                .len()
        {
            let mut l: TableId = 0 as TableId;
            while (l as usize) < (*table).lookups.len() {
                if (&(*(&(*table).features)[j as usize])
                    .lookups)[k as usize]
                    == &raw const *(&(*table).lookups)[l as usize]
                {
                    bk_push(fea, &[bk_int(BkCellType::B16, (l as ::core::ffi::c_int) as u32)]);
                    break;
                } else {
                    l = l.wrapping_add(1);
                }
            }
            k = k.wrapping_add(1);
        }
        bk_push(root, &[bk_int(BkCellType::B32, (feature_name_to_tag(&(*(&(*table).features)[j as usize]).name)) as u32), bk_ptr(BkCellType::P16, fea)]);
        j = j.wrapping_add(1);
    }
    return root;
}
unsafe extern "C" fn feature_index(
    mut feature: *const Feature,
    mut table: *const OtlTable,
) -> TableId {
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*table).features.len() {
        if &raw const *(&(*table).features)[j as usize] == feature {
            return j;
        }
        j = j.wrapping_add(1);
    }
    return 0xffff as TableId;
}
unsafe extern "C" fn write_language(
    mut lang: *const LanguageSystem,
    mut table: *const OtlTable,
) -> *mut BkBlock {
    if lang.is_null() {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_ptr(BkCellType::P16, ::core::ptr::null_mut()), bk_int(BkCellType::B16, (feature_index((*lang).required_feature as *const Feature, table) as ::core::ffi::c_int) as u32), bk_int(BkCellType::B16, ((*lang).features.len()) as u32)]);
    let mut k: TableId = 0 as TableId;
    while (k as usize) < (*lang).features.len() {
        bk_push(root, &[bk_int(BkCellType::B16, (feature_index(
                (&(*lang).features)[k as usize] as *const Feature,
                table,
            ) as ::core::ffi::c_int) as u32)]);
        k = k.wrapping_add(1);
    }
    return root;
}
#[allow(improper_ctypes_definitions)] // internal call only, never crosses FFI
unsafe extern "C" fn write_script(
    mut dl: *const LanguageSystem,
    mut ll: &[*const LanguageSystem],
    mut table: *const OtlTable,
) -> *mut BkBlock {
    let mut root: *mut BkBlock = bk_new_block(&[bk_ptr(BkCellType::P16, write_language(dl, table)), bk_int(BkCellType::B16, (ll.len()) as u32)]);
    let mut j: TableId = 0 as TableId;
    while (j as usize) < ll.len() {
        let tag: &[u8] = ::core::slice::from_raw_parts(
            (*ll[j as usize]).name.as_ptr().offset(5 as ::core::ffi::c_int as isize),
            4 as usize,
        );
        bk_push(root, &[bk_int(BkCellType::B32, (feature_name_to_tag(tag)) as u32), bk_ptr(BkCellType::P16, write_language(ll[j as usize], table))]);
        j = j.wrapping_add(1);
    }
    return root;
}
unsafe extern "C" fn write_otl_script_and_languages(
    mut table: *const OtlTable,
    mut _options: *const Options,
) -> *mut BkBlock {
    // Groups languages by script tag (the first 4 bytes of `language.name`),
    // tracking each script's default (dflt/DFLT) language separately from
    // its other languages, in the order languages are first seen. Unlike
    // every other uthash instance converted so far in this migration, the
    // original C here never calls `HASH_SORT` before its `HASH_ITER` --
    // output order is insertion order, not tag order, so `BTreeMap` (which
    // this migration has used for every prior instance) is the wrong
    // container. A plain `Vec` with a linear "already seen" scan preserves
    // insertion order directly; the number of distinct scripts in a real
    // font is small (typically single digits), so the O(n) scan costs
    // nothing observable -- not worth introducing an `indexmap` dependency
    // for a handful of entries (that crate remains the intended tool for
    // the much larger order-dependent uthash tables noted in rust/README.md).
    //
    // A later language with the same script tag whose name is *also*
    // dflt/DFLT silently overwrites the script's recorded default -- the
    // original never guarded against a second default and neither does
    // this rewrite; not a case this function warns about.
    struct ScriptGroup {
        tag: Vec<u8>,
        default_language: *const LanguageSystem,
        languages: Vec<*const LanguageSystem>,
    }
    let mut scripts: Vec<ScriptGroup> = Vec::new();
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*table).languages.len() {
        let language: *const LanguageSystem = &raw const *(&(*table).languages)[j as usize];
        let script_tag: Vec<u8> =
            ::core::slice::from_raw_parts((*language).name.as_ptr(), 4 as usize).to_vec();
        let is_default: bool = strncmp(
            (*language).name.as_ptr().offset(5 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            b"DFLT\0" as *const u8 as *const ::core::ffi::c_char,
            4 as usize,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                (*language).name.as_ptr().offset(5 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
                b"dflt\0" as *const u8 as *const ::core::ffi::c_char,
                4 as usize,
            ) == 0 as ::core::ffi::c_int;
        let mut found: Option<usize> = None;
        for (idx, group) in scripts.iter().enumerate() {
            if group.tag == script_tag {
                found = Some(idx);
                break;
            }
        }
        match found {
            Some(idx) => {
                if is_default {
                    scripts[idx].default_language = language;
                } else {
                    scripts[idx].languages.push(language);
                }
            }
            None => {
                if is_default {
                    scripts.push(ScriptGroup {
                        tag: script_tag,
                        default_language: language,
                        languages: Vec::new(),
                    });
                } else {
                    scripts.push(ScriptGroup {
                        tag: script_tag,
                        default_language: ::core::ptr::null::<LanguageSystem>(),
                        languages: vec![language],
                    });
                }
            }
        }
        j = j.wrapping_add(1);
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (scripts.len()) as u32)]);
    for group in &scripts {
        bk_push(root, &[bk_int(BkCellType::B32, (feature_name_to_tag(&group.tag)) as u32), bk_ptr(BkCellType::P16, write_script(group.default_language, &group.languages, table))]);
    }
    return root;
}
pub unsafe extern "C" fn otfcc_build_otl(
    mut table: Option<&OtlTable>,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut Buffer {
    let table: *const OtlTable = table.map_or(::core::ptr::null(), |t| t as *const OtlTable);
    if table.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    (*(*options).logger)
        .start_sds
        .expect("non-null function pointer")(
        (*options).logger as *mut ILogger,
        crate::sdsbuild!(sdsempty(), tag),
    );
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let mut lookups: *mut BkBlock = write_otl_lookups(table, options, tag);
        let mut features: *mut BkBlock = write_otl_features(table, options);
        let mut languages: *mut BkBlock = write_otl_script_and_languages(table, options);
        let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B32, 0x10000 as u32), bk_ptr(BkCellType::P16, languages), bk_ptr(BkCellType::P16, features), bk_ptr(BkCellType::P16, lookups)]);
        buf = bk_build_block(root);
        ___loggedstep_v = false;
        (*(*options).logger)
            .finish
            .expect("non-null function pointer")((*options).logger as *mut ILogger);
    }
    return buf;
}
