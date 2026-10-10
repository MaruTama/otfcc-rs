#![forbid(unsafe_code)]
use crate::support::primitives::GlyphId;
use ::core::marker::PhantomData;

/// Which of `Handle`'s fields is meaningful.
///
/// No state comes from a font file, so there is no fallible conversion --
/// unlike, say, a lookup type read off the wire.
///
/// `#[repr(u32)]` keeps `Handle`'s layout exactly as the C struct's, and
/// the variants are re-exported below so the existing call sites keep spelling
/// them unqualified, the way the C code does.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HandleState {
    Empty = 0,
    Index = 1,
    Name = 2,
    Consolidated = 3,
}
/// `name` is `Vec<u8>`, not `String`: glyph/lookup names come from font
/// data and are not guaranteed valid UTF-8.
pub struct Handle<K = GlyphKind> {
    pub state: HandleState,
    pub index: GlyphId,
    pub name: Vec<u8>,
    kind: PhantomData<K>,
}
/// What a `Handle` refers to. Glyph, lookup and CFF font-dict handles share
/// one representation (state + index + name) but index different tables, so
/// the kind is part of the type: a lookup handle cannot be compared with, or
/// assigned to, a glyph handle. `Handle` without a parameter is a glyph handle.
#[derive(Copy, Clone, Debug)]
pub enum GlyphKind {}
#[derive(Copy, Clone, Debug)]
pub enum LookupKind {}
#[derive(Copy, Clone, Debug)]
pub enum FdKind {}
// Manual impls: deriving would bound `K: Clone + Debug`, which the marker
// types satisfy but which says nothing about what a handle needs.
impl<K> Clone for Handle<K> {
    fn clone(&self) -> Self {
        Handle {
            state: self.state,
            index: self.index,
            name: self.name.clone(),
            kind: PhantomData,
        }
    }
}
impl<K> ::core::fmt::Debug for Handle<K> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Handle")
            .field("state", &self.state)
            .field("index", &self.index)
            .field("name", &self.name)
            .finish()
    }
}
impl<K> Handle<K> {
    pub fn new(state: HandleState, index: GlyphId, name: Vec<u8>) -> Self {
        Handle { state, index, name, kind: PhantomData }
    }
}
pub type GlyphHandle = Handle<GlyphKind>;
pub type LookupHandle = Handle<LookupKind>;
pub type FdHandle = Handle<FdKind>;
impl<K> Default for Handle<K> {
    fn default() -> Self {
        Handle {
            state: HandleState::Empty,
            index: 0,
            name: Vec::new(),
            kind: PhantomData,
        }
    }
}
#[inline]
pub(crate) fn handle_empty<K>() -> Handle<K> {
    Handle::default()
}
pub(crate) fn handle_from_index<K>(id: GlyphId) -> Handle<K> {
    let h = Handle::new(HandleState::Index, id, Vec::new());
    return h;
}
/// NUL-truncating comparison for two `Vec<u8>`-shaped names (e.g. comparing
/// a `Handle.name` against a `Lookup.name`, both moved off `sds`) --
/// truncates *both* sides at their first embedded NUL, matching what
/// `strcmp`-via-`CStr` did back when one side was still a real C string.
pub(crate) fn handle_name_eq_bytes(a: &[u8], b: &[u8]) -> bool {
    let a_trunc = match a.iter().position(|&x| x == 0) {
        Some(p) => &a[..p],
        None => a,
    };
    let b_trunc = match b.iter().position(|&x| x == 0) {
        Some(p) => &b[..p],
        None => b,
    };
    a_trunc == b_trunc
}
// `None` leaves the handle in `HandleState::Empty`, while `Some(v)` always
// becomes `HandleState::Name` even when `v` is empty -- an empty-but-present
// name is a different state from no name at all.
pub(crate) fn handle_from_name<K>(s: Option<Vec<u8>>) -> Handle<K> {
    let mut h = Handle::new(HandleState::Empty, 0, Vec::new());
    if let Some(name) = s {
        h.state = HandleState::Name;
        h.name = name;
    }
    return h;
}

#[cfg(test)]
mod tests {
    use super::*;

    // The discriminants *are* the C ABI here: `Handle` is written into and
    // read out of fonts through code that was transpiled from C, and a shifted
    // discriminant would silently reinterpret every handle. The byte comparison
    // cannot see this on its own, because a font whose handles are all
    // consolidated by the time they are serialized exercises only one value.
    #[test]
    fn handle_state_discriminants_match_the_c_enum() {
        assert_eq!(HandleState::Empty as u32, 0);
        assert_eq!(HandleState::Index as u32, 1);
        assert_eq!(HandleState::Name as u32, 2);
        assert_eq!(HandleState::Consolidated as u32, 3);
    }

    // `#[repr(u32)]` is what keeps `Handle` laid out as the C struct.
    #[test]
    fn handle_state_is_a_u32() {
        assert_eq!(::core::mem::size_of::<HandleState>(), 4);
        assert_eq!(::core::mem::align_of::<HandleState>(), 4);
    }

    #[test]
    fn a_fresh_handle_is_empty() {
        let h: GlyphHandle = handle_empty();
        assert_eq!(h.state, HandleState::Empty);
        assert_eq!(h.index, 0);
        assert!(h.name.is_empty());
    }

    #[test]
    fn from_index_records_the_index_and_no_name() {
        let h: GlyphHandle = handle_from_index(42);
        assert_eq!(h.state, HandleState::Index);
        assert_eq!(h.index, 42);
        assert!(h.name.is_empty());
    }
}
