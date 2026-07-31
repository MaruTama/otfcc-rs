# Builds a minimal, self-contained variable font (fvar + gvar, one "wght"
# axis, two masters interpolated) purely from fontTools APIs — no external
# download. Used to exercise otfcc's gvar delta-application path
# (applyPolymorphism in c/lib/table/glyf/read.c), which none of the existing
# tests/payload/*.ttf fonts exercise (confirmed: none has an fvar table).
#
# Also carries two named instances (Regular/Bold), so the built fvar's
# `instances` array (FvarInstanceList/FvarInstance/VV in rust/src/table/
# fvar.rs and rust/src/vf/{vv,vq}.rs) is exercised too -- without them the
# axes-only fvar this used to produce left otfcc_read_fvar/otfcc_dump_fvar's
# instance-parsing loop, and json_new_v_vp's coordinate dump, entirely
# untested by compare-with-c.sh/run-cycles.sh.
import fontTools.fontBuilder as fb
from fontTools.varLib import build
import os

GLYPHS = [".notdef", "space", "A"]
UNITS_PER_EM = 1000
ADVANCE = 600


def make_master(path, a_contour, weight_class):
    f = fb.FontBuilder(UNITS_PER_EM, isTTF=True)
    f.setupGlyphOrder(GLYPHS)
    f.setupCharacterMap({0x20: "space", 0x41: "A"})
    pen_glyphs = {".notdef": None, "space": None, "A": a_contour}

    from fontTools.pens.ttGlyphPen import TTGlyphPen

    glyphs = {}
    for name in GLYPHS:
        pen = TTGlyphPen(None)
        if name == "A" and a_contour:
            pen.moveTo(a_contour[0])
            for pt in a_contour[1:]:
                pen.lineTo(pt)
            pen.closePath()
        glyphs[name] = pen.glyph()
    f.setupGlyf(glyphs)

    metrics = {n: (ADVANCE, 0) for n in GLYPHS}
    f.setupHorizontalMetrics(metrics)
    f.setupHorizontalHeader(ascent=800, descent=-200)
    f.setupNameTable({"familyName": "OtfccGvarTest", "styleName": "Regular"})
    f.setupOS2(sTypoAscender=800, sTypoDescender=-200, usWeightClass=weight_class)
    f.setupPost()
    f.save(path)


def main():
    out_dir = os.path.join(os.path.dirname(__file__), "..", "..", "build")
    os.makedirs(out_dir, exist_ok=True)

    # Two masters: a narrow "A" outline (light) and a wide one (bold). The
    # outline shifts, giving gvar real per-point deltas to encode/apply.
    light = os.path.join(out_dir, "gvar-master-light.ttf")
    bold = os.path.join(out_dir, "gvar-master-bold.ttf")
    make_master(light, [(50, 0), (250, 700), (450, 0)], 400)
    make_master(bold, [(30, 0), (300, 700), (570, 0)], 700)

    # designspace
    from fontTools.designspaceLib import (
        DesignSpaceDocument, AxisDescriptor, SourceDescriptor, InstanceDescriptor,
    )

    doc = DesignSpaceDocument()
    axis = AxisDescriptor()
    axis.tag = "wght"
    axis.name = "Weight"
    axis.minimum = 400
    axis.default = 400
    axis.maximum = 700
    doc.addAxis(axis)

    s1 = SourceDescriptor()
    s1.path = light
    s1.name = "light"
    s1.location = {"Weight": 400}
    doc.addSource(s1)

    s2 = SourceDescriptor()
    s2.path = bold
    s2.name = "bold"
    s2.location = {"Weight": 700}
    doc.addSource(s2)

    i1 = InstanceDescriptor()
    i1.name = "Regular"
    i1.styleName = "Regular"
    i1.familyName = "OtfccGvarTest"
    i1.location = {"Weight": 400}
    doc.addInstance(i1)

    i2 = InstanceDescriptor()
    i2.name = "Bold"
    i2.styleName = "Bold"
    i2.familyName = "OtfccGvarTest"
    i2.location = {"Weight": 700}
    doc.addInstance(i2)

    varfont, _, _ = build(doc)
    out = os.path.join(out_dir, "gvar-test.ttf")
    varfont.save(out)
    print(f"wrote {out}")
    print(f"tables: {sorted(varfont.keys())}")


if __name__ == "__main__":
    main()
