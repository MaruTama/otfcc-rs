use crate::support::primitives::{arity_t};
use crate::libcff::cff_charset::{cff_Charset};
use crate::libcff::cff_parser::{cff_EncodingType};
use crate::libcff::cff_fdselect::{cff_FDSelect};
use crate::libcff::cff_index::{cff_Index};
use crate::libcff::cff_value::{cff_Value};
pub mod cff_charset;
pub mod cff_codecs;
pub mod cff_dict;
pub mod cff_fdselect;
pub mod cff_index;
pub mod cff_opmean;
pub mod cff_parser;
pub mod cff_string;
pub mod cff_value;
pub mod cff_writer;
pub mod charstring_il;
pub mod subr;

/// A CFF DICT operator, in otfcc's own encoding: the operator byte, or
/// `12 << 8 | b` for the two-byte operators the spec escapes with 12.
///
/// `i32` because that is what these are compared against -- a decoded operator
/// arrives as [`cff_Value`](crate::libcff::cff_value::cff_Value)'s integer arm
/// -- and because they are numbers, not a closed set: nothing enumerates them,
/// and an operator otfcc does not know simply matches no arm.
///
/// **Not interchangeable with [`cff_CharstringOperator`]**, even though both are
/// `i32`: 38 of the numbers mean one thing in a DICT and something else in a
/// CharString (`op_Notice` and `op_hstem` are both 1, `op_FDArray` and
/// `op_hflex1` are both 3108). The names are all distinct and the two sets are
/// used by disjoint code, but the compiler cannot tell them apart while they are
/// aliases. Making them newtypes would -- see rust/README.md's next steps.
pub type cff_DictOperator = i32;

pub const op_FontName: cff_DictOperator = 3110;

pub const op_FDSelect: cff_DictOperator = 3109;

pub const op_FDArray: cff_DictOperator = 3108;

pub const op_UIDBase: cff_DictOperator = 3107;

pub const op_CIDCount: cff_DictOperator = 3106;

pub const op_CIDFontType: cff_DictOperator = 3105;

pub const op_CIDFontRevision: cff_DictOperator = 3104;

pub const op_CIDFontVersion: cff_DictOperator = 3103;

pub const op_ROS: cff_DictOperator = 3102;

pub const op_maxstack: cff_DictOperator = 25;

pub const op_vstore: cff_DictOperator = 24;

pub const op_BaseFontBlend: cff_DictOperator = 3095;

pub const op_blend: cff_DictOperator = 23;

pub const op_BaseFontName: cff_DictOperator = 3094;

pub const op_vsindex: cff_DictOperator = 22;

pub const op_PostScript: cff_DictOperator = 3093;

pub const op_nominalWidthX: cff_DictOperator = 21;

pub const op_SyntheicBase: cff_DictOperator = 3092;

pub const op_defaultWidthX: cff_DictOperator = 20;

pub const op_initialRandomSeed: cff_DictOperator = 3091;

pub const op_Subrs: cff_DictOperator = 19;

pub const op_ExpansionFactor: cff_DictOperator = 3090;

pub const op_Private: cff_DictOperator = 18;

pub const op_LanguageGroup: cff_DictOperator = 3089;

pub const op_CharStrings: cff_DictOperator = 17;

pub const op_Encoding: cff_DictOperator = 16;

pub const op_charset: cff_DictOperator = 15;

pub const op_ForceBold: cff_DictOperator = 3086;

pub const op_XUID: cff_DictOperator = 14;

pub const op_StemSnapV: cff_DictOperator = 3085;

pub const op_UniqueID: cff_DictOperator = 13;

pub const op_StemSnapH: cff_DictOperator = 3084;

pub const op_BlueFuzz: cff_DictOperator = 3083;

pub const op_StdVW: cff_DictOperator = 11;

pub const op_BlueShift: cff_DictOperator = 3082;

pub const op_StdHW: cff_DictOperator = 10;

pub const op_BlueScale: cff_DictOperator = 3081;

