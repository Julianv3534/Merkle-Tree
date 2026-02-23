.PHONY: build run test clean check fmt

build:
	cargo build

run:
	cargo run

test:
	cargo test

clean:
	cargo clean

check:
	cargo check

fmt:
	cargo fmt