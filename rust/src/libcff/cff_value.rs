#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
/// What a [`CffValue`] holds: an operator, or a number in one of the two forms
/// cff encodes.
///
/// One value, two spellings. C declares each number twice -- `CffValueType::Operator` for
/// code reading a DICT and `CS2_OPERATOR` for code reading a CharString -- so
/// that each reader can use its own vocabulary for the same three states. Rust
/// cannot give one value two variant names, so the DICT names are the variants
/// and the CharString names are consts equal to them.
///
/// `CffValueType::Unset` is a name this port adds. C had none: it wrote
/// `(CffValueType)0` in six struct initialisers and let `calloc` supply it
/// everywhere else. The state is real and reachable, not padding --
/// `parse_dict_key` resets `context.res.t` to it to mean "key not
/// found", and such a value reaching [`cffnum`] returns 0.0 because it matches
/// neither number arm. An `enum` without a zero variant would make every one of
/// those initialisers instant UB, so the zero gets a name.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CffValueType {
    Unset = 0,
    Operator = 1,
    Integer = 2,
    Double = 3,
}

/// The CharString reader's name for [`CffValueType::Operator`].
pub const CS2_OPERATOR: CffValueType = CffValueType::Operator;
/// The CharString reader's name for [`CffValueType::Integer`] — an operand still in
/// integer form. `cff_decode_cs2_token` converts every one of these to
/// [`CS2_FRACTION`] before it returns, so it is only ever seen mid-decode.
pub const CS2_OPERAND: CffValueType = CffValueType::Integer;
/// The CharString reader's name for [`CffValueType::Double`].
pub const CS2_FRACTION: CffValueType = CffValueType::Double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffValue {
    pub t: CffValueType,
    pub c2rust_unnamed: CffValueBody,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CffValueBody {
    pub i: i32,
    pub d: ::core::ffi::c_double,
}
pub unsafe fn cffnum(mut val: CffValue) -> ::core::ffi::c_double {
    if val.t as ::core::ffi::c_uint == CffValueType::Integer as ::core::ffi::c_int as ::core::ffi::c_uint {
        return val.c2rust_unnamed.i as ::core::ffi::c_double;
    }
    if val.t as ::core::ffi::c_uint == CffValueType::Double as ::core::ffi::c_int as ::core::ffi::c_uint {
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
    // being interchangeable: `cff_decode_cs2_token` writes `CS2_FRACTION` and
    // `cffnum` reads `CffValueType::Double`.
    #[test]
    fn the_two_spellings_are_the_same_states() {
        assert_eq!(CS2_OPERATOR, CffValueType::Operator);
        assert_eq!(CS2_OPERAND, CffValueType::Integer);
        assert_eq!(CS2_FRACTION, CffValueType::Double);
        assert!(matches!(CS2_FRACTION, CffValueType::Double));
    }

    // `CffValueType::Unset` is this port's name for a state C left nameless: six struct
    // initialisers wrote `(CffValueType)0` and every `calloc`ed stack of values
    // starts there. It has to be a legal value of the type, and it has to be 0.
    #[test]
    fn unset_is_zero_and_legal() {
        assert_eq!(CffValueType::Unset as u32, 0);
        assert_eq!(
            [CffValueType::Operator as u32, CffValueType::Integer as u32, CffValueType::Double as u32],
            [1, 2, 3]
        );
        assert_eq!(::core::mem::size_of::<CffValueType>(), 4);
        // An unset value is not a number, which is how a missing DICT key comes
        // back as 0.0 from `cffnum`.
        let unset = CffValue { t: CffValueType::Unset, c2rust_unnamed: CffValueBody { i: 42 } };
        assert_eq!(unsafe { cffnum(unset) }, 0.0);
    }
}
