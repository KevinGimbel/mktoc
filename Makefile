.PHONY = bench build docs

bench:
	@cargo bench

build:
	@cargo build --release

docs:
	@cargo doc