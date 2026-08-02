#!/usr/bin/env python3
"""Forge a GSUB "multiple substitution" lookup whose single subtable has two
rules for the *same* "from" glyph -- the case
`consolidate_gsub_multi`/`consolidate_gsub_alternative`
(rust/src/consolidate/otl/gsub_multi.rs) dedupes: first occurrence wins, the
rest are silently dropped (their `to` coverage discarded along with it).

None of the committed payloads has a gsub_multiple or gsub_alternate lookup
at all (confirmed by grepping every dumped payload's lookup "type" values),
let alone one exercising the duplicate-"from" path -- so this gap existed
before the uthash-hash-table -> BTreeMap rewrite of that function, and
nothing would have caught a subtly wrong dedup/sort order without it.

A genuine JSON object cannot have two members with the same key -- Python's
`json` module (and every conforming parser) collapses them. otfcc's own
vendored parser deliberately does NOT: `table/otl/subtables/gsub_multi.rs`'s
reader iterates every raw member of the subtable object by index
(`0..object.length`), not by a key lookup, so a hand-written duplicate key
survives as two separate `GsubMultiEntry`s -- precisely mirroring how two
distinct rules for the same input glyph would arrive from a binary font's
rule array. This script builds the rest of the font with `json.dump` as
usual and splices in that one hand-written object literal by string
replacement, since Python's own dict/json machinery cannot produce it.

Usage: make-test-gsub-multi-dedup.py <in.json> <out.json>
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
    from_glyph, to_a, to_b = order[4], order[5], order[6]

    lookup_name = "lookup_test_gsub_multi_dedup"
    font.setdefault("GSUB", {})
    font["GSUB"].setdefault("lookups", {})
    font["GSUB"].setdefault("languages", {})
    font["GSUB"].setdefault("features", {})

    placeholder = "__DUPLICATE_FROM_SUBTABLE__"
    font["GSUB"]["lookups"][lookup_name] = {
        "type": "gsub_multiple",
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
    # dropped, not merged, not appended.
    raw_subtable_array = (
        "["
        "{"
        f'"{from_glyph}":["{to_a}"],'
        f'"{from_glyph}":["{to_b}"]'
        "}"
        "]"
    )
    marker = f'"{placeholder}"'
    assert marker in text, "placeholder not found in serialized JSON"
    text = text.replace(marker, raw_subtable_array, 1)

    with open(dst, "w") as f:
        f.write(text)
    print(
        f"  wrote {dst} (gsub_multiple lookup, one subtable, duplicate "
        f"'from' glyph {from_glyph!r}: first rule -> {to_a!r} must win "
        f"over the second rule's -> {to_b!r})"
    )


main()
