# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Project baseline documentation (`CHANGELOG.md`, `CONTRIBUTING.md`, PR template).
- `.env.example` template to replace the previously tracked `.env`.
- Pinned Rust toolchain via `rust-toolchain.toml` (1.94.0).
- Optimized `[profile.release]` (LTO, single codegen unit, panic=abort, symbol stripping).

### Changed
- Fixed broken Rust example in `README_CRATE.md` (missing semicolon and stale
  variable name).
- Cleaned up and deduplicated `.gitignore`.

### Removed
- Unmaintained `rusty-hook` dependency and `.rusty-hook.toml` configuration.
  Pre-commit and pre-push checks are moving to CI.
- `.env` stopped being tracked (its contents moved to `.env.example`).

## [0.1.4] - 2024-10-09

### Added
- `--debug-level` CLI argument to control verbosity of the interpreter output.

### Changed
- Documentation updates across `README.md`.

## [0.1.3] - 2023-10-16

### Added
- `src/lib.rs` exposing the core types so the crate can be consumed as a
  library, not only as a binary.

## [0.1.2] - 2023-09-19

### Changed
- `Cargo.toml` metadata polish (package info, readme, excludes) for crates.io.

## [0.1.1] - 2023-09-19

### Fixed
- Install script.

## [0.1.0] - 2023-09-11

### Added
- Initial public release.
- EVM bytecode interpreter covering most opcodes (arithmetic, bitwise,
  comparison, stack, memory, storage, control flow, logs, system).
- In-memory state with optional RPC fork support via `ethers`.
- CLI binary `evm-rs` with configurable caller, origin, address, value,
  calldata, fork URL and debug level.

[Unreleased]: https://github.com/Yashiru/evm-rs-emulator/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/Yashiru/evm-rs-emulator/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/Yashiru/evm-rs-emulator/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/Yashiru/evm-rs-emulator/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/Yashiru/evm-rs-emulator/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Yashiru/evm-rs-emulator/releases/tag/v0.1.0
