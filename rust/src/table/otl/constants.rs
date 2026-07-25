
#[unsafe(no_mangle)]
pub static SCRIPT_LANGUAGE_SEPARATOR: ::core::ffi::c_char = '_' as i32 as ::core::ffi::c_char;
pub static lookupFlagsLabels: [&::core::ffi::CStr; 4] = [
    c"rightToLeft",
    c"ignoreBases",
    c"ignoreLigatures",
    c"ignoreMarks",
];
