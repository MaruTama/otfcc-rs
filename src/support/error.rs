//! Errors for writing a font.

use core::fmt;

/// Why a font could not be written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A table needs more entries than its format can count.
    TooManyEntries {
        /// What is being counted, e.g. "CFF INDEX".
        what: &'static str,
        count: usize,
        max: usize,
    },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::TooManyEntries { what, count, max } => {
                write!(f, "{what} has {count} entries, but the format allows at most {max}")
            }
        }
    }
}

impl std::error::Error for BuildError {}
