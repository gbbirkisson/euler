.PHONY: clean all check test fmt clippy lint build-bin build-docker
.DEFAULT_GOAL:=all

all: check test lint

clean:
	cargo clean

check:
	cargo check

test:
	cargo test --release -- --test-threads=4 --show-output

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy -- -D warnings

lint: fmt clippy

build-bin:
	cargo build --release

build-docker: build-bin
	docker build -t gbbirkisson/euler .