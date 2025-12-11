#!/usr/bin/env just --justfile

# private is used so these two don't show up in the --list output
[private]
default: list
[private]
list:
    just --list

# Start a nix dev environment with `nix develop`
dev:
    nix develop --ignore-environment

# Run benchmarks with criterion
bench:
	cargo bench

# Run cargo build with --release flag to build the binary
build:
	cargo build --release

# Build the wasm code
build-web:
	cd wasm && wasm-pack build --release --target web --out-dir pkg

# Publish the wasm code as npm package
publish-web:
	cd wasm && wasm-pack pack
	cd wasm && wasm-pack publish --target web
