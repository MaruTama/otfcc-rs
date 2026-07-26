#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
/// What a [`cff_Value`] holds: an operator, or a number in one of the two forms
/// CFF encodes.
///
/// One value, two spellings. C declares each number twice -- `cff_OPERATOR` for
/// code reading a DICT and `CS2_OPERATOR` for code reading a CharString -- so
/// that each reader can use its own vocabulary for the same three states. Rust
/// cannot give one value two variant names, so the DICT names are the variants
/// and the CharString names are consts equal to them.
///
/// `cff_UNSET` is a name this port adds. C had none: it wrote
/// `(cff_Value_Type)0` in six struct initialisers and let `calloc` supply it
/// everywhere else. The state is real and reachable, not padding --
/// `cff_iDict.parseDictKey` resets `context.res.t` to it to mean "key not
/// found", and such a value reaching [`cffnum`] returns 0.0 because it matches
/// neither number arm. An `enum` without a zero variant would make every one of
/// those initialisers instant UB, so the zero gets a name.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum cff_Value_Type {
    cff_UNSET = 0,
    cff_OPERATOR = 1,
    cff_INTEGER = 2,
    cff_DOUBLE = 3,
}
pub use cff_Value_Type::*;

/// The CharString reader's name for [`cff_OPERATOR`].
pub const CS2_OPERATOR: cff_Value_Type = cff_OPERATOR;
/// The CharString reader's name for [`cff_INTEGER`] — an operand still in
/// integer form. `cff_decodeCS2Token` converts every one of these to
/// [`CS2_FRACTION`] before it returns, so it is only ever seen mid-decode.
pub const CS2_OPERAND: cff_Value_Type = cff_INTEGER;
/// The CharString reader's name for [`cff_DOUBLE`].
pub const CS2_FRACTION: cff_Value_Type = cff_DOUBLE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Value {
    pub t: cff_Value_Type,
    pub c2rust_unnamed: cff_ValueBody,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cff_ValueBody {
    pub i: i32,
    pub d: ::core::ffi::c_double,
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cffnum(mut val: cff_Value) -> ::core::ffi::c_double {
    if val.t as ::core::ffi::c_uint == cff_INTEGER as ::core::ffi::c_int as ::core::ffi::c_uint {
        return val.c2rust_unnamed.i as ::core::ffi::c_double;
    }
    if val.t as ::core::ffi::c_uint == cff_DOUBLE as ::core::ffi::c_int as ::core::ffi::c_uint {
        return val.c2rust_unnamed.d;
    }
    return 0 as ::core::ffi::c_int as ::core::ffi::c_double;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Six names, four states. The DICT reader and the CharString reader each have
    // their own word for the same three of them, which is why half of these are
    // consts rather than variants -- and the code relies on the two spellings
    // being interchangeable: `cff_decodeCS2Token` writes `CS2_FRACTION` and
    // `cffnum` reads `cff_DOUBLE`.
    #[test]
    fn the_two_spellings_are_the_same_states() {
        assert_eq!(CS2_OPERATOR, cff_OPERATOR);
        assert_eq!(CS2_OPERAND, cff_INTEGER);
        assert_eq!(CS2_FRACTION, cff_DOUBLE);
        assert!(matches!(CS2_FRACTION, cff_DOUBLE));
    }

    // `cff_UNSET` is this port's name for a state C left nameless: six struct
    // initialisers wrote `(cff_Value_Type)0` and every `calloc`ed stack of values
    // starts there. It has to be a legal value of the type, and it has to be 0.
    #[test]
    fn unset_is_zero_and_legal() {
        assert_eq!(cff_UNSET as u32, 0);
        assert_eq!(
            [cff_OPERATOR as u32, cff_INTEGER as u32, cff_DOUBLE as u32],
            [1, 2, 3]
        );
        assert_eq!(::core::mem::size_of::<cff_Value_Type>(), 4);
        // An unset value is not a number, which is how a missing DICT key comes
        // back as 0.0 from `cffnum`.
        let unset = cff_Value { t: cff_UNSET, c2rust_unnamed: cff_ValueBody { i: 42 } };
        assert_eq!(unsafe { cffnum(unset) }, 0.0);
    }
}
