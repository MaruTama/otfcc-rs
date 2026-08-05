#!/usr/bin/env python3
"""Forge gpos_mark_to_base and gpos_mark_to_ligature lookups whose "marks"
and "bases" objects each have two members for the *same* target glyph --
the case `consolidate_mark_array` / `consolidate_base_array` /
`consolidate_lig_array` (rust/src/consolidate/otl/mark.rs) dedupe by
resolved glyph id: first occurrence wins, a
"[Consolidate] Ignored invalid or double-mapping mark definition" /
"[Consolidate] Ignored anchor double-definition" warning is logged, and
the second occurrence is dropped (not merged).

`NotoNastaliqUrdu-Regular.ttf` (in tests/golden/) already has real
gpos_mark_to_base (11 lookups) and gpos_mark_to_ligature (1 lookup)
subtables, so the ordinary path is covered -- but none of the committed
payloads has a *duplicate* target glyph within one subtable's "marks" or
"bases" object, so this consolidation-time dedup (the whole reason
BaseHash/MarkHash/LigHash's uthash tables existed) has never actually run
under test. Same technique as make-test-gpos-single-dedup.py: a
hand-written JSON object with two members sharing the same key, which a
conforming parser (including the one used to write this script) would
collapse, but otfcc's own vendored parser does not -- it produces two
separate MarkRecord/BaseRecord/LigatureBaseRecord entries during parsing,
which then collide by resolved glyph id during consolidation -- see
make-test-gsub-multi-dedup.py's docstring for the full explanation of why
duplicate JSON keys survive the parse.

Note this is a *different* dedup step from the class-name-based one in
otl_parse_mark_array (ClassNameHash, rust/README.md): that one assigns
`mark_class` ids by distinct class name at parse time and runs regardless
of duplicate glyphs; this script's target is the later, gid-keyed
consolidation step that drops whole duplicate mark/base/ligature-base
records.

Usage: make-test-mark-consolidate-dedup.py <in.json> <out.json>
"""
import json
import sys


def main():
    src, dst = sys.argv[1], sys.argv[2]
    with open(src) as f:
        font = json.load(f)

    order = font["glyph_order"]
    # Ordinary glyphs already in the font, picked well inside the order so
    # they exist in every payload this might run against.
    mark_glyph = order[10]
    base_glyph = order[11]
    lig_glyph = order[12]

    font.setdefault("GPOS", {})
    font["GPOS"].setdefault("lookups", {})
    font["GPOS"].setdefault("languages", {})
    font["GPOS"].setdefault("features", {})

    mark_base_placeholder = "__DUPLICATE_MARK_BASE_SUBTABLE__"
    mark_base_lookup = "lookup_test_mark_to_base_dedup"
    font["GPOS"]["lookups"][mark_base_lookup] = {
        "type": "gpos_mark_to_base",
        "flags": {},
        "subtables": mark_base_placeholder,
    }

    mark_lig_placeholder = "__DUPLICATE_MARK_LIGATURE_SUBTABLE__"
    mark_lig_lookup = "lookup_test_mark_to_ligature_dedup"
    font["GPOS"]["lookups"][mark_lig_lookup] = {
        "type": "gpos_mark_to_ligature",
        "flags": {},
        "subtables": mark_lig_placeholder,
    }

    feature_name = "test_00000"
    font["GPOS"]["features"][feature_name] = [mark_base_lookup, mark_lig_lookup]
    dflt = font["GPOS"]["languages"].setdefault("DFLT_DFLT", {"features": []})
    if feature_name not in dflt["features"]:
        dflt["features"].append(feature_name)
    order_list = font["GPOS"].setdefault("lookupOrder", [])
    for name in (mark_base_lookup, mark_lig_lookup):
        if name not in order_list:
            order_list.append(name)

    text = json.dumps(font)

    # gpos_mark_to_base: two "marks" entries and two "bases" entries, each
    # pair sharing a glyph name. Only the first mark (class "top", anchor
    # 111/111) and the first base (anchor0 111/111) may survive
    # consolidation; the seconds (class "bottom" 222/222, anchor0 222/222)
    # must be dropped, not merged.
    mark_base_subtable = (
        "["
        "{"
        f'"marks":{{'
        f'"{mark_glyph}":{{"class":"top","x":111,"y":111}},'
        f'"{mark_glyph}":{{"class":"bottom","x":222,"y":222}}'
        "},"
        f'"bases":{{'
        f'"{base_glyph}":{{"top":{{"x":111,"y":111}},"bottom":{{"x":111,"y":111}}}},'
        f'"{base_glyph}":{{"top":{{"x":222,"y":222}},"bottom":{{"x":222,"y":222}}}}'
        "}"
        "}"
        "]"
    )
    marker = f'"{mark_base_placeholder}"'
    assert marker in text, "mark_to_base placeholder not found in serialized JSON"
    text = text.replace(marker, mark_base_subtable, 1)

    # gpos_mark_to_ligature: one ordinary mark (so `marks` is non-empty and
    # registers class "top"), and two "bases" (ligature) entries sharing a
    # glyph name, each a one-component ligature. Only the first (anchor
    # 111/111) may survive; the second (222/222) must be dropped.
    mark_lig_subtable = (
        "["
        "{"
        f'"marks":{{"{mark_glyph}":{{"class":"top","x":0,"y":0}}}},'
        f'"bases":{{'
        f'"{lig_glyph}":[{{"top":{{"x":111,"y":111}}}}],'
        f'"{lig_glyph}":[{{"top":{{"x":222,"y":222}}}}]'
        "}"
        "}"
        "]"
    )
    marker = f'"{mark_lig_placeholder}"'
    assert marker in text, "mark_to_ligature placeholder not found in serialized JSON"
    text = text.replace(marker, mark_lig_subtable, 1)

    with open(dst, "w") as f:
        f.write(text)
    print(
        f"  wrote {dst} (gpos_mark_to_base + gpos_mark_to_ligature lookups, "
        f"duplicate target glyphs {mark_glyph!r}/{base_glyph!r}/{lig_glyph!r}: "
        f"first occurrence of each must win, second must be dropped)"
    )


main()
