//! Feature parameters: the table a Feature's `featureParamsOffset` points
//! at. The OpenType feature tag registry defines three:
//!
//! - `ss01`-`ss20`: a version (0) and `uiLabelNameID`, the name the user
//!   sees for the stylistic set;
//! - `cv01`-`cv99`: a format (0), name IDs for the label, tooltip, sample
//!   text and parameter labels, and the characters the feature varies;
//! - `size`: the design size and the size range, in decipoints, and a
//!   subfamily identifier and name ID.
//!
//! The parameters of any other tag have no defined layout, so they are not
//! kept. In JSON a feature with parameters is written as
//! `{"lookups": [...], "params": {...}}` instead of the bare lookup list; a
//! name ID or other value of 0 means "none" and is left out.

use crate::logger::ByteStr;
use crate::table::otl::budget::OtlReadBudget;

/// Bounds the characters read from every `cv` feature's parameters across a
/// whole table (`OtlReadBudget::feature_param_characters`). One table holds
/// up to 65,535 characters, and features can all point at the same one, so
/// a few KB of table could otherwise ask for billions. A real font lists a
/// few characters per feature, for at most 99 features.
pub(crate) const MAX_TOTAL_FEATURE_PARAM_CHARACTERS_PER_TABLE: u32 = 1_000_000;
use otfcc_binary::{Buffer, FontReader};
use otfcc_json::{BuiltValue, JsonType, ParsedValue};

/// The parameters of one feature, by the kind its tag selects.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FeatureParams {
    /// `ss01`-`ss20`.
    StylisticSet { ui_label: u16 },
    /// `cv01`-`cv99`.
    CharacterVariant {
        ui_label: u16,
        tooltip: u16,
        sample_text: u16,
        first_param_label: u16,
        param_count: u16,
        /// Unicode scalar values.
        characters: Vec<u32>,
    },
    /// `size`. The sizes are in decipoints (tenths of a point).
    Size {
        design_size: u16,
        subfamily: u16,
        subfamily_name: u16,
        range_start: u16,
        range_end: u16,
    },
}

/// Which kind of parameters the feature tag `tag` (four bytes, as in a
/// feature's name) takes, if any.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    StylisticSet,
    CharacterVariant,
    Size,
}

fn kind_of(tag: &[u8]) -> Option<Kind> {
    let two_digits = |a: u8, b: u8| a.is_ascii_digit() && b.is_ascii_digit();
    match tag {
        [b's', b's', a, b, ..] if two_digits(*a, *b) => {
            let n = (a - b'0') * 10 + (b - b'0');
            (1..=20).contains(&n).then_some(Kind::StylisticSet)
        }
        [b'c', b'v', a, b, ..] if two_digits(*a, *b) && (*a, *b) != (b'0', b'0') => {
            Some(Kind::CharacterVariant)
        }
        [b's', b'i', b'z', b'e', ..] => Some(Kind::Size),
        _ => None,
    }
}

/// Reads the parameters of the feature tagged `tag` at `offset` of `data`.
/// `None` when the tag takes none or they cannot be read; a tag that takes
/// none gets a warning, as its parameters are dropped.
pub(crate) fn read_feature_params(
    data: &[u8],
    offset: usize,
    tag: &[u8],
    budget: &mut OtlReadBudget,
) -> Option<FeatureParams> {
    let Some(kind) = kind_of(tag) else {
        tracing::warn!(
            "[otl] Feature {} has parameters of no known layout; they are ignored.\n",
            ByteStr(tag)
        );
        return None;
    };
    let mut r = FontReader::new(data).at(offset).ok()?;
    match kind {
        Kind::StylisticSet => {
            r.skip(2).ok()?; // version
            Some(FeatureParams::StylisticSet {
                ui_label: r.u16().ok()?,
            })
        }
        Kind::CharacterVariant => {
            r.skip(2).ok()?; // format
            let ui_label = r.u16().ok()?;
            let tooltip = r.u16().ok()?;
            let sample_text = r.u16().ok()?;
            let param_count = r.u16().ok()?;
            let first_param_label = r.u16().ok()?;
            let count = r.u16().ok()?;
            r.require_room(usize::from(count), 3).ok()?;
            // See `MAX_TOTAL_FEATURE_PARAM_CHARACTERS_PER_TABLE`.
            if !budget.try_spend_feature_param_characters(u32::from(count)) {
                return None;
            }
            let mut characters = Vec::with_capacity(usize::from(count));
            for _ in 0..count {
                let b = r.bytes(3).ok()?;
                characters.push(u32::from_be_bytes([0, b[0], b[1], b[2]]));
            }
            Some(FeatureParams::CharacterVariant {
                ui_label,
                tooltip,
                sample_text,
                first_param_label,
                param_count,
                characters,
            })
        }
        Kind::Size => Some(FeatureParams::Size {
            design_size: r.u16().ok()?,
            subfamily: r.u16().ok()?,
            subfamily_name: r.u16().ok()?,
            range_start: r.u16().ok()?,
            range_end: r.u16().ok()?,
        }),
    }
}

