#!/bin/sh

ROOT_DIR="$(dirname $(readlink -f $0))/.."
cd "$ROOT_DIR" && \
RUST_LOG="app=debug,wishlist=debug" cargo build --release --bin app
