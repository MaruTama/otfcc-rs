#!/usr/bin/env python3
"""Download chapters of the Microsoft OpenType specification as plain text.

Usage: fetch_spec.py OUT_DIR CHAPTER...

A chapter is the last part of its URL: `otff`, `cmap`, `gpos`, `chapter2`.
Each one is written to OUT_DIR/CHAPTER.txt, one table row per line with
cells separated by ` | `, so that `grep -n` finds field names, format
numbers and versions. A chapter that cannot be downloaded is reported and
the exit status is 1.
"""
import html
import pathlib
import re
import subprocess
import sys

BASE = "https://learn.microsoft.com/en-us/typography/opentype/spec/"


def to_text(page: str) -> str:
    main = re.search(r"<main.*?</main>", page, re.S)
    text = main.group(0) if main else page
    text = re.sub(r"<(script|style).*?</\1>", "", text, flags=re.S)
    # One table row per line, whatever line breaks its cells hold.
    text = re.sub(
        r"<tr.*?</tr>",
        lambda row: re.sub(r"\s+", " ", row.group(0)),
        text,
        flags=re.S,
    )
    text = re.sub(r"</t[dh]>\s*", " | ", text)
    text = re.sub(r"</(tr|p|h\d|li|table|pre)>", "\n", text)
    text = html.unescape(re.sub(r"<[^>]+>", "", text))
    lines = (re.sub(r"\s+", " ", line).strip() for line in text.split("\n"))
    return "\n".join(line for line in lines if line) + "\n"


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__, file=sys.stderr)
        return 2
    out = pathlib.Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    failed = False
    for chapter in sys.argv[2:]:
        if not re.fullmatch(r"[a-z0-9_]+", chapter):
            print(f"{chapter}: not a chapter name", file=sys.stderr)
            failed = True
            continue
        # curl, not urllib: it follows the proxy and CA settings of the
        # environment without extra configuration.
        result = subprocess.run(
            ["curl", "-sSfL", "-m", "60", BASE + chapter],
            capture_output=True,
        )
        if result.returncode != 0:
            message = result.stderr.decode(errors="replace").strip()
            print(f"{chapter}: download failed: {message}", file=sys.stderr)
            failed = True
            continue
        text = to_text(result.stdout.decode("utf-8", errors="replace"))
        (out / f"{chapter}.txt").write_text(text, encoding="utf-8")
        print(f"{chapter}: {text.count(chr(10))} lines")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
