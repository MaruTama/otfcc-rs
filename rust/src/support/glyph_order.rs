#![allow(unsafe_op_in_unsafe_fn)]
// Stage 6 removes this; see rust/README.md
// `GlyphOrderEntry.name` and every string-carrying `GlyphOrderPackage`
// vtable slot (`set_by_gid`/`set_by_name`/`lookup_name`/
// `name_a_field_shared`) now carry `Vec<u8>` across an `extern "C" fn`
// boundary -- none of these are `#[no_mangle]` (this crate's only real FFI
// surface is the 4 symbols in `ffi/dll.rs`), so `extern "C"` here is
// c2rust's calling-convention residue, not real FFI. Same rationale as
// `support/handle.rs`.
#![allow(improper_ctypes_definitions)]
use crate::support::handle::{GlyphHandle, Handle, HandleState};

use crate::support::primitives::GlyphId;
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

pub struct GlyphOrderEntry {
    pub gid: GlyphId,
    pub name: Vec<u8>,
    pub order_type: GlyphOrderPass,
    pub order_entry: u32,
}
/// Replaces the uthash-based dual index (`GlyphOrderEntry` used to carry
/// two independent `UtHashHandle`s, `hh_id`/`hh_name`, threading the same
/// heap-allocated entry into two separate uthash tables at once). The
/// individually-heap-allocated-and-aliased-by-raw-pointer shape those two
/// hash tables had doesn't map onto ownership Rust can check: `json_reader.
/// rs`'s `set_order_by_name`/`order_glyphs` pair shows an entry can
/// legitimately exist in `by_name` alone for a while (a JSON-driven
/// glyph-order entry starts with a placeholder `gid` and is only inserted
/// into `by_gid` once `order_glyphs` assigns it a real one), so `by_gid`
/// cannot simply be "the owner" the way the old disposal code (which only
/// ever walked `by_gid`) implicitly assumed. `entries` is the actual owner
/// now -- a single growing arena nothing is ever removed from -- and
/// `by_gid`/`by_name` hold plain `usize` indices into it, valid for as
/// long as `GlyphOrder` lives (an index survives handles from either map
/// referring to the same entry, since it's Copy and has no dangling-
/// pointer failure mode the way the old aliased raw pointers did).
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
pub struct GlyphOrder {
    pub entries: Vec<GlyphOrderEntry>,
    pub by_gid: std::collections::BTreeMap<GlyphId, usize>,
    pub by_name: std::collections::HashMap<Vec<u8>, usize>,
}
// No `Drop` impl needed: `entries: Vec<GlyphOrderEntry>` is the sole owner
// now (see the comment on the struct above), and a plain `Vec`'s own drop
// glue already frees every entry's `name: Vec<u8>` on the way down --
// `by_gid`/`by_name` hold non-owning `usize` indices, nothing for them to
// free. This is what let the old per-entry `__caryll_allocate_clean` +
// manual walk-and-free disposal go away entirely.
//
// This type is still constructed and freed as a bare `*mut GlyphOrder` in
// other places that are *not* `Font.glyph_order` (the `aglfn`/`gord` locals
// in `otf_reader/unconsolidate.rs`; `PostTable.post_name_map` was migrated
// off this path to an owned `Option<Box<GlyphOrder>>`, matching
// `Font.glyph_order`) -- those keep going through
// `otfcc_glyph_order_create`/`otfcc_glyph_order_free` unchanged, now backed
// by `Box::into_raw`/`Box::from_raw` (same shape as `otl_class_def_create`
// and the OTL subtable `_create()`s): there's no separate init/dispose step
// to call, since `Box::new` constructs the fields directly and dropping the
// `Box` on the way out already runs `entries`'s own `Vec` drop glue (which
// frees every entry's `name: Vec<u8>`) -- `by_gid`/`by_name` hold
// non-owning indices, nothing for them to free separately.
#[inline]
pub(crate) unsafe fn otfcc_glyph_order_free(x: *mut GlyphOrder) {
    if x.is_null() {
        return;
    }
    drop(Box::from_raw(x));
}
#[inline]
pub(crate) unsafe fn otfcc_glyph_order_create() -> *mut GlyphOrder {
    Box::into_raw(Box::new(GlyphOrder {
        entries: Vec::new(),
        by_gid: std::collections::BTreeMap::new(),
        by_name: std::collections::HashMap::new(),
    }))
}
// Returns an owned copy of the canonical name -- callers that discard the
// return value (the ~590 fire-and-forget `set_by_gid` calls in
// `support/aglfn.rs`/`table/post.rs`) simply drop it immediately, no leak,
// no code change needed there. `name` is `Vec<u8>` now instead of `SdsRaw`,
// so it drops on its own wherever this returns -- no explicit free needed
// in any branch.
pub(crate) unsafe fn otfcc_set_glyph_order_by_gid(
    go: *mut GlyphOrder,
    gid: GlyphId,
    mut name: Vec<u8>,
) -> Vec<u8> {
    if let Some(&idx) = (*go).by_gid.get(&gid) {
        return (&(*go).entries)[idx].name.clone();
    }
    let final_bytes: Vec<u8> = if (*go).by_name.contains_key(&name) {
        crate::bytesbuild!(b"$$gid", gid as i32)
    } else {
        ::core::mem::take(&mut name)
    };
    (*go).entries.push(GlyphOrderEntry {
        gid,
        name: final_bytes.clone(),
        order_type: GlyphOrderPass::Unset,
        order_entry: 0,
    });
    let idx = (*go).entries.len() - 1;
    (*go).by_gid.insert(gid, idx);
    (*go).by_name.insert(final_bytes.clone(), idx);
    return final_bytes;
}
// `name` is a caller-owned clone now (see the two `.clone()` call sites in
// `consolidate.rs`): on the "already taken" path it simply drops here,
// matching the original's "deliberately left un-freed" contract without
// needing a comment to explain why -- the caller's own copy was never
// touched, so there is nothing for it to double-free or leak.
pub(crate) unsafe fn otfcc_set_glyph_order_by_name(
    go: *mut GlyphOrder,
    name: Vec<u8>,
    gid: GlyphId,
) -> bool {
    if (*go).by_name.contains_key(&name) {
        return false;
    }
    (*go).entries.push(GlyphOrderEntry {
        gid,
        name: name.clone(),
        order_type: GlyphOrderPass::Unset,
        order_entry: 0,
    });
    let idx = (*go).entries.len() - 1;
    (*go).by_gid.insert(gid, idx);
    (*go).by_name.insert(name, idx);
    return true;
}
pub(crate) unsafe fn otfcc_gord_name_a_field_shared(
    go: *mut GlyphOrder,
    gid: GlyphId,
    field: *mut Vec<u8>,
) -> bool {
    match (*go).by_gid.get(&gid) {
        Some(&idx) => {
            *field = (&(*go).entries)[idx].name.clone();
            true
        }
        None => {
            *field = Vec::new();
            false
        }
    }
}
// Builds the consolidated `Handle` directly rather than through
// `handle_consolidate_to` (deleted -- it had no other callers by the time
// the `sds` sweep reached it) -- same simplification already used
// throughout the `consolidate/otl/*.rs` sweep, since the name is already
// the exact `Vec<u8>` a `Handle` wants.
pub(crate) unsafe fn otfcc_gord_consolidate_handle(
    go: *mut GlyphOrder,
    h: *mut GlyphHandle,
) -> bool {
    if (*h).state == HandleState::Consolidated {
        let name_bytes = (*h).name.clone();
        if let Some(&entry_idx) = (*go).by_name.get(&name_bytes) {
            let entry = &(&(*go).entries)[entry_idx];
            *h = Handle {
                state: HandleState::Consolidated,
                index: entry.gid,
                name: entry.name.clone(),
            } as GlyphHandle;
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
        if let Some(&entry_idx) = (*go).by_gid.get(&(*h).index) {
            let entry = &(&(*go).entries)[entry_idx];
            *h = Handle {
                state: HandleState::Consolidated,
                index: entry.gid,
                name: entry.name.clone(),
            } as GlyphHandle;
            return true;
        }
    } else if (*h).state == HandleState::Name {
        let name_bytes = (*h).name.clone();
        if let Some(&entry_idx) = (*go).by_name.get(&name_bytes) {
            let entry = &(&(*go).entries)[entry_idx];
            *h = Handle {
                state: HandleState::Consolidated,
                index: entry.gid,
                name: entry.name.clone(),
            } as GlyphHandle;
            return true;
        }
    } else if (*h).state == HandleState::Index {
        let mut name: Vec<u8> = Vec::new();
        otfcc_gord_name_a_field_shared(go, (*h).index, &raw mut name);
        if !name.is_empty() {
            let idx = (*h).index;
            *h = Handle {
                state: HandleState::Consolidated,
                index: idx,
                name,
            } as GlyphHandle;
            return true;
        }
    }
    return false;
}
pub(crate) unsafe fn gord_lookup_name(go: *mut GlyphOrder, name: Vec<u8>) -> bool {
    (*go).by_name.contains_key(&name)
}
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
        let all = [
            GlyphOrderPass::Unset,
            GlyphOrderPass::GlyphOrder,
            GlyphOrderPass::Notdef,
            GlyphOrderPass::Cmap,
            GlyphOrderPass::Glyf,
        ];
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
