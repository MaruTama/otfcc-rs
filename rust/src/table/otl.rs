#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
pub mod build;
pub mod classdef;
pub mod constants;
pub mod coverage;
pub mod dump;
pub mod parse;
pub mod read;
pub mod subtables;

use libc::{calloc, free};

use crate::table::otl::classdef::{ClassDef};
use crate::table::otl::coverage::{Coverage};
use crate::support::handle::{GlyphHandle, LookupHandle};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::primitives::{GlyphClass, GlyphId, Pos, TableId};
use crate::vendor::sds::{SdsRaw};
use crate::table::otl::subtables::chaining::common::{I_SUBTABLE_CHAINING};
use crate::table::otl::subtables::gpos_cursive::{subtable_gpos_cursive_free};
use crate::table::otl::subtables::gpos_mark_to_ligature::{subtable_gpos_mark_to_ligature_free};
use crate::table::otl::subtables::gpos_mark_to_single::{subtable_gpos_mark_to_single_free};
use crate::table::otl::subtables::gpos_pair::{I_SUBTABLE_GPOS_PAIR};
use crate::table::otl::subtables::gpos_single::{subtable_gpos_single_free};
use crate::table::otl::subtables::gsub_ligature::{subtable_gsub_ligature_free};
use crate::table::otl::subtables::gsub_multi::{subtable_gsub_multi_free};
use crate::table::otl::subtables::gsub_reverse::{I_SUBTABLE_GSUB_REVERSE};
use crate::table::otl::subtables::gsub_single::{subtable_gsub_single_free};
use crate::vendor::sds::{sdsfree};


/// Which gsub/gpos subtable format a lookup is, in otfcc's own numbering: the
/// file's 16-bit format number offset by the table's base, `otl_type_gsub_*`
/// starting at 16 and `otl_type_gpos_*` at 32, so one value names both the
/// table and the format.
///
/// **Deliberately not an `enum`.** The value is read from the font:
/// `otfcc_read_otl_common` does `lookup->type = read_16u(data) + base`, so
/// anything in `16..=65551` can turn up, and C does not clamp it. An
/// unrecognised type is carried through as-is — `otfcc_read_otl_subtable`
/// returns NULL for it, and the lookup's generated name puts the *raw number*
/// in the output as hex (`lookup_0019_3`, from
/// `sdsbuild!(… Hex2(lookup->type) …)` in `read.rs`). A `#[repr(u32)]` enum
/// could not hold such a value, `transmute`ing one in would be UB, and
/// rejecting it would change the JSON that otfcc writes for a font with an
/// unknown lookup type — which no test payload has, so the byte comparison
/// would not have caught it.
///
/// So this is the honest shape: a newtype over the number, with the known
/// values as named constants and the two file-derived construction sites going
/// through [`LookupType::from_file`]. What it buys over the bare `c_uint`
/// c2rust emitted is that the compiler now separates it from every other 32-bit
/// quantity in the OTL code, and that [`LookupType::name`] replaces a
/// 42-entry sparse table of C string pointers.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct LookupType(u32);

pub const OTL_TYPE_GPOS_EXTEND: LookupType = LookupType(41);
pub const OTL_TYPE_GPOS_CHAINING: LookupType = LookupType(40);
pub const OTL_TYPE_GPOS_CONTEXT: LookupType = LookupType(39);
pub const OTL_TYPE_GPOS_MARK_TO_MARK: LookupType = LookupType(38);
pub const OTL_TYPE_GPOS_MARK_TO_LIGATURE: LookupType = LookupType(37);
pub const OTL_TYPE_GPOS_MARK_TO_BASE: LookupType = LookupType(36);
pub const OTL_TYPE_GPOS_CURSIVE: LookupType = LookupType(35);
pub const OTL_TYPE_GPOS_PAIR: LookupType = LookupType(34);
pub const OTL_TYPE_GPOS_SINGLE: LookupType = LookupType(33);
pub const OTL_TYPE_GPOS_UNKNOWN: LookupType = LookupType(32);
pub const OTL_TYPE_GSUB_REVERSE: LookupType = LookupType(24);
pub const OTL_TYPE_GSUB_EXTEND: LookupType = LookupType(23);
pub const OTL_TYPE_GSUB_CHAINING: LookupType = LookupType(22);
pub const OTL_TYPE_GSUB_CONTEXT: LookupType = LookupType(21);
pub const OTL_TYPE_GSUB_LIGATURE: LookupType = LookupType(20);
pub const OTL_TYPE_GSUB_ALTERNATE: LookupType = LookupType(19);
pub const OTL_TYPE_GSUB_MULTIPLE: LookupType = LookupType(18);
pub const OTL_TYPE_GSUB_SINGLE: LookupType = LookupType(17);
pub const OTL_TYPE_GSUB_UNKNOWN: LookupType = LookupType(16);
pub const OTL_TYPE_UNKNOWN: LookupType = LookupType(0);

