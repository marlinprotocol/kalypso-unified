#!/bin/sh

# To be used inside a rust docker container for reproducible builds

set -e

cd /code

rm -rf .cargo

rustup target add aarch64-unknown-linux-musl

cargo build --release --target aarch64-unknown-linux-musl