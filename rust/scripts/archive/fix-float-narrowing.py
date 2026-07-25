import re, sys, glob

# Fix c2rust's mistranslation of C's implicit `pos_t` (double) -> `uintN_t`
# narrowing conversion at bufwriteNNb() call sites.
#
# When the C source has an EXPLICIT intermediate cast (e.g.
# `bufwrite16b(buf, (int16_t)g->stat.xMin)`), c2rust translates it correctly
# as `xMin as int16_t as uint16_t`. But when the C source relies on an
# IMPLICIT double->uint16_t conversion at the call boundary (no explicit cast
# in the C source, e.g. `bufwrite16b(buf, hmtx->metrics[j].lsb)` where `lsb`
# is `pos_t`), c2rust emits a direct `lsb as uint16_t`.
#
# This differs from C's actual (compiler-consistent, if technically
# unspecified) behavior for negative values: C effectively converts through a
# signed integer first, then reinterprets the bits as unsigned (e.g. -41.0 ->
# 0xFFD7). Rust's `as` for float->unsigned instead SATURATES negative values
# to 0. Observed impact: any negative pos_t value serialized this way (glyph
# left/top side bearings, vertical origins, ...) is corrupted to 0 instead of
# its correct two's-complement bit pattern.
#
# Fix: for each known `pos_t`-typed field written via `bufwriteNNb(buf, EXPR
# as uintN_t)` with no signed intermediate, insert the signed intermediate
# cast: `EXPR as intN_t as uintN_t`, which reproduces C's actual behavior
# (verified: -41.0f64 as i16 as u16 == 0xFFD7, matching C's compiled output).
#
# This is a narrow, field-name-targeted fix (not a blanket rewrite of every
# `as uintN_t` in the tree) because many other call sites legitimately widen
# non-negative unsigned values (glyph indices, counts, offsets) up to 65535,
# where routing through a signed intermediate would wrongly saturate them.

# (file, field-access regex fragment, bit width) — every pos_t-typed field
# found (via grep across c/include/otfcc/table/*.h) to reach a bufwriteNNb call
# in the C sources without an explicit intermediate cast.
TARGETS = [
    ("lib/table/hmtx.rs", r"\(\*\(\*hmtx\)\.metrics\.offset\(j as isize\)\)\.lsb", 16),
    ("lib/table/hmtx.rs", r"\*\(\*hmtx\)\.leftSideBearing\.offset\(j_0 as isize\)", 16),
    ("lib/table/vmtx.rs", r"\(\*\(\*vmtx\)\.metrics\.offset\(j as isize\)\)\.tsb", 16),
    ("lib/table/vmtx.rs", r"\*\(\*vmtx\)\.topSideBearing\.offset\(j_0 as isize\)", 16),
    ("lib/table/VORG.rs", r"\(\*table\)\.defaultVerticalOrigin", 16),
]

def fix_file(path, field_pat, bits):
    with open(path) as f:
        text = f.read()
    pat = re.compile(field_pat + r"\s+as\s+uint" + str(bits) + r"_t")
    def repl(m):
        return m.group(0).replace(
            f"as uint{bits}_t", f"as int{bits}_t as uint{bits}_t"
        )
    new_text, n = pat.subn(repl, text)
    if n:
        with open(path, "w") as f:
            f.write(new_text)
    return n

root = sys.argv[1]
total = 0
for rel, field_pat, bits in TARGETS:
    path = f"{root}/src/{rel}"
    n = fix_file(path, field_pat, bits)
    total += n
    if n == 0:
        print(f"WARNING: pattern not found (crate structure may have changed): {rel}: {field_pat}", file=sys.stderr)
print(f"patched {total} float-narrowing-cast occurrences")
