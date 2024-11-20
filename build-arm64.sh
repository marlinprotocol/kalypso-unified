#!/bin/sh

# To be used inside a rust docker container for reproducible builds

set -e

cd /code

apt-get update && apt-get install -y --no-install-recommends \
    musl-dev \
    musl-tools \
    gcc-aarch64-linux-musl \
    wget \
    tar

# Add the Rust target for aarch64 with musl
rustup target add aarch64-unknown-linux-musl

# Download and extract the musl cross-compilation toolchain
MUSL_TOOLCHAIN_URL="https://musl.cc/aarch64-linux-musl-gcc-12.2.0-x86_64-linux-musl.tar.gz"
wget "$MUSL_TOOLCHAIN_URL" -O /tmp/aarch64-linux-musl-gcc.tar.gz
tar -xzf /tmp/aarch64-linux-musl-gcc.tar.gz -C /usr/local
rm /tmp/aarch64-linux-musl-gcc.tar.gz

# Add the musl toolchain to the PATH
export PATH=/usr/local/aarch64-linux-musl-gcc-12.2.0-x86_64-linux-musl/bin:$PATH

# Set the musl cross-compiler for the target
export CC_aarch64_unknown_linux_musl=aarch64-linux-musl-gcc

cargo build --release --target aarch64-unknown-linux-musl