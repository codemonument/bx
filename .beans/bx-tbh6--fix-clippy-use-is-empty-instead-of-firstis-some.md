---
# bx-tbh6
title: 'Fix clippy: use is_empty instead of first().is_some()'
status: completed
type: task
priority: low
created_at: 2026-01-21T15:54:13Z
updated_at: 2026-01-21T19:58:04Z
---

## Summary
Clippy `unnecessary_first_then_check` warning: using `.first().is_some()` to check if slice is not empty.

## Location
### src/bin/main.rs:50
```rust
// Before
if prog_args.first().is_some() {

// After
if !prog_args.is_empty() {
```

## Checklist
- [x] Change `prog_args.first().is_some()` to `!prog_args.is_empty()`
- [x] Run `cargo test` to ensure nothing broke
- [x] Run `cargo clippy` to verify warning is gone

## Notes
Simple fix but wasn't auto-applied.