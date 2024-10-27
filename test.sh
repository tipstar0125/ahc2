#!/bin/bash
clear >$(tty)
echo ""
export RUSTFLAGS=-Awarnings
cargo test -r --features local --bin $1 -- --nocapture < in