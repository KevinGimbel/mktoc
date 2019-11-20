.PHONY = bench build docs

bench:
	@cargo bench
build:
	@cargo build --release

docs:
	@cargo doc

frontend:
	@cp -r ./target/criterion docs/benches/
	@cp -r ./target/doc docs/