#!/usr/bin/env python3
"""Inject a synthetic 'BASE' table into an existing canonical payload JSON.

None of the committed payloads has a BASE table, and that is a real gap:
every otfcc build/read/dump/parse path touching it (table/base.rs) has gone
untested by compare-with-c.sh's byte comparison. Two scripts (one with a
default baseline and two named baselines, one with only a single baseline
and no default) on both the horizontal and vertical axis, to exercise the
multi-tag BkBlock assembly in axis_to_bk (dedup of shared baseline tags
across scripts) as well as the "entry has zero baselines" and "no default
baseline" branches.

Usage: make-test-base.py <in.json> <out.json>
"""
import json
import sys


def main():
    src, dst = sys.argv[1], sys.argv[2]
    with open(src) as f:
        data = json.load(f)

    data["BASE"] = {
        "horizontal": {
            "latn": {
                "defaultBaseline": "romn",
                "baselines": {"romn": 0, "hang": -120, "icfb": 40},
            },
            "DFLT": {
                "baselines": {"romn": 0},
            },
        },
        "vertical": {
            "latn": {
                "defaultBaseline": "ideo",
                "baselines": {"ideo": 0, "romn": 880},
            },
        },
    }

    with open(dst, "w") as f:
        json.dump(data, f, indent=2)


if __name__ == "__main__":
    main()
