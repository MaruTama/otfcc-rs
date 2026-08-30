#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, strncmp};

use crate::bk::bkblock::bk_new_block_from_buffer;
use crate::bk::bkblock::{BkBlock, BkCellType, bk_int, bk_new_block, bk_ptr, bk_push};
use crate::bk::bkgraph::bk_build_block;
use crate::logger::{
    LOG_VL_NOTICE, LOG_VL_PROGRESS, LoggerType, logger_finish, logger_log_sds, logger_start_sds,
};
use crate::support::buffer::Buffer;
use crate::support::options::Options;
use crate::support::primitives::TableId;
use crate::table::otl::subtables::BuildHeuristics;
use crate::table::otl::subtables::chaining::build::otfcc_chaining_lookup_is_contextual_lookup;
use crate::table::otl::subtables::chaining::classifier::otfcc_classified_build_chaining;
use crate::table::otl::subtables::gpos_cursive::otfcc_build_gpos_cursive;
use crate::table::otl::subtables::gpos_mark_to_ligature::otfcc_build_gpos_mark_to_ligature;
use crate::table::otl::subtables::gpos_mark_to_single::otfcc_build_gpos_mark_to_single;
use crate::table::otl::subtables::gpos_pair::otfcc_build_gpos_pair;
use crate::table::otl::subtables::gpos_single::otfcc_build_gpos_single;
use crate::table::otl::subtables::gsub_ligature::otfcc_build_gsub_ligature_subtable;
use crate::table::otl::subtables::gsub_multi::otfcc_build_gsub_multi_subtable_split;
use crate::table::otl::subtables::gsub_reverse::otfcc_build_gsub_reverse;
use crate::table::otl::subtables::gsub_single::otfcc_build_gsub_single_subtable;
use crate::table::otl::{
    Feature, LanguageSystem, Lookup, LookupType, OTL_TYPE_GPOS_CHAINING, OTL_TYPE_GPOS_CURSIVE,
    OTL_TYPE_GPOS_EXTEND, OTL_TYPE_GPOS_MARK_TO_BASE, OTL_TYPE_GPOS_MARK_TO_LIGATURE,
    OTL_TYPE_GPOS_MARK_TO_MARK, OTL_TYPE_GPOS_PAIR, OTL_TYPE_GPOS_SINGLE, OTL_TYPE_GPOS_UNKNOWN,
    OTL_TYPE_GSUB_ALTERNATE, OTL_TYPE_GSUB_CHAINING, OTL_TYPE_GSUB_EXTEND, OTL_TYPE_GSUB_LIGATURE,
    OTL_TYPE_GSUB_MULTIPLE, OTL_TYPE_GSUB_REVERSE, OTL_TYPE_GSUB_SINGLE, OTL_TYPE_GSUB_UNKNOWN,
    OtlTable, Subtable, subtable_at,
};
// No longer `extern "C"`: each of the 9 concrete builders passed into
// `_declare_lookup_writer`/`_declare_lookup_writer_split` below is used in
// exactly one fixed association with its own `LookupType` -- the
// `_build_lookup` sequence is a `match` in disguise (first-match-wins via
// `written == 0`), not real runtime dispatch through a varying value
// (confirmed by grep: none of the 9 builder functions are referenced
// anywhere outside this file).
pub type OtlBuilder = Option<unsafe fn(*const Subtable, BuildHeuristics) -> *mut Buffer>;
pub type OtlSplitBuilder =
    Option<unsafe fn(*const Subtable, BuildHeuristics, *mut TableId) -> *mut *mut Buffer>;
