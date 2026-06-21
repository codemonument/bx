---
# bx-yxdl
title: Fix version inconsistencies across codebase
status: completed
type: bug
priority: high
created_at: 2026-01-22T08:56:08Z
updated_at: 2026-01-22T11:00:27Z
---

## Goal
Decouple CLI version from config file format version. CLI version comes from Cargo.toml. Config file format version is tracked separately with a list of supported versions.

## Checklist
- [x] Add `CLI_VERSION` constant using `env!("CARGO_PKG_VERSION")`
- [x] Add `SUPPORTED_CONFIG_VERSIONS` list (currently: ["0.3.2"])
- [x] Add `LATEST_CONFIG_VERSION` for templates/error messages
- [x] Update version checking to use supported versions list instead of exact match
- [x] Update help/version output to use CLI_VERSION
- [x] Update template generation to use LATEST_CONFIG_VERSION
- [x] Fix README.md Docker section with VERSION placeholder
- [x] Run tests to ensure everything works

## Resolution Notes
- **CLI version** (0.4.1): Read from `Cargo.toml` via `env!("CARGO_PKG_VERSION")`
- **Config format version** (0.3.2): Tracked in `SUPPORTED_CONFIG_VERSIONS` list
- Config files use `version = "0.3.2"` which remains valid
- When breaking changes are introduced, add new version to supported list
- `bx -v` now shows "bx v0.4.1" with correct GitHub URL

## New Constants in src/version.rs
```rust
pub const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SUPPORTED_CONFIG_VERSIONS: &[&str] = &["0.3.2"];
pub const LATEST_CONFIG_VERSION: &str = "0.3.2";
```