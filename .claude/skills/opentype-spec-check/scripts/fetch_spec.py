#!/usr/bin/env python3
"""Download font specifications as plain text.

Usage: fetch_spec.py OUT_DIR SPEC...

A SPEC names one document:

  cmap, ot:cmap   a chapter of Microsoft's OpenType specification
                  (the last part of its URL: otff, gpos, chapter2, os2)
  aat:morx        a chapter of Apple's TrueType Reference Manual, by table
                  tag; aat:overview is the list of tables
  woff:1, woff:2  the W3C WOFF 1.0 and WOFF 2.0 recommendations
  w3c:png, w3c:eot  the PNG recommendation and the EOT submission
  adobe:5176      an Adobe technical note: 5176 (CFF), 5177 (Type 2
                  charstrings), t1 (Type 1 font format), 5015 (Type 1
                  supplement), 5014 (CID-keyed fonts), 5092 (CID overview),
                  5094 (CJK CID-keyed fonts), 5099 (CMap resources),
                  5012 (Type 42), 5180 (sfnt-wrapped CID fonts),
                  5088 (font naming), 5902 (PostScript name generation)
  adobe:japan1    the Adobe-Japan1 character collection's description
  ps:plrm         the PostScript Language Reference, third edition
  agl:spec, agl:glyphlist, agl:aglfn
                  the Adobe Glyph List specification and its two lists
  unicode:Blocks  a file of the Unicode Character Database, by its name
                  without `.txt` (Blocks, Scripts, StandardizedVariants,
                  UnicodeData); unicode:tr37 is the Ideographic Variation
                  Database report
  mac:ROMAN       an Apple character set mapping to Unicode, by its name
                  (ROMAN, JAPANESE, ...)
  rfc:8259        an RFC by number: 8259 (JSON), 5646 (BCP 47 language
                  tags), 1950 (zlib), 1952 (gzip), 7932 (Brotli),
                  2315 (PKCS #7)

Each one is written to OUT_DIR/<name>.txt, where <name> is the SPEC with
`:` replaced by `-` (an OpenType chapter is just its name: cmap.txt). HTML
documents have one table row per line with cells separated by ` | `, so
that `grep -n` finds field names, format numbers and versions. PDF
documents are converted with `pdftotext -layout`. Plain-text documents are
kept as they are.

A document that cannot be downloaded or converted is reported and the exit
status is 1.
"""
import html
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

OPENTYPE = "https://learn.microsoft.com/en-us/typography/opentype/spec/"
AAT = "https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6"
ADOBE = "https://adobe-type-tools.github.io/font-tech-notes/pdfs/"
WOFF = {"1": "https://www.w3.org/TR/WOFF/", "2": "https://www.w3.org/TR/WOFF2/"}
W3C = {
    "png": "https://www.w3.org/TR/png/",
    "eot": "https://www.w3.org/submissions/EOT/",
}
ADOBE_GITHUB = "https://raw.githubusercontent.com/adobe-type-tools/"
AGL = {
    "spec": ADOBE_GITHUB + "agl-specification/master/README.md",
    "glyphlist": ADOBE_GITHUB + "agl-aglfn/master/glyphlist.txt",
    "aglfn": ADOBE_GITHUB + "agl-aglfn/master/aglfn.txt",
}
UCD = "https://www.unicode.org/Public/UCD/latest/ucd/"
UNICODE_REPORTS = {"tr37": "https://www.unicode.org/reports/tr37/"}
MAC = "https://www.unicode.org/Public/MAPPINGS/VENDORS/APPLE/"
RFC = "https://www.rfc-editor.org/rfc/rfc"
ADOBE_NOTES = {
    "5176": "5176.CFF.pdf",
    "5177": "5177.Type2.pdf",
    "t1": "T1_SPEC.pdf",
    "5015": "5015.Type1_Supp.pdf",
    "5014": "5014.CIDFont_Spec.pdf",
    "5092": "5092.CID_Overview.pdf",
    "5012": "5012.Type42_Spec.pdf",
    "5094": "5094.CJK_CID.pdf",
    "5099": "5099.CMapResources.pdf",
    "5180": "5180.sfnt.pdf",
    "5088": "5088.FontNames.pdf",
    "5902": "5902.AdobePSNameGeneration.pdf",
}
ADOBE_OTHER = {"japan1": ADOBE_GITHUB + "Adobe-Japan1/master/README.md"}
POSTSCRIPT = {"plrm": "https://www.adobe.com/jp/print/postscript/pdfs/PLRM.pdf"}


