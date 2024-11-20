#!/bin/sh

# To be used inside a rust docker container for reproducible builds

set -e

cd /code

apt-get install -y musl-dev musl-tools musl-gcc

rustup target add x86_64-unknown-linux-musl

cargo build --release --target x86_64-unknown-linux-musl