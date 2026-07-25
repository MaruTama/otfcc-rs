pub mod aglfn;
pub mod alloc;
pub mod base64;
pub mod binio;
pub mod buffer;
pub mod ctype_compat;
pub mod cvec;
pub mod glyph_order;
pub mod handle;
pub mod json_ident;
pub mod options;
pub mod primitives;
pub mod sha1;
pub mod stdio;
pub mod stopwatch;
pub mod ttinstr;
pub mod unicodeconv;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
