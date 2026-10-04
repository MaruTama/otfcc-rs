/// The kind of a JSON value. `ParsedValue::kind` reports it, and the
/// typed accessors (`ParsedValue::get_typed` and friends) take it to say
/// which kind they expect. `PreSerialized` is the build side's
/// already-serialized fragment, which no parsed value ever has.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum JsonType {
    None = 0,
    Object = 1,
    Array = 2,
    Integer = 3,
    Double = 4,
    String = 5,
    Boolean = 6,
    Null = 7,
    PreSerialized = 8,
}
