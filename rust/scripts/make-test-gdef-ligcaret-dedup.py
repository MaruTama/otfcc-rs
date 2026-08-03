#!/usr/bin/env python3
"""Forge a GDEF table whose `ligCarets` object has two entries for the
*same* glyph -- the case `consolidate_gdef`'s ligature-caret dedup pass
(rust/src/consolidate/otl/gdef.rs) handles: first occurrence wins, a
"[Consolidate] Detected caret value double-mapping" warning is logged, and
the second entry's caret list is dropped (not merged).

`NotoNastaliqUrdu-Regular.ttf` (in tests/golden/) already has real
`ligCarets` entries, so the ordinary path is covered -- but none of the
committed payloads has a *duplicate* glyph within that object, so this
dedup branch has never actually run under test. Same technique as the
other make-test-*-dedup.py scripts in this directory: a hand-written JSON
object with two members sharing the same key, which a conforming parser
(including the one used to write this script) would collapse, but otfcc's
own vendored parser does not -- see make-test-gsub-multi-dedup.py's
docstring for the full explanation.

Usage: make-test-gdef-ligcaret-dedup.py <in.json> <out.json>
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

    font.setdefault("GDEF", {})
    placeholder = "__DUPLICATE_GLYPH_LIGCARETS__"
    font["GDEF"]["ligCarets"] = placeholder

    text = json.dumps(font)
    # Two entries for the SAME glyph: only the first ([{"at": 111}]) is
    # expected to survive consolidation, the second ([{"at": 222}]) must be
    # dropped, not merged, not appended.
    raw_ligcarets_object = (
        "{"
        f'"{target_glyph}":[{{"at":111}}],'
        f'"{target_glyph}":[{{"at":222}}]'
        "}"
    )
    marker = f'"{placeholder}"'
    assert marker in text, "placeholder not found in serialized JSON"
    text = text.replace(marker, raw_ligcarets_object, 1)

    with open(dst, "w") as f:
        f.write(text)
    print(
        f"  wrote {dst} (GDEF ligCarets, duplicate glyph {target_glyph!r}: "
        f"first entry's caret at 111 must win over the second entry's at 222)"
    )


main()
