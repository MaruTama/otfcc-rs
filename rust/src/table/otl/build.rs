#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{exit, free, malloc, memcmp, memset, strlen, strncmp};





use crate::support::alloc::{__caryll_allocate_clean, __caryll_reallocate};
use crate::logger::{LoggerType, LOG_VL_NOTICE, LOG_VL_PROGRESS, ILogger};
use crate::support::buffer::{Buffer};
use crate::support::options::{Options};
use crate::support::primitives::{TableId};
use crate::vendor::sds::{SDS_TYPE_16, SDS_TYPE_32, SDS_TYPE_5, SDS_TYPE_64, SDS_TYPE_8, SDS_TYPE_BITS, SDS_TYPE_MASK, SdsRaw, SdsHdr16, SdsHdr32, SdsHdr64, SdsHdr8};
use crate::bk::bkblock::{BkCellType, BkBlock, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::support::{NULL};
use crate::table::otl::{Feature, FeaturePtr, LanguageSystem, Lookup, LookupRef, LookupType, Subtable, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE, OTL_TYPE_GPOS_EXTEND, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE, OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GPOS_UNKNOWN, OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_EXTEND, OTL_TYPE_GSUB_LIGATURE, OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OTL_TYPE_GSUB_UNKNOWN, OtlTable};
use crate::table::otl::subtables::BuildHeuristics;
use crate::vendor::uthash::{HASH_BKT_CAPACITY_THRESH, HASH_INITIAL_NUM_BUCKETS, HASH_INITIAL_NUM_BUCKETS_LOG2, HASH_SIGNATURE, UtHashBucket, UtHashHandle, UtHashTable};
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
use crate::vendor::sds::{sdsempty, sdsfree, sdsnewlen};
pub type OtlBuilder =
    Option<unsafe extern "C" fn(*const Subtable, BuildHeuristics) -> *mut Buffer>;
pub type OtlSplitBuilder = Option<
    unsafe extern "C" fn(
        *const Subtable,
        BuildHeuristics,
        *mut TableId,
    ) -> *mut *mut Buffer,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScriptStatHash {
    pub tag: SdsRaw,
    pub lc: u16,
    pub dl: *mut LanguageSystem,
    pub ll: *mut *mut LanguageSystem,
    pub hh: UtHashHandle,
}
#[inline]
unsafe extern "C" fn sdslen(s: SdsRaw) -> usize {
    let mut flags: ::core::ffi::c_uchar =
        *s.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar;
    match flags as ::core::ffi::c_int & SDS_TYPE_MASK {
        SDS_TYPE_5 => return (flags as ::core::ffi::c_int >> SDS_TYPE_BITS) as usize,
        SDS_TYPE_8 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr8>() as isize))
                as *mut SdsHdr8))
                .len as usize;
        }
        SDS_TYPE_16 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr16>() as isize))
                as *mut SdsHdr16))
                .len as usize;
        }
        SDS_TYPE_32 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr32>() as isize))
                as *mut SdsHdr32))
                .len as usize;
        }
        SDS_TYPE_64 => {
            return (*(s.offset(-(::core::mem::size_of::<SdsHdr64>() as isize))
                as *mut SdsHdr64))
                .len as usize;
        }
        _ => {}
    }
    return 0 as usize;
}
pub const LARGE_SUBTABLE_LIMIT: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe extern "C" fn feature_name_to_tag(name: SdsRaw) -> u32 {
    let mut tag: u32 = 0 as u32;
    if sdslen(name) > 0 as usize {
        tag |= ((*name.offset(0 as ::core::ffi::c_int as isize) as u8 as ::core::ffi::c_int)
            << 24 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as u32;
    }
    if sdslen(name) > 1 as usize {
        tag |= ((*name.offset(1 as ::core::ffi::c_int as isize) as u8 as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as u32;
    }
    if sdslen(name) > 2 as usize {
        tag |= ((*name.offset(2 as ::core::ffi::c_int as isize) as u8 as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int) as u32;
    } else {
        tag |=
            ((' ' as i32 as u8 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as u32;
    }
    if sdslen(name) > 3 as usize {
        tag |= ((*name.offset(3 as ::core::ffi::c_int as isize) as u8 as ::core::ffi::c_int)
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
            let fea: *const Feature = (&(*table).features)[j as usize];
            if !(feature_name_to_tag((*fea).name) != 1986359924i32 as u32) {
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
        let lookup: *mut Lookup = (&(*table).lookups)[j as usize];
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
                (*lookup).name,
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
                    (*(&(*table).lookups)[j_1 as usize]).name,
                    b" is empty.\n",
                ),
            );
        }
        let lookup_0: *mut Lookup = (&(*table).lookups)[j_1 as usize];
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
                    (*lookup_0).name,
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
                    == (&(*table).lookups)[l as usize] as LookupRef
                {
                    bk_push(fea, &[bk_int(BkCellType::B16, (l as ::core::ffi::c_int) as u32)]);
                    break;
                } else {
                    l = l.wrapping_add(1);
                }
            }
            k = k.wrapping_add(1);
        }
        bk_push(root, &[bk_int(BkCellType::B32, (feature_name_to_tag((*(&(*table).features)[j as usize]).name)) as u32), bk_ptr(BkCellType::P16, fea)]);
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
        if (&(*table).features)[j as usize] == feature as FeaturePtr {
            return j;
        }
        j = j.wrapping_add(1);
    }
    return 0xffff as TableId;
}
unsafe extern "C" fn write_language(
    mut lang: *mut LanguageSystem,
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
unsafe extern "C" fn write_script(
    mut script: *mut ScriptStatHash,
    mut table: *const OtlTable,
) -> *mut BkBlock {
    let mut root: *mut BkBlock = bk_new_block(&[bk_ptr(BkCellType::P16, write_language((*script).dl, table)), bk_int(BkCellType::B16, ((*script).lc as ::core::ffi::c_int) as u32)]);
    let mut j: TableId = 0 as TableId;
    while (j as ::core::ffi::c_int) < (*script).lc as ::core::ffi::c_int {
        let mut tag: SdsRaw = sdsnewlen(
            (**(*script).ll.offset(j as isize))
                .name
                .offset(5 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            4 as usize,
        );
        bk_push(root, &[bk_int(BkCellType::B32, (feature_name_to_tag(tag)) as u32), bk_ptr(BkCellType::P16, write_language(*(*script).ll.offset(j as isize), table))]);
        sdsfree(tag);
        j = j.wrapping_add(1);
    }
    return root;
}
unsafe extern "C" fn write_otl_script_and_languages(
    mut table: *const OtlTable,
    mut _options: *const Options,
) -> *mut BkBlock {
    let mut h: *mut ScriptStatHash = ::core::ptr::null_mut::<ScriptStatHash>();
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*table).languages.len() {
        let language: *mut LanguageSystem = (&(*table).languages)[j as usize];
        let mut script_tag: SdsRaw =
            sdsnewlen((*language).name as *const ::core::ffi::c_void, 4 as usize);
        let mut is_default: bool = strncmp(
            (*language).name.offset(5 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            b"DFLT\0" as *const u8 as *const ::core::ffi::c_char,
            4 as usize,
        ) == 0 as ::core::ffi::c_int
            || strncmp(
                (*language).name.offset(5 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
                b"dflt\0" as *const u8 as *const ::core::ffi::c_char,
                4 as usize,
            ) == 0 as ::core::ffi::c_int;
        let mut s: *mut ScriptStatHash = ::core::ptr::null_mut::<ScriptStatHash>();
        let mut _hf_hashv: ::core::ffi::c_uint = 0;
        let mut _hj_i: ::core::ffi::c_uint = 0;
        let mut _hj_j: ::core::ffi::c_uint = 0;
        let mut _hj_k: ::core::ffi::c_uint = 0;
        let mut _hj_key: *const ::core::ffi::c_uchar = script_tag as *const ::core::ffi::c_uchar;
        _hf_hashv = 0xfeedbeef as ::core::ffi::c_uint;
        _hj_j = 0x9e3779b9 as ::core::ffi::c_uint;
        _hj_i = _hj_j;
        _hj_k = strlen(script_tag as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
        while _hj_k >= 12 as ::core::ffi::c_uint {
            _hj_i = _hj_i.wrapping_add(
                (*_hj_key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_j = _hj_j.wrapping_add(
                (*_hj_key.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hf_hashv = _hf_hashv.wrapping_add(
                (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                    .wrapping_add(
                        (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    ),
            );
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
            _hj_key = _hj_key.offset(12 as ::core::ffi::c_int as isize);
            _hj_k = _hj_k.wrapping_sub(12 as ::core::ffi::c_uint);
        }
        _hf_hashv = _hf_hashv
            .wrapping_add(strlen(script_tag as *const ::core::ffi::c_char) as ::core::ffi::c_uint);
        let mut current_block_50: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 14823687005023999748;
            }
            10 => {
                current_block_50 = 14823687005023999748;
            }
            9 => {
                current_block_50 = 2331656302760911566;
            }
            8 => {
                current_block_50 = 17352930546238167574;
            }
            7 => {
                current_block_50 = 13740184123479751145;
            }
            6 => {
                current_block_50 = 14860613234516215618;
            }
            5 => {
                current_block_50 = 13669816829141938816;
            }
            4 => {
                current_block_50 = 7711255570521815756;
            }
            3 => {
                current_block_50 = 8659757378588889400;
            }
            2 => {
                current_block_50 = 16517549058459909004;
            }
            1 => {
                current_block_50 = 640113823387602610;
            }
            _ => {
                current_block_50 = 15004371738079956865;
            }
        }
        match current_block_50 {
            14823687005023999748 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 2331656302760911566;
            }
            _ => {}
        }
        match current_block_50 {
            2331656302760911566 => {
                _hf_hashv = _hf_hashv.wrapping_add(
                    (*_hj_key.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 17352930546238167574;
            }
            _ => {}
        }
        match current_block_50 {
            17352930546238167574 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 13740184123479751145;
            }
            _ => {}
        }
        match current_block_50 {
            13740184123479751145 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 14860613234516215618;
            }
            _ => {}
        }
        match current_block_50 {
            14860613234516215618 => {
                _hj_j = _hj_j.wrapping_add(
                    (*_hj_key.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 13669816829141938816;
            }
            _ => {}
        }
        match current_block_50 {
            13669816829141938816 => {
                _hj_j =
                    _hj_j
                        .wrapping_add(*_hj_key.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                current_block_50 = 7711255570521815756;
            }
            _ => {}
        }
        match current_block_50 {
            7711255570521815756 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int,
                );
                current_block_50 = 8659757378588889400;
            }
            _ => {}
        }
        match current_block_50 {
            8659757378588889400 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 16 as ::core::ffi::c_int,
                );
                current_block_50 = 16517549058459909004;
            }
            _ => {}
        }
        match current_block_50 {
            16517549058459909004 => {
                _hj_i = _hj_i.wrapping_add(
                    (*_hj_key.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                current_block_50 = 640113823387602610;
            }
            _ => {}
        }
        match current_block_50 {
            640113823387602610 => {
                _hj_i =
                    _hj_i
                        .wrapping_add(*_hj_key.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
            }
            _ => {}
        }
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 13 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 8 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 13 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 12 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 16 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 5 as ::core::ffi::c_int;
        _hj_i = _hj_i.wrapping_sub(_hj_j);
        _hj_i = _hj_i.wrapping_sub(_hf_hashv);
        _hj_i ^= _hf_hashv >> 3 as ::core::ffi::c_int;
        _hj_j = _hj_j.wrapping_sub(_hf_hashv);
        _hj_j = _hj_j.wrapping_sub(_hj_i);
        _hj_j ^= _hj_i << 10 as ::core::ffi::c_int;
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
        _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
        _hf_hashv ^= _hj_j >> 15 as ::core::ffi::c_int;
        s = ::core::ptr::null_mut::<ScriptStatHash>();
        if !h.is_null() {
            let mut _hf_bkt: ::core::ffi::c_uint = 0;
            _hf_bkt = _hf_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if !(*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize))
                    .hh_head
                    .is_null()
                {
                    s = ((*(*(*h).hh.tbl).buckets.offset(_hf_bkt as isize)).hh_head
                        as *mut ::core::ffi::c_char)
                        .offset(-(*(*h).hh.tbl).hho)
                        as *mut ::core::ffi::c_void as *mut ScriptStatHash
                        as *mut ScriptStatHash;
                } else {
                    s = ::core::ptr::null_mut::<ScriptStatHash>();
                }
                while !s.is_null() {
                    if (*s).hh.hashv == _hf_hashv
                        && (*s).hh.keylen
                            == strlen(script_tag as *const ::core::ffi::c_char)
                                as ::core::ffi::c_uint
                    {
                        if memcmp(
                            (*s).hh.key,
                            script_tag as *const ::core::ffi::c_void,
                            strlen(script_tag as *const ::core::ffi::c_char) as ::core::ffi::c_uint
                                as usize,
                        ) == 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                    }
                    if !(*s).hh.hh_next.is_null() {
                        s = ((*s).hh.hh_next as *mut ::core::ffi::c_char)
                            .offset(-(*(*h).hh.tbl).hho)
                            as *mut ::core::ffi::c_void
                            as *mut ScriptStatHash
                            as *mut ScriptStatHash;
                    } else {
                        s = ::core::ptr::null_mut::<ScriptStatHash>();
                    }
                }
            }
        }
        if !s.is_null() {
            if is_default {
                (*s).dl = language;
            } else {
                (*s).lc = ((*s).lc as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as u16;
                let ref mut fresh3 = *(*s)
                    .ll
                    .offset(((*s).lc as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
                *fresh3 = language;
            }
            sdsfree(script_tag);
        } else {
            s = __caryll_allocate_clean(
                ::core::mem::size_of::<ScriptStatHash>() as usize,
                316 as ::core::ffi::c_ulong,
            ) as *mut ScriptStatHash;
            (*s).tag = script_tag;
            (*s).dl = ::core::ptr::null_mut::<LanguageSystem>();
            (*s).ll = __caryll_allocate_clean(
                (::core::mem::size_of::<*mut LanguageSystem>() as usize)
                    .wrapping_mul((*table).languages.len()),
                319 as ::core::ffi::c_ulong,
            ) as *mut *mut LanguageSystem;
            if is_default {
                (*s).dl = language;
                (*s).lc = 0 as u16;
            } else {
                (*s).lc = 1 as u16;
                let ref mut fresh4 = *(*s)
                    .ll
                    .offset(((*s).lc as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
                *fresh4 = language;
            }
            let mut _ha_hashv: ::core::ffi::c_uint = 0;
            let mut _hj_i_0: ::core::ffi::c_uint = 0;
            let mut _hj_j_0: ::core::ffi::c_uint = 0;
            let mut _hj_k_0: ::core::ffi::c_uint = 0;
            let mut _hj_key_0: *const ::core::ffi::c_uchar =
                (*s).tag.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_uchar;
            _ha_hashv = 0xfeedbeef as ::core::ffi::c_uint;
            _hj_j_0 = 0x9e3779b9 as ::core::ffi::c_uint;
            _hj_i_0 = _hj_j_0;
            _hj_k_0 = strlen((*s).tag as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
            while _hj_k_0 >= 12 as ::core::ffi::c_uint {
                _hj_i_0 = _hj_i_0.wrapping_add(
                    (*_hj_key_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_j_0 = _hj_j_0.wrapping_add(
                    (*_hj_key_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _ha_hashv = _ha_hashv.wrapping_add(
                    (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                        .wrapping_add(
                            (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 16 as ::core::ffi::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key_0.offset(11 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int,
                        ),
                );
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
                _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
                _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
                _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
                _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
                _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
                _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
                _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
                _hj_key_0 = _hj_key_0.offset(12 as ::core::ffi::c_int as isize);
                _hj_k_0 = _hj_k_0.wrapping_sub(12 as ::core::ffi::c_uint);
            }
            _ha_hashv =
                _ha_hashv.wrapping_add(
                    strlen((*s).tag as *const ::core::ffi::c_char) as ::core::ffi::c_uint
                );
            let mut current_block_183: u64;
            match _hj_k_0 {
                11 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_183 = 9646444951891790190;
                }
                10 => {
                    current_block_183 = 9646444951891790190;
                }
                9 => {
                    current_block_183 = 3334602948526645343;
                }
                8 => {
                    current_block_183 = 2212965138932432936;
                }
                7 => {
                    current_block_183 = 10274451560863855454;
                }
                6 => {
                    current_block_183 = 14269574886101392281;
                }
                5 => {
                    current_block_183 = 12841885000613847604;
                }
                4 => {
                    current_block_183 = 9712482253163437245;
                }
                3 => {
                    current_block_183 = 3099469230490028345;
                }
                2 => {
                    current_block_183 = 10402200969996848048;
                }
                1 => {
                    current_block_183 = 3799609926439758155;
                }
                _ => {
                    current_block_183 = 5832582820025303349;
                }
            }
            match current_block_183 {
                9646444951891790190 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(9 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_183 = 3334602948526645343;
                }
                _ => {}
            }
            match current_block_183 {
                3334602948526645343 => {
                    _ha_hashv = _ha_hashv.wrapping_add(
                        (*_hj_key_0.offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_183 = 2212965138932432936;
                }
                _ => {}
            }
            match current_block_183 {
                2212965138932432936 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(7 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_183 = 10274451560863855454;
                }
                _ => {}
            }
            match current_block_183 {
                10274451560863855454 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(6 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_183 = 14269574886101392281;
                }
                _ => {}
            }
            match current_block_183 {
                14269574886101392281 => {
                    _hj_j_0 = _hj_j_0.wrapping_add(
                        (*_hj_key_0.offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_183 = 12841885000613847604;
                }
                _ => {}
            }
            match current_block_183 {
                12841885000613847604 => {
                    _hj_j_0 = _hj_j_0
                        .wrapping_add(*_hj_key_0.offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                    current_block_183 = 9712482253163437245;
                }
                _ => {}
            }
            match current_block_183 {
                9712482253163437245 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int,
                    );
                    current_block_183 = 3099469230490028345;
                }
                _ => {}
            }
            match current_block_183 {
                3099469230490028345 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 16 as ::core::ffi::c_int,
                    );
                    current_block_183 = 10402200969996848048;
                }
                _ => {}
            }
            match current_block_183 {
                10402200969996848048 => {
                    _hj_i_0 = _hj_i_0.wrapping_add(
                        (*_hj_key_0.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint)
                            << 8 as ::core::ffi::c_int,
                    );
                    current_block_183 = 3799609926439758155;
                }
                _ => {}
            }
            match current_block_183 {
                3799609926439758155 => {
                    _hj_i_0 = _hj_i_0
                        .wrapping_add(*_hj_key_0.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uint);
                }
                _ => {}
            }
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 13 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 8 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 13 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 12 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 16 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 5 as ::core::ffi::c_int;
            _hj_i_0 = _hj_i_0.wrapping_sub(_hj_j_0);
            _hj_i_0 = _hj_i_0.wrapping_sub(_ha_hashv);
            _hj_i_0 ^= _ha_hashv >> 3 as ::core::ffi::c_int;
            _hj_j_0 = _hj_j_0.wrapping_sub(_ha_hashv);
            _hj_j_0 = _hj_j_0.wrapping_sub(_hj_i_0);
            _hj_j_0 ^= _hj_i_0 << 10 as ::core::ffi::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i_0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j_0);
            _ha_hashv ^= _hj_j_0 >> 15 as ::core::ffi::c_int;
            (*s).hh.hashv = _ha_hashv;
            (*s).hh.key = (*s).tag.offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
            (*s).hh.keylen = strlen((*s).tag as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
            if h.is_null() {
                (*s).hh.next = NULL;
                (*s).hh.prev = NULL;
                (*s).hh.tbl = malloc(::core::mem::size_of::<UtHashTable>() as usize)
                    as *mut UtHashTable as *mut UtHashTable;
                if (*s).hh.tbl.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        (*s).hh.tbl as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<UtHashTable>() as usize,
                    );
                    (*(*s).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
                    (*(*s).hh.tbl).num_buckets = HASH_INITIAL_NUM_BUCKETS;
                    (*(*s).hh.tbl).log2_num_buckets = HASH_INITIAL_NUM_BUCKETS_LOG2;
                    (*(*s).hh.tbl).hho = (&raw mut (*s).hh as *mut ::core::ffi::c_char)
                        .offset_from(s as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        as isize;
                    (*(*s).hh.tbl).buckets = malloc(
                        (32 as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    ) as *mut UtHashBucket;
                    (*(*s).hh.tbl).signature = HASH_SIGNATURE as u32;
                    if (*(*s).hh.tbl).buckets.is_null() {
                        exit(-(1 as ::core::ffi::c_int));
                    } else {
                        memset(
                            (*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void,
                            '\0' as i32,
                            (32 as usize)
                                .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                        );
                    }
                }
                h = s;
            } else {
                (*s).hh.tbl = (*h).hh.tbl;
                (*s).hh.next = NULL;
                (*s).hh.prev = ((*(*h).hh.tbl).tail as *mut ::core::ffi::c_char)
                    .offset(-(*(*h).hh.tbl).hho)
                    as *mut ::core::ffi::c_void;
                (*(*(*h).hh.tbl).tail).next = s as *mut ::core::ffi::c_void;
                (*(*h).hh.tbl).tail = &raw mut (*s).hh as *mut UtHashHandle;
            }
            let mut _ha_bkt: ::core::ffi::c_uint = 0;
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_add(1);
            _ha_bkt = _ha_hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _ha_head: *mut UtHashBucket =
                (*(*h).hh.tbl).buckets.offset(_ha_bkt as isize) as *mut UtHashBucket;
            (*_ha_head).count = (*_ha_head).count.wrapping_add(1);
            (*s).hh.hh_next = (*_ha_head).hh_head as *mut UtHashHandle;
            (*s).hh.hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
            if !(*_ha_head).hh_head.is_null() {
                (*(*_ha_head).hh_head).hh_prev = &raw mut (*s).hh as *mut UtHashHandle;
            }
            (*_ha_head).hh_head = &raw mut (*s).hh as *mut UtHashHandle;
            if (*_ha_head).count
                >= (*_ha_head)
                    .expand_mult
                    .wrapping_add(1 as ::core::ffi::c_uint)
                    .wrapping_mul(HASH_BKT_CAPACITY_THRESH)
                && (*(*s).hh.tbl).noexpand == 0
            {
                let mut _he_bkt: ::core::ffi::c_uint = 0;
                let mut _he_bkt_i: ::core::ffi::c_uint = 0;
                let mut _he_thh: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _he_hh_nxt: *mut UtHashHandle = ::core::ptr::null_mut::<UtHashHandle>();
                let mut _he_new_buckets: *mut UtHashBucket =
                    ::core::ptr::null_mut::<UtHashBucket>();
                let mut _he_newbkt: *mut UtHashBucket = ::core::ptr::null_mut::<UtHashBucket>();
                _he_new_buckets = malloc(
                    (2 as usize)
                        .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                        .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                ) as *mut UtHashBucket;
                if _he_new_buckets.is_null() {
                    exit(-(1 as ::core::ffi::c_int));
                } else {
                    memset(
                        _he_new_buckets as *mut ::core::ffi::c_void,
                        '\0' as i32,
                        (2 as usize)
                            .wrapping_mul((*(*s).hh.tbl).num_buckets as usize)
                            .wrapping_mul(::core::mem::size_of::<UtHashBucket>() as usize),
                    );
                    (*(*s).hh.tbl).ideal_chain_maxlen = ((*(*s).hh.tbl).num_items
                        >> (*(*s).hh.tbl)
                            .log2_num_buckets
                            .wrapping_add(1 as ::core::ffi::c_uint))
                    .wrapping_add(
                        if (*(*s).hh.tbl).num_items
                            & (*(*s).hh.tbl)
                                .num_buckets
                                .wrapping_mul(2 as ::core::ffi::c_uint)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                            != 0 as ::core::ffi::c_uint
                        {
                            1 as ::core::ffi::c_uint
                        } else {
                            0 as ::core::ffi::c_uint
                        },
                    );
                    (*(*s).hh.tbl).nonideal_items = 0 as ::core::ffi::c_uint;
                    _he_bkt_i = 0 as ::core::ffi::c_uint;
                    while _he_bkt_i < (*(*s).hh.tbl).num_buckets {
                        _he_thh = (*(*(*s).hh.tbl).buckets.offset(_he_bkt_i as isize)).hh_head
                            as *mut UtHashHandle;
                        while !_he_thh.is_null() {
                            _he_hh_nxt = (*_he_thh).hh_next;
                            _he_bkt = (*_he_thh).hashv
                                & (*(*s).hh.tbl)
                                    .num_buckets
                                    .wrapping_mul(2 as ::core::ffi::c_uint)
                                    .wrapping_sub(1 as ::core::ffi::c_uint);
                            _he_newbkt =
                                _he_new_buckets.offset(_he_bkt as isize) as *mut UtHashBucket;
                            (*_he_newbkt).count = (*_he_newbkt).count.wrapping_add(1);
                            if (*_he_newbkt).count > (*(*s).hh.tbl).ideal_chain_maxlen {
                                (*(*s).hh.tbl).nonideal_items =
                                    (*(*s).hh.tbl).nonideal_items.wrapping_add(1);
                                (*_he_newbkt).expand_mult = (*_he_newbkt)
                                    .count
                                    .wrapping_div((*(*s).hh.tbl).ideal_chain_maxlen);
                            }
                            (*_he_thh).hh_prev = ::core::ptr::null_mut::<UtHashHandle>();
                            (*_he_thh).hh_next = (*_he_newbkt).hh_head as *mut UtHashHandle;
                            if !(*_he_newbkt).hh_head.is_null() {
                                (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                            }
                            (*_he_newbkt).hh_head = _he_thh as *mut UtHashHandle;
                            _he_thh = _he_hh_nxt;
                        }
                        _he_bkt_i = _he_bkt_i.wrapping_add(1);
                    }
                    free((*(*s).hh.tbl).buckets as *mut ::core::ffi::c_void);
                    (*(*s).hh.tbl).num_buckets = (*(*s).hh.tbl)
                        .num_buckets
                        .wrapping_mul(2 as ::core::ffi::c_uint);
                    (*(*s).hh.tbl).log2_num_buckets =
                        (*(*s).hh.tbl).log2_num_buckets.wrapping_add(1);
                    (*(*s).hh.tbl).buckets = _he_new_buckets;
                    (*(*s).hh.tbl).ineff_expands = if (*(*s).hh.tbl).nonideal_items
                        > (*(*s).hh.tbl).num_items >> 1 as ::core::ffi::c_int
                    {
                        (*(*s).hh.tbl)
                            .ineff_expands
                            .wrapping_add(1 as ::core::ffi::c_uint)
                    } else {
                        0 as ::core::ffi::c_uint
                    };
                    if (*(*s).hh.tbl).ineff_expands > 1 as ::core::ffi::c_uint {
                        (*(*s).hh.tbl).noexpand = 1 as ::core::ffi::c_uint;
                    }
                }
            }
        }
        j = j.wrapping_add(1);
    }
    let mut root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (if !h.is_null() {
            (*(*h).hh.tbl).num_items
        } else {
            0 as ::core::ffi::c_uint
        }) as u32)]);
    let mut s_0: *mut ScriptStatHash = ::core::ptr::null_mut::<ScriptStatHash>();
    let mut tmp: *mut ScriptStatHash = ::core::ptr::null_mut::<ScriptStatHash>();
    s_0 = h;
    tmp = (if !h.is_null() { (*h).hh.next } else { NULL }) as *mut ScriptStatHash
        as *mut ScriptStatHash;
    while !s_0.is_null() {
        bk_push(root, &[bk_int(BkCellType::B32, (feature_name_to_tag((*s_0).tag)) as u32), bk_ptr(BkCellType::P16, write_script(s_0, table))]);
        let mut _hd_hh_del: *mut UtHashHandle = &raw mut (*s_0).hh;
        if (*_hd_hh_del).prev.is_null() && (*_hd_hh_del).next.is_null() {
            free((*(*h).hh.tbl).buckets as *mut ::core::ffi::c_void);
            free((*h).hh.tbl as *mut ::core::ffi::c_void);
            h = ::core::ptr::null_mut::<ScriptStatHash>();
        } else {
            let mut _hd_bkt: ::core::ffi::c_uint = 0;
            if _hd_hh_del == (*(*h).hh.tbl).tail {
                (*(*h).hh.tbl).tail = ((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle
                    as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).prev.is_null() {
                let ref mut fresh5 = (*(((*_hd_hh_del).prev as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .next;
                *fresh5 = (*_hd_hh_del).next;
            } else {
                h = (*_hd_hh_del).next as *mut ScriptStatHash as *mut ScriptStatHash;
            }
            if !(*_hd_hh_del).next.is_null() {
                let ref mut fresh6 = (*(((*_hd_hh_del).next as *mut ::core::ffi::c_char)
                    .offset((*(*h).hh.tbl).hho)
                    as *mut UtHashHandle))
                    .prev;
                *fresh6 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & (*(*h).hh.tbl)
                    .num_buckets
                    .wrapping_sub(1 as ::core::ffi::c_uint);
            let mut _hd_head: *mut UtHashBucket =
                (*(*h).hh.tbl).buckets.offset(_hd_bkt as isize) as *mut UtHashBucket;
            (*_hd_head).count = (*_hd_head).count.wrapping_sub(1);
            if (*_hd_head).hh_head == _hd_hh_del {
                (*_hd_head).hh_head = (*_hd_hh_del).hh_next as *mut UtHashHandle;
            }
            if !(*_hd_hh_del).hh_prev.is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !(*_hd_hh_del).hh_next.is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*h).hh.tbl).num_items = (*(*h).hh.tbl).num_items.wrapping_sub(1);
        }
        sdsfree((*s_0).tag);
        free((*s_0).ll as *mut ::core::ffi::c_void);
        (*s_0).ll = ::core::ptr::null_mut::<*mut LanguageSystem>();
        free(s_0 as *mut ::core::ffi::c_void);
        s_0 = ::core::ptr::null_mut::<ScriptStatHash>();
        s_0 = tmp;
        tmp = (if !tmp.is_null() { (*tmp).hh.next } else { NULL }) as *mut ScriptStatHash
            as *mut ScriptStatHash;
    }
    return root;
}
pub unsafe extern "C" fn otfcc_build_otl(
    mut table: *const OtlTable,
    mut options: *const Options,
    mut tag: *const ::core::ffi::c_char,
) -> *mut Buffer {
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
