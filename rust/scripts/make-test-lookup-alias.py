#!/usr/bin/env python3
"""Forge a font whose GSUB "lookups" object contains a *string-valued* entry
-- a lookup alias, i.e. a second name for an already-defined lookup, as
opposed to the usual object-valued (real definition) entry.

None of the committed payloads has one, and that is a gap: otfcc's "lookups"
JSON object supports this exact shape (see `figureOutLookupsFromJSON` in
c/lib/table/otl/parse.c, and its Rust translation in
rust/src/table/otl/parse.rs), but nothing exercises it. Worse, in the
ORIGINAL C code this path has a genuine double-push bug -- the final drain
loop over the lookup hash table pushes every node's `.lookup` unconditionally,
including alias nodes, so an alias entry causes the same `Lookup` to be
pushed (and later freed) twice. C segfaults on this input; a naive Rust
`Vec<Box<Lookup>>` translation hangs in an infinite loop (Box::from_raw'd
twice, corrupting the allocator's free list). See rust/README.md for the
full writeup and the fix (LookupHash gained an `alias: bool` field mirroring
FeatureHash's existing one -- Rust-only, by explicit decision, since a C fix
was out of scope here).

Because C still crashes on this input, this payload is deliberately NOT
wired into compare-with-c.sh's byte-for-byte comparison (there is nothing
for Rust's output to be compared against). It exists purely so a Rust-only
regression test can assert the crate no longer crashes/hangs on it and
produces stable, deterministic output.

Derived from a committed JSON payload rather than checked in as another
file. Only the GSUB "lookups" object is touched.

Usage: make-test-lookup-alias.py <in.json> <out.json>
"""
import json
import sys


def main():
    src, dst = sys.argv[1], sys.argv[2]
    with open(src) as f:
        font = json.load(f)

    gsub = font.get("GSUB")
    if not isinstance(gsub, dict) or "lookups" not in gsub:
        sys.exit(f"{src} has no GSUB.lookups; pick a payload that does")

    lookups = gsub["lookups"]
    real_lookups = [k for k, v in lookups.items() if isinstance(v, dict)]
    if not real_lookups:
        sys.exit(f"{src} has no real (object-valued) GSUB lookups to alias")
    target = real_lookups[0]

    alias_name = "lookup_alias_test"
    lookups[alias_name] = target

    # Reference the alias from a feature so it is reachable the same way a
    # real font's alias would be, not just present-but-unused in the table.
    features = gsub.get("features")
    if isinstance(features, dict) and features:
        first_feature = next(iter(features.values()))
        if isinstance(first_feature, list) and target in first_feature:
            first_feature.append(alias_name)

    with open(dst, "w") as f:
        json.dump(font, f)
    print(f"  wrote {dst} (GSUB.lookups.{alias_name} aliases {target})")


main()
