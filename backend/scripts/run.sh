#!/bin/sh

RUST_LOG="api=debug,app=debug,wishlist=debug" cargo run --release --bin app
