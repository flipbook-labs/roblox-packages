# roblox-packages agent guide

roblox-packages is a Rust CLI that installs Roblox Luau packages (Foundation, Promise, Dash, and friends) into a project. It is a Cargo crate that ships as prebuilt binaries for macOS, Linux, and Windows. This file is the entry point for any agent working in the repo. It routes you to the shared skills library and the repo's own setup.

## Mandatory first steps

Complete every step below before you write any code, tests, changelog entries, or PR prose. This is a gate, and it does not depend on how you classify your task.

1. Install the skills toolchain and dependencies:

   ```sh
   rokit install && lute pkg install --vendor
   ```

   `rokit install` puts `lute` on PATH, and `lute pkg install --vendor` copies each dependency in [`pkg.luau`](pkg.luau) into a gitignored `.lpm/<key>@<rev>` folder. The shared skills library is a dev dependency, so it lands in `.lpm/` for you to read; it is not part of the Rust build. Without this step the skills are not on disk. The crate itself builds with Cargo and does not need Lute; see "Repo specifics".

2. Find the skills folder: it's `.lpm/agent-skills@<rev>`, where `<rev>` is the `agent-skills` rev in [`pkg.luau`](pkg.luau). Use that path wherever `<skills>` appears below.

3. Read the routing index at `<skills>/AGENTS.md` in full. Its **Project Skills** section lists every skill with a trigger-rich one-liner, so you know what exists before you start. This read is unconditional.

Deep reads of individual `<skills>/src/<scope>/<name>/SKILL.md` files stay on demand: when a task matches a trigger from the index, read that skill before doing the work it covers. Reading the index up front is what lets those triggers fire. Note that the shared library is Luau-oriented, so some skills will not apply to this Rust crate; use judgment.

Reading a skill once does not mean you applied it when you wrote the code hours later. When you take a pass over a change and before you call a task done, follow the `org/review-pass` skill: it maps the diff to the skills that govern each file, has you re-open them at that point, runs the writing-style and code-comments critic passes over what you generated, and catches skill drift to feed back (see below).

## Forward this file to subagents

The Task tool spawns subagents in a fresh context that does not include this guide, so they never see the gate unless you hand it to them. Whenever you delegate to a subagent, you MUST include in its prompt:

1. The mandatory first steps above (the bootstrap command and the index read).
2. The concrete resolved path(s) to the specific `SKILL.md` files relevant to its task.

Never assume a subagent will discover the skills on its own.

## Keeping skills current

Skills are living documents. When your work here contradicts a skill (a renamed symbol, a changed value, a fixed bug it still calls known), fix it in the agent-skills repo with a `.changes/` entry in the same effort. The fix reaches this repo on its next `rev` bump. The `org/review-pass` skill folds this check into closing out a task, so it fires without you having to remember it.

## Repo specifics

- This is a Rust crate. The toolchain is stable Rust via `rustup`; build with `cargo build`, lint with `cargo clippy --all-features` and `cargo fmt --check`, and run tests with `cargo nextest run` (or `cargo test`).
- The library lives in [`src/`](src) and the CLI entry point is wired through `Cargo.toml`'s `[lib]`. `cargo run -- install <index>` is the primary smoke test.
- Releases are handled by the [Changewrite](https://github.com/flipbook-labs/changewrite) action. The version of record is `current` in [`changewrite.luau`](changewrite.luau), mirrored into `Cargo.toml`, `Cargo.lock`, and `pkg.luau`. Unreleased entries live as partials in [`.changes/`](.changes) and are assembled into [`CHANGELOG.md`](CHANGELOG.md) at release time. Add a `.changes/` entry for any user-facing change.
