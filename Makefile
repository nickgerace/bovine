MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
BINARY:=$(MAKEPATH)/target/release/bovine

all: prepare build

final: prepare scan build

debug: prepare
	cd $(MAKEPATH); cargo build

build:
	cd $(MAKEPATH); cargo build --release
	@du -h $(BINARY)
	@du $(BINARY)

prepare:
	cd $(MAKEPATH); cargo update
	cd $(MAKEPATH); cargo fix --edition-idioms --allow-dirty --allow-staged
	cd $(MAKEPATH); cargo +nightly fmt
	cd $(MAKEPATH); cargo clippy --all-features --all-targets

scan:
	cd $(MAKEPATH); cargo +nightly udeps
	cd $(MAKEPATH); cargo audit

bloat:
	cd $(MAKEPATH); cargo bloat --release
	cd $(MAKEPATH); cargo bloat --release --crates

release:
	cd $(MAKEPATH); cargo +nightly fmt --all -- --check
	cd $(MAKEPATH); cargo clippy -- -D warnings
	cd $(MAKEPATH); cargo test -- --nocapture
	cd $(MAKEPATH); cargo build --release

compare:
	cd $(MAKEPATH); cargo build --release
	@du $(BINARY)
	@du ~/.cargo/bin/bovine
