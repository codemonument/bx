---
# bx-vomo
title: 'Fix clippy: needless late initialization'
status: todo
type: task
priority: low
created_at: 2026-01-21T12:26:44Z
updated_at: 2026-01-21T12:26:44Z
---

## Summary
Clippy `needless_late_init` warning: variables are declared and then immediately assigned in conditional branches. Can use `let x = if {...} else {...}` pattern instead.

## Locations
- `src/bones.rs:336` - `directive_json` can be initialized directly with if/else expression
- `src/bin/main.rs:90` - `cfg` can be initialized directly with if/else expression

## Checklist
- [ ] Refactor `src/bones.rs:336` to use direct initialization
- [ ] Refactor `src/bin/main.rs:90` to use direct initialization
- [ ] Run `cargo clippy` to verify

## Notes
Auto-fixable with `cargo clippy --fix`