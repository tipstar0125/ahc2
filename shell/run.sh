#!/bin/bash
clear >$(tty)
echo ""
export RUSTFLAGS=-Awarnings
cargo +1.70-x86_64-unknown-linux-gnu run -r --features local --bin $1 < tools/in/$2.txt > tools/out/$2.txt
cp tools/in/$2.txt in
cp tools/out/$2.txt out