# Release

This document contains all information related to release.

## Checklist

This checklist details the `bovine` release process.

### Preparation

- [ ] Checkout (or create a branch of) `main` at its latest commit.
- [ ] Change the `version` field in `Cargo.toml` to the new tag.
- [ ] Run `make ci` and verify that everything looks/works as expected.
- [ ] (Skip for release candidates) change the version in `CHANGELOG.md` and uncomment the line, `<!--The latest version contains all changes.-->`.
- [ ] Create a commit with the following message `Update to <new-tag>`. Do not push (or merge) the commit.
- [ ] Test and verify the publishing workflow: `cargo publish --dry-run`.
- [ ] Finally, push (or merge) the preparation commit into `main`.

### Release Time

- [ ] Once the prepation commit has been pushed (or merged) into `main`, checkout and/or update `main`.
- [ ] Tag with `git tag` and push the new tag: `git push --tags origin main`.
- [ ] Now, publish the crate: `cargo publish`.

### Post Release

- [ ] Check the [crate](https://crates.io/crates/bovine) on `crates.io`.
- [ ] Download the crate via `cargo install bovine` or `cargo install --version <tag> bovine`
- [ ] Check the [release](https://github.com/nickgerace/bovine/releases) on the repository's releases page.
- [ ] Update the [tap](https://github.com/nickgerace/homebrew-nickgerace) for Homebrew users
