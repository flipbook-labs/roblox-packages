---
bump: patch
category: Changes
---

Adopt changewrite for releases: the version (`Cargo.toml`, mirrored to `Cargo.lock`) and changelog are now driven by entry files under `.changes/`, and CI requires every pull request to add one.