impl LookupType {
    /// The type of a lookup as the font file spells it: a format number
    /// relative to `base`, which is `OTL_TYPE_GSUB_UNKNOWN` for gsub and
    /// `OTL_TYPE_GPOS_UNKNOWN` for gpos. Wrapping, like the C addition it
    /// replaces — `raw` is a full `u16` straight out of the file and is not
    /// validated here, exactly as C does not validate it.
    pub const fn from_file(base: Self, raw: u16) -> Self {
        Self(base.0.wrapping_add(raw as u32))
    }

    /// The number itself. It reaches the output: a lookup with no name gets
    /// `lookup_<this as %04x>_<index>`.
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// The format number to write back into the file — this value with its
    /// table's base taken off again, and 0 for anything at or below gsub's base.
    ///
    /// The comparisons are `>`, not `>=`, exactly as in C: `OTL_TYPE_UNKNOWN`
    /// and `OTL_TYPE_GSUB_UNKNOWN` give 0, while `OTL_TYPE_GPOS_UNKNOWN` (32)
    /// is above *gsub's* base and so reads as gsub format 16. That is a quirk
    /// of the original, reachable only from a font declaring a gpos lookup of
    /// format 0; the number reaches the lookup header, so it is reproduced
    /// rather than tidied. `file_format_undoes_the_table_base` pins it.
    pub const fn file_format(self) -> u32 {
        if self.0 > OTL_TYPE_GPOS_UNKNOWN.0 {
            self.0 - OTL_TYPE_GPOS_UNKNOWN.0
        } else if self.0 > OTL_TYPE_GSUB_UNKNOWN.0 {
            self.0 - OTL_TYPE_GSUB_UNKNOWN.0
        } else {
            0
        }
    }

