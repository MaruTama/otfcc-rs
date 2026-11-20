//! Pieces shared by the GPOS subtables: anchors and value records, read,
//! dumped, parsed and written the same way everywhere. What only the mark
//! attachment subtables share is in `gpos_mark_common`.

use otfcc_json::ParsedValue;

use otfcc_binary::FontReader;

use crate::support::primitives::Pos;
use crate::table::otl::{Anchor, PositionValue};
use otfcc_binary::Buffer;
use otfcc_binary::bk::block::{BkBlock, BkCellType, bk_int, bk_new_block, bk_push};
use otfcc_json::BuiltValue;
/// An anchor that is not there (an offset of 0 in the font, `null` in JSON).
pub fn otl_anchor_absent() -> Anchor {
    let anchor: Anchor = Anchor {
        present: false,
        x: Pos::from(0),
        y: Pos::from(0),
    };
    anchor
}
/// Reads the Anchor table at `offset`, keeping only its x and y; any format
/// is read as format 1. An anchor that does not fit in `data` is absent.
pub fn otl_read_anchor(data: &[u8], offset: u32) -> Anchor {
    let mut anchor: Anchor = Anchor {
        present: false,
        x: Pos::from(0),
        y: Pos::from(0),
    };
    let Ok(bytes) = FontReader::new(data)
        .at(offset as usize)
        .and_then(|mut r| r.bytes(6))
    else {
        return anchor;
    };
    anchor.present = true;
    anchor.x = Pos::from(i16::from_be_bytes([bytes[2], bytes[3]]));
    anchor.y = Pos::from(i16::from_be_bytes([bytes[4], bytes[5]]));
    anchor
}
/// Writes an anchor as `{ x, y }`, or `null` if it is absent.
pub fn otl_dump_anchor(a: Anchor) -> BuiltValue {
    if a.present {
        let mut v = BuiltValue::new_object(2);
        v.push_field(b"x", BuiltValue::position(a.x));
        v.push_field(b"y", BuiltValue::position(a.y));
        v
    } else {
        BuiltValue::Null
    }
}
/// Reads a JSON `{ x, y }` anchor; anything that is not an object is absent.
pub fn otl_parse_anchor(v: Option<&ParsedValue>) -> Anchor {
    let mut anchor: Anchor = Anchor {
        present: false,
        x: Pos::from(0),
        y: Pos::from(0),
    };
    let Some(v) = v.filter(|v| v.as_object().is_some()) else {
        return anchor;
    };
    anchor.present = true;
    anchor.x = v.get_num_or(b"x", 0.0) as Pos;
    anchor.y = v.get_num_or(b"y", 0.0) as Pos;
    anchor
}
/// Lays out an anchor as a format 1 Anchor table, or `None` if it is absent
/// (the caller then writes a null offset).
pub fn bk_from_anchor(a: Anchor) -> Option<BkBlock> {
    if !a.present {
        return None;
    }
    Some(bk_new_block(vec![
        bk_int(BkCellType::B16, 1),
        bk_int(BkCellType::B16, i32::from(a.x as i16) as u32),
        bk_int(BkCellType::B16, i32::from(a.y as i16) as u32),
    ]))
}
// ValueFormat bits for the four fields otfcc keeps. Device table offsets
// are not read.
/// ValueFormat bit for XPlacement (`dx`).
pub const FORMAT_DX: u16 = 1;
/// ValueFormat bit for YPlacement (`dy`).
pub const FORMAT_DY: u16 = 2;
/// ValueFormat bit for XAdvance (`dWidth`).
pub const FORMAT_DWIDTH: u16 = 4;
/// ValueFormat bit for YAdvance (`dHeight`).
pub const FORMAT_DHEIGHT: u16 = 8;
/// Size in bytes of a GPOS ValueRecord with this ValueFormat: 2 bytes per
/// field whose bit is set among the low 8 bits (`XPlacement` through
/// `YAdvDevice`); the reserved high bits never contribute a field.
pub fn position_format_length(format: u16) -> u8 {
    ((format & 0xff).count_ones() << 1) as u8
}
/// A value record that changes nothing.
pub fn position_zero() -> PositionValue {
    let v: PositionValue = PositionValue {
        dx: 0.0,
        dy: 0.0,
        d_width: 0.0,
        d_height: 0.0,
    };
    v
}
/// Reads a ValueRecord of `format` at `offset`. The device-table fields
/// are skipped; a record that does not fit in `data` reads as zero.
pub fn read_gpos_value(data: &[u8], offset: u32, format: u16) -> PositionValue {
    let mut v: PositionValue = PositionValue {
        dx: 0.0,
        dy: 0.0,
        d_width: 0.0,
        d_height: 0.0,
    };
    let len = position_format_length(format) as usize;
    let Ok(bytes) = FontReader::new(data)
        .at(offset as usize)
        .and_then(|mut r| r.bytes(len))
    else {
        return v;
    };
    let mut pos = 0;
    if format & FORMAT_DX != 0 {
        v.dx = Pos::from(i16::from_be_bytes([bytes[pos], bytes[pos + 1]]));
        pos += 2;
    }
    if format & FORMAT_DY != 0 {
        v.dy = Pos::from(i16::from_be_bytes([bytes[pos], bytes[pos + 1]]));
        pos += 2;
    }
    if format & FORMAT_DWIDTH != 0 {
        v.d_width = Pos::from(i16::from_be_bytes([bytes[pos], bytes[pos + 1]]));
        pos += 2;
    }
    if format & FORMAT_DHEIGHT != 0 {
        v.d_height = Pos::from(i16::from_be_bytes([bytes[pos], bytes[pos + 1]]));
    }
    v
}
/// Writes a value record as `{ dx, dy, dWidth, dHeight }`, leaving out the
/// fields that are zero.
pub fn gpos_dump_value(value: PositionValue) -> BuiltValue {
    let mut v = BuiltValue::new_object(4);
    if value.dx != 0. {
        v.push_field(b"dx", BuiltValue::position(value.dx));
    }
    if value.dy != 0. {
        v.push_field(b"dy", BuiltValue::position(value.dy));
    }
    if value.d_width != 0. {
        v.push_field(b"dWidth", BuiltValue::position(value.d_width));
    }
    if value.d_height != 0. {
        v.push_field(b"dHeight", BuiltValue::position(value.d_height));
    }
    v.preserialize()
}
/// Reads a JSON value record; missing fields, or a non-object, are zero.
pub fn gpos_parse_value(pos: Option<&ParsedValue>) -> PositionValue {
    let mut v: PositionValue = PositionValue {
        dx: 0.0,
        dy: 0.0,
        d_width: 0.0,
        d_height: 0.0,
    };
    let Some(pos) = pos.filter(|p| p.as_object().is_some()) else {
        return v;
    };
    v.dx = pos.get_num(b"dx") as Pos;
    v.dy = pos.get_num(b"dy") as Pos;
    v.d_width = pos.get_num(b"dWidth") as Pos;
    v.d_height = pos.get_num(b"dHeight") as Pos;
    v
}
/// The smallest ValueFormat that holds `v`: one bit per non-zero field.
pub fn required_position_format(v: PositionValue) -> u8 {
    let mut format: u16 = 0;
    if v.dx != 0. {
        format |= FORMAT_DX;
    }
    if v.dy != 0. {
        format |= FORMAT_DY;
    }
    if v.d_width != 0. {
        format |= FORMAT_DWIDTH;
    }
    if v.d_height != 0. {
        format |= FORMAT_DHEIGHT;
    }
    format as u8
}
/// Writes the fields of `v` that `format` selects, in ValueRecord order.
pub fn write_gpos_value(buf: &mut Buffer, v: PositionValue, format: u16) {
    if format & FORMAT_DX != 0 {
        buf.write_i16be(v.dx as i16);
    }
    if format & FORMAT_DY != 0 {
        buf.write_i16be(v.dy as i16);
    }
    if format & FORMAT_DWIDTH != 0 {
        buf.write_i16be(v.d_width as i16);
    }
    if format & FORMAT_DHEIGHT != 0 {
        buf.write_i16be(v.d_height as i16);
    }
}
/// Lays out the fields of `v` that `format` selects as a block of 16-bit
/// cells, in ValueRecord order.
pub fn bk_gpos_value(v: PositionValue, format: u16) -> BkBlock {
    let mut b: BkBlock = bk_new_block(Vec::new());
    if format & FORMAT_DX != 0 {
        bk_push(
            &mut b,
            vec![bk_int(BkCellType::B16, i32::from(v.dx as i16) as u32)],
        );
    }
    if format & FORMAT_DY != 0 {
        bk_push(
            &mut b,
            vec![bk_int(BkCellType::B16, i32::from(v.dy as i16) as u32)],
        );
    }
    if format & FORMAT_DWIDTH != 0 {
        bk_push(
            &mut b,
            vec![bk_int(BkCellType::B16, i32::from(v.d_width as i16) as u32)],
        );
    }
    if format & FORMAT_DHEIGHT != 0 {
        bk_push(
            &mut b,
            vec![bk_int(BkCellType::B16, i32::from(v.d_height as i16) as u32)],
        );
    }
    b
}

