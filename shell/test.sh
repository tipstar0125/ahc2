#!/bin/bash
clear >$(tty)
echo ""
export RUSTFLAGS=-Awarnings
cargo +1.70-x86_64-unknown-linux-gnu test -r --features local --bin $1 -- --nocapture < in

