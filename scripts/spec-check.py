#!/usr/bin/env python3
"""Check that a font survives otfccdump and otfccbuild, by other tools' rules.

Usage: spec-check.py [--bin DIR] [--work DIR] [--only CHECK] [--known FILE] FONT...

Each FONT is dumped and built again, and the rebuilt font is checked with
tools that do not share otfcc's code:

  structure/ots       the OpenType Sanitizer accepts it (skipped when the
                      `ots` module is not installed)
  structure/parse     fontTools reads every table
  roundtrip/cmap      the same code points map to the same glyph ids
  roundtrip/outline   every glyph draws the same outline
  roundtrip/advance   every glyph has the same advance width
  roundtrip/origin    a TrueType glyph's left phantom point (header xMin
                      minus left side bearing) is where it was
  roundtrip/shaping   HarfBuzz shapes random strings to the same glyphs at
                      the same positions (skipped without `uharfbuzz`)
  derived/bbox        a TrueType glyph's stored box is the box of its points

`tests/golden` pins what otfcc wrote the day a checksum was taken, bugs
included. These checks ask a different question: is the output right? They
compare with the source font, not with a recomputation, wherever the source
decides the answer: a font whose own boxes are wrong keeps its origins, and
that is correct.

Needs `fonttools`; `uharfbuzz` and `opentype-sanitizer` add their checks.
Prints one line per finding and exits 1 when there is one that `--known`
does not list, or when a listed one is no longer found.
"""
import argparse
import copy
import os
import random
import subprocess
import sys
import tempfile

from fontTools.pens.recordingPen import DecomposingRecordingPen
from fontTools.ttLib import TTFont

try:
    import uharfbuzz as hb
except ImportError:
    hb = None
try:
    import ots
except ImportError:
    ots = None

# A font of 65,535 glyphs is checked on a spread of this many.
MAX_GLYPHS = 4000
SHAPED_STRINGS = 2000


