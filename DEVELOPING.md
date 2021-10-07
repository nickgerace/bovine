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

## Building

Building `bovine` should be possible on every tier one platform that Rust supports with no restriction on multi-platform development.
We use `cargo xtask` for automation, which only requires `cargo` to be installed to work.
For ease of use, you can alias `cargo xtask` to `cx` or something similar.

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
However, logging and printing requires manual review for style:

- **logging (non-`INFO`)**: log messages should start with lowercase letters and _usually_ not end with punctuation
- **logging (`INFO`)**: user messages intended for the user (`INFO`) are flexible in their style, but should start with uppercase letters when possible
- **printing**: printing directly to `STDOUT` or `STDERR` is only permitted when re-printing a formatted log