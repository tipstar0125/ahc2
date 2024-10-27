#!/bin/bash
clear >$(tty)
echo ""
export RUSTFLAGS=-Awarnings
cargo run -r --features local --bin $1 < tools/in/$2.txt > tools/out/$2.txt
cp tools/in/$2.txt in
cp tools/out/$2.txt out