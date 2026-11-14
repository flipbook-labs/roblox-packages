# Changelog


## v0.5.1

### Fixes

- Installing with `-d` no longer prunes dependencies that Rotriever marks as `<patched>`, such as Foundation's React and Promise.

### Changes

- Releases now go through a Changewrite publish PR, with release notes assembled from `.changes/` entries.

- CI caches Rust builds per target and no longer installs nextest, which it didn't use and which took over six minutes to compile on macOS.
