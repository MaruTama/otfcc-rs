#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

//! otfcc — an OpenType font manipulation library, transpiled from C and
//! progressively rewritten in Rust (see rust/README.md).
//!
//! The only symbols any external caller uses are the four `extern "C"`
//! functions re-exported from [`ffi::dll`]; everything else is internal.

pub mod bk;
pub mod consolidate;
pub mod ffi;
pub mod font;
pub mod json_reader;
pub mod json_writer;
pub mod libcff;
pub mod logger;
pub mod otf_reader;
pub mod otf_writer;
pub mod support;
pub mod table;
pub mod vendor;
pub mod vf;
pub mod version;
