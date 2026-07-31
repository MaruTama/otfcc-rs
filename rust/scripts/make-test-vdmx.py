#!/usr/bin/env python3
"""Inject a synthetic 'VDMX' table into an existing canonical payload JSON.

None of the committed payloads has a VDMX table, and that is a real gap:
every otfcc build touching it (table/vdmx/{types,funcs}.rs) has gone untested
by compare-with-c.sh's byte comparison. Two ratio ranges, one with several
size records (exercises the min/max scan in otfcc_build_vdmx) and one with a
single record, to touch both the nested-Vec push paths and the group-block
bk_new_block sizing.

Usage: make-test-vdmx.py <in.json> <out.json>
"""
import json
import sys


def main():
    src, dst = sys.argv[1], sys.argv[2]
    with open(src) as f:
        data = json.load(f)

    data["VDMX"] = {
        "version": 1,
        "ratios": [
            {
                "bCharset": 1,
                "xRatio": 1,
                "yStartRatio": 0,
                "yEndRatio": 0,
                "records": [
                    {"yPelHeight": 9, "yMax": 8, "yMin": -2},
                    {"yPelHeight": 10, "yMax": 9, "yMin": -2},
                    {"yPelHeight": 12, "yMax": 11, "yMin": -3},
                ],
            },
            {
                "bCharset": 1,
                "xRatio": 2,
                "yStartRatio": 1,
                "yEndRatio": 0,
                "records": [
                    {"yPelHeight": 16, "yMax": 15, "yMin": -4},
                ],
            },
        ],
    }

    with open(dst, "w") as f:
        json.dump(data, f, indent=2)


if __name__ == "__main__":
    main()
