//! otfcc — an OpenType font manipulation library, ported from C to Rust
//! (see RUST_MIGRATION.md).
//!
//! The only symbols any external caller uses are the four `extern "C"`
//! functions re-exported from [`ffi::dll`]; everything else is internal.
//!
//! Rough module map, for a newcomer orienting themselves in the source:
//! - [`font`]/[`table`] hold the in-memory font model, one submodule per
//!   OpenType table (`glyf`, `cff`, `fvar`, ...).
//! - [`otf_reader`]/[`otf_writer`] and [`json_reader`]/[`json_writer`]
//!   convert between that model and, respectively, binary OTF/TTF bytes
//!   and the JSON dump/build format otfcc's CLI exposes.
//! - [`consolidate`] resolves the reader's raw offset-based tables into
//!   the linked-up in-memory model; [`vf`] holds the variable-font
//!   (variation-quantity) math shared by several tables.
//! - [`libcff`] holds the lower-level CFF support: DICT and CharString
//!   codecs, INDEX reading and writing, and subroutinization. Binary tables
//!   are written with [`otfcc_binary::bk`], which lays a table out as
//!   blocks linked by offsets.
//! - [`support`] has shared primitives (numeric and glyph-id types, glyph
//!   handles, CLI `Options`); [`logger`], [`tag`] and [`version`] are
//!   smaller standalone utilities; [`ffi`] is the `extern "C"` boundary
//!   this crate is called through. `Buffer` lives in `otfcc_binary`.

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
pub mod tag;
pub mod version;
pub mod vf;
