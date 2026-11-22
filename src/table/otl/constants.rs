pub const SCRIPT_LANGUAGE_SEPARATOR: u8 = b'_';
/// The `lookupFlag` bit saying that a `markFilteringSet` follows the
/// lookup's subtable offsets. It is not kept in `Lookup::flags`: a lookup
/// has it exactly when `Lookup::mark_filtering_set` is `Some`.
pub const USE_MARK_FILTERING_SET: u16 = 0x0010;
pub static LOOKUP_FLAGS_LABELS: [&str; 4] = [
    "rightToLeft",
    "ignoreBases",
    "ignoreLigatures",
    "ignoreMarks",
];
