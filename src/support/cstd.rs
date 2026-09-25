// Pieces of the C standard library that `libc` does not carry -- grouped
// here because they are that one family, not otfcc's own vocabulary (that
// is `support::primitives` and friends, deliberately left at `support/`'s
// top level). Matches the "not otfcc's own vocabulary" doc comment that
// used to sit at the bottom of `support.rs` itself before this split.
pub mod binio;
pub mod ctype_compat;
pub mod stdio;
pub mod strtol;
