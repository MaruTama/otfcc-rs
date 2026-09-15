# Sourced helper: sha256_of <file> prints the file's SHA-256 as a bare hex
# string. Wraps whichever of sha256sum (Linux/coreutils) or shasum -a 256
# (macOS/Perl) is available, so callers don't need to care which platform
# they're on.
sha256_of() {
	if command -v sha256sum >/dev/null 2>&1; then
		sha256sum "$1" | awk '{print $1}'
	else
		shasum -a 256 "$1" | awk '{print $1}'
	fi
}
