#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{free, malloc};

use crate::support::handle::{handle_consolidate_to, Handle, GlyphHandle, HandleState};

use crate::support::alloc::{__caryll_allocate_clean};
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::sds::{sdsempty, sdsfree, sdslen};
/// Which pass of a JSON font's glyph naming placed a glyph, and therefore how
/// strongly it is placed: the *lowest* pass wins, because `set_order_by_name`
/// escalates an entry only when the new pass ranks below the one on record and
/// `_by_order` sorts ascending. That makes the ordering the meaning, so `Ord` is
/// derived -- and since it compares by *declaration* order, the variants are
/// declared in ascending discriminant order and
/// `glyphorderpass_order_is_its_encoding` pins that the two agree.
///
/// `GlyphOrderPass::Unset` is a name this port adds; C had none. Its `enum` lives inside
/// `json-reader.c` while this struct's field is a plain `uint8_t` in the shared
/// header, so the OTF path could leave the field at whatever `calloc` gave it --
/// and it does: `otfcc_set_glyph_order_by_gid` and `otfcc_set_glyph_order_by_name`
/// allocate an entry and set only `gid` and `name`. An enum without a zero
/// variant would make both of them UB. The state is meaningful, not padding:
/// zero outranks every named pass, so an entry placed by GID can never be
/// escalated by one.
///
/// The type lives here rather than in `json_reader` -- where C keeps it, and
/// where the values are still produced -- because this is the field it types.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum GlyphOrderPass {
    Unset = 0,
    GlyphOrder = 1,
    Notdef = 2,
    Cmap = 3,
    Glyf = 4,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphOrderEntry {
    pub gid: GlyphId,
    pub name: SdsRaw,
    pub order_type: GlyphOrderPass,
    pub order_entry: u32,
}
/// Replaces the uthash-based dual index (`GlyphOrderEntry` used to carry
/// two independent `UtHashHandle`s, `hh_id`/`hh_name`, threading the same
/// heap-allocated entry into two separate uthash tables at once). Each
/// entry is still individually heap-allocated and referenced by raw
/// pointer -- these two containers are non-owning indices over that same
/// set of allocations, not owners in their own right; disposal walks
/// `by_gid` once, frees each entry, then clears both maps (see
/// `dispose_glyph_order`).
///
/// `by_gid: BTreeMap`, not `HashMap`: no `HASH_SORT` ever existed on it,
/// but `order_glyphs` (json_reader.rs) rebuilds it from scratch by
/// inserting gids 0, 1, 2, ... in ascending order after sorting `by_name`,
/// and the OTF-read path (`otfcc_set_glyph_order_by_gid`) inserts in the
/// gid order its callers already iterate in -- so a `BTreeMap` reproduces
/// the original's effective iteration order exactly, without leaning on
/// incidental insertion order the way the uthash version implicitly did.
///
/// `by_name: HashMap`, not `BTreeMap`: it is only ever point-looked-up by
/// name day to day. The one place that needs a different order --
/// `order_glyphs`, sorting by `(order_type, order_entry)`, not
/// alphabetically -- already does its own explicit sort at the point of
/// use, the same "sort key != dedup key, defer the sort to drain time"
/// shape as `LookupHash`/`FeatureHash` earlier in this migration.
#[repr(C)]
pub struct GlyphOrder {
    pub by_gid: std::collections::BTreeMap<GlyphId, *mut GlyphOrderEntry>,
    pub by_name: std::collections::HashMap<Vec<u8>, *mut GlyphOrderEntry>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphOrderPackage {
    pub init: Option<unsafe extern "C" fn(*mut GlyphOrder) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GlyphOrder) -> ()>,
    pub create: Option<unsafe extern "C" fn() -> *mut GlyphOrder>,
    pub free: Option<unsafe extern "C" fn(*mut GlyphOrder) -> ()>,
    pub set_by_gid: Option<unsafe extern "C" fn(*mut GlyphOrder, GlyphId, SdsRaw) -> SdsRaw>,
    pub set_by_name: Option<unsafe extern "C" fn(*mut GlyphOrder, SdsRaw, GlyphId) -> bool>,
    pub name_a_field_shared:
        Option<unsafe extern "C" fn(*mut GlyphOrder, GlyphId, *mut SdsRaw) -> bool>,
    pub consolidate_handle:
        Option<unsafe extern "C" fn(*mut GlyphOrder, *mut GlyphHandle) -> bool>,
    pub lookup_name: Option<unsafe extern "C" fn(*mut GlyphOrder, SdsRaw) -> bool>,
}
#[inline]
unsafe extern "C" fn init_glyph_order(mut go: *mut GlyphOrder) {
    // Placement-construct, not a field assignment: the one live caller
    // (`otfcc_glyph_order_create`) hands this fresh `malloc`'d
    // (uninitialized) memory, so there is nothing to read or drop first.
    ::core::ptr::write(&raw mut (*go).by_gid, std::collections::BTreeMap::new());
    ::core::ptr::write(&raw mut (*go).by_name, std::collections::HashMap::new());
}
#[inline]
unsafe extern "C" fn dispose_glyph_order(mut go: *mut GlyphOrder) {
    // Every entry is discovered by walking `by_gid` (matching the
    // original's single-walk-frees-both-indices shape); `by_name` never
    // owns anything of its own to free, only a second reference to the
    // same allocations, so clearing it needs no walk.
    for (_, &entry) in (*go).by_gid.iter() {
        if !(*entry).name.is_null() {
            sdsfree((*entry).name);
        }
        free(entry as *mut ::core::ffi::c_void);
    }
    (*go).by_gid = std::collections::BTreeMap::new();
    (*go).by_name = std::collections::HashMap::new();
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_init(mut x: *mut GlyphOrder) {
    init_glyph_order(x);
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_dispose(mut x: *mut GlyphOrder) {
    dispose_glyph_order(x);
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_free(mut x: *mut GlyphOrder) {
    if x.is_null() {
        return;
    }
    otfcc_glyph_order_dispose(x);
    free(x as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn otfcc_glyph_order_create() -> *mut GlyphOrder {
    let mut x: *mut GlyphOrder =
        malloc(::core::mem::size_of::<GlyphOrder>() as usize) as *mut GlyphOrder;
    otfcc_glyph_order_init(x);
    return x;
}
unsafe extern "C" fn otfcc_set_glyph_order_by_gid(
    mut go: *mut GlyphOrder,
    mut gid: GlyphId,
    mut name: SdsRaw,
) -> SdsRaw {
    if let Some(&existing) = (*go).by_gid.get(&gid) {
        sdsfree(name);
        return (*existing).name;
    }
    let name_bytes = std::slice::from_raw_parts(name as *const u8, sdslen(name)).to_vec();
    if (*go).by_name.contains_key(&name_bytes) {
        sdsfree(name);
        name = crate::sdsbuild!(sdsempty(), b"$$gid", gid as ::core::ffi::c_int);
    }
    let mut s: *mut GlyphOrderEntry = __caryll_allocate_clean(
        ::core::mem::size_of::<GlyphOrderEntry>() as usize,
        36 as ::core::ffi::c_ulong,
    ) as *mut GlyphOrderEntry;
    (*s).gid = gid;
    (*s).name = name;
    (*go).by_gid.insert(gid, s);
    let final_name_bytes =
        std::slice::from_raw_parts((*s).name as *const u8, sdslen((*s).name)).to_vec();
    (*go).by_name.insert(final_name_bytes, s);
    return name;
}
unsafe extern "C" fn otfcc_set_glyph_order_by_name(
    mut go: *mut GlyphOrder,
    mut name: SdsRaw,
    mut gid: GlyphId,
) -> bool {
    let name_bytes = std::slice::from_raw_parts(name as *const u8, sdslen(name)).to_vec();
    if (*go).by_name.contains_key(&name_bytes) {
        return false;
    }
    let mut s: *mut GlyphOrderEntry = __caryll_allocate_clean(
        ::core::mem::size_of::<GlyphOrderEntry>() as usize,
        54 as ::core::ffi::c_ulong,
    ) as *mut GlyphOrderEntry;
    (*s).gid = gid;
    (*s).name = name;
    (*go).by_gid.insert(gid, s);
    (*go).by_name.insert(name_bytes, s);
    return true;
}
unsafe extern "C" fn otfcc_gord_name_a_field_shared(
    mut go: *mut GlyphOrder,
    mut gid: GlyphId,
    mut field: *mut SdsRaw,
) -> bool {
    match (*go).by_gid.get(&gid) {
        Some(&t) => {
            *field = (*t).name;
            true
        }
        None => {
            *field = ::core::ptr::null_mut::<::core::ffi::c_char>();
            false
        }
    }
}
unsafe extern "C" fn otfcc_gord_consolidate_handle(
    mut go: *mut GlyphOrder,
    mut h: *mut GlyphHandle,
) -> bool {
    if (*h).state == HandleState::Consolidated {
        let name_bytes =
            std::slice::from_raw_parts((*h).name as *const u8, sdslen((*h).name)).to_vec();
        if let Some(&t) = (*go).by_name.get(&name_bytes) {
            handle_consolidate_to(h as *mut Handle, (*t).gid, (*t).name);
            return true;
        }
        // Original C (glyph-order.c:83) passed the wrong hash-handle
        // selector here -- `HASH_FIND(hhName, go->byGID, &(h->index), ...)`
        // compared a gid against by_name's name-keyed entries, so this
        // fallback could never find anything (a name is essentially never
        // exactly sizeof(glyphid_t) bytes, and even then the compared
        // bytes are unrelated). The mirrored HANDLE_STATE_INDEX branch
        // below shows what this was clearly meant to do: fall back to a
        // by_gid lookup, exactly like otfcc_gord_name_a_field_shared's
        // already-correct search. Fixed here.
        if let Some(&t) = (*go).by_gid.get(&(*h).index) {
            handle_consolidate_to(h as *mut Handle, (*t).gid, (*t).name);
            return true;
        }
    } else if (*h).state == HandleState::Name {
        let name_bytes =
            std::slice::from_raw_parts((*h).name as *const u8, sdslen((*h).name)).to_vec();
        if let Some(&t) = (*go).by_name.get(&name_bytes) {
            handle_consolidate_to(h as *mut Handle, (*t).gid, (*t).name);
            return true;
        }
    } else if (*h).state == HandleState::Index {
        let mut name: SdsRaw = ::core::ptr::null_mut::<::core::ffi::c_char>();
        otfcc_gord_name_a_field_shared(go, (*h).index, &raw mut name);
        if !name.is_null() {
            handle_consolidate_to(h as *mut Handle, (*h).index, name);
            return true;
        }
    }
    return false;
}
unsafe extern "C" fn gord_lookup_name(mut go: *mut GlyphOrder, mut name: SdsRaw) -> bool {
    let name_bytes = std::slice::from_raw_parts(name as *const u8, sdslen(name)).to_vec();
    (*go).by_name.contains_key(&name_bytes)
}
pub static OTFCC_PKG_GLYPH_ORDER: GlyphOrderPackage = {
    GlyphOrderPackage {
        init: Some(otfcc_glyph_order_init as unsafe extern "C" fn(*mut GlyphOrder) -> ()),
        dispose: Some(
            otfcc_glyph_order_dispose as unsafe extern "C" fn(*mut GlyphOrder) -> (),
        ),
        create: Some(otfcc_glyph_order_create),
        free: Some(otfcc_glyph_order_free as unsafe extern "C" fn(*mut GlyphOrder) -> ()),
        set_by_gid: Some(
            otfcc_set_glyph_order_by_gid
                as unsafe extern "C" fn(*mut GlyphOrder, GlyphId, SdsRaw) -> SdsRaw,
        ),
        set_by_name: Some(
            otfcc_set_glyph_order_by_name
                as unsafe extern "C" fn(*mut GlyphOrder, SdsRaw, GlyphId) -> bool,
        ),
        name_a_field_shared: Some(
            otfcc_gord_name_a_field_shared
                as unsafe extern "C" fn(*mut GlyphOrder, GlyphId, *mut SdsRaw) -> bool,
        ),
        consolidate_handle: Some(
            otfcc_gord_consolidate_handle
                as unsafe extern "C" fn(*mut GlyphOrder, *mut GlyphHandle) -> bool,
        ),
        lookup_name: Some(
            gord_lookup_name as unsafe extern "C" fn(*mut GlyphOrder, SdsRaw) -> bool,
        ),
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    // The passes are a priority, so `Ord` is the whole point of the type -- but
    // derived `Ord` compares by declaration order, which is only the encoding
    // because the declarations happen to be in ascending order. Pin that, and
    // pin the zero: `otfcc_set_glyph_order_by_gid` calloc's an entry and never
    // assigns this field, so `GlyphOrderPass::Unset` has to be the all-zero value for the
    // field to be a valid `GlyphOrderPass` at all.
    #[test]
    fn glyphorderpass_order_is_its_encoding() {
        let all = [GlyphOrderPass::Unset, GlyphOrderPass::GlyphOrder, GlyphOrderPass::Notdef, GlyphOrderPass::Cmap, GlyphOrderPass::Glyf];
        for w in all.windows(2) {
            assert!(w[0] < w[1], "{:?} should rank above {:?}", w[0], w[1]);
            assert!((w[0] as u8) < (w[1] as u8));
        }
        assert_eq!(GlyphOrderPass::Unset as u8, 0);
        assert_eq!(GlyphOrderPass::GlyphOrder as u8, 1);
        assert_eq!(GlyphOrderPass::Notdef as u8, 2);
        assert_eq!(GlyphOrderPass::Cmap as u8, 3);
        assert_eq!(GlyphOrderPass::Glyf as u8, 4);
        assert_eq!(::core::mem::size_of::<GlyphOrderPass>(), 1);
    }
}