pub const op_FamilyOtherBlues: cff_DictOperator = 9;

pub const op_StrokeWidth: cff_DictOperator = 3080;

pub const op_FamilyBlues: cff_DictOperator = 8;

pub const op_FontMatrix: cff_DictOperator = 3079;

pub const op_OtherBlues: cff_DictOperator = 7;

pub const op_CharstringType: cff_DictOperator = 3078;

pub const op_BlueValues: cff_DictOperator = 6;

pub const op_PaintType: cff_DictOperator = 3077;

pub const op_FontBBox: cff_DictOperator = 5;

pub const op_UnderlineThickness: cff_DictOperator = 3076;

pub const op_Weight: cff_DictOperator = 4;

pub const op_UnderlinePosition: cff_DictOperator = 3075;

pub const op_FamilyName: cff_DictOperator = 3;

pub const op_ItalicAngle: cff_DictOperator = 3074;

pub const op_FullName: cff_DictOperator = 2;

pub const op_isFixedPitch: cff_DictOperator = 3073;

pub const op_Notice: cff_DictOperator = 1;

pub const op_Copyright: cff_DictOperator = 3072;

pub const op_version: cff_DictOperator = 0;

/// A Type 2 CharString operator, encoded like [`cff_DictOperator`] and, as noted
/// there, sharing 38 of its numbers while meaning something else by them.
pub type cff_CharstringOperator = i32;

pub const op_flex1: cff_CharstringOperator = 3109;

pub const op_hflex1: cff_CharstringOperator = 3108;

pub const op_flex: cff_CharstringOperator = 3107;

pub const op_hflex: cff_CharstringOperator = 3106;

pub const op_hvcurveto: cff_CharstringOperator = 31;

pub const op_roll: cff_CharstringOperator = 3102;

pub const op_vhcurveto: cff_CharstringOperator = 30;

pub const op_index: cff_CharstringOperator = 3101;

pub const op_callgsubr: cff_CharstringOperator = 29;

pub const op_exch: cff_CharstringOperator = 3100;

pub const op_dup: cff_CharstringOperator = 3099;

pub const op_hhcurveto: cff_CharstringOperator = 27;

pub const op_sqrt: cff_CharstringOperator = 3098;

pub const op_vvcurveto: cff_CharstringOperator = 26;

pub const op_rlinecurve: cff_CharstringOperator = 25;

pub const op_mul: cff_CharstringOperator = 3096;

pub const op_rcurveline: cff_CharstringOperator = 24;

pub const op_random: cff_CharstringOperator = 3095;

pub const op_vstemhm: cff_CharstringOperator = 23;

pub const op_ifelse: cff_CharstringOperator = 3094;

pub const op_hmoveto: cff_CharstringOperator = 22;

pub const op_get: cff_CharstringOperator = 3093;

pub const op_rmoveto: cff_CharstringOperator = 21;

pub const op_put: cff_CharstringOperator = 3092;

pub const op_cntrmask: cff_CharstringOperator = 20;

pub const op_hintmask: cff_CharstringOperator = 19;

pub const op_drop: cff_CharstringOperator = 3090;

pub const op_hstemhm: cff_CharstringOperator = 18;

pub const op_cff2blend: cff_CharstringOperator = 16;

pub const op_eq: cff_CharstringOperator = 3087;

pub const op_cff2vsidx: cff_CharstringOperator = 15;

pub const op_neg: cff_CharstringOperator = 3086;

pub const op_endchar: cff_CharstringOperator = 14;

pub const op_div: cff_CharstringOperator = 3084;

pub const op_sub: cff_CharstringOperator = 3083;

pub const op_return: cff_CharstringOperator = 11;

pub const op_add: cff_CharstringOperator = 3082;

pub const op_callsubr: cff_CharstringOperator = 10;

pub const op_abs: cff_CharstringOperator = 3081;

pub const op_rrcurveto: cff_CharstringOperator = 8;

pub const op_vlineto: cff_CharstringOperator = 7;

