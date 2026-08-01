#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use crate::support::primitives::{GlyphId};
use crate::vendor::sds::{SdsRaw};
use crate::vendor::sds::{sdsdup, sdsfree};

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
#[repr(C)]
pub struct Handle {
    pub state: HandleState,
    pub index: GlyphId,
    pub name: SdsRaw,
}
pub type GlyphHandle = Handle;
pub type LookupHandle = Handle;
impl Default for Handle {
    fn default() -> Self {
        Handle {
            state: HandleState::Empty,
            index: 0,
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        }
    }
}
/// Deep copy: duplicates `name` via `sdsdup` rather than aliasing the
/// pointer, so a `Handle` can never end up with two owners of the same
/// allocation. Every struct embedding a `Handle`/`GlyphHandle`/`LookupHandle`
/// field relies on this: `#[derive(Clone)]` on the outer struct composes
/// correctly (field-by-field `.clone()`) only because this impl is correct on
/// its own.
impl Clone for Handle {
    fn clone(&self) -> Self {
        unsafe {
            Handle {
                state: self.state,
                index: self.index,
                name: if self.name.is_null() {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                } else {
                    sdsdup(self.name)
                },
            }
        }
    }
}
/// The reason `Handle` was kept `Copy` everywhere in the crate until now:
/// once it owns `name` for real, every place that used to bitwise-copy a
/// `Handle` through a raw pointer (`let h = *ptr;`) needs to become an
/// explicit `.clone()` (duplicate) or a genuine move -- the compiler enforces
/// this at every call site (`cannot move out of ... which is behind a raw
/// pointer`), which is what made this conversion tractable to verify.
impl Drop for Handle {
    fn drop(&mut self) {
        if !self.name.is_null() {
            unsafe {
                sdsfree(self.name);
            }
            self.name = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    return h;
}
pub(crate) unsafe extern "C" fn handle_from_name(mut s: SdsRaw) -> Handle {
    let mut h: Handle = Handle {
        state: HandleState::Empty,
        index: 0 as GlyphId,
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if !s.is_null() {
        h.state = HandleState::Name;
        h.name = s;
    }
    return h;
}
pub(crate) unsafe extern "C" fn handle_from_consolidated(mut id: GlyphId, mut s: SdsRaw) -> Handle {
    let mut h: Handle = Handle {
        state: HandleState::Consolidated,
        index: id,
        name: sdsdup(s),
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
    (*h).name = sdsdup(name);
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
            assert!(h.name.is_null());
        }
    }

    #[test]
    fn from_index_records_the_index_and_no_name() {
        unsafe {
            let h = handle_from_index(42);
            assert_eq!(h.state, HandleState::Index);
            assert_eq!(h.index, 42);
            assert!(h.name.is_null());
        }
    }
}