def rebuild(source, bin_dir, work):
    """Dumps and builds `source`; the path of the rebuilt font, or None."""
    name = os.path.basename(source)
    dumped = os.path.join(work, name + ".json")
    rebuilt = os.path.join(work, name + ".rebuilt")
    for tool, args in (
        ("otfccdump", [source, "-o", dumped]),
        ("otfccbuild", [dumped, "-o", rebuilt]),
    ):
        done = subprocess.run(
            [os.path.join(bin_dir, tool), *args],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        if done.returncode != 0:
            return None
    os.remove(dumped)
    return rebuilt


def spread(count):
    """Up to MAX_GLYPHS glyph ids, evenly spread over `count`."""
    step = max(1, count // MAX_GLYPHS)
    return range(0, count, step)


def outline(font, glyph_set, name):
    """What the glyph draws. A TrueType glyph is its flattened points, as
    stored (the pen would shift them by the left side bearing, which
    `roundtrip/origin` checks apart); a CFF glyph is its path, without a
    closing line back to the start, which a charstring may leave out."""
    if "glyf" in font:
        glyf = font["glyf"]
        coordinates, end_points, flags = glyf[name].getCoordinates(glyf)
        return list(coordinates), list(end_points), [f & 1 for f in flags]
    pen = DecomposingRecordingPen(glyph_set)
    glyph_set[name].draw(pen)
    path, start = [], None
    for operator, points in pen.value:
        if operator == "moveTo":
            start = points[0]
        if operator in ("closePath", "endPath") and path and path[-1] == ("lineTo", (start,)):
            path.pop()
        path.append((operator, points))
    return path


def check_ots(_source, rebuilt, _paths):
    if ots is None:
        return None
    with tempfile.NamedTemporaryFile() as out:
        done = ots.sanitize(rebuilt["path"], out.name, capture_output=True)
    lines = done.stderr.decode(errors="replace").strip().splitlines()
    if done.returncode != 0 or lines:
        return [lines[-1] if lines else f"exit code {done.returncode}"]
    return []


def check_parse(_source, rebuilt, _paths):
    found = []
    for tag in rebuilt["font"].keys():
        if tag == "GlyphOrder":
            continue
        try:
            rebuilt["font"][tag]  # decompiles the table
        except Exception as error:  # fontTools raises many kinds
            found.append(f"{tag}: {type(error).__name__}: {error}")
    return found


def check_cmap(source, rebuilt, _paths):
    def by_id(font):
        order = {name: gid for gid, name in enumerate(font.getGlyphOrder())}
        cmap = font.getBestCmap() or {}
        # A code mapped to glyph 0 is not mapped.
        return {c: order[g] for c, g in cmap.items() if order[g] != 0}

    before, after = by_id(source["font"]), by_id(rebuilt["font"])
    changed = sorted(c for c in before.keys() | after.keys() if before.get(c) != after.get(c))
    if changed:
        return [f"{len(changed)} code points differ, first U+{changed[0]:04X}"]
    return []


def check_outline(source, rebuilt, _paths):
    names_a, names_b = source["font"].getGlyphOrder(), rebuilt["font"].getGlyphOrder()
    if len(names_a) != len(names_b):
        return [f"{len(names_a)} glyphs became {len(names_b)}"]
    set_a, set_b = source["font"].getGlyphSet(), rebuilt["font"].getGlyphSet()
    changed = [
        g
        for g in spread(len(names_a))
        if outline(source["font"], set_a, names_a[g]) != outline(rebuilt["font"], set_b, names_b[g])
    ]
    if changed:
        return [f"{len(changed)} glyphs differ, first glyph {changed[0]} ({names_a[changed[0]]})"]
    return []


def check_advance(source, rebuilt, _paths):
    names_a, names_b = source["font"].getGlyphOrder(), rebuilt["font"].getGlyphOrder()
    hmtx_a, hmtx_b = source["font"]["hmtx"], rebuilt["font"]["hmtx"]
    changed = [
        g
        for g in range(min(len(names_a), len(names_b)))
        if hmtx_a[names_a[g]][0] != hmtx_b[names_b[g]][0]
    ]
    if changed:
        return [f"{len(changed)} glyphs differ, first glyph {changed[0]} ({names_a[changed[0]]})"]
    return []


def check_origin(source, rebuilt, _paths):
    if "glyf" not in source["font"] or "glyf" not in rebuilt["font"]:
        return None

    def origins(font):
        glyf, hmtx = font["glyf"], font["hmtx"]
        result = []
        for name in font.getGlyphOrder():
            glyph = glyf[name]
            has_box = glyph.numberOfContours != 0 and hasattr(glyph, "xMin")
            result.append(glyph.xMin - hmtx[name][1] if has_box else None)
        return result

    before, after = origins(source["font"]), origins(rebuilt["font"])
    names = source["font"].getGlyphOrder()
    changed = [g for g in range(min(len(before), len(after))) if before[g] != after[g]]
    if changed:
        g = changed[0]
        return [f"{len(changed)} glyphs differ, first glyph {g} ({names[g]}): {before[g]} became {after[g]}"]
    return []


def check_shaping(source, rebuilt, paths):
    if hb is None:
        return None
    code_points = sorted(
        c for c in (source["font"].getBestCmap() or {}) if c >= 0x20 and not 0x7F <= c < 0xA0
    )
    if not code_points:
        return None
    fonts = [hb.Font(hb.Face(open(path, "rb").read())) for path in paths]

    def shape(font, text):
        buf = hb.Buffer()
        buf.add_str(text)
        buf.guess_segment_properties()
        hb.shape(font, buf)
        return (
            [(i.codepoint, i.cluster) for i in buf.glyph_infos],
            [(p.x_advance, p.y_advance, p.x_offset, p.y_offset) for p in buf.glyph_positions],
        )

    rnd = random.Random(1)
    differing, first = 0, None
    for _ in range(SHAPED_STRINGS):
        start = rnd.randrange(len(code_points))
        window = code_points[start : start + 80]
        text = "".join(chr(rnd.choice(window)) for _ in range(rnd.randint(2, 8)))
        if shape(fonts[0], text) != shape(fonts[1], text):
            differing += 1
            first = first or text
    if differing:
        shown = " ".join(f"U+{ord(c):04X}" for c in first)
        return [f"{differing} of {SHAPED_STRINGS} strings differ, first {shown}"]
    return []


def check_bbox(_source, rebuilt, _paths):
    if "glyf" not in rebuilt["font"]:
        return None
    glyf = rebuilt["font"]["glyf"]
    names = rebuilt["font"].getGlyphOrder()
    wrong = []
    for g in spread(len(names)):
        glyph = glyf[names[g]]
        if glyph.numberOfContours == 0 or not hasattr(glyph, "xMin"):
            continue
        stored = (glyph.xMin, glyph.yMin, glyph.xMax, glyph.yMax)
        again = copy.deepcopy(glyph)
        again.recalcBounds(glyf)
        if stored != (again.xMin, again.yMin, again.xMax, again.yMax):
            wrong.append((g, stored, (again.xMin, again.yMin, again.xMax, again.yMax)))
    if wrong:
        g, stored, expected = wrong[0]
        return [f"{len(wrong)} glyphs differ, first glyph {g} ({names[g]}): {stored}, its points give {expected}"]
    return []


CHECKS = [
    ("structure/ots", check_ots),
    ("structure/parse", check_parse),
    ("roundtrip/cmap", check_cmap),
    ("roundtrip/outline", check_outline),
    ("roundtrip/advance", check_advance),
    ("roundtrip/origin", check_origin),
    ("roundtrip/shaping", check_shaping),
    ("derived/bbox", check_bbox),
]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("fonts", nargs="+", metavar="FONT")
    parser.add_argument("--bin", default="target/release", help="where otfccdump and otfccbuild are")
    parser.add_argument("--work", help="a directory for the rebuilt fonts (default: a temporary one)")
    parser.add_argument("--only", help="run the checks whose id starts with this")
    parser.add_argument(
        "--known",
        help="a file of known findings, one `FONT: CHECK` per line; they are printed but do not fail the run",
    )
    args = parser.parse_args()

    work = args.work or tempfile.mkdtemp(prefix="spec-check-")
    os.makedirs(work, exist_ok=True)
    known = set()
    if args.known:
        with open(args.known, encoding="utf-8") as lines:
            known = {line.strip() for line in lines if line.strip() and not line.startswith("#")}
    seen_known = set()
    findings = skipped = 0
    for path in args.fonts:
        name = os.path.basename(path)
        rebuilt_path = rebuild(path, args.bin, work)
        if rebuilt_path is None:
            print(f"{name}: roundtrip/builds: otfccdump or otfccbuild failed")
            findings += 1
            continue
        source = {"path": path, "font": TTFont(path, lazy=False)}
        rebuilt = {"path": rebuilt_path, "font": TTFont(rebuilt_path, lazy=False)}
        for check_id, check in CHECKS:
            if args.only and not check_id.startswith(args.only):
                continue
            try:
                found = check(source, rebuilt, (path, rebuilt_path))
            except Exception as error:  # a check that cannot run is a finding
                found = [f"the check failed: {type(error).__name__}: {error}"]
            if found is None:
                skipped += 1
                continue
            key = f"{name}: {check_id}"
            for line in found:
                print(f"{key}: {line}" + (" (known)" if key in known else ""))
            if found and key in known:
                seen_known.add(key)
            elif found:
                findings += len(found)
        os.remove(rebuilt_path)
    # A known finding that no longer shows is good news, and the list is
    # then out of date.
    for key in sorted(known - seen_known):
        if key.split(":")[0] in {os.path.basename(p) for p in args.fonts} and not args.only:
            print(f"{key}: no longer found; remove it from {args.known}")
            findings += 1
    print(
        f"{len(args.fonts)} fonts, {findings} new findings, {len(seen_known)} known, {skipped} checks skipped"
    )
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
