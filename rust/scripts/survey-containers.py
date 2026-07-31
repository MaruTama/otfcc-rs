#!/usr/bin/env python3
"""Classify the remaining CVecRaw<T>-shaped containers by conversion risk.

Stage 6 replaces `support/cvec.rs`'s `CVecRaw<T>` (length/capacity/items)
containers with plain `Vec<T>`, one type at a time. Three conversions in
(VqSegList, GaspRecordList, MetaEntries) it became clear the difficulty isn't
predicted by which file a container lives in or how simple its element looks
-- it's three independent questions, checked here for every remaining
container in one pass instead of re-deriving them by hand before each PR:

  1. Does the element own a resource (an `sds` string, another CVecRaw
     container, a non-Copy type like VQ, or any owned raw pointer)? If so,
     the container's own `Vec::drop` won't be enough -- a manual dispose
     loop has to stay, freeing each element first (see MetaEntries).
  2. Is the container ever embedded *by value* in a struct in another file?
     If so, that owning struct loses `Copy` too, and so does anything that
     embeds *that* by value -- check the whole chain before starting (see
     VQ's cascade into Point/ComponentReference/Glyph/CffFontMatrix).
  3. Does the container (or its element type) appear by value in any
     function signature, particularly an `extern "C" fn`? A non-repr(C),
     non-Copy type passed by value through `extern "C"` trips
     `improper_ctypes_definitions` at every such site.

None of these is optional to check -- skipping any one of them is what
turned the VQ pilot into the most invasive of the three conversions.

Usage: survey-containers.py   (run from rust/, no arguments)
"""
import re
import os

SRC = "src"


def main():
    files = {}
    for root, _, names in os.walk(SRC):
        for n in names:
            if n.endswith(".rs"):
                p = os.path.join(root, n)
                files[p] = open(p).read()
    all_src = "\n".join(files.values())

    pat = re.compile(
        r'pub struct (\w+)\s*\{\s*pub length: usize,\s*pub capacity: usize,'
        r'\s*pub items: \*mut (\w+),?\s*\}',
        re.S,
    )
    containers = []
    for p, txt in files.items():
        for m in pat.finditer(txt):
            containers.append((m.group(1), m.group(2), p))

    def struct_body(name):
        m = re.search(r'pub struct ' + re.escape(name) + r'\s*\{(.*?)\n\}', all_src, re.S)
        return m.group(1) if m else None

    def elem_owns_resources(elem):
        body = struct_body(elem)
        if body is None:
            return "?"
        hits = []
        if re.search(r':\s*SdsRaw', body):
            hits.append("sds")
        for c, _, _ in containers:
            if re.search(r':\s*' + re.escape(c) + r'\b', body):
                hits.append(c)
        if re.search(r':\s*VQ\b', body):
            hits.append("VQ(non-Copy)")
        if re.search(r':\s*\*mut ', body):
            hits.append("rawptr")
        return ",".join(sorted(set(hits))) if hits else "-"

    def embedded_by_value_outside(cont, own_file):
        out = set()
        for p, txt in files.items():
            if p == own_file:
                continue
            if re.search(r'pub \w+:\s*' + re.escape(cont) + r'\s*,', txt):
                out.add(p)
        return sorted(out)

    def passed_by_value(cont):
        n = 0
        for m in re.finditer(r'fn [\w:]+\s*\([^)]*\)', all_src, re.S):
            sig = m.group(0)
            for mm in re.finditer(r'(?<![\w*])' + re.escape(cont) + r'(?![\w])', sig):
                before = sig[:mm.start()]
                if not before.rstrip().endswith("*mut") and not before.rstrip().endswith("*const"):
                    n += 1
        return n

    print(f"{'container':26s} {'elem':24s} {'elem-owns':22s} {'byval-outside':28s} byval-sig")
    print("-" * 115)
    rows = []
    for c, e, p in sorted(containers, key=lambda x: (x[2], x[0])):
        owns = elem_owns_resources(e)
        ext = embedded_by_value_outside(c, p)
        bv = passed_by_value(c)
        rows.append((c, e, p, owns, ext, bv))
        exts = ",".join(os.path.basename(x) for x in ext) or "-"
        print(f"{c:26s} {e:24s} {owns:22s} {exts:28s} {bv}")

    print()
    simple = [r for r in rows if r[3] == "-" and not r[4] and r[5] == 0]
    print(f"=== Straightforward (element owns nothing, never embedded by value "
          f"outside its file, never passed by value): {len(simple)}")
    for r in simple:
        print("   ", r[0], "  <-", os.path.basename(r[2]))


if __name__ == "__main__":
    main()
