---
# bx-aorw
title: Fix clippy warnings in tests
status: completed
type: bug
priority: low
created_at: 2026-01-22T08:56:30Z
updated_at: 2026-01-22T10:55:17Z
---

Clippy reports unused code in test files.

## Checklist
- [x] tests/known_cases.rs:9 - Gate import with `#[cfg(target_os = "linux")]` (only used by Linux-specific tests)
- [x] tests/known_cases.rs:34 - Gate macro with `#[cfg(all(test, target_os = "linux"))]`
- [x] tests/known_cases.rs:56 - Gate macro with `#[cfg(all(test, target_os = "linux"))]`
- [x] tests/known_cases.rs:76 - Gate macro with `#[cfg(all(test, target_os = "linux"))]`
- [x] tests/known_cases.rs:90 - Gate macro with `#[cfg(all(test, target_os = "linux"))]`
- [x] tests/known_cases.rs:15 - Gate function with `#[cfg(all(test, target_os = "linux"))]`
- [x] tests/caching.rs:76 - Changed to `.is_err()` instead of `matches!(..., Err(_))`
- [x] Run cargo clippy to verify all warnings are resolved