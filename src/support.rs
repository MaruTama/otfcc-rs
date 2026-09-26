pub mod base64;
pub mod buffer;
pub mod built_json;
pub mod cli;
pub mod cstd;
pub mod fmt;
pub mod font_reader;
pub mod glyph_order;
pub mod handle;
pub mod options;
pub mod parsed_json;
pub mod primitives;
pub mod ttinstr;
pub mod unicode;

// c2rust re-emitted these in every translation unit that included the C header
// defining them, since it has no way to refer to another file's copy. They are
// not otfcc's own vocabulary (that is `support::primitives`) -- just the pieces
// of the C standard library that `libc` does not carry.

pub const EXIT_FAILURE: i32 = 1_i32;

pub const TRUE_0: i32 = 1_i32;

pub const FALSE_0: i32 = 0_i32;
