---
# bx-gmd5
title: 'Fix clippy: redundant pattern matching with matches!'
status: todo
type: task
priority: low
created_at: 2026-01-21T12:26:39Z
updated_at: 2026-01-21T12:26:39Z
---

## Summary
Clippy `redundant_pattern_matching` warning: using `matches!(x, Some(_))` or `matches!(x, None)` instead of `.is_some()` / `.is_none()`.

## Locations
### src/init.rs
- Line 14: `matches!(template, Some(_))` → `template.is_some()`
- Line 23: `matches!(template, Some(_))` → `template.is_some()`

### src/raw_schema.rs
- Line 162 (3 occurrences): multiple matches! → is_some/is_none
- Line 164: `matches!(args, Some(_))` → `args.is_some()`
- Line 177: `matches!(order, Some(_))` → `order.is_some()`
- Line 183: `matches!(subcommands, Some(_))` → `subcommands.is_some()`
- Line 198: `matches!(order, Some(_))` → `order.is_some()`
- Line 200: `matches!(subcommands, Some(_))` → `subcommands.is_some()`

### src/schema.rs
- Line 57: `matches!(command.cmd, Some(_))` → `command.cmd.is_some()`
- Line 66: `matches!(command.order, None)` → `command.order.is_none()`
- Line 214: `matches!(self.subcommands, None)` → `self.subcommands.is_none()`
- Line 215 (2 occurrences): matches! → is_some
- Line 243 (2 occurrences): matches! → is_some

### src/bin/main.rs
- Line 50: `matches!(prog_args.get(0), Some(_))` → can simplify entire check

## Checklist
- [ ] Fix all occurrences in `src/init.rs`
- [ ] Fix all occurrences in `src/raw_schema.rs`
- [ ] Fix all occurrences in `src/schema.rs`
- [ ] Fix occurrence in `src/bin/main.rs`
- [ ] Run `cargo clippy` to verify

## Notes
Auto-fixable with `cargo clippy --fix`