#[cfg(test)]
mod read_anchor_and_value_tests {
    use super::*;

    #[test]
    fn otl_read_anchor_well_formed_reads_x_and_y() {
        let mut data = vec![0u8; 6];
        data[2..4].copy_from_slice(&100i16.to_be_bytes());
        data[4..6].copy_from_slice(&(-50i16).to_be_bytes());
        let anchor = otl_read_anchor(&data, 0);
        assert!(anchor.present);
        assert_eq!(anchor.x, 100.0);
        assert_eq!(anchor.y, -50.0);
    }

    #[test]
    fn otl_read_anchor_truncated_is_absent_not_oob() {
        let data = vec![0u8; 4];
        let anchor = otl_read_anchor(&data, 0);
        assert!(!anchor.present);
    }

    #[test]
    fn read_gpos_value_reads_only_the_fields_the_format_selects() {
        let format = FORMAT_DX | FORMAT_DY;
        let mut data = vec![0u8; 4];
        data[0..2].copy_from_slice(&10i16.to_be_bytes());
        data[2..4].copy_from_slice(&20i16.to_be_bytes());
        let v = read_gpos_value(&data, 0, format);
        assert_eq!((v.dx, v.dy, v.d_width, v.d_height), (10.0, 20.0, 0.0, 0.0));
    }

    #[test]
    fn read_gpos_value_truncated_is_zero_not_oob() {
        let format = FORMAT_DX | FORMAT_DY;
        let data = vec![0u8; 2]; // needs 4 bytes for dx+dy, only 2 present
        let v = read_gpos_value(&data, 0, format);
        assert_eq!((v.dx, v.dy), (0.0, 0.0));
    }

    #[test]
    fn position_format_length_is_two_bytes_per_low_format_bit() {
        // The same per-bit count the removed 256-entry `BITS_IN` table
        // encoded, checked across every possible format word.
        for format in 0..=u16::MAX {
            let fields = (0..8).filter(|bit| format & (1 << bit) != 0).count() as u8;
            assert_eq!(
                position_format_length(format),
                fields * 2,
                "format {format:#06x}"
            );
        }
    }
}
