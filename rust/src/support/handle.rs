#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
// `Handle` now owns a `Vec<u8>` name, so every `extern "C" fn` here that
// passes/returns `Handle` by value trips `improper_ctypes_definitions` --
// none of these are `#[no_mangle]` (the crate's only real FFI surface is
// the 4 symbols in `ffi/dll.rs`), so `extern "C"` here is c2rust's calling-
// convention residue, not real FFI. Same rationale as `CaretValueRecord`/
// `GsubLigatureSubtable` elsewhere in the crate.
#![allow(improper_ctypes_definitions)]
use crate::support::primitives::{GlyphId};

/// Which of `Handle`'s fields is meaningful.
///
/// A real `enum` rather than c2rust's `pub type HandleState = c_uint` plus
/// four `pub const`s, so `state` cannot hold a value that is none of these and
/// a `match` on it is exhaustive. Every one of the ~50 assignments in the crate
/// is a struct literal naming one of these four, and none of them comes from a
/// font file, so there is nothing here that needs a fallible conversion --
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
/// `name` was a raw `sds` (`SdsRaw`) since the `Handle` pilot (PR #68) gave
/// it real `Drop`/`Clone` wrapped around `sdsfree`/`sdsdup` -- this PR
/// replaces the storage itself with `Vec<u8>` (not `String`: glyph/lookup
/// names come from font data and are not guaranteed valid UTF-8). `state`/
/// `index` are already `Copy`, so `#[derive(Clone)]` now composes
/// correctly on its own -- the manual `Clone`/`Drop` impls this struct
/// used to need (wrapping `sdsdup`/`sdsfree`) are gone; `Vec<u8>` already
/// has both.
#[derive(Clone)]
#[repr(C)]
pub struct Handle {
    pub state: HandleState,
    pub index: GlyphId,
    pub name: Vec<u8>,
}
pub type GlyphHandle = Handle;
pub type LookupHandle = Handle;
impl Default for Handle {
    fn default() -> Self {
        Handle {
            state: HandleState::Empty,
            index: 0,
            name: Vec::new(),
        }
    }
}
#[inline]
pub(crate) unsafe fn otfcc_handle_empty() -> Handle {
    Handle::default()
}
#[inline]
pub(crate) unsafe fn otfcc_handle_copy(dst: *mut Handle, src: *const Handle) {
    *dst = (*src).clone();
}
#[inline]
pub(crate) unsafe fn otfcc_handle_copy_replace(dst: *mut Handle, src: Handle) {
    *dst = src.clone();
}
#[inline]
pub(crate) unsafe fn otfcc_handle_dup(src: Handle) -> Handle {
    src.clone()
}
#[inline]
pub(crate) unsafe fn otfcc_handle_init(x: *mut Handle) {
    *x = Handle::default();
}
#[inline]
pub(crate) unsafe fn otfcc_handle_dispose(x: *mut Handle) {
    *x = Handle::default();
}
#[inline]
pub(crate) unsafe fn otfcc_handle_replace(dst: *mut Handle, src: Handle) {
    *dst = src;
}
#[inline]
pub(crate) unsafe fn otfcc_handle_move(dst: *mut Handle, src: *mut Handle) {
    *dst = ::core::mem::take(&mut *src);
}
pub(crate) unsafe fn handle_from_index(mut id: GlyphId) -> Handle {
    let mut h: Handle = Handle {
        state: HandleState::Index,
        index: id,
        name: Vec::new(),
    };
    return h;
}
/// Compares a `Handle.name`-shaped `Vec<u8>` against a null-terminated
/// `sds`/C string the way `strcmp(a.name, b.name) == 0` used to, before
/// `Handle.name` stopped being `SdsRaw`: truncates at the first embedded
/// NUL on the `Vec<u8>` side (matching `strcmp`'s own behavior on `other`).
/// Doesn't null-check `other` -- `strcmp` was already UB on a null argument,
/// so this preserves the original risk profile rather than adding a new one.
pub(crate) unsafe fn handle_name_eq_cstr(name: &[u8], other: *const ::core::ffi::c_char) -> bool {
    let name_trunc = match name.iter().position(|&b| b == 0) {
        Some(p) => &name[..p],
        None => name,
    };
    name_trunc == ::core::ffi::CStr::from_ptr(other).to_bytes()
}
/// Same NUL-truncating comparison as `handle_name_eq_cstr`, for the case
/// where both sides are now a `Vec<u8>`-shaped name (e.g. comparing a
/// `Handle.name` against a `Lookup.name` now that both have moved off
/// `sds`) -- truncates *both* sides at their first embedded NUL, matching
/// what `strcmp`-via-`CStr` did when one side was still a real C string.
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
// `s` is `Option<Vec<u8>>`, not a bare `Vec<u8>`, to preserve the exact
// null-vs-non-null distinction the old `SdsRaw` signature had: `None`
// (was: a null pointer) leaves the handle in `HandleState::Empty`, while
// `Some(v)` (was: any non-null `sds`, including a valid empty one) always
// becomes `HandleState::Name` even when `v` is empty -- an empty-but-
// present name is a different state from no name at all, and collapsing
// the two by testing `v.is_empty()` instead would be an observable (if
// exotic -- an empty-string glyph name) behavior change.
pub(crate) unsafe fn handle_from_name(mut s: Option<Vec<u8>>) -> Handle {
    let mut h: Handle = Handle {
        state: HandleState::Empty,
        index: 0 as GlyphId,
        name: Vec::new(),
    };
    if let Some(name) = s {
        h.state = HandleState::Name;
        h.name = name;
    }
    return h;
}
pub type FdHandle = Handle;

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
        unsafe {
            let h = otfcc_handle_empty();
            assert_eq!(h.state, HandleState::Empty);
            assert_eq!(h.index, 0);
            assert!(h.name.is_empty());
        }
    }

    #[test]
    fn from_index_records_the_index_and_no_name() {
        unsafe {
            let h = handle_from_index(42);
            assert_eq!(h.state, HandleState::Index);
            assert_eq!(h.index, 42);
            assert!(h.name.is_empty());
        }
    }
}