impl FeatureParams {
    /// Whether `size` parameters make sense, by the test HarfBuzz applies
    /// (the specification describes the same values): a design size, and
    /// either nothing else or a range that holds the design size and a
    /// subfamily name ID in the font-specific range 256 to 32767. Other
    /// kinds of parameters always pass.
    pub(crate) fn is_plausible(&self) -> bool {
        match *self {
            FeatureParams::Size {
                design_size,
                subfamily,
                subfamily_name,
                range_start,
                range_end,
            } => {
                design_size != 0
                    && ((subfamily, subfamily_name, range_start, range_end) == (0, 0, 0, 0)
                        || ((range_start..=range_end).contains(&design_size)
                            && (256..=32767).contains(&subfamily_name)))
            }
            _ => true,
        }
    }
}

/// The binary table of `params`.
pub(crate) fn build_feature_params(params: &FeatureParams) -> Buffer {
    let mut b = Buffer::new();
    match params {
        FeatureParams::StylisticSet { ui_label } => {
            b.write_u16be(0);
            b.write_u16be(*ui_label);
        }
        FeatureParams::CharacterVariant {
            ui_label,
            tooltip,
            sample_text,
            first_param_label,
            param_count,
            characters,
        } => {
            for v in [
                0,
                *ui_label,
                *tooltip,
                *sample_text,
                *param_count,
                *first_param_label,
            ] {
                b.write_u16be(v);
            }
            b.write_u16be(characters.len() as u16);
            for &c in characters {
                b.write_u24be(c);
            }
        }
        FeatureParams::Size {
            design_size,
            subfamily,
            subfamily_name,
            range_start,
            range_end,
        } => {
            for v in [
                *design_size,
                *subfamily,
                *subfamily_name,
                *range_start,
                *range_end,
            ] {
                b.write_u16be(v);
            }
        }
    }
    b
}

/// Decipoints as points: 139 is 13.9.
fn points(decipoints: u16) -> BuiltValue {
    if decipoints.is_multiple_of(10) {
        BuiltValue::Int(i64::from(decipoints / 10))
    } else {
        BuiltValue::Double(f64::from(decipoints) / 10.0)
    }
}

/// Writes `params` as the JSON `params` object, leaving out values of 0.
pub(crate) fn dump_feature_params(params: &FeatureParams) -> BuiltValue {
    let mut o = BuiltValue::new_object(6);
    let id = |o: &mut BuiltValue, key: &[u8], v: u16| {
        if v != 0 {
            o.push_field(key, BuiltValue::Int(i64::from(v)));
        }
    };
    match params {
        FeatureParams::StylisticSet { ui_label } => id(&mut o, b"uiLabel", *ui_label),
        FeatureParams::CharacterVariant {
            ui_label,
            tooltip,
            sample_text,
            first_param_label,
            param_count,
            characters,
        } => {
            id(&mut o, b"uiLabel", *ui_label);
            id(&mut o, b"tooltip", *tooltip);
            id(&mut o, b"sampleText", *sample_text);
            if *param_count != 0 || *first_param_label != 0 {
                let mut labels = BuiltValue::new_object(2);
                labels.push_field(b"first", BuiltValue::Int(i64::from(*first_param_label)));
                labels.push_field(b"count", BuiltValue::Int(i64::from(*param_count)));
                o.push_field(b"paramLabels", labels);
            }
            if !characters.is_empty() {
                let mut chars = BuiltValue::new_array(characters.len());
                for &c in characters {
                    chars.push_item(BuiltValue::Int(i64::from(c)));
                }
                o.push_field(b"characters", chars.preserialize());
            }
        }
        FeatureParams::Size {
            design_size,
            subfamily,
            subfamily_name,
            range_start,
            range_end,
        } => {
            o.push_field(b"designSize", points(*design_size));
            id(&mut o, b"subfamily", *subfamily);
            id(&mut o, b"subfamilyName", *subfamily_name);
            if *range_start != 0 || *range_end != 0 {
                o.push_field(b"rangeStart", points(*range_start));
                o.push_field(b"rangeEnd", points(*range_end));
            }
        }
    }
    o
}

