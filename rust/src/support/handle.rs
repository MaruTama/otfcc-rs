#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
use libc::{memcpy};
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Handle {
    pub state: HandleState,
    pub index: GlyphId,
    pub name: SdsRaw,
}
pub type GlyphHandle = Handle;
pub type LookupHandle = Handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HandlePackage {
    pub init: Option<unsafe extern "C" fn(*mut Handle) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut Handle, *const Handle) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut Handle, *mut Handle) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut Handle) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut Handle, Handle) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut Handle, Handle) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> Handle>,
    pub dup: Option<unsafe extern "C" fn(Handle) -> Handle>,
    pub fromIndex: Option<unsafe extern "C" fn(GlyphId) -> Handle>,
    pub fromName: Option<unsafe extern "C" fn(SdsRaw) -> Handle>,
    pub fromConsolidated: Option<unsafe extern "C" fn(GlyphId, SdsRaw) -> Handle>,
    pub consolidateTo: Option<unsafe extern "C" fn(*mut Handle, GlyphId, SdsRaw) -> ()>,
}
#[inline]
unsafe extern "C" fn init_handle(mut h: *mut Handle) {
    (*h).state = HandleState::Empty;
    (*h).index = 0 as GlyphId;
    (*h).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[inline]
unsafe extern "C" fn dispose_handle(mut h: *mut Handle) {
    if !(*h).name.is_null() {
        sdsfree((*h).name);
        (*h).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*h).index = 0 as GlyphId;
    (*h).state = HandleState::Empty;
}
unsafe extern "C" fn copy_handle(mut dst: *mut Handle, mut src: *const Handle) {
    (*dst).state = (*src).state;
    (*dst).index = (*src).index;
    if !(*src).name.is_null() {
        (*dst).name = sdsdup((*src).name);
    } else {
        (*dst).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_empty() -> Handle {
    let mut x: Handle = Handle {
        state: HandleState::Empty,
        index: 0,
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    otfcc_handle_init(&raw mut x);
    return x;
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_copy(mut dst: *mut Handle, mut src: *const Handle) {
    copy_handle(dst, src);
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_copy_replace(mut dst: *mut Handle, src: Handle) {
    otfcc_handle_dispose(dst);
    otfcc_handle_copy(dst, &raw const src);
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_dup(src: Handle) -> Handle {
    let mut dst: Handle = Handle {
        state: HandleState::Empty,
        index: 0,
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    otfcc_handle_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_init(mut x: *mut Handle) {
    init_handle(x);
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_dispose(mut x: *mut Handle) {
    dispose_handle(x as *mut Handle);
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_replace(mut dst: *mut Handle, src: Handle) {
    otfcc_handle_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Handle>() as usize,
    );
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_handle_move(mut dst: *mut Handle, mut src: *mut Handle) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Handle>() as usize,
    );
    otfcc_handle_init(src);
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
pub static OTFCC_I_HANDLE: HandlePackage = {
    HandlePackage {
        init: Some(otfcc_handle_init as unsafe extern "C" fn(*mut Handle) -> ()),
        copy: Some(
            otfcc_handle_copy as unsafe extern "C" fn(*mut Handle, *const Handle) -> (),
        ),
        move_0: Some(
            otfcc_handle_move as unsafe extern "C" fn(*mut Handle, *mut Handle) -> (),
        ),
        dispose: Some(otfcc_handle_dispose as unsafe extern "C" fn(*mut Handle) -> ()),
        replace: Some(
            otfcc_handle_replace as unsafe extern "C" fn(*mut Handle, Handle) -> (),
        ),
        copyReplace: Some(
            otfcc_handle_copy_replace as unsafe extern "C" fn(*mut Handle, Handle) -> (),
        ),
        empty: Some(otfcc_handle_empty),
        dup: Some(otfcc_handle_dup as unsafe extern "C" fn(Handle) -> Handle),
        fromIndex: Some(handle_from_index as unsafe extern "C" fn(GlyphId) -> Handle),
        fromName: Some(handle_from_name as unsafe extern "C" fn(SdsRaw) -> Handle),
        fromConsolidated: Some(
            handle_from_consolidated as unsafe extern "C" fn(GlyphId, SdsRaw) -> Handle,
        ),
        consolidateTo: Some(
            handle_consolidate_to as unsafe extern "C" fn(*mut Handle, GlyphId, SdsRaw) -> (),
        ),
    }
};

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
