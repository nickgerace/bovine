MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
BINARY:=$(MAKEPATH)/target/release/bovine
INSTALLED:=$(HOME)/.cargo/bin/bovine

prepare:
	cd $(MAKEPATH); cargo update
	cd $(MAKEPATH); cargo fix --edition-idioms --allow-dirty --allow-staged
	cd $(MAKEPATH); cargo +nightly fmt
	cd $(MAKEPATH); cargo clippy --all-features --all-targets

release: prepare scan
	cd $(MAKEPATH); cargo build --release
	@du -h $(BINARY)
	@du $(BINARY)

scan:
	cd $(MAKEPATH); cargo +nightly udeps
	cd $(MAKEPATH); cargo audit

bloat:
	cd $(MAKEPATH); cargo bloat --release
	cd $(MAKEPATH); cargo bloat --release --crates

ci:
	cd $(MAKEPATH); cargo +nightly fmt --all -- --check
	cd $(MAKEPATH); cargo clippy -- -D warnings
	cd $(MAKEPATH); cargo test
	cd $(MAKEPATH); cargo build --release

compare:
	@du -h $(INSTALLED)
	@du -h $(BINARY)
	@du $(INSTALLED)
	@du $(BINARY)