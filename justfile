set shell := ["bash", "-cu"]

default:
    @just --list

check:
    cargo check -q

clippy:
    cargo clippy -q --all-targets --all-features -- -D warnings

test:
    cargo test -q

run:
    cargo run -q

clean:
    cargo clean -q
