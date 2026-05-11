set shell := ["bash", "-cu"]

compose := "docker compose"

default:
    @just --list

check:
    @cargo check -q

clippy:
    @cargo clippy -q --all-targets --all-features -- -D warnings

test:
    @cargo test -q

run:
    @cargo run -q --bin fittrack

clean:
    @cargo clean -q

env:
    @sh scripts/init-env.sh

up: _compose_up

down: _compose_down

restart: _compose_restart

rebuild: _compose_rebuild

_compose_up:
    @{{ compose }} up -d --build

_compose_down:
    @{{ compose }} down --remove-orphans

_compose_build_no_cache:
    @{{ compose }} build --no-cache

_compose_restart: _compose_down _compose_up

_compose_rebuild: _compose_down _compose_build_no_cache _compose_up
