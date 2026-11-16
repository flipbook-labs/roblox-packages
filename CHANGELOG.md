# Changelog


## v0.5.2

Internal 2

<details>
<summary>Internal (2)</summary>

1. <picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-release-dark.svg"><source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-release-light.svg"><img src="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-release-light.png" alt="Release" height="18" align="texttop"></picture> Upgrade Changewrite to `v0.11.0`, so release notes mark each entry with a chip naming the part of roblox-packages it changed and collapse the Dependencies and Internal sections. ([#23](https://github.com/flipbook-labs/roblox-packages/pull/23))
2. <picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-tooling-dark.svg"><source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-tooling-light.svg"><img src="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-tooling-light.png" alt="Tooling" height="18" align="texttop"></picture> Declare the shared skills dev dependency in `pkg.luau` on Lute `1.0.1-nightly.20261115`. ([#26](https://github.com/flipbook-labs/roblox-packages/pull/26))

</details>


## v0.5.1

### Fixes

- Installing with `-d` no longer prunes dependencies that Rotriever marks as `<patched>`, such as Foundation's React and Promise.

### Changes

- Releases now go through a Changewrite publish PR, with release notes assembled from `.changes/` entries.

- CI caches Rust builds per target and no longer installs nextest, which it didn't use and which took over six minutes to compile on macOS.
