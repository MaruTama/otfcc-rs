import re, sys, glob

# Fix c2rust's ABI-breaking pattern:
#   ::core::mem::transmute::<_, fn(ARGS) -> RET>( EXPR )
# which drops `unsafe extern "C"`, corrupting calls that return structs by
# value. EXPR already has the correct `unsafe extern "C" fn(...) -> RET`
# type (from `.expect("non-null function pointer")` on an Option), so the
# transmute is both unnecessary and actively harmful. Rewrite the whole
# `transmute::<_, ...>( EXPR )` to just `( EXPR )`, preserving the trailing
# call `()` / arguments and outer context untouched.

pat = re.compile(
    r'::core::mem::transmute::<_,\s*fn\([^)]*\)\s*->\s*[^>]+>\s*\(',
)

def fix_file(path):
    with open(path) as f:
        text = f.read()
    out = []
    i = 0
    n = 0
    while True:
        m = pat.search(text, i)
        if not m:
            out.append(text[i:])
            break
        out.append(text[i:m.start()])
        # find matching close paren for the '(' at end of match
        depth = 1
        j = m.end()
        while depth > 0:
            if text[j] == '(':
                depth += 1
            elif text[j] == ')':
                depth -= 1
            j += 1
        inner = text[m.end():j-1]
        # The original multi-line call args may leave a trailing comma
        # (e.g. `expr.expect(...),`), which would turn `(inner)` into a
        # 1-tuple instead of a parenthesized call target. Strip it.
        inner = inner.rstrip()
        if inner.endswith(','):
            inner = inner[:-1]
        out.append('(' + inner + ')')
        i = j
        n += 1
    if n:
        with open(path, 'w') as f:
            f.write(''.join(out))
    return n

total = 0
files = 0
for path in glob.glob(sys.argv[1] + '/**/*.rs', recursive=True):
    n = fix_file(path)
    if n:
        files += 1
        total += n
print(f"patched {total} occurrences in {files} files")
