#!/usr/bin/env bash
export LLVM_PROFILE_FILE="mktoc-%p-%m.profraw"
export RUSTFLAGS="-Cinstrument-coverage"

# build project
cargo build

# run tests to create the profraw files needed by grcov
cargo test

# run grcov with "defaults", this is straight up taken from their README 
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

# copy badge to asset directory
cp target/debug/coverage/badges/flat.svg assets/coverage

# cleanup
rm ./*.profraw