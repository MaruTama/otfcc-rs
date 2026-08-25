use crate::libcff::{CffCharstringOperator, CffDictOperator};

/// The DICT operator names. Dead in this crate (nothing calls it) -- it is
/// libcff's own API, kept because it is the DICT half of the pair whose
/// halves this newtype exists to keep apart. Matching on `op.0` rather than
/// rewriting 56 arms into `CffDictOperator(n) =>` keeps the table readable.
pub unsafe fn op_cff_name(op: CffDictOperator) -> *mut ::core::ffi::c_char {
    match op.0 {
        0 => {
            return b"Version\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        1 => {
            return b"Notice\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        2 => {
            return b"FullName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3 => {
            return b"FamilyName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        4 => {
            return b"Weight\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        5 => {
            return b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        6 => {
            return b"BlueValues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        7 => {
            return b"OtherBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        8 => {
            return b"FamilyBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        9 => {
            return b"FamilyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        10 => {
            return b"StdHW\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        11 => {
            return b"StdVW\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        13 => {
            return b"UniqueID\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        14 => {
            return b"XUID\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        15 => {
            return b"charset\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        16 => {
            return b"Encoding\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        17 => {
            return b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        18 => {
            return b"Private\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        19 => {
            return b"Subrs\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        20 => {
            return b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        21 => {
            return b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3072 => {
            return b"Copyright\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3073 => {
            return b"isFixedPitch\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3074 => {
            return b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3075 => {
            return b"UnderlinePosition\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3076 => {
            return b"UnderlineThickness\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3077 => {
            return b"PaintType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3078 => {
            return b"CharstringType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3079 => {
            return b"FontMatrix\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3080 => {
            return b"StrokeWidth\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3081 => {
            return b"BlueScale\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3082 => {
            return b"BlueShift\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3083 => {
            return b"BlueFuzz\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3084 => {
            return b"StemSnapH\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3085 => {
            return b"StemSnapV\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3086 => {
            return b"ForceBold\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3089 => {
            return b"LanguageGroup\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3090 => {
            return b"ExpansionFactor\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3091 => {
            return b"initialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3092 => {
            return b"SyntheicBase\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3093 => {
            return b"PostScript\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3094 => {
            return b"BaseFontName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3095 => {
            return b"BaseFontBlend\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3102 => {
            return b"ROS\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3103 => {
            return b"CIDFontVersion\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3104 => {
            return b"CIDFontReversion\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3105 => {
            return b"CIDFontType\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3106 => {
            return b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3107 => {
            return b"UIDBase\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3108 => {
            return b"FDArray\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3109 => {
            return b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3110 => {
            return b"FontName\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        _ => {
            return b"Unkown\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    };
}
/// The CharString operator names -- the other half; see [`op_cff_name`].
pub unsafe fn op_cs2_name(op: CffCharstringOperator) -> *mut ::core::ffi::c_char {
    match op.0 {
        1 => {
            return b"hstem\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3 => {
            return b"vstem\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        4 => {
            return b"vmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        5 => {
            return b"rlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        6 => {
            return b"hlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        7 => {
            return b"vlineto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        8 => {
            return b"rrcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        10 => {
            return b"callsubr\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        11 => {
            return b"return\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        14 => {
            return b"endchar\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        18 => {
            return b"hstemhm\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        19 => {
            return b"hintmask\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        20 => {
            return b"cntrmask\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        21 => {
            return b"rmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        22 => {
            return b"hmoveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        23 => {
            return b"vstemhm\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        24 => {
            return b"rcurveline\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        25 => {
            return b"rlinecurve\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        26 => {
            return b"vvcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        27 => {
            return b"hhcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        29 => {
            return b"callgsubr\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        30 => {
            return b"vhcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        31 => {
            return b"hvcurveto\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3075 => {
            return b"and\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3076 => {
            return b"or\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3077 => {
            return b"not\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3081 => {
            return b"abs\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3082 => {
            return b"add\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3083 => {
            return b"sub\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3084 => {
            return b"div\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3086 => {
            return b"neg\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3087 => {
            return b"eq\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3090 => {
            return b"drop\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3092 => {
            return b"put\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3093 => {
            return b"get\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3094 => {
            return b"ifelse\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3095 => {
            return b"random\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3096 => {
            return b"mul\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3098 => {
            return b"sqrt\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3099 => {
            return b"dup\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        3100 => {
            return b"exch\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3101 => {
            return b"index\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3102 => {
            return b"roll\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3106 => {
            return b"hflex\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3107 => {
            return b"fles\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3108 => {
            return b"hflex1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        3109 => {
            return b"flex1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        _ => {
            return b"Unknown\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
    };
}
pub unsafe fn cff_get_standard_arity(op: CffCharstringOperator) -> u8 {
    match op.0 {
        5 | 21 => return 2 as u8,
        6 | 7 => return 1 as u8,
        27 | 26 | 31 | 30 => return 4 as u8,
        8 => return 6 as u8,
        19 | 20 => return 0 as u8,
        _ => return 2 as u8,
    };
}
