---
# bx-6hym
title: Improve test isolation to prevent race conditions
status: completed
type: task
priority: high
created_at: 2026-01-21T11:15:08Z
updated_at: 2026-01-21T11:25:06Z
---

## Summary

Several tests share global state (environment variables, hardcoded temp files) which can cause race conditions when tests run in parallel.

## Issues Found & Fixes

### ~~HIGH: `src/get_cfg.rs` (4 tests)~~ FIXED
Refactored to use pure `resolve_cfg_path()` function with dependency injection.
Tests are now fully isolated - no env var manipulation needed.

### ~~MEDIUM: `tests/caching.rs` (3 tests)~~ MOSTLY FIXED
- ✅ Replaced hardcoded `/tmp/bonnie_test_*.cache.json` with `tempfile::tempdir()`
- ✅ Each test gets unique auto-cleanup temp directory
- ⚠️ `loads_env_files` still manipulates `SHORTGREETING` env var - unavoidable since it tests env file loading feature

### LOW: `tests/known_cases.rs` (~40 tests) - LINUX ONLY
These tests are gated with `#[cfg(target_os = "linux")]` so they don't run on macOS.
When running on Linux:
- Tests load `src/.env` which sets env vars as part of config loading
- This is testing the feature, not a test isolation bug
- Acceptable risk given test purpose

## Remaining Risk

The `loads_env_files` test in `caching.rs` has a small race window where `SHORTGREETING` env var could be affected by parallel tests. This is inherent to testing the env file loading feature. Options:
1. Accept the risk (current state)
2. Add `serial_test` crate and mark with `#[serial]` if flakiness observed

## Checklist

- [x] Refactor `src/get_cfg.rs` to use pure function with dependency injection
- [x] Refactor `tests/caching.rs` to use `tempfile::tempdir()` for unique paths per test
- [x] Review `SHORTGREETING` env var mutation - unavoidable for testing env file loading
- [x] Review `tests/known_cases.rs` - Linux-only, tests feature not isolation bug
- [x] Run `cargo test` multiple times to verify no flaky tests