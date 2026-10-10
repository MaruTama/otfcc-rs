use crate::logger::ByteStr;
use crate::support::handle::{GlyphHandle, Handle, handle_from_index, handle_from_name};
use otfcc_json::ParsedValue;
use crate::table::otl::coverage::Coverage;

use otfcc_binary::Buffer;
use otfcc_json::BuiltValue;
use otfcc_binary::FontReader;
use crate::support::primitives::{GlyphClass, GlyphId, count_u16};
/// A class definition: `glyphs[i]` is in class `classes[i]` (the two are
/// always grown and truncated together), and `maxclass` is the largest
/// class seen. `Default` is class 0 with both arrays empty.
#[derive(Clone, Debug, Default)]
pub struct ClassDef {
    pub maxclass: GlyphClass,
    pub glyphs: Vec<GlyphHandle>,
    pub classes: Vec<GlyphClass>,
}
#[derive(Copy, Clone, Debug)]
pub struct ClassDefSortRecord {
    pub gid: GlyphId,
    pub cid: GlyphClass,
}
pub(crate) fn push_class_def(cd: &mut ClassDef, h: GlyphHandle, cls: GlyphClass) {
    cd.glyphs.push(h);
    cd.classes.push(cls);
    if cls as i32 > cd.maxclass as i32 {
        cd.maxclass = cls;
    }
}
// Same trustworthiness reasoning as `coverage.rs::read_coverage` (see its
// comment) -- `data` is untrusted font bytes, but the bounds check is now
// `FontReader`'s, not a raw slice reconstructed from a pointer/length pair.
pub(crate) fn read_class_def(data: &[u8], offset: u32) -> ClassDef {
    let mut cd = ClassDef::default();
    let Ok(mut r) = FontReader::new(data).at(offset as usize) else {
        return cd;
    };
    let Ok(format) = r.u16() else {
        return cd;
    };
    if format == 1 {
        let Ok(start_gid) = r.u16() else {
            return cd;
        };
        let Ok(count) = r.u16() else {
            return cd;
        };
        if count != 0 && r.require_room(count as usize, 2).is_ok() {
            for j in 0..count {
                let cls = r.u16().unwrap();
                push_class_def(
                    &mut cd,
                    handle_from_index(start_gid.wrapping_add(j)) as GlyphHandle,
                    cls as GlyphClass,
                );
            }
        }
    } else if format == 2 {
        let Ok(range_count) = r.u16() else {
            return cd;
        };
        if r.require_room(range_count as usize, 6).is_err() {
            return cd;
        }
        // `covIndex` carries the class value here, not a coverage position,
        // so the final `ClassDef` is ordered by ascending *class value*, not
        // by gid. That is observable (it's the order `dump_class_def`
        // walks): dedup-by-gid (first occurrence wins) via `IndexMap`, then
        // a stable sort by the stored class value.
        let mut h: indexmap::IndexMap<GlyphId, GlyphClass> = indexmap::IndexMap::new();
        for _ in 0..range_count {
            let start = r.u16().unwrap();
            let end = r.u16().unwrap();
            let cls = r.u16().unwrap();
            for k in start as i32..=end as i32 {
                h.entry(k as GlyphId).or_insert(cls as GlyphClass);
            }
        }
        let mut entries: Vec<(GlyphId, GlyphClass)> = h.into_iter().collect();
        entries.sort_by_key(|&(_, cls)| cls);
        for (gid, cls) in entries {
            push_class_def(&mut cd, handle_from_index(gid) as GlyphHandle, cls);
        }
    }
    cd
}
// `ocd` is consumed here (its entries are read once, then it's dropped),
// so taking it by value lets the compiler's own drop glue replace the old
// explicit `otl_class_def_free(ocd)` call at the end.
pub(crate) fn expand_class_def(cov: &Coverage, ocd: ClassDef) -> ClassDef {
    // Insertion order, deduped by gid (first occurrence wins): `ocd`'s
    // entries first, in `ocd`'s own order; then every glyph in `cov` not
    // already present, with class 0, in `cov`'s order.
    let mut h: indexmap::IndexMap<GlyphId, GlyphClass> = indexmap::IndexMap::new();
    for j in 0..ocd.glyphs.len() {
        h.entry(ocd.glyphs[j].index).or_insert(ocd.classes[j]);
    }
    for g in cov.iter() {
        h.entry(g.index).or_insert(0 as GlyphClass);
    }
    let mut cd = ClassDef::default();
    for (gid, cid) in h.into_iter() {
        push_class_def(&mut cd, handle_from_index(gid) as GlyphHandle, cid);
    }
    // `ocd` drops here (its own Vec-drop glue), no explicit free needed.
    cd
}
pub(crate) fn dump_class_def(cd: &ClassDef) -> BuiltValue {
    let mut a = BuiltValue::new_object(cd.glyphs.len());
    for j in 0..cd.glyphs.len() {
        a.push_field_bytes_key(&cd.glyphs[j].name, BuiltValue::Int(cd.classes[j] as i64));
    }
    a.preserialize()
}
pub(crate) fn parse_class_def(cd: Option<&ParsedValue>) -> Option<ClassDef> {
    let fields = cd.and_then(ParsedValue::as_object)?;
    let mut cd = ClassDef::default();
    for (key, val) in fields {
        let number: i64 = if let Some(i) = val.as_int() {
            i
        } else if let Some(d) = val.as_double() {
            d as i64
        } else {
            0
        };
        // Classes are 16-bit. A value outside that range used to wrap (-1
        // became 65535), which made a pair-positioning class count wrap to
        // 0 and panic when the subtable was built.
        let Ok(cls) = GlyphClass::try_from(number) else {
            tracing::warn!("[OTFCC-fea] Class {} of glyph /{} is out of range. This class assignment is ignored.\n", number, ByteStr(&key[..key.len() - 1]));
            continue;
        };
        let h: GlyphHandle = handle_from_name(Some(key[..key.len() - 1].to_vec())) as GlyphHandle;
        push_class_def(&mut cd, h, cls);
    }
    Some(cd)
}
pub(crate) fn build_class_def(cd: &ClassDef) -> Buffer {
    let mut buf = Buffer::new();
    buf.write_u16be(2_u16);
    if cd.glyphs.is_empty() {
        buf.write_u16be(0_u16);
        return buf;
    }
    // `sort_by_key` (stable), same as `Coverage`'s `build_coverage_format`.
    let mut r: Vec<ClassDefSortRecord> = Vec::new();
    for j in 0..cd.glyphs.len() {
        if cd.classes[j] != 0 {
            r.push(ClassDefSortRecord {
                gid: cd.glyphs[j].index,
                cid: cd.classes[j],
            });
        }
    }
    let jj: GlyphId = count_u16(r.len());
    if jj == 0 {
        buf.write_u16be(0_u16);
        return buf;
    }
    r.sort_by_key(|rec| rec.gid);
    let mut start_gid: GlyphId = r[0].gid;
    let mut end_gid: GlyphId = start_gid;
    let mut last_class: GlyphClass = r[0].cid;
    let mut n_ranges: GlyphId = 0 as GlyphId;
    let mut last_gid: GlyphId = start_gid;
    let mut ranges = Buffer::new();
    for rec in r.iter().skip(1) {
        let current: GlyphId = rec.gid;
        if !(current as i32 <= last_gid as i32) {
            if current as i32 == end_gid as i32 + 1_i32 && rec.cid as i32 == last_class as i32 {
                end_gid = current;
            } else {
                ranges.write_u16be(start_gid as u16);
                ranges.write_u16be(end_gid as u16);
                ranges.write_u16be(last_class as u16);
                n_ranges = (n_ranges as i32 + 1_i32) as GlyphId;
                end_gid = current;
                start_gid = end_gid;
                last_class = rec.cid;
            }
            last_gid = current;
        }
    }
    ranges.write_u16be(start_gid as u16);
    ranges.write_u16be(end_gid as u16);
    ranges.write_u16be(last_class as u16);
    n_ranges = (n_ranges as i32 + 1_i32) as GlyphId;
    buf.write_u16be(n_ranges as u16);
    buf.write_buffer_owned(ranges);
    buf
}
pub(crate) fn shrink_class_def(cd: &mut ClassDef) {
    // Single `truncate` at the end lets `Vec`'s drop glue free any handle
    // this loop's own resets to `Handle::default()` didn't reach -- same
    // reasoning as `shrink_coverage`.
    let mut k: usize = 0;
    for j in 0..cd.glyphs.len() {
        if !cd.glyphs[j].name.is_empty() {
            let g = cd.glyphs[j].clone();
            let c = cd.classes[j];
            cd.glyphs[k] = g;
            cd.classes[k] = c;
            k += 1;
        } else {
            cd.glyphs[j] = Handle::default();
        }
    }
    cd.glyphs.truncate(k);
    cd.classes.truncate(k);
}

