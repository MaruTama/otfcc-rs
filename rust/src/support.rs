pub mod aglfn;
pub mod alloc;
pub mod base64;
pub mod binio;
pub mod buffer;
pub mod built_json;
pub mod ctype_compat;
pub mod fmt;
pub mod font_reader;
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

pub const EXIT_FAILURE: i32 = 1_i32;

pub const TRUE_0: i32 = 1_i32;

pub const FALSE_0: i32 = 0_i32;
