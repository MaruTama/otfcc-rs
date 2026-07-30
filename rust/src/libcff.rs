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

/// A cff DICT operator, in otfcc's own encoding: the operator byte, or
/// `12 << 8 | b` for the two-byte operators the spec escapes with 12.
///
/// `i32` because that is what these are compared against -- a decoded operator
/// arrives as [`CffValue`](crate::libcff::cff_value::CffValue)'s integer arm
/// -- and because they are numbers, not a closed set: nothing enumerates them,
/// and an operator otfcc does not know simply matches no arm.
///
/// **Not interchangeable with [`CffCharstringOperator`]**, even though both are
/// `i32`: 38 of the numbers mean one thing in a DICT and something else in a
/// CharString (`OP_NOTICE` and `OP_HSTEM` are both 1, `OP_FD_ARRAY` and
/// `OP_HFLEX1` are both 3108). The names are all distinct and the two sets are
/// used by disjoint code, but the compiler cannot tell them apart while they are
/// aliases. Making them newtypes would -- see rust/README.md's next steps.
pub type CffDictOperator = i32;

pub const OP_FONT_NAME: CffDictOperator = 3110;

pub const OP_FD_SELECT: CffDictOperator = 3109;

pub const OP_FD_ARRAY: CffDictOperator = 3108;

pub const OP_UID_BASE: CffDictOperator = 3107;

pub const OP_CID_COUNT: CffDictOperator = 3106;

pub const OP_CID_FONT_TYPE: CffDictOperator = 3105;

pub const OP_CID_FONT_REVISION: CffDictOperator = 3104;

pub const OP_CID_FONT_VERSION: CffDictOperator = 3103;

pub const OP_ROS: CffDictOperator = 3102;

pub const OP_MAXSTACK: CffDictOperator = 25;

pub const OP_VSTORE: CffDictOperator = 24;

pub const OP_BASE_FONT_BLEND: CffDictOperator = 3095;

pub const OP_BLEND: CffDictOperator = 23;

pub const OP_BASE_FONT_NAME: CffDictOperator = 3094;

pub const OP_VSINDEX: CffDictOperator = 22;

pub const OP_POST_SCRIPT: CffDictOperator = 3093;

pub const OP_NOMINAL_WIDTH_X: CffDictOperator = 21;

pub const OP_SYNTHEIC_BASE: CffDictOperator = 3092;

pub const OP_DEFAULT_WIDTH_X: CffDictOperator = 20;

pub const OP_INITIAL_RANDOM_SEED: CffDictOperator = 3091;

pub const OP_SUBRS: CffDictOperator = 19;

pub const OP_EXPANSION_FACTOR: CffDictOperator = 3090;

pub const OP_PRIVATE: CffDictOperator = 18;

pub const OP_LANGUAGE_GROUP: CffDictOperator = 3089;

pub const OP_CHAR_STRINGS: CffDictOperator = 17;

pub const OP_ENCODING: CffDictOperator = 16;

pub const OP_CHARSET: CffDictOperator = 15;

pub const OP_FORCE_BOLD: CffDictOperator = 3086;

pub const OP_XUID: CffDictOperator = 14;

pub const OP_STEM_SNAP_V: CffDictOperator = 3085;

pub const OP_UNIQUE_ID: CffDictOperator = 13;

pub const OP_STEM_SNAP_H: CffDictOperator = 3084;

pub const OP_BLUE_FUZZ: CffDictOperator = 3083;

pub const OP_STD_VW: CffDictOperator = 11;

pub const OP_BLUE_SHIFT: CffDictOperator = 3082;

pub const OP_STD_HW: CffDictOperator = 10;

pub const OP_BLUE_SCALE: CffDictOperator = 3081;

pub const OP_FAMILY_OTHER_BLUES: CffDictOperator = 9;

pub const OP_STROKE_WIDTH: CffDictOperator = 3080;

pub const OP_FAMILY_BLUES: CffDictOperator = 8;

pub const OP_FONT_MATRIX: CffDictOperator = 3079;

pub const OP_OTHER_BLUES: CffDictOperator = 7;

pub const OP_CHARSTRING_TYPE: CffDictOperator = 3078;

pub const OP_BLUE_VALUES: CffDictOperator = 6;

pub const OP_PAINT_TYPE: CffDictOperator = 3077;

pub const OP_FONT_BBOX: CffDictOperator = 5;

pub const OP_UNDERLINE_THICKNESS: CffDictOperator = 3076;

pub const OP_WEIGHT: CffDictOperator = 4;

pub const OP_UNDERLINE_POSITION: CffDictOperator = 3075;

pub const OP_FAMILY_NAME: CffDictOperator = 3;

pub const OP_ITALIC_ANGLE: CffDictOperator = 3074;

pub const OP_FULL_NAME: CffDictOperator = 2;

pub const OP_IS_FIXED_PITCH: CffDictOperator = 3073;

pub const OP_NOTICE: CffDictOperator = 1;

pub const OP_COPYRIGHT: CffDictOperator = 3072;

pub const OP_VERSION: CffDictOperator = 0;

/// A Type 2 CharString operator, encoded like [`CffDictOperator`] and, as noted
/// there, sharing 38 of its numbers while meaning something else by them.
pub type CffCharstringOperator = i32;

pub const OP_FLEX1: CffCharstringOperator = 3109;

