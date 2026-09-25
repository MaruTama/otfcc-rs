use crate::libcff::{CffCharstringOperator, CffDictOperator};

/// The DICT operator names. Dead in this crate (nothing calls it) -- it is
/// libcff's own API, kept because it is the DICT half of the pair whose
/// halves this newtype exists to keep apart. Matching on `op.0` rather than
/// rewriting 56 arms into `CffDictOperator(n) =>` keeps the table readable.
pub fn op_cff_name(op: CffDictOperator) -> *mut ::core::ffi::c_char {
    match op.0 {
        0 => {
            b"Version\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        1 => {
            b"Notice\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        2 => {
            b"FullName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3 => {
            b"FamilyName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        4 => {
            b"Weight\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        5 => {
            b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        6 => {
            b"BlueValues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        7 => {
            b"OtherBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        8 => {
            b"FamilyBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        9 => {
            b"FamilyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        10 => {
            b"StdHW\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        11 => {
            b"StdVW\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        13 => {
            b"UniqueID\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        14 => {
            b"XUID\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        15 => {
            b"charset\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        16 => {
            b"Encoding\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        17 => {
            b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        18 => {
            b"Private\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        19 => {
            b"Subrs\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        20 => {
            b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        21 => {
            b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3072 => {
            b"Copyright\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3073 => {
            b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3074 => {
            b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3075 => {
            b"UnderlinePosition\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3076 => {
            b"UnderlineThickness\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3077 => {
            b"PaintType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3078 => {
            b"CharstringType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3079 => {
            b"FontMatrix\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3080 => {
            b"StrokeWidth\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3081 => {
            b"BlueScale\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3082 => {
            b"BlueShift\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3083 => {
            b"BlueFuzz\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3084 => {
            b"StemSnapH\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3085 => {
            b"StemSnapV\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3086 => {
            b"ForceBold\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3089 => {
            b"LanguageGroup\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3090 => {
            b"ExpansionFactor\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3091 => {
            b"initialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3092 => {
            b"SyntheicBase\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3093 => {
            b"PostScript\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3094 => {
            b"BaseFontName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3095 => {
            b"BaseFontBlend\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3102 => {
            b"ROS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3103 => {
            b"CIDFontVersion\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3104 => {
            b"CIDFontReversion\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3105 => {
            b"CIDFontType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3106 => {
            b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3107 => {
            b"UIDBase\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3108 => {
            b"FDArray\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3109 => {
            b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3110 => {
            b"FontName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        _ => {
            b"Unkown\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
    }
}
/// The CharString operator names -- the other half; see [`op_cff_name`].
pub fn op_cs2_name(op: CffCharstringOperator) -> *mut ::core::ffi::c_char {
    match op.0 {
        1 => {
            b"hstem\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3 => {
            b"vstem\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        4 => {
            b"vmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        5 => {
            b"rlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        6 => {
            b"hlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        7 => {
            b"vlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        8 => {
            b"rrcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        10 => {
            b"callsubr\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        11 => {
            b"return\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        14 => {
            b"endchar\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        18 => {
            b"hstemhm\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        19 => {
            b"hintmask\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        20 => {
            b"cntrmask\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        21 => {
            b"rmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        22 => {
            b"hmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        23 => {
            b"vstemhm\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        24 => {
            b"rcurveline\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        25 => {
            b"rlinecurve\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        26 => {
            b"vvcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        27 => {
            b"hhcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        29 => {
            b"callgsubr\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        30 => {
            b"vhcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        31 => {
            b"hvcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3075 => {
            b"and\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3076 => {
            b"or\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3077 => {
            b"not\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3081 => {
            b"abs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3082 => {
            b"add\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3083 => {
            b"sub\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3084 => {
            b"div\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3086 => {
            b"neg\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3087 => {
            b"eq\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3090 => {
            b"drop\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3092 => {
            b"put\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3093 => {
            b"get\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3094 => {
            b"ifelse\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3095 => {
            b"random\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3096 => {
            b"mul\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3098 => {
            b"sqrt\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3099 => {
            b"dup\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        }
        3100 => {
            b"exch\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3101 => {
            b"index\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3102 => {
            b"roll\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3106 => {
            b"hflex\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3107 => {
            b"fles\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3108 => {
            b"hflex1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        3109 => {
            b"flex1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
        _ => {
            b"Unknown\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
        }
    }
}
pub fn cff_get_standard_arity(op: CffCharstringOperator) -> u8 {
    match op.0 {
        5 | 21 => 2_u8,
        6 | 7 => 1_u8,
        27 | 26 | 31 | 30 => 4_u8,
        8 => 6_u8,
        19 | 20 => 0_u8,
        _ => 2_u8,
    }
}
