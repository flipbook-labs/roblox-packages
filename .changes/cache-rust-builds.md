---
bump: patch
category: Changes
---

CI caches Rust builds per target and no longer installs nextest, which it didn't use and which took over six minutes to compile on macOS.
