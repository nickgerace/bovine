# Developing

This document contains tips, workflows, etc. for working within this repository.

## Prerequisites

Please note: these dependencies are for `bovine` *development*.
Users dependencies are specified in the [README](../README.md).

- **[Docker](https://docs.docker.com/get-docker)**: only the daemon is required
- **[Rust stable toolchain](https://www.rust-lang.org/learn/get-started)**: primary toolchain 
- **[Rust nightly toolchain](https://www.rust-lang.org/learn/get-started)**: secondary toolchain
  - This is only used for formatting and external tooling.
  - `bovine` does not depend on nightly.

You may need to install external crates in use by the Makefile.

## Building

Building `bovine` should be possible on every tier one platform that Rust supports with no restriction on multi-platform development.

### In that case, why a Makefile?

This repository leverages an optional Makefile for common developer workflows.
It is "optional" because it's primarily a wrapper for `cargo` commands.
Makefiles are not ideal for multi-platform use, so our Makefile is designed to be as readable as possible for environments where `make` is not preferred.

## Design Considerations

This section contains design considerations when working within this repository.

### Why not use `lib.rs`?

At the moment, `bovine` does not leverage `lib.rs` since it's designed for application-use.
Moreover, many of the project's "would-be library contents" log errors and debug messages, which is often seen as an anti-pattern for libraries.

### Why are Docker errors sometimes not returned in favor of custom errors?

The Docker daemon can return errors that aren't user friendly in a CLI context.
They are entirely adequate for the daemon, but not for `bovine`.

By using the debug flag (`bovine --debug`), all daemon errors returned are logged.
Users can see both the daemon error(s) *and* the custom error(s) in this context.

This might encroach on poor design territory, but since `bovine` runs with non-persistent execution, the pros outweight the cons: clean UX is provided with the option to enable logging to see both errors.
Having said that, this design point is subject to change.

## Style Guide

Executing `cargo +nightly fmt` within the repository should suffice for most users.
However, logging and printing to `STDOUT` require manual review:

- **logging**: log messages should start with lowercase letters and _usually_ not end with punctuation
- **printing**: user messages printed to `STDOUT` are flexible in their style, but should start with uppercase letters when possible