---
# bx-6hym
title: Improve test isolation to prevent race conditions
status: todo
type: task
priority: high
created_at: 2026-01-21T11:15:08Z
updated_at: 2026-01-21T11:22:08Z
---

## Summary

Several tests share global state (environment variables, hardcoded temp files) which can cause race conditions when tests run in parallel.

## Issues Found

### ~~HIGH: `src/get_cfg.rs` (4 tests)~~ FIXED
Refactored to use pure `resolve_cfg_path()` function with dependency injection.
Tests are now fully isolated - no env var manipulation needed.

### MEDIUM: `tests/caching.rs` (3 tests)
- Uses hardcoded `/tmp/bonnie_test_*.cache.json` paths
- `loads_env_files` removes `SHORTGREETING` env var
- No temp file cleanup after tests

**Fix:** Use `tempfile` crate to create unique temp directories per test, clean up automatically.

### LOW: `tests/known_cases.rs` (~40 tests)
- Multiple tests load `src/.env` which sets `SHORTGREETING` globally
- `succeeds_with_env_var_interpolation` reads `USER` env var

**Fix:** Consider isolating env file loading or running these tests serially.

## Checklist

- [x] Refactor `src/get_cfg.rs` to use pure function with dependency injection
- [ ] Refactor `tests/caching.rs` to use `tempfile::tempdir()` for unique paths per test
- [ ] Remove `SHORTGREETING` env var mutation in caching tests
- [ ] Review `tests/known_cases.rs` for env var isolation
- [ ] Run `cargo test` multiple times to verify no flaky tests