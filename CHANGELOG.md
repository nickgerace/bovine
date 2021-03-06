# Changelog

All notable changes to this project will be documented in this file.
All changes are from [@nickgerace](https://github.com/nickgerace) unless otherwise specified.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

The latest version contains all changes.

## [1.0.3] - 2021-12-01

### Changed

- Misc packge bumps

## [1.0.2] - 2021-10-29

### Changed

- Misc package bumps

## [1.0.1] - 2021-10-21

### Changed

- Clap CLI from `3.0.0-beta.4` to `3.0.0-beta.5` due to a breaking change on `cargo install`

## [1.0.0] - 2021-10-09

### Added

- Ability to handle all environment usages of `RUST_LOG`
- Homebrew install for macOS users
- Independent bootstrap password command from the original `logs` subcommand

### Changed

- Many "behind the scenes" and repository functions, which include switching from `make` to `cargo xtask`
- Version subcommand output to include permissions errors

### Removed

- Bootstrap password flags and functionality for `logs` subcommand

## [0.3.0] - 2021-09-10

### Changed

- All direct-to-STDOUT statements to use `INFO` logging (except when printing formatted logs)

### Removed

- The ability to run in debug mode while using the `logs` subcommand (avoid logging a formatted log)

## [0.2.0] - 2021-09-04

### Added

- Wait flag when finding the bootstrap password

### Changed

- The version command to print valid JSON rather than invalid YAML (previous implementation)
- The version command to return an error if JSON pretty printing fails

## [0.1.2] - 2021-08-30

### Added

- Ability to find the bootstrap password (Rancher >=v2.6)
- Short flag for deleting container(s) and volume(s)

## [0.1.1] - 2021-08-25

### Added

- Alias for `force-pull` flag: `fp`
- Explicit `no-cacerts` flag
- Short flag for bootstrap password

### Changed

- Windows binary name to include the `.exe` suffix

## [0.1.0] - 2021-08-17

### Added

- Base contents for repository
- GitHub action for merges and pull requests
- GitHub action for releases
- Pull request template
