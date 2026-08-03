#!/usr/bin/env python3
"""Forge a GSUB "single substitution" lookup whose single subtable has two
rules for the *same* "from" glyph -- the case `consolidate_gsub_single`
(rust/src/consolidate/otl/gsub_single.rs) dedupes: first occurrence wins, a
"[Consolidate] Double-mapping a glyph in a single substitution" warning is
logged, and the second rule is dropped (not merged).

`iosevka-r.ttf`/`NotoNastaliqUrdu-Regular.ttf`/etc. (already in
tests/golden/) have real gsub_single lookups, so the ordinary path is
covered -- but none of the committed payloads has a *duplicate* "from"
glyph within one subtable, so this dedup branch has never actually run
under test. Same duplicate-JSON-key technique as make-test-gsub-multi-dedup.py:
`gsub_single`'s subtable is a JSON object (`{"from": "to", ...}`), and a
hand-written object literal with two members sharing the same key survives
otfcc's own vendored parser (which iterates raw members by index, not by
key lookup) where a conforming parser -- including the one used to author
this script -- would collapse them.

Usage: make-test-gsub-single-dedup.py <in.json> <out.json>
"""
import json
import sys


def main():
    src, dst = sys.argv[1], sys.argv[2]
    with open(src) as f:
        font = json.load(f)

    order = font["glyph_order"]
    from_glyph, to_a, to_b = order[4], order[5], order[6]

    lookup_name = "lookup_test_gsub_single_dedup"
    font.setdefault("GSUB", {})
    font["GSUB"].setdefault("lookups", {})
    font["GSUB"].setdefault("languages", {})
    font["GSUB"].setdefault("features", {})

    placeholder = "__DUPLICATE_FROM_SUBTABLE__"
    font["GSUB"]["lookups"][lookup_name] = {
        "type": "gsub_single",
        "flags": {},
        "subtables": placeholder,
    }
    feature_name = "test_00000"
    font["GSUB"]["features"][feature_name] = [lookup_name]
    dflt = font["GSUB"]["languages"].setdefault("DFLT_DFLT", {"features": []})
    if feature_name not in dflt["features"]:
        dflt["features"].append(feature_name)
    if lookup_name not in font["GSUB"].setdefault("lookupOrder", []):
        font["GSUB"]["lookupOrder"].append(lookup_name)

    text = json.dumps(font)
    # Two rules for the SAME "from" glyph, in one subtable object: only the
    # first ("to_a") is expected to survive consolidation, "to_b" must be
    # dropped, not merged, not overwritten.
    raw_subtable_array = (
        "["
        "{"
        f'"{from_glyph}":"{to_a}",'
        f'"{from_glyph}":"{to_b}"'
        "}"
        "]"
    )
    marker = f'"{placeholder}"'
    assert marker in text, "placeholder not found in serialized JSON"
    text = text.replace(marker, raw_subtable_array, 1)

    with open(dst, "w") as f:
        f.write(text)
    print(
        f"  wrote {dst} (gsub_single lookup, one subtable, duplicate "
        f"'from' glyph {from_glyph!r}: first rule -> {to_a!r} must win "
        f"over the second rule's -> {to_b!r})"
    )


main()
