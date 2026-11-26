# Changelog

## v0.6.0

Features 1 · Fixes 1 · Internal 3

### Features

<picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-install-dark.svg" /><source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-install-light.svg" /><img src="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-install-light.png" alt="Install" height="18" align="texttop" /></picture> Add `--external` to `install`, which links a package to a copy the project provides and prunes Roblox's copy. ([#18](https://github.com/flipbook-labs/roblox-packages/pull/18))

### Fixes

<picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-rotriever-dark.svg" /><source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-rotriever-light.svg" /><img src="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-rotriever-light.png" alt="Rotriever" height="18" align="texttop" /></picture> Resolve packages from Rotriever linkers, so an unknown package name fails the install instead of leaving Packages empty. ([#17](https://github.com/flipbook-labs/roblox-packages/pull/17))

### Internal

<picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-docs-dark.svg" /><source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-docs-light.svg" /><img src="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-docs-light.png" alt="Docs" height="18" align="texttop" /></picture> Document installing, running, and choosing a Roblox version in the README. ([#30](https://github.com/flipbook-labs/roblox-packages/pull/30))

<picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-packages-dark.svg" /><source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-packages-light.svg" /><img src="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-packages-light.png" alt="Packages" height="18" align="texttop" /></picture> Move to Changewrite v0.16.0 and AgentSkills v0.9.0. ([#30](https://github.com/flipbook-labs/roblox-packages/pull/30))

<picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-tooling-dark.svg" /><source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-tooling-light.svg" /><img src="https://raw.githubusercontent.com/flipbook-labs/roblox-packages/main/.changes/assets/kind-tooling-light.png" alt="Tooling" height="18" align="texttop" /></picture> Vendor the shared skills into `.lpm/` on Lute `1.0.1-nightly.20261117`. ([#30](https://github.com/flipbook-labs/roblox-packages/pull/30))

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
