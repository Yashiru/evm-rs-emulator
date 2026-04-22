# Contributing to evm-rs-emulator

Thanks for your interest in contributing! This document describes how to get
set up and what is expected of a pull request.

## Getting started

```bash
git clone https://github.com/Yashiru/evm-rs-emulator
cd evm-rs-emulator
cargo build
cargo test
```

The Rust toolchain pinned in `rust-toolchain.toml` is used automatically by
rustup — you should not need to install a specific version manually.

## Development workflow

1. Fork the repository (or create a branch if you have write access).
2. Create a branch with a descriptive prefix:
   - `feat/<slug>` for new features
   - `fix/<slug>` for bug fixes
   - `refactor/<slug>` for internal cleanups that do not change behavior
   - `perf/<slug>` for performance work
   - `test/<slug>` for test-only changes
   - `docs/<slug>` for documentation
   - `chore/<slug>` for tooling / repo hygiene
   - `ci/<slug>` for CI workflow changes
3. Make **atomic commits**. Each commit must pass:
   - `cargo build --all-targets`
   - `cargo test`
   - `cargo fmt --all -- --check`
   - `cargo clippy --all-targets -- -D warnings`
   - `cargo doc --no-deps`
4. Write commit messages following
   [Conventional Commits](https://www.conventionalcommits.org/):
   ```
   <type>(<scope>): <short imperative summary>

   <optional body explaining WHY, not WHAT>

   <optional footer: BREAKING CHANGE:, refs #issue>
   ```
5. Open a pull request against `master` and fill the PR template checklist.

## Code style

- Rust edition 2021.
- `rustfmt` enforced via `rustfmt.toml`.
- `clippy` with `-D warnings` enforced.
- Public items should carry rustdoc documentation.

## Tests

- Unit tests live next to the code they exercise, inside `#[cfg(test)] mod tests`.
- Integration tests go under `tests/` at the workspace root.
- When fixing a bug, add a regression test that fails before the fix and
  passes after.

## Release process

Maintainers only:

1. Bump `version` in `Cargo.toml`.
2. Move the `[Unreleased]` section of `CHANGELOG.md` under the new version
   number and today's date.
3. Commit (`chore(release): X.Y.Z`) and create an annotated tag
   (`git tag -a vX.Y.Z -m "X.Y.Z"`).
4. Push tag and master.
5. `cargo publish --dry-run` then `cargo publish`.

## Reporting issues

Open a GitHub issue describing:
- What you were trying to do.
- What happened.
- What you expected.
- The Rust version, OS, and a minimal reproducer when possible.
