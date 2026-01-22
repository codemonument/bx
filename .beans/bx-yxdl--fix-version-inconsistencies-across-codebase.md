---
# bx-yxdl
title: Fix version inconsistencies across codebase
status: todo
type: bug
priority: high
created_at: 2026-01-22T08:56:08Z
updated_at: 2026-01-22T08:56:08Z
---

## Goal
Establish **Cargo.toml as the single source of truth** for the version. All other version references should derive from it.

## Current State
Multiple files have different hardcoded versions:
- Cargo.toml: 0.4.1 ← **This should be the source of truth**
- src/version.rs: 0.3.2 (BONNIE_VERSION constant) - hardcoded
- bonnie.toml (root): 0.3.2 - config file format version
- src/bonnie.toml: 0.3.2 - config file format version
- README.md: references v0.4.0 in Docker section

## Solution
Use Rust's `env!("CARGO_PKG_VERSION")` macro to read version from Cargo.toml at compile time.

## Checklist
- [ ] Update src/version.rs: Replace hardcoded `BONNIE_VERSION` with `env!("CARGO_PKG_VERSION")`
- [ ] Update README.md Docker section to reference current version (or use a placeholder like "latest")
- [ ] Verify bonnie.toml versions - these are config file format versions, may be intentionally different from CLI version
- [ ] Run tests to ensure version compatibility still works
- [ ] Consider if src/bonnie.toml is even needed (it's a test/example file)