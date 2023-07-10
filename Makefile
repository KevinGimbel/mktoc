.DEFAULT_GOAL:=help
.PHONY = bench build docs

help:	## Show this help.
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

bench:	## Run benchmarks with criterion
	@cargo bench

build:	## Run cargo build with --release flag to build the binary
	@cargo build --release

build-web:	## Build the wasm code
	@cd wasm && wasm-pack build --release --target web --out-dir pkg

publish-web:	## Publish the wasm code as npm package
	@cd wasm && wasm-pack pack
	@cd wasm && wasm-pack publish --target web