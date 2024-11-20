#!/bin/sh

# To be used inside a rust docker container for reproducible builds

set -e

cd /code

rustup target add aarch64-unknown-linux-musl

apt-get install -y musl-dev musl-tools gcc-aarch64-linux-musl

cargo build --release --target aarch64-unknown-linux-musl