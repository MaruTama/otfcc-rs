//! Third-party C libraries, transpiled alongside otfcc itself.
//!
//! `emyg_dtoa` must keep producing the exact digits it produces today:
//! every float in the JSON output goes through it, so swapping in Rust's
//! own float formatting would change the output bytes.

pub mod emyg_dtoa;
pub mod json;
pub mod json_builder;
