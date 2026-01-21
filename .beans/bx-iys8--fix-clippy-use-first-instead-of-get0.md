---
# bx-iys8
title: 'Fix clippy: use .first() instead of .get(0)'
status: todo
type: task
priority: low
created_at: 2026-01-21T12:26:33Z
updated_at: 2026-01-21T12:26:33Z
---

## Summary
Clippy `get_first` warning: accessing first element with `.get(0)` should use `.first()` for idiomatic Rust.

## Locations
- `src/bones.rs:263` - `self.shell.get(0)` → `self.shell.first()`
- `src/schema.rs:33` - `args.get(0)` → `args.first()`
- `src/version.rs:89` - `split.get(0)` → `split.first()`
- `src/bin/main.rs:50` - `prog_args.get(0)` → `prog_args.first()`

## Checklist
- [ ] Fix `src/bones.rs:263`
- [ ] Fix `src/schema.rs:33`
- [ ] Fix `src/version.rs:89`
- [ ] Fix `src/bin/main.rs:50`
- [ ] Run `cargo clippy` to verify

## Notes
Auto-fixable with `cargo clippy --fix`