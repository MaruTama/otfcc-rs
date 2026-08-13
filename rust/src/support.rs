pub mod aglfn;
pub mod alloc;
pub mod base64;
pub mod binio;
pub mod buffer;
pub mod built_json;
pub mod ctype_compat;
pub mod getopt;
pub mod glyph_order;
pub mod handle;
pub mod options;
pub mod parsed_json;
pub mod primitives;
pub mod sha1;
pub mod stdio;
pub mod stopwatch;
pub mod ttinstr;
pub mod unicodeconv;

// c2rust re-emitted these in every translation unit that included the C header
// defining them, since it has no way to refer to another file's copy. They are
// not otfcc's own vocabulary (that is `support::primitives`) -- just the pieces
// of the C standard library that `libc` does not carry.

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const TRUE_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const FALSE_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

/// The type of `qsort`'s `compar` argument, as `stdlib.h` spells it.
///
/// `libc::qsort` writes this signature out inline rather than naming it, but
/// otfcc's sort call sites `transmute` a concretely-typed comparator into it,
/// and they need something to name as the target.
pub type ComparFn = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
