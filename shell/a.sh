#!/bin/bash
export RUSTFLAGS=-Awarnings
cargo +1.70-x86_64-unknown-linux-gnu run -r --features local --bin a