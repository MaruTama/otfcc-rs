#!/usr/bin/env python3
"""Inject a synthetic 'meta' table into an existing canonical payload JSON.

None of the committed payloads has a 'meta' table, and that is a real gap:
every otfcc build touching it (table/meta/{types,read,parse,build,dump}.rs)
has gone untested by compare-with-c.sh's byte comparison. Covers a
string-valued entry via each of the two well-known string tags ('dlng'/
'slng', see otfcc_dump_meta's is_string_tag), plus one non-string tag that
round-trips through the base64 path.

Usage: make-test-meta.py <in.json> <out.json>
"""
import json
import sys


def main():
    src, dst = sys.argv[1], sys.argv[2]
    with open(src) as f:
        data = json.load(f)

    data["meta"] = {
        "version": 1,
        "flags": 0,
        "entries": [
            {"tag": "dlng", "string": "Latn, Grek"},
            {"tag": "slng", "string": "Latn"},
            {"tag": "test", "base64": "AQIDBA=="},
        ],
    }

    with open(dst, "w") as f:
        json.dump(data, f, indent=2)


if __name__ == "__main__":
    main()
