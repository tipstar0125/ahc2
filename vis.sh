export RUSTFLAGS=-Awarnings
cargo +1.74-x86_64-unknown-linux-gnu run -r --manifest-path tools/Cargo.toml --bin vis $1 $2