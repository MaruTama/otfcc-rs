pub mod chaining;
pub mod extend;
pub mod gpos_common;
pub mod gpos_cursive;
pub mod gpos_mark_to_ligature;
pub mod gpos_mark_to_single;
pub mod gpos_pair;
pub mod gpos_single;
pub mod gsub_ligature;
pub mod gsub_multi;
pub mod gsub_reverse;
pub mod gsub_single;

bitflags::bitflags! {
    /// Hints from the lookup's surroundings that change *how* a subtable is
    /// encoded, never *what* it means.
    ///
    /// A bit set rather than an enum because that is how it is used:
    /// `getLookupHeuristics` starts from nothing and ors bits in as it finds
    /// reasons to. C's `OTL_BH_NORMAL = 0` is spelled `empty()` here -- naming
    /// the empty set invites `contains(NORMAL)`, which would be vacuously true.
    ///
    /// The one bit so far: the lookup is reachable from the `vert` feature, so
    /// `gsub_single` prefers coverage format 1 and gives up the
    /// constant-difference shortcut.
    ///
    /// `#[repr(transparent)]` is required, not decoration: this type is a
    /// parameter of the `extern "C"` subtable builders, and `bitflags`' struct
    /// is not FFI-safe without it (`improper_ctypes`, which
    /// `warnings = "deny"` turns into a build failure).
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    #[repr(transparent)]
    pub struct otl_BuildHeuristics: u32 {
        const GSUB_VERT = 1;
    }
}