/// A 16-bit value of `json`'s `key`, 0 when absent or out of range.
fn u16_field(json: &ParsedValue, key: &[u8]) -> u16 {
    json.get_typed(key, JsonType::Integer)
        .and_then(ParsedValue::as_int)
        .and_then(|v| u16::try_from(v).ok())
        .unwrap_or(0)
}

/// A size of `json`'s `key` in points, as decipoints; 0 when absent or
/// out of range.
fn decipoints_field(json: &ParsedValue, key: &[u8]) -> u16 {
    let Some(v) = json.get(key) else {
        return 0;
    };
    let points = v
        .as_int()
        .map(|i| i as f64)
        .or_else(|| v.as_double())
        .unwrap_or(0.0);
    let d = (points * 10.0).round();
    if (0.0..=f64::from(u16::MAX)).contains(&d) {
        d as u16
    } else {
        0
    }
}

/// Reads the JSON `params` object of the feature tagged `tag`. `None` when
/// the tag takes no parameters, which is warned about.
pub(crate) fn parse_feature_params(json: &ParsedValue, tag: &[u8]) -> Option<FeatureParams> {
    let Some(kind) = kind_of(tag) else {
        tracing::warn!(
            "[OTFCC-fea] Feature {} takes no parameters; its params are ignored.\n",
            ByteStr(tag)
        );
        return None;
    };
    json.as_object()?;
    Some(match kind {
        Kind::StylisticSet => FeatureParams::StylisticSet {
            ui_label: u16_field(json, b"uiLabel"),
        },
        Kind::CharacterVariant => {
            let labels = json.get_typed(b"paramLabels", JsonType::Object);
            let characters = json
                .get_typed(b"characters", JsonType::Array)
                .and_then(ParsedValue::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(ParsedValue::as_int)
                        .filter_map(|c| u32::try_from(c).ok())
                        .filter(|&c| c <= 0xFF_FFFF)
                        .collect()
                })
                .unwrap_or_default();
            FeatureParams::CharacterVariant {
                ui_label: u16_field(json, b"uiLabel"),
                tooltip: u16_field(json, b"tooltip"),
                sample_text: u16_field(json, b"sampleText"),
                first_param_label: labels.map_or(0, |l| u16_field(l, b"first")),
                param_count: labels.map_or(0, |l| u16_field(l, b"count")),
                characters,
            }
        }
        Kind::Size => FeatureParams::Size {
            design_size: decipoints_field(json, b"designSize"),
            subfamily: u16_field(json, b"subfamily"),
            subfamily_name: u16_field(json, b"subfamilyName"),
            range_start: decipoints_field(json, b"rangeStart"),
            range_end: decipoints_field(json, b"rangeEnd"),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use otfcc_json::{
        JSON_SERIALIZE_MODE_SINGLE_LINE, JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON,
        JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA, JSON_SERIALIZE_OPT_PACK_BRACKETS,
        JsonSerializeOpts, json_serialize_ex, parse_json,
    };

    fn json(v: &BuiltValue) -> String {
        let opts = JsonSerializeOpts {
            mode: JSON_SERIALIZE_MODE_SINGLE_LINE,
            opts: JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COLON
                | JSON_SERIALIZE_OPT_NO_SPACE_AFTER_COMMA
                | JSON_SERIALIZE_OPT_PACK_BRACKETS,
            indent_size: 0,
        };
        String::from_utf8(json_serialize_ex(v, opts)).unwrap()
    }

    /// Writes, reads back, dumps and parses `params` of the tag `tag`, and
    /// checks each step gives them back; returns the JSON.
    fn round_trip(params: FeatureParams, tag: &[u8]) -> String {
        let bytes = build_feature_params(&params).data;
        assert_eq!(
            read_feature_params(&bytes, 0, tag, &mut OtlReadBudget::new()).as_ref(),
            Some(&params)
        );
        let text = json(&dump_feature_params(&params));
        let parsed = parse_json(text.as_bytes()).unwrap();
        assert_eq!(parse_feature_params(&parsed, tag), Some(params));
        text
    }

    #[test]
    fn size_parameters_must_make_sense_as_harfbuzz_requires() {
        let size = |design_size, subfamily, subfamily_name, range_start, range_end| {
            FeatureParams::Size {
                design_size,
                subfamily,
                subfamily_name,
                range_start,
                range_end,
            }
            .is_plausible()
        };
        assert!(size(100, 0, 0, 0, 0), "a design size alone");
        assert!(size(100, 3, 256, 80, 120));
        assert!(!size(0, 0, 0, 0, 0), "no design size");
        assert!(
            !size(100, 3, 256, 110, 120),
            "a range without the design size"
        );
        assert!(!size(100, 3, 255, 80, 120), "a name ID below 256");
        assert!(!size(100, 3, 32768, 80, 120), "a name ID above 32767");
        assert!(FeatureParams::StylisticSet { ui_label: 0 }.is_plausible());
    }

    #[test]
    fn a_stylistic_set_keeps_its_label() {
        let text = round_trip(FeatureParams::StylisticSet { ui_label: 276 }, b"ss01");
        assert_eq!(text, r#"{"uiLabel":276}"#);
    }

    #[test]
    fn a_character_variant_keeps_every_field() {
        let params = FeatureParams::CharacterVariant {
            ui_label: 256,
            tooltip: 257,
            sample_text: 258,
            first_param_label: 259,
            param_count: 2,
            characters: vec![0x61, 0x1F600],
        };
        let text = round_trip(params, b"cv12");
        assert_eq!(
            text,
            r#"{"uiLabel":256,"tooltip":257,"sampleText":258,"paramLabels":{"first":259,"count":2},"characters":[97,128512]}"#
        );
    }

    #[test]
    fn a_character_variant_with_only_a_label_is_short() {
        let params = FeatureParams::CharacterVariant {
            ui_label: 284,
            tooltip: 0,
            sample_text: 0,
            first_param_label: 0,
            param_count: 0,
            characters: Vec::new(),
        };
        assert_eq!(round_trip(params, b"cv01"), r#"{"uiLabel":284}"#);
    }

    #[test]
    fn size_is_written_in_points() {
        // The specification's Minion Pro example: design size 18 points,
        // subfamily 3 named by name ID 257, for 13.9 to 24 points.
        let params = FeatureParams::Size {
            design_size: 180,
            subfamily: 3,
            subfamily_name: 257,
            range_start: 139,
            range_end: 240,
        };
        let text = round_trip(params, b"size");
        assert_eq!(
            text,
            r#"{"designSize":18,"subfamily":3,"subfamilyName":257,"rangeStart":13.9,"rangeEnd":24}"#
        );
    }

    #[test]
    fn only_the_registered_tags_take_parameters() {
        for tag in [&b"ss01"[..], b"ss20", b"cv01", b"cv99", b"size"] {
            assert!(kind_of(tag).is_some(), "{}", String::from_utf8_lossy(tag));
        }
        for tag in [&b"ss00"[..], b"ss21", b"cv00", b"liga", b"kern"] {
            assert!(kind_of(tag).is_none(), "{}", String::from_utf8_lossy(tag));
        }
        assert_eq!(
            read_feature_params(&[0, 0, 1, 20], 0, b"liga", &mut OtlReadBudget::new()),
            None
        );
    }

    #[test]
    fn character_variant_characters_share_a_table_wide_budget() {
        let params = FeatureParams::CharacterVariant {
            ui_label: 256,
            tooltip: 0,
            sample_text: 0,
            first_param_label: 0,
            param_count: 0,
            characters: vec![97, 98, 99],
        };
        let bytes = build_feature_params(&params).data;
        let mut budget = OtlReadBudget {
            feature_param_characters: 5,
            ..OtlReadBudget::new()
        };
        assert!(read_feature_params(&bytes, 0, b"cv01", &mut budget).is_some());
        assert!(read_feature_params(&bytes, 0, b"cv02", &mut budget).is_none());
        assert_eq!(budget.feature_param_characters, 2);
    }
}
