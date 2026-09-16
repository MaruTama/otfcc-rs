# c2rust transpile environment for otfcc (issue #2, Phase 1) — native arm64.
#
# Built for the host's native architecture (arm64): an amd64 image under QEMU
# emulation hits a fatal GNU Make jobserver bug while building c2rust's vendored
# tinycbor. Native avoids emulation entirely.
#
# Uses the modern coherent stack — Ubuntu 24.04 (LLVM/Clang 17) + c2rust 0.22.1.
# (clang-18 tripped an unrelated c2rust ast-exporter bug on some ordinary code
# in this codebase; clang-17 didn't.)
#
# c2rust cannot represent the SIMD vector-math declarations in aarch64 glibc's
# <bits/math-vector.h> (the `_ZGVnN..` libm variants) and panics with "Could not
# find CTypeId ... in TypedAstContext". c2rust's frontend ignores --target and
# --sysroot, so it always parses the native aarch64 headers. otfcc never uses
# the vectorized libm, so we replace that header with glibc's empty SIMD-decl
# stubs (which still define the __DECL_SIMD_* macros mathcalls.h needs).
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
        clang-17 \
        llvm-17 \
        llvm-17-dev \
        libclang-17-dev \
        cmake \
        pkg-config \
        libssl-dev \
        zlib1g-dev \
        python3 \
        git \
        curl \
        ca-certificates \
        xz-utils \
    && rm -rf /var/lib/apt/lists/* \
    && ln -sf /usr/bin/clang-17 /usr/bin/clang \
    && echo '#include <bits/libm-simd-decl-stubs.h>' \
        > /usr/include/aarch64-linux-gnu/bits/math-vector.h

# c2rust-ast-exporter's build.rs needs LLVM pointed out explicitly.
ENV LLVM_CONFIG_PATH=/usr/bin/llvm-config-17
ENV LLVM_LIB_DIR=/usr/lib/llvm-17/lib
ENV LIBCLANG_PATH=/usr/lib/llvm-17/lib

# Recent stable builds current c2rust cleanly; nightly compiles the output.
ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo
ENV PATH=/opt/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --default-toolchain stable --profile minimal \
    && rustup toolchain install nightly --profile minimal \
    && rustup component add rustfmt

# Latest c2rust (2024): fixes the AST-export panics and supports LLVM 18.
RUN cargo install c2rust --version 0.22.1 --locked

WORKDIR /work
ENTRYPOINT ["/bin/bash"]
