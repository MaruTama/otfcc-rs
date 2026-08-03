#!/usr/bin/env python3
"""Forge a GPOS "single positioning" lookup whose single subtable has two
rules for the *same* target glyph -- the case `consolidate_gpos_single`
(rust/src/consolidate/otl/gpos_single.rs) dedupes: first occurrence wins, a
"[Consolidate] Detected glyph double-mapping" warning is logged, and the
second rule is dropped (not merged).

`NotoNastaliqUrdu-Regular.ttf` (in tests/golden/) already has real
gpos_single lookups, so the ordinary path is covered -- but none of the
committed payloads has a *duplicate* target glyph within one subtable, so
this dedup branch (the whole reason `consolidate_gpos_single`'s uthash
table existed) has never actually run under test. Same technique as
make-test-gsub-multi-dedup.py: a hand-written JSON object with two members
sharing the same key, which a conforming parser (including the one used to
write this script) would collapse, but otfcc's own vendored parser does
not -- see that script's docstring for the full explanation.

Usage: make-test-gpos-single-dedup.py <in.json> <out.json>
"""
import json
import sys


def main():
    src, dst = sys.argv[1], sys.argv[2]
    with open(src) as f:
        font = json.load(f)

    order = font["glyph_order"]
    # An ordinary glyph already in the font, picked well inside the order so
    # it exists in every payload this might run against.
    target_glyph = order[4]

    lookup_name = "lookup_test_gpos_single_dedup"
    font.setdefault("GPOS", {})
    font["GPOS"].setdefault("lookups", {})
    font["GPOS"].setdefault("languages", {})
    font["GPOS"].setdefault("features", {})

    placeholder = "__DUPLICATE_TARGET_SUBTABLE__"
    font["GPOS"]["lookups"][lookup_name] = {
        "type": "gpos_single",
        "flags": {},
        "subtables": placeholder,
    }
    feature_name = "test_00000"
    font["GPOS"]["features"][feature_name] = [lookup_name]
    dflt = font["GPOS"]["languages"].setdefault("DFLT_DFLT", {"features": []})
    if feature_name not in dflt["features"]:
        dflt["features"].append(feature_name)
    if lookup_name not in font["GPOS"].setdefault("lookupOrder", []):
        font["GPOS"]["lookupOrder"].append(lookup_name)

    text = json.dumps(font)
    # Two rules for the SAME target glyph, in one subtable object: only the
    # first (dWidth 111) is expected to survive consolidation, the second
    # (dWidth 222) must be dropped, not merged, not averaged.
    raw_subtable_array = (
        "["
        "{"
        f'"{target_glyph}":{{"dWidth":111}},'
        f'"{target_glyph}":{{"dWidth":222}}'
        "}"
        "]"
    )
    marker = f'"{placeholder}"'
    assert marker in text, "placeholder not found in serialized JSON"
    text = text.replace(marker, raw_subtable_array, 1)

    with open(dst, "w") as f:
        f.write(text)
    print(
        f"  wrote {dst} (gpos_single lookup, one subtable, duplicate "
        f"target glyph {target_glyph!r}: first rule's dWidth 111 must win "
        f"over the second rule's dWidth 222)"
    )


main()
