---
bump: patch
category: Internal
---

Adopt the [Changewrite](https://github.com/flipbook-labs/changewrite) release flow. Releases are now driven by a publish PR that bumps the version across `Cargo.toml`, `Cargo.lock`, and `loom.config.luau` and assembles `CHANGELOG.md` from `.changes/` entries; merging it tags the release, drafts it, attaches the platform binaries, and publishes.
