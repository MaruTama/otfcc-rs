use crate::support::primitives::{Arity};
use crate::libcff::cff_charset::{CffCharset};
use crate::libcff::cff_parser::{CffEncodingType};
use crate::libcff::cff_fdselect::{CffFdSelect};
use crate::libcff::cff_index::{CffIndex};
use crate::libcff::cff_value::{CffValue};
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
/// arrives as [`CffValue`](crate::libcff::cff_value::CffValue)'s integer arm
/// -- and because they are numbers, not a closed set: nothing enumerates them,
/// and an operator otfcc does not know simply matches no arm.
///
/// **Not interchangeable with [`CffCharstringOperator`]**, even though both are
/// `i32`: 38 of the numbers mean one thing in a DICT and something else in a
/// CharString (`op_Notice` and `op_hstem` are both 1, `op_FDArray` and
/// `op_hflex1` are both 3108). The names are all distinct and the two sets are
/// used by disjoint code, but the compiler cannot tell them apart while they are
/// aliases. Making them newtypes would -- see rust/README.md's next steps.
pub type CffDictOperator = i32;

pub const op_FontName: CffDictOperator = 3110;

pub const op_FDSelect: CffDictOperator = 3109;

pub const op_FDArray: CffDictOperator = 3108;

pub const op_UIDBase: CffDictOperator = 3107;

pub const op_CIDCount: CffDictOperator = 3106;

pub const op_CIDFontType: CffDictOperator = 3105;

pub const op_CIDFontRevision: CffDictOperator = 3104;

pub const op_CIDFontVersion: CffDictOperator = 3103;

pub const op_ROS: CffDictOperator = 3102;

pub const op_maxstack: CffDictOperator = 25;

pub const op_vstore: CffDictOperator = 24;

pub const op_BaseFontBlend: CffDictOperator = 3095;

pub const op_blend: CffDictOperator = 23;

pub const op_BaseFontName: CffDictOperator = 3094;

pub const op_vsindex: CffDictOperator = 22;

pub const op_PostScript: CffDictOperator = 3093;

pub const op_nominalWidthX: CffDictOperator = 21;

pub const op_SyntheicBase: CffDictOperator = 3092;

pub const op_defaultWidthX: CffDictOperator = 20;

pub const op_initialRandomSeed: CffDictOperator = 3091;

pub const op_Subrs: CffDictOperator = 19;

pub const op_ExpansionFactor: CffDictOperator = 3090;

pub const op_Private: CffDictOperator = 18;

pub const op_LanguageGroup: CffDictOperator = 3089;

pub const op_CharStrings: CffDictOperator = 17;

pub const op_Encoding: CffDictOperator = 16;

pub const op_charset: CffDictOperator = 15;

pub const op_ForceBold: CffDictOperator = 3086;

pub const op_XUID: CffDictOperator = 14;

pub const op_StemSnapV: CffDictOperator = 3085;

pub const op_UniqueID: CffDictOperator = 13;

pub const op_StemSnapH: CffDictOperator = 3084;

pub const op_BlueFuzz: CffDictOperator = 3083;

pub const op_StdVW: CffDictOperator = 11;

pub const op_BlueShift: CffDictOperator = 3082;

pub const op_StdHW: CffDictOperator = 10;

pub const op_BlueScale: CffDictOperator = 3081;

pub const op_FamilyOtherBlues: CffDictOperator = 9;

pub const op_StrokeWidth: CffDictOperator = 3080;

pub const op_FamilyBlues: CffDictOperator = 8;

pub const op_FontMatrix: CffDictOperator = 3079;

pub const op_OtherBlues: CffDictOperator = 7;

pub const op_CharstringType: CffDictOperator = 3078;

pub const op_BlueValues: CffDictOperator = 6;

pub const op_PaintType: CffDictOperator = 3077;

pub const op_FontBBox: CffDictOperator = 5;

pub const op_UnderlineThickness: CffDictOperator = 3076;

pub const op_Weight: CffDictOperator = 4;

pub const op_UnderlinePosition: CffDictOperator = 3075;

pub const op_FamilyName: CffDictOperator = 3;

pub const op_ItalicAngle: CffDictOperator = 3074;

pub const op_FullName: CffDictOperator = 2;

pub const op_isFixedPitch: CffDictOperator = 3073;

pub const op_Notice: CffDictOperator = 1;

pub const op_Copyright: CffDictOperator = 3072;

pub const op_version: CffDictOperator = 0;

/// A Type 2 CharString operator, encoded like [`CffDictOperator`] and, as noted
/// there, sharing 38 of its numbers while meaning something else by them.
pub type CffCharstringOperator = i32;

pub const op_flex1: CffCharstringOperator = 3109;

pub const op_hflex1: CffCharstringOperator = 3108;

pub const op_flex: CffCharstringOperator = 3107;

pub const op_hflex: CffCharstringOperator = 3106;

pub const op_hvcurveto: CffCharstringOperator = 31;

pub const op_roll: CffCharstringOperator = 3102;

pub const op_vhcurveto: CffCharstringOperator = 30;