pub const LARGE_SUBTABLE_LIMIT: i32 = 4096_i32;
fn feature_name_to_tag(name: &[u8]) -> u32 {
    let mut tag: u32 = 0_u32;
    if name.len() > 0_usize {
        tag |= ((name[0_usize] as i32) << 24_i32) as u32;
    } else {
        tag |= ((' ' as i32 as u8 as i32) << 24_i32) as u32;
    }
    if name.len() > 1_usize {
        tag |= ((name[1_usize] as i32) << 16_i32) as u32;
    } else {
        tag |= ((' ' as i32 as u8 as i32) << 16_i32) as u32;
    }
    if name.len() > 2_usize {
        tag |= ((name[2_usize] as i32) << 8_i32) as u32;
    } else {
        tag |= ((' ' as i32 as u8 as i32) << 8_i32) as u32;
    }
    if name.len() > 3_usize {
        tag |= (name[3_usize] as i32) as u32;
    } else {
        tag |= (' ' as i32 as u8 as i32) as u32;
    }
    return tag;
}
unsafe fn _declare_lookup_writer(
    type_0: LookupType,
    fn_0: OtlBuilder,
    lookup: *const Lookup,
    subtables: &mut Vec<*mut Buffer>,
    last_offset: *mut usize,
    prefer_extension_for_this_lut: *mut bool,
    heuristics: BuildHeuristics,
) -> TableId {
    if (*lookup).type_0 == type_0 {
        subtables.clear();
        subtables.reserve((*lookup).subtables.len());
        let mut total_buf_size_short: usize = 0_usize;
        let mut total_buf_size_ext: usize = 0_usize;
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*lookup).subtables.len() {
            let buf: *mut Buffer = fn_0.expect("non-null function pointer")(
                subtable_at(&(*lookup).subtables, j as usize) as *const Subtable,
                heuristics,
            );
            subtables.push(buf);
            total_buf_size_short = total_buf_size_short.wrapping_add((*buf).data.len());
            total_buf_size_ext = total_buf_size_ext.wrapping_add(8_usize);
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
unsafe fn _declare_lookup_writer_split(
    type_0: LookupType,
    fn_0: OtlSplitBuilder,
    lookup: *const Lookup,
    subtables: &mut Vec<*mut Buffer>,
    last_offset: *mut usize,
    prefer_extension_for_this_lut: *mut bool,
    heuristics: BuildHeuristics,
) -> TableId {
    if (*lookup).type_0 == type_0 {
        subtables.clear();
        let mut total_buf_size_short: usize = 0_usize;
        let mut j: TableId = 0 as TableId;
        while (j as usize) < (*lookup).subtables.len() {
            let mut n_part: TableId = 0 as TableId;
            let mut part: *mut *mut Buffer = fn_0.expect("non-null function pointer")(
                subtable_at(&(*lookup).subtables, j as usize) as *const Subtable,
                heuristics,
                &raw mut n_part,
            );
            let mut k: TableId = 0 as TableId;
            while (k as i32) < n_part as i32 {
                subtables.push(*part.offset(k as isize));
                total_buf_size_short =
                    total_buf_size_short.wrapping_add((**part.offset(k as isize)).data.len());
                k = k.wrapping_add(1);
            }
            // `part` itself -- the raw `*mut *mut Buffer` shell `fn_0`
            // (a split-builder in another file, e.g. `gsub_multi.rs`)
            // returned -- is still freed here exactly as before; each
            // `*mut Buffer` it pointed to was copied into `subtables`
            // above, not owned by `part`, so this only releases the
            // now-empty index array.
            free(part as *mut ::core::ffi::c_void);
            part = ::core::ptr::null_mut::<*mut Buffer>();
            j = j.wrapping_add(1);
        }
        let total = subtables.len() as TableId;
        if total_buf_size_short > LARGE_SUBTABLE_LIMIT as usize {
            *last_offset = (*last_offset)
                .wrapping_add((8_i32 * total as i32) as usize);
            *prefer_extension_for_this_lut = true;
        } else {
            *last_offset = (*last_offset).wrapping_add(total_buf_size_short);
            *prefer_extension_for_this_lut = false;
        }
        return total;
    }
    return 0 as TableId;
}
unsafe fn _build_lookup(
    lookup: *const Lookup,
    subtables: &mut Vec<*mut Buffer>,
    last_offset: *mut usize,
    prefer_extension_for_this_lut: *mut bool,
    heuristics: BuildHeuristics,
) -> TableId {
    if (*lookup).type_0 == OTL_TYPE_GPOS_CHAINING || (*lookup).type_0 == OTL_TYPE_GSUB_CHAINING {
        return otfcc_classified_build_chaining(lookup, subtables, last_offset);
    }
    let mut written: TableId = 0 as TableId;
    if written == 0 {
        written = _declare_lookup_writer(
            OTL_TYPE_GSUB_SINGLE,
            Some(
                otfcc_build_gsub_single_subtable
                    as unsafe fn(*const Subtable, BuildHeuristics) -> *mut Buffer,
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
                    as unsafe fn(*const Subtable, BuildHeuristics, *mut TableId) -> *mut *mut Buffer,
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
                    as unsafe fn(*const Subtable, BuildHeuristics, *mut TableId) -> *mut *mut Buffer,
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
                    as unsafe fn(*const Subtable, BuildHeuristics) -> *mut Buffer,
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
                    as unsafe fn(*const Subtable, BuildHeuristics) -> *mut Buffer,
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
                    as unsafe fn(*const Subtable, BuildHeuristics) -> *mut Buffer,
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
                    as unsafe fn(*const Subtable, BuildHeuristics) -> *mut Buffer,
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
                    as unsafe fn(*const Subtable, BuildHeuristics) -> *mut Buffer,
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
                    as unsafe fn(*const Subtable, BuildHeuristics) -> *mut Buffer,
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
                    as unsafe fn(*const Subtable, BuildHeuristics) -> *mut Buffer,
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
                    as unsafe fn(*const Subtable, BuildHeuristics) -> *mut Buffer,
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
unsafe fn get_lookup_heuristics(
    table: *const OtlTable,
    lut: *const Lookup,
) -> BuildHeuristics {
    let mut heu: BuildHeuristics = BuildHeuristics::empty();
    if (*lut).type_0 == OTL_TYPE_GSUB_SINGLE {
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
unsafe fn write_otl_lookups(
    table: *const OtlTable,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) -> *mut BkBlock {
    // `subtables`/`subtable_quantity`/`prefer_ext_for_this_lut` were three
    // separately `__caryll_allocate_clean`'d, index-parallel arrays, sized
    // once to `lookups.len()` and never resized after -- `Vec`s built the
    // same way (`vec![default; lookups.len()]`) reproduce the exact same
    // shape without a matching `free()` trio to remember at every exit
    // point below.
    let mut subtables: Vec<Vec<*mut Buffer>> = vec![Vec::new(); (*table).lookups.len()];
    let mut subtable_quantity: Vec<TableId> = vec![0 as TableId; (*table).lookups.len()];
    let mut prefer_ext_for_this_lut: Vec<bool> = vec![false; (*table).lookups.len()];
    let mut last_offset: usize = 0_usize;
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*table).lookups.len() {
        let lookup: *const Lookup = &raw const *(&(*table).lookups)[j as usize];
        let heu: BuildHeuristics = get_lookup_heuristics(table, lookup);
        logger_log_sds(
            &mut *options.logger.borrow_mut(),
            LOG_VL_PROGRESS,
            LoggerType::Progress,
            crate::bytesbuild!(
                b"Building lookup ",
                &(*lookup).name,
                b" (",
                j as i32,
                b"/",
                (*table).lookups.len() as u32,
                b")\n",
            ),
        );
        subtable_quantity[j as usize] = _build_lookup(
            lookup,
            &mut subtables[j as usize],
            &raw mut last_offset,
            &mut prefer_ext_for_this_lut[j as usize],
            heu,
        );
        j = j.wrapping_add(1);
    }
    let mut header_size: usize =
        2_usize.wrapping_add(2_usize.wrapping_mul((*table).lookups.len()));
    let mut j_0: TableId = 0 as TableId;
    while (j_0 as usize) < (*table).lookups.len() {
        if subtable_quantity[j_0 as usize] != 0 {
            header_size = header_size.wrapping_add(
                (6_i32
                    + 2_i32
                        * subtable_quantity[j_0 as usize] as i32)
                    as usize,
            );
        }
        j_0 = j_0.wrapping_add(1);
    }
    let use_extended: bool = last_offset >= 0xff00_usize.wrapping_sub(header_size);
    let root: *mut BkBlock =
        bk_new_block(&[bk_int(BkCellType::B16, ((*table).lookups.len()) as u32)]);
    let mut j_1: TableId = 0 as TableId;
    while (j_1 as usize) < (*table).lookups.len() {
        if subtable_quantity[j_1 as usize] == 0 {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_NOTICE,
                LoggerType::Info,
                crate::bytesbuild!(
                    b"Lookup ",
                    &(*(&(*table).lookups)[j_1 as usize]).name,
                    b" is empty.\n",
                ),
            );
        }
        let lookup_0: *const Lookup = &raw const *(&(*table).lookups)[j_1 as usize];
        let can_be_contextual: bool = otfcc_chaining_lookup_is_contextual_lookup(lookup_0);
        let use_extended_for_it: bool = use_extended as i32 != 0
            || prefer_ext_for_this_lut[j_1 as usize] as i32 != 0;
        if use_extended_for_it {
            logger_log_sds(
                &mut *options.logger.borrow_mut(),
                LOG_VL_NOTICE,
                LoggerType::Info,
                crate::bytesbuild!(
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
        let lookup_type: u16 = (if use_extended_for_it {
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
        let blk: *mut BkBlock = bk_new_block(&[
            bk_int(BkCellType::B16, (lookup_type as i32) as u32),
            bk_int(
                BkCellType::B16,
                ((*lookup_0).flags as i32) as u32,
            ),
            bk_int(
                BkCellType::B16,
                (subtable_quantity[j_1 as usize] as i32) as u32,
            ),
        ]);
        let mut k: TableId = 0 as TableId;
        while (k as i32) < subtable_quantity[j_1 as usize] as i32 {
            if use_extended_for_it {
                let extension_lookup_type: u16 = (*lookup_0)
                    .type_0
                    .file_format()
                    .wrapping_sub(can_be_contextual as u32)
                    as u16;
                let stub: *mut BkBlock = bk_new_block(&[
                    bk_int(BkCellType::B16, 1_u32),
                    bk_int(
                        BkCellType::B16,
                        (extension_lookup_type as i32) as u32,
                    ),
                    bk_ptr(
                        BkCellType::P32,
                        bk_new_block_from_buffer(subtables[j_1 as usize][k as usize]),
                    ),
                ]);
                bk_push(blk, &[bk_ptr(BkCellType::P16, stub)]);
            } else {
                bk_push(
                    blk,
                    &[bk_ptr(
                        BkCellType::P16,
                        bk_new_block_from_buffer(subtables[j_1 as usize][k as usize]),
                    )],
                );
            }
            k = k.wrapping_add(1);
        }
        bk_push(blk, &[bk_int(BkCellType::B16, 0_u32)]);
        bk_push(root, &[bk_ptr(BkCellType::P16, blk)]);
        j_1 = j_1.wrapping_add(1);
    }
    return root;
}
unsafe fn write_otl_features(table: *const OtlTable) -> *mut BkBlock {
    let root: *mut BkBlock =
        bk_new_block(&[bk_int(BkCellType::B16, ((*table).features.len()) as u32)]);
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*table).features.len() {
        let fea: *mut BkBlock = bk_new_block(&[
            bk_ptr(BkCellType::P16, ::core::ptr::null_mut()),
            bk_int(
                BkCellType::B16,
                ((*(&(*table).features)[j as usize]).lookups.len()) as u32,
            ),
        ]);
        let mut k: TableId = 0 as TableId;
        while (k as usize) < (*(&(*table).features)[j as usize]).lookups.len() {
            let mut l: TableId = 0 as TableId;
            while (l as usize) < (*table).lookups.len() {
                if (&(*(&(*table).features)[j as usize]).lookups)[k as usize]
                    == &raw const *(&(*table).lookups)[l as usize]
                {
                    bk_push(
                        fea,
                        &[bk_int(BkCellType::B16, (l as i32) as u32)],
                    );
                    break;
                } else {
                    l = l.wrapping_add(1);
                }
            }
            k = k.wrapping_add(1);
        }
        bk_push(
            root,
            &[
                bk_int(
                    BkCellType::B32,
                    (feature_name_to_tag(&(*(&(*table).features)[j as usize]).name)) as u32,
                ),
                bk_ptr(BkCellType::P16, fea),
            ],
        );
        j = j.wrapping_add(1);
    }
    return root;
}
unsafe fn feature_index(feature: *const Feature, table: *const OtlTable) -> TableId {
    let mut j: TableId = 0 as TableId;
    while (j as usize) < (*table).features.len() {
        if &raw const *(&(*table).features)[j as usize] == feature {
            return j;
        }
        j = j.wrapping_add(1);
    }
    return 0xffff as TableId;
}
unsafe fn write_language(
    lang: *const LanguageSystem,
    table: *const OtlTable,
) -> *mut BkBlock {
    if lang.is_null() {
        return ::core::ptr::null_mut::<BkBlock>();
    }
    let root: *mut BkBlock = bk_new_block(&[
        bk_ptr(BkCellType::P16, ::core::ptr::null_mut()),
        bk_int(
            BkCellType::B16,
            (feature_index((*lang).required_feature, table) as i32) as u32,
        ),
        bk_int(BkCellType::B16, ((*lang).features.len()) as u32),
    ]);
    let mut k: TableId = 0 as TableId;
    while (k as usize) < (*lang).features.len() {
        bk_push(
            root,
            &[bk_int(
                BkCellType::B16,
                (feature_index((&(*lang).features)[k as usize], table) as i32)
                    as u32,
            )],
        );
        k = k.wrapping_add(1);
    }
    return root;
}
#[allow(improper_ctypes_definitions)] // internal call only, never crosses FFI
unsafe fn write_script(
    dl: *const LanguageSystem,
    ll: &[*const LanguageSystem],
    table: *const OtlTable,
) -> *mut BkBlock {
    let root: *mut BkBlock = bk_new_block(&[
        bk_ptr(BkCellType::P16, write_language(dl, table)),
        bk_int(BkCellType::B16, (ll.len()) as u32),
    ]);
    let mut j: TableId = 0 as TableId;
    while (j as usize) < ll.len() {
        let tag: &[u8] = ::core::slice::from_raw_parts(
            (*ll[j as usize])
                .name
                .as_ptr()
                .offset(5_i32 as isize),
            4_usize,
        );
        bk_push(
            root,
            &[
                bk_int(BkCellType::B32, (feature_name_to_tag(tag)) as u32),
                bk_ptr(BkCellType::P16, write_language(ll[j as usize], table)),
            ],
        );
        j = j.wrapping_add(1);
    }
    return root;
}
unsafe fn write_otl_script_and_languages(table: *const OtlTable) -> *mut BkBlock {
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
            ::core::slice::from_raw_parts((*language).name.as_ptr(), 4_usize).to_vec();
        let is_default: bool = strncmp(
            (*language)
                .name
                .as_ptr()
                .offset(5_i32 as isize) as *const ::core::ffi::c_char,
            b"DFLT\0" as *const u8 as *const ::core::ffi::c_char,
            4_usize,
        ) == 0_i32
            || strncmp(
                (*language)
                    .name
                    .as_ptr()
                    .offset(5_i32 as isize)
                    as *const ::core::ffi::c_char,
                b"dflt\0" as *const u8 as *const ::core::ffi::c_char,
                4_usize,
            ) == 0_i32;
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
    let root: *mut BkBlock = bk_new_block(&[bk_int(BkCellType::B16, (scripts.len()) as u32)]);
    for group in &scripts {
        bk_push(
            root,
            &[
                bk_int(BkCellType::B32, feature_name_to_tag(&group.tag)),
                bk_ptr(
                    BkCellType::P16,
                    write_script(group.default_language, &group.languages, table),
                ),
            ],
        );
    }
    return root;
}
pub unsafe fn otfcc_build_otl(
    table: Option<&OtlTable>,
    options: &Options,
    tag: *const ::core::ffi::c_char,
) -> *mut Buffer {
    let table: *const OtlTable = table.map_or(::core::ptr::null(), |t| t as *const OtlTable);
    if table.is_null() {
        return ::core::ptr::null_mut::<Buffer>();
    }
    let mut buf: *mut Buffer = ::core::ptr::null_mut::<Buffer>();
    logger_start_sds(&mut *options.logger.borrow_mut(), crate::bytesbuild!(tag));
    let mut ___loggedstep_v: bool = true;
    while ___loggedstep_v {
        let lookups: *mut BkBlock = write_otl_lookups(table, options, tag);
        let features: *mut BkBlock = write_otl_features(table);
        let languages: *mut BkBlock = write_otl_script_and_languages(table);
        let root: *mut BkBlock = bk_new_block(&[
            bk_int(BkCellType::B32, 0x10000_u32),
            bk_ptr(BkCellType::P16, languages),
            bk_ptr(BkCellType::P16, features),
            bk_ptr(BkCellType::P16, lookups),
        ]);
        buf = bk_build_block(root).into_raw();
        ___loggedstep_v = false;
        logger_finish(&mut *options.logger.borrow_mut());
    }
    return buf;
}
