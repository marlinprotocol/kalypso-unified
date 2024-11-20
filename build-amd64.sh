#!/bin/sh

# To be used inside a rust docker container for reproducible builds

set -e

cd /code

apt-get update && apt-get install -y --no-install-recommends \
    musl-dev \
    musl-tools \
    musl-gcc

# Add the musl target for Rust
rustup target add x86_64-unknown-linux-musl

# Set the compiler for the musl target
export CC_x86_64_unknown_linux_musl=musl-gcc

cargo build --release --target x86_64-unknown-linux-musl