#[cfg(test)]
mod read_class_def_tests {
    use super::*;

    #[test]
    fn format1_sequential_gids_get_sequential_classes() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes()); // format
        data.extend_from_slice(&10u16.to_be_bytes()); // startGlyphID
        data.extend_from_slice(&2u16.to_be_bytes()); // glyphCount
        data.extend_from_slice(&3u16.to_be_bytes()); // classValueArray[0]
        data.extend_from_slice(&4u16.to_be_bytes()); // classValueArray[1]
        {
            let cd = read_class_def(&data, 0);
            assert_eq!(
                cd.glyphs.iter().map(|h| h.index).collect::<Vec<_>>(),
                vec![10, 11]
            );
            assert_eq!(cd.classes, vec![3, 4]);
        }
    }

    #[test]
    fn format2_ranges_sort_by_class_value_not_gid() {
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_be_bytes()); // format
        data.extend_from_slice(&2u16.to_be_bytes()); // classRangeCount
        data.extend_from_slice(&20u16.to_be_bytes()); // range0: startGlyphID
        data.extend_from_slice(&20u16.to_be_bytes()); // range0: endGlyphID
        data.extend_from_slice(&5u16.to_be_bytes()); // range0: class
        data.extend_from_slice(&10u16.to_be_bytes()); // range1: startGlyphID
        data.extend_from_slice(&10u16.to_be_bytes()); // range1: endGlyphID
        data.extend_from_slice(&1u16.to_be_bytes()); // range1: class
        {
            let cd = read_class_def(&data, 0);
            // Sorted by class value ascending: class 1 (gid 10) then class 5 (gid 20).
            assert_eq!(cd.classes, vec![1, 5]);
            assert_eq!(
                cd.glyphs.iter().map(|h| h.index).collect::<Vec<_>>(),
                vec![10, 20]
            );
        }
    }

    #[test]
    fn offset_near_u32_max_does_not_wrap_the_guard() {
        let data = [0u8; 8];
        {
            let cd = read_class_def(&data, 0xFFFF_FFF0);
            assert!(cd.glyphs.is_empty());
        }
    }

    #[test]
    fn zero_count_format1_is_empty_not_a_panic() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes()); // startGlyphID
        data.extend_from_slice(&0u16.to_be_bytes()); // glyphCount = 0
        {
            let cd = read_class_def(&data, 0);
            assert!(cd.glyphs.is_empty());
        }
    }
}
