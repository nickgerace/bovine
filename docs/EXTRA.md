# Extra

This document contains extra information related to using `bovine`.

## Post-Installation

It is highly recommended to run `strip` against the binary on compatible systems to reduce executable size.

```sh
( TEMP=$(command -v bovine); du -h $TEMP; strip $TEMP; du -h $TEMP )
```

> The above script will exit with a non-zero exit code if the binary is not installed and/or is not in your `PATH`.

## Automatic Upgrades with Cargo

Keeping the crate up to date is easy with [cargo-update](https://crates.io/crates/cargo-update).

```sh
cargo install cargo-update
cargo install-update -a
```

You can chain this together with the previous step for seamless, automatic upgrades.