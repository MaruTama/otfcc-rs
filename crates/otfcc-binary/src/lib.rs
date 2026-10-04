//! otfcc's binary I/O: [`FontReader`] reads a table's bytes with every read
//! bounds-checked, [`Buffer`] writes big-endian data, and [`bk`] lays out a
//! graph of subtables linked by offsets and packs it into bytes.
//!
//! It knows nothing about any particular table; the table formats stay in
//! `otfcc_rust`.
#![forbid(unsafe_code)]

mod buffer;
pub mod bk;
mod reader;

pub use buffer::Buffer;
pub use reader::{FontReader, ReadError};
