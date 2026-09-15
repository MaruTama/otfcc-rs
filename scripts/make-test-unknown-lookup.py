#!/usr/bin/env python3
"""Forge a font whose GSUB/GPOS lookups have types no version of the spec defines.

None of the committed payloads has one, and that is a gap: otfcc reads a
lookup's type as `read_16u(data) + base` and *keeps whatever it gets*. An
unrecognised type gets no subtable reader, so the lookup dumps as `{}`, and if
the lookup has no name from the feature list it is named after the raw number in
hex (`lookup_0019_3`). That behaviour is why `otl_LookupType` is a newtype over
`u32` and not an `enum` — an enum could not hold the value, and rejecting it
would change the JSON otfcc writes. Without this payload, nothing checks that
the two implementations still agree there.

Derived from a committed payload rather than checked in as another binary. Only
the LookupType field of every lookup in GSUB and GPOS is touched; offsets, the
sfnt directory and the checksums are left alone, which is fine because otfcc
does not verify them and this font is only ever fed to otfccdump.

Usage: make-test-unknown-lookup.py <in.ttf> <out.ttf>
"""
import struct
import sys

# One past the last format either table defines (GSUB 8 = reverse chaining,
# GPOS 9 = extension), so it is unknown in both.
FORGED_FORMAT = 10


def table_offset(data, tag):
    for i in range(struct.unpack_from(">H", data, 4)[0]):
        entry = 12 + 16 * i
        if bytes(data[entry:entry + 4]) == tag:
            return struct.unpack_from(">I", data, entry + 8)[0]
    return None


def main():
    src, dst = sys.argv[1], sys.argv[2]
    data = bytearray(open(src, "rb").read())

    patched = {}
    for tag in (b"GSUB", b"GPOS"):
        base = table_offset(data, tag)
        if base is None:
            continue
        # GSUB/GPOS header 1.0: version(4) scriptList(2) featureList(2) lookupList(2)
        lookup_list = base + struct.unpack_from(">H", data, base + 8)[0]
        count = struct.unpack_from(">H", data, lookup_list)[0]
        for j in range(count):
            lookup = lookup_list + struct.unpack_from(">H", data, lookup_list + 2 + 2 * j)[0]
            struct.pack_into(">H", data, lookup, FORGED_FORMAT)
        patched[tag.decode()] = count

    if not patched:
        sys.exit(f"{src} has neither GSUB nor GPOS; pick a payload that has one")
    open(dst, "wb").write(bytes(data))
    counts = ", ".join(f"{t}: {n} lookups" for t, n in patched.items())
    print(f"  wrote {dst} ({counts} forced to format {FORGED_FORMAT})")


main()
