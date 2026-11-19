//! Contextual and chaining contextual lookups (GSUB 5/6, GPOS 7/8).
//!
//! A subtable is read into a rule set (`read`), split into one rule per
//! subtable for JSON (`dump`, `parse`), and merged back into class-based
//! rule sets where possible when building (`classifier`, `build`).
pub mod build;
pub mod classifier;
pub mod common;
pub mod dump;
pub mod parse;
pub mod read;
