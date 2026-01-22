---
# bx-bqd6
title: Upgrade Rust edition from 2018 to 2021
status: todo
type: task
priority: low
created_at: 2026-01-22T10:46:24Z
updated_at: 2026-01-22T10:46:24Z
---

Upgrade the Rust edition from 2018 to 2021 for access to newer language features.

## Why
- 2018 edition is outdated
- 2021 edition has better defaults and features
- Improved error messages from compiler
- Access to newer language patterns

## Checklist
- [ ] Run `cargo fix --edition` to auto-fix compatibility issues
- [ ] Update Cargo.toml: change `edition = "2018"` to `edition = "2021"`
- [ ] Run `cargo build` to check for issues
- [ ] Run `cargo test` to verify all tests pass
- [ ] Run `cargo clippy` to check for new warnings
- [ ] Address any compiler warnings or errors

## Notes
- This is low priority and can be deferred
- 2021 edition is stable and well-tested
- Consider 2024 edition later once more mature
- Should be done after other modernization work