    /// The name this type has in otfcc's JSON, and the key its lookups are
    /// looked up by when reading JSON back.
    ///
    /// This was `tableNames`, a 42-element `static mut` array of C string
    /// pointers indexed by the type — 23 of whose entries were NULL, since the
    /// numbering leaves holes between the two tables. Every one of the 26 uses
    /// indexed it with a constant, so nothing was ever at risk of reading a
    /// hole; the fallback here is index 0's own text, which keeps the function
    /// total without inventing a name.
    pub const fn name(self) -> &'static ::core::ffi::CStr {
        match self.0 {
            17 => c"gsub_single",
            18 => c"gsub_multiple",
            19 => c"gsub_alternate",
            20 => c"gsub_ligature",
            21 => c"gsub_context",
            22 => c"gsub_chaining",
            23 => c"gsub_extend",
            24 => c"gsub_reverse",
            16 => c"gsub_unknown",
            33 => c"gpos_single",
            34 => c"gpos_pair",
            35 => c"gpos_cursive",
            36 => c"gpos_mark_to_base",
            37 => c"gpos_mark_to_ligature",
            38 => c"gpos_mark_to_mark",
            39 => c"gpos_context",
            40 => c"gpos_chaining",
            41 => c"gpos_extend",
            32 => c"gpos_unknown",
            _ => c"unknown",
        }
    }
}
// Never copied or moved by value anywhere in the crate -- every use is
// `*mut Subtable`/`*const Subtable`/`size_of::<Subtable>()`/
// `null_mut::<Subtable>()`, so `Subtable` itself needs neither `Copy` nor
// `Clone`. The 7 variants that now own a `Vec` (all container types that
// used to block their `Vec` conversion on this union) are wrapped in
// `ManuallyDrop` -- the only field shape besides `Copy` a union may hold.
// `ManuallyDrop<T>` is `#[repr(transparent)]`, so every extraction site
// (`&raw mut/const (*subtable).field`) just adds `as *mut/*const T` to get
// back the field's real type; nothing downstream of that cast changes.
#[repr(C)]
pub union Subtable {
    pub gsub_single: ::core::mem::ManuallyDrop<GsubSingleSubtable>,
    pub gsub_multi: ::core::mem::ManuallyDrop<GsubMultiSubtable>,
    pub gsub_ligature: ::core::mem::ManuallyDrop<GsubLigatureSubtable>,
    pub chaining: ChainingSubtable,
    pub gsub_reverse: GsubReverseSubtable,
    pub gpos_single: ::core::mem::ManuallyDrop<GposSingleSubtable>,
    pub gpos_pair: GposPairSubtable,
    pub gpos_cursive: ::core::mem::ManuallyDrop<GposCursiveSubtable>,
    pub gpos_mark_to_single: ::core::mem::ManuallyDrop<GposMarkToSingleSubtable>,
    pub gpos_mark_to_ligature: ::core::mem::ManuallyDrop<GposMarkToLigatureSubtable>,
    pub extend: ExtendSubtable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtendSubtable {
    pub type_0: LookupType,
    pub subtable: *mut Subtable,
}
// Never passed or embedded by value anywhere in the crate (only ever behind
// `*mut`/`*const`, as a `Subtable` union field) -- no `Copy`/`Clone` needed
// once `mark_array`/`lig_array` own `Vec`s.
#[repr(C)]
pub struct GposMarkToLigatureSubtable {
    pub class_count: GlyphClass,
    pub mark_array: MarkArray,
    pub lig_array: LigatureArray,
}
/// Embedded by value in both `GposMarkToSingleSubtable` and
/// `GposMarkToLigatureSubtable`, not a `Subtable` union field itself.
pub type LigatureArray = Vec<LigatureBaseRecord>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LigatureBaseRecord {
    pub glyph: GlyphHandle,
    pub component_count: GlyphId,
    pub anchors: *mut *mut Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Anchor {
    pub present: bool,
    pub x: Pos,
    pub y: Pos,
}
/// Embedded by value in both `GposMarkToSingleSubtable` and
/// `GposMarkToLigatureSubtable`, not a `Subtable` union field itself.
pub type MarkArray = Vec<MarkRecord>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MarkRecord {
    pub glyph: GlyphHandle,
    pub mark_class: GlyphClass,
    pub anchor: Anchor,
}
// Never passed or embedded by value anywhere in the crate -- no
// `Copy`/`Clone` needed once `mark_array`/`base_array` own `Vec`s.
#[repr(C)]
pub struct GposMarkToSingleSubtable {
    pub class_count: GlyphClass,
    pub mark_array: MarkArray,
    pub base_array: BaseArray,
}
/// Embedded by value in `GposMarkToSingleSubtable`, not a `Subtable` union
/// field itself.
pub type BaseArray = Vec<BaseRecord>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseRecord {
    pub glyph: GlyphHandle,
    pub anchors: *mut Anchor,
}
pub type GposCursiveSubtable = Vec<GposCursiveEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposCursiveEntry {
    pub target: GlyphHandle,
    pub enter: Anchor,
    pub exit: Anchor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposPairSubtable {
    pub first: *mut ClassDef,
    pub second: *mut ClassDef,
    pub first_values: *mut *mut PositionValue,
    pub second_values: *mut *mut PositionValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PositionValue {
    pub dx: Pos,
    pub dy: Pos,
    pub d_width: Pos,
    pub d_height: Pos,
}
pub type GposSingleSubtable = Vec<GposSingleEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposSingleEntry {
    pub target: GlyphHandle,
    pub value: PositionValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubReverseSubtable {
    pub match_count: TableId,
    pub input_index: TableId,
    pub match_0: *mut *mut Coverage,
    pub to: *mut Coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainingSubtable {
    pub type_0: ChainingType,
    pub c2rust_unnamed: ChainingBody,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ChainingBody {
    pub rule: ChainingRule,
    pub c2rust_unnamed: ChainingRuleSet,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainingRuleSet {
    pub rules_count: TableId,
    pub rules: *mut *mut ChainingRule,
    pub bc: *mut ClassDef,
    pub ic: *mut ClassDef,
    pub fc: *mut ClassDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainingRule {
    pub match_count: TableId,
    pub input_begins: TableId,
    pub input_ends: TableId,
    pub match_0: *mut *mut Coverage,
    pub apply_count: TableId,
    pub apply: *mut ChainLookupApplication,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainLookupApplication {
    pub index: TableId,
    pub lookup: LookupHandle,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ChainingType {
    Canonical = 0,
    Poly = 1,
    Classified = 2,
}
pub type GsubLigatureSubtable = Vec<GsubLigatureEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubLigatureEntry {
    pub from: *mut Coverage,
    pub to: GlyphHandle,
}
pub type GsubMultiSubtable = Vec<GsubMultiEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubMultiEntry {
    pub from: GlyphHandle,
    pub to: *mut Coverage,
}
pub type GsubSingleSubtable = Vec<GsubSingleEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubSingleEntry {
    pub from: GlyphHandle,
    pub to: GlyphHandle,
}
// `subtables: SubtableList`(`Vec<SubtablePtr>`)を値で持つため `Copy` を落とす。
// 常に `*mut`/`*const` 経由でしか触られない（値渡し・値コピーの箇所は無い）。
#[repr(C)]
pub struct Lookup {
    pub name: SdsRaw,
    pub type_0: LookupType,
    pub _offset: u32,
    pub flags: u16,
    pub subtables: SubtableList,
}
pub type SubtablePtr = *mut Subtable;
// 所有するポインタ配列（各要素は `Lookup.type_0` に応じた型で解釈される
// `*mut Subtable`）。分類その3: `Vec<*mut T>` への機械的置換に留め、
// 要素の `Box` 化は Stage 6-4 に送る。
pub type SubtableList = Vec<SubtablePtr>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChainingSubtableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut ChainingSubtable, *const ChainingSubtable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut ChainingSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GsubReverseSubtableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GsubReverseSubtable) -> ()>,
    pub copy: Option<
        unsafe extern "C" fn(*mut GsubReverseSubtable, *const GsubReverseSubtable) -> (),
    >,
    pub dispose: Option<unsafe extern "C" fn(*mut GsubReverseSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GsubReverseSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GsubReverseSubtable) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GposPairSubtableElementInterface {
    pub init: Option<unsafe extern "C" fn(*mut GposPairSubtable) -> ()>,
    pub copy:
        Option<unsafe extern "C" fn(*mut GposPairSubtable, *const GposPairSubtable) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GposPairSubtable) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GposPairSubtable>,
    pub free: Option<unsafe extern "C" fn(*mut GposPairSubtable) -> ()>,
}
pub type LookupPtr = *mut Lookup;
// 所有するポインタ配列。分類その3、`SubtableList` と同じ扱い。
pub type LookupList = Vec<LookupPtr>;
pub type LookupRef = *const Lookup;
// 所有しない参照配列（`LookupList` の要素を指すだけ）。分類その3。
pub type LookupRefList = Vec<LookupRef>;
// `lookups: LookupRefList`(`Vec<LookupRef>`)を値で持つため `Copy` を落とす。
// `Lookup` と同じく常に `*mut`/`*const` 経由。
#[repr(C)]
pub struct Feature {
    pub name: SdsRaw,
    pub lookups: LookupRefList,
}
pub type FeaturePtr = *mut Feature;
// 所有するポインタ配列。
pub type FeatureList = Vec<FeaturePtr>;
pub type FeatureRef = *const Feature;
// 所有しない参照配列（`FeatureList` の要素を指すだけ）。
pub type FeatureRefList = Vec<FeatureRef>;
// `required_feature`はポインタなので無関係、`features: FeatureRefList`
// (`Vec<FeatureRef>`)を値で持つため `Copy` を落とす。
#[repr(C)]
pub struct LanguageSystem {
    pub name: SdsRaw,
    pub required_feature: FeatureRef,
    pub features: FeatureRefList,
}
pub type LanguageSystemPtr = *mut LanguageSystem;
// 所有するポインタ配列。
pub type LangSystemList = Vec<LanguageSystemPtr>;
// 3つとも値でVecを持つため `Copy` を落とす。`Font.gsub`/`Font.gpos` は
// `*mut OtlTable` フィールドで、crate全体を通じて常にポインタ経由。
#[repr(C)]
pub struct OtlTable {
    pub lookups: LookupList,
    pub features: FeatureList,
    pub languages: LangSystemList,
}
#[inline]
unsafe extern "C" fn dispose_subtable_dependent(
    mut subtable_ref: *mut SubtablePtr,
    mut lookup: *const Lookup,
) {
    match (*lookup).type_0 {
        OTL_TYPE_GSUB_SINGLE => {
            subtable_gsub_single_free(*subtable_ref as *mut GsubSingleSubtable);
        }
        OTL_TYPE_GSUB_MULTIPLE => {
            subtable_gsub_multi_free(*subtable_ref as *mut GsubMultiSubtable);
        }
        OTL_TYPE_GSUB_ALTERNATE => {
            subtable_gsub_multi_free(*subtable_ref as *mut GsubMultiSubtable);
        }
        OTL_TYPE_GSUB_LIGATURE => {
            subtable_gsub_ligature_free(*subtable_ref as *mut GsubLigatureSubtable);
        }
        OTL_TYPE_GSUB_CHAINING => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_CHAINING.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GSUB_REVERSE => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GsubReverseSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GSUB_REVERSE.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GPOS_SINGLE => {
            subtable_gpos_single_free(*subtable_ref as *mut GposSingleSubtable);
        }
        OTL_TYPE_GPOS_PAIR => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GposPairSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_GPOS_PAIR.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GPOS_CURSIVE => {
            subtable_gpos_cursive_free(*subtable_ref as *mut GposCursiveSubtable);
        }
        OTL_TYPE_GPOS_CHAINING => {
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut ChainingSubtable) -> ()>,
                Option<unsafe extern "C" fn(*mut Subtable) -> ()>,
            >(I_SUBTABLE_CHAINING.free)
            .expect("non-null function pointer")(*subtable_ref);
        }
        OTL_TYPE_GPOS_MARK_TO_BASE => {
            subtable_gpos_mark_to_single_free(*subtable_ref as *mut GposMarkToSingleSubtable);
        }
        OTL_TYPE_GPOS_MARK_TO_MARK => {
            subtable_gpos_mark_to_single_free(*subtable_ref as *mut GposMarkToSingleSubtable);
        }
        OTL_TYPE_GPOS_MARK_TO_LIGATURE => {
            subtable_gpos_mark_to_ligature_free(*subtable_ref as *mut GposMarkToLigatureSubtable);
        }
        _ => {}
    };
}
// `SubtablePtr`単体の要素インターフェースは全フィールドNoneだったため削除
// （`SubtableList`の破棄は個々の要素ではなく型ごとdispatchする
// `dispose_subtable_dependent`が担う——下記参照）。
//
// `.dispose`/`.copy`/`.create`/`.free`（非dependent版）はcrate全体で一度も
// 呼ばれておらず削除。実際の破棄経路は全て`dispose_dependent`のみ。
//
// 破棄後に呼び出し元が enclosing 構造体自体を生の `free()` で解放する場合
// （`otfcc_delete_lookup`）は `.clear()` ではなく `*arr = Vec::new()` で
// バッキング配列ごと即座に解放する。`.clear()`は容量を保持したままなので
// `libc::free`（Dropを起動しない）と組み合わせるとバッキング配列がリークする。
pub(crate) unsafe fn otl_subtable_list_dispose_dependent(
    arr: *mut SubtableList,
    enclosure: *const Lookup,
) {
    if arr.is_null() {
        return;
    }
    for subtable in (*arr).iter_mut() {
        dispose_subtable_dependent(subtable as *mut SubtablePtr, enclosure);
    }
    *arr = Vec::new();
}
pub unsafe extern "C" fn otfcc_delete_lookup(lookup: *mut Lookup) {
    if lookup.is_null() {
        return;
    }
    otl_subtable_list_dispose_dependent(&raw mut (*lookup).subtables, lookup);
    sdsfree((*lookup).name);
    free(lookup as *mut ::core::ffi::c_void);
}
// `__caryll_allocate_clean`（callocの罠が当てはまらないゼロ埋めアロケータ）
// で新しい `Lookup` を確保し、フィールドを埋めてから呼び出し元が
// `LookupList` に push する。生存: `table/otl/{read,parse}.rs`から直接呼ぶ。
#[inline]
pub(crate) unsafe fn init_lookup_ptr(entry: *mut LookupPtr) {
    *entry = __caryll_allocate_clean(
        ::core::mem::size_of::<Lookup>() as usize,
        47 as ::core::ffi::c_ulong,
    ) as LookupPtr;
    (**entry).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (**entry).subtables = Vec::new();
}
#[inline]
pub(crate) unsafe fn dispose_lookup_ptr(entry: *mut LookupPtr) {
    otfcc_delete_lookup(*entry);
}
// `LookupPtr`単体の`.copy`（`otl_lookup_ptr_copy`、生ポインタのmemcpy）は
// `LookupList`自体の`.copy`（テーブル全体クローン、死んでいる）からしか
// 呼ばれておらず削除。
// テーブル全体の `.copy`（`otl_lookup_list_copy`、生存していた `LookupPtr`
// 単体copyと同じく死んでいる）は削除。`.dispose`は`dispose_otl`から生存
// （enclosing `OtlTable`が生の`free()`で解放される直前に呼ばれるため
// `SubtableList`と同じ理由で`.clear()`ではなくフルドロップにする）。
pub(crate) unsafe fn otl_lookup_list_dispose(arr: *mut LookupList) {
    if arr.is_null() {
        return;
    }
    for lookup in (*arr).iter_mut() {
        dispose_lookup_ptr(lookup as *mut LookupPtr);
    }
    *arr = Vec::new();
}
// 元の「スワップして末尾を切り詰め」ループを`Vec::retain`に素直に置き換え。
pub(crate) unsafe fn otl_lookup_list_filter_env(
    arr: *mut LookupList,
    fn_0: Option<unsafe extern "C" fn(*const LookupPtr, *mut ::core::ffi::c_void) -> bool>,
    env: *mut ::core::ffi::c_void,
) {
    (*arr).retain(|&item| {
        if fn_0.expect("non-null function pointer")(&item as *const LookupPtr, env) {
            true
        } else {
            dispose_lookup_ptr(&item as *const LookupPtr as *mut LookupPtr);
            false
        }
    });
}
// `LookupRef`単体の要素インターフェース(`otl_lookup_ref_init`/`_copy`/
// `_dispose`)は`LookupRefList`自体の死んだ`.copy`スロットからしか呼ばれて
// おらず削除——`.dispose`(`otl_lookup_ref_dispose`)も含め、`LookupRef`は
// 所有物を持たない（`LookupList`が指し先の`Lookup`を所有する）ため、
// disposeは何もしない。`Vec<LookupRef>`自身の`Drop`だけで十分。
// `LookupRefList`は所有物を持たない要素の配列。disposeはバッキング配列を
// 解放するだけ（要素そのものへの処理は不要）。`.copy`（テーブル全体クローン）
// は死んでいたため削除。
pub(crate) unsafe fn otl_lookup_ref_list_dispose(arr: *mut LookupRefList) {
    if arr.is_null() {
        return;
    }
    *arr = Vec::new();
}
// 元のスワップ&切り詰めループを`Vec::retain`に。要素のdisposeは無いので
// 述語の結果をそのまま`retain`の判定に使うだけで済む。
pub(crate) unsafe fn otl_lookup_ref_list_filter_env(
    arr: *mut LookupRefList,
    fn_0: Option<unsafe extern "C" fn(*const LookupRef, *mut ::core::ffi::c_void) -> bool>,
    env: *mut ::core::ffi::c_void,
) {
    (*arr).retain(|&item| fn_0.expect("non-null function pointer")(&item as *const LookupRef, env));
}
// `.replace`の唯一の呼び出し箇所(`table/otl/parse.rs`)は毎回、直前に
// `init_feature_ptr`で作った空のdestに対して呼ばれる——単純な move-assign
// で置き換え可能（旧`dispose`+`memcpy`と等価、Rustの代入が古い値を
// 正しくドロップする）。
pub(crate) unsafe fn otl_lookup_ref_list_replace(dst: *mut LookupRefList, src: LookupRefList) {
    *dst = src;
}
#[inline]
pub(crate) unsafe fn init_feature_ptr(feature: *mut FeaturePtr) {
    *feature = __caryll_allocate_clean(
        ::core::mem::size_of::<Feature>() as usize,
        61 as ::core::ffi::c_ulong,
    ) as FeaturePtr;
    (**feature).lookups = Vec::new();
}
#[inline]
pub(crate) unsafe fn dispose_feature_ptr(feature: *mut FeaturePtr) {
    if (*feature).is_null() {
        return;
    }
    if !(**feature).name.is_null() {
        sdsfree((**feature).name);
    }
    otl_lookup_ref_list_dispose(&raw mut (**feature).lookups);
    free(*feature as *mut ::core::ffi::c_void);
    *feature = ::core::ptr::null_mut::<Feature>();
}
// `FeaturePtr`単体の`.copy`(生ポインタmemcpy)は`FeatureList`の死んだ
// `.copy`からしか呼ばれておらず削除。
// テーブル全体の`.copy`（死んでいる）は削除。`.dispose`は`dispose_otl`から
// 生存（`SubtableList`/`LookupList`と同じ理由でフルドロップ）。
pub(crate) unsafe fn otl_feature_list_dispose(arr: *mut FeatureList) {
    if arr.is_null() {
        return;
    }
    for feature in (*arr).iter_mut() {
        dispose_feature_ptr(feature as *mut FeaturePtr);
    }
    *arr = Vec::new();
}
pub(crate) unsafe fn otl_feature_list_filter_env(
    arr: *mut FeatureList,
    fn_0: Option<unsafe extern "C" fn(*const FeaturePtr, *mut ::core::ffi::c_void) -> bool>,
    env: *mut ::core::ffi::c_void,
) {
    (*arr).retain(|&item| {
        if fn_0.expect("non-null function pointer")(&item as *const FeaturePtr, env) {
            true
        } else {
            dispose_feature_ptr(&item as *const FeaturePtr as *mut FeaturePtr);
            false
        }
    });
}
// `FeatureRef`単体の要素インターフェースは`FeatureRefList`の死んだ`.copy`
// からしか呼ばれておらず削除。`FeatureRef`は所有物を持たない
// （`FeatureList`が指し先の`Feature`を所有する）。
// `.replace`の唯一の呼び出し箇所(`table/otl/parse.rs`)は`init_language_ptr`
// で作った空のdestに対して呼ばれる——move-assignで置き換え可能。
pub(crate) unsafe fn otl_feature_ref_list_replace(dst: *mut FeatureRefList, src: FeatureRefList) {
    *dst = src;
}
// `LookupRefList`と同じく所有物を持たない要素の配列。
pub(crate) unsafe fn otl_feature_ref_list_dispose(arr: *mut FeatureRefList) {
    if arr.is_null() {
        return;
    }
    *arr = Vec::new();
}
pub(crate) unsafe fn otl_feature_ref_list_filter_env(
    arr: *mut FeatureRefList,
    fn_0: Option<unsafe extern "C" fn(*const FeatureRef, *mut ::core::ffi::c_void) -> bool>,
    env: *mut ::core::ffi::c_void,
) {
    (*arr).retain(|&item| fn_0.expect("non-null function pointer")(&item as *const FeatureRef, env));
}
#[inline]
pub(crate) unsafe fn init_language_ptr(language: *mut LanguageSystemPtr) {
    *language = __caryll_allocate_clean(
        ::core::mem::size_of::<LanguageSystem>() as usize,
        77 as ::core::ffi::c_ulong,
    ) as LanguageSystemPtr;
    (**language).features = Vec::new();
}
#[inline]
pub(crate) unsafe fn dispose_language_ptr(language: *mut LanguageSystemPtr) {
    if (*language).is_null() {
        return;
    }
    if !(**language).name.is_null() {
        sdsfree((**language).name);
    }
    otl_feature_ref_list_dispose(&raw mut (**language).features);
    free(*language as *mut ::core::ffi::c_void);
    *language = ::core::ptr::null_mut::<LanguageSystem>();
}
// テーブル全体の`.copy`（死んでいる）は削除。`.dispose`は`dispose_otl`から
// 生存（同じ理由でフルドロップ）。このコンテナだけ`.filter_env`スロットが
// 元から無い——言語システム自体は間引かれず、`.features`(FeatureRefList)
// だけが間引かれる。
pub(crate) unsafe fn otl_lang_system_list_dispose(arr: *mut LangSystemList) {
    if arr.is_null() {
        return;
    }
    for language in (*arr).iter_mut() {
        dispose_language_ptr(language as *mut LanguageSystemPtr);
    }
    *arr = Vec::new();
}
// `calloc`、`malloc`ではない: `init_otl`が`.lookups`/`.features`/`.languages`
// へ直接フィールド代入(`= Vec::new()`)するため、gaspと同じ罠が当てはまる。
#[inline]
pub(crate) unsafe fn table_otl_create() -> *mut OtlTable {
    let x: *mut OtlTable =
        calloc(1, ::core::mem::size_of::<OtlTable>() as usize) as *mut OtlTable;
    init_otl(x);
    x
}
#[inline]
unsafe fn init_otl(table: *mut OtlTable) {
    (*table).lookups = Vec::new();
    (*table).features = Vec::new();
    (*table).languages = Vec::new();
}
// enclosing `OtlTable`自体が直後に生の`free()`で解放されるため、3つとも
// `.clear()`ではなくフルドロップ。
#[inline]
pub(crate) unsafe fn table_otl_free(x: *mut OtlTable) {
    if x.is_null() {
        return;
    }
    otl_lookup_list_dispose(&raw mut (*x).lookups);
    otl_feature_list_dispose(&raw mut (*x).features);
    otl_lang_system_list_dispose(&raw mut (*x).languages);
    free(x as *mut ::core::ffi::c_void);
}
// テーブル全体の`.copy`（`table_otl_copy`、生ポインタのmemcpy）は
// crate全体で一度も呼ばれておらず削除——Vec所有下でのmemcpyは
// 3つの内側リストすべての二重解放になるため、`.clone()`への移植も不要。

#[cfg(test)]
mod tests {
    use super::*;

    // These are not internal labels: `name()` supplies the `"type"` string
    // otfccdump writes for every lookup, and the key otfccbuild matches a
    // lookup against when reading the JSON back. They are also *not* the
    // constants' own spelling -- the JSON says `gpos_mark_to_base` where the
    // constant is `OTL_TYPE_GPOS_MARK_TO_BASE` -- so they were copied from the
    // `tableNames` table this replaced and are pinned here rather than derived.
    #[test]
    fn lookup_type_names_are_the_json_strings() {
        for (t, name) in [
            (OTL_TYPE_UNKNOWN, c"unknown"),
            (OTL_TYPE_GSUB_UNKNOWN, c"gsub_unknown"),
            (OTL_TYPE_GSUB_SINGLE, c"gsub_single"),
            (OTL_TYPE_GSUB_MULTIPLE, c"gsub_multiple"),
            (OTL_TYPE_GSUB_ALTERNATE, c"gsub_alternate"),
            (OTL_TYPE_GSUB_LIGATURE, c"gsub_ligature"),
            (OTL_TYPE_GSUB_CONTEXT, c"gsub_context"),
            (OTL_TYPE_GSUB_CHAINING, c"gsub_chaining"),
            (OTL_TYPE_GSUB_EXTEND, c"gsub_extend"),
            (OTL_TYPE_GSUB_REVERSE, c"gsub_reverse"),
            (OTL_TYPE_GPOS_UNKNOWN, c"gpos_unknown"),
            (OTL_TYPE_GPOS_SINGLE, c"gpos_single"),
            (OTL_TYPE_GPOS_PAIR, c"gpos_pair"),
            (OTL_TYPE_GPOS_CURSIVE, c"gpos_cursive"),
            (OTL_TYPE_GPOS_MARK_TO_BASE, c"gpos_mark_to_base"),
            (OTL_TYPE_GPOS_MARK_TO_LIGATURE, c"gpos_mark_to_ligature"),
            (OTL_TYPE_GPOS_MARK_TO_MARK, c"gpos_mark_to_mark"),
            (OTL_TYPE_GPOS_CONTEXT, c"gpos_context"),
            (OTL_TYPE_GPOS_CHAINING, c"gpos_chaining"),
            (OTL_TYPE_GPOS_EXTEND, c"gpos_extend"),
        ] {
            assert_eq!(t.name(), name, "name for {t:?}");
        }
    }

    // The numbering is otfcc's own: the file's format number plus 16 for gsub or
    // 32 for gpos. `file_format` has to undo exactly that, because its result is
    // written straight into the lookup header.
    #[test]
    fn file_format_undoes_the_table_base() {
        assert_eq!(OTL_TYPE_GSUB_SINGLE.file_format(), 1);
        assert_eq!(OTL_TYPE_GSUB_REVERSE.file_format(), 8);
        assert_eq!(OTL_TYPE_GSUB_EXTEND.file_format(), 7);
        assert_eq!(OTL_TYPE_GPOS_SINGLE.file_format(), 1);
        assert_eq!(OTL_TYPE_GPOS_EXTEND.file_format(), 9);
        // The bases themselves are *not* above their own base -- C compares with
        // `>`, not `>=` -- so they carry no format number.
        assert_eq!(OTL_TYPE_UNKNOWN.file_format(), 0);
        assert_eq!(OTL_TYPE_GSUB_UNKNOWN.file_format(), 0);
        // Except `gpos_unknown`, and this one is a quirk kept on purpose: 32 is
        // not above gpos's base but it *is* above gsub's, so C's nested
        // comparisons read it as gsub format 16. Reachable only from a font
        // declaring a gpos lookup of format 0, which no version of the spec has
        // -- but the number would go straight into the lookup header, so it is
        // reproduced rather than tidied.
        assert_eq!(OTL_TYPE_GPOS_UNKNOWN.file_format(), 16);
    }

    // A lookup type comes out of the font as a 16-bit number added to a base,
    // and C keeps whatever that gives -- including values no variant names,
    // which is why this type is not an enum. The raw value is observable: an
    // unnamed lookup is called `lookup_<raw as %04x>_<index>` in the JSON.
    #[test]
    fn from_file_keeps_unnamed_types() {
        assert_eq!(
            LookupType::from_file(OTL_TYPE_GSUB_UNKNOWN, 1),
            OTL_TYPE_GSUB_SINGLE
        );
        assert_eq!(
            LookupType::from_file(OTL_TYPE_GPOS_UNKNOWN, 9),
            OTL_TYPE_GPOS_EXTEND
        );
        // gsub format 9 exists in no version of the spec otfcc knows; it stays
        // 25, gets no subtable, and reaches the output as `lookup_0019_…`.
        let unnamed = LookupType::from_file(OTL_TYPE_GSUB_UNKNOWN, 9);
        assert_eq!(unnamed.raw(), 25);
        assert_eq!(unnamed.name(), c"unknown");
        assert_eq!(LookupType::from_file(OTL_TYPE_GSUB_UNKNOWN, 0xffff).raw(), 65551);
        assert_eq!(::core::mem::size_of::<LookupType>(), 4);
    }
}
