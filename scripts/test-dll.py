import ctypes
import sys

# Exercises the otfccdll C API (otfccbuild_json_otf / otfcc_get_buf_len /
# otfcc_get_buf_data / otfccbuild_free_otfbuf) via ctypes, against a shared
# library (either the C libotfccdll.{dylib,so} or the Rust cdylib), to
# compare byte-for-byte with the same JSON input.
#
# Usage: python3 test-dll.py <path-to-shared-lib> <path-to-input-json> <output-bin-path>

lib_path, json_path, out_path = sys.argv[1], sys.argv[2], sys.argv[3]

lib = ctypes.CDLL(lib_path)
lib.otfccbuild_json_otf.restype = ctypes.c_void_p
lib.otfccbuild_json_otf.argtypes = [
    ctypes.c_uint32, ctypes.c_char_p, ctypes.c_uint8, ctypes.c_bool,
]
lib.otfcc_get_buf_len.restype = ctypes.c_size_t
lib.otfcc_get_buf_len.argtypes = [ctypes.c_void_p]
lib.otfcc_get_buf_data.restype = ctypes.POINTER(ctypes.c_uint8)
lib.otfcc_get_buf_data.argtypes = [ctypes.c_void_p]
lib.otfccbuild_free_otfbuf.argtypes = [ctypes.c_void_p]

with open(json_path, "rb") as f:
    injson = f.read()

buf = lib.otfccbuild_json_otf(len(injson), injson, 0, False)
if not buf:
    print("FAIL: otfccbuild_json_otf returned NULL")
    sys.exit(1)

length = lib.otfcc_get_buf_len(buf)
data_ptr = lib.otfcc_get_buf_data(buf)
data = ctypes.string_at(data_ptr, length)

with open(out_path, "wb") as f:
    f.write(data)

lib.otfccbuild_free_otfbuf(buf)
print(f"OK: built {length} bytes -> {out_path}")