def resolve(spec: str) -> tuple[str, str] | None:
    """The URL of `spec` and the name of its output file, without `.txt`."""
    source, _, name = spec.rpartition(":")
    source = source or "ot"
    if not re.fullmatch(r"[A-Za-z0-9_]+", name):
        return None
    if source == "ot":
        return OPENTYPE + name.lower(), name.lower()
    if source == "aat":
        chapter = "" if name == "overview" else name
        return f"{AAT}{chapter}.html", f"aat-{name}"
    if source == "woff" and name in WOFF:
        return WOFF[name], f"woff-{name}"
    if source == "w3c" and name.lower() in W3C:
        return W3C[name.lower()], f"w3c-{name.lower()}"
    if source == "adobe" and name.lower() in ADOBE_NOTES:
        return ADOBE + ADOBE_NOTES[name.lower()], f"adobe-{name.lower()}"
    if source == "adobe" and name.lower() in ADOBE_OTHER:
        return ADOBE_OTHER[name.lower()], f"adobe-{name.lower()}"
    if source == "agl" and name.lower() in AGL:
        return AGL[name.lower()], f"agl-{name.lower()}"
    if source == "unicode" and name.lower() in UNICODE_REPORTS:
        return UNICODE_REPORTS[name.lower()], f"unicode-{name.lower()}"
    if source == "unicode":
        return f"{UCD}{name}.txt", f"unicode-{name}"
    if source == "mac":
        return f"{MAC}{name.upper()}.TXT", f"mac-{name.upper()}"
    if source == "rfc" and name.isdigit():
        return f"{RFC}{name}.txt", f"rfc-{name}"
    if source == "ps" and name.lower() in POSTSCRIPT:
        return POSTSCRIPT[name.lower()], f"ps-{name.lower()}"
    return None


def html_to_text(page: str) -> str:
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
    text = re.sub(r"</(tr|p|h\d|li|table|pre|dt|dd|div)>|<br ?/?>", "\n", text)
    text = html.unescape(re.sub(r"<[^>]+>", "", text))
    lines = (re.sub(r"\s+", " ", line).strip() for line in text.split("\n"))
    return "\n".join(line for line in lines if line) + "\n"


def pdf_to_text(data: bytes) -> str:
    if not data.startswith(b"%PDF"):
        raise RuntimeError("not a PDF")
    if shutil.which("pdftotext") is None:
        raise RuntimeError("pdftotext is not installed (package poppler-utils)")
    with tempfile.TemporaryDirectory() as tmp:
        pdf = pathlib.Path(tmp, "in.pdf")
        pdf.write_bytes(data)
        result = subprocess.run(
            ["pdftotext", "-layout", str(pdf), "-"], capture_output=True
        )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.decode(errors="replace").strip())
    return result.stdout.decode("utf-8", errors="replace")


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__, file=sys.stderr)
        return 2
    out = pathlib.Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    failed = False
    for spec in sys.argv[2:]:
        resolved = resolve(spec)
        if resolved is None:
            print(f"{spec}: not a known document", file=sys.stderr)
            failed = True
            continue
        url, name = resolved
        # curl, not urllib: it follows the proxy and CA settings of the
        # environment without extra configuration.
        result = subprocess.run(
            ["curl", "-sSfL", "-m", "180", url], capture_output=True
        )
        if result.returncode != 0:
            message = result.stderr.decode(errors="replace").strip()
            print(f"{spec}: download failed: {message}", file=sys.stderr)
            failed = True
            continue
        try:
            if url.endswith(".pdf"):
                text = pdf_to_text(result.stdout)
            elif url.lower().endswith((".txt", ".md")):
                text = result.stdout.decode("utf-8", errors="replace")
            else:
                text = html_to_text(result.stdout.decode("utf-8", errors="replace"))
        except RuntimeError as error:
            print(f"{spec}: {error}", file=sys.stderr)
            failed = True
            continue
        (out / f"{name}.txt").write_text(text, encoding="utf-8")
        print(f"{spec}: {name}.txt, {text.count(chr(10))} lines")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