pub const op_index: CffCharstringOperator = 3101;

pub const op_callgsubr: CffCharstringOperator = 29;

pub const op_exch: CffCharstringOperator = 3100;

pub const op_dup: CffCharstringOperator = 3099;

pub const op_hhcurveto: CffCharstringOperator = 27;

pub const op_sqrt: CffCharstringOperator = 3098;

pub const op_vvcurveto: CffCharstringOperator = 26;

pub const op_rlinecurve: CffCharstringOperator = 25;

pub const op_mul: CffCharstringOperator = 3096;

pub const op_rcurveline: CffCharstringOperator = 24;

pub const op_random: CffCharstringOperator = 3095;

pub const op_vstemhm: CffCharstringOperator = 23;

pub const op_ifelse: CffCharstringOperator = 3094;

pub const op_hmoveto: CffCharstringOperator = 22;

pub const op_get: CffCharstringOperator = 3093;

pub const op_rmoveto: CffCharstringOperator = 21;

pub const op_put: CffCharstringOperator = 3092;

pub const op_cntrmask: CffCharstringOperator = 20;

pub const op_hintmask: CffCharstringOperator = 19;

pub const op_drop: CffCharstringOperator = 3090;

pub const op_hstemhm: CffCharstringOperator = 18;

pub const op_cff2blend: CffCharstringOperator = 16;

pub const op_eq: CffCharstringOperator = 3087;

pub const op_cff2vsidx: CffCharstringOperator = 15;

pub const op_neg: CffCharstringOperator = 3086;

pub const op_endchar: CffCharstringOperator = 14;

pub const op_div: CffCharstringOperator = 3084;

pub const op_sub: CffCharstringOperator = 3083;

pub const op_return: CffCharstringOperator = 11;

pub const op_add: CffCharstringOperator = 3082;

pub const op_callsubr: CffCharstringOperator = 10;

pub const op_abs: CffCharstringOperator = 3081;

pub const op_rrcurveto: CffCharstringOperator = 8;

pub const op_vlineto: CffCharstringOperator = 7;

pub const op_hlineto: CffCharstringOperator = 6;

pub const op_not: CffCharstringOperator = 3077;

pub const op_rlineto: CffCharstringOperator = 5;

pub const op_or: CffCharstringOperator = 3076;

pub const op_vmoveto: CffCharstringOperator = 4;

pub const op_and: CffCharstringOperator = 3075;

pub const op_vstem: CffCharstringOperator = 3;

pub const op_hstem: CffCharstringOperator = 1;

// The Type 2 charstring spec's implementation limits. C gave them a
// `cff_Type2Limits` type of their own, but they are not a set of anything --
// they are six unrelated capacities, only ever compared against a count -- so
// each is typed as whatever it is compared with instead, which is what removes
// the casts at the fifteen sites that use them. `type2_charstring_len` and
// `type2_stem_hints` are the two otfcc never checks; they stay because the
// spec's table is easier to verify whole than with holes in it.
/// Size of [`CffStack::transient`], which it declares -- so the modulus that
/// wraps an index into that array cannot drift from the array itself.
pub const type2_transient_array: usize = 32;
pub const type2_max_subrs: u32 = 65300;
pub const type2_charstring_len: u32 = 65535;
pub const type2_subr_nesting: u32 = 10;
pub const type2_stem_hints: u32 = 96;
pub const type2_argument_stack: u32 = 48;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffHeader {
    pub major: u8,
    pub minor: u8,
    pub hdrSize: u8,
    pub offSize: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffEncodingFormat0 {
    pub format: u8,
    pub ncodes: u8,
    pub code: *mut u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffEncodingRangeFormat1 {
    pub first: u8,
    pub nleft: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffEncodingFormat1 {
    pub format: u8,
    pub nranges: u8,
    pub range1: *mut CffEncodingRangeFormat1,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffEncodingSupplement {
    pub code: u8,
    pub glyph: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffEncodingNs {
    pub nsup: u8,
    pub supplement: *mut CffEncodingSupplement,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffEncoding {
    pub t: CffEncodingType,
    pub c2rust_unnamed: CffEncodingBody,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union CffEncodingBody {
    pub f0: CffEncodingFormat0,
    pub f1: CffEncodingFormat1,
    pub ns: CffEncodingNs,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffStack {
    pub stack: *mut CffValue,
    pub transient: [CffValue; type2_transient_array],
    pub index: Arity,
    pub max: Arity,
    pub stem: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffFile {
    pub raw_data: *mut u8,
    pub raw_length: u32,
    pub cnt_glyph: u16,
    pub head: CffHeader,
    pub name: CffIndex,
    pub top_dict: CffIndex,
    pub string: CffIndex,
    pub global_subr: CffIndex,
    pub encodings: CffEncoding,
    pub charsets: CffCharset,
    pub fdselect: CffFdSelect,
    pub char_strings: CffIndex,
    pub font_dict: CffIndex,
    pub local_subr: CffIndex,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffIOutlineBuilder {
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
