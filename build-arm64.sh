#!/bin/sh

# To be used inside a rust docker container for reproducible builds

set -e

cd /code

apt-get install -y musl-dev musl-tools gcc-aarch64-linux-musl

rustup target add aarch64-unknown-linux-musl

wget https://musl.cc/aarch64-linux-musl-gcc-12.2.0-x86_64-linux-musl.tar.gz

tar -xzf aarch64-linux-musl-gcc-12.2.0-x86_64-linux-musl.tar.gz -C /usr/local

export PATH=/usr/local/aarch64-linux-musl-gcc-12.2.0-x86_64-linux-musl/bin:$PATH

export CC_aarch64_unknown_linux_musl=aarch64-linux-musl-gcc

cargo build --release --target aarch64-unknown-linux-musl