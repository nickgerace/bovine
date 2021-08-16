# Release

This document contains all information related to release.

## Preparation

- [ ] Change the `version` field in `Cargo.toml` to `$NEW_TAG`.
- [ ] Run `make release` and verify that everything looks/works as expected.
- [ ] Change the version in `CHANGELOG.md` and uncomment the line, `<!--The latest version contains all changes.-->`.
- [ ] Create a commit with the following message: `Update to <new-tag>`. Do not push (or merge) the commit.
- [ ] Test and verify the publishing workflow: `cargo publish --dry-run`.
- [ ] Finally, push (or merge) the preparation commit.

## Tagging and Publishing

- [ ] Once the prepation commit has been pushed (or merged) into `main`, tag with `git tag $NEW_TAG` and push the tag: `git push --tags origin main`.
- [ ] Now, publish the crate: `cargo publish`.
- [ ] Check the [crate](https://crates.io/crates/bovine) on `crates.io`.
- [ ] Check the [docs](https://docs.rs/bovine) on `docs.rs`.