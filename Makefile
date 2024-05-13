all: build

build: 
	cargo build

run: build
	cargo run

test: build
	cargo test

clean:
	cargo clean

.PHONY: all build run test clean

.SILENT: clean
