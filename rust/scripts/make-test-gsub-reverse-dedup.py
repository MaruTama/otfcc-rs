#!/usr/bin/env python3
"""Forge a GSUB "reverse chaining single substitution" lookup whose single
subtable's `match[inputIndex]` coverage has the same glyph twice -- the
case `consolidate_gsub_reverse` (rust/src/consolidate/otl/gsub_reverse.rs)
dedupes: first occurrence wins, a "[Consolidate] Double-mapping a glyph in
a reverse substitution" warning is logged, and the second occurrence is
dropped (not merged).

No committed payload has a gsub_reverse lookup at all (confirmed by
grepping every dumped payload's lookup "type" values), so this is a
coverage gap in the ordinary path too, not just the dedup branch.

`match`/`to` are plain JSON arrays (not the object shape `gsub_single` and
friends use), and a JSON array is allowed to repeat an element -- no
duplicate-key trick is needed here; a hand-written array with the same
glyph name twice survives `json.dump` and every conforming parser without
help.

The duplicate is placed at index 0 and 2 of a 4-entry `match[0]`, not at
the end: `consolidate_gsub_reverse`'s original uthash-based dedup collected
survivors into a hash table and then wrote them back over the front of the
`match[0]`/`to` arrays before truncating both to the survivor count. Only a
duplicate away from the tail exercises the case where a *surviving* entry
(not the dropped duplicate) sits at an index the truncation removes.

Usage: make-test-gsub-reverse-dedup.py <in.json> <out.json>
"""
import json
import sys


def main():
    src, dst = sys.argv[1], sys.argv[2]
    with open(src) as f:
        font = json.load(f)

    order = font["glyph_order"]
    g0, g1, g3 = order[4], order[5], order[7]
    t0, t1, t2, t3 = order[10], order[11], order[12], order[13]

    lookup_name = "lookup_test_gsub_reverse_dedup"
    font.setdefault("GSUB", {})
    font["GSUB"].setdefault("lookups", {})
    font["GSUB"].setdefault("languages", {})
    font["GSUB"].setdefault("features", {})
    font["GSUB"]["lookups"][lookup_name] = {
        "type": "gsub_reverse",
        "flags": {},
        "subtables": [
            {
                "match": [[g0, g1, g0, g3]],
                "to": [t0, t1, t2, t3],
                "inputIndex": 0,
            }
        ],
    }
    feature_name = "test_00000"
    font["GSUB"]["features"][feature_name] = [lookup_name]
    dflt = font["GSUB"]["languages"].setdefault("DFLT_DFLT", {"features": []})
    if feature_name not in dflt["features"]:
        dflt["features"].append(feature_name)
    if lookup_name not in font["GSUB"].setdefault("lookupOrder", []):
        font["GSUB"]["lookupOrder"].append(lookup_name)

    with open(dst, "w") as f:
        json.dump(font, f)
    print(
        f"  wrote {dst} (gsub_reverse lookup, one subtable, duplicate "
        f"'from' glyph {g0!r} at match[0] positions 0 and 2 (not the "
        f"tail): {g0!r}->{t0!r} must win, {g1!r}->{t1!r} and "
        f"{g3!r}->{t3!r} must both survive)"
    )


main()
