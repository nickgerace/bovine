MAKEPATH:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
BINARY:=$(MAKEPATH)/target/release/bovine

all: fmt build

final: prepare scan build

build:
	cd $(MAKEPATH); cargo build --release
	du -h $(BINARY)
	strip $(BINARY)
	du -h $(BINARY)

fmt:
	cd $(MAKEPATH); cargo +nightly fmt
	cd $(MAKEPATH); cargo clippy

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