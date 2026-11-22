#!/usr/bin/env python3
"""Shape random strings with two fonts and count the strings that differ.

Usage: shape_compare.py ORIGINAL REBUILT [COUNT [MAX_LENGTH]]

Needs `fonttools` and `uharfbuzz` (install them in a virtual environment
outside the repository). The strings are drawn from the characters ORIGINAL
maps, each from a window of 80 neighbouring code points so that a string
stays mostly within one script. Glyph IDs are compared, so REBUILT must keep
the glyph order (otfccbuild does by default).

Prints one line: how many strings gave different glyphs, how many gave the
same glyphs at different positions, and the first differing string. Random
strings find common differences; a rule that needs a particular sequence
needs a string written for it.
"""
import random
import sys

import uharfbuzz as hb
from fontTools.ttLib import TTFont


def shape(font, text):
    buf = hb.Buffer()
    buf.add_str(text)
    buf.guess_segment_properties()
    hb.shape(font, buf)
    glyphs = [(i.codepoint, i.cluster) for i in buf.glyph_infos]
    positions = [
        (p.x_advance, p.y_advance, p.x_offset, p.y_offset)
        for p in buf.glyph_positions
    ]
    return glyphs, positions


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__, file=sys.stderr)
        return 2
    original, rebuilt = sys.argv[1], sys.argv[2]
    count = int(sys.argv[3]) if len(sys.argv) > 3 else 5000
    max_length = int(sys.argv[4]) if len(sys.argv) > 4 else 8
    # Control characters are left out: they shape to nothing useful.
    code_points = sorted(
        c
        for c in TTFont(original).getBestCmap()
        if c >= 0x20 and not 0x7F <= c < 0xA0
    )
    if not code_points:
        print("ORIGINAL maps no characters", file=sys.stderr)
        return 1
    fonts = [
        hb.Font(hb.Face(open(path, "rb").read())) for path in (original, rebuilt)
    ]
    rnd = random.Random(1)
    glyph_diffs = position_diffs = 0
    first = None
    for _ in range(count):
        start = rnd.randrange(len(code_points))
        window = code_points[start : start + 80]
        text = "".join(
            chr(rnd.choice(window)) for _ in range(rnd.randint(2, max_length))
        )
        (ga, pa), (gb, pb) = (shape(font, text) for font in fonts)
        if ga != gb:
            glyph_diffs += 1
        elif pa != pb:
            position_diffs += 1
        else:
            continue
        first = first or text
    line = (
        f"{count} strings: glyphs differ in {glyph_diffs}, "
        f"positions only in {position_diffs}"
    )
    if first:
        line += ", first: " + " ".join(f"U+{ord(c):04X}" for c in first)
    print(line)
    return 0


if __name__ == "__main__":
    sys.exit(main())
