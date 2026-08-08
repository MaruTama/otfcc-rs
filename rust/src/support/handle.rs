#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
// `Handle` now owns a `Vec<u8>` name, so every `extern "C" fn` here that
// passes/returns `Handle` by value trips `improper_ctypes_definitions` --
// none of these are `#[no_mangle]` (the crate's only real FFI surface is
// the 4 symbols in `ffi/dll.rs`), so `extern "C"` here is c2rust's calling-
// convention residue, not real FFI. Same rationale as `CaretValueRecord`/
// `GsubLigatureSubtable` elsewhere in the crate.
#![allow(improper_ctypes_definitions)]
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::sds::{sdsfree, sdslen};

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
pub(crate) unsafe extern "C" fn otfcc_handle_empty() -> Handle {
    Handle::default()
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_copy(dst: *mut Handle, src: *const Handle) {
    *dst = (*src).clone();
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_copy_replace(dst: *mut Handle, src: Handle) {
    *dst = src.clone();
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_dup(src: Handle) -> Handle {
    src.clone()
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_init(x: *mut Handle) {
    *x = Handle::default();
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_dispose(x: *mut Handle) {
    *x = Handle::default();
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_replace(dst: *mut Handle, src: Handle) {
    *dst = src;
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_move(dst: *mut Handle, src: *mut Handle) {
    *dst = ::core::mem::take(&mut *src);
}
pub(crate) unsafe extern "C" fn handle_from_index(mut id: GlyphId) -> Handle {
    let mut h: Handle = Handle {
        state: HandleState::Index,
        index: id,
        name: Vec::new(),
    };
    return h;
}
// Callers still pass an owned `SdsRaw` here -- keeping these three
// functions' public signatures `SdsRaw`-in means none of their ~40 call
// sites across the crate need to change, only the conversion internals
// here. `handle_from_name` takes ownership of `s` (the same contract the
// old `h.name = s;` had), so it copies the bytes out and frees the
// now-redundant `sds` allocation; `handle_from_consolidated`/
// `handle_consolidate_to` only ever borrowed `s` (the caller already
// `sdsdup`'d before calling, and frees its own copy afterward), so they
// just copy the bytes without touching `s`'s lifetime.
pub(crate) unsafe fn sds_to_vec(s: SdsRaw) -> Vec<u8> {
    ::core::slice::from_raw_parts(s as *const u8, sdslen(s)).to_vec()
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
pub(crate) unsafe extern "C" fn handle_from_name(mut s: SdsRaw) -> Handle {
    let mut h: Handle = Handle {
        state: HandleState::Empty,
        index: 0 as GlyphId,
        name: Vec::new(),
    };
    if !s.is_null() {
        h.state = HandleState::Name;
        h.name = sds_to_vec(s);
        sdsfree(s);
    }
    return h;
}
pub(crate) unsafe extern "C" fn handle_from_consolidated(mut id: GlyphId, mut s: SdsRaw) -> Handle {
    let mut h: Handle = Handle {
        state: HandleState::Consolidated,
        index: id,
        name: sds_to_vec(s),
    };
    return h;
}
pub(crate) unsafe extern "C" fn handle_consolidate_to(
    mut h: *mut Handle,
    mut id: GlyphId,
    mut name: SdsRaw,
) {
    otfcc_handle_dispose(h as *mut Handle);
    (*h).state = HandleState::Consolidated;
    (*h).index = id;
    (*h).name = sds_to_vec(name);
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