pub const op_hlineto: cff_CharstringOperator = 6;

pub const op_not: cff_CharstringOperator = 3077;

pub const op_rlineto: cff_CharstringOperator = 5;

pub const op_or: cff_CharstringOperator = 3076;

pub const op_vmoveto: cff_CharstringOperator = 4;

pub const op_and: cff_CharstringOperator = 3075;

pub const op_vstem: cff_CharstringOperator = 3;

pub const op_hstem: cff_CharstringOperator = 1;

// The Type 2 charstring spec's implementation limits. C gave them a
// `cff_Type2Limits` type of their own, but they are not a set of anything --
// they are six unrelated capacities, only ever compared against a count -- so
// each is typed as whatever it is compared with instead, which is what removes
// the casts at the fifteen sites that use them. `type2_charstring_len` and
// `type2_stem_hints` are the two otfcc never checks; they stay because the
// spec's table is easier to verify whole than with holes in it.
/// Size of [`cff_Stack::transient`], which it declares -- so the modulus that
/// wraps an index into that array cannot drift from the array itself.
pub const type2_transient_array: usize = 32;
pub const type2_max_subrs: u32 = 65300;
pub const type2_charstring_len: u32 = 65535;
pub const type2_subr_nesting: u32 = 10;
pub const type2_stem_hints: u32 = 96;
pub const type2_argument_stack: u32 = 48;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Header {
    pub major: u8,
    pub minor: u8,
    pub hdrSize: u8,
    pub offSize: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingFormat0 {
    pub format: u8,
    pub ncodes: u8,
    pub code: *mut u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingRangeFormat1 {
    pub first: u8,
    pub nleft: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingFormat1 {
    pub format: u8,
    pub nranges: u8,
    pub range1: *mut cff_EncodingRangeFormat1,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingSupplement {
    pub code: u8,
    pub glyph: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_EncodingNS {
    pub nsup: u8,
    pub supplement: *mut cff_EncodingSupplement,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Encoding {
    pub t: cff_EncodingType,
    pub c2rust_unnamed: cff_EncodingBody,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union cff_EncodingBody {
    pub f0: cff_EncodingFormat0,
    pub f1: cff_EncodingFormat1,
    pub ns: cff_EncodingNS,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_Stack {
    pub stack: *mut cff_Value,
    pub transient: [cff_Value; type2_transient_array],
    pub index: arity_t,
    pub max: arity_t,
    pub stem: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_File {
    pub raw_data: *mut u8,
    pub raw_length: u32,
    pub cnt_glyph: u16,
    pub head: cff_Header,
    pub name: cff_Index,
    pub top_dict: cff_Index,
    pub string: cff_Index,
    pub global_subr: cff_Index,
    pub encodings: cff_Encoding,
    pub charsets: cff_Charset,
    pub fdselect: cff_FDSelect,
    pub char_strings: cff_Index,
    pub font_dict: cff_Index,
    pub local_subr: cff_Index,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_IOutlineBuilder {
    pub setWidth:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> ()>,
    pub newContour: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub lineTo: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub curveTo: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub setHint: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            bool,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub setMask: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> ()>,
    pub getrand: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_double>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // The two operator tables are both `i32` aliases, and 38 of their numbers
    // mean one thing in a DICT and something else entirely in a CharString. The
    // names never collide, and the two sets are read by disjoint code, so nothing
    // is wrong today -- but the compiler cannot see the distinction while they
    // are aliases, and this is the record of how much it would be covering if
    // they became newtypes.
    #[test]
    fn the_two_operator_tables_share_numbers() {
        assert_eq!(op_Notice, op_hstem);
        assert_eq!(op_FDArray, op_hflex1);
        assert_eq!(op_FamilyName, op_vstem);
        // Both encode a two-byte operator as `12 << 8 | b`.
        assert_eq!(op_FDArray, 12 << 8 | 36);
        assert_eq!(op_hflex1, 12 << 8 | 36);
        assert_eq!(op_version, 0);
    }
}
