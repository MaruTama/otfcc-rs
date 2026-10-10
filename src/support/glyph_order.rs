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
/// `GlyphOrderPass::Unset` is the zero value: entries placed by
/// `set_glyph_order_by_gid` and `set_glyph_order_by_name` (the OTF path)
/// keep it. The state is meaningful, not padding: zero outranks every named
/// pass, so an entry placed by GID can never be escalated by one.
///
/// The type lives here rather than in `json_reader` -- where the values are
/// produced -- because this is the field it types.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum GlyphOrderPass {
    Unset = 0,
    GlyphOrder = 1,
    Notdef = 2,
    Cmap = 3,
    Glyf = 4,
}

#[derive(Debug)]
pub struct GlyphOrderEntry {
    pub gid: GlyphId,
    pub name: Vec<u8>,
    pub order_type: GlyphOrderPass,
    pub order_entry: u32,
}
/// `entries` owns every entry -- an arena nothing is ever removed from --
/// and `by_gid`/`by_name` hold indices into it. An entry can exist in
/// `by_name` alone for a while: a JSON-driven glyph-order entry starts with
/// a placeholder `gid` and is only inserted into `by_gid` once
/// `order_glyphs` (json_reader.rs) assigns it a real one.
///
/// `by_gid` is a `BTreeMap` so it iterates in gid order. `by_name` is a
/// `HashMap` because it is only point-looked-up; `order_glyphs`, which
/// needs entries in `(order_type, order_entry)` order, sorts explicitly.
#[derive(Debug)]
pub struct GlyphOrder {
    pub entries: Vec<GlyphOrderEntry>,
    pub by_gid: std::collections::BTreeMap<GlyphId, usize>,
    pub by_name: std::collections::HashMap<Vec<u8>, usize>,
}
// Returns an owned copy of the canonical name; most callers (the
// fire-and-forget `set_by_gid` calls in `support/aglfn.rs`/`table/post.rs`)
// just drop it.
pub(crate) fn set_glyph_order_by_gid(
    go: &mut GlyphOrder,
    gid: GlyphId,
    mut name: Vec<u8>,
) -> Vec<u8> {
    if let Some(&idx) = go.by_gid.get(&gid) {
        return go.entries[idx].name.clone();
    }
    let final_bytes: Vec<u8> = if go.by_name.contains_key(&name) {
        crate::bytesbuild!(b"$$gid", gid as i32)
    } else {
        ::core::mem::take(&mut name)
    };
    go.entries.push(GlyphOrderEntry {
        gid,
        name: final_bytes.clone(),
        order_type: GlyphOrderPass::Unset,
        order_entry: 0,
    });
    let idx = go.entries.len() - 1;
    go.by_gid.insert(gid, idx);
    go.by_name.insert(final_bytes.clone(), idx);
    return final_bytes;
}
// On the "already taken" path the caller's `name` is simply dropped.
pub(crate) fn set_glyph_order_by_name(go: &mut GlyphOrder, name: Vec<u8>, gid: GlyphId) -> bool {
    if go.by_name.contains_key(&name) {
        return false;
    }
    go.entries.push(GlyphOrderEntry {
        gid,
        name: name.clone(),
        order_type: GlyphOrderPass::Unset,
        order_entry: 0,
    });
    let idx = go.entries.len() - 1;
    go.by_gid.insert(gid, idx);
    go.by_name.insert(name, idx);
    return true;
}
pub(crate) fn gord_name_a_field_shared(
    go: &GlyphOrder,
    gid: GlyphId,
    field: &mut Vec<u8>,
) -> bool {
    match go.by_gid.get(&gid) {
        Some(&idx) => {
            *field = go.entries[idx].name.clone();
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
pub(crate) fn gord_consolidate_handle(go: &GlyphOrder, h: &mut GlyphHandle) -> bool {
    if h.state == HandleState::Consolidated {
        let name_bytes = h.name.clone();
        if let Some(&entry_idx) = go.by_name.get(&name_bytes) {
            let entry = &go.entries[entry_idx];
            *h = Handle::new(HandleState::Consolidated, entry.gid, entry.name.clone()) as GlyphHandle;
            return true;
        }
        // Fall back to a by_gid lookup, like the HANDLE_STATE_INDEX branch
        // below and `gord_name_a_field_shared`'s search. (Upstream otfcc
        // passed the wrong hash-handle selector here, so its fallback could
        // never find anything.)
        if let Some(&entry_idx) = go.by_gid.get(&h.index) {
            let entry = &go.entries[entry_idx];
            *h = Handle::new(HandleState::Consolidated, entry.gid, entry.name.clone()) as GlyphHandle;
            return true;
        }
    } else if h.state == HandleState::Name {
        let name_bytes = h.name.clone();
        if let Some(&entry_idx) = go.by_name.get(&name_bytes) {
            let entry = &go.entries[entry_idx];
            *h = Handle::new(HandleState::Consolidated, entry.gid, entry.name.clone()) as GlyphHandle;
            return true;
        }
    } else if h.state == HandleState::Index {
        let mut name: Vec<u8> = Vec::new();
        gord_name_a_field_shared(go, h.index, &mut name);
        if !name.is_empty() {
            let idx = h.index;
            *h = Handle::new(HandleState::Consolidated, idx, name) as GlyphHandle;
            return true;
        }
    }
    return false;
}
pub(crate) fn gord_lookup_name(go: &GlyphOrder, name: Vec<u8>) -> bool {
    go.by_name.contains_key(&name)
}
#[cfg(test)]
mod tests {
    use super::*;

    // The passes are a priority, so `Ord` is the whole point of the type -- but
    // derived `Ord` compares by declaration order, which is only the encoding
    // because the declarations happen to be in ascending order. Pin that, and
    // pin the zero: `GlyphOrderPass::Unset` is the zero value, the pass an
    // entry placed by GID keeps.
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
