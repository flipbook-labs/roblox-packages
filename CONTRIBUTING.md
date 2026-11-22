# Contributing

roblox-packages is a Rust crate. Build it with the stable Rust toolchain. Cargo does not need Lute or Rokit.

## Checks

Run these before opening a pull request:

```sh
cargo build --locked --all-features
cargo clippy --workspace --locked --all-features
cargo fmt --check
cargo test
```

CI also runs a smoke test that installs the latest packages into a `RobloxPackages` folder and checks that `Foundation.lua` is there. Run it locally with:

```sh
cargo run -- install RobloxPackages
find RobloxPackages -name Foundation.lua
```

## Changes

Each pull request needs a `.changes/` entry for any user-facing change. CI checks for one. The fields and the list of kinds are in [.changes/README.md](.changes/README.md).

## Releases

Releases are cut by [Changewrite](https://github.com/flipbook-labs/changewrite). When unreleased entries exist on `main`, its release workflow opens a publish PR that bumps the version and writes the changelog from those entries. Merging that PR creates the tag, publishes a GitHub release, and attaches the binaries for macOS, Linux, and Windows.