pub const OP_HFLEX1: CffCharstringOperator = 3108;

pub const OP_FLEX: CffCharstringOperator = 3107;

pub const OP_HFLEX: CffCharstringOperator = 3106;

pub const OP_HVCURVETO: CffCharstringOperator = 31;

pub const OP_ROLL: CffCharstringOperator = 3102;

pub const OP_VHCURVETO: CffCharstringOperator = 30;

pub const OP_INDEX: CffCharstringOperator = 3101;

pub const OP_CALLGSUBR: CffCharstringOperator = 29;

pub const OP_EXCH: CffCharstringOperator = 3100;

pub const OP_DUP: CffCharstringOperator = 3099;

pub const OP_HHCURVETO: CffCharstringOperator = 27;

pub const OP_SQRT: CffCharstringOperator = 3098;

pub const OP_VVCURVETO: CffCharstringOperator = 26;

pub const OP_RLINECURVE: CffCharstringOperator = 25;

pub const OP_MUL: CffCharstringOperator = 3096;

pub const OP_RCURVELINE: CffCharstringOperator = 24;

pub const OP_RANDOM: CffCharstringOperator = 3095;

pub const OP_VSTEMHM: CffCharstringOperator = 23;

pub const OP_IFELSE: CffCharstringOperator = 3094;

pub const OP_HMOVETO: CffCharstringOperator = 22;

pub const OP_GET: CffCharstringOperator = 3093;

pub const OP_RMOVETO: CffCharstringOperator = 21;

pub const OP_PUT: CffCharstringOperator = 3092;

pub const OP_CNTRMASK: CffCharstringOperator = 20;

pub const OP_HINTMASK: CffCharstringOperator = 19;

pub const OP_DROP: CffCharstringOperator = 3090;

pub const OP_HSTEMHM: CffCharstringOperator = 18;

pub const OP_CFF2BLEND: CffCharstringOperator = 16;

pub const OP_EQ: CffCharstringOperator = 3087;

pub const OP_CFF2VSIDX: CffCharstringOperator = 15;

pub const OP_NEG: CffCharstringOperator = 3086;

pub const OP_ENDCHAR: CffCharstringOperator = 14;

pub const OP_DIV: CffCharstringOperator = 3084;

pub const OP_SUB: CffCharstringOperator = 3083;

pub const OP_RETURN: CffCharstringOperator = 11;

pub const OP_ADD: CffCharstringOperator = 3082;

pub const OP_CALLSUBR: CffCharstringOperator = 10;

pub const OP_ABS: CffCharstringOperator = 3081;

pub const OP_RRCURVETO: CffCharstringOperator = 8;

pub const OP_VLINETO: CffCharstringOperator = 7;

pub const OP_HLINETO: CffCharstringOperator = 6;

pub const OP_NOT: CffCharstringOperator = 3077;

pub const OP_RLINETO: CffCharstringOperator = 5;

pub const OP_OR: CffCharstringOperator = 3076;

pub const OP_VMOVETO: CffCharstringOperator = 4;

pub const OP_AND: CffCharstringOperator = 3075;

pub const OP_VSTEM: CffCharstringOperator = 3;

pub const OP_HSTEM: CffCharstringOperator = 1;

// The Type 2 charstring spec's implementation limits. C gave them a
// `cff_Type2Limits` type of their own, but they are not a set of anything --
// they are six unrelated capacities, only ever compared against a count -- so
// each is typed as whatever it is compared with instead, which is what removes
// the casts at the fifteen sites that use them. `TYPE2_CHARSTRING_LEN` and
// `TYPE2_STEM_HINTS` are the two otfcc never checks; they stay because the
// spec's table is easier to verify whole than with holes in it.
/// Size of [`CffStack::transient`], which it declares -- so the modulus that
/// wraps an index into that array cannot drift from the array itself.
pub const TYPE2_TRANSIENT_ARRAY: usize = 32;
pub const TYPE2_MAX_SUBRS: u32 = 65300;
pub const TYPE2_CHARSTRING_LEN: u32 = 65535;
pub const TYPE2_SUBR_NESTING: u32 = 10;
pub const TYPE2_STEM_HINTS: u32 = 96;
pub const TYPE2_ARGUMENT_STACK: u32 = 48;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CffHeader {
    pub major: u8,
    pub minor: u8,
    pub hdr_size: u8,
    pub off_size: u8,
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
    pub transient: [CffValue; TYPE2_TRANSIENT_ARRAY],
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
    pub set_width:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_double) -> ()>,
    pub new_contour: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub line_to: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub curve_to: Option<
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
    pub set_hint: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            bool,
            ::core::ffi::c_double,
            ::core::ffi::c_double,
        ) -> (),
    >,
    pub set_mask: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, bool, *mut bool) -> ()>,
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
        assert_eq!(OP_NOTICE, OP_HSTEM);
        assert_eq!(OP_FD_ARRAY, OP_HFLEX1);
        assert_eq!(OP_FAMILY_NAME, OP_VSTEM);
        // Both encode a two-byte operator as `12 << 8 | b`.
        assert_eq!(OP_FD_ARRAY, 12 << 8 | 36);
        assert_eq!(OP_HFLEX1, 12 << 8 | 36);
        assert_eq!(OP_VERSION, 0);
    }
}
