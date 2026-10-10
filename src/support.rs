pub mod base64;
pub mod cli;
pub mod fmt;
pub mod glyph_order;
pub mod handle;
pub mod json_limits;
pub mod options;
pub mod primitives;
pub mod ttinstr;
pub mod unicode;

// Process exit status used by the binaries (C's `EXIT_FAILURE`). Not otfcc's
// own vocabulary -- that is `support::primitives`.

pub const EXIT_FAILURE: i32 = 1_i32;

pub const TRUE_0: i32 = 1_i32;

pub const FALSE_0: i32 = 0_i32;
