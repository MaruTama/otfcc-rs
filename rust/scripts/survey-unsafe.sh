#!/usr/bin/env bash
# Reports the residual-work counters the Phase 5 plan tracks across
# Stage 7-1/7-2/7-4 (parse-boundary safety, ownership, C-ism cleanup) --
# the same numbers rust/README.md's "Next steps" entries cite when a PR
# reports what it moved. Each PR that touches these areas should paste this
# script's output (before/after, or just after with the delta called out)
# into its own writeup, the same way `grep -rc
# "allow(unsafe_op_in_unsafe_fn)"` already serves as the burn-down's own
# counter.
#
# Read-only: makes no changes, just counts.
#
# Invoke as: ./rust/scripts/survey-unsafe.sh
set -euo pipefail
cd "$(dirname "$0")/../src"

count() {
	# count <label> <grep-pattern...>
	local label="$1"
	shift
	printf '%-45s %s\n' "${label}" "$(grep -rho "$@" --include='*.rs' . | wc -l | tr -d ' ')"
}

echo "== rust/src residual-unsafety counters =="
printf '%-45s %s\n' "files with allow(unsafe_op_in_unsafe_fn)" \
	"$(grep -rl 'allow(unsafe_op_in_unsafe_fn)' --include='*.rs' . | wc -l | tr -d ' ') / $(find . -name '*.rs' | wc -l | tr -d ' ')"
count "unsafe fn" -E 'unsafe fn '
count "unsafe blocks" 'unsafe {'
count "raw pointer types (*mut / *const)" -E '\*(mut|const) '
count ".offset( calls" '\.offset('
count "is_null() calls" 'is_null()'
count "__fortable_* (foreach-macro emulation)" -E '__fortable_[a-z0-9_]*'
count "current_block (goto emulation)" 'current_block'
count "c2rust_unnamed (union field access)" 'c2rust_unnamed'
count "as ::core::ffi::c_int casts" 'as ::core::ffi::c_int'
count "while loops" -E '\bwhile '
count "Result< usage" 'Result<'
count "Option< usage" 'Option